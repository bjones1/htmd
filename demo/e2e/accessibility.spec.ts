import AxeBuilder from '@axe-core/playwright';
import { test, expect, configureHtml, convertInput } from './fixtures.js';

for (const state of ['empty', 'rendered', 'source', 'faithful', 'size-limit'] as const) {
  test(`axe WCAG 2.1 AA: ${state}`, async ({ demo: page }, testInfo) => {
    if (state === 'rendered' || state === 'source') {
      await page.getByRole('button', { name: 'Basics', exact: true }).click();
      await expect(page.getByRole('button', { name: 'Use Output as Input' })).toBeEnabled();
    }
    if (state === 'source') {
      await page.getByRole('tab', { name: 'Source', exact: true }).click();
    }
    if (state === 'faithful') {
      await configureHtml(page, 'Faithful');
      await convertInput(page, '<p><span style="white-space: pre">a  b</span></p>');
      await expect(page.locator('.tok-diff ins, .tok-diff del').first()).toBeVisible();
    }
    if (state === 'size-limit') {
      await page.getByRole('textbox', { name: 'Input editor' }).fill('a'.repeat(100_001));
      await expect(page.locator('#banner-size')).toBeVisible();
      await expect(page.locator('#banner-region')).toHaveAttribute('aria-live', 'assertive');
    }

    // User-authored preview documents are outside the demo UI's WCAG scope.
    const results = await new AxeBuilder({ page })
      .setLegacyMode()
      .options({ iframes: false })
      .withTags(['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa'])
      .analyze();
    await testInfo.attach(`axe-${state}`, {
      body: JSON.stringify(results, null, 2),
      contentType: 'application/json',
    });
    expect(results.violations).toEqual([]);
  });
}

test('output tabs follow the keyboard and ARIA tabs pattern', async ({ demo: page }) => {
  await convertInput(page, '# Keyboard tabs');
  const rendered = page.getByRole('tab', { name: 'Rendered', exact: true });
  const source = page.getByRole('tab', { name: 'Source', exact: true });
  await rendered.focus();

  for (const [key, selected, hidden] of [
    ['ArrowRight', source, rendered],
    ['ArrowRight', rendered, source],
    ['ArrowLeft', source, rendered],
    ['Home', rendered, source],
    ['End', source, rendered],
  ] as const) {
    await page.keyboard.press(key);
    await expect(selected).toBeFocused();
    await expect(selected).toHaveAttribute('aria-selected', 'true');
    await expect(selected).toHaveAttribute('tabindex', '0');
    await expect(hidden).toHaveAttribute('aria-selected', 'false');
    await expect(hidden).toHaveAttribute('tabindex', '-1');
    const selectedPanel = await selected.getAttribute('aria-controls');
    const hiddenPanel = await hidden.getAttribute('aria-controls');
    await expect(page.locator(`#${selectedPanel}`)).toBeVisible();
    await expect(page.locator(`#${hiddenPanel}`)).toBeHidden();
  }
});
