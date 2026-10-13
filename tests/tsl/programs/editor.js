// Simple uv.x animation
const { texture, uniform, vec2, vec4, uv, oscSine, time, grayscale } = await import( 'three/tsl' );

const samplerTexture = new THREE.TextureLoader().load( './textures/uv_grid_opengl.jpg' );
samplerTexture.wrapS = THREE.RepeatWrapping;
samplerTexture.colorSpace = THREE.SRGBColorSpace;

const scaledTime = time.mul( .5 ); // .5 is speed
const uv0 = uv();
const animateUv = vec2( uv0.x.add( oscSine( scaledTime ) ), uv0.y );

// .setName() is optional
const myMap = texture( samplerTexture, animateUv ).rgb.setName( 'myTexture' );
const myColor = uniform( new THREE.Color( 0x0066ff ) ).setName( 'myColor' );
const opacity = .7;

const desaturatedMap = grayscale( myMap.rgb );

const finalColor = desaturatedMap.add( myColor );

output = vec4( finalColor, opacity );
