// The port's live GPU memory, for the Inspector's Memory tab: what the page
// allocates through WebGPU, counted in the categories of three's
// renderer.info.memory. Buffers count by usage ( VERTEX attributes, INDEX
// index attributes, INDIRECT, STORAGE, UNIFORM, MAP_READ readback ), each
// vertex buffer as a geometry; textures by their estimated size, and those
// that can be rendered to as render targets too; shader modules as programs,
// sized by their code. Loaded before the Wasm creates its device.
( () => {

	if ( typeof GPUDevice === 'undefined' ) return;
	const memory = {
		attributes: 0, attributesSize: 0, geometries: 0, indexAttributes: 0, indexAttributesSize: 0,
		indirectStorageAttributes: 0, indirectStorageAttributesSize: 0, programs: 0, programsSize: 0,
		readbackBuffers: 0, readbackBuffersSize: 0, renderTargets: 0, storageAttributes: 0, storageAttributesSize: 0,
		textures: 0, texturesSize: 0, uniformBuffers: 0, uniformBuffersSize: 0, total: 0
	};
	window.galleryGpuMemory = memory;
	// The page's device, for the Inspector's GPU timing ( gpu-timing.js ).
	const requestDevice = GPUAdapter.prototype.requestDevice;
	GPUAdapter.prototype.requestDevice = async function ( ...args ) {

		const device = await requestDevice.apply( this, args );
		window.galleryGpuDevice = device;
		return device;

	};
	const live = new WeakMap();
	const add = ( object, entries ) => {

		live.set( object, entries );
		for ( const [ key, size ] of entries ) {

			memory[ key ] ++;
			if ( size !== null ) {

				memory[ key + 'Size' ] += size;
				memory.total += size;

			}

		}

	};

	const remove = object => {

		const entries = live.get( object );
		if ( ! entries ) return;
		live.delete( object );
		for ( const [ key, size ] of entries ) {

			memory[ key ] --;
			if ( size !== null ) {

				memory[ key + 'Size' ] -= size;
				memory.total -= size;

			}

		}

	};

	const U = { MAP_READ: 1, INDEX: 16, VERTEX: 32, UNIFORM: 64, STORAGE: 128, INDIRECT: 256 };
	const createBuffer = GPUDevice.prototype.createBuffer;
	GPUDevice.prototype.createBuffer = function ( descriptor ) {

		const buffer = createBuffer.call( this, descriptor );
		const { size, usage } = descriptor;
		const key = usage & U.INDIRECT ? 'indirectStorageAttributes' : usage & U.INDEX ? 'indexAttributes' : usage & U.VERTEX ? 'attributes'
			: usage & U.STORAGE ? 'storageAttributes' : usage & U.UNIFORM ? 'uniformBuffers' : usage & U.MAP_READ ? 'readbackBuffers' : null;
		if ( key ) add( buffer, key === 'attributes' ? [ [ key, size ], [ 'geometries', null ] ] : [ [ key, size ] ] );
		return buffer;

	};

	const destroyBuffer = GPUBuffer.prototype.destroy;
	GPUBuffer.prototype.destroy = function () {

		remove( this );
		return destroyBuffer.call( this );

	};

	// Bytes per texel of the formats the port uses ( compressed: per block, as 4×4 ).
	const texel = format => /^(r8|r8unorm|r8snorm|r8uint|r8sint|stencil8)$/.test( format ) ? 1
		: /^(rg8|r16|depth16)/.test( format ) ? 2
			: /^(rgba8|bgra8|rg16|r32|rgb10a2|rg11b10|rgb9e5|depth24plus|depth32float$|depth24plus-stencil8)/.test( format ) ? 4
				: /^(rgba16|rg32|depth32float-stencil8)/.test( format ) ? 8
					: /^rgba32/.test( format ) ? 16
						: /^(bc1|bc4|etc2-rgb8|etc2-rgb8a1|eac-r11)/.test( format ) ? 0.5
							: /^(bc|etc2|eac|astc)/.test( format ) ? 1 : 4;
	const createTexture = GPUDevice.prototype.createTexture;
	GPUDevice.prototype.createTexture = function ( descriptor ) {

		const texture = createTexture.call( this, descriptor );
		let bytes = 0;
		for ( let level = 0; level < texture.mipLevelCount; level ++ ) {

			const w = Math.max( 1, texture.width >> level ), h = Math.max( 1, texture.height >> level );
			const d = texture.dimension === '3d' ? Math.max( 1, texture.depthOrArrayLayers >> level ) : texture.depthOrArrayLayers;
			bytes += w * h * d * texel( texture.format );

		}

		bytes *= texture.sampleCount;
		const entries = [ [ 'textures', Math.round( bytes ) ] ];
		if ( texture.usage & 16 /* RENDER_ATTACHMENT */ ) entries.push( [ 'renderTargets', null ] );
		add( texture, entries );
		return texture;

	};

	const destroyTexture = GPUTexture.prototype.destroy;
	GPUTexture.prototype.destroy = function () {

		remove( this );
		return destroyTexture.call( this );

	};

	const createShaderModule = GPUDevice.prototype.createShaderModule;
	GPUDevice.prototype.createShaderModule = function ( descriptor ) {

		const module = createShaderModule.call( this, descriptor );
		add( module, [ [ 'programs', descriptor.code.length ] ] );
		return module;

	};

} )();
