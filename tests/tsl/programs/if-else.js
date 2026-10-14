const { Fn, vec4, vec3, uv, If, select } = await import( 'three/tsl' );

output = Fn( () => {

	const color = vec3( 0 ).toVar( 'color' );
	const p = uv();

	If( p.x.lessThan( 0.33 ), () => {

		color.assign( vec3( 1, 0, 0 ) );

	} ).ElseIf( p.x.lessThan( 0.66 ), () => {

		color.assign( vec3( 0, 1, 0 ) );

	} ).Else( () => {

		color.assign( vec3( 0, 0, 1 ) );

	} );

	return vec4( color.mul( select( p.y.greaterThan( 0.5 ), 1.0, 0.5 ) ), 1 );

} )();
