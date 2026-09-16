import { defineConfig } from '@playwright/test';
export default defineConfig({
  testDir: './tests/browser', workers: 1, timeout: 60000,
  use: {
    baseURL: 'http://127.0.0.1:8173',
    launchOptions: {
      ...(process.platform === 'darwin' ? { executablePath: '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' } : {}),
      args: ['--enable-unsafe-webgpu', '--enable-features=Vulkan', '--use-angle=default'],
    },
  },
  webServer: { command: 'python3 tools/serve.py', url: 'http://127.0.0.1:8173/web/', reuseExistingServer: false },
});
