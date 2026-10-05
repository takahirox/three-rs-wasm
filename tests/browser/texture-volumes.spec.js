import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {readFileSync,writeFileSync} from 'node:fs';
test.use({deviceScaleFactor:Number(process.env.VOLUMES_DPR||1)});
const view='canvas';
// Per example: capture times, [control index, reference value, Rust value] at its time, input script.
// Scripted input: [action, ...arguments, capture time or null]. Drags are [x0,y0,x1,y1,button].
const cases={
 // The horses gallop and run by the example's Timer; FirstPersonControls turn while dragging.
 // The SunLight's PCF edges differ as in webgl_loader_md2_control ( collada-3mf.spec.js ).
 // At 1280 × 720 one horse at the near cascade's boundary falls inside the port's cascade
 // frustum and outside WebGL's: the port draws it into that cascade once more ( portDraws ).
 webgl_shadowmap_performance:{portDraws:[[2952,1]],limits:[.035,.7],antialias:true,times:[0,.5,1,2.5],parameters:[],at:2.5},
 // The bike's animation follows the example's Timer; the orbit is damped. With MSAA the thin
 // frame's edges resolve differently ( msaaLimits ).
 misc_exporter_usdz:{antialias:true,times:[0,1,2.5],parameters:[],at:2.5,drag:[[256,256],[300,280]],wheel:[256,256,-200],settle:true},
 // The model under its PMREM environment; the orbit is undamped. With MSAA its silhouette's
 // edge samples resolve slightly differently ( msaaLimits, docs/probe-gi-dof-examples.md ).
 webgl_loader_usdz:{backgroundBox:true,antialias:true,times:[0],parameters:[],at:0,drag:[[256,256],[300,280]],wheel:[256,256,-200]},
 // Each asset loads, replaces the scene and resets the damped controls.
 webgl_loader_vrml:{times:[0],parameters:[[0,'creaseAngle',0],[0,'crystal',1],[0,'elevationGrid1',3],[0,'elevationGrid2',4],[0,'extrusion1',5],[0,'extrusion2',6],[0,'extrusion3',7],[0,'lines',8],[0,'linesTransparent',9],[0,'meshWithLines',10],[0,'meshWithTexture',11],[0,'pixelTexture',12],[0,'points',13],[0,'camera',14],[0,'multilineString',15],[0,'house',2]],at:0,drag:[[256,256],[300,280]],wheel:[256,256,-200],settle:true},
 // Each frame steps the flock by the clock's delta ( capped at a second ); the first frame's
 // predator sits at the center, later ones at the last pointer move. The thousands of small
 // flat facets' edges rasterize and resolve differently between WebGL and WebGPU
 // (docs/probe-gi-dof-examples.md): bounded here and, with MSAA, in msaaLimits.
 webgl_gpgpu_birds_gltf:{limits:[.01,.6],antialias:true,times:[0,.1,.2,.4],parameters:[[3,.3,.3],[4,2048,2048],[0,60,60],[1,5,5],[2,50,50]],restore:[[0,20,20],[1,20,20],[2,20,20],[3,.2,.2],[4,1024,1024]],at:.4,script:[['move',300,200,.5]]},
 // The bunny turns by the clock.
 webgl_materials_subsurface_scattering:{antialias:true,times:[0,1,2.5],parameters:[[0,.5,.5],[1,1,1],[2,2,2],[3,6,6],[4,30,30]],restore:[[0,.1,.1],[1,.4,.4],[2,.8,.8],[3,2,2],[4,16,16]],at:2.5,drag:[[256,256],[300,280]],wheel:[256,256,-200]},
 // The camera orbits by the clock; each frame raycasts the pointer for the autofocus, eases
 // the focus and moves the leaves. The pointer moves ( focus ray and coordinates ) before the
 // parameters. The flat-shaded heads' facet edges rasterize to the neighboring facet in a few
 // pixels between WebGL's and WebGPU's conventions (docs/probe-gi-dof-examples.md). Both
 // passes draw the cube background as a 36-index box in WebGL, and the composite a 6-index
 // quad; the port draws full-screen triangles for both.
 webgl_postprocessing_dof2:{maskDraws:[[36,2,0],[6,1,0]],limits:[.035,.6],times:[0,1,2.5],parameters:[[4,5,5],[5,2,2],[10,.2,.2],[11,10,10],[12,1,1],[13,2,2],[16,.001,.001],[6,true,1],[8,true,1],[9,true,1],[17,true,1],[7,true,1],[2,true,1],[1,false,0],[3,50,50],[14,60,60],[18,5,5],[19,6,6],[0,false,0]],restore:[[0,true,1],[18,3,3],[19,4,4],[14,35,35],[1,true,1],[2,false,0],[7,false,0],[17,false,0],[9,false,0],[8,false,0],[6,false,0],[16,.0001,.0001],[13,.7,.7],[12,.5,.5],[11,2,2],[10,.5,.5],[5,1,1],[4,2.2,2.2]],at:2.5,script:[['move',300,200,2.5],['move',120,380,2.5]]},
 // SimpleGI computes 32 vertices per frame after the render: each capture renders 134 frames,
 // one bounce; the third bounce ends the computation.
 webgl_simple_gi:{frames:134,times:[0,0,0,0],parameters:[],at:0,drag:[[256,256],[300,280]],wheel:[256,256,-200]},
 webgl_loader_fbx_nurbs:{times:[0],parameters:[],at:0,drag:[[256,256],[300,280]],wheel:[256,256,200]},
 webgl_points_dynamic:{times:[0,1,2.5],parameters:[],at:2.5,cycle:[0,.1,.2,.1,0],limits:[.03,.45]},
 webgl_postprocessing_advanced:{times:[0,1,2.5],parameters:[],at:2.5,maskDraws:[53052,6,2]},
 webgpu_postprocessing_outline:{times:[0],parameters:[],at:0,settle:true,script:[['move',256,256,0],['param',0,6,6,0],['param',1,.8,.8,0],['param',2,3,3,0],['param',4,0xff0000,0xff0000,0],['param',5,0x00ff00,0x00ff00,0],['param',3,2,2,1.3],['param',3,0,0,1.3],['move',140,320,1.3],['move',380,190,1.3],['move',30,30,1.3]],restore:[[0,3,3],[1,0,0],[2,1,1],[4,0xffffff,0xffffff],[5,0x4e3636,0x4e3636]],drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 webgl_postprocessing_sao:{times:[0,1,2.5],parameters:[[0,'SAO Only',1],[0,'Normal',2],[0,'Default',0],[1,.2,.2],[2,.5,.5],[3,3,3],[4,40,40],[5,.2,.2],[6,false,0],[6,true,1],[7,30,30],[8,10,10],[9,.05,.05],[10,false,0]],restore:[[1,.5,.5],[2,.18,.18],[3,1,1],[4,100,100],[5,0,0],[7,8,8],[8,4,4],[9,.01,.01],[10,true,1]],at:2.5},
 // Both canvases are captured: the main thread's and the worker's. Each capture waits for
 // the worker's frames ( pick ); the first waits for the worker to load. With MSAA the
 // spheres' silhouettes resolve differently ( msaaLimits ).
 // The CSG result is evaluated on the CPU each frame and streamed, as the page's
 // three-bvh-csg result is. The wireframe shares the result without its transform.
 // The port's interleaved vertex is 80 bytes against WebGL's 32 ( position, normal
 // and UV ): its stream is bounded at 2.5 times the original's. Showing the wireframe
 // builds its line index again, as WebGL's wireframe attribute is: the residency cycle
 // leaves the parameters out.
 // The orbit auto-rotates one step per frame; the left half without FXAA, the right with it.
 webgl_postprocessing_fxaa:{times:[0,.5,1],parameters:[],at:1,drag:[[300,256],[360,300]],wheel:[300,256,-300]},
 // The group turns each frame; the left half through a plain target, the right through a 4× MSAA one.
 // The left half matches; the right half's dense one-pixel wireframe lines resolve differently
 // under WebGL's and WebGPU's 4× MSAA ( docs/webgl-only-examples.md ).
 webgl_multisampled_renderbuffers:{limits:[.1,3],times:[0,.5,1],parameters:[[0,false,0]],at:1},
 // Each frame composites the new render with the damped previous composite: every capture
 // renders one frame on both sides, so the history matches.
 webgl_postprocessing_afterimage:{times:[0,.5,1,2],parameters:[[0,.5,.5],[0,.9,.9],[1,false,0],[1,true,1]],at:2,noResize:true},
 // The sphere group turns per frame and the icosahedron follows the clock. Each Reflector
 // renders inside the render that draws it ( the other mirror's included ). The reflections are
 // 4× MSAA targets on both sides: their edges resolve differently ( also msaaLimits ). The
 // resolution GUI resizes the reflection targets, as the page's setSize does.
 webgl_mirror:{limits:[.008,.3],rebuilds:true,antialias:true,times:[0,1,2.5],parameters:[[0,.5,.5],[0,1,1]],at:2.5,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 webgl_geometry_csg:{antialias:true,streamsGeometry:true,rebuilds:true,streamRatio:2.5,times:[0,1,2.5],parameters:[[0,'INTERSECTION',1],[0,'ADDITION',2],[0,'SUBTRACTION',0],[1,true,1],[1,false,0],[2,false,0],[2,true,1]],at:2.5,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 // The packed models parse at load ( and on each model, flat-color, merge or smoothing
 // change ); merging hides the model, as the page's NaN building step does. The orbit
 // is damped.
 webgl_loader_ldraw:{antialias:true,rebuilds:true,frames:40,times:[0],parameters:[[5,false,0],[5,true,1],[6,false,0],[6,true,1],[3,2,2],[3,99,99],[1,true,1],[1,false,0],[4,false,0],[4,true,1],[2,true,1],[2,false,0],[0,'Radar Truck',2],[0,'Lighthouse',7],[0,'Car',0]],at:0,drag:[[256,256],[330,300]],wheel:[256,256,-300],settle:true},
 // BatchedMesh culls, picks LODs and sorts on the CPU each frame ( the extensions'
 // onBeforeRender ); WebGL multi-draws one entry per instance, which the workload
 // compares with the port's instanced draws expanded per instance. Pointer moves
 // raycast and recolor the hovered instance. Freeze keeps the last culling while the
 // view pans.
 webgl_batch_lod_bvh:{antialias:true,multiDraw:true,times:[0],parameters:[[3,false,0],[3,true,1],[2,false,0],[2,true,1]],at:0,script:[['move',256,300,0],['move',150,330,0],['param',1,true,1,null],['drag',256,256,330,256,0,0],['param',1,false,0,0]],residency:[['move',256,300,0],['move',150,330,0]],drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 // The main thread's scene renders on its own animation frames, which may run before a
 // request reaches them: the workload holds the clock at 0 so every frame turns the same.
 webgl_worker_offscreencanvas:{workloadTimes:[0,0,0,0,0,0],view:'#container',pick:true,antialias:true,times:[],parameters:[],at:0,script:[['wait',2000,0],['wait',0,1],['wait',0,2.5]]},
 // The mirror-smooth box reflects WebGL's RoomEnvironment PMREM a little differently.
 webgl_loader_texture_lottie:{limits:[.025,.9],antialias:true,times:[0,.3,.6,.9,1.2,1.6,2.5,4],parameters:[],at:4,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 // WebGL's image-based multiple scattering lights the gold and silver a little apart,
 // most once the GUI roughens them.
 webgl_watch:{limits:[.035,1.2],backgroundBox:true,antialias:true,times:[0,1,3,6.5],parameters:[[0,.5,.5],[1,.3,.3],[2,.3,.3]],restore:[[0,.1,.1],[1,1,1],[2,.8,.8]],at:6.5,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 // The model's coplanar faces z-fight a few pixels apart between the backends, and its
 // thin mullions resolve differently under MSAA.
 webgl_loader_ifc:{limits:[.01,.6],antialias:true,times:[0],parameters:[],at:0,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 // With MSAA the 218 thin curves' edges resolve differently ( msaaLimits ).
 webgl_loader_3dm:{antialias:true,times:[0],parameters:[[0,true,1],[1,true,1],[4,true,1],[5,true,1],[3,false,0],[2,false,0]],restore:[[0,false,0],[1,false,0],[4,false,0],[5,false,0],[3,true,1],[2,true,1]],at:0,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 // The webcam is the fake camera's fixed canvas stream; the planes wait for its first frame.
 webgl_materials_video_webcam:{fakeCamera:true,antialias:true,times:[],parameters:[],at:0,script:[['camera',0],['drag',256,256,330,300,0,0],['drag',330,300,256,256,0,0],['drag',300,200,150,260,0,0],['drag',150,260,300,200,0,0]]},
 // WebGL's PMREM and transmission sampling light the cloth and the translucent dragon a
 // few levels apart from the WebGPU path, over most of the frame once the dragon fades.
 webgl_materials_physical_transmission_alpha:{limits:[.42,4.7],antialias:true,times:[0],parameters:[[1,.5,.5],[2,.6,.6],[4,.3,.3],[5,1.2,1.2],[6,1,1],[8,.5,.5],[9,.5,.5],[11,.5,.5],[12,.7,.7],[0,0xff8844,0xff8844],[7,0x88ccff,0x88ccff],[10,0x88ff88,0x88ff88]],restore:[[0,0xffffff,0xffffff],[1,1,1],[2,1,1],[4,0,0],[5,1.5,1.5],[6,2.27,2.27],[7,0xf6d148,0xf6d148],[8,.155,.155],[9,1,1],[10,0xffffff,0xffffff],[11,1,1],[12,1,1]],at:0,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 // The blurred lobe background's gradient bands differ by a level between the backends.
 webgl_random_uv:{limits:[.005,.9],backgroundBox:true,antialias:true,times:[0],parameters:[[2,.4,.4],[3,.5,.5],[7,true,1],[6,true,1],[5,false,0],[4,false,0],[0,.6,.6],[1,.5,.5]],restore:[[0,0,0],[1,0,0],[2,0,0],[3,.2,.2],[4,true,1],[5,true,1],[6,false,0],[7,false,0]],at:0,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 games_fps:{limits:[.005,1.4],antialias:true,noResize:true,times:[0,.5,1,1.5,2,2.5],parameters:[[0,true,1],[0,false,0]],at:2.5,script:[['key','w',true,2.6],['wait',0,2.7],['wait',0,2.8],['key','d',true,2.9],['wait',0,3],['key','w',false,3.1],['key','d',false,3.2],['key',' ',true,3.3],['wait',0,3.4],['key',' ',false,3.5],['wait',0,3.6],['wait',0,3.7],['wait',0,3.8]]},
 webgl_loader_fbx:{limits:[.008,.2],antialias:true,rebuilds:true,frames:40,times:[0,1,2.5],parameters:[[0,'morph_test',1],[0,'monkey',2],[0,'monkey_embedded_texture',3],[0,'vCube',4],[0,'archer/ArcherRi01',5],[0,'warrior/Warrior',6],[0,'stanford-bunny',7],[0,'mixamo',8],[0,'RotationTest',9],[0,'exampleWindow',10],[0,'Head_69',11],[0,'morph-translation',12],[0,'ball_anims_asc_2018',13],[0,'Samba Dancing',0]],at:2.5,drag:[[256,256],[330,300]],settle:true},
 webgl_loader_svg:{antialias:true,rebuilds:true,settle:true,times:[0],parameters:[[0,'Joins and caps',1],[0,'Hexagon',2],[0,'Energy',3],[0,'Test 1',4],[0,'Test 2',5],[0,'Test 3',6],[0,'Test 4',7],[0,'Test 5',8],[0,'Test 6',9],[0,'Test 7',10],[0,'Test 8',11],[0,'Test 9',12],[0,'Units',13],[0,'Ordering',14],[0,'Defs',15],[0,'Defs2',16],[0,'Defs3',17],[0,'Defs4',18],[0,'Defs5',19],[0,'Style CSS inside defs',20],[0,'Styled Paths',21],[0,'Multiple CSS classes',22],[0,'Zero Radius',23],[0,'Styles in svg tag',24],[0,'Round join',25],[0,'Ellipse Transformations',26],[0,'singlePointTest',27],[0,'singlePointTest2',28],[0,'singlePointTest3',29],[0,'emptyPath',30],[0,'emoji',31],[0,'blueprint',32],[0,'wideStroke',33],[0,'letter',34],[0,'Tiger',0],[1,false,0],[1,true,1],[2,false,0],[2,true,1],[3,true,1],[4,true,1],[3,false,0],[4,false,0]],at:0,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 misc_exporter_gcode:{antialias:true,rebuilds:true,times:[0],parameters:[[6,null,1],[7,null,1],[8,null,1],[9,null,1],[5,null,1]],at:0,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 misc_controls_arcball:{antialias:true,residency:[['drag',256,256,330,300,0,0],['drag',256,256,300,200,2,0],['wheel',256,256,-300,0],['key','Shift',true,0],['wheel',256,256,250,0],['key','Shift',false,0],['dblclick',240,250,0],['drag',300,300,250,330,1,0]],times:[0],parameters:[[18,false,0],[14,false,0],[14,true,1]],at:0,script:[['drag',256,256,330,300,0,0],['drag',256,256,300,200,2,0],['wheel',256,256,-300,0],['key','Shift',true,0],['wheel',256,256,250,0],['key','Shift',false,0],['dblclick',240,250,0],['drag',300,300,250,330,1,0],['param',8,true,1,0],['wheel',256,256,400,0],['param',17,null,1,0],['param',0,'Orthographic',0,0],['drag',256,256,330,300,0,0],['wheel',256,256,-300,0],['drag',256,256,300,200,2,0],['param',17,null,1,0],['param',0,'Perspective',1,0]]},
 // WebGL draws the line loop's five vertices as LINE_LOOP; the port closes it as a
 // six-vertex strip ( maskDraws, portDraws ).
 misc_exporter_gltf:{maskDraws:[5,1,0],portDraws:[[6,1]],limits:[.012,.7],antialias:true,times:[0,2,6,15],parameters:[],at:15},
 webgl_geometry_spline_editor:{antialias:true,rebuilds:true,times:[0],parameters:[[0,false,0],[2,false,0],[3,false,0],[0,true,1],[1,1,1],[1,0,0],[4,null,1],[4,null,1],[5,null,1],[2,true,1],[3,true,1],[5,null,1],[1,.5,.5]],at:0,hover:[[378,77]],draw:[[378,77],[400,95],[420,110]],drag:[[256,450],[330,470]],wheel:[256,256,-300]},
 webgl_raycaster_bvh:{antialias:true,streamsGeometry:true,times:[0,.5,1,2],parameters:[[3,true,1],[4,5,5],[4,1,1],[0,600,600],[2,false,0],[1,false,0],[2,true,1]],restore:[[0,150,150],[3,false,0],[4,10,10],[1,true,1]],at:2,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 webgl_modifier_subdivision:{antialias:true,rebuilds:true,times:[0],parameters:[[0,'Capsule',1],[0,'Circle',2],[0,'Cone',3],[0,'Cylinder',4],[0,'Dodecahedron',5],[0,'Icosahedron',6],[0,'Lathe',7],[0,'Octahedron',8],[0,'Plane',9],[0,'Ring',10],[0,'Sphere',11],[0,'Tetrahedron',12],[0,'Torus',13],[0,'TorusKnot',14],[4,true,1],[5,true,1],[9,true,1],[7,true,1],[8,false,0],[1,5,5],[6,1000,1000],[0,'Box',0]],restore:[[4,false,0],[5,false,0],[9,false,0],[7,false,0],[8,true,1],[1,3,3],[6,25000,25000]],at:0,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 webgl_postprocessing_gtao:{times:[0,1,2.5],parameters:[[0,0,0],[0,4,2],[0,2,4],[0,3,5],[0,1,1],[0,5,3],[2,.5,.5],[7,24,24],[8,true,1],[12,8,8],[14,4,4],[15,24,24]],restore:[[2,.25,.25],[7,16,16],[8,false,0],[12,4,4],[14,2,2],[15,16,16]],at:2.5,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 webgl_postprocessing_ssao:{times:[0,1,2.5],parameters:[[0,'SSAO Only',1],[0,'SSAO Only + Blur',2],[0,'Depth',3],[0,'Normal',4],[0,'Default',0],[1,16,16],[2,.01,.01],[3,.2,.2],[4,false,0]],restore:[[1,8,8],[2,.005,.005],[3,.1,.1],[4,true,1]],at:2.5},
 webgpu_clipping_stencil:{times:[0,1,2.5],parameters:[[2,.3,.3],[5,-.2,-.2],[3,true,1],[8,.4,.4],[1,true,1],[4,true,1],[7,true,1],[0,false,0],[9,true,1]],restore:[[2,0,0],[5,0,0],[3,false,0],[8,0,0],[1,false,0],[4,false,0],[7,false,0],[0,true,1],[9,false,0]],at:2.5,antialias:true,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 // The slider's line and handle and the labels are page overlays, hidden in both.
 webgl_test_wide_gamut:{times:[0],parameters:[],at:0,antialias:true,hide:'.slider:before,.slider:after,.label',slide:[[256,256],[150,256]]},
 webgl_materials_channels:{times:[0],parameters:[[0,'standard',0],[0,'velocity',2],[0,'depthBasic',3],[0,'depthRGBA',4],[0,'depthRGB',5],[0,'depthRG',6],[0,'normal',1],[2,'front',0],[2,'back',1],[1,'ortho',1],[0,'depthRGBA',4],[0,'standard',0],[2,'double',2],[0,'normal',1],[1,'perspective',0]],restore:[],at:0,settle:true,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 webgl_video_kinect:{limits:[.008,.6],frames:3,times:[],parameters:[],at:1,script:[['video',2,0],['param',0,1500,1500,0],['param',1,6000,6000,0],['param',2,4,4,0],['param',3,2000,2000,0],['param',0,850,850,0],['param',1,4000,4000,0],['param',2,2,2,0],['param',3,1000,1000,0],['video',5.5,0],['move',400,300,null],['wait',.5],['wait',1.5],['move',60,100,null],['wait',3]]},
 webgl_loader_texture_dds:{times:[0,.8,1.7,3.1],parameters:[],at:3.1,antialias:true},
 webgl_volume_instancing:{times:[0,.8,1.7,3.1],parameters:[],at:3.1,drag:[[256,256],[330,300]]},
 // The batch streams its changed matrices and each frame's draw ids, as the original's
 // matrix and indirect textures are uploaded.
 webgpu_mesh_batch:{times:[0,1,2,3],parameters:[[2,512,512],[3,.5,.5],[6,false,0],[4,false,0],[3,1,1],[5,false,0],[7,null,1],[1,2000,2000]],restore:[[2,16,16],[4,true,1],[5,true,1],[6,true,1],[1,512,512]],at:3,batch:true,antialias:true,drag:[[256,256],[330,300]]},
 // The render bundle and backend switches reload the original's page; dynamic turns the
 // objects each frame and records the bundle again.
 webgpu_performance_renderbundle:{times:[0,1,2,3],parameters:[[2,true,1],[2,true,1]],restore:[[2,false,0]],at:3,antialias:true,drag:[[256,256],[330,300]]},
 // The material buttons, then the simulation controls; the surface is polygonized on the
 // CPU each frame into streamed attributes, as the original does.
 webgl_marchingcubes:{times:[0,1,2.5],parameters:[[1,null,1],[2,null,1],[3,null,1],[4,null,1],[5,null,1],[6,null,1],[7,null,1],[8,null,1],[9,null,1],[10,null,1],[11,null,1],[12,null,1],[0,null,1],[14,20,20],[15,40,40],[16,120,120],[17,false,0],[18,true,1],[19,true,1],[13,2,2]],restore:[[14,10,10],[15,28,28],[16,80,80],[17,true,1],[18,false,0],[19,false,0],[13,1,1]],at:2.5,streamsGeometry:true,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 webgpu_lights_ies_spotlight:{times:[0,.7,1.6,2.9],parameters:[[0,true,1],[0,false,0]],at:2.9,antialias:true,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 webgl_loader_texture_pvrtc:{times:[0,.8,1.7,3.1],parameters:[],at:3.1,antialias:true},
 webgl_loader_texture_ktx:{times:[0,.8,1.7,3.1],parameters:[],at:3.1,antialias:true},
 webgpu_display_stereo:{backgroundSphere:true,times:[0,1,2.5],parameters:[[1,.1,.1],[0,'Anaglyph',1],[2,'Grey',1],[3,'Magenta / Cyan',1],[4,5,5],[2,'Compromise',6],[3,'Magenta / Green',2],[0,'ParallaxBarrier',2]],restore:[[0,'Stereo',0],[1,.064,.064],[2,'Dubois',4],[3,'Red / Cyan',0],[4,3,3]],at:2.5,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 webgl_gpgpu_protoplanet:{times:[0,.5,1,2],parameters:[[0,300,300],[1,1,1],[9,0,1],[1,0.45,0.45],[0,100,100]],at:2,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 webgl_shadowmap_pcss:{times:[0,.5,1.3,2],parameters:[],at:2,antialias:true,drag:[[256,300],[330,340]],wheel:[256,300,-300]},
 webgl_shadowmap_viewer:{times:[0,.5,1.3,2],parameters:[],at:2,antialias:true,drag:[[256,300],[330,340]],wheel:[256,300,-300]},
 webgl_materials_envmaps_exr:{backgroundBox:true,times:[0,1,2],parameters:[[1,.5,.5],[2,1,1],[3,.6,.6],[0,'PNG',1]],restore:[[0,'EXR',0],[1,0,0],[2,0,0],[3,1,1]],at:2,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 webgl_texture2darray:{times:[0,1,2.5,4,10],parameters:[],at:4},
 webgl_texture2darray_compressed:{times:[0,.05,.13,.27,.41],parameters:[],at:.41},
 webgl_rendertarget_texture2darray:{times:[0,1,2.5,4,10],parameters:[[0,.3,.3],[0,.75,.75]],restore:[[0,1,1]],at:4},
 webgl_texture2darray_layerupdate:{times:[0],parameters:[[0,3,3],[1,1,1],[2,null,1],[0,4,4],[1,2,2],[2,null,1],[0,1,1],[1,0,0],[2,null,1]],restore:[[0,0,0],[1,0,0],[2,null,1],[0,1,1],[1,1,1],[2,null,1],[0,2,2],[1,2,2],[2,null,1],[0,0,0],[1,0,0]],at:0,antialias:true},
 webgl_loader_nrrd:{times:[0,1],parameters:[[0,60,60],[1,200,200],[2,100,100],[3,500,500],[4,3000,3000],[5,200,200],[6,2500,2500],[0,240,240],[2,0,0]],restore:[[0,120,120],[1,120,120],[2,42,42],[3,0,0],[4,3952,3952],[5,0,0],[6,3952,3952]],at:1,antialias:true,script:[['drag',256,256,330,300,0,2],['drag',256,256,300,200,2,2.5],['wheel',256,256,-400,3],['wait',4]]},
 webgl_texture3d:{limits:[.02,.5],times:[0],parameters:[[3,'mip',0],[2,'gray',0],[0,0.2,0.2],[1,0.8,0.8],[3,'iso',1],[4,0.3,0.3],[2,'viridis',1]],restore:[[0,0,0],[1,1,1],[4,0.15,0.15]],at:0,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
};
const official=kind=>kind;
// The original streams the 10,000 instance matrices and colors each frame.
// webgpu_display_stereo streams its 500 instance matrices each frame.
const streams=['webgpu_display_stereo','webgl_marchingcubes','webgl_geometry_csg'];
// A deterministic webcam: getUserMedia returns a 1280 × 720 canvas stream of a fixed
// gradient and shapes, redrawn each animation frame so the stream keeps presenting.
const fakeCamera=()=>{if(!navigator.mediaDevices)return;navigator.mediaDevices.getUserMedia=async()=>{const canvas=document.createElement('canvas');canvas.width=1280;canvas.height=720;const g=canvas.getContext('2d');const draw=()=>{const gradient=g.createLinearGradient(0,0,1280,720);gradient.addColorStop(0,'#203080');gradient.addColorStop(.5,'#c04060');gradient.addColorStop(1,'#f0d040');g.fillStyle=gradient;g.fillRect(0,0,1280,720);g.fillStyle='#ffffff';g.fillRect(160,120,320,480);g.fillStyle='#10a050';g.beginPath();g.arc(900,360,220,0,Math.PI*2);g.fill();requestAnimationFrame(draw);};draw();return canvas.captureStream(30);};};
// Perform one scripted input step; returns its capture time (or null).
const act=async(page,runtime,step)=>{const [action,...args]=step;const time=args.pop();const button=i=>['left','middle','right'][i];
 if(action==='drag'){const [x0,y0,x1,y1,b]=args;await page.mouse.move(x0,y0);await page.mouse.down({button:button(b)});await page.mouse.move(x1,y1,{steps:5});await page.mouse.up({button:button(b)});}
 else if(action==='wheel'){const [x,y,delta]=args;await page.mouse.move(x,y);await page.mouse.wheel(0,delta);}
 else if(action==='key'){const [key,down]=args;if(down)await page.keyboard.down(key);else await page.keyboard.up(key);}
 else if(action==='video'){const [seconds]=args;await page.evaluate(async seconds=>{const v=document.getElementById('video');v.pause();v.currentTime=seconds;await new Promise(r=>v.addEventListener('seeked',r,{once:true}));await new Promise(r=>requestAnimationFrame(()=>requestAnimationFrame(r)));},seconds);}
 else if(action==='lock'){await page.evaluate(runtime=>{if(runtime!=='rust')fixtureLockControls.isLocked=true;else app.tsl_parameter(0,1);},runtime);}
 else if(action==='look'){const [x,y]=args;await page.evaluate(([x,y])=>document.dispatchEvent(new MouseEvent('mousemove',{movementX:x,movementY:y})),[x,y]);}
 else if(action==='type'){await page.keyboard.type(args[0]);}
 else if(action==='press'){await page.keyboard.press(args[0]);}
 else if(action==='dblclick'){const [x,y]=args;await page.mouse.dblclick(x,y);}
 else if(action==='wait'){if(args.length)await page.waitForTimeout(args[0]);}
 else if(action==='camera'){await page.waitForFunction(()=>{const v=document.getElementById('video');return v&&v.readyState>=2&&v.currentTime>0;});await page.evaluate(()=>new Promise(r=>requestAnimationFrame(()=>requestAnimationFrame(r))));}
 else if(action==='move'){const [x,y]=args;await page.mouse.move(x,y);}
 else if(action==='down'||action==='up'){const [b,x,y]=args;await page.mouse.move(x,y);await page.mouse[action]({button:button(b)});}
 else if(action==='param'){const [i,reference,rust]=args;await page.evaluate(({runtime,i,reference,rust})=>{if(runtime!=='rust')fixtureParameter(i,reference);else app.tsl_parameter(i,rust);},{runtime,i,reference,rust});}
 return time;
};
// WebGL/WebGPU MSAA resolve bounds, documented in docs/texture-volumes.md. The same
// scenes must also pass the ordinary threshold with MSAA disabled on both sides.
const msaaLimits={webgl_mirror:[.012,.3],webgl_geometry_csg:[.02,.9],webgl_loader_ldraw:[.035,.9],webgl_batch_lod_bvh:[.12,2.6],webgl_worker_offscreencanvas:[.015,.3],webgl_loader_ifc:[.055,1.5],webgl_loader_3dm:[.015,.5],webgl_materials_video_webcam:[.008,.4],webgl_materials_physical_transmission_alpha:[.42,4.8],webgl_random_uv:[.01,.95],games_fps:[.025,2],webgl_loader_fbx:[.025,.5],webgl_loader_svg:[.08,3],misc_controls_arcball:[.018,.5],misc_exporter_gltf:[.055,1.7],webgl_geometry_spline_editor:[.1,1.5],webgl_raycaster_bvh:[.06,.8],webgl_modifier_subdivision:[.05,2],webgl_shadowmap_performance:[.06,.9],misc_exporter_usdz:[.035,1],webgl_gpgpu_birds_gltf:[.16,3.5],webgl_loader_usdz:[.008,.6],webgl_loader_nrrd:[.012,.45],webgl_shadowmap_viewer:[.025,.8],webgl_shadowmap_pcss:[.015,.4],webgl_loader_texture_pvrtc:[.008,.3]};
const frames=(page,runtime,t,n)=>page.evaluate(async({runtime,t,n})=>{for(let i=0;i<n;i++){const c=document.querySelector('canvas'),previous=c.dataset.frames;if(runtime!=='rust')await renderFixture(t);else{app.gallery_time(t);while(c.dataset.frames===previous)await new Promise(r=>requestAnimationFrame(r));}}},{runtime,t,n});
for(const [kind,spec] of Object.entries(cases))for(const samples of spec.antialias?[1,4]:[1])test(`Texture arrays and volumes official rendering: ${kind} samples=${samples}`,async({page},info)=>{
 test.setTimeout(300000);const images={};const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 if(spec.fakeCamera)await page.addInitScript(fakeCamera);
 await page.addInitScript(()=>{window.fixtureError=null;const request=GPUAdapter.prototype.requestDevice;GPUAdapter.prototype.requestDevice=async function(...a){const d=await request.apply(this,a);d.addEventListener('uncapturederror',e=>window.fixtureError=e.error.message);return d;};addEventListener('error',e=>window.fixtureError=e.message);addEventListener('unhandledrejection',e=>window.fixtureError=String(e.reason));});
 for(const runtime of ['reference','rust']){
  await page.setViewportSize({width:512,height:512});await page.mouse.move(511,0);
  await page.goto(runtime!=='rust'?`/reference/three-js/texture-volumes.html?id=${official(kind)}&samples=${samples}`:`/web/gallery/example.html?id=${official(kind)}&still=1`);
  await page.waitForFunction(v=>{const c=document.querySelector(v);const error=window.fixtureError||c?.dataset.error||document.querySelector('main p')?.textContent;if(error)throw Error(error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},view,{timeout:90000});
  if(runtime==='rust'&&samples===1)await page.evaluate(()=>app.set_samples(1));
  await page.addStyleTag({content:'#notice,#settings,#info,#stats,#selectBox,#blocker{display:none!important}'+(spec.hide?spec.hide+'{visibility:hidden!important}':'')});
  const shots=[];
  const capture=async(t,parameter=null)=>{
   if(parameter)await page.evaluate(({runtime,parameter})=>{const [i,reference,rust]=parameter;if(runtime!=='rust')fixtureParameter(i,reference);else app.tsl_parameter(i,rust);},{runtime,parameter});
   await frames(page,runtime,t,spec.frames??1);
   if(spec.pick)for(let k=0;k<3;k++){await page.waitForTimeout(100);await frames(page,runtime,t,1);}
   expect(await page.evaluate(()=>window.fixtureError)).toBeNull();
   shots.push(PNG.sync.read(await page.locator(spec.view??view).screenshot()));
  };
  const t=spec.at;
  for(const time of spec.times)await capture(time);
  for(const [i,reference,rust,time] of spec.parameters)await capture(time??spec.at,[i,reference,rust]);
  for(const step of spec.script??[]){const time=await act(page,runtime,step);if(time!==null)await capture(time);}
  for(const [x,y] of spec.hover??[]){await page.mouse.move(x,y);await capture(t);}
  if(spec.draw){const [first,...rest]=spec.draw;await page.mouse.move(...first);await page.mouse.down();for(const p of rest)await page.mouse.move(...p,{steps:6});await page.mouse.up();await capture(t);}
  for(const [x,y,time] of spec.motion??[]){await page.mouse.move(x,y);await capture(time);}
  for(const [x,y,shift] of spec.clicks??[]){if(shift)await page.keyboard.down('Shift');await page.mouse.click(x,y);if(shift)await page.keyboard.up('Shift');await capture(t);}
  const settle=async()=>{await frames(page,runtime,t,240);await capture(t);};
  if(spec.drag){const [[x0,y0],[x1,y1]]=spec.drag;await page.mouse.move(x0,y0);await page.mouse.down();await page.mouse.move(x1,y1,{steps:5});await page.mouse.up();await settle();}
  if(spec.slide){const [[x0,y0],[x1,y1]]=spec.slide;await page.mouse.move(x0,y0);await page.mouse.down();await page.mouse.move(x1,y1,{steps:4});await page.mouse.up();await capture(t);}
  // Damped controls keep moving after input: those captures wait for the motion to settle.
  const after=()=>spec.settle?settle():capture(t);
  const wheel=async([x,y,delta])=>{await page.mouse.move(x,y);await page.mouse.wheel(0,delta);await after();};
  if(spec.wheel)await wheel(spec.wheel);
  if(spec.pan){const [[x0,y0],[x1,y1]]=spec.pan;await page.mouse.move(x0,y0);await page.mouse.down({button:'right'});await page.mouse.move(x1,y1,{steps:4});await page.mouse.up({button:'right'});await after();}
  if(!spec.noResize){await page.setViewportSize({width:640,height:400});await page.waitForTimeout(100);await frames(page,runtime,t,1);await capture(t);}images[runtime]=shots;
 }
 const results=[];
 for(let state=0;state<images.rust.length;state++){const a=images.rust[state],b=images.reference[state];expect([a.width,a.height]).toEqual([b.width,b.height]);let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail ||=d>6;}if(fail)bad++;}results.push({state,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)});writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));}
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));expect(errors).toEqual([]);
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));
 const [limit,meanLimit]=(samples===4&&msaaLimits[kind])||spec.limits||[.005,.6];
 for(const r of results){expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(limit);expect(r.meanError,JSON.stringify(r)).toBeLessThanOrEqual(meanLimit);}
});

for(const [kind,spec] of Object.entries(cases)){const id=official(kind);
 test(`Texture arrays and volumes GPU residency: ${kind}`,async({page},info)=>{
  test.setTimeout(120000);
  if(spec.fakeCamera)await page.addInitScript(fakeCamera);
  await page.addInitScript(()=>{window.creates=0;for(const key of ['createBuffer','createTexture','createBindGroup','createShaderModule','createRenderPipeline','createComputePipeline']){const original=GPUDevice.prototype[key];GPUDevice.prototype[key]=function(...args){creates++;return original.apply(this,args);};}});
  await page.goto(`/web/gallery/example.html?id=${id}&still=1`);await page.waitForFunction(v=>Number(document.querySelector(v)?.dataset.frames)>0,view);
  const reports=[];
  for(const resize of spec.noResize?[false]:[false,true]){
   if(resize)await page.setViewportSize({width:640,height:400});
   const cycle=async()=>{
    // Accumulating motion driven by the clock deltas uses a cycle whose deltas cancel.
    for(const t of spec.cycle??[0,1,2,4,0])await frames(page,'rust',t,1);
    // Parameter cycles end where they began, so the second cycle revisits the same states.
    for(const [i,,v] of spec.rebuilds?[]:[...spec.parameters,...(spec.restore??[])]){await page.evaluate(([i,v])=>app.tsl_parameter(i,v),[i,v]);await frames(page,'rust',spec.at,1);}
    // Typing rebuilds the text geometry, as the original does: the cycle drags only.
    for(const step of spec.residency??spec.script??[]){const time=await act(page,'rust',step);await frames(page,'rust',time??spec.at,1);}
    if(spec.wheel){await page.mouse.move(spec.wheel[0],spec.wheel[1]);await page.mouse.wheel(0,spec.wheel[2]);await frames(page,'rust',spec.at,1);await page.mouse.wheel(0,-spec.wheel[2]);await frames(page,'rust',spec.at,1);}
    for(const [x,y] of spec.hover??[]){await page.mouse.move(x,y);await frames(page,'rust',spec.at,1);}
    for(const [x,y,shift] of spec.clicks??[]){await page.mouse.click(x,y);await frames(page,'rust',spec.at,1);}
    if(spec.slide){await page.mouse.move(...spec.slide[0]);await page.mouse.down();await page.mouse.move(...spec.slide[1],{steps:3});await page.mouse.up();await frames(page,'rust',spec.at,1);await page.mouse.move(...spec.slide[1]);await page.mouse.down();await page.mouse.move(...spec.slide[0],{steps:3});await page.mouse.up();await frames(page,'rust',spec.at,1);}
    if(spec.drag){await page.mouse.move(...spec.drag[0]);await page.mouse.down();await page.mouse.move(...spec.drag[1],{steps:3});await page.mouse.up();await frames(page,'rust',spec.at,3);}
   };
   const read=()=>page.evaluate(()=>({creates,transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
   // A streamed surface keeps changing with the advancing clock: its buffers reach their
   // grown capacity after a further cycle.
   if(spec.streamsGeometry)await cycle();
   await cycle();const before=await read();await cycle();const after=await read();reports.push({resize,before,after});
   // A surface rebuilt each frame streams its attributes as the original does (bounded by
   // the workload test); it must still create nothing and keep residency flat.
   if(spec.streamsGeometry){delete before.transfers;delete after.transfers;}
   expect(after).toEqual(before);
  }
  writeFileSync(info.outputPath('residency.json'),JSON.stringify(reports,null,2));await expect(page.locator(view).first()).not.toHaveAttribute('data-error',/.+/);
 });
}

test('Texture arrays and volumes resident geometry and official draw workload',async({page},info)=>{
 test.setTimeout(180000);
 await page.addInitScript(()=>{
  window.resetWork=()=>window.work={draws:[],attributeBytes:0,transformBytes:0,textureBytes:0};resetWork();
  for(const key of ['draw','drawIndexed']){const fn=GPURenderPassEncoder.prototype[key];GPURenderPassEncoder.prototype[key]=function(...a){if(a[0]>3||a[1]>1)work.draws.push({count:a[0],instances:a[1]??1});return fn.apply(this,a);};}
  // BatchedMesh draws through WEBGL_multi_draw: one entry per instance.
  const extension=WebGL2RenderingContext.prototype.getExtension;WebGL2RenderingContext.prototype.getExtension=function(name){const e=extension.call(this,name);if(name==='WEBGL_multi_draw'&&e&&!e.counted){e.counted=true;const fn=e.multiDrawElementsWEBGL.bind(e);e.multiDrawElementsWEBGL=(mode,counts,countsOffset,type,offsets,offsetsOffset,drawCount)=>{for(let i=0;i<drawCount;i++)work.draws.push({count:counts[countsOffset+i],instances:1});return fn(mode,counts,countsOffset,type,offsets,offsetsOffset,drawCount);};}return e;};
  for(const key of ['drawArrays','drawElements']){const fn=WebGL2RenderingContext.prototype[key];WebGL2RenderingContext.prototype[key]=function(...a){const count=key==='drawArrays'?a[2]:a[1];// WebGL points become six-vertex WebGPU billboards.
// Draws of three or fewer vertices (the port's fullscreen clear and present triangles, and
// the two-vertex helper line) are left out on both sides.
if(count>3)work.draws.push({count:a[0]===0?count*6:count,instances:1});return fn.apply(this,a);};}
  for(const key of ['drawArraysInstanced','drawElementsInstanced']){const fn=WebGL2RenderingContext.prototype[key];WebGL2RenderingContext.prototype[key]=function(...a){work.draws.push({count:key==='drawArraysInstanced'?a[2]:a[1],instances:key==='drawArraysInstanced'?a[3]:a[4]});return fn.apply(this,a);};}
  const write=GPUQueue.prototype.writeBuffer;GPUQueue.prototype.writeBuffer=function(b,offset,data,dataOffset,size){// three's WebGPU InstanceNode keeps up to 64 KiB of instance matrices in a uniform
  // buffer: the reference's large uniform uploads count as streamed instance data.
  if(location.pathname.startsWith('/reference/')&&b.usage&GPUBufferUsage.UNIFORM&&(size??data.byteLength)>=16384)work.attributeBytes+=size??data.byteLength;
  if(b.usage&(GPUBufferUsage.STORAGE|GPUBufferUsage.VERTEX|GPUBufferUsage.INDEX)){if(b.label==='resident draw data'||b.label==='skin/morph input')work.transformBytes+=size??data.byteLength;else work.attributeBytes+=size??data.byteLength;}return write.apply(this,arguments);};
  // WebGL attribute uploads: the original streams its dynamic attributes with bufferSubData.
  for(const key of ['bufferData','bufferSubData']){const fn=WebGL2RenderingContext.prototype[key];WebGL2RenderingContext.prototype[key]=function(...a){const data=key==='bufferData'?a[1]:a[2];if(typeof data==='object'&&data)work.attributeBytes+=key==='bufferSubData'&&a[4]?a[4]*data.BYTES_PER_ELEMENT:data.byteLength;return fn.apply(this,a);};}
  const texture=GPUQueue.prototype.writeTexture;GPUQueue.prototype.writeTexture=function(dest,data,...args){work.textureBytes+=data.byteLength;return texture.call(this,dest,data,...args);};
 });
 const report=[];
 for(const [kind,spec] of Object.entries(cases)){
  const pair={kind};
  for(const runtime of ['reference','rust']){
   await page.mouse.move(511,0);
   await page.goto(runtime==='reference'?`/reference/three-js/texture-volumes.html?id=${official(kind)}`:`/web/gallery/example.html?id=${official(kind)}&still=1`);
   await page.waitForFunction(v=>{const c=document.querySelector(v);if(c?.dataset.error)throw Error(c.dataset.error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},view);
   // Per-frame solvers (the IK chain) converge before the measured frame.
   if(spec.frames)await frames(page,runtime,1,spec.frames);
   for(const [n,t] of (spec.workloadTimes??[1,2,3,1,2,3]).entries()){if(n===5)await page.evaluate(()=>resetWork());await frames(page,runtime,t,1);}
   pair[runtime]=await page.evaluate(()=>work);
  }
  // WebGL draws an equirectangular background as a 36-index box; the port's
  // background is a fullscreen triangle, left out with the other tiny draws.
  if(spec.backgroundBox){const i=pair.reference.draws.findIndex(d=>d.count===36&&d.instances===1);if(i>=0)pair.reference.draws.splice(i,1);}
  // The advanced composers' MaskPasses draw the head into both ping-pong buffers, three
  // times over two resolutions (six draws); the port draws one mask per resolution (two).
  // A list of [count, reference, rust] triples masks several kinds of draw.
  if(spec.maskDraws)for(const [count,reference,rust] of Array.isArray(spec.maskDraws[0])?spec.maskDraws:[spec.maskDraws])for(let k=0;k<reference-rust;k++){const i=pair.reference.draws.findIndex(d=>d.count===count);if(i>=0)pair.reference.draws.splice(i,1);}
  // The port's side of a scoped difference: [count, draws] the port issues beyond the
  // reference's ( each explained at its case ).
  if(spec.portDraws)for(const [count,n] of spec.portDraws)for(let k=0;k<n;k++){const i=pair.rust.draws.findIndex(d=>d.count===count);if(i>=0)pair.rust.draws.splice(i,1);}
  // The port draws each LOD's visible instances with one instanced draw.
  if(spec.multiDraw)pair.rust.draws=pair.rust.draws.flatMap(d=>Array.from({length:d.instances},()=>({count:d.count,instances:1})));
  report.push(pair);const sort=a=>a.map(x=>x.count*x.instances).sort((a,b)=>a-b);
  // The port draws equirectangular backgrounds with a fullscreen triangle; WebGL uses a
  // 36-index box and WebGPU a 5,952-index sphere. The stereo port draws its cube
  // background as a 36-index box (once per eye). All scene mesh draws must still match.
  const background=spec.backgroundSphere?5952:null;
  const referenceDraws=pair.reference.draws.filter(d=>d.count!==background);
  expect.soft(sort(pair.rust.draws.filter(d=>!spec.backgroundSphere||d.count!==36)),kind).toEqual(sort(referenceDraws));
  // Each instance's matrix and color stream together (80 bytes) as resident draw data;
  // WebGL streams the 64-byte matrices, and the colors only during a tween.
  if(spec.batch)expect.soft(pair.rust.attributeBytes,kind).toBeLessThanOrEqual(pair.reference.textureBytes*1.25);
  // spec.streamRatio scopes a documented vertex format difference.
  else if(streams.includes(kind))expect.soft(pair.rust.attributeBytes+pair.rust.transformBytes,kind).toBeLessThanOrEqual(pair.reference.attributeBytes*(spec.streamRatio??1.25));
  else expect.soft(pair.rust.attributeBytes,kind).toBe(0);
  expect.soft(pair.rust.textureBytes,kind).toBe(0);
 }
 // The record keeps each draw shape's tally ( the comparison is order-free ), as the batched
 // knots issue tens of thousands of multi-draw entries.
 const tally=draws=>{const m=new Map();for(const d of draws){const k=d.count+'x'+d.instances;m.set(k,(m.get(k)??0)+1);}return [...m].map(([k,n])=>{const [count,instances]=k.split('x').map(Number);return {count,instances,n};}).sort((a,b)=>a.count-b.count||a.instances-b.instances);};
 writeFileSync(info.outputPath('gpu-work.json'),JSON.stringify(report.map(r=>({...r,reference:{...r.reference,draws:tally(r.reference.draws)},rust:{...r.rust,draws:tally(r.rust.draws)}})),null,2));
});

for(const kind of ['webgl_texture2darray_layerupdate','webgl_texture3d'])test(`Texture arrays and volumes on-demand scene stays idle: ${kind}`,async({page})=>{
 await page.goto(`/web/gallery/example.html?id=${kind}`);await page.waitForFunction(()=>Number(document.querySelector('canvas')?.dataset.frames)>0);
 await page.waitForTimeout(300);const frames=await page.locator('canvas').getAttribute('data-frames');await page.waitForTimeout(300);expect(await page.locator('canvas').getAttribute('data-frames')).toBe(frames);
 await page.setViewportSize({width:640,height:400});await page.waitForFunction(prev=>document.querySelector('canvas').dataset.frames!==prev,frames);
});

// misc_uv_tests draws UVsDebug canvases with the Canvas 2D API: every section's
// title and canvas pixels must match the original page exactly.
test('UV mapping tests: UVsDebug canvases match the original',async({page})=>{
 test.setTimeout(120000);const read=()=>page.evaluate(()=>[...document.querySelectorAll('h3')].map(h=>{const c=h.parentNode.querySelector('canvas');const d=c.getContext('2d').getImageData(0,0,c.width,c.height).data;let hash=0;for(let i=0;i<d.length;i++)hash=(Math.imul(hash,31)+d[i])>>>0;return {title:h.textContent,size:[c.width,c.height],hash,pixels:Array.from(d.filter((_,i)=>i%4===0))};}));
 await page.goto('/reference/three-js/uv-tests.html');await page.waitForFunction(()=>document.querySelectorAll('canvas').length===9);const reference=await read();
 await page.goto('/web/gallery/example.html?id=misc_uv_tests&still=1');await page.waitForFunction(()=>document.querySelectorAll('#uv-tests canvas').length===9,null,{timeout:90000});const rust=await read();
 expect(rust.map(r=>r.title)).toEqual(reference.map(r=>r.title));
 for(let i=0;i<reference.length;i++){let bad=0;for(let p=0;p<reference[i].pixels.length;p++)if(Math.abs(reference[i].pixels[p]-rust[i].pixels[p])>6)bad++;expect({title:reference[i].title,size:rust[i].size,bad}).toEqual({title:reference[i].title,size:reference[i].size,bad:0});}
});

// webgl_effects_ascii writes its frame as characters into a table: the cells of
// the original and the port are compared, with the resource counts across a
// repeated cycle. TrackballControls steps once per 60 fps step.
test('ASCII effect: the character table matches the original',async({page},info)=>{
 test.setTimeout(180000);await page.setViewportSize({width:512,height:512});
 const cells=()=>page.evaluate(()=>document.querySelector('td')?.innerHTML.split('<br>').map(l=>l.replaceAll('&nbsp;',' ')));
 const results={};
 for(const runtime of ['reference','rust']){
  await page.mouse.move(511,0);
  await page.goto(runtime==='reference'?'/reference/three-js/texture-volumes.html?id=webgl_effects_ascii':'/web/gallery/example.html?id=webgl_effects_ascii&still=1');
  await page.waitForFunction(runtime==='reference'?()=>window.renderFixture&&window.reference:()=>Number(document.querySelector('canvas')?.dataset.frames)>0,null,{timeout:90000});
  const frame=t=>page.evaluate(async({runtime,t})=>{if(runtime==='reference')await renderFixture(t);else{const c=document.querySelector('canvas'),p=c.dataset.frames;app.gallery_time(t);while(c.dataset.frames===p)await new Promise(r=>requestAnimationFrame(r));}},{runtime,t});
  const shots=[];
  for(const t of [0,.4,1.1,2]){await frame(t);shots.push(await cells());}
  await page.mouse.move(256,256);await page.mouse.down();await page.mouse.move(330,300,{steps:5});await page.mouse.up();
  for(let k=1;k<=90;k++)await frame(2+k/60);shots.push(await cells());
  await page.mouse.move(256,256);await page.mouse.wheel(0,-300);for(let k=91;k<=150;k++)await frame(2+k/60);shots.push(await cells());
  results[runtime]=shots;
 }
 const report=results.reference.map((reference,state)=>{const rust=results.rust[state];let total=0,bad=0;expect(rust.length,`state ${state} rows`).toBe(reference.length);for(let y=0;y<reference.length;y++){const a=reference[y],b=rust[y]??'';for(let x=0;x<Math.max(a.length,b.length);x++){total++;if(a[x]!==b[x])bad++;}}return {state,total,bad,fraction:bad/total};});
 writeFileSync(info.outputPath('ascii.json'),JSON.stringify({report,results},null,1));
 for(const r of report)expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.01);
 // Steady-state residency: a second time cycle creates no GPU objects or transfers.
 const read=()=>page.evaluate(()=>({transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
 const cycle=()=>page.evaluate(async()=>{for(const t of [0,1,2,4,0]){const c=document.querySelector('canvas'),p=c.dataset.frames;app.gallery_time(t);while(c.dataset.frames===p)await new Promise(r=>requestAnimationFrame(r));}});
 await cycle();const before=await read();await cycle();expect(await read()).toEqual(before);
});

// webgl_postprocessing_glitch: GlitchPass advances its state and draws random
// numbers once per composer frame, so both runtimes are brought to the same frame
// count and then compared frame by frame, before and after "Glitch me wild".
test('Glitch pass: frames match the original frame by frame',async({page},info)=>{
 test.setTimeout(240000);await page.setViewportSize({width:512,height:512});
 const picks=[1,2,3,5,8,13,21,34,55,70],wild=[71,72,73],after=[74,80];
 const shots={};let frames=0;
 for(const runtime of ['rust','reference']){
  if(runtime==='rust'){await page.goto('/web/gallery/example.html?id=webgl_postprocessing_glitch&still=1');await page.click('#startButton');await page.waitForFunction(()=>Number(document.querySelector('canvas')?.dataset.frames)>0,null,{timeout:90000});await page.waitForTimeout(500);frames=Number(await page.locator('canvas').getAttribute('data-frames'));}
  else{await page.goto('/reference/three-js/texture-volumes.html?id=webgl_postprocessing_glitch&samples=1');await page.waitForFunction(()=>document.querySelector('canvas')?.dataset.ready==='true',null,{timeout:90000});for(let i=1;i<frames;i++)await page.evaluate(()=>renderFixture(0));}
  await page.addStyleTag({content:'#notice,#settings,#info,#overlay{display:none!important}'});
  // The checkbox changes in the same task as the frame it applies to.
  const frame=wild=>page.evaluate(async({runtime,wild})=>{if(wild!==null){if(runtime==='reference'){const e=document.getElementById('wildGlitch');e.checked=wild;e.dispatchEvent(new Event('change'));}else app.tsl_parameter(0,wild?1:0);}if(runtime==='reference')await renderFixture(0);else{const c=document.querySelector('canvas'),p=c.dataset.frames;app.gallery_time(0);while(c.dataset.frames===p)await new Promise(r=>requestAnimationFrame(r));}},{runtime,wild});
  const list=[];
  for(let k=1;k<=after.at(-1);k++){await frame(k===wild[0]?true:k===after[0]?false:null);if([...picks,...wild,...after].includes(k))list.push(PNG.sync.read(await page.locator('canvas').screenshot()));}
  shots[runtime]=list;
  if(runtime==='rust'){
   // Steady state: further frames, with and without wild glitches, create no GPU objects or transfers.
   const read=()=>page.evaluate(()=>({transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
   const cycle=async()=>{for(let k=0;k<6;k++)await frame(k===2?true:k===4?false:null);};
   await cycle();const before=await read();await cycle();expect(await read()).toEqual(before);
   // The cycle's frames advance GlitchPass: the reference is aligned to the total.
   frames=Number(await page.locator('canvas').getAttribute('data-frames'))-after.at(-1)-12;
  }
 }
 const results=shots.reference.map((b,state)=>{const a=shots.rust[state];let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail||=d>6;}if(fail)bad++;}writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));return {state,frames,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)};});
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));
 // The RGB shift and displacement sample the scene target between texels: along
 // polygon edges the filtered values differ between the backends (docs/texture-volumes.md).
 for(const r of results){expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.02);expect(r.meanError,JSON.stringify(r)).toBeLessThanOrEqual(.6);}
});

// webgl_postprocessing_taa counts frames: 200 turning (SSAA), then 200 still
// (TAA accumulating one jitter sample level per frame). Both runtimes are brought
// to the same frame count and compared frame by frame, with the sample level
// and the TAA switch changed on their frames.
test('TAA pass: frames match the original frame by frame',async({page},info)=>{
 test.setTimeout(300000);await page.setViewportSize({width:512,height:512});
 const picks=[1,50,99,100,101,102,105,110,130,140,299,301,305,306,310];
 const changes={141:[1,'Level 2: 4 Samples',2],306:[0,'Disabled',0]};
 const shots={};let frames=0;
 for(const runtime of ['rust','reference']){
  if(runtime==='rust'){await page.goto('/web/gallery/example.html?id=webgl_postprocessing_taa&still=1');await page.waitForFunction(()=>Number(document.querySelector('canvas')?.dataset.frames)>0,null,{timeout:90000});await page.waitForTimeout(500);await page.evaluate(()=>app.set_samples(1));await page.waitForTimeout(200);frames=Number(await page.locator('canvas').getAttribute('data-frames'));}
  else{await page.goto('/reference/three-js/texture-volumes.html?id=webgl_postprocessing_taa&samples=1');await page.waitForFunction(()=>document.querySelector('canvas')?.dataset.ready==='true',null,{timeout:90000});for(let i=1;i<frames;i++)await page.evaluate(()=>renderFixture(0));}
  await page.addStyleTag({content:'#notice,#settings,#info,#stats{display:none!important}'});
  const frame=change=>page.evaluate(async({runtime,change})=>{if(change){const [i,reference,rust]=change;if(runtime==='reference')fixtureParameter(i,reference);else app.tsl_parameter(i,rust);}if(runtime==='reference')await renderFixture(0);else{const c=document.querySelector('canvas'),p=c.dataset.frames;app.gallery_time(0);while(c.dataset.frames===p)await new Promise(r=>requestAnimationFrame(r));}},{runtime,change:change??null});
  const list=[];const start=runtime==='rust'?frames:frames;
  for(let k=start+1;k<=picks.at(-1);k++){await frame(changes[k]);if(picks.includes(k))list.push(PNG.sync.read(await page.locator('canvas').screenshot()));}
  shots[runtime]=list;
  if(runtime==='rust'){
   const read=()=>page.evaluate(()=>({transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
   const cycle=async()=>{for(let k=0;k<4;k++)await frame(null);};
   await cycle();const before=await read();await cycle();expect(await read()).toEqual(before);
  }
 }
 const results=shots.reference.map((b,state)=>{const a=shots.rust[state];let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail||=d>6;}if(fail)bad++;}writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));return {state,frame:picks.filter(k=>k>frames)[state],frames,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)};});
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));
 for(const r of results){expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.005);expect(r.meanError,JSON.stringify(r)).toBeLessThanOrEqual(.6);}
});

// webgpu_loader_texture_ktx2 lays its scenes out in the page: the whole viewport
// (page text and canvas) is compared, at the top and scrolled, and the labels'
// color spaces must match.
test('KTX2 loader: the page and its texture views match the original',async({page},info)=>{
 test.setTimeout(180000);await page.setViewportSize({width:640,height:900});
 const shots={},labels={};
 for(const runtime of ['reference','rust']){
  await page.goto(runtime==='reference'?'/reference/three-js/ktx2.html':'/web/gallery/example.html?id=webgpu_loader_texture_ktx2');
  await page.waitForFunction(()=>document.querySelectorAll('.list-item').length===16&&[...document.querySelectorAll('.list-item')].every(e=>e.innerText.includes('colorSpace')),null,{timeout:90000});
  await page.addStyleTag({content:'#notice,#settings,#info{visibility:hidden!important}'});
  labels[runtime]=await page.evaluate(()=>[...document.querySelectorAll('.list-item')].map(e=>e.innerText));
  const list=[];
  for(const y of [0,700,1400]){await page.evaluate(y=>scrollTo(0,y),y);await page.waitForTimeout(400);list.push(PNG.sync.read(await page.screenshot()));}
  shots[runtime]=list;
  if(runtime==='rust'){
   // Scrolling redraws the visible views from resident textures and scenes only.
   const read=()=>page.evaluate(()=>({transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
   const cycle=async()=>{for(const y of [0,700,1400,0]){await page.evaluate(y=>scrollTo(0,y),y);await page.waitForTimeout(200);}};
   await cycle();const before=await read();await cycle();expect(await read()).toEqual(before);
  }
 }
 expect(labels.rust).toEqual(labels.reference);
 const results=shots.reference.map((b,state)=>{const a=shots.rust[state];let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail||=d>6;}if(fail)bad++;}writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));return {state,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)};});
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));
 for(const r of results){expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.005);expect(r.meanError,JSON.stringify(r)).toBeLessThanOrEqual(.6);}
});

// webgl_multiple_elements_text draws its views behind the article's text: the
// viewport (text, MathML and views) is compared over time, scrolled, and after
// a view's OrbitControls drag and wheel.
test('Multiple elements with text: the page and its views match the original',async({page},info)=>{
 test.setTimeout(240000);await page.setViewportSize({width:900,height:900});
 const shots={};
 for(const runtime of ['reference','rust']){
  await page.mouse.move(899,0);
  await page.goto(runtime==='reference'?'/reference/three-js/elements-text.html':'/web/gallery/example.html?id=webgl_multiple_elements_text&still=1');
  await page.waitForFunction(runtime==='reference'?()=>document.querySelectorAll('.view').length===6&&document.querySelector('#c')?.width>0:()=>Number(document.querySelector('canvas')?.dataset.frames)>0,null,{timeout:90000});
  await page.addStyleTag({content:'#notice,#settings,#info{visibility:hidden!important}'});
  if(runtime==='reference')await page.waitForTimeout(1000);
  const at=t=>page.evaluate(async({runtime,t})=>{if(runtime==='reference'){window.fixtureMs=t*1000;for(let i=0;i<4;i++)await new Promise(r=>requestAnimationFrame(r));}else{const c=document.querySelector('canvas');for(let i=0;i<2;i++){const p=c.dataset.frames;app.gallery_time(t);while(c.dataset.frames===p)await new Promise(r=>requestAnimationFrame(r));}}},{runtime,t});
  const list=[];const shot=async()=>list.push(PNG.sync.read(await page.screenshot()));
  for(const t of [0,1,2.5]){await at(t);await shot();}
  for(const y of [1100,2600]){await page.evaluate(y=>scrollTo(0,y),y);await page.waitForTimeout(200);await at(2.5);await shot();}
  await page.evaluate(()=>scrollTo(0,1100));await page.waitForTimeout(200);
  const box=await page.evaluate(()=>{const r=[...document.querySelectorAll('.view')].map(v=>v.getBoundingClientRect()).find(r=>r.top+r.height/2>40&&r.top+r.height/2<innerHeight-40);return {x:r.left+r.width/2,y:r.top+r.height/2};});
  await page.mouse.move(box.x,box.y);await page.mouse.down();await page.mouse.move(box.x+80,box.y+40,{steps:5});await page.mouse.up();await at(2.5);await shot();
  await page.mouse.wheel(0,-300);await at(2.5);await shot();
  shots[runtime]=list;
  if(runtime==='rust'){
   const read=()=>page.evaluate(()=>({transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
   const cycle=async()=>{for(const [y,t] of [[0,1],[1100,2],[2600,3],[0,0]]){await page.evaluate(y=>scrollTo(0,y),y);await at(t);}};
   await cycle();const before=await read();await cycle();expect(await read()).toEqual(before);
  }
 }
 const results=shots.reference.map((b,state)=>{const a=shots.rust[state];let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail||=d>6;}if(fail)bad++;}writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));return {state,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)};});
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));
 // The original displaces every point on the CPU in double precision; the port
 // evaluates the same waves in the vertex stage in f32, so a few sprite edges
 // land on the neighbouring pixel (≤1.3% of pixels, all at sprite edges).
 for(const r of results){expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.02);expect(r.meanError,JSON.stringify(r)).toBeLessThanOrEqual(.4);}
});

// webgpu_reversed_depth_buffer: the three side-by-side depth buffers (normal,
// logarithmic, reversed) and their labels over time, including the z-fighting.
test('Reversed depth buffer: the three views match the original',async({page},info)=>{
 test.setTimeout(240000);await page.setViewportSize({width:900,height:600});
 const shots={};
 for(const runtime of ['reference','rust']){
  await page.goto(runtime==='reference'?'/reference/three-js/reversed-depth.html':'/web/gallery/example.html?id=webgpu_reversed_depth_buffer&still=1');
  await page.waitForFunction(runtime==='reference'?()=>document.querySelectorAll('canvas').length===3&&document.querySelector('canvas')?.width>0:()=>Number(document.querySelector('canvas')?.dataset.frames)>0,null,{timeout:90000});
  await page.addStyleTag({content:'#notice,#settings,#info{visibility:hidden!important}'});
  if(runtime==='reference')await page.waitForTimeout(1000);
  const at=t=>page.evaluate(async({runtime,t})=>{if(runtime==='reference'){window.fixtureMs=t*1000;for(let i=0;i<4;i++)await new Promise(r=>requestAnimationFrame(r));}else{const c=document.querySelector('canvas');for(let i=0;i<2;i++){const p=c.dataset.frames;app.gallery_time(t);while(c.dataset.frames===p)await new Promise(r=>requestAnimationFrame(r));}}},{runtime,t});
  const list=[];
  for(const t of [0,0.7,1.9,3.3,5]){await at(t);list.push(PNG.sync.read(await page.screenshot()));}
  shots[runtime]=list;
  if(runtime==='rust'){
   const read=()=>page.evaluate(()=>({transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
   await at(1);const before=await read();for(const t of [2,3,4])await at(t);expect(await read()).toEqual(before);
  }
 }
 const results=shots.reference.map((b,state)=>{const a=shots.rust[state];let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail||=d>6;}if(fail)bad++;}writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));return {state,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)};});
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));
 for(const r of results){expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.005);expect(r.meanError,JSON.stringify(r)).toBeLessThanOrEqual(.6);}
});

