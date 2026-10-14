const { vec4, uv, int, uint, float, vec3, bitAnd, bitOr, bitXor, shiftLeft, shiftRight, bitNot } = await import( 'three/tsl' );

const x = uint( uv().x.mul( 255 ) );
const y = uint( uv().y.mul( 255 ) );
const v = bitXor( bitAnd( x, y ), bitOr( shiftLeft( x, uint( 1 ) ), shiftRight( y, uint( 2 ) ) ) );
const w = int( bitNot( int( v ) ) ).abs();
output = vec4( vec3( float( v ).div( 255 ), float( w ).div( 512 ), 0.5 ), 1 );
