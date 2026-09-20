import {defineConfig} from '@playwright/test';
import base from './playwright.config.js';
export default defineConfig({...base,
 testMatch:['tsl-extended.spec.js','tsl-surface.spec.js','tsl-compute.spec.js','tsl-particles.spec.js','tsl-filters.spec.js','tsl-passes.spec.js','tsl.spec.js','gltf-instancing.spec.js','gltf-physical.spec.js','gltf-iridescence.spec.js','gltf-avif.spec.js','expanded.spec.js','gallery.spec.js','gallery-scenes.spec.js','gltf-pbr.spec.js','point-lights.spec.js'],
 use:{...base.use,baseURL:'http://127.0.0.1:8174'},
 webServer:{command:'python3 -m http.server 8174 --bind 127.0.0.1',url:'http://127.0.0.1:8174/web/gallery/',reuseExistingServer:false}
});
