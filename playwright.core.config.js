import {defineConfig} from '@playwright/test';
import base from './playwright.config.js';
export default defineConfig({...base,
 testMatch:['core-animation.spec.js','core-materials.spec.js','core-robot.spec.js','gpu-performance.spec.js'],
 use:{...base.use,baseURL:'http://127.0.0.1:8176'},
 webServer:{command:'python3 -m http.server 8176 --bind 127.0.0.1',url:'http://127.0.0.1:8176/',reuseExistingServer:false}
});
