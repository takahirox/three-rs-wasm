const { vec4, vec3, uv, select, and, or, not, xor, bool } = await import( 'three/tsl' );

const a = uv().x.greaterThan( 0.5 );
const b = uv().y.lessThanEqual( 0.5 );
const c = and( a, b ).or( not( a ).and( not( b ) ) );
output = vec4( select( c, vec3( 1, 0.5, 0 ), vec3( 0, 0.5, 1 ) ).mul( select( xor( a, b ), 1, 0.5 ) ), 1 );
