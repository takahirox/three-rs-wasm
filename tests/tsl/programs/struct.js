const { Fn, struct, vec3, vec4, float, uv } = await import( 'three/tsl' );

const Light = struct( {
	color: 'vec3',
	intensity: 'float'
}, 'Light' );

const shade = Fn( ( [ light ] ) => {

	return light.get( 'color' ).mul( light.get( 'intensity' ) );

} );

const light = Light( vec3( 1, 0.5, 0.25 ), float( 0.8 ) );
output = vec4( shade( light ).mul( uv().x ), 1 );
