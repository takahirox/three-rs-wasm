import {defineConfig} from '@playwright/test';
import base from './playwright.config.js';

export default defineConfig({...base,
  // Fixture setup shares this budget. On software GPUs a scene that compiles one
  // pipeline per blend state can keep the GPU busy into the next test's setup.
  timeout: 180000,
  testDir: './tests/pages',
  // Lets CI shard the per-example tests of the single spec file.
  fullyParallel: true,
  // three.js's own example check renders 800×500 ( test/e2e/puppeteer.js ): the
  // software GPU's cost follows the canvas's pixels.
  use: {...base.use, baseURL: 'http://127.0.0.1:28180', viewport: {width: 800, height: 500},
    // On Linux, Mesa's lavapipe as three.js's check uses, in place of SwiftShader:
    // like it, a headed browser on a virtual display ( xvfb-run, pages.yml ).
    ...(process.platform === 'linux' ? {headless: false, launchOptions: {...base.use.launchOptions,
      env: {...process.env, VK_DRIVER_FILES: '/usr/share/vulkan/icd.d/lvp_icd.x86_64.json'},
      args: ['--enable-unsafe-webgpu', '--enable-features=Vulkan', '--use-angle=vulkan', '--disable-vulkan-surface', '--ignore-gpu-blocklist']}} : {})},
  webServer: {
    command: 'python3 -m http.server 28180 --bind 127.0.0.1 --directory .cache/pages',
    url: 'http://127.0.0.1:28180/three-rs-wasm/',
    reuseExistingServer: false,
  },
});
