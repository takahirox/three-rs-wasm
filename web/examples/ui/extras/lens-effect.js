// The folders an original shows and hides as one of its controllers changes
// ( web/examples/ui/ui.js ). `examples` are the pages that load it
// ( tools/gallery/pages.mjs ); it receives the page's GUIs and Inspectors.
export const examples = [ 'webgpu_postprocessing_lensflare' ];

// webgpu_postprocessing_lensflare: updateOutput shows the lensflare folder
// for the flares and the lens dirt folder for the dirt.
export default function lensEffect( guis, inspectors ) {

	for ( const inspector of inspectors ) {

		const select = inspector.galleryEditors?.get( 'lens effect' );
		const flares = inspector.galleryFolders?.get( 'lensflare' );
		const dirt = inspector.galleryFolders?.get( 'lens dirt' );
		if ( ! select || ! flares || ! dirt ) continue;
		select.addEventListener( 'change', ( { value } ) => {

			if ( value === 'flares' ) {

				flares.show();
				dirt.hide();

			} else {

				flares.hide();
				dirt.show();

			}

		} );

	}

}
