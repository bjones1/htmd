import { test as base, expect, type Page } from '@playwright/test';
import { installTiming } from './timing.js';

export const test = base.extend<{ demo: Page; measurePerformance: boolean }>({
  measurePerformance: [false, { option: true }],
  demo: async ({ page, measurePerformance }, use) => {
    const errors: string[] = [];
    page.on('pageerror', error => errors.push(error.message));
    if (measurePerformance) await installTiming(page);
    await page.goto('./');
    await expect(page.getByRole('textbox', { name: 'Input editor' })).toBeVisible();
    await use(page);
    expect(errors, 'Uncaught browser errors').toEqual([]);
    await expect(page.locator('#crash-overlay')).toBeHidden();
  },
});

export { expect };

export async function configureHtml(page: Page, translation: 'Pure' | 'Faithful' = 'Pure') {
  await page.getByRole('radio', { name: 'HTML → Markdown', exact: true }).check();
  await page.getByLabel('Translation mode', { exact: true }).selectOption(translation);
}

export async function convertInput(page: Page, input: string) {
  const editor = page.getByRole('textbox', { name: 'Input editor' });
  await editor.fill('');
  await page.evaluate(() => {
    performance.clearMarks('htmd:convert:start');
    performance.clearMarks('htmd:convert:end');
    performance.clearMeasures('htmd:convert');
  });
  await editor.fill(input);
  await page.waitForFunction(() => performance.getEntriesByName('htmd:convert', 'measure').length > 0);
}

export async function sourceText(page: Page) {
  return page.locator('#source-view').textContent();
}
