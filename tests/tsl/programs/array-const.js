const { array, vec3, vec4, uv, int, uniform } = await import( 'three/tsl' );

const palette = array( [ vec3( 1, 0, 0 ), vec3( 0, 1, 0 ), vec3( 0, 0, 1 ), vec3( 1, 1, 0 ) ] );
const index = int( uv().x.mul( 4.0 ) ).clamp( 0, 3 );
output = vec4( palette.element( index ), uniform( 1.0 ) );
