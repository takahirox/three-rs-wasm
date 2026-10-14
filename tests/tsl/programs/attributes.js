const { vec4, attribute, normalLocal, positionGeometry, uv } = await import( 'three/tsl' );

output = vec4( attribute( 'position', 'vec3' ).add( normalLocal ).add( positionGeometry ).mul( 0.5 ).add( uv().toVec3().mul( 0.1 ) ), 1 );
