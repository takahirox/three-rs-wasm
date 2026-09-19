// Measure actual raster sample coverage without glTF, textures, lighting or HDR.
import { chromium } from '@playwright/test';
import { mkdirSync, writeFileSync } from 'node:fs';
const output = process.env.OUTPUT || '.cache/instancing-investigation/msaa';
mkdirSync(output, { recursive: true });
const browser = await chromium.launch({
  executablePath: process.env.CHROME || '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
  args: ['--enable-unsafe-webgpu'],
});
try {
  const page = await browser.newPage();
  const url = (process.env.BASE_URL || 'http://127.0.0.1:8173') + '/__msaa_probe__';
  await page.route(url, route => route.fulfill({ contentType: 'text/html', body: '<!doctype html><title>MSAA probe</title>' }));
  await page.goto(url);
  const result = await page.evaluate(async () => {
    const canvas = document.createElement('canvas');
    canvas.width = canvas.height = 8;
    const gl = canvas.getContext('webgl2', { antialias: true, alpha: false, preserveDrawingBuffer: true });
    if (!gl) throw Error('WebGL2 unavailable');
    const vs = `#version 300 es
    precision highp float;
    uniform vec4 params;out vec2 coord;
    void main(){vec2 corners[6]=vec2[6](vec2(0,0),vec2(1,0),vec2(0,1),vec2(0,1),vec2(1,0),vec2(1,1));
    vec2 p=corners[gl_VertexID]*(vec2(3)+params.xy);coord=p-vec2(3);gl_Position=vec4(p.x/4.-1.,1.-p.y/4.,0,1);}`;
    const fs = `#version 300 es
    precision highp float;
    uniform vec4 params;in vec2 coord;out vec4 color;
    void main(){color=params.w>0.5?vec4(coord,0.,1.):vec4(vec3(params.z),1.);}`;
    function shader(type, source) {
      const shader = gl.createShader(type);gl.shaderSource(shader,source);gl.compileShader(shader);
      if (!gl.getShaderParameter(shader,gl.COMPILE_STATUS)) throw Error(gl.getShaderInfoLog(shader));
      return shader;
    }
    const program=gl.createProgram();gl.attachShader(program,shader(gl.VERTEX_SHADER,vs));gl.attachShader(program,shader(gl.FRAGMENT_SHADER,fs));gl.linkProgram(program);
    if (!gl.getProgramParameter(program,gl.LINK_STATUS)) throw Error(gl.getProgramInfoLog(program));
    gl.useProgram(program);gl.disable(gl.DITHER);gl.viewport(0,0,8,8);gl.clearColor(0,0,0,1);
    const location=gl.getUniformLocation(program,'params');
    const offscreen=gl.createFramebuffer(),glResolve=gl.createFramebuffer();
    const colorBuffer=gl.createRenderbuffer();gl.bindRenderbuffer(gl.RENDERBUFFER,colorBuffer);gl.renderbufferStorageMultisample(gl.RENDERBUFFER,4,gl.RGBA8,8,8);
    gl.bindFramebuffer(gl.FRAMEBUFFER,offscreen);gl.framebufferRenderbuffer(gl.FRAMEBUFFER,gl.COLOR_ATTACHMENT0,gl.RENDERBUFFER,colorBuffer);
    const colorTexture=gl.createTexture();gl.bindTexture(gl.TEXTURE_2D,colorTexture);gl.texStorage2D(gl.TEXTURE_2D,1,gl.RGBA8,8,8);
    gl.bindFramebuffer(gl.FRAMEBUFFER,glResolve);gl.framebufferTexture2D(gl.FRAMEBUFFER,gl.COLOR_ATTACHMENT0,gl.TEXTURE_2D,colorTexture,0);gl.bindFramebuffer(gl.FRAMEBUFFER,null);

    const adapter=await navigator.gpu.requestAdapter();if(!adapter)throw Error('WebGPU unavailable');
    const device=await adapter.requestDevice();const errors=[];device.addEventListener('uncapturederror',e=>errors.push(e.error.message));
    const module=device.createShaderModule({code:`
    @group(0) @binding(0) var<uniform> params:vec4<f32>;
    struct Out {@builtin(position) position:vec4<f32>,@location(0) coord:vec2<f32>};
    @vertex fn vs(@builtin(vertex_index) i:u32)->Out {
      let corners=array<vec2<f32>,6>(vec2(0.,0.),vec2(1.,0.),vec2(0.,1.),vec2(0.,1.),vec2(1.,0.),vec2(1.,1.));
      let p=corners[i]*(vec2(3.)+params.xy);var out:Out;out.position=vec4(p.x/4.-1.,1.-p.y/4.,0.,1.);out.coord=p-vec2(3.);return out;
    }
    @fragment fn fs(in:Out)->@location(0) vec4<f32>{if params.w>0.5{return vec4(in.coord,0.,1.);}return vec4(vec3(params.z),1.);}`});
    const pipeline=device.createRenderPipeline({layout:'auto',vertex:{module,entryPoint:'vs'},fragment:{module,entryPoint:'fs',targets:[{format:'rgba8unorm'}]},multisample:{count:4}});
    const uniform=device.createBuffer({size:16,usage:GPUBufferUsage.UNIFORM|GPUBufferUsage.COPY_DST});
    const bindings=device.createBindGroup({layout:pipeline.getBindGroupLayout(0),entries:[{binding:0,resource:{buffer:uniform}}]});
    const ms=device.createTexture({size:[8,8],format:'rgba8unorm',sampleCount:4,usage:GPUTextureUsage.RENDER_ATTACHMENT});
    const resolved=device.createTexture({size:[8,8],format:'rgba8unorm',usage:GPUTextureUsage.RENDER_ATTACHMENT|GPUTextureUsage.COPY_SRC});
    const msView=ms.createView(),resolvedView=resolved.createView();
    const readback=device.createBuffer({size:256,usage:GPUBufferUsage.COPY_DST|GPUBufferUsage.MAP_READ});
    async function pixel(x,y,gray=1,mode=0,background=0){
      const p=new Float32Array([x,y,gray,mode]);gl.uniform4fv(location,p);gl.clearColor(background,background,background,1);gl.clear(gl.COLOR_BUFFER_BIT);gl.drawArrays(gl.TRIANGLES,0,6);
      const a=new Uint8Array(4);gl.readPixels(3,4,1,1,gl.RGBA,gl.UNSIGNED_BYTE,a);
      gl.bindFramebuffer(gl.FRAMEBUFFER,offscreen);gl.clear(gl.COLOR_BUFFER_BIT);gl.drawArrays(gl.TRIANGLES,0,6);
      gl.bindFramebuffer(gl.DRAW_FRAMEBUFFER,glResolve);gl.blitFramebuffer(0,0,8,8,0,0,8,8,gl.COLOR_BUFFER_BIT,gl.NEAREST);
      gl.bindFramebuffer(gl.READ_FRAMEBUFFER,glResolve);const off=new Uint8Array(4);gl.readPixels(3,4,1,1,gl.RGBA,gl.UNSIGNED_BYTE,off);gl.bindFramebuffer(gl.FRAMEBUFFER,null);
      device.queue.writeBuffer(uniform,0,p);const encoder=device.createCommandEncoder();
      const pass=encoder.beginRenderPass({colorAttachments:[{view:msView,resolveTarget:resolvedView,loadOp:'clear',storeOp:'discard',clearValue:[background,background,background,1]}]});
      pass.setPipeline(pipeline);pass.setBindGroup(0,bindings);pass.draw(6);pass.end();
      encoder.copyTextureToBuffer({texture:resolved,origin:[3,3]},{buffer:readback,bytesPerRow:256},{width:1,height:1});
      device.queue.submit([encoder.finish()]);await readback.mapAsync(GPUMapMode.READ);const b=Array.from(new Uint8Array(readback.getMappedRange()).slice(0,4));readback.unmap();
      if(gl.getError())throw Error('WebGL error');if(errors.length)throw Error(errors.join('\n'));
      return {gl:Array.from(a),glOffscreen:Array.from(off),gpu:b};
    }
    const grids={gl:[],glOffscreen:[],gpu:[]};
    for(let y=0;y<=16;y++){const rows={gl:[],glOffscreen:[],gpu:[]};for(let x=0;x<=16;x++){
      // Offset exceeds raster subpixel quantization but is smaller than the sweep step.
      const p=await pixel(x/16+.01,y/16+.01);for(const backend of ['gl','glOffscreen','gpu'])rows[backend].push(Math.round(p[backend][0]*4/255));
    }for(const backend of ['gl','glOffscreen','gpu'])grids[backend].push(rows[backend]);}
    const positions={};for(const backend of ['gl','glOffscreen','gpu']){positions[backend]=[];const a=grids[backend];for(let y=1;y<=16;y++)for(let x=1;x<=16;x++){
      const n=a[y][x]-a[y-1][x]-a[y][x-1]+a[y-1][x-1];if(n)positions[backend].push({x:x/16,y:y/16,count:n,xInterval:[(x-1)/16+.01,x/16+.01],yInterval:[(y-1)/16+.01,y/16+.01]});
    }}
    const probes=[];for(const [x,y]of [[1,1],[.5,1],[1,.5],[.5,.5],[.25,1],[1,.25],[.5,.25],[.25,.5]]){
      probes.push({x,y,coverage:await pixel(x,y),encodedGray:await pixel(x,y,.5),interpolated:await pixel(x,y,1,1)});
    }
    const resolveStress={cases:0,changed:0,max:0,examples:[]};
    for(const dither of [false,true]){if(dither)gl.enable(gl.DITHER);else gl.disable(gl.DITHER);
      for(const background of [0,.2,.5,.8,1])for(const [x,y]of [[1,1],[.5,1],[.5,.5]])for(let i=0;i<=32;i++){
        const gray=Math.min(1,(i*8+.37)/255),p=await pixel(x,y,gray,0,background);
        const error=Math.abs(p.gl[0]-p.gpu[0]);resolveStress.cases++;resolveStress.changed+=error!==0;resolveStress.max=Math.max(resolveStress.max,error);
        if(error&&resolveStress.examples.length<8)resolveStress.examples.push({dither,background,x,y,gray,gl:p.gl[0],gpu:p.gpu[0],error});
      }
    }
    const result={coordinateConvention:'Top-left of pixel is (0,0), screen x right, y down; coverage boundary located to 1/16 pixel.',glSamples:gl.getParameter(gl.SAMPLES),glAttributes:gl.getContextAttributes(),gpuAdapter:{vendor:adapter.info.vendor,architecture:adapter.info.architecture,description:adapter.info.description},positions,grids,probes,resolveStress};
    device.destroy();return result;
  });
  result.browser=browser.version();
  writeFileSync(`${output}/results.json`,JSON.stringify(result,null,2)+'\n');
  console.log(JSON.stringify({browser:result.browser,samples:result.glSamples,positions:result.positions,probes:result.probes},null,2));
} finally { await browser.close(); }
