// The port's headless WebGPU backend: r186's WebGPUBackend ( transpiled
// beside this as WebGPUBackend.original.js ) on a device with the reference
// adapter's features and limits ( tests/tsl/device.json, Chrome on macOS ),
// with the methods that record GPU work left empty. Node building, render
// objects, bindings and caches run as in the original; nothing is drawn.
import WebGPUBackendBase from './WebGPUBackend.original.js';

const FEATURES = ["bgra8unorm-storage", "clip-distances", "core-features-and-limits", "depth-clip-control", "depth32float-stencil8", "dual-source-blending", "float32-blendable", "float32-filterable", "indirect-first-instance", "rg11b10ufloat-renderable", "shader-f16", "subgroups", "texture-compression-astc", "texture-compression-astc-sliced-3d", "texture-compression-bc", "texture-compression-bc-sliced-3d", "texture-compression-etc2", "texture-formats-tier1", "texture-formats-tier2", "timestamp-query"];
const LIMITS = {"maxTextureDimension1D": 8192, "maxTextureDimension2D": 8192, "maxTextureDimension3D": 2048, "maxTextureArrayLayers": 256, "maxBindGroups": 4, "maxBindGroupsPlusVertexBuffers": 24, "maxBindingsPerBindGroup": 1000, "maxDynamicUniformBuffersPerPipelineLayout": 8, "maxDynamicStorageBuffersPerPipelineLayout": 4, "maxSampledTexturesPerShaderStage": 16, "maxSamplersPerShaderStage": 16, "maxStorageBuffersPerShaderStage": 8, "maxStorageTexturesPerShaderStage": 4, "maxUniformBuffersPerShaderStage": 12, "maxUniformBufferBindingSize": 65536, "maxStorageBufferBindingSize": 134217728, "minUniformBufferOffsetAlignment": 256, "minStorageBufferOffsetAlignment": 256, "maxVertexBuffers": 8, "maxBufferSize": 268435456, "maxVertexAttributes": 16, "maxVertexBufferArrayStride": 2048, "maxInterStageShaderVariables": 16, "maxColorAttachments": 8, "maxColorAttachmentBytesPerSample": 32, "maxComputeWorkgroupStorageSize": 16384, "maxComputeInvocationsPerWorkgroup": 256, "maxComputeWorkgroupSizeX": 256, "maxComputeWorkgroupSizeY": 256, "maxComputeWorkgroupSizeZ": 64, "maxComputeWorkgroupsPerDimension": 65535, "maxImmediateSize": 64, "maxStorageBuffersInFragmentStage": 8, "maxStorageTexturesInFragmentStage": 4, "maxStorageBuffersInVertexStage": 8, "maxStorageTexturesInVertexStage": 4};

function headlessDevice() {

	return {
		features: new Set( FEATURES ),
		limits: Object.assign( {}, LIMITS ),
		lost: new Promise( () => {} ),
		onuncapturederror: null,
		queue: {
			submit() {},
			writeBuffer() {},
			writeTexture() {},
			copyExternalImageToTexture() {},
			onSubmittedWorkDone() {

				return Promise.resolve();

			}
		},
		destroy() {}
	};

}

class WebGPUBackend extends WebGPUBackendBase {

	async init( renderer ) {

		if ( this.parameters.device === undefined ) this.parameters.device = headlessDevice();

		await super.init( renderer );

	}

	beginRender() {}
	finishRender() {}
	draw() {}
	beginCompute() {}
	compute() {}
	finishCompute() {}
	createProgram() {}
	destroyProgram() {}
	createRenderPipeline() {}
	createComputePipeline() {}
	beginBundle() {}
	finishBundle() {}
	addBundle() {}
	createBindings() {}
	updateBindings() {}
	updateBinding() {}
	deleteBindGroupData() {}
	createIndexAttribute() {}
	createAttribute() {}
	createStorageAttribute() {}
	createIndirectStorageAttribute() {}
	updateAttribute() {}
	destroyAttribute() {}
	createSampler() {}
	updateSampler() {}
	destroySampler() {}
	createDefaultTexture() {}
	createTexture() {}
	updateTexture() {}
	generateMipmaps() {}
	destroyTexture() {}
	async copyTextureToBuffer() {}
	copyTextureToTexture() {}
	copyFramebufferToTexture() {}
	clear() {}
	initTimestampQuery() {}
	async resolveTimestampsAsync() {}
	async getArrayBufferAsync() {}
	createUniformBuffer() {}
	destroyUniformBuffer() {}
	needsRenderUpdate() {

		return false;

	}

	isOccluded() {

		return false;

	}

}

export default WebGPUBackend;