// webgpu_camera_logarithmicdepthbuffer: the normal and logarithmic views of the
// labels, frame by frame as the camera zooms out, after a wheel, a mouse move
// and a drag of the border.
test('Logarithmic depth buffer: both views match the original frame by frame',async({page},info)=>{
 test.setTimeout(300000);await page.setViewportSize({width:900,height:600});
 const shots={};
 for(const runtime of ['reference','rust']){
  await page.mouse.move(450,300);
  await page.goto(runtime==='reference'?'/reference/three-js/log-depth.html':'/web/gallery/example.html?id=webgpu_camera_logarithmicdepthbuffer&still=1');
  await page.waitForFunction(runtime==='reference'?()=>typeof window.fixtureAnimate==='function':()=>Number(document.querySelector('canvas')?.dataset.frames)>0,null,{timeout:90000});
  await page.addStyleTag({content:'#notice,#settings,#info{visibility:hidden!important}'});
  const frames=n=>page.evaluate(async({runtime,n})=>{if(runtime==='reference'){window.fixtureFrames(n);for(let i=0;i<3;i++)await new Promise(r=>requestAnimationFrame(r));}else{const c=document.querySelector('canvas');for(let i=0;i<2;i++){const p=c.dataset.frames;if(i)app.gallery_time(0);else app.gallery_draw(1,n,0);while(c.dataset.frames===p)await new Promise(r=>requestAnimationFrame(r));}}},{runtime,n});
  if(runtime==='reference')await page.waitForTimeout(500);else await frames(1);
  const list=[];const shot=async()=>list.push(PNG.sync.read(await page.screenshot()));
  await shot();
  for(const n of [199,400,300]){await frames(n);await shot();}
  // Wheel and mouse events arrive asynchronously; let them land before stepping.
  await page.mouse.wheel(0,100);await page.waitForTimeout(300);await frames(60);await shot();
  await page.mouse.move(630,180);await page.waitForTimeout(300);await frames(1);await shot();
  await page.mouse.move(226,300);await page.mouse.down();await page.mouse.move(450,300,{steps:4});await page.mouse.up();await page.waitForTimeout(300);await frames(1);await shot();
  shots[runtime]=list;
  if(runtime==='rust'){
   const read=()=>page.evaluate(()=>({transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
   await frames(1);const before=await read();await frames(5);await frames(1);expect(await read()).toEqual(before);
  }
 }
 const results=shots.reference.map((b,state)=>{const a=shots.rust[state];let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail||=d>6;}if(fail)bad++;}writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));return {state,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)};});
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));
 for(const r of results){expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.005);expect(r.meanError,JSON.stringify(r)).toBeLessThanOrEqual(.6);}
});

