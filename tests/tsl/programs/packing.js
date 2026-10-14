const { vec4, vec3, uv, packHalf2x16, unpackHalf2x16, packSnorm2x16, unpackSnorm2x16, directionToColor, colorToDirection } = await import( 'three/tsl' );

const packed = packHalf2x16( uv() );
const back = unpackHalf2x16( packed );
const s = unpackSnorm2x16( packSnorm2x16( uv().sub( 0.5 ) ) );
const dir = colorToDirection( directionToColor( vec3( back, 0.5 ).normalize() ) );
output = vec4( dir.mul( 0.5 ).add( 0.5 ).add( vec3( s, 0 ).mul( 0.1 ) ), 1 );
