const { Fn, vec3, vec4, float, uv, sin, cos, time } = await import( 'three/tsl' );

const wave = Fn( ( [ p, t ] ) => {

	return sin( p.mul( 10.0 ).add( t ) ).mul( 0.5 ).add( 0.5 );

} ).setLayout( {
	name: 'wave',
	type: 'float',
	inputs: [
		{ name: 'p', type: 'float' },
		{ name: 't', type: 'float' }
	]
} );

const st = uv();
output = vec4( wave( st.x, time ), wave( st.y, time.mul( 1.3 ) ), cos( time ).mul( 0.5 ).add( 0.5 ), float( 1 ) );