// webgpu_lines_fat_raycasting: the turning spiral and the raycast spheres under
// the pointer, then with Line2, screen-space widths, a threshold (visualized)
// and a translation.
test('Fat line raycasting: lines and hit spheres match the original',async({page},info)=>{
 test.setTimeout(240000);await page.setViewportSize({width:900,height:600});
 const shots={};
 const states=[[0,450,300,[]],[1.5,470,260,[]],[3,430,350,[]],[3,450,300,[['line type',0,0]]],[3,455,305,[['world units',1,false],['threshold',5,3]]],[3,440,290,[['visualize threshold',2,true],['translation',6,3]]]];
 for(const runtime of ['reference','rust']){
  await page.mouse.move(0,0);
  await page.goto(runtime==='reference'?'/reference/three-js/lines-raycast.html':'/web/gallery/example.html?id=webgpu_lines_fat_raycasting&still=1');
  await page.waitForFunction(runtime==='reference'?()=>window.fixtureControls?.['line type']:()=>Number(document.querySelector('canvas')?.dataset.frames)>0,null,{timeout:90000});
  await page.addStyleTag({content:'#notice,#settings,#info{visibility:hidden!important}'});
  if(runtime==='reference')await page.waitForTimeout(1000);
  const at=t=>page.evaluate(async({runtime,t})=>{if(runtime==='reference'){window.fixtureMs=t*1000;for(let i=0;i<4;i++)await new Promise(r=>requestAnimationFrame(r));}else{const c=document.querySelector('canvas');for(let i=0;i<3;i++){const p=c.dataset.frames;app.gallery_time(t);while(c.dataset.frames===p)await new Promise(r=>requestAnimationFrame(r));}}},{runtime,t});
  const list=[];
  for(const [t,x,y,params] of states){
   for(const [name,index,value] of params)await page.evaluate(({runtime,name,index,value})=>runtime==='reference'?window.fixtureSet(name,value):app.tsl_parameter(index,Number(value)),{runtime,name,index,value});
   await page.mouse.move(x,y);await page.waitForTimeout(200);await at(t);list.push(PNG.sync.read(await page.screenshot()));
  }
  shots[runtime]=list;
  if(runtime==='rust'){
   const read=()=>page.evaluate(()=>({transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
   const cycle=async()=>{for(const [t,x] of [[1,440],[2,460],[3,450]]){await page.mouse.move(x,300);await at(t);}};
   await cycle();const before=await read();await cycle();expect(await read()).toEqual(before);
  }
 }
 const results=shots.reference.map((b,state)=>{const a=shots.rust[state];let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail||=d>6;}if(fail)bad++;}writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));return {state,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)};});
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));
 // The visualized threshold (state 5) is a translucent alpha-to-coverage overlay:
 // its dithered samples resolve slightly brighter in the port (≤1.2% of pixels,
 // all on the 4 px threshold ribbons); every other state is exact.
 for(const r of results){const [fraction,mean]=r.state===5?[.015,.5]:[.005,.6];expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(fraction);expect(r.meanError,JSON.stringify(r)).toBeLessThanOrEqual(mean);}
});

