import assert from 'node:assert/strict';
import { readFileSync, readdirSync } from 'node:fs';
import type { Page, TestInfo } from '@playwright/test';
import { test, expect, configureHtml, convertInput, sourceText } from './fixtures.js';

const corpusDirectory = new URL('../bench/corpus/', import.meta.url);
const corpusFiles = readdirSync(corpusDirectory)
  .filter(filename => filename !== 'README.md' && /\.(?:md|html)$/.test(filename))
  .sort();
for (const required of ['basics-scaled.md', 'math-scaled.md', 'inline-html.html', 'real-page.html']) {
  assert(corpusFiles.includes(required), `Missing Kamila corpus document: ${required}`);
}

async function measureConversion(page: Page, testInfo: TestInfo, input: string, budgetMs: number) {
  await convertInput(page, input);
  const timing = await page.evaluate(() => ({
    measures: performance.getEntriesByName('htmd:convert', 'measure').map(entry => entry.toJSON()),
    starts: performance.getEntriesByName('htmd:convert:start', 'mark').map(entry => entry.startTime),
    ends: performance.getEntriesByName('htmd:convert:end', 'mark').map(entry => entry.startTime),
  }));
  await testInfo.attach('conversion-timing', {
    body: JSON.stringify({ bytes: Buffer.byteLength(input), budgetMs, ...timing }, null, 2),
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
}

for (const filename of corpusFiles) {
  const input = readFileSync(new URL(filename, corpusDirectory), 'utf8');
  const translations = filename.endsWith('.html') ? ['Pure', 'Faithful'] as const : [null];
  for (const translation of translations) {
    test(`corpus ${filename}${translation ? ` (${translation})` : ''}: < 100 ms`, async ({ demo: page }, testInfo) => {
      if (translation) await configureHtml(page, translation);
      await convertInput(page, translation ? '<p>Warmup</p>' : '# Warmup');
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
  await convertInput(page, '# Previous output');
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

test('adversarial 100 kB divergent fidelity diff: < 1000 ms and skips diff', async ({ demo: page }, testInfo) => {
  await configureHtml(page, 'Faithful');
  await page.getByLabel('Skipped tags (comma-separated)').fill('div');
  await convertInput(page, '<p>Warmup</p>');
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
  await convertInput(page, '# Warmup');
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
