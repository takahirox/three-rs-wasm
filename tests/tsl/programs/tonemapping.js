const { vec4, vec3, uv, acesFilmicToneMapping, agxToneMapping, neutralToneMapping, reinhardToneMapping, cineonToneMapping, linearToneMapping, float } = await import( 'three/tsl' );

const hdr = vec3( uv().mul( 4 ), 2 );
const e = float( 1 );
const a = acesFilmicToneMapping( hdr, e ).add( agxToneMapping( hdr, e ) ).add( neutralToneMapping( hdr, e ) );
const b = reinhardToneMapping( hdr, e ).add( cineonToneMapping( hdr, e ) ).add( linearToneMapping( hdr, e ) );
output = vec4( a.add( b ).div( 6 ), 1 );