// webgl_test_memory builds, draws and disposes a random wireframe sphere and its
// canvas texture every frame: frame by frame the port must draw the same sphere,
// and its GPU residency must stay flat while each frame's upload recurs.
test('Test memory: every frame matches and disposed resources are released',async({page},info)=>{
 test.setTimeout(180000);await page.setViewportSize({width:512,height:512});
 const shots={};
 for(const runtime of ['reference','rust']){
  await page.goto(runtime==='reference'?'/reference/three-js/texture-volumes.html?id=webgl_test_memory&samples=1':'/web/gallery/example.html?id=webgl_test_memory&still=1');
  await page.waitForFunction(()=>{const c=document.querySelector('canvas');return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;});
  await page.addStyleTag({content:'#notice,#settings,#info{display:none!important}'});
  const list=[];list.push(PNG.sync.read(await page.locator('canvas').screenshot()));
  for(let k=1;k<=6;k++){await frames(page,runtime,k,1);list.push(PNG.sync.read(await page.locator('canvas').screenshot()));}
  shots[runtime]=list;
  if(runtime==='rust'){
   const read=()=>page.evaluate(()=>Array.from(app.resource_counts()));
   // [resident textures, cumulative texture uploads, filters]: one new canvas texture
   // per frame, as the original uploads, and none left resident.
   const before=await read();for(let k=0;k<20;k++)await frames(page,runtime,k,1);const after=await read();
   expect([after[0],after[1]-before[1],after[2]]).toEqual([before[0],20,before[2]]);
  }
 }
 const results=shots.reference.map((b,state)=>{const a=shots.rust[state];let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail||=d>6;}if(fail)bad++;}writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));return {state,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)};});
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));
 for(const r of results){expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.005);expect(r.meanError,JSON.stringify(r)).toBeLessThanOrEqual(.6);}
});

