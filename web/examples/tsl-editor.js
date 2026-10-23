// webgpu_tsl_editor: the page's Monaco editors, lil-gui options and preview,
// as the original builds them. THREE is the three-tsl crate ( three r186's
// node system, renderers and core transpiled to Rust ): the user's TSL runs
// as the original runs it ( an AsyncFunction with THREE, `three/tsl` through
// the gallery's import map ), and the shaders, errors and preview come from
// the port. The preview renders through the browser's GPUDevice, requested
// as WebGPUBackend requests it.
const MONACO = 'https://cdn.jsdelivr.net/npm/monaco-editor@0.55.1/min';
const GUI_URL = 'https://cdn.jsdelivr.net/npm/three@0.186.0/examples/jsm/libs/lil-gui.module.min.js';
// WebGPUBackend's GPUFeatureName values.
const FEATURES = [ 'core-features-and-limits', 'depth-clip-control', 'depth32float-stencil8', 'texture-compression-bc', 'texture-compression-bc-sliced-3d', 'texture-compression-etc2', 'texture-compression-astc', 'texture-compression-astc-sliced-3d', 'timestamp-query', 'indirect-first-instance', 'shader-f16', 'rg11b10ufloat-renderable', 'bgra8unorm-storage', 'float32-filterable', 'float32-blendable', 'clip-distances', 'dual-source-blending', 'subgroups', 'texture-formats-tier1', 'texture-formats-tier2' ];

const STYLE = `
			#source {
				position: absolute;
				top: 0;
				left: 0;
				width: 50%;
				height: 100%;
			}
			#result {
				position: absolute;
				top: 0;
				right: 0;
				width: 50%;
				height: 100%;
			}
			#renderer {
				position: absolute;
				bottom: 15px;
				right: calc( 50% + 15px );
				width: 200px;
				height: 200px;
				z-index: 100;
				pointer-events: none;
			}
		`;

function loadMonaco() {

	return new Promise( ( resolve, reject ) => {

		const script = document.createElement( 'script' );
		script.src = `${ MONACO }/vs/loader.js`;
		script.onload = () => resolve();
		script.onerror = () => reject( new Error( 'Monaco editor の読み込みに失敗しました。' ) );
		document.head.append( script );

	} );

}

// WebGPUBackend.init's adapter and device, made before the port needs them:
// the port's runtime is synchronous, so the device's asynchronous pipeline
// creation returns its result at once ( a settled thenable ).
async function requestDevice() {

	const adapter = await navigator.gpu.requestAdapter( { featureLevel: 'compatibility' } );
	if ( adapter === null ) throw new Error( 'THREE.WebGPUBackend: Unable to create WebGPU adapter.' );
	const requiredFeatures = FEATURES.filter( name => adapter.features.has( name ) );
	const device = await adapter.requestDevice( { requiredFeatures, requiredLimits: {} } );
	const settled = ( value ) => ( { __tslSettled: true, value, then( resolve ) {

		resolve( value );

	} } );
	device.createRenderPipelineAsync = ( descriptor ) => settled( device.createRenderPipeline( descriptor ) );
	device.createComputePipelineAsync = ( descriptor ) => settled( device.createComputePipeline( descriptor ) );
	return device;

}

