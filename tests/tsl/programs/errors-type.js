const { vec4, uv, float, vec2 } = await import( 'three/tsl' );

// A deliberate type mismatch and a non-node output.
const bad = vec2( uv(), uv() );
output = vec4( bad, float( 'x' ), 1 );
