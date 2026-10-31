// webgpu_xr_cubes / webgpu_xr_rollercoaster / webgpu_xr_shadows: the page's
// side of three r187's XRManager on a WebGPU backend, without the examples'
// WebGL fallback. The XRButton ( cubes ) or VRButton requests an immersive
// session with the 'webgpu' feature; the session's XRGPUBinding projection
// layer is created on the app's GPUDevice, and every session frame hands the
// views, the layer's texture and the input sources' poses to the Rust app
// ( the example's program ), which renders into the layer. Before and after
// the session the app renders on the canvas with the browser's animation
// frames.

const MULTIVIEW_WARNING = 'THREE.THREE.XRManager: WebGPU XR does not support multiview yet. Disabling multiview for this XR session.';

function stylizeElement( element ) {

	element.style.position = 'absolute';
	element.style.bottom = '20px';
	element.style.padding = '12px 6px';
	element.style.border = '1px solid #fff';
	element.style.borderRadius = '4px';
	element.style.background = 'rgba(0,0,0,0.1)';
	element.style.color = '#fff';
	element.style.font = 'normal 13px sans-serif';
	element.style.textAlign = 'center';
	element.style.opacity = '0.5';
	element.style.outline = 'none';
	element.style.zIndex = '999';

}

// XRButton ( immersive-ar, else immersive-vr ) or VRButton ( immersive-vr ).
function createButton( xr, vr, sessionInit ) {

	const labels = vr
		? { start: 'ENTER VR', stop: 'EXIT VR', unsupported: 'VR NOT SUPPORTED', notAllowed: 'VR NOT ALLOWED', id: 'VRButton' }
		: { start: 'START XR', stop: 'STOP XR', unsupported: 'XR NOT SUPPORTED', notAllowed: 'XR NOT ALLOWED', id: 'XRButton' };
	const button = document.createElement( 'button' );

	function showStart( mode ) {

		let currentSession = null;

		async function onSessionStarted( session ) {

			session.addEventListener( 'end', onSessionEnded );
			await xr.setSession( session );
			button.textContent = labels.stop;
			currentSession = session;

		}

		function onSessionEnded() {

			currentSession.removeEventListener( 'end', onSessionEnded );
			button.textContent = labels.start;
			currentSession = null;

		}

		button.style.display = '';
		button.style.cursor = 'pointer';
		button.style.left = 'calc(50% - 50px)';
		button.style.width = '100px';
		button.textContent = labels.start;
		const sessionOptions = {
			...sessionInit,
			optionalFeatures: [ 'local-floor', 'bounded-floor', 'layers', ...( sessionInit.optionalFeatures || [] ) ]
		};
		button.onmouseenter = () => button.style.opacity = '1.0';
		button.onmouseleave = () => button.style.opacity = '0.5';
		button.onclick = () => {

			if ( currentSession === null ) {

				navigator.xr.requestSession( mode, sessionOptions ).then( onSessionStarted );

			} else {

				currentSession.end();
				if ( navigator.xr.offerSession !== undefined ) {

					navigator.xr.offerSession( mode, sessionOptions ).then( onSessionStarted ).catch( err => console.warn( err ) );

				}

			}

		};
		if ( navigator.xr.offerSession !== undefined ) {

			navigator.xr.offerSession( mode, sessionOptions ).then( onSessionStarted ).catch( err => console.warn( err ) );

		}

	}

	function disableButton() {

		button.style.display = '';
		button.style.cursor = 'auto';
		button.style.left = 'calc(50% - 75px)';
		button.style.width = '150px';
		button.onmouseenter = null;
		button.onmouseleave = null;
		button.onclick = null;

	}

	function showNotSupported() {

		disableButton();
		button.textContent = labels.unsupported;

	}

	function showNotAllowed( exception ) {

		disableButton();
		console.warn( 'Exception when trying to call xr.isSessionSupported', exception );
		button.textContent = labels.notAllowed;

	}

	if ( 'xr' in navigator ) {

		button.id = labels.id;
		button.style.display = 'none';
		stylizeElement( button );
		if ( vr ) {

			navigator.xr.isSessionSupported( 'immersive-vr' ).then( supported => {

				supported ? showStart( 'immersive-vr' ) : showNotSupported();

			} ).catch( showNotAllowed );

		} else {

			navigator.xr.isSessionSupported( 'immersive-ar' ).then( supported => {

				if ( supported ) showStart( 'immersive-ar' );
				else navigator.xr.isSessionSupported( 'immersive-vr' ).then( supported => supported ? showStart( 'immersive-vr' ) : showNotSupported() ).catch( showNotAllowed );

			} ).catch( showNotAllowed );

		}

		return button;

	}

	const message = document.createElement( 'a' );
	if ( window.isSecureContext === false ) {

		message.href = document.location.href.replace( /^http:/, 'https:' );
		message.innerHTML = 'WEBXR NEEDS HTTPS';

	} else {

		message.href = 'https://immersiveweb.dev/';
		message.innerHTML = 'WEBXR NOT AVAILABLE';

	}

	message.style.left = 'calc(50% - 90px)';
	message.style.width = '180px';
	message.style.textDecoration = 'none';
	stylizeElement( message );
	return message;

}