export async function startTslEditor( loading ) {

	const style = document.createElement( 'style' );
	style.textContent = STYLE;
	document.head.append( style );
	for ( const id of [ 'source', 'result', 'renderer' ] ) {

		const div = document.createElement( 'div' );
		div.id = id;
		document.body.append( div );

	}

	globalThis.__tslRealDevice = await requestDevice();
	const revision = 'tsl-2';
	const tsl = await import( `../build/tsl/three_tsl.js?v=${ revision }` );
	await tsl.default( { module_or_path: new URL( `../build/tsl/three_tsl_bg.wasm?v=${ revision }`, import.meta.url ) } );
	tsl.tsl_init();
	const THREE = tsl.three_namespace();
	globalThis.__tslNamespace = tsl.tsl_namespace();
	const { vec4 } = await import( 'three/tsl' );
	const { GUI } = await import( GUI_URL );
	await loadMonaco();
	loading.hidden = true;

	// init(): the original's script from here on.
	const width = 200;
	const height = 200;
	const camera = new THREE.PerspectiveCamera( 70, width / height, 0.1, 10 );
	camera.position.z = .72;
	camera.lookAt( 0, 0, 0 );
	const scene = new THREE.Scene();
	scene.background = new THREE.Color( 0x222222 );
	const rendererDOM = document.getElementById( 'renderer' );
	const renderer = new THREE.WebGPURenderer( { antialias: true } );
	renderer.setPixelRatio( window.devicePixelRatio );
	renderer.setSize( 200, 200 );
	renderer.debug.diagnostics.keywords = true;
	rendererDOM.appendChild( renderer.domElement );
	const material = new THREE.NodeMaterial();
	material.fragmentNode = vec4( 0, 0, 0, 1 );
	const mesh = new THREE.Mesh( new THREE.PlaneGeometry( 1, 1 ), material );
	scene.add( mesh );
	let compiling = false;
	let frames = 0;
	renderer.setAnimationLoop( () => {

		if ( compiling ) return;
		renderer.render( scene, camera );
		renderer.domElement.dataset.frames = String( ++ frames );

	} );
	window.require.config( { paths: { 'vs': `${ MONACO }/vs` } } );
	window.require( [ 'vs/editor/editor.main' ], () => {

		const options = {
			shader: 'fragment',
			outputColorSpace: THREE.SRGBColorSpace,
			output: 'WGSL',
			preview: true
		};
		let timeout = null;
		let rawShader = null;
		const editorDOM = document.getElementById( 'source' );
		const resultDOM = document.getElementById( 'result' );
		const tslCode = `// Simple uv.x animation
const { texture, uniform, vec2, vec4, uv, oscSine, time, grayscale } = await import( 'three/tsl' );

const samplerTexture = new THREE.TextureLoader().load( './textures/uv_grid_opengl.jpg' );
samplerTexture.wrapS = THREE.RepeatWrapping;
samplerTexture.colorSpace = THREE.SRGBColorSpace;

const scaledTime = time.mul( .5 ); // .5 is speed
const uv0 = uv();
const animateUv = vec2( uv0.x.add( oscSine( scaledTime ) ), uv0.y );

// .setName() is optional
const myMap = texture( samplerTexture, animateUv ).rgb.setName( 'myTexture' );
const myColor = uniform( new THREE.Color( 0x0066ff ) ).setName( 'myColor' );
const opacity = .7;

const desaturatedMap = grayscale( myMap.rgb );

const finalColor = desaturatedMap.add( myColor );

output = vec4( finalColor, opacity );
`;
		const editor = window.monaco.editor.create( editorDOM, {
			value: tslCode,
			language: 'javascript',
			theme: 'vs-dark',
			automaticLayout: true,
			minimap: { enabled: false }
		} );
		const result = window.monaco.editor.create( resultDOM, {
			value: '',
			language: 'wgsl',
			theme: 'vs-dark',
			automaticLayout: true,
			readOnly: true,
			minimap: { enabled: false }
		} );
		const showCode = () => {

			result.setValue( rawShader[ options.shader + 'Shader' ] );
			result.revealLine( 1 );

		};

		const webGLRenderer = new THREE.WebGPURenderer( { forceWebGL: true } );
		webGLRenderer.debug.diagnostics.keywords = true;
		const build = async () => {

			try {

				const AsyncFunction = async function () {}.constructor;
				const tslCode = `let output = null;\n${ editor.getValue() }\nreturn { output };`;
				const nodes = await ( new AsyncFunction( 'THREE', tslCode )( THREE ) );
				mesh.material.fragmentNode = nodes.output;
				mesh.material.needsUpdate = true;
				compiling = true;
				mesh.visible = false;
				if ( options.output === 'WGSL' ) {

					rawShader = await renderer.debug.getShaderAsync( scene, camera, mesh );

				} else if ( options.output === 'GLSL ES 3.0' ) {

					rawShader = await webGLRenderer.debug.getShaderAsync( scene, camera, mesh );

				}

				mesh.visible = true;
				compiling = false;
				showCode();

			} catch ( e ) {

				result.setValue( 'Error: ' + e.message );

			}

			document.body.dataset.builds = String( Number( document.body.dataset.builds || 0 ) + 1 );

		};

		build();
		editor.getModel().onDidChangeContent( () => {

			if ( timeout ) clearTimeout( timeout );
			timeout = setTimeout( build, 1000 );

		} );

		// gui
		const gui = new GUI();
		gui.add( options, 'output', [ 'WGSL', 'GLSL ES 3.0' ] ).onChange( build );
		gui.add( options, 'shader', [ 'vertex', 'fragment' ] ).onChange( showCode );
		gui.add( options, 'outputColorSpace', [ THREE.LinearSRGBColorSpace, THREE.SRGBColorSpace ] ).onChange( ( value ) => {

			renderer.outputColorSpace = value;
			build();

		} );
		gui.add( options, 'preview' ).onChange( ( value ) => {

			rendererDOM.style.display = value ? '' : 'none';

		} );
		window.tslEditor = { editor, result, options, build, gui };
		document.body.dataset.ui = 'ready';

	} );

}
