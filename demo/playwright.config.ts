import { defineConfig, devices } from '@playwright/test';
import { publicPath } from './e2e/public-path.mjs';

const baseURL = `http://127.0.0.1:4173${publicPath}`;

export default defineConfig({
  testDir: './e2e',
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
    { name: 'chromium', use: { ...devices['Desktop Chrome'], channel: 'chromium' } },
    {
      name: 'firefox',
      use: { ...devices['Desktop Firefox'] },
      testIgnore: '**/performance.spec.ts',
    },
    {
      name: 'webkit',
      use: { ...devices['Desktop Safari'] },
      testIgnore: '**/performance.spec.ts',
    },
  ],
  webServer: {
    command: 'node e2e/serve.mjs',
    url: baseURL,
    reuseExistingServer: false,
    timeout: 10_000,
  },
});
