// Diagnostic ablations, NOT reproduction acceptance or a production rendering path.
// Serve the repository on BASE_URL (default http://127.0.0.1:8173), then run Node.
import { chromium } from '@playwright/test';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { PNG } from 'pngjs';

const base = process.env.BASE_URL || 'http://127.0.0.1:8173';
const output = process.env.OUTPUT || '.cache/instancing-investigation/repro';
const id = 'webgl_loader_gltf_instancing';
const catalog = JSON.parse(readFileSync('web/gallery/catalog.json'));
catalog.examples.find(e => e.id === id).port = { example: 16, limitations: [] };
mkdirSync(output, { recursive: true });
const diagnosticCases = [
  { name: 'original-1x', samples: 1 },
  { name: 'original-4x', samples: 4 },
  { name: 'solid-1x', samples: 1, mode: 'solid' },
  { name: 'basic-4x', samples: 4, mode: 'basic' },
  { name: 'aligned-basic-4x', samples: 4, flip: true, mode: 'basic' },
  { name: 'aligned-4x', samples: 4, flip: true },
  { name: 'aligned-noMR-4x', samples: 4, flip: true, mode: 'noMR' },
  { name: 'aligned-normal-4x', samples: 4, flip: true, mode: 'normal' },
  { name: 'aligned-roughness-4x', samples: 4, flip: true, mode: 'roughness' },
  { name: 'aligned-radiance-4x', samples: 4, flip: true, mode: 'radiance' },
  { name: 'aligned-copyMR-4x', samples: 4, flip: true, mode: 'copyMR' },
];
// Hold the same direct light on both sides so HDR-off remains visible. Keeping
// it in HDR-on as well avoids changing two lighting variables simultaneously.
const matrixCases = [false, true].flatMap(hdr => [1, 4].map(samples => ({
  name: `hdr-${hdr ? 'on' : 'off'}-msaa-${samples === 4 ? 'on' : 'off'}`,
  samples, hdr, background: hdr, fixedLight: true,
})));
matrixCases.push(...[1, 4].map(samples => ({
  name: `reflection-only-msaa-${samples === 4 ? 'on' : 'off'}`,
  samples, hdr: true, background: false, fixedLight: true,
})));
const referenceBackend = process.env.REFERENCE_BACKEND || 'webgl';
if (!['webgl','webgpu'].includes(referenceBackend)) throw Error('Unknown reference backend');
const matrix = process.env.MATRIX === '1';
const rustFlip = process.env.RUST_FLIP === '1';
const repeats = Number(process.env.REPEATS || 1);
const variant = process.env.VARIANT || 'default';
if (!['default','depth24','coarse','fine'].includes(variant)) throw Error('Unknown VARIANT');
if (!Number.isInteger(repeats) || repeats < 1 || repeats > 10) throw Error('REPEATS must be 1–10');
const selectedCases = (matrix
  ? (rustFlip ? matrixCases.flatMap(c => [false,true].map(reflect => ({...c, name:`${c.name}-${reflect?'reflected':'original'}`,rustFlip:reflect}))) : matrixCases)
  : rustFlip ? [1,4].flatMap(samples => [false,true].map(reflect => ({name:`rust-${reflect?'reflected':'original'}-${samples}x`,samples,rustFlip:reflect}))) : referenceBackend === 'webgpu' ? diagnosticCases.filter(c => c.name.startsWith('original-')) : diagnosticCases)
  .filter(c => !process.env.CASE || c.name.startsWith(process.env.CASE));
if (referenceBackend === 'webgpu' && (matrix || rustFlip || variant !== 'default')) throw Error('WebGPU comparison currently supports the unchanged original scene only');
if (!selectedCases.length) throw Error('No matching diagnostic cases');
const cases = Array.from({length: repeats}, (_, repeat) => selectedCases
  .map(c => ({...c, variant, name: repeats > 1 ? `${c.name}-run${repeat + 1}` : c.name, repeat: repeat + 1}))).flat();
const browser = await chromium.launch({
  executablePath: process.env.CHROME || '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
  args: ['--enable-unsafe-webgpu'],
});
const results = { browser: browser.version(), viewport: [512, 512], threshold: 6,
  referenceBackend, matrix, rustFlip, repeats, lighting: matrix ? 'Fixed white directional light, intensity 3, position (-3,4,-2), on in every case. HDR toggles environment illumination/reflection and background; reflection-only holds the background black. ACES and materials remain unchanged.' : 'Original scene', cases: [] };

