import { test, expect, configureHtml } from './fixtures.js';
import { clearTimings, measureInput, readJank, waitForPreview } from './timing.js';

test.use({ measurePerformance: true, trace: 'on' });

for (const translation of [null, 'Pure', 'Faithful'] as const) {
  test(`typing near 100 kB remains responsive (${translation ?? 'Markdown'})`, async ({ demo: page, browserName }, testInfo) => {
    if (translation) await configureHtml(page, translation);
    const input = translation ? `<p>${'text '.repeat(19_988)}</p>` : 'text '.repeat(19_988);
    const typed = 'abcdefghijklmnopqrstuvwx';
    expect(Buffer.byteLength(input + typed)).toBeLessThanOrEqual(100_000);
    await measureInput(page, input);
    const editor = page.getByRole('textbox', { name: 'Input editor' });
    await editor.press('ControlOrMeta+End');
    await clearTimings(page);
    await editor.pressSequentially(typed, { delay: 50 });
    await waitForPreview(page, input + typed);
    const jank = await readJank(page);
    await testInfo.attach('typing-jank', {
      body: JSON.stringify({ browserName, bytes: Buffer.byteLength(input + typed), ...jank }, null, 2),
      contentType: 'application/json',
    });
    expect(jank.handlers).toHaveLength(typed.length);
    for (const handler of jank.handlers) expect(handler.duration, 'Input handler must stay below 50 ms').toBeLessThan(50);
    if (browserName === 'chromium') {
      expect(jank.supported, 'Chromium must expose Long Tasks').toBe(true);
      for (const task of jank.longTasks.filter(entry => entry.inputHandlers.length > 0)) {
        const conversionTime = task.conversions.reduce((total, conversion) => total
          + Math.max(0, Math.min(task.startTime + task.duration, conversion.startTime + conversion.duration)
            - Math.max(task.startTime, conversion.startTime)), 0);
        expect(task.duration - conversionTime, 'Keystroke task excluding measured conversion work').toBeLessThan(50);
      }
    }
    await expect(editor).toHaveValue(input + typed);
  });
}

test.describe('Chromium observer', () => {
  test.skip(({ browserName }) => browserName !== 'chromium', 'Long Tasks assertion is Chromium-only');
  test('detects an intentionally slow input handler', async ({ demo: page }) => {
    await measureInput(page, '# Warmup');
    await clearTimings(page);
    await page.evaluate(() => {
      document.querySelector('#editor')!.addEventListener('input', () => {
        const end = performance.now() + 80;
        while (performance.now() < end) { /* Deliberate keystroke-jank fixture. */ }
      }, { once: true });
    });
    await page.getByRole('textbox', { name: 'Input editor' }).pressSequentially('!');
    await waitForPreview(page, '# Warmup!');
    await expect.poll(async () => (await readJank(page)).longTasks.filter(task => task.inputHandlers.length > 0).length)
      .toBeGreaterThan(0);
  });
});
