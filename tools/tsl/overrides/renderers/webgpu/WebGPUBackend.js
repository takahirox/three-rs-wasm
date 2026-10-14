// The port's WebGPU backend: r186's WebGPUBackend ( transpiled beside this
// as WebGPUBackend.original.js ), unchanged, on a device and a canvas context
// that record nothing ( HeadlessGPU.js ). Node building, render objects,
// bindings, pipelines and caches run as in the original.
import WebGPUBackendBase from './WebGPUBackend.original.js';
import { HeadlessGPUDevice, HeadlessGPUCanvasContext } from './HeadlessGPU.js';

class WebGPUBackend extends WebGPUBackendBase {

	async init( renderer ) {

		if ( this.parameters.device === undefined ) this.parameters.device = new HeadlessGPUDevice();
		if ( this.parameters.context === undefined ) this.parameters.context = new HeadlessGPUCanvasContext( renderer.domElement );

		await super.init( renderer );

	}

}

export default WebGPUBackend;