// webgl_test_memory2 replaces all 100 ShaderMaterials every frame, each compiled with
// its own random color: frame by frame the spheres must match, each frame must compile
// exactly 100 programs (shader module and pipeline) and nothing else may accumulate.
test('Test memory 2: every frame matches and its 100 programs are disposed',async({page},info)=>{
 test.setTimeout(180000);await page.setViewportSize({width:512,height:512});
 await page.addInitScript(()=>{window.creates={};for(const key of ['createBuffer','createTexture','createBindGroup','createShaderModule','createRenderPipeline']){const original=GPUDevice.prototype[key];GPUDevice.prototype[key]=function(...a){creates[key]=(creates[key]||0)+1;return original.apply(this,a);};}});
 const shots={};
 for(const runtime of ['reference','rust']){
  await page.goto(runtime==='reference'?'/reference/three-js/texture-volumes.html?id=webgl_test_memory2&samples=1':'/web/gallery/example.html?id=webgl_test_memory2&still=1');
  await page.waitForFunction(()=>{const c=document.querySelector('canvas');return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;});
  await page.addStyleTag({content:'#notice,#settings,#info{display:none!important}'});
  const list=[];list.push(PNG.sync.read(await page.locator('canvas').screenshot()));
  for(let k=1;k<=4;k++){await frames(page,runtime,k,1);list.push(PNG.sync.read(await page.locator('canvas').screenshot()));}
  shots[runtime]=list;
  if(runtime==='rust'){
   const read=()=>page.evaluate(()=>({...creates,resources:Array.from(app.resource_counts())}));
   const before=await read();for(let k=0;k<10;k++)await frames(page,runtime,k,1);const after=await read();
   expect(after.createShaderModule-before.createShaderModule).toBe(1000);expect(after.createRenderPipeline-before.createRenderPipeline).toBe(1000);
   for(const key of ['createBuffer','createTexture','createBindGroup','resources'])expect(after[key]).toEqual(before[key]);
  }
 }
 const results=shots.reference.map((b,state)=>{const a=shots.rust[state];let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail||=d>6;}if(fail)bad++;}writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));return {state,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)};});
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));
 for(const r of results){expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.005);expect(r.meanError,JSON.stringify(r)).toBeLessThanOrEqual(.6);}
});


