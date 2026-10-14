const { vec4, uv, dFdx, dFdy, fwidth, length, vec3 } = await import( 'three/tsl' );

const p = uv().mul( 10 );
const g = vec3( dFdx( p.x ), dFdy( p.y ), length( fwidth( p ) ) );
output = vec4( g.mul( 10 ), 1 );
