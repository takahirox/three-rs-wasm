const { vec4, normalMap, texture, uv, vec3, bumpMap, normalView } = await import( 'three/tsl' );

const data = new THREE.DataTexture( new Uint8Array( [ 128, 128, 255, 255 ] ), 1, 1 );
data.needsUpdate = true;
const n = normalMap( texture( data, uv() ) );
const b = bumpMap( texture( data, uv() ).r );
output = vec4( n.add( b ).add( normalView ).mul( 0.33 ).mul( 0.5 ).add( 0.5 ), 1 );
