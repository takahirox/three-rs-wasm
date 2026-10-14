const { texture, uv, vec4, rotateUV, vec2, time, PI } = await import( 'three/tsl' );

const map = new THREE.TextureLoader().load( './textures/uv_grid_opengl.jpg' );
map.wrapS = map.wrapT = THREE.RepeatWrapping;
map.colorSpace = THREE.SRGBColorSpace;

const rotated = rotateUV( uv(), time.mul( PI.div( 8 ) ), vec2( 0.5 ) );
output = vec4( texture( map, rotated.mul( 2 ) ).rgb, 1 );
