import assert from 'node:assert/strict';
import { readFileSync, readdirSync } from 'node:fs';
import type { Page, TestInfo } from '@playwright/test';
import { test, expect, configureHtml, convertInput, sourceText } from './fixtures.js';
import { clearTimings, installTiming, measureInput, waitForPreview } from './timing.js';

test.use({ measurePerformance: true, trace: 'off' });

const corpusDirectory = new URL('../bench/corpus/', import.meta.url);
const corpusFiles = readdirSync(corpusDirectory)
  .filter(filename => filename !== 'README.md' && /\.(?:md|html)$/.test(filename))
  .sort();
for (const required of ['basics-scaled.md', 'math-scaled.md', 'inline-html.html', 'real-page.html']) {
  assert(corpusFiles.includes(required), `Missing corpus document: ${required}`);
}

async function measureConversion(page: Page, testInfo: TestInfo, input: string, budgetMs: number) {
  const rendered = await measureInput(page, input);
  const endToEndBudgetMs = budgetMs === 100 ? 400 : 1_000;
  const timing = await page.evaluate(() => ({
    measures: performance.getEntriesByName('htmd:convert', 'measure').map(entry => entry.toJSON()),
    starts: performance.getEntriesByName('htmd:convert:start', 'mark').map(entry => entry.startTime),
    ends: performance.getEntriesByName('htmd:convert:end', 'mark').map(entry => entry.startTime),
  }));
  await testInfo.attach('conversion-timing', {
    body: JSON.stringify({ bytes: Buffer.byteLength(input), budgetMs, endToEndBudgetMs, rendered, ...timing }, null, 2),
    contentType: 'application/json',
  });
  expect(timing.measures, 'Exactly one full conversion must be measured').toHaveLength(1);
  expect(timing.starts).toHaveLength(1);
  expect(timing.ends).toHaveLength(1);
  const measurement = timing.measures[0];
  expect(measurement.startTime).toBe(timing.starts[0]);
  expect(measurement.duration).toBeCloseTo(timing.ends[0] - timing.starts[0], 3);
  expect(measurement.duration).toBeGreaterThan(0);
  expect(measurement.duration, `Full conversion must take < ${budgetMs} ms`).toBeLessThan(budgetMs);
  expect(rendered.elapsed, `Input through preview paint opportunity must take < ${endToEndBudgetMs} ms`)
    .toBeLessThan(endToEndBudgetMs);
}

for (const filename of corpusFiles) {
  const input = readFileSync(new URL(filename, corpusDirectory), 'utf8');
  const translations = filename.endsWith('.html') ? ['Pure', 'Faithful'] as const : [null];
  for (const translation of translations) {
    test(`corpus ${filename}${translation ? ` (${translation})` : ''}: compute < 100 ms, preview < 400 ms`, async ({ demo: page }, testInfo) => {
      if (translation) await configureHtml(page, translation);
      await measureInput(page, translation ? '<p>Warmup</p>' : '# Warmup');
      await measureConversion(page, testInfo, input, 100);
      await expect(page.getByRole('button', { name: 'Use Output as Input' })).toBeEnabled();
      if (translation === 'Faithful') {
        await expect(page.locator('.fidelity-panel .canon').first()).not.toBeEmpty();
        await expect(page.locator('.fidelity-panel .canon').nth(1)).not.toBeEmpty();
      }
    });
  }
}

test('adversarial 100 kB table amplification: < 1000 ms and clears output', async ({ demo: page }, testInfo) => {
  await measureInput(page, '# Previous output');
  const table = '|'.repeat(51) + '\n' + '|:-'.repeat(50) + '|\n'
    + ('|'.repeat(51) + '\n').repeat(1_200);
  const input = table.padEnd(100_000, ' ');
  expect(Buffer.byteLength(input)).toBe(100_000);
  await measureConversion(page, testInfo, input, 1_000);
  await expect(page.locator('#banner-cap')).toHaveText('Output exceeds the 2 MB display limit.');
  await expect.poll(() => sourceText(page)).toBe('');
  await expect(page.locator('#preview-frame')).toHaveJSProperty('srcdoc', '');
  await expect(page.locator('.fidelity-panel')).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Use Output as Input' })).toBeDisabled();
  await convertInput(page, '# Recovered after amplification');
  await expect(page.locator('#banner-cap')).toHaveCount(0);
});

