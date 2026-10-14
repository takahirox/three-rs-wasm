const { uniform, vec4, uv, mix, time, oscSquare, oscTriangle, oscSawtooth } = await import( 'three/tsl' );

const colorA = uniform( new THREE.Color( 0xff0066 ) );
const colorB = uniform( new THREE.Color( 0x0066ff ) );
const amount = uniform( 0.5 ).onFrameUpdate( ( { time } ) => Math.sin( time ) * 0.5 + 0.5 );
const vector = uniform( new THREE.Vector3( 1, 2, 3 ) );
const matrix = uniform( new THREE.Matrix4() );

const t = oscSquare( time ).add( oscTriangle( time ) ).add( oscSawtooth( time ) ).div( 3.0 );
output = vec4( mix( colorA, colorB, uv().x.mul( amount ).add( t ) ).add( matrix.mul( vector.normalize() ).xyz.mul( 0.01 ) ), 1 );