// Runs before Wasm initialization. All mutations stay in this diagnostic page.
function instrumentRust({ mode, hdr, background, fixedLight, variant }) {
  window.probeTextures = [];
  window.probeErrors = [];
  const seenDevices = new WeakSet();
  const createTexture = GPUDevice.prototype.createTexture;
  GPUDevice.prototype.createTexture = function (descriptor) {
    window.probeDevice = this;
    if (!seenDevices.has(this)) { seenDevices.add(this); this.addEventListener('uncapturederror', event => probeErrors.push(event.error.message)); }
    const texture = createTexture.call(this, variant === 'depth24' && descriptor.format === 'depth32float' ? {...descriptor, format:'depth24plus'} : descriptor);
    if (descriptor.label === 'cached material texture') probeTextures.push(texture);
    return texture;
  };
  const createShader = GPUDevice.prototype.createShaderModule;
  GPUDevice.prototype.createShaderModule = function (descriptor) {
    let code = descriptor.code;
    if (code.includes('fn shade_fragment')) {
      if (variant === 'coarse' || variant === 'fine') {
        const suffix = variant === 'coarse' ? 'Coarse' : 'Fine';
        code = code.replaceAll('dpdx(', `dpdx${suffix}(`).replaceAll('dpdy(', `dpdy${suffix}(`);
      }
      if (fixedLight) {
        // Override only scene-light inputs; retain the renderer's actual PBR code.
        code = code.replace('fn light_count()->u32 {if LIGHT_COUNT<0 {return u32(u.material.w);}return u32(LIGHT_COUNT);}', 'fn light_count()->u32 {return 1u;}');
        code = code.replace('fn light_type(i:u32)->f32 {if LIGHT_COUNT<0 {return u.light_position[i].w;}return f32((LIGHT_TYPES>>(i*3u))&7u);}', 'fn light_type(i:u32)->f32 {return 0.0;}');
        code = code.replaceAll('u.light_position[i].xyz', 'normalize(vec3(-3.0,4.0,-2.0))');
        code = code.replaceAll('u.light_color[i].xyz', 'vec3(3.0)');
      }

      if (mode === 'solid') code = code.replace('let color=shade_fragment(in,front);', 'let color=vec4(0.5,0.5,0.5,1.0);');
      if (mode === 'basic') code = code.replace('if material_kind()<0.5', 'if true');
      if (mode === 'normal') code = code.replace('if material_kind()==8.0', 'return vec4(n*0.5+0.5,1.0); if material_kind()==8.0');
      if (mode === 'roughness') code = code.replace('let metalness=', 'return vec4(vec3(roughness),1.0); let metalness=');
      if (mode === 'radiance') code = code.replace('let irradiance=environment_sample(n,1.0);', 'return vec4(radiance,1.0); let irradiance=environment_sample(n,1.0);');
    }
    return createShader.call(this, { ...descriptor, code });
  };
  const createPipeline = GPUDevice.prototype.createRenderPipeline;
  GPUDevice.prototype.createRenderPipeline = function (descriptor) {
    if (variant === 'depth24' && descriptor.depthStencil?.format === 'depth32float') descriptor = {...descriptor, depthStencil:{...descriptor.depthStencil,format:'depth24plus'}};
    if (mode === 'noMR' && descriptor.fragment?.constants) descriptor.fragment.constants.MR_MAP = 0;
    return createPipeline.call(this, descriptor);
  };
}

