import {defineConfig} from '@playwright/test';
import base from './playwright.config.js';

export default defineConfig({...base,
  // Fixture setup shares this budget. On software GPUs a scene that compiles one
  // pipeline per blend state can keep the GPU busy into the next test's setup.
  timeout: 180000,
  testDir: './tests/pages',
  use: {...base.use, baseURL: 'http://127.0.0.1:28180'},
  webServer: {
    command: 'python3 -m http.server 28180 --bind 127.0.0.1 --directory .cache/pages',
    url: 'http://127.0.0.1:28180/three-rs-wasm/',
    reuseExistingServer: false,
  },
});
