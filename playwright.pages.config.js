import {defineConfig} from '@playwright/test';
import base from './playwright.config.js';

export default defineConfig({...base,
  // Fixture setup shares this budget. On software GPUs a scene that compiles one
  // pipeline per blend state can keep the GPU busy into the next test's setup.
  timeout: 180000,
  testDir: './tests/pages',
  // Lets CI shard the per-example tests of the single spec file.
  fullyParallel: true,
  // The software GPU leaves the runner's cores idle between a test's steps:
  // two browsers per shard run a shard 1.4× faster. With four, scenes that
  // compile many pipelines missed their 90-second frame deadlines.
  workers: 2,
  use: {...base.use, baseURL: 'http://127.0.0.1:28180'},
  // The repository's threaded server: http.server's own backlog of 5 resets the
  // connections of a page's burst of Wasm part requests, and the page stops.
  webServer: {
    command: 'python3 tools/serve.py 28180 .cache/pages',
    url: 'http://127.0.0.1:28180/three-rs-wasm/',
    reuseExistingServer: false,
  },
});
