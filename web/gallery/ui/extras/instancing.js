// A live readout an original adds to its lil-gui folders, kept up to date as
// the original keeps it, from the panel's state or what the port publishes
// ( web/gallery/ui/ui.js ). `examples` are the pages that load it
// ( tools/gallery/pages.mjs ); it receives the page's GUIs.
export const examples = [ 'webgl_instancing_performance', 'webgl_tsl_instancing' ];

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
export default function instancing( guis ) {

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
