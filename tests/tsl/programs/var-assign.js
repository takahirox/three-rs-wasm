const { Fn, vec4, vec3, float, uv, Loop } = await import( 'three/tsl' );

output = Fn( () => {

	const sum = vec3( 0 ).toVar();
	const weight = float( 1 ).toVar();

	Loop( { start: 0, end: 5, type: 'int', condition: '<' }, ( { i } ) => {

		sum.addAssign( vec3( uv(), float( i ).div( 5 ) ).mul( weight ) );
		weight.mulAssign( 0.5 );

	} );

	const result = sum.div( 2 ).toConst();
	return vec4( result, 1 );

} )();
