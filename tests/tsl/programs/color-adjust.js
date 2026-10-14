const { vec4, vec3, uv, saturation, vibrance, hue, luminance, posterize, grayscale, cdl, time } = await import( 'three/tsl' );

const base = vec3( uv(), 0.5 );
const adjusted = hue( vibrance( saturation( base, 1.5 ), 0.3 ), time );
const lum = luminance( adjusted );
const graded = cdl( posterize( adjusted, 8 ), vec3( 1.1 ), vec3( 0.0 ), vec3( 0.9 ), 1.2 );
output = vec4( graded.mix( grayscale( graded ), lum.oneMinus() ), 1 );
