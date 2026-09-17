import {defineConfig} from '@playwright/test';
import base from './playwright.config.js';
export default defineConfig({...base,
 testMatch:['expanded.spec.js','gallery.spec.js','gallery-scenes.spec.js','gltf-pbr.spec.js','point-lights.spec.js'],
 use:{...base.use,baseURL:'http://127.0.0.1:8174'},
 webServer:{command:'python3 -m http.server 8174 --bind 127.0.0.1',url:'http://127.0.0.1:8174/web/gallery/',reuseExistingServer:false}
});
