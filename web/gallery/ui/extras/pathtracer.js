// A live readout an original adds to its lil-gui folders, kept up to date as
// the original keeps it, from the panel's state or what the port publishes
// ( web/gallery/ui/ui.js ). `examples` are the pages that load it
// ( tools/gallery/pages.mjs ); it receives the page's GUIs.
export const examples = [ 'webgl_renderer_pathtracer' ];

// webgl_renderer_pathtracer: the path tracer's sample count, each frame.
export default function pathtracer() {

	const element = document.querySelector( '.lil-gui .gui-render' );
	if ( ! element ) return;
	const end = window.galleryFrameEnd;
	window.galleryFrameEnd = () => {

		end?.();
		element.innerText = 'samples: ' + Math.floor( Number( document.body.dataset.ptSamples ?? 0 ) );

	};

}
