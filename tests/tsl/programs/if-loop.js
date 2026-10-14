const { Fn, vec4, float, uv, If, Loop, Break } = await import( 'three/tsl' );

const mandel = Fn( () => {

	const c = uv().sub( 0.5 ).mul( 3.0 ).toVar();
	const z = c.toVar();
	const n = float( 0 ).toVar();

	Loop( 64, ( { i } ) => {

		If( z.dot( z ).greaterThan( 4.0 ), () => {

			Break();

		} );

		z.assign( z.x.mul( z.x ).sub( z.y.mul( z.y ) ).add( c.x ).toVec2().setY( z.x.mul( z.y ).mul( 2.0 ).add( c.y ) ) );
		n.addAssign( 1.0 );

	} );

	return vec4( n.div( 64.0 ).toVec3(), 1.0 );

} );

output = mandel();
