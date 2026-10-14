const { vec4, vec3, uv, remap, remapClamp, float } = await import( 'three/tsl' );

const x = uv().x;
output = vec4( vec3( remap( x, 0.2, 0.8, 0, 1 ), remapClamp( uv().y, 0.25, 0.75 ), float( 0.5 ) ), 1 );
