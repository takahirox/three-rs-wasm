// A WebGPU device that records nothing: r186's WebGPUBackend runs on it
// unchanged. It reports the reference adapter's features and limits
// ( tests/tsl/device.json ) and returns inert objects. Its methods throw the
// exceptions the browser's WebIDL bindings throw synchronously for the
// argument values three can pass, with Chrome's messages. The GPU's own
// validation, which never throws, is not modeled.

const FEATURES = [ 'bgra8unorm-storage', 'clip-distances', 'core-features-and-limits', 'depth-clip-control', 'depth32float-stencil8', 'dual-source-blending', 'float32-blendable', 'float32-filterable', 'indirect-first-instance', 'rg11b10ufloat-renderable', 'shader-f16', 'subgroups', 'texture-compression-astc', 'texture-compression-astc-sliced-3d', 'texture-compression-bc', 'texture-compression-bc-sliced-3d', 'texture-compression-etc2', 'texture-formats-tier1', 'texture-formats-tier2', 'timestamp-query' ];

const LIMITS = { maxTextureDimension1D: 8192, maxTextureDimension2D: 8192, maxTextureDimension3D: 2048, maxTextureArrayLayers: 256, maxBindGroups: 4, maxBindGroupsPlusVertexBuffers: 24, maxBindingsPerBindGroup: 1000, maxDynamicUniformBuffersPerPipelineLayout: 8, maxDynamicStorageBuffersPerPipelineLayout: 4, maxSampledTexturesPerShaderStage: 16, maxSamplersPerShaderStage: 16, maxStorageBuffersPerShaderStage: 8, maxStorageTexturesPerShaderStage: 4, maxUniformBuffersPerShaderStage: 12, maxUniformBufferBindingSize: 65536, maxStorageBufferBindingSize: 134217728, minUniformBufferOffsetAlignment: 256, minStorageBufferOffsetAlignment: 256, maxVertexBuffers: 8, maxBufferSize: 268435456, maxVertexAttributes: 16, maxVertexBufferArrayStride: 2048, maxInterStageShaderVariables: 16, maxColorAttachments: 8, maxColorAttachmentBytesPerSample: 32, maxComputeWorkgroupStorageSize: 16384, maxComputeInvocationsPerWorkgroup: 256, maxComputeWorkgroupSizeX: 256, maxComputeWorkgroupSizeY: 256, maxComputeWorkgroupSizeZ: 64, maxComputeWorkgroupsPerDimension: 65535 };

// WebIDL [EnforceRange] integer conversions: a non-finite value is "not of type".
function enforceRange( value, type, context ) {

	const x = Number( value );
	if ( ! Number.isFinite( x ) ) throw new TypeError( `${ context }Value is not of type '${ type }'.` );
	return Math.trunc( x );

}

function member( dictionary, name, type, method, iface, dictName, required = false ) {

	const value = dictionary[ name ];
	if ( value === undefined ) {

		if ( required ) throw new TypeError( `Failed to execute '${ method }' on '${ iface }': Failed to read the '${ name }' property from '${ dictName }': Required member is undefined.` );
		return undefined;

	}

	return enforceRange( value, type, `Failed to execute '${ method }' on '${ iface }': Failed to read the '${ name }' property from '${ dictName }': ` );

}

class HeadlessGPUObject {

	constructor( descriptor = {} ) {

		this.label = descriptor.label !== undefined ? descriptor.label : '';

	}

}

class HeadlessGPUBuffer extends HeadlessGPUObject {

	constructor( descriptor ) {

		super( descriptor );
		this.size = descriptor.size;
		this.usage = descriptor.usage;
		this.mapState = descriptor.mappedAtCreation ? 'mapped' : 'unmapped';

	}

	getMappedRange( offset = 0, size = this.size - offset ) {

		return new ArrayBuffer( size );

	}

	mapAsync() {

		this.mapState = 'mapped';
		return Promise.resolve();

	}

	unmap() {

		this.mapState = 'unmapped';

	}

	destroy() {}

}

class HeadlessGPUTextureView extends HeadlessGPUObject {}

