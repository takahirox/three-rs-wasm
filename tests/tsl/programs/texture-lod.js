const { texture, uv, vec4, textureSize, float } = await import( 'three/tsl' );

const map = new THREE.DataTexture( new Uint8Array( [ 255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255 ] ), 2, 2 );
map.needsUpdate = true;

const size = textureSize( texture( map ), 0 );
output = vec4( texture( map, uv() ).level( 0 ).rgb.mul( float( size.x ).div( 2 ) ), 1 );