async function configureGL({ mode, flip, hdr, background, fixedLight }) {
  const T = await import('/.cache/three-r186/src/Three.js');
  const { scene, renderer, camera } = reference;
  if (hdr === false) scene.environment = null;
  if (background === false) scene.background = new T.Color(0);
  if (fixedLight) {
    const light = new T.DirectionalLight(0xffffff, 3);
    light.position.set(-3, 4, -2);
    scene.add(light);
  }
  scene.traverse(object => {
    if (!object.isMesh) return;
    const material = object.material;
    // Reflecting the projection reverses the derivative-built tangent frame.
    if (flip) material.normalScale.multiplyScalar(-1);
    if (mode === 'noMR') material.metalnessMap = material.roughnessMap = null;
    if (mode === 'solid') object.material = new T.MeshBasicMaterial({ color: new T.Color().setRGB(.5, .5, .5) });
    if (mode === 'basic') object.material = new T.MeshBasicMaterial({ color: material.color, map: material.map });
    if (['normal', 'roughness', 'radiance'].includes(mode)) {
      const value = mode === 'normal' ? 'normal*0.5+0.5' : mode === 'roughness' ? 'vec3(material.roughness)' : 'radiance';
      material.onBeforeCompile = shader => {
        shader.fragmentShader = shader.fragmentShader.replace('#include <opaque_fragment>', `gl_FragColor=vec4(${value},1.0);`);
      };
    }
    material.needsUpdate = true;
  });
  const gl = renderer.getContext();
  const actualSamples = gl.getParameter(gl.SAMPLES);
  if (flip) {
    camera.projectionMatrix.elements[5] *= -1;
    camera.projectionMatrixInverse.copy(camera.projectionMatrix).invert();
    const frontFace = gl.frontFace.bind(gl);
    gl.frontFace = value => frontFace(value === gl.CW ? gl.CCW : gl.CW);
    renderer.state.reset();
  }
  await renderFixture();
  return actualSamples;
}

function readGLMips() {
  let mesh;
  reference.scene.traverse(object => { if (object.isMesh) mesh = object; });
  const texture = mesh.material.metalnessMap;
  const renderer = reference.renderer, gl = renderer.getContext();
  const framebuffer = gl.createFramebuffer(), result = [];
  gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
  for (let level = 0; level <= Math.log2(texture.image.width); level++) {
    const width = Math.max(1, texture.image.width >> level), height = Math.max(1, texture.image.height >> level);
    const bytes = new Uint8Array(width * height * 4);
    gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, renderer.properties.get(texture).__webglTexture, level);
    gl.readPixels(0, 0, width, height, gl.RGBA, gl.UNSIGNED_BYTE, bytes);
    if (gl.getError()) throw new Error(`Mip ${level} readback failed`);
    let binary = '';
    for (let i = 0; i < bytes.length; i += 8192) binary += String.fromCharCode(...bytes.subarray(i, i + 8192));
    result.push({ level, width, height, data: btoa(binary) });
  }
  gl.deleteFramebuffer(framebuffer);
  return result;
}

