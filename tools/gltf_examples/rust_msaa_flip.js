// Instancing-only experiment. Reflect the offscreen scene, then undo it during
// presentation. No extra pass, CPU image manipulation, or sample-count reduction.
(() => {
  const meshModules = new WeakSet();
  window.msaaFlip = { mesh: 0, background: 0, present: 0, pipelines: 0 };
  function replace(code, before, after) {
    if (!(typeof before === 'string' ? code.includes(before) : before.test(code))) throw new Error(`MSAA experiment: missing shader anchor ${before}`);
    return code.replace(before, after);
  }
  const createShader = GPUDevice.prototype.createShaderModule;
  GPUDevice.prototype.createShaderModule = function (descriptor) {
    let code = descriptor.code;
    const mesh = code.includes('fn shade_fragment');
    if (mesh) {
      code = replace(code, '    return out;\n}\nfn map_uv', '    out.clip.y=-out.clip.y;\n    return out;\n}\nfn map_uv');
      // Projection reflection reverses screen Y derivatives. Restore the same
      // view-space cotangent frame; this model has derivative normal mapping.
      code = replace(code, /let q1=-dpdy(Fine|Coarse)?\(in.view_position\);/, 'let q1=dpdy$1(in.view_position);');
      code = replace(code, /let st1=-dpdy(Fine|Coarse)?\(normal_uv\);/, 'let st1=dpdy$1(normal_uv);');
      msaaFlip.mesh++;
    } else if (code.includes('fn background_color')) {
      code = replace(code, 'u.inverse_projection*vec4(in.ndc,1.0,1.0)', 'u.inverse_projection*vec4(in.ndc*vec2(1.0,-1.0),1.0,1.0)');
      msaaFlip.background++;
    } else if (code.includes('textureLoad(source,vec2<i32>(p.xy),0)')) {
      code = replace(code, 'textureLoad(source,vec2<i32>(p.xy),0)', 'textureLoad(source,vec2(i32(p.x),i32(textureDimensions(source).y)-1-i32(p.y)),0)');
      msaaFlip.present++;
    }
    const module = createShader.call(this, { ...descriptor, code });
    if (mesh) meshModules.add(module);
    return module;
  };
  const createPipeline = GPUDevice.prototype.createRenderPipeline;
  GPUDevice.prototype.createRenderPipeline = function (descriptor) {
    if (meshModules.has(descriptor.vertex.module)) {
      descriptor = { ...descriptor, primitive: { ...descriptor.primitive,
        frontFace: descriptor.primitive?.frontFace === 'cw' ? 'ccw' : 'cw' } };
      msaaFlip.pipelines++;
    }
    return createPipeline.call(this, descriptor);
  };
})();
