// Three.js r186 webgpu_tsl_interoperability (MIT), CRT by Xor.
fn crtFragment(
						vUv: vec2f,
						tex: texture_2d<f32>,
						texSampler: sampler,
						crtWidth: f32,
						crtHeight: f32,
						cellOffset: f32,
						cellSize: f32,
						borderMask: f32,
						time: f32,
						speed: f32,
						pulseIntensity: f32,
						pulseWidth: f32,
						pulseRate: f32
					) -> vec3<f32> {
						// Convert uv into map of pixels
						var pixel = ( vUv * 0.5 + 0.5 ) * vec2<f32>(
							crtWidth,
							crtHeight
						);
						// Coordinate for each cell in the pixel map
						let coord = pixel / cellSize;
						// Three color values for each cell (r, g, b)
						let subcoord = coord * vec2f( 3.0, 1.0 );
						let offset = vec2<f32>( 0, fract( floor( coord.x ) * cellOffset ) );

						let maskCoord = floor( coord + offset ) * cellSize;

						var samplePoint = maskCoord / vec2<f32>(crtWidth, crtHeight);
						samplePoint.x += fract( time * speed / 20 );

						var color = textureSample(
							tex,
							texSampler,
							samplePoint
						).xyz;

						// Current implementation does not give an even amount of space to each r, g, b unit of a cell
						// Fix/hack this by multiplying subCoord.x by cellSize at cellSizes below 6
						let ind = floor( subcoord.x ) % 3;

						var maskColor = vec3<f32>(
							f32( ind == 0.0 ),
							f32( ind == 1.0 ),
							f32( ind == 2.0 )
						) * 3.0;

						let cellUV = fract( subcoord + offset ) * 2.0 - 1.0;
						var border: vec2<f32> = 1.0 - cellUV * cellUV * borderMask;

						maskColor *= vec3f( clamp( border.x, 0.0, 1.0 ) * clamp( border.y, 0.0, 1.0) );

						color *= maskColor;

						color.r *= 1.0 + pulseIntensity * sin( pixel.y / pulseWidth + time * pulseRate );
						color.b *= 1.0 + pulseIntensity * sin( pixel.y / pulseWidth + time * pulseRate );
						color.g *= 1.0 + pulseIntensity * sin( pixel.y / pulseWidth + time * pulseRate );

						return color;
					}