class HeadlessGPUTexture extends HeadlessGPUObject {

	constructor( descriptor ) {

		super( descriptor );
		const size = descriptor.size;
		const isArray = Array.isArray( size );
		this.width = isArray ? size[ 0 ] : size.width;
		this.height = isArray ? ( size[ 1 ] !== undefined ? size[ 1 ] : 1 ) : ( size.height !== undefined ? size.height : 1 );
		this.depthOrArrayLayers = isArray ? ( size[ 2 ] !== undefined ? size[ 2 ] : 1 ) : ( size.depthOrArrayLayers !== undefined ? size.depthOrArrayLayers : 1 );
		this.mipLevelCount = descriptor.mipLevelCount !== undefined ? descriptor.mipLevelCount : 1;
		this.sampleCount = descriptor.sampleCount !== undefined ? descriptor.sampleCount : 1;
		this.dimension = descriptor.dimension !== undefined ? descriptor.dimension : '2d';
		this.format = descriptor.format;
		this.usage = descriptor.usage;

	}

	createView( descriptor = {} ) {

		return new HeadlessGPUTextureView( descriptor );

	}

	destroy() {}

}

class HeadlessGPUPipeline extends HeadlessGPUObject {

	getBindGroupLayout() {

		return new HeadlessGPUObject();

	}

}

class HeadlessGPUPassEncoder extends HeadlessGPUObject {

	setPipeline() {}
	setBindGroup() {}
	setVertexBuffer() {}
	setIndexBuffer() {}
	setViewport() {}
	setScissorRect() {}
	setBlendConstant() {}
	setStencilReference() {}
	beginOcclusionQuery() {}
	endOcclusionQuery() {}
	executeBundles() {}
	draw( vertexCount ) {

		enforceRange( vertexCount, 'unsigned long', 'Failed to execute \'draw\' on \'GPURenderPassEncoder\': ' );

	}

	drawIndexed( indexCount ) {

		enforceRange( indexCount, 'unsigned long', 'Failed to execute \'drawIndexed\' on \'GPURenderPassEncoder\': ' );

	}

	drawIndirect() {}
	drawIndexedIndirect() {}
	end() {}

}

class HeadlessGPUComputePassEncoder extends HeadlessGPUObject {

	setPipeline() {}
	setBindGroup() {}
	dispatchWorkgroups( x, y = 1, z = 1 ) {

		const context = 'Failed to execute \'dispatchWorkgroups\' on \'GPUComputePassEncoder\': ';
		enforceRange( x, 'unsigned long', context );
		enforceRange( y, 'unsigned long', context );
		enforceRange( z, 'unsigned long', context );

	}

	dispatchWorkgroupsIndirect() {}
	end() {}

}

class HeadlessGPURenderBundleEncoder extends HeadlessGPUPassEncoder {

	finish( descriptor = {} ) {

		return new HeadlessGPUObject( descriptor );

	}

}

class HeadlessGPUCommandEncoder extends HeadlessGPUObject {

	beginRenderPass( descriptor = {} ) {

		return new HeadlessGPUPassEncoder( descriptor );

	}

	beginComputePass( descriptor = {} ) {

		return new HeadlessGPUComputePassEncoder( descriptor );

	}

	copyBufferToBuffer() {}
	copyBufferToTexture() {}
	copyTextureToBuffer() {}
	copyTextureToTexture() {}
	clearBuffer() {}
	resolveQuerySet() {}
	writeTimestamp() {}
	finish( descriptor = {} ) {

		return new HeadlessGPUObject( descriptor );

	}

}

class HeadlessGPUQueue extends HeadlessGPUObject {

	submit() {}
	writeBuffer() {}
	writeTexture() {}
	copyExternalImageToTexture() {}
	onSubmittedWorkDone() {

		return Promise.resolve();

	}

}

class HeadlessGPUDevice extends HeadlessGPUObject {

	constructor() {

		super();
		this.features = new Set( FEATURES );
		this.limits = Object.assign( {}, LIMITS );
		this.queue = new HeadlessGPUQueue();
		this.lost = new Promise( () => {} );
		this.onuncapturederror = null;

	}

