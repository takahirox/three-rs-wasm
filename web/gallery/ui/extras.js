// Live readouts some originals add to their lil-gui folders, kept up to date
// as the original keeps them, from the panel's state or what the port
// publishes. Keyed by example id; each receives the page's GUIs.
function formatBytes( bytes, decimals ) {

	if ( bytes === 0 ) return '0 bytes';
	const k = 1024;
	const dm = decimals < 0 ? 0 : decimals;
	const sizes = [ 'bytes', 'KB', 'MB' ];
	const i = Math.floor( Math.log( bytes ) / Math.log( k ) );
	return parseFloat( ( bytes / Math.pow( k, i ) ).toFixed( dm ) ) + ' ' + sizes[ i ];

}

const controller = ( guis, property ) => guis.flatMap( g => g.controllersRecursive() ).find( c => c.property === property );

// webgl_instancing_performance and webgl_tsl_instancing: the method's draw
// calls and memory. Suzanne ( models/json/suzanne_buffergeometry.json, with
// computed normals ): 1515 vertices × ( 12 + 12 ) bytes and 2901 Uint16
// indices, 17922 bytes; a merged copy's index is Uint32 past 65535 vertices.
function instancing( guis ) {

	const element = document.querySelector( '.lil-gui .gui-stats' );
	const method = controller( guis, 'method' ), count = controller( guis, 'count' );
	if ( ! element || ! method || ! count ) return;
	const update = () => {

		const n = count.getValue();
		const geometry = 17922;
		let calls = 1, bytes;
		if ( method.getValue() === 'MERGED' ) {

			const vertices = n * 1515;
			bytes = n * 1515 * 24 + n * 2901 * ( vertices - 1 > 65535 ? 4 : 2 );

		} else {

			if ( method.getValue() === 'NAIVE' ) calls = n;
			bytes = n * 16 + geometry;

		}

		element.innerHTML = [ '<i>GPU draw calls</i>: ' + calls, '<i>GPU memory</i>: ' + formatBytes( bytes, 2 ) ].join( '<br/>' );

	};

	for ( const g of guis ) g.onChange( update );
	update();

}

// webgl_renderer_pathtracer: the path tracer's sample count, each frame.
function pathtracer() {

	const element = document.querySelector( '.lil-gui .gui-render' );
	if ( ! element ) return;
	const end = window.galleryFrameEnd;
	window.galleryFrameEnd = () => {

		end?.();
		element.innerText = 'samples: ' + Math.floor( Number( document.body.dataset.ptSamples ?? 0 ) );

	};

}

// webgl_animation_skinning_additive_blending: prepareCrossFade marks the
// chosen base action's button active and the others inactive.
function baseActions( guis ) {

	const folder = guis.flatMap( g => g.foldersRecursive() ).find( f => f._title === 'Base Actions' );
	if ( ! folder ) return;
	for ( const c of folder.controllers ) {

		c.domElement.querySelector( 'button' )?.addEventListener( 'click', () => {

			for ( const other of folder.controllers ) other.domElement.classList.toggle( 'control-inactive', other !== c );

		} );

	}

}

export const extras = {
	webgl_animation_skinning_additive_blending: baseActions,
	webgl_instancing_performance: instancing,
	webgl_tsl_instancing: instancing,
	webgl_renderer_pathtracer: pathtracer
};
