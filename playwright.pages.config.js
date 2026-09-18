import {defineConfig} from '@playwright/test';
import base from './playwright.config.js';

export default defineConfig({...base,
  testDir: './tests/pages',
  use: {...base.use, baseURL: 'http://127.0.0.1:28180'},
  webServer: {
    command: 'python3 -m http.server 28180 --bind 127.0.0.1 --directory .cache/pages',
    url: 'http://127.0.0.1:28180/three-rs-wasm/',
    reuseExistingServer: false,
  },
});