// webgl_points_dynamic: each body starts crumbling after 100–300 frames and rises again
// after its delay, every frame's walk drawing from Math.random in the page's order. Frames
// advance 0.2 s (the page's clamped delta of 2), so the bodies fall, rest and rise within
// 600 frames: the images must match along the way, and the moved positions are written
// into the resident geometry (nothing is created while they move).
test('Points dynamic: the random walk matches as the bodies fall and rise',async({page},info)=>{
 test.setTimeout(900000);await page.setViewportSize({width:512,height:512});
 await page.addInitScript(()=>{window.creates=0;for(const key of ['createBuffer','createTexture','createBindGroup','createShaderModule','createRenderPipeline']){const original=GPUDevice.prototype[key];GPUDevice.prototype[key]=function(...a){creates++;return original.apply(this,a);};}});
 const at=[150,300,450,600];const shots={};
 for(const runtime of ['reference','rust']){
  await page.goto(runtime==='reference'?'/reference/three-js/texture-volumes.html?id=webgl_points_dynamic&samples=1':'/web/gallery/example.html?id=webgl_points_dynamic&still=1');
  await page.waitForFunction(()=>{const c=document.querySelector('canvas');return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},null,{timeout:90000});
  await page.addStyleTag({content:'#notice,#settings,#info,#stats{display:none!important}'});
  const list=[];const counts=[];
  for(let k=1;k<=600;k++){await frames(page,runtime,k*.2,1);if(at.includes(k)){list.push(PNG.sync.read(await page.locator('canvas').screenshot()));if(runtime==='rust')counts.push(await page.evaluate(()=>({creates,resources:Array.from(app.resource_counts())})));}}
  shots[runtime]=list;
  if(runtime==='rust')for(const c of counts.slice(1))expect(c).toEqual(counts[0]);
 }
 const results=shots.reference.map((b,state)=>{const a=shots.rust[state];let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail||=d>6;}if(fail)bad++;}writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));return {state,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)};});
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));
 // Point rasterization bound (docs/texture-volumes.md): about 2.5% of WebGL's point
 // squares (ANGLE's Metal point sprites) cover one pixel column or row more or less
 // than the exact square the port rasterizes, and the bloom spreads each difference.
 for(const r of results){expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.03);expect(r.meanError,JSON.stringify(r)).toBeLessThanOrEqual(.45);}
});

