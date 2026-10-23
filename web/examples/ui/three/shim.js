// What the WebGPU Inspector's modules import from 'three', 'three/webgpu'
// and 'three/tsl', for the gallery's ports: no three.js renderer runs here.
// constants.js and EventDispatcher.js are three r186's own ( unchanged, MIT );
// InspectorBase follows r186's renderers/common/InspectorBase.js. The
// rendering classes and TSL functions the Inspector's viewer and overdraw
// features use are inert stand-ins: those features are not ported.
export * from './constants.js';
import { EventDispatcher } from './EventDispatcher.js';
export { EventDispatcher };

let consoleFunction = null;
export function setConsoleFunction( fn ) {

	consoleFunction = fn;

}

export function getConsoleFunction() {

	return consoleFunction;

}

const warned = new Set();
export function warnOnce( message ) {

	if ( warned.has( message ) ) return;
	warned.add( message );
	console.warn( 'THREE.' + message );

}

export class InspectorBase extends EventDispatcher {

	constructor() {

		super();
		this._renderer = null;
		this.currentFrame = null;
		this.isRunning = false;
		this.enabled = true;

	}

	get nodeFrame() {

		return this._renderer._nodes.nodeFrame;

	}

	setRenderer( renderer ) {

		this._renderer = renderer;
		return this;

	}

	getRenderer() {

		return this._renderer;

	}

	begin() {

		this.isRunning = true;

	}

	finish() {

		this.isRunning = false;

	}

	inspect() {}
	computeAsync() {}
	beginCompute() {}
	finishCompute() {}
	beginRender() {}
	finishRender() {}
	copyTextureToTexture() {}
	copyFramebufferToTexture() {}

	dispose() {

		if ( this.isRunning === true ) this.finish();
		this.dispatchEvent( { type: 'dispose' } );

	}

}

export const TimestampQuery = { COMPUTE: 'compute', RENDER: 'render' };

// Anything: callable, constructible, every property another stand-in.
function inert() {

	return new Proxy( function () {}, {
		get: ( target, key ) => key === Symbol.toPrimitive ? () => 0 : key === 'then' ? undefined : inert(),
		apply: () => inert(),
		construct: () => inert()
	} );

}

export const RendererUtils = inert();
export class MeshBasicNodeMaterial {}
export class NodeMaterial {}
export class QuadMesh {}
export class CanvasTarget {}
export class WebGPURenderer {}
export class WebGLBackend {}
export class Node {}
export class Vector2 {

	constructor( x = 0, y = 0 ) {

		this.x = x;
		this.y = y;

	}

	set( x, y ) {

		this.x = x;
		this.y = y;
		return this;

	}

}

export class Color {

	constructor() {

		this.r = 1;
		this.g = 1;
		this.b = 1;

	}

	set() {

		return this;

	}

}

// three/tsl.
export const vec2 = inert(), vec3 = inert(), vec4 = inert(), float = inert(), uniform = inert(), Fn = inert(), renderOutput = inert(), screenUV = inert(), step = inert(), OnMaterialUpdate = inert();