// §8.6: also behind a leading <template>, whose contents RcDom keeps
// outside `children` but which still cost the quadratic parse.
for (const translation of [null, 'Pure'] as const) {
  for (const prefix of ['', '<template>']) {
    const shape = prefix ? 'nested <ul> behind <template>' : 'nested <ul>';
    test(`adversarial 100 kB of ${shape} (${translation ?? 'Markdown'}): < 1000 ms and rejected`, async ({ demo: page }, testInfo) => {
      if (translation) await configureHtml(page, translation);
      await measureInput(page, translation ? '<p>Previous output</p>' : '# Previous output');
      const input = prefix + '<ul>'.repeat(Math.floor((100_000 - prefix.length) / 4));
      expect(Buffer.byteLength(input)).toBeLessThanOrEqual(100_000);
      expect(Buffer.byteLength(input)).toBeGreaterThan(99_990);
      await measureConversion(page, testInfo, input, 1_000);
      await expect(page.locator('#banner-nesting')).toHaveText('Input exceeds the supported nesting depth.');
      await expect.poll(() => sourceText(page)).toBe('');
      await expect(page.getByRole('button', { name: 'Use Output as Input' })).toBeDisabled();
    });
  }
}

test('adversarial padded HTML table: < 1000 ms and rejected before htmd converts', async ({ demo: page }, testInfo) => {
  await configureHtml(page, 'Pure');
  await measureInput(page, '<p>Previous output</p>');
  const input = '<table><tr>' + `<th>${'w'.repeat(1_000)}</th>`.repeat(50) + '</tr>'
    + `<tr>${'<td></td>'.repeat(50)}</tr>`.repeat(100) + '</table>';
  expect(Buffer.byteLength(input)).toBeLessThanOrEqual(100_000);
  await measureConversion(page, testInfo, input, 1_000);
  await expect(page.locator('#banner-cap')).toHaveText('Output exceeds the 2 MB display limit.');
  await expect.poll(() => sourceText(page)).toBe('');
  await expect(page.getByRole('button', { name: 'Use Output as Input' })).toBeDisabled();
});

test('adversarial 100 kB divergent fidelity diff: < 1000 ms and skips diff', async ({ demo: page }, testInfo) => {
  await configureHtml(page, 'Faithful');
  await page.getByLabel('Skipped tags (comma-separated)').fill('div');
  await measureInput(page, '<p>Warmup</p>');
  const input = '<div>' + 'alpha '.repeat(16_664).padEnd(99_989, ' ') + '</div>';
  expect(Buffer.byteLength(input)).toBe(100_000);
  await measureConversion(page, testInfo, input, 1_000);
  await expect(page.locator('.diff-cap')).toHaveText('Diff too large to display.');
  await expect(page.locator('.fidelity-panel .canon').first()).toContainText('alpha');
  await expect(page.locator('.fidelity-panel .canon').nth(1)).toBeEmpty();
  await expect(page.locator('#banner-size')).toHaveCount(0);
  await convertInput(page, '<p>Recovered after large diff</p>');
  await expect(page.locator('.diff-ok')).toHaveText('No differences.');
});

