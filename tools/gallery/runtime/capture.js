// Diagnostic bridge only: the product never executes upstream Three.js scenes.
// Collect a real frame, then let the Rust probe reject unsupported semantics or render it.
(() => {
 const state=window.__galleryProbe={frame:null,blocked:[],pending:0,changed:performance.now()};
 state.reached=(scene,camera,renderer)=>{if(scene&&camera)state.frame={scene,camera,renderer};};
 state.unsupported=feature=>{if(!state.blocked.includes(feature))state.blocked.push(feature);throw new Error(`Rust port prerequisite: ${feature}`);};
 state.snapshot=()=>{
  const blockers=[...new Set(state.blocked)];
  if(!state.frame)return {blockers:blockers.length?blockers:['No scene reached the renderer']};
  const {scene,camera,renderer}=state.frame;
  scene.updateMatrixWorld(true);camera.updateMatrixWorld(true);
  const objects=[],lights=[],textures=[],textureIds=new Map();
  const add=message=>{if(!blockers.includes(message))blockers.push(message);};
  const color=c=>c?[c.r,c.g,c.b]:[0,0,0];
  if(camera.isArrayCamera||(!camera.isPerspectiveCamera&&!camera.isOrthographicCamera))add('Camera type: '+camera.type);
  if(renderer.xr?.enabled)add('WebXR session integration');
  if(scene.fog)add('Fog: '+(scene.fog.isFogExp2?'exponential':'linear'));
  if(scene.backgroundNode)add('Programmable background node');
  if(scene.environmentNode)add('Programmable environment node');
  if(renderer.clippingPlanes?.length||renderer.localClippingEnabled)add('Clipping planes');
  if(renderer.logarithmicDepthBuffer)add('Logarithmic depth');
  if(![0,4].includes(renderer.toneMapping))add('Tone mapping mode: '+renderer.toneMapping);
  if(renderer.outputColorSpace&&renderer.outputColorSpace!=='srgb')add('Output color space: '+renderer.outputColorSpace);
  let environment=scene.environment||null;
  const materials=[];
  const knownMaterials=new Set(['MeshBasicMaterial','MeshStandardMaterial','MeshPhysicalMaterial','LineBasicMaterial','PointsMaterial','MeshBasicNodeMaterial','MeshStandardNodeMaterial','MeshPhysicalNodeMaterial','LineBasicNodeMaterial','PointsNodeMaterial']);
  scene.traverseVisible(object=>{
   if(object.isLight){lights.push(object);if(!object.isAmbientLight&&!object.isDirectionalLight&&!object.isPointLight)add('Light type: '+object.type);return;}
   if(object.isSkinnedMesh)add('Skinned mesh / skeleton');
   if(object.isInstancedMesh||object.isBatchedMesh)add('Instance/batch transforms: '+object.type);
   if(object.isSprite)add('Sprite rendering');
   if(!object.geometry||!object.material)return;
   if(!object.isMesh&&!object.isLine&&!object.isPoints)add('Object type: '+object.type);
   if(object.geometry.isInstancedBufferGeometry)add('Instanced custom attributes');
   if(Object.keys(object.geometry.morphAttributes||{}).length)add('Morph target evaluation');
   if(renderer.shadowMap?.enabled&&(object.castShadow||object.receiveShadow))add('Shadow maps');
   for(const m of Array.isArray(object.material)?object.material:[object.material]){
    if(!m.isMeshBasicMaterial&&!m.isMeshStandardMaterial&&!m.isMeshPhysicalMaterial&&!m.isLineBasicMaterial&&!m.isPointsMaterial&&!['MeshBasicNodeMaterial','MeshStandardNodeMaterial','MeshPhysicalNodeMaterial','LineBasicNodeMaterial','PointsNodeMaterial'].includes(m.type))add('Material type: '+m.type);
    if(!knownMaterials.has(m.constructor.name))add('Custom material class: '+m.constructor.name);
    if(state.Material&&m.onBeforeCompile!==state.Material.prototype.onBeforeCompile)add('Custom shader onBeforeCompile hook');
    if(m.alphaHash)add('Stochastic alpha hashing');
    for(const [key,value] of Object.entries(m))if(key.endsWith('Node')&&value?.isNode)add('Programmable material: '+key);
    for(const key of ['transmission','clearcoat','sheen','iridescence','anisotropy','dispersion'])if(m[key]>0)add('Physical material extension: '+key);
    if(m.ior!==undefined&&m.ior!==1.5)add('Non-default material IOR');
    if(m.specularIntensity!==undefined&&m.specularIntensity!==1)add('Physical specular intensity');
    if(m.wireframe)add('Wireframe material');
    if(m.flatShading&&!m.isMeshBasicMaterial)add('Flat shaded lit material');
    if(m.stencilWrite)add('Stencil operations');
    if(m.clippingPlanes?.length)add('Material clipping planes');
    if(![0,1].includes(m.blending))add('Blend mode: '+m.blending);
    if(m.polygonOffset)add('Polygon offset');
    if(m.normalMapType===1)add('Object-space normal map');
    for(const key of ['lightMap','bumpMap','displacementMap','alphaMap','specularMap'])if(m[key])add('Material map: '+key);
    if((m.roughnessMap||m.metalnessMap)&&m.roughnessMap!==m.metalnessMap)add('Separate roughness and metalness textures');
    if(m.isLineDashedMaterial)add('Dashed lines');
    if(m.alphaToCoverage)add('Alpha-to-coverage');
    if(m.premultipliedAlpha)add('Premultiplied alpha material');
    if(m.envMap){if(m.isMeshBasicMaterial||m.isMeshBasicNodeMaterial)add('Unlit environment reflection/refraction');if(environment&&environment!==m.envMap)add('Multiple material environments');environment=m.envMap;}
    materials.push(m);
   }
   objects.push(object);
  });
  if(lights.filter(l=>!l.isAmbientLight&&l.intensity>0).length>8)add('More than eight non-ambient lights');
  if(objects.length>2000)add('Probe resource limit: more than 2000 objects');
  const vertices=objects.reduce((n,o)=>n+(o.geometry.attributes.position?.count||0),0);
  if(vertices>1000000)add('Probe resource limit: more than 1000000 vertices');
  if(scene.background?.isTexture){if(environment&&environment!==scene.background)add('Independent background and lighting environments');environment=scene.background;}
  const dimensions=image=>[image?.width||image?.videoWidth||0,image?.height||image?.videoHeight||0];
  let pixelBudget=0;
  function pixels(texture,env=false){
   const image=texture.image;const [width,height]=dimensions(image);
   if(Array.isArray(image)||texture.isCubeTexture)throw new Error('Cube texture import');
   if(!width||!height)throw new Error('Texture image is not loaded or is GPU-only');
   pixelBudget+=width*height;
   if(width*height>8388608||pixelBudget>8388608)throw new Error('Probe resource limit: combined texture data exceeds 8M pixels');
   let rgba;
   if(image.data){
    if(image.data.length!==width*height*4)throw new Error('Non-RGBA data texture');
    if(!env&&!(image.data instanceof Uint8Array)&&!(image.data instanceof Uint8ClampedArray))throw new Error('Float/integer material texture');
    const half=h=>{const s=h&32768?-1:1,e=(h>>10)&31,f=h&1023;return s*(e===0?f*Math.pow(2,-24):(1+f/1024)*Math.pow(2,e-15));};
    rgba=Array.from(image.data,v=>env?(image.data instanceof Uint16Array?half(v):image.data instanceof Uint8Array?v/255:v):v);
   }else{
    const canvas=document.createElement('canvas');canvas.width=width;canvas.height=height;
    const context=canvas.getContext('2d');context.drawImage(image,0,0);rgba=Array.from(context.getImageData(0,0,width,height).data);
    if(env)rgba=rgba.map((v,i)=>i%4===3?v/255:texture.colorSpace==='srgb'?(v/255<=0.04045?v/3294.6:Math.pow((v/255+0.055)/1.055,2.4)):v/255);
   }
   return {width,height,rgba};
  }
  function textureId(t){
   if(!t)return null;if(textureIds.has(t))return textureIds.get(t);
   if(t.channel!==0)throw new Error('Texture UV channel '+t.channel);
   if(t.mapping!==300)throw new Error('Material texture mapping '+t.mapping);
   const data=pixels(t);const id=textures.length;textureIds.set(t,id);
   textures.push({...data,srgb:t.colorSpace==='srgb',flip_y:t.flipY,wrap_s:t.wrapS,wrap_t:t.wrapT,mag:t.magFilter,min:t.minFilter,anisotropy:t.anisotropy,offset:t.offset.toArray(),repeat:t.repeat.toArray(),center:t.center.toArray(),rotation:t.rotation});return id;
  }
  let env=null;const output=[];
  if(!blockers.length)try{
   if(environment)env=pixels(environment,true);
   for(const object of objects){
    const g=object.geometry;
    const attribute=a=>a?Array.from({length:a.count*a.itemSize},(_,i)=>a.getComponent(Math.floor(i/a.itemSize),i%a.itemSize)):null;
    const mats=(Array.isArray(object.material)?object.material:[object.material]).map(m=>({
     kind:m.isMeshStandardMaterial||m.isMeshPhysicalMaterial||['MeshStandardNodeMaterial','MeshPhysicalNodeMaterial'].includes(m.type)?'standard':object.isPoints?'points':object.isLine?'line':'basic',
     color:color(m.color),opacity:m.opacity,transparent:m.transparent,side:m.side,alpha_test:m.alphaTest,depth_test:m.depthTest,depth_write:m.depthWrite,vertex_colors:m.vertexColors,
     roughness:m.roughness??1,metalness:m.metalness??0,emissive:color(m.emissive).map(v=>v*(m.emissiveIntensity??1)),normal_scale:m.normalScale?.toArray()||[1,1],ao_intensity:m.aoMapIntensity??1,
     map:textureId(m.map),mr_map:textureId(m.roughnessMap),normal_map:textureId(m.normalMap),emissive_map:textureId(m.emissiveMap),ao_map:textureId(m.aoMap),size:m.size??1,size_attenuation:m.sizeAttenuation??true
    }));
    output.push({kind:object.isPoints?'points':object.isLineSegments?'segments':object.isLineLoop?'loop':object.isLine?'line':'mesh',matrix:object.matrixWorld.elements,position:attribute(g.attributes.position),normal:attribute(g.attributes.normal),uv:attribute(g.attributes.uv),color:attribute(g.attributes.color),color_size:g.attributes.color?.itemSize||3,tangent:attribute(g.attributes.tangent),index:g.index?Array.from(g.index.array):null,groups:g.groups,draw_start:g.drawRange.start,draw_count:Number.isFinite(g.drawRange.count)?g.drawRange.count:null,materials:mats});
   }
  }catch(error){add(String(error.message));}
  if(blockers.length)return {blockers,observed:{objects:objects.length,vertices,lights:lights.length}};
  return {blockers,objects:output,textures,environment:env,observed:{objects:objects.length,vertices,lights:lights.length},background:scene.background?.isColor?color(scene.background):[0,0,0],background_environment:!!scene.background?.isTexture,background_blur:scene.backgroundBlurriness||0,background_intensity:scene.backgroundIntensity??1,environment_intensity:scene.environmentIntensity??1,exposure:renderer.toneMappingExposure,aces:renderer.toneMapping===4,
   camera:{kind:camera.isOrthographicCamera?'orthographic':'perspective',matrix:camera.matrixWorld.elements,fov:camera.fov??50,aspect:camera.aspect??1,near:camera.near,far:camera.far,zoom:camera.zoom,left:camera.left??-1,right:camera.right??1,top:camera.top??1,bottom:camera.bottom??-1},
   lights:lights.map(l=>({kind:l.isAmbientLight?'ambient':l.isDirectionalLight?'directional':'point',color:color(l.color),intensity:l.intensity,position:l.getWorldPosition(l.position.clone()).toArray(),target:l.target?l.target.getWorldPosition(l.position.clone()).toArray():[0,0,0],distance:l.distance??0,decay:l.decay??2}))};
 };
})();
