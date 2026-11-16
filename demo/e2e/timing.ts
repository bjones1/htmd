import { expect, type Page } from '@playwright/test';

interface InputTiming {
  id: number;
  value: string;
  start: number;
}

interface PreviewTiming {
  inputId: number;
  srcdoc: string;
  load: number;
  painted?: number;
}

declare global {
  interface Window {
    demoTiming: {
      inputs: InputTiming[];
      previews: PreviewTiming[];
      longTasks: PerformanceEntry[];
      observer?: PerformanceObserver;
    };
  }
}

export async function installTiming(page: Page) {
  await page.addInitScript(() => {
    if (window !== window.top) return;
    const timing: Window['demoTiming'] = { inputs: [], previews: [], longTasks: [] };
    window.demoTiming = timing;
    document.addEventListener('input', event => {
      if (!(event.target instanceof HTMLTextAreaElement) || event.target.id !== 'editor') return;
      timing.inputs.push({
        id: timing.inputs.length + 1,
        value: event.target.value,
        start: performance.now(),
      });
    }, true);
    document.addEventListener('load', event => {
      if (!event.isTrusted || !(event.target instanceof HTMLIFrameElement)
        || event.target.id !== 'preview-frame') return;
      const frame = event.target;
      const preview: PreviewTiming = {
        inputId: timing.inputs.at(-1)?.id ?? 0,
        srcdoc: frame.srcdoc,
        load: performance.now(),
      };
      timing.previews.push(preview);
      // Two parent frames after load are a paint opportunity, not proof of pixels painted.
      requestAnimationFrame(() => requestAnimationFrame(() => {
        if (frame.srcdoc === preview.srcdoc) preview.painted = performance.now();
      }));
    }, true);
    if (PerformanceObserver.supportedEntryTypes.includes('longtask')) {
      timing.observer = new PerformanceObserver(list => timing.longTasks.push(...list.getEntries()));
      timing.observer.observe({ type: 'longtask', buffered: true });
    }
  });
}

export async function clearTimings(page: Page) {
  await page.evaluate(() => {
    for (const name of ['htmd:convert:start', 'htmd:convert:end', 'htmd:input:start', 'htmd:input:end']) {
      performance.clearMarks(name);
    }
    performance.clearMeasures('htmd:convert');
    performance.clearMeasures('htmd:input');
    const timing = window.demoTiming;
    timing.observer?.takeRecords();
    timing.inputs = [];
    timing.previews = [];
    timing.longTasks = [];
  });
}

export async function waitForPreview(page: Page, expectedInput: string) {
  let result: { inputId: number; inputStart: number; loaded: number; paintOpportunity: number; elapsed: number } | undefined;
  await expect.poll(async () => {
    const snapshot = await page.evaluate(expected => {
      const input = window.demoTiming.inputs.findLast(entry => entry.value === expected);
      const frame = document.querySelector<HTMLIFrameElement>('#preview-frame');
      if (!input || !frame) return null;
      const preview = window.demoTiming.previews.findLast(entry => entry.inputId === input.id
        && entry.srcdoc === frame.srcdoc && entry.painted !== undefined);
      if (!preview) return null;
      return {
        input, preview, origin: performance.timeOrigin,
        html: new DOMParser().parseFromString(frame.srcdoc, 'text/html').documentElement.outerHTML,
      };
    }, expectedInput);
    if (!snapshot) return 'waiting for the input and its preview load';
    const element = await page.locator('#preview-frame').elementHandle();
    const frame = await element?.contentFrame();
    if (!frame) return 'waiting for the preview frame';
    try {
      // Playwright can inspect the opaque sandbox without granting the document same-origin access.
      const documentTiming = await frame.evaluate(() => {
        const navigation = performance.getEntriesByType('navigation')[0] as PerformanceNavigationTiming;
        return {
          html: document.documentElement.outerHTML,
          complete: document.readyState === 'complete',
          origin: performance.timeOrigin,
          // WebKit omits Navigation Timing entries for srcdoc documents.
          domComplete: performance.timeOrigin + (navigation?.domComplete ?? 0),
        };
      });
      if (!documentTiming.complete) return 'waiting for the document to complete';
      if (documentTiming.html !== snapshot.html) return 'preview document differs from current srcdoc';
      if (documentTiming.origin < snapshot.origin + snapshot.input.start) return 'preview navigation predates input';
      // Firefox rounds the parent and sandbox clocks independently to milliseconds.
      if (documentTiming.domComplete > snapshot.origin + snapshot.preview.load + 2) {
        return `document completed ${documentTiming.domComplete - snapshot.origin - snapshot.preview.load} ms after the recorded load`;
      }
    } catch (error) {
      if (error instanceof Error && /context was destroyed|frame was detached/i.test(error.message)) return 'preview is navigating';
      throw error;
    }
    result = {
      inputId: snapshot.input.id,
      inputStart: snapshot.input.start,
      loaded: snapshot.preview.load,
      paintOpportunity: snapshot.preview.painted!,
      elapsed: snapshot.preview.painted! - snapshot.input.start,
    };
    return 'ready';
  }, { message: 'The current input must finish loading its own preview and reach a paint opportunity' }).toBe('ready');
  return result!;
}

export async function measureInput(page: Page, input: string) {
  await clearTimings(page);
  await page.getByRole('textbox', { name: 'Input editor' }).fill(input);
  return waitForPreview(page, input);
}

export async function readJank(page: Page) {
  return page.evaluate(() => {
    const timing = window.demoTiming;
    timing.longTasks.push(...(timing.observer?.takeRecords() ?? []));
    const intervals = (name: string) => performance.getEntriesByName(name, 'measure').map(entry => ({
      startTime: entry.startTime, duration: entry.duration,
    }));
    const handlers = intervals('htmd:input');
    const conversions = intervals('htmd:convert');
    const longTasks = timing.longTasks.map(entry => ({
      startTime: entry.startTime,
      duration: entry.duration,
      inputHandlers: handlers.filter(handler => handler.startTime < entry.startTime + entry.duration
        && handler.startTime + handler.duration > entry.startTime),
      conversions: conversions.filter(conversion => conversion.startTime < entry.startTime + entry.duration
        && conversion.startTime + conversion.duration > entry.startTime),
    }));
    return { supported: Boolean(timing.observer), handlers, conversions, longTasks };
  });
}