test('adversarial table below the output cap: full preview pipeline < 1000 ms', async ({ demo: page }, testInfo) => {
  await measureInput(page, '# Warmup');
  const input = ('|'.repeat(51) + '\n' + '|:-'.repeat(50) + '|\n'
    + ('|'.repeat(51) + '\n').repeat(1_100)).padEnd(100_000, ' ');
  await measureConversion(page, testInfo, input, 1_000);
  const output = await sourceText(page);
  expect(Buffer.byteLength(output ?? '')).toBeGreaterThan(1_800_000);
  expect(Buffer.byteLength(output ?? '')).toBeLessThanOrEqual(2_000_000);
  await expect(page.locator('#banner-cap')).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Use Output as Input' })).toBeEnabled();
  await expect(page.locator('#preview-frame')).not.toHaveJSProperty('srcdoc', '');
});

for (const translation of [null, 'Pure', 'Faithful'] as const) {
  test(`2,000-byte keystroke to preview <= 100 ms (${translation ?? 'Markdown'})`, async ({ demo: page }, testInfo) => {
    if (translation) await configureHtml(page, translation);
    const input = translation ? `<p>${'é'.repeat(996)}</p>` : '# Immediate\n\n' + 'é'.repeat(993);
    expect(Buffer.byteLength(input + '!')).toBe(2_000);
    await measureInput(page, input);
    const editor = page.getByRole('textbox', { name: 'Input editor' });
    await editor.press('ControlOrMeta+End');
    await clearTimings(page);
    await editor.pressSequentially('!');
    const rendered = await waitForPreview(page, input + '!');
    await testInfo.attach('keystroke-to-preview', {
      body: JSON.stringify(rendered, null, 2), contentType: 'application/json',
    });
    expect(rendered.elapsed).toBeLessThanOrEqual(100);
  });
}

test('warm-cache navigation to conversion-ready <= 1500 ms', async ({ page }, testInfo) => {
  await installTiming(page);
  await page.goto('./');
  await page.waitForFunction(() => performance.getEntriesByName('htmd:ready', 'mark').length === 1);
  await page.goto('./');
  await page.waitForFunction(() => performance.getEntriesByName('htmd:ready', 'mark').length === 1);
  const startup = await page.evaluate(() => ({
    ready: performance.getEntriesByName('htmd:ready', 'mark')[0].startTime,
    resources: (performance.getEntriesByType('resource') as PerformanceResourceTiming[])
      .filter(entry => /\.(wasm|js)$/.test(entry.name)).map(entry => entry.toJSON()),
  }));
  await testInfo.attach('startup-timing', {
    body: JSON.stringify(startup, null, 2), contentType: 'application/json',
  });
  expect(startup.resources.length).toBeGreaterThanOrEqual(2);
  for (const resource of startup.resources) expect(resource.transferSize, 'Warm-cache asset').toBe(0);
  expect(startup.ready).toBeGreaterThan(0);
  expect(startup.ready).toBeLessThanOrEqual(1_500);
  await measureInput(page, '# Conversion-ready proof');
  await expect(page.getByRole('button', { name: 'Use Output as Input' })).toBeEnabled();
});

test('preview timing includes delayed navigation and ignores an unrelated load', async ({ demo: page }) => {
  await measureInput(page, '# Previous preview');
  await page.evaluate(() => {
    const descriptor = Object.getOwnPropertyDescriptor(HTMLIFrameElement.prototype, 'srcdoc')!;
    Object.defineProperty(HTMLIFrameElement.prototype, 'srcdoc', {
      ...descriptor,
      set(this: HTMLIFrameElement, value: string) {
        this.dispatchEvent(new Event('load'));
        setTimeout(() => descriptor.set!.call(this, value), 150);
      },
    });
  });
  const rendered = await measureInput(page, '# Delayed preview');
  expect(rendered.elapsed).toBeGreaterThanOrEqual(150);
  const compute = await page.evaluate(() => performance.getEntriesByName('htmd:convert', 'measure')[0].duration);
  expect(compute).toBeLessThan(100);
  await expect(page.frameLocator('#preview-frame').getByRole('heading')).toHaveText('Delayed preview');
});
