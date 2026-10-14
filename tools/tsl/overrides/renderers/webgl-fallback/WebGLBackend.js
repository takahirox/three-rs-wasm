// The port's headless WebGL 2 backend: r186's WebGLBackend ( transpiled
// beside this as WebGLBackend.original.js ) with the reference context's
// extensions and parameters ( tests/tsl/device.json, Chrome on macOS ) and
// the methods that issue GL work left empty. init() keeps the original's
// extension queries; the GL state objects only GL work used are not made.
import WebGLBackendBase from './WebGLBackend.original.js';
import Backend from '../common/Backend.js';
import WebGLExtensions from './utils/WebGLExtensions.js';
import WebGLCapabilities from './utils/WebGLCapabilities.js';
import { warn } from '../../utils.js';

const EXTENSIONS = ["EXT_clip_control", "EXT_color_buffer_float", "EXT_color_buffer_half_float", "EXT_conservative_depth", "EXT_depth_clamp", "EXT_disjoint_timer_query_webgl2", "EXT_float_blend", "EXT_polygon_offset_clamp", "EXT_render_snorm", "EXT_texture_compression_bptc", "EXT_texture_compression_rgtc", "EXT_texture_filter_anisotropic", "EXT_texture_mirror_clamp_to_edge", "EXT_texture_norm16", "KHR_parallel_shader_compile", "NV_shader_noperspective_interpolation", "OES_draw_buffers_indexed", "OES_sample_variables", "OES_shader_multisample_interpolation", "OES_texture_float_linear", "WEBGL_blend_func_extended", "WEBGL_clip_cull_distance", "WEBGL_compressed_texture_astc", "WEBGL_compressed_texture_etc", "WEBGL_compressed_texture_etc1", "WEBGL_compressed_texture_pvrtc", "WEBGL_compressed_texture_s3tc", "WEBGL_compressed_texture_s3tc_srgb", "WEBGL_debug_renderer_info", "WEBGL_debug_shaders", "WEBGL_lose_context", "WEBGL_multi_draw", "WEBGL_polygon_mode", "WEBGL_provoking_vertex", "WEBGL_render_shared_exponent", "WEBGL_stencil_texturing"];
const MAX_TEXTURE_MAX_ANISOTROPY_EXT = 0x84FF;
const MAX_UNIFORM_BLOCK_SIZE = 0x8A31;
const PARAMETERS = { [ MAX_TEXTURE_MAX_ANISOTROPY_EXT ]: 16, [ MAX_UNIFORM_BLOCK_SIZE ]: 16384 };

function headlessContext() {

	return {
		MAX_UNIFORM_BLOCK_SIZE,
		getSupportedExtensions() {

			return EXTENSIONS.slice();

		},
		getExtension( name ) {

			if ( EXTENSIONS.includes( name ) === false ) return null;
			return name === 'EXT_texture_filter_anisotropic' ? { MAX_TEXTURE_MAX_ANISOTROPY_EXT } : {};

		},
		getParameter( name ) {

			return PARAMETERS[ name ] !== undefined ? PARAMETERS[ name ] : 0;

		}
	};

}

class WebGLBackend extends WebGLBackendBase {

	init( renderer ) {

		Backend.prototype.init.call( this, renderer );

		const parameters = this.parameters;
		this.gl = parameters.context !== undefined ? parameters.context : headlessContext();
		this.extensions = new WebGLExtensions( this );
		this.capabilities = new WebGLCapabilities( this );

		this.extensions.get( 'EXT_color_buffer_float' );
		this.extensions.get( 'WEBGL_clip_cull_distance' );
		this.extensions.get( 'OES_texture_float_linear' );
		this.extensions.get( 'EXT_color_buffer_half_float' );
		this.extensions.get( 'WEBGL_multisampled_render_to_texture' );
		this.extensions.get( 'WEBGL_render_shared_exponent' );
		this.extensions.get( 'WEBGL_multi_draw' );
		this.extensions.get( 'OVR_multiview2' );
		this.extensions.get( 'EXT_clip_control' );
		this.disjoint = this.extensions.get( 'EXT_disjoint_timer_query_webgl2' );
		this.parallel = this.extensions.get( 'KHR_parallel_shader_compile' );
		this.drawBuffersIndexedExt = this.extensions.get( 'OES_draw_buffers_indexed' );

		if ( parameters.reversedDepthBuffer ) {

			if ( this.extensions.has( 'EXT_clip_control' ) ) {

				renderer.reversedDepthBuffer = true;

			} else {

				warn( 'WebGPURenderer: Unable to use reversed depth buffer due to missing EXT_clip_control extension. Fallback to default depth buffer.' );
				renderer.reversedDepthBuffer = false;

			}

		}

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
	createBindings() {}
	updateBindings() {}
	updateBinding() {}
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
	updateSize() {}
	needsRenderUpdate() {

		return false;

	}

	isOccluded() {

		return false;

	}

}

export default WebGLBackend;
