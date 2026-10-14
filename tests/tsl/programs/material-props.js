const { vec4, materialColor, materialOpacity, materialRoughness, uv, mix } = await import( 'three/tsl' );

output = vec4( mix( materialColor, uv().toVec3(), 0.5 ).mul( materialRoughness.add( 0.5 ) ), materialOpacity );
