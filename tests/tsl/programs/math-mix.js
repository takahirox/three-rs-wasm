const { vec4, vec3, uv, smoothstep, step, clamp, saturate, mix, abs, sign, fract, floor, ceil, round, trunc, mod, pow, exp, exp2, log, log2, sqrt, inverseSqrt, atan, min, max, reflect, refract, distance, normalize, cross, dot, length, negate, oneMinus, reciprocal } = await import( 'three/tsl' );

const p = vec3( uv(), 0.25 );
const a = smoothstep( 0.2, 0.8, p ).add( step( 0.5, p ) ).add( clamp( p, 0.1, 0.9 ) ).add( saturate( p.mul( 2 ) ) );
const b = abs( sign( p.sub( 0.5 ) ) ).add( fract( p.mul( 3 ) ) ).add( floor( p ) ).add( ceil( p ) ).add( round( p ) ).add( trunc( p ) );
const c = mod( p, 0.3 ).add( pow( p, vec3( 2 ) ) ).add( exp( p ) ).add( exp2( p ) ).add( log( p.add( 1 ) ) ).add( log2( p.add( 1 ) ) );
const d = sqrt( p ).add( inverseSqrt( p.add( 1 ) ) ).add( atan( p.y, p.x ) ).add( min( p, 0.5 ) ).add( max( p, 0.5 ) );
const e = reflect( normalize( p ), vec3( 0, 1, 0 ) ).add( refract( normalize( p ), vec3( 0, 0, 1 ), 0.9 ) ).add( cross( p, vec3( 1, 0, 0 ) ) );
const f = distance( p, vec3( 0.5 ) ).add( dot( p, p ) ).add( length( p ) ).add( negate( p.x ) ).add( oneMinus( p.y ) ).add( reciprocal( p.z ) );
output = vec4( mix( a.add( b ), c.add( d ), 0.5 ).add( e ).mul( f ).mul( 0.05 ), 1 );
