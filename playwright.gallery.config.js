import {defineConfig} from '@playwright/test';
import base from './playwright.config.js';
export default defineConfig({...base,
 testMatch:['interactive-shaders.spec.js','environment-materials.spec.js','geometry-materials.spec.js','shader-geometry.spec.js','point-clouds.spec.js','buffer-particles.spec.js','shapes.spec.js','material-textures.spec.js','tsl-audio.spec.js','tsl-procedural.spec.js','tsl-primitives.spec.js','tsl-viewport.spec.js', 'tsl-materials.spec.js','tsl-lighting.spec.js','tsl-environment.spec.js','tsl-next.spec.js','tsl-extended.spec.js','tsl-surface.spec.js','tsl-compute.spec.js','tsl-particles.spec.js','tsl-filters.spec.js','tsl-passes.spec.js','tsl.spec.js','gltf-instancing.spec.js','gltf-physical.spec.js','gltf-iridescence.spec.js','gltf-avif.spec.js','expanded.spec.js','gallery.spec.js','gallery-scenes.spec.js','gltf-pbr.spec.js','point-lights.spec.js'],
 use:{...base.use,baseURL:'http://127.0.0.1:8174'},
 webServer:{command:'python3 -m http.server 8174 --bind 127.0.0.1',url:'http://127.0.0.1:8174/web/gallery/',reuseExistingServer:false}
});
