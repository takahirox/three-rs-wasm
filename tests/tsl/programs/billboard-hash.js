const { vec4, vec3, uv, hash, time, triNoise3D, fract } = await import( 'three/tsl' );

const h = hash( uv().mul( 512 ).floor().x.add( uv().mul( 512 ).floor().y.mul( 512 ) ) );
const n = triNoise3D( vec3( uv(), time.mul( 0.1 ) ), 0.5, time );
output = vec4( vec3( h, n, fract( time ) ), 1 );
