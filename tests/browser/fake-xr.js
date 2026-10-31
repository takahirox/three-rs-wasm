// A WebXR device with the WebXR/WebGPU binding ( XRGPUBinding, Immersive Web
// editor's draft of 2026-06-15 ), for comparing the gallery's ports with the
// pinned r187 pages without a headset. It is injected with addInitScript and
// installs navigator.xr, XRGPUBinding, XRRigidTransform and XRWebGLLayer-free
// globals before the page runs.
//
// The device is stepped by the test: window.fakeXR.step( n ) runs n session
// frames ( each 1000 / 60 ms of performance.now ) and resolves when the GPU has
// finished them; fakeXR.windowFrames( n ) runs the window's animation frames
// before a session the same way, without advancing the clock. Nothing runs
// between steps. The views are a fixed
// stereo pair with asymmetric fields of view; the projections use WebGPU's
// [ 0, 1 ] depth range, as the binding defines for 'webgpu' sessions, with the
// render state's depthNear and depthFar. One right-hand tracked-pointer
// controller with the oculus-touch-v3 profile is connected at session start.
( () => {

	const EYE = 1024; // projection layer texture size, per view
	const IPD = 0.064;
	// Tangents of the eye fields of view: left, right, up, down.
	const FOV = [ [ - 1.0, 0.9, 0.95, - 1.05 ], [ - 0.9, 1.0, 0.95, - 1.05 ] ];

	let clock = 0;
	const realNow = performance.now.bind( performance );
	performance.now = () => clock;

	// The window's animation frames are stepped too ( fakeXR.windowFrames ),
	// so both pages run the same number of frames before a session.
	let windowCallbacks = new Map();
	let windowHandle = 0;
	window.requestAnimationFrame = callback => {

		windowCallbacks.set( ++ windowHandle, callback );
		return windowHandle;

	};
	window.cancelAnimationFrame = handle => windowCallbacks.delete( handle );

	const state = {
		session: null,
		// window.fakeXROptions = { ar: true } ( set by an earlier init script )
		// adds immersive-ar with an alpha-blend display.
		supported: { 'immersive-vr': true, 'immersive-ar': window.fakeXROptions?.ar === true },
		head: { position: [ 0, 0, 0 ], quaternion: [ 0, 0, 0, 1 ] },
		controller: { position: [ 0.25, - 0.35, - 0.4 ], quaternion: [ 0, 0, 0, 1 ] },
		grip: { position: [ 0.25, - 0.38, - 0.35 ], quaternion: [ - 0.3826834, 0, 0, 0.9238795 ] },
		trigger: 0,
		layer: null,
		device: null,
		frames: 0,
		events: []
	};

	function compose( p, q ) {

		const [ x, y, z, w ] = q;
		const x2 = x + x, y2 = y + y, z2 = z + z;
		const xx = x * x2, xy = x * y2, xz = x * z2, yy = y * y2, yz = y * z2, zz = z * z2;
		const wx = w * x2, wy = w * y2, wz = w * z2;
		return new Float32Array( [
			1 - ( yy + zz ), xy + wz, xz - wy, 0,
			xy - wz, 1 - ( xx + zz ), yz + wx, 0,
			xz + wy, yz - wx, 1 - ( xx + yy ), 0,
			p[ 0 ], p[ 1 ], p[ 2 ], 1
		] );

	}

	function multiply( a, b ) {

		const out = new Float32Array( 16 );
		for ( let c = 0; c < 4; c ++ ) for ( let r = 0; r < 4; r ++ ) {

			let sum = 0;
			for ( let k = 0; k < 4; k ++ ) sum += a[ k * 4 + r ] * b[ c * 4 + k ];
			out[ c * 4 + r ] = sum;

		}

		return out;

	}

	function invertRigid( m ) {

		const out = new Float32Array( 16 );
		for ( let r = 0; r < 3; r ++ ) for ( let c = 0; c < 3; c ++ ) out[ c * 4 + r ] = m[ r * 4 + c ];
		for ( let r = 0; r < 3; r ++ ) out[ 12 + r ] = - ( out[ r ] * m[ 12 ] + out[ 4 + r ] * m[ 13 ] + out[ 8 + r ] * m[ 14 ] );
		out[ 15 ] = 1;
		return out;

	}

	function projection( [ l, r, t, b ], n, f ) {

		const m = new Float32Array( 16 );
		m[ 0 ] = 2 / ( r - l );
		m[ 5 ] = 2 / ( t - b );
		m[ 8 ] = ( r + l ) / ( r - l );
		m[ 9 ] = ( t + b ) / ( t - b );
		m[ 10 ] = f / ( n - f );
		m[ 11 ] = - 1;
		m[ 14 ] = n * f / ( n - f );
		return m;

	}

	class FakeXRRigidTransform {

		constructor( position = {}, orientation = {} ) {

			const p = [ position.x ?? 0, position.y ?? 0, position.z ?? 0 ];
			const q = [ orientation.x ?? 0, orientation.y ?? 0, orientation.z ?? 0, orientation.w ?? 1 ];
			this.position = DOMPointReadOnly.fromPoint( { x: p[ 0 ], y: p[ 1 ], z: p[ 2 ], w: 1 } );
			this.orientation = DOMPointReadOnly.fromPoint( { x: q[ 0 ], y: q[ 1 ], z: q[ 2 ], w: q[ 3 ] } );
			this.matrix = compose( p, q );

		}

		get inverse() {

			const t = new FakeXRRigidTransform();
			t.matrix = invertRigid( this.matrix );
			return t;

		}

	}

	class FakeXRSpace extends EventTarget {

		constructor( name, offset = null ) {

			super();
			this.name = name;
			this.offset = offset;

		}

	}

	class FakeXRReferenceSpace extends FakeXRSpace {

		getOffsetReferenceSpace( transform ) {

			return new FakeXRReferenceSpace( this.name, transform );

		}

	}

	// Poses are relative to the device's tracking origin at the viewer's start.
	// This maps them into a reference space: local-floor's origin is on the
	// floor, 1.6 m below that start; local's is at the start.
	function spaceMatrix( space ) {

		const height = space.name === 'local-floor' || space.name === 'bounded-floor' ? 1.6 : 0;
		let m = compose( [ 0, height, 0 ], [ 0, 0, 0, 1 ] );
		if ( space.offset ) m = multiply( invertRigid( space.offset.matrix ), m );
		return m;

	}

	const views = [ 0, 1 ].map( i => ( { eye: i === 0 ? 'left' : 'right', index: i } ) );

	class FakeXRView {

		constructor( frame, view, base ) {

			const session = frame.session;
			const head = compose( state.head.position, state.head.quaternion );
			const eye = compose( [ ( view.index === 0 ? - 0.5 : 0.5 ) * IPD, 0, 0 ], [ 0, 0, 0, 1 ] );
			const matrix = multiply( spaceMatrix( base ), multiply( head, eye ) );
			this.eye = view.eye;
			this.index = view.index;
			this.transform = new FakeXRRigidTransform();
			this.transform.matrix = matrix;
			this.projectionMatrix = projection( FOV[ view.index ], session.renderState.depthNear, session.renderState.depthFar );
			this.recommendedViewportScale = 1;

		}

		requestViewportScale() {}

	}

	class FakeXRInputSource {

		constructor() {

			this.handedness = 'right';
			this.targetRayMode = 'tracked-pointer';
			this.targetRaySpace = new FakeXRSpace( 'target-ray' );
			this.gripSpace = new FakeXRSpace( 'grip' );
			this.profiles = [ 'oculus-touch-v3', 'oculus-touch-v2', 'oculus-touch', 'generic-trigger-squeeze-thumbstick', 'generic-trigger' ];
			this.hand = null;
			const button = () => ( { pressed: false, touched: false, value: 0 } );
			this.gamepad = { id: '', index: - 1, connected: true, mapping: 'xr-standard', buttons: [ button(), button(), button(), button(), button(), button(), button() ], axes: [ 0, 0, 0, 0 ], timestamp: 0 };

		}

	}

	class FakeXRFrame {

		constructor( session ) {

			this.session = session;
			this.predictedDisplayTime = clock;

		}

		getViewerPose( base ) {

			const pose = { views: views.map( v => new FakeXRView( this, v, base ) ), emulatedPosition: false };
			pose.transform = new FakeXRRigidTransform();
			pose.transform.matrix = multiply( spaceMatrix( base ), compose( state.head.position, state.head.quaternion ) );
			return pose;

		}

		getPose( space, base ) {

			let local;
			if ( space.name === 'target-ray' ) local = compose( state.controller.position, state.controller.quaternion );
			else if ( space.name === 'grip' ) local = compose( state.grip.position, state.grip.quaternion );
			else return null;
			const head = compose( state.head.position, state.head.quaternion );
			const transform = new FakeXRRigidTransform();
			transform.matrix = multiply( spaceMatrix( base ), multiply( head, local ) );
			return { transform, emulatedPosition: false };

		}

	}

	class FakeXRSession extends EventTarget {

		constructor( mode, enabledFeatures ) {

			super();
			this.mode = mode;
			this.enabledFeatures = enabledFeatures;
			this.renderState = { depthNear: 0.1, depthFar: 1000, inlineVerticalFieldOfView: null, baseLayer: null, layers: [] };
			this.inputSources = [];
			this.visibilityState = 'visible';
			this.environmentBlendMode = mode === 'immersive-ar' ? 'alpha-blend' : 'opaque';
			this.interactionMode = 'world-space';
			this.frameRate = 60;
			this.supportedFrameRates = new Float32Array( [ 60 ] );
			this._callbacks = new Map();
			this._handle = 0;
			this._ended = false;

		}

		updateRenderState( init = {} ) {

			for ( const key of Object.keys( init ) ) this.renderState[ key ] = init[ key ];

		}

		async requestReferenceSpace( type ) {

			if ( ! [ 'viewer', 'local', 'local-floor', 'bounded-floor' ].includes( type ) ) throw new DOMException( `The ${ type } reference space is not supported.`, 'NotSupportedError' );
			return new FakeXRReferenceSpace( type );

		}

		requestAnimationFrame( callback ) {

			const handle = ++ this._handle;
			this._callbacks.set( handle, callback );
			return handle;

		}

		cancelAnimationFrame( handle ) {

			this._callbacks.delete( handle );

		}

		updateTargetFrameRate() {

			return Promise.resolve();

		}

		async end() {

			if ( this._ended ) return;
			this._ended = true;
			state.session = null;
			this.dispatchEvent( new Event( 'end' ) );

		}

		_frame() {

			const callbacks = [ ...this._callbacks.values() ];
			this._callbacks.clear();
			const frame = new FakeXRFrame( this );
			for ( const callback of callbacks ) callback( clock, frame );

		}

		_input( type ) {

			const event = new Event( type );
			event.inputSource = this.inputSources[ 0 ];
			event.frame = new FakeXRFrame( this );
			this.dispatchEvent( event );

		}

	}

	class XRGPUSubImage {

		constructor( texture, index ) {

			this.colorTexture = texture;
			this.depthStencilTexture = null;
			this.motionVectorTexture = null;
			this.viewport = { x: 0, y: 0, width: EYE, height: EYE };
			this._index = index;

		}

		getViewDescriptor() {

			return { dimension: '2d', baseArrayLayer: this._index, arrayLayerCount: 1, baseMipLevel: 0, mipLevelCount: 1 };

		}

	}

	class XRGPUBinding {

		constructor( session, device ) {

			if ( ! ( session instanceof FakeXRSession ) ) throw new TypeError( "Failed to construct 'XRGPUBinding': parameter 1 is not of type 'XRSession'." );
			if ( ! session.enabledFeatures.includes( 'webgpu' ) ) throw new DOMException( "Failed to construct 'XRGPUBinding': The session was not created with the 'webgpu' feature.", 'InvalidStateError' );
			this.session = session;
			this.device = device;
			this.nativeProjectionScaleFactor = 1;
			state.device = device;

		}

		getPreferredColorFormat() {

			return navigator.gpu.getPreferredCanvasFormat();

		}

		createProjectionLayer( init = {} ) {

			const format = init.colorFormat ?? this.getPreferredColorFormat();
			const texture = this.device.createTexture( {
				label: 'fake XR projection layer',
				size: [ EYE, EYE, 2 ],
				format,
				usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC | GPUTextureUsage.TEXTURE_BINDING
			} );
			const layer = { textureWidth: EYE, textureHeight: EYE, textureArrayLength: 2, ignoreDepthValues: true, fixedFoveation: 0, colorTexture: texture, format };
			state.layer = layer;
			return layer;

		}

		getViewSubImage( layer, view ) {

			return new XRGPUSubImage( layer.colorTexture, view.index );

		}

	}

	const xr = new EventTarget();
	xr.isSessionSupported = async ( mode ) => !! state.supported[ mode ];
	xr.requestSession = async ( mode, init = {} ) => {

		if ( ! state.supported[ mode ] ) throw new DOMException( `The specified session mode is not supported: ${ mode }`, 'NotSupportedError' );
		if ( state.session ) throw new DOMException( 'An immersive session is already active.', 'InvalidStateError' );
		const known = [ 'viewer', 'local', 'local-floor', 'bounded-floor', 'webgpu', 'layers', 'hand-tracking' ];
		const required = init.requiredFeatures ?? [];
		for ( const f of required ) if ( ! known.includes( f ) ) throw new DOMException( `Feature ${ f } is not supported.`, 'NotSupportedError' );
		// 'layers' is not offered by the WebGPU projection-layer runtime here.
		const enabled = [ 'viewer', 'local', ...[ ...required, ...( init.optionalFeatures ?? [] ) ].filter( f => known.includes( f ) && f !== 'layers' && f !== 'hand-tracking' ) ];
		const session = new FakeXRSession( mode, [ ...new Set( enabled ) ] );
		state.session = session;
		state.events.push( { type: 'requestSession', mode, init: JSON.parse( JSON.stringify( init ) ) } );
		// The controller connects once the page listens ( after setSession's first await ).
		setTimeout( () => {

			if ( state.session !== session ) return;
			const source = new FakeXRInputSource();
			session.inputSources.push( source );
			const event = new Event( 'inputsourceschange' );
			event.added = [ source ];
			event.removed = [];
			session.dispatchEvent( event );

		}, 0 );
		return session;

	};

	Object.defineProperty( navigator, 'xr', { value: xr, configurable: true } );
	globalThis.XRGPUBinding = XRGPUBinding;
	globalThis.XRRigidTransform = FakeXRRigidTransform;
	globalThis.XRSession = FakeXRSession;
	globalThis.XRFrame = FakeXRFrame;
	globalThis.XRReferenceSpace = FakeXRReferenceSpace;

	// The adapter request must ask for an XR-compatible adapter.
	const requestAdapter = GPU.prototype.requestAdapter;
	GPU.prototype.requestAdapter = function ( options = {} ) {

		state.events.push( { type: 'requestAdapter', xrCompatible: options.xrCompatible === true } );
		return requestAdapter.call( this, options );

	};

	async function settle() {

		if ( state.device ) await state.device.queue.onSubmittedWorkDone();

	}

	window.fakeXR = {
		state,
		EYE,
		realNow,
		// Runs n session frames; each advances the clock by one 60 Hz frame first.
		async step( n = 1 ) {

			for ( let i = 0; i < n; i ++ ) {

				clock += 1000 / 60;
				if ( state.session ) {

					state.session._frame();
					state.frames ++;

				}

				await settle();
				// Let promise continuations ( loaders, async setSession ) run between frames.
				await new Promise( r => setTimeout( r, 0 ) );

			}

		},
		// Runs n window animation frames; the clock does not advance.
		async windowFrames( n = 1 ) {

			for ( let i = 0; i < n; i ++ ) {

				const callbacks = [ ...windowCallbacks.values() ];
				windowCallbacks = new Map();
				for ( const callback of callbacks ) callback( clock );
				await settle();
				await new Promise( r => setTimeout( r, 0 ) );

			}

		},
		setTrigger( value ) {

			const button = state.session.inputSources[ 0 ].gamepad.buttons[ 0 ];
			button.value = value;
			button.pressed = value >= 1;
			button.touched = value > 0;

		},
		select( type ) {

			state.session._input( type );

		},
		// Both eyes of the projection layer as RGBA8 rows ( BGRA swizzled to RGBA ).
		async readEyes() {

			const { device, layer } = state;
			const bytesPerRow = EYE * 4;
			const buffer = device.createBuffer( { size: bytesPerRow * EYE * 2, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ } );
			const encoder = device.createCommandEncoder();
			encoder.copyTextureToBuffer( { texture: layer.colorTexture }, { buffer, bytesPerRow, rowsPerImage: EYE }, [ EYE, EYE, 2 ] );
			device.queue.submit( [ encoder.finish() ] );
			await buffer.mapAsync( GPUMapMode.READ );
			const data = new Uint8Array( buffer.getMappedRange().slice( 0 ) );
			buffer.unmap();
			buffer.destroy();
			if ( layer.format.startsWith( 'bgra' ) ) for ( let i = 0; i < data.length; i += 4 ) {

				const b = data[ i ];
				data[ i ] = data[ i + 2 ];
				data[ i + 2 ] = b;

			}

			let binary = '';
			for ( let i = 0; i < data.length; i += 0x8000 ) binary += String.fromCharCode.apply( null, data.subarray( i, i + 0x8000 ) );
			return btoa( binary );

		},
		async end() {

			await state.session?.end();

		}
	};

} )();
