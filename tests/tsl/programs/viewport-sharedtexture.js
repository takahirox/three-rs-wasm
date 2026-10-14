const { vec4, viewportSharedTexture, viewportUV, uv } = await import( 'three/tsl' );

output = vec4( viewportSharedTexture( viewportUV.add( uv().mul( 0.01 ) ) ).rgb.oneMinus(), 1 );
