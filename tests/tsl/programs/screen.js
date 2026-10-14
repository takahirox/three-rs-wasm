const { vec4, screenUV, screenCoordinate, screenSize, viewportUV, uv, fract, length } = await import( 'three/tsl' );

const grid = fract( screenCoordinate.xy.div( 20 ) ).step( 0.5 );
const d = length( screenUV.sub( 0.5 ) );
output = vec4( grid.x.mul( grid.y ).mul( d ).add( viewportUV.x.mul( 0.1 ) ).add( screenSize.x.mul( 0.0001 ) ).add( uv().y.mul( 0.1 ) ).toVec3(), 1 );
