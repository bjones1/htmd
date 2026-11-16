import { defineConfig, devices } from '@playwright/test';
import { publicPath } from './e2e/public-path.mjs';

const baseURL = `http://127.0.0.1:4173${publicPath}`;

// DEMO_PANIC_TEST=1 marks a `demo-panic-test` build (served via DEMO_DIST):
// only the crash-recovery spec runs there, and never against the shipped
// build, which lacks the deliberate-panic button.
const panicBuild = process.env.DEMO_PANIC_TEST === '1';
const crashSpec = '**/crash-recovery.spec.ts';
const ignored = panicBuild ? [] : [crashSpec];

export default defineConfig({
  testDir: './e2e',
  testMatch: panicBuild ? crashSpec : '**/*.spec.ts',
  fullyParallel: false,
  forbidOnly: Boolean(process.env.CI),
  retries: 0,
  workers: 1,
  timeout: 30_000,
  expect: { timeout: 5_000 },
  reporter: [['list'], ['html', { open: 'never' }]],
  use: {
    baseURL,
    trace: 'retain-on-failure',
    screenshot: 'only-on-failure',
  },
  projects: [
    { name: 'chromium', use: { ...devices['Desktop Chrome'], channel: 'chromium' }, testIgnore: ignored },
    {
      name: 'firefox',
      use: { ...devices['Desktop Firefox'] },
      testIgnore: [...ignored, '**/performance.spec.ts'],
    },
    {
      name: 'webkit',
      use: { ...devices['Desktop Safari'] },
      testIgnore: [...ignored, '**/performance.spec.ts'],
    },
  ],
  webServer: {
    command: 'node e2e/serve.mjs',
    url: baseURL,
    reuseExistingServer: false,
    timeout: 10_000,
  },
});
