const { mx_noise_float, mx_fractal_noise_vec3, mx_worley_noise_float, mx_cell_noise_float, vec3, vec4, uv, time } = await import( 'three/tsl' );

const p = uv().mul( 6 ).toVec3().setZ( time.mul( 0.2 ) );
const n = mx_noise_float( p ).mul( 0.5 ).add( 0.5 );
const f = mx_fractal_noise_vec3( p, 4, 2.0, 0.5 );
const w = mx_worley_noise_float( p );
const c = mx_cell_noise_float( p );
output = vec4( vec3( n ).mul( f ).add( w.mul( 0.1 ) ).add( c.mul( 0.05 ) ), 1 );
