const { vec4, positionWorld, positionView, positionLocal, normalView, normalWorld, cameraPosition, modelViewMatrix, transformedNormalView } = await import( 'three/tsl' );

const viewDir = cameraPosition.sub( positionWorld ).normalize();
const fresnel = normalWorld.dot( viewDir ).oneMinus().pow( 2 );
output = vec4( normalView.mul( 0.5 ).add( 0.5 ).mul( fresnel ).add( positionLocal.mul( 0.1 ) ).add( transformedNormalView.mul( 0.01 ) ).add( modelViewMatrix.mul( positionView.toVec4() ).xyz.mul( 0.001 ) ), 1 );
