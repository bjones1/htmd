import { test, expect, configureHtml, convertInput, sourceText } from './fixtures.js';

test('converts in both directions and renders an isolated preview', async ({ demo: page }) => {
  await convertInput(page, '# Browser conversion\n\nA **strong** result.');
  await expect.poll(() => sourceText(page)).toContain('<h1>Browser conversion</h1>');
  await expect(page.frameLocator('#preview-frame').getByRole('heading')).toHaveText('Browser conversion');
  const preview = page.locator('#preview-frame');
  await expect(preview).toHaveAttribute('sandbox', '');
  await expect(preview).toHaveAttribute('title', 'Rendered preview');
  await expect(preview).toHaveAttribute('referrerpolicy', 'no-referrer');

  await configureHtml(page);
  await convertInput(page, '<h1>HTML conversion</h1><p>A <strong>strong</strong> result.</p>');
  await expect.poll(() => sourceText(page)).toBe('# HTML conversion\n\nA **strong** result.');
  await expect(page.frameLocator('#preview-frame').getByRole('heading')).toHaveText('HTML conversion');
});

test('preserves mode buffers and copies source with Undo', async ({ demo: page }) => {
  await configureHtml(page);
  await convertInput(page, '<p>Original HTML buffer</p>');
  await page.getByRole('radio', { name: 'Markdown → HTML', exact: true }).check();
  await convertInput(page, '**Replacement**');
  await page.getByRole('button', { name: 'Use Output as Input', exact: true }).click();
  await expect(page.getByRole('radio', { name: 'HTML → Markdown', exact: true })).toBeChecked();
  await expect(page.getByRole('textbox', { name: 'Input editor' })).toHaveValue('<p><strong>Replacement</strong></p>\n');
  await expect.poll(() => sourceText(page)).toBe('**Replacement**');
  await page.getByRole('button', { name: 'Undo', exact: true }).click();
  await expect(page.getByRole('textbox', { name: 'Input editor' })).toHaveValue('<p>Original HTML buffer</p>');
  await expect(page.getByRole('radio', { name: 'HTML → Markdown', exact: true })).toBeChecked();
  await page.getByRole('radio', { name: 'Markdown → HTML', exact: true }).check();
  await expect(page.getByRole('textbox', { name: 'Input editor' })).toHaveValue('**Replacement**');
});

test('options reconvert and reset to the demo preset without replacing input', async ({ demo: page }) => {
  await configureHtml(page);
  const input = '<h1>Heading</h1><p>Text.</p>';
  await convertInput(page, input);
  await page.getByLabel('Heading style', { exact: true }).selectOption('Setex');
  await expect.poll(() => sourceText(page)).toBe('Heading\n=======\n\nText.');
  await page.getByRole('button', { name: 'Reset Options to Demo Preset' }).click();
  await expect(page.getByLabel('Heading style', { exact: true })).toHaveValue('Atx');
  await expect.poll(() => sourceText(page)).toBe('# Heading\n\nText.');
  await expect(page.getByRole('textbox', { name: 'Input editor' })).toHaveValue(input);
  await page.getByLabel('Translation mode', { exact: true }).selectOption('Faithful');
  await expect(page.getByLabel('Line break style', { exact: true })).toBeDisabled();
  await expect(page.locator('.diff-ok')).toHaveText('No differences.');
});

test('loads and reloads samples and restores a workspace after refresh', async ({ demo: page }) => {
  await page.getByRole('button', { name: 'Basics', exact: true }).click();
  await expect(page.getByRole('button', { name: 'Use Output as Input' })).toBeEnabled();
  const editor = page.getByRole('textbox', { name: 'Input editor' });
  const sample = await editor.inputValue();
  await convertInput(page, '# Edited sample');
  await page.getByRole('button', { name: 'Reload Sample', exact: true }).click();
  await expect(editor).toHaveValue(sample);
  await convertInput(page, '# Persisted workspace');
  await page.getByRole('tab', { name: 'Source', exact: true }).click();
  await page.reload();
  await expect(editor).toHaveValue('# Persisted workspace');
  await expect(page.getByRole('tab', { name: 'Source', exact: true })).toHaveAttribute('aria-selected', 'true');
  await expect.poll(() => sourceText(page)).toContain('<h1>Persisted workspace</h1>');
});

