const { vec4, vec3, uv, checker, length, smoothstep, mix } = await import( 'three/tsl' );

const c = checker( uv().mul( 8 ) );
const d = length( uv().sub( 0.5 ) );
output = vec4( mix( vec3( c ), vec3( 1, 0, 0 ), smoothstep( 0.3, 0.29, d ) ), 1 );
