// The port's WebGPU backend: the pinned release's WebGPUBackend ( transpiled beside this
// as WebGPUBackend.original.js ), unchanged. It runs on a device and a canvas
// context that record nothing ( HeadlessGPU.js ). When the page provides a
// real device ( __tslRealDevice, the editor's preview ), it renders through
// that device into the canvas instead. Node building, render objects,
// bindings, pipelines and caches run as in the original.
import WebGPUBackendBase from './WebGPUBackend.original.js';
import { HeadlessGPUDevice, HeadlessGPUCanvasContext } from './HeadlessGPU.js';

class WebGPUBackend extends WebGPUBackendBase {

	async init( renderer ) {

		if ( typeof __tslRealDevice !== 'undefined' ) {

			if ( this.parameters.device === undefined ) this.parameters.device = __tslRealDevice;

		} else {

			if ( this.parameters.device === undefined ) this.parameters.device = new HeadlessGPUDevice();
			if ( this.parameters.context === undefined ) this.parameters.context = new HeadlessGPUCanvasContext( renderer.domElement );

		}

		await super.init( renderer );

	}

}

export default WebGPUBackend;