// A pose as [ tracked, 16 matrix floats ].
function poseArray( out, pose ) {

	if ( pose ) {

		out.push( 1, ...pose.transform.matrix );

	} else {

		out.push( 0, ...new Array( 16 ).fill( 0 ) );

	}

}

// program: the example's program; button: its page's XRButton ( 'XR' ) or
// VRButton ( 'VR' ); multiview: its renderer's multiview parameter.
export async function startXr( program, canvas, loading, { button, multiview } ) {

	const app = await program( canvas );
	window.app = app;
	loading.hidden = true;

	const resize = () => {

		canvas.style.width = innerWidth + 'px';
		canvas.style.height = innerHeight + 'px';
		canvas.width = Math.max( 1, Math.floor( innerWidth * devicePixelRatio ) );
		canvas.height = Math.max( 1, Math.floor( innerHeight * devicePixelRatio ) );

	};

	resize();
	addEventListener( 'resize', () => {

		if ( ! xr.isPresenting ) resize();

	} );

	// The renderer's animation loop on the window, stopped while presenting.
	// Like three's Animation.start, restarting it after a session runs a frame
	// at once; at load the loop is set after the renderer's start, so the
	// first frame waits for the browser.
	let windowFrame = null;
	const windowLoop = () => {

		windowFrame = requestAnimationFrame( windowLoop );
		try {

			app.frame();

		} catch ( e ) {

			canvas.dataset.error = String( e );
			throw e;

		}

	};

	windowFrame = requestAnimationFrame( windowLoop );

	const xr = {
		isPresenting: false,
		session: null,
		referenceSpace: null,
		binding: null,
		layer: null,
		controllerInputSources: [],
		depthRange: null,

		async setSession( session ) {

			this.session = session;
			const events = [ 'select', 'selectstart', 'selectend', 'squeeze', 'squeezestart', 'squeezeend' ];
			const onSessionEvent = event => this.onSessionEvent( event );
			for ( const type of events ) session.addEventListener( type, onSessionEvent );
			const onInputSourcesChange = event => this.onInputSourcesChange( event );
			session.addEventListener( 'inputsourceschange', onInputSourcesChange );
			const onSessionEnd = () => {

				for ( const type of events ) session.removeEventListener( type, onSessionEvent );
				session.removeEventListener( 'end', onSessionEnd );
				session.removeEventListener( 'inputsourceschange', onInputSourcesChange );
				this.onSessionEnd();

			};

			session.addEventListener( 'end', onSessionEnd );
			// _validateWebGPUSession: no WebGL fallback.
			if ( session.enabledFeatures.includes( 'webgpu' ) === false ) {

				throw new Error( 'THREE.XRManager: WebGPU XR sessions require the "webgpu" session feature. Use VRButtonGPU/XRButton with "webgpu" enabled or use a WebGL backend.' );

			}

			cancelAnimationFrame( windowFrame );
			windowFrame = null;
			// _initWebGPUSession.
			this.binding = new XRGPUBinding( session, app.gpu_device() );
			this.layer = this.binding.createProjectionLayer( { colorFormat: this.binding.getPreferredColorFormat() } );
			session.updateRenderState( { layers: [ this.layer ] } );
			this.referenceSpace = await session.requestReferenceSpace( app.reference_space_type() );
			app.xr_start( this.layer.textureWidth, this.layer.textureHeight, this.binding.getPreferredColorFormat(), session.environmentBlendMode ?? 'opaque' );
			if ( multiview ) console.warn( MULTIVIEW_WARNING );
			this.depthRange = null;
			session.requestAnimationFrame( ( time, frame ) => this.onAnimationFrame( time, frame ) );
			this.isPresenting = true;

		},

		onAnimationFrame( time, frame ) {

			const session = this.session;
			if ( session === null || frame === undefined ) return;
			session.requestAnimationFrame( ( time, frame ) => this.onAnimationFrame( time, frame ) );
			const pose = frame.getViewerPose( this.referenceSpace );
			if ( pose === null ) return;
			const matrices = [];
			const viewports = [];
			let colorTexture = null;
			for ( const view of pose.views ) {

				const subImage = this.binding.getViewSubImage( this.layer, view );
				if ( colorTexture === null ) colorTexture = subImage.colorTexture;
				matrices.push( ...view.transform.matrix, ...view.projectionMatrix );
				const v = subImage.viewport;
				viewports.push( v.x, v.y, v.width, v.height );

			}

			const poses = [];
			const gamepads = [];
			const visible = session.visibilityState !== 'visible-blurred';
			for ( let i = 0; i < app.controller_count(); i ++ ) {

				const inputSource = this.controllerInputSources[ i ] ?? null;
				const tracked = inputSource !== null && visible;
				poseArray( poses, tracked ? frame.getPose( inputSource.targetRaySpace, this.referenceSpace ) : null );
				poseArray( poses, tracked && inputSource.gripSpace ? frame.getPose( inputSource.gripSpace, this.referenceSpace ) : null );
				const gamepad = inputSource?.gamepad;
				gamepads.push( gamepad ? gamepad.buttons.length : 0 );
				for ( const b of gamepad?.buttons ?? [] ) gamepads.push( b.pressed ? 1 : 0, b.touched ? 1 : 0, b.value );
				gamepads.push( gamepad ? gamepad.axes.length : 0, ...( gamepad?.axes ?? [] ) );

			}

			app.xr_frame( new Float32Array( matrices ), new Uint32Array( viewports ), colorTexture, new Float32Array( poses ), new Float32Array( gamepads ) );
			// XRManager.updateCamera: the render state follows the camera's near and far.
			const [ near, far ] = app.xr_depth_range();
			if ( near !== undefined && ( this.depthRange === null || this.depthRange[ 0 ] !== near || this.depthRange[ 1 ] !== far ) ) {

				session.updateRenderState( { depthNear: near, depthFar: far } );
				this.depthRange = [ near, far ];

			}

		},

		onSessionEvent( event ) {

			const index = this.controllerInputSources.indexOf( event.inputSource );
			if ( index === - 1 || index >= app.controller_count() ) return;
			const pose = [];
			poseArray( pose, event.frame.getPose( event.inputSource.targetRaySpace, this.referenceSpace ) );
			poseArray( pose, event.inputSource.gripSpace ? event.frame.getPose( event.inputSource.gripSpace, this.referenceSpace ) : null );
			app.xr_event( index, event.type, new Float32Array( pose ) );

		},

		onInputSourcesChange( event ) {

			const sources = this.controllerInputSources;
			const count = app.controller_count();
			for ( const inputSource of event.removed ) {

				const index = sources.indexOf( inputSource );
				if ( index >= 0 ) {

					sources[ index ] = null;
					app.xr_disconnect( index );

				}

			}

			for ( const inputSource of event.added ) {

				let index = sources.indexOf( inputSource );
				if ( index === - 1 ) {

					for ( let i = 0; i < count; i ++ ) {

						if ( i >= sources.length ) {

							sources.push( inputSource );
							index = i;
							break;

						} else if ( sources[ i ] === null ) {

							sources[ i ] = inputSource;
							index = i;
							break;

						}

					}

					if ( index === - 1 ) break;

				}

				app.xr_connect( index, inputSource.handedness, inputSource.targetRayMode, [ ...inputSource.profiles ], !! inputSource.gamepad, !! inputSource.hand ).catch( e => console.warn( e ) );

			}

		},

		onSessionEnd() {

			this.controllerInputSources = [];
			app.xr_end();
			this.session = null;
			this.binding = null;
			this.layer = null;
			this.isPresenting = false;
			resize();
			windowLoop();

		}
	};

	const vr = button === 'VR';
	document.body.appendChild( createButton( xr, vr, { optionalFeatures: [ 'webgpu' ] } ) );
	return app;

}
