import { test, expect, type Page } from '@playwright/test';
import { configureHtml, sourceText } from './fixtures.js';

// §8.6 crash-loop recovery. Runs only against a `demo-panic-test` build
// (DEMO_PANIC_TEST=1; see playwright.config.ts), which adds the
// deliberate-panic button; CI runs it on debug and release builds.

const WORKSPACE_KEY = 'htmd_cmark_live_workspace';
const CRASH_MARKER_KEY = 'htmd_cmark_live_crashed';
const INPUT = '<h1>Before the crash</h1>';

function storedItem(page: Page, key: string) {
  return page.evaluate(storageKey => localStorage.getItem(storageKey), key);
}

async function crashOnPersistedInput(page: Page) {
  await page.goto('./');
  await configureHtml(page, 'Faithful');
  await page.getByRole('textbox', { name: 'Input editor' }).fill(INPUT);
  await expect.poll(() => sourceText(page)).toBe('# Before the crash');
  await expect.poll(async () => (await storedItem(page, WORKSPACE_KEY)) ?? '').toContain('Before the crash');

  await page.locator('#panic-test-button').click();

  // index.html's trap listener also shows the overlay and writes the
  // marker; only the Rust hook sets `inert` and moves focus.
  const overlay = page.locator('#crash-overlay');
  await expect(overlay).toBeVisible();
  await expect(overlay).toBeFocused();
  await expect(page.locator('#app-root')).toHaveAttribute('inert', '');
  expect(await storedItem(page, CRASH_MARKER_KEY)).toBe('1');
}

async function expectRestoredWithoutConverting(page: Page) {
  await expect(page.getByRole('textbox', { name: 'Input editor' })).toHaveValue(INPUT);
  await expect(page.locator('#banner-recovery')).toBeVisible();
  await expect(page.locator('#crash-overlay')).toBeHidden();
  await expect(page.locator('#app-root')).not.toHaveAttribute('inert');
  expect(await sourceText(page)).toBe('');
  await expect(page.locator('#preview-frame')).toHaveJSProperty('srcdoc', '');
  for (const canonical of await page.locator('.fidelity-panel .canon').all()) {
    await expect(canonical).toHaveText('');
  }
  await expect(page.locator('.tok-eq, .tok-rem, .tok-ins')).toHaveCount(0);
  expect(await storedItem(page, CRASH_MARKER_KEY)).toBe('1');
}

async function reloadTwiceWithoutChoosing(page: Page) {
  const errors: string[] = [];
  page.on('pageerror', error => errors.push(error.message));
  await page.reload();
  await expectRestoredWithoutConverting(page);
  await page.reload();
  await expectRestoredWithoutConverting(page);
  return errors;
}

test('converting the restored input clears the crash marker', async ({ page }) => {
  await crashOnPersistedInput(page);
  const errors = await reloadTwiceWithoutChoosing(page);

  await page.getByRole('button', { name: 'Convert restored input', exact: true }).click();
  await expect.poll(() => sourceText(page)).toBe('# Before the crash');
  await expect(page.locator('#banner-recovery')).toHaveCount(0);
  await expect.poll(() => storedItem(page, CRASH_MARKER_KEY)).toBeNull();

  await page.reload();
  await expect(page.locator('#banner-recovery')).toHaveCount(0);
  await expect.poll(() => sourceText(page)).toBe('# Before the crash');
  expect(errors, 'Uncaught browser errors after the crash').toEqual([]);
});

test('clearing the buffers clears the crash marker and both inputs', async ({ page }) => {
  await crashOnPersistedInput(page);
  const errors = await reloadTwiceWithoutChoosing(page);

  await page.getByRole('button', { name: 'Clear buffers', exact: true }).click();
  const editor = page.getByRole('textbox', { name: 'Input editor' });
  await expect(editor).toHaveValue('');
  await expect(page.locator('#banner-recovery')).toHaveCount(0);
  expect(await sourceText(page)).toBe('');
  await expect.poll(() => storedItem(page, CRASH_MARKER_KEY)).toBeNull();

  await page.reload();
  await expect(page.locator('#banner-recovery')).toHaveCount(0);
  await expect(editor).toHaveValue('');
  await page.getByRole('radio', { name: 'Markdown → HTML', exact: true }).check();
  await expect(editor).toHaveValue('');
  expect(errors, 'Uncaught browser errors after the crash').toEqual([]);
});