test('Use Output and Undo preserve the opposite buffer and keyboard focus', async ({ demo: page }) => {
  await configureHtml(page);
  await convertInput(page, '<p>Saved HTML buffer</p>');
  await page.getByRole('radio', { name: 'Markdown → HTML', exact: true }).check();
  await convertInput(page, '# Replacement');
  await page.getByRole('button', { name: 'Use Output as Input', exact: true }).click();
  const editor = page.getByRole('textbox', { name: 'Input editor' });
  await expect(page.getByRole('radio', { name: 'HTML → Markdown', exact: true })).toBeChecked();
  await expect(editor).toHaveValue('<h1>Replacement</h1>\n');
  await editor.focus();
  const undo = page.getByRole('button', { name: 'Undo', exact: true });
  for (let step = 0; step < 20 && !(await undo.evaluate(element => element === document.activeElement)); step++) {
    await page.keyboard.press('Tab');
  }
  await expect(undo).toBeFocused();
  await page.keyboard.press('Enter');
  await expect(editor).toHaveValue('<p>Saved HTML buffer</p>');
  await expect(page.getByRole('radio', { name: 'HTML → Markdown', exact: true })).toBeChecked();
});

test('enforces the UTF-8 size limit and resumes after a correction', async ({ demo: page }) => {
  await convertInput(page, 'é'.repeat(50_000));
  await expect(page.getByRole('button', { name: 'Use Output as Input' })).toBeEnabled();
  await page.getByRole('textbox', { name: 'Input editor' }).fill('é'.repeat(50_000) + 'a');
  await expect(page.locator('#banner-size')).toBeVisible();
  await expect.poll(() => sourceText(page)).toBe('');
  await expect(page.getByRole('button', { name: 'Use Output as Input' })).toBeDisabled();
  await convertInput(page, '# Recovered');
  await expect(page.locator('#banner-size')).toHaveCount(0);
  await expect.poll(() => sourceText(page)).toContain('<h1>Recovered</h1>');
});

test('shows escaped source and strips preview navigation directives', async ({ demo: page }) => {
  const hostile = '<base href="https://blocked.example/"><meta http-equiv="refresh" content="0;url=https://blocked.example/">'
    + '<link rel="prefetch" href="https://blocked.example/asset"><noscript><a href="https://blocked.example/">hidden</a></noscript>'
    + '<template><p>hidden template</p></template><p><a href="https://blocked.example/" target="_self">Link</a></p>'
    + '<img src="https://blocked.example/image" onerror="window.parent.alert(1)">';
  const navigationRequests: string[] = [];
  page.on('request', request => {
    if (request.isNavigationRequest() && request.url().startsWith('https://blocked.example/')) {
      navigationRequests.push(request.url());
    }
  });
  await convertInput(page, hostile);
  await page.getByRole('tab', { name: 'Source', exact: true }).click();
  await expect(page.locator('#source-view')).toContainText('onerror=');
  await expect(page.locator('#source-view img')).toHaveCount(0);
  const previewDocument = await page.locator('#preview-frame').evaluate((frame: HTMLIFrameElement) => {
    const document = new DOMParser().parseFromString(frame.srcdoc, 'text/html');
    return {
      stripped: document.body.querySelectorAll('base, meta, link, noscript, template, [target]').length,
      csp: document.head.querySelector('meta')?.getAttribute('content'),
      base: document.head.querySelector('base')?.getAttribute('target'),
    };
  });
  expect(previewDocument.stripped).toBe(0);
  expect(previewDocument.csp).toBe("default-src 'none'; img-src data:; style-src 'unsafe-inline';");
  expect(previewDocument.base).toBe('_blank');
  await page.getByRole('tab', { name: 'Rendered', exact: true }).click();
  const frame = page.frameLocator('#preview-frame');
  await frame.getByRole('link', { name: 'Link', exact: true }).click();
  await expect(frame.getByRole('link', { name: 'Link', exact: true })).toBeVisible();
  expect(navigationRequests).toEqual([]);
});