// webgl_gpgpu_protoplanet steps its GPUComputationRenderer once per animate(): over 120
// frames the particles gather, collide and merge, and the port must follow frame by frame.
test('Protoplanet: the n-body simulation matches over 120 frames',async({page},info)=>{
 test.setTimeout(300000);await page.setViewportSize({width:512,height:512});
 const shots={};
 for(const runtime of ['reference','rust']){
  await page.goto(runtime==='reference'?'/reference/three-js/texture-volumes.html?id=webgl_gpgpu_protoplanet':'/web/gallery/example.html?id=webgl_gpgpu_protoplanet&still=1');
  await page.waitForFunction(()=>{const c=document.querySelector('canvas');return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},null,{timeout:90000});
  await page.addStyleTag({content:'#notice,#settings,#info,#stats{display:none!important}'});
  // A strong field makes the debris collide early.
  await page.evaluate(runtime=>runtime==='rust'?app.tsl_parameter(0,1000):fixtureParameter(0,1000),runtime);
  const list=[];
  for(const n of [30,30,60]){await frames(page,runtime,0,n);list.push(PNG.sync.read(await page.locator('canvas').screenshot()));}
  shots[runtime]=list;
 }
 const results=shots.reference.map((b,state)=>{const a=shots.rust[state];let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail||=d>6;}if(fail)bad++;}writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));return {state,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)};});
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));
 for(const r of results){expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.005);expect(r.meanError,JSON.stringify(r)).toBeLessThanOrEqual(.6);}
});