	createBuffer( descriptor ) {

		member( descriptor, 'size', 'unsigned long long', 'createBuffer', 'GPUDevice', 'GPUBufferDescriptor', true );
		member( descriptor, 'usage', 'unsigned long', 'createBuffer', 'GPUDevice', 'GPUBufferDescriptor', true );
		return new HeadlessGPUBuffer( descriptor );

	}

	createTexture( descriptor ) {

		return new HeadlessGPUTexture( descriptor );

	}

	createSampler( descriptor = {} ) {

		return new HeadlessGPUObject( descriptor );

	}

	importExternalTexture( descriptor = {} ) {

		return new HeadlessGPUObject( descriptor );

	}

	createBindGroupLayout( descriptor = {} ) {

		return new HeadlessGPUObject( descriptor );

	}

	createPipelineLayout( descriptor = {} ) {

		return new HeadlessGPUObject( descriptor );

	}

	createBindGroup( descriptor = {} ) {

		return new HeadlessGPUObject( descriptor );

	}

	createShaderModule( descriptor = {} ) {

		const module = new HeadlessGPUObject( descriptor );
		module.getCompilationInfo = () => Promise.resolve( { messages: [] } );
		return module;

	}

	createRenderPipeline( descriptor = {} ) {

		return new HeadlessGPUPipeline( descriptor );

	}

	createComputePipeline( descriptor = {} ) {

		return new HeadlessGPUPipeline( descriptor );

	}

	createRenderPipelineAsync( descriptor = {} ) {

		return Promise.resolve( new HeadlessGPUPipeline( descriptor ) );

	}

	createComputePipelineAsync( descriptor = {} ) {

		return Promise.resolve( new HeadlessGPUPipeline( descriptor ) );

	}

	createCommandEncoder( descriptor = {} ) {

		return new HeadlessGPUCommandEncoder( descriptor );

	}

	createRenderBundleEncoder( descriptor = {} ) {

		return new HeadlessGPURenderBundleEncoder( descriptor );

	}

	createQuerySet( descriptor = {} ) {

		return new HeadlessGPUObject( descriptor );

	}

	pushErrorScope() {}

	popErrorScope() {

		return Promise.resolve( null );

	}

	destroy() {}

}

// The canvas context the backend configures and draws into.
class HeadlessGPUCanvasContext {

	constructor( canvas ) {

		this.canvas = canvas;
		this.configuration = null;

	}

	configure( configuration ) {

		this.configuration = configuration;

	}

	unconfigure() {}

	getCurrentTexture() {

		return new HeadlessGPUTexture( { size: { width: this.canvas.width, height: this.canvas.height }, format: this.configuration ? this.configuration.format : 'bgra8unorm' } );

	}

}

// The backend tests passes with `instanceof GPURenderBundleEncoder`, and reads
// the browser's flag namespaces ( the reference browser's values ).
globalThis.GPURenderBundleEncoder = HeadlessGPURenderBundleEncoder;
globalThis.GPUBufferUsage = { MAP_READ: 1, MAP_WRITE: 2, COPY_SRC: 4, COPY_DST: 8, INDEX: 16, VERTEX: 32, UNIFORM: 64, STORAGE: 128, INDIRECT: 256, QUERY_RESOLVE: 512 };
globalThis.GPUTextureUsage = { COPY_SRC: 1, COPY_DST: 2, TEXTURE_BINDING: 4, STORAGE_BINDING: 8, RENDER_ATTACHMENT: 16, TRANSIENT_ATTACHMENT: 32 };
globalThis.GPUShaderStage = { VERTEX: 1, FRAGMENT: 2, COMPUTE: 4 };
globalThis.GPUMapMode = { READ: 1, WRITE: 2 };
globalThis.GPUColorWrite = { RED: 1, GREEN: 2, BLUE: 4, ALPHA: 8, ALL: 15 };

export { HeadlessGPUDevice, HeadlessGPUCanvasContext };
