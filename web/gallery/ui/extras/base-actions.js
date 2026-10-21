// A live readout an original adds to its lil-gui folders, kept up to date as
// the original keeps it, from the panel's state or what the port publishes
// ( web/gallery/ui/ui.js ). `examples` are the pages that load it
// ( tools/gallery/pages.mjs ); it receives the page's GUIs.
export const examples = [ 'webgl_animation_skinning_additive_blending' ];

// webgl_animation_skinning_additive_blending: prepareCrossFade marks the
// chosen base action's button active and the others inactive.
export default function baseActions( guis ) {

	const folder = guis.flatMap( g => g.foldersRecursive() ).find( f => f._title === 'Base Actions' );
	if ( ! folder ) return;
	for ( const c of folder.controllers ) {

		c.domElement.querySelector( 'button' )?.addEventListener( 'click', () => {

			for ( const other of folder.controllers ) other.domElement.classList.toggle( 'control-inactive', other !== c );

		} );

	}

}