async function replaceRustMips(mips) {
  const texture = probeTextures[1]; // Pinned fixture's metallic/roughness texture.
  if (texture.width !== 2048 || texture.format !== 'rgba8unorm') throw new Error('Unexpected MR texture');
  const differences = [];
  for (const mip of mips) {
    const expected = Uint8Array.from(atob(mip.data), c => c.charCodeAt(0));
    const stride = Math.ceil(mip.width * 4 / 256) * 256;
    const buffer = probeDevice.createBuffer({ size: stride * mip.height, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
    const encoder = probeDevice.createCommandEncoder();
    encoder.copyTextureToBuffer({ texture, mipLevel: mip.level }, { buffer, bytesPerRow: stride }, { width: mip.width, height: mip.height });
    probeDevice.queue.submit([encoder.finish()]);
    await buffer.mapAsync(GPUMapMode.READ);
    const actual = new Uint8Array(buffer.getMappedRange());
    let count = 0, max = 0;
    for (let y = 0; y < mip.height; y++) for (let x = 0; x < mip.width * 4; x++) {
      const difference = Math.abs(actual[y * stride + x] - expected[y * mip.width * 4 + x]);
      count += difference > 0;
      max = Math.max(max, difference);
    }
    differences.push({ level: mip.level, fraction: count / expected.length, max });
    buffer.unmap(); buffer.destroy();
    probeDevice.queue.writeTexture({ texture, mipLevel: mip.level }, expected, { bytesPerRow: mip.width * 4 }, { width: mip.width, height: mip.height });
  }
  await probeDevice.queue.onSubmittedWorkDone();
  return differences;
}

function flipRows(png) {
  for (let y = 0; y < png.height / 2; y++) {
    const top = Buffer.from(png.data.subarray(y * png.width * 4, (y + 1) * png.width * 4));
    png.data.copy(png.data, y * png.width * 4, (png.height - 1 - y) * png.width * 4, (png.height - y) * png.width * 4);
    top.copy(png.data, (png.height - 1 - y) * png.width * 4);
  }
}

try {
  for (const scenario of cases) {
    const images = {}, result = { ...scenario };
    let mips;
    for (const runtime of ['gl', 'rust']) {
      const page = await browser.newPage({ viewport: { width: 512, height: 512 } });
      await page.route('**/favicon.ico', route => route.fulfill({ status: 204 }));
      const errors = [];
      page.on('pageerror', error => errors.push(String(error)));
      page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
      if (runtime === 'gl' && referenceBackend === 'webgpu') {
        await page.addInitScript(() => {
          window.referencePipelines = [];
          const create = GPUDevice.prototype.createRenderPipeline;
          GPUDevice.prototype.createRenderPipeline = function (descriptor) {
            referencePipelines.push({samples:descriptor.multisample?.count || 1, formats:descriptor.fragment?.targets.map(t=>t.format)});
            return create.call(this,descriptor);
          };
        });
      }
      if (runtime === 'rust') {
        await page.route('**/catalog.json', route => route.fulfill({ json: catalog }));
        // Playwright does not guarantee the order of multiple init scripts.
        const flipScript = scenario.rustFlip ? readFileSync('tools/gltf_examples/rust_msaa_flip.js','utf8') : '';
        await page.addInitScript({content:`(${instrumentRust.toString()})(${JSON.stringify(scenario)});\n${flipScript}`});
      }
      await page.goto(base + (runtime === 'gl'
        ? `/reference/three-js/expanded.html?id=${id}&samples=${scenario.samples}&backend=${referenceBackend}`
        : `/web/gallery/example.html?id=${id}`));
      await page.waitForFunction(() => document.querySelector('canvas')?.dataset.ready === 'true' || Number(document.querySelector('canvas')?.dataset.frames) > 0, null, { timeout: 90000 });
      if (runtime === 'gl') {
        if (referenceBackend === 'webgpu') {
          result.reference = await page.evaluate(async () => {
            await renderFixture();
            const renderer = reference.renderer;
            if (!renderer.backend.isWebGPUBackend || !renderer.backend.device) throw Error('Reference is not WebGPU');
            await renderer.backend.device.queue.onSubmittedWorkDone();
            return {backend:'webgpu', samples:renderer.samples || 1, pipelines:[...new Map(referencePipelines.map(p=>[JSON.stringify(p),p])).values()]};
          });
          if (result.reference.samples !== scenario.samples) throw Error('Reference MSAA sample count differs');
        } else result.glSamples = await page.evaluate(configureGL, scenario);
        if (scenario.mode === 'copyMR') mips = await page.evaluate(readGLMips);
      } else {
        if (mips) result.mips = await page.evaluate(replaceRustMips, mips);
        await page.evaluate(({samples, hdr, background}) => {
          if (hdr !== undefined) app.gltf_controls(1, hdr ? 1 : 0, 0, 0, true);
          if (background === false) app.background_intensity(0);
          app.set_samples(samples);
          app.request_render();
        }, scenario);
      }
      await page.addStyleTag({ content: '#notice,#settings{display:none!important}' });
      await page.waitForTimeout(150);
      if (runtime === 'rust') {
        errors.push(...await page.evaluate(() => probeErrors));
        if (scenario.rustFlip) {
          result.shaderPatches = await page.evaluate(() => msaaFlip);
          if (Object.values(result.shaderPatches).some(n => n === 0)) throw Error('Incomplete MSAA reflection');
        }
      }
      images[runtime] = PNG.sync.read(await page.locator('canvas').screenshot());
      if (runtime === 'gl' && scenario.flip) flipRows(images[runtime]);
      writeFileSync(`${output}/${scenario.name}-${runtime === 'gl' && referenceBackend === 'webgpu' ? 'three-webgpu' : runtime}.png`, PNG.sync.write(images[runtime]));
      await page.close();
      if (errors.length) throw new Error(errors.join('\n'));
    }
    let count = 0, sum = 0, max = 0;
    for (let i = 0; i < images.gl.data.length; i += 4) {
      let largest = 0;
      for (let c = 0; c < 3; c++) {
        const difference = Math.abs(images.gl.data[i + c] - images.rust.data[i + c]);
        sum += difference; largest = Math.max(largest, difference);
      }
      count += largest > 6; max = Math.max(max, largest);
    }
    Object.assign(result, { fraction: count / (512 * 512), mean: sum / (512 * 512 * 3), max });
    results.cases.push(result);
    console.log(JSON.stringify(result));
    writeFileSync(`${output}/results.json`, JSON.stringify(results, null, 2) + '\n');
  }
} finally {
  await browser.close();
}
