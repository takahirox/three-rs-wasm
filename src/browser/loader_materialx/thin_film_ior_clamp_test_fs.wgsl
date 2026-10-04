// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform8 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform15_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform15 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform1 : f32,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform10 : f32,
	nodeUniform11 : mat4x4<f32>,
	nodeUniform13 : f32,
	nodeUniform14 : f32,
	nodeUniform16 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> IOR : f32;
var<private> SpecularColor : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> Iridescence : f32;
var<private> IridescenceIOR : f32;
var<private> IridescenceThickness : f32;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : vec2<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec2<f32>;
var<private> nodeVar18 : vec4<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : vec2<f32>;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec2<f32>;
var<private> nodeVar36 : vec4<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : vec2<f32>;
var<private> nodeVar44 : vec4<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : f32;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec4<f32>;

// codes
fn roughnessToMip ( roughness : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = 0.0;

	if ( ( roughness >= 0.8 ) ) {

		nodeVar0 = ( ( ( ( 1.0 - roughness ) * ( -1.0 - -2.0 ) ) / ( 1.0 - 0.8 ) ) + -2.0 );
		

	} else {


		if ( ( roughness >= 0.4 ) ) {

			nodeVar0 = ( ( ( ( 0.8 - roughness ) * ( 2.0 - -1.0 ) ) / ( 0.8 - 0.4 ) ) + -1.0 );
			

		} else {


			if ( ( roughness >= 0.305 ) ) {

				nodeVar0 = ( ( ( ( 0.4 - roughness ) * ( 3.0 - 2.0 ) ) / ( 0.4 - 0.305 ) ) + 2.0 );
				

			} else {


				if ( ( roughness >= 0.21 ) ) {

					nodeVar0 = ( ( ( ( 0.305 - roughness ) * ( 4.0 - 3.0 ) ) / ( 0.305 - 0.21 ) ) + 3.0 );
					

				} else {

					nodeVar0 = ( -2.0 * log2( ( 1.16 * roughness ) ) );
					

				}

				

			}

			

		}

		

	}


	return nodeVar0;

}


fn getFace ( direction : vec3<f32> ) -> f32 {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;

	nodeVar0 = abs( direction );
	nodeVar1 = -1.0;

	if ( ( nodeVar0.x > nodeVar0.z ) ) {


		if ( ( nodeVar0.x > nodeVar0.y ) ) {


			if ( ( direction.x > 0.0 ) ) {

				nodeVar2 = 0.0;

			} else {

				nodeVar2 = 3.0;

			}

			nodeVar1 = nodeVar2;
			

		} else {


			if ( ( direction.y > 0.0 ) ) {

				nodeVar3 = 1.0;

			} else {

				nodeVar3 = 4.0;

			}

			nodeVar1 = nodeVar3;
			

		}

		

	} else {


		if ( ( nodeVar0.z > nodeVar0.y ) ) {


			if ( ( direction.z > 0.0 ) ) {

				nodeVar4 = 2.0;

			} else {

				nodeVar4 = 5.0;

			}

			nodeVar1 = nodeVar4;
			

		} else {


			if ( ( direction.y > 0.0 ) ) {

				nodeVar5 = 1.0;

			} else {

				nodeVar5 = 4.0;

			}

			nodeVar1 = nodeVar5;
			

		}

		

	}


	return nodeVar1;

}


fn getUV ( direction : vec3<f32>, face : f32 ) -> vec2<f32> {

	var nodeVar0 : vec2<f32>;

	nodeVar0 = vec2<f32>( 0.0, 0.0 );

	if ( ( face == 0.0 ) ) {

		nodeVar0 = ( vec2<f32>( direction.z, direction.y ) / vec2<f32>( abs( direction.x ) ) );
		

	} else {


		if ( ( face == 1.0 ) ) {

			nodeVar0 = ( vec2<f32>( ( - direction.x ), ( - direction.z ) ) / vec2<f32>( abs( direction.y ) ) );
			

		} else {


			if ( ( face == 2.0 ) ) {

				nodeVar0 = ( vec2<f32>( ( - direction.x ), direction.y ) / vec2<f32>( abs( direction.z ) ) );
				

			} else {


				if ( ( face == 3.0 ) ) {

					nodeVar0 = ( vec2<f32>( ( - direction.z ), direction.y ) / vec2<f32>( abs( direction.x ) ) );
					

				} else {


					if ( ( face == 4.0 ) ) {

						nodeVar0 = ( vec2<f32>( ( - direction.x ), direction.z ) / vec2<f32>( abs( direction.y ) ) );
						

					} else {

						nodeVar0 = ( vec2<f32>( direction.x, direction.y ) / vec2<f32>( abs( direction.z ) ) );
						

					}

					

				}

				

			}

			

		}

		

	}


	return ( vec2<f32>( 0.5 ) * ( nodeVar0 + vec2<f32>( 1.0 ) ) );

}


fn evalIridescence ( outsideIOR : f32, eta2 : f32, cosTheta1 : f32, thinFilmThickness : f32, baseF0 : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;
	var nodeVar7 : vec3<f32>;
	var nodeVar8 : vec3<f32>;
	var nodeVar9 : vec3<f32>;
	var nodeVar10 : f32;
	var nodeVar11 : f32;
	var nodeVar12 : vec3<f32>;
	var nodeVar13 : vec3<f32>;
	var nodeVar14 : vec3<f32>;
	var nodeVar15 : vec3<f32>;
	var nodeVar16 : vec3<f32>;
	var nodeVar17 : f32;
	var nodeVar18 : f32;
	var nodeVar19 : f32;
	var nodeVar20 : f32;
	var nodeVar21 : f32;
	var nodeVar22 : vec3<f32>;
	var nodeVar23 : vec3<f32>;

	nodeVar0 = mix( outsideIOR, eta2, smoothstep( 0.0, 0.03, thinFilmThickness ) );
	nodeVar1 = ( outsideIOR / nodeVar0 );
	nodeVar2 = ( 1.0 - ( ( nodeVar1 * nodeVar1 ) * ( 1.0 - ( cosTheta1 * cosTheta1 ) ) ) );

	if ( ( nodeVar2 < 0.0 ) ) {

		return vec3<f32>( 1.0, 1.0, 1.0 );

	}

	nodeVar3 = ( ( nodeVar0 - outsideIOR ) / ( nodeVar0 + outsideIOR ) );
	nodeVar4 = exp2( ( ( ( cosTheta1 * -5.55473 ) - 6.98316 ) * cosTheta1 ) );
	nodeVar5 = ( ( ( nodeVar3 * nodeVar3 ) * ( 1.0 - nodeVar4 ) ) + ( 1.0 * nodeVar4 ) );
	nodeVar6 = ( 1.0 - nodeVar5 );
	nodeVar7 = sqrt( clamp( baseF0, vec3<f32>( 0.0 ), vec3<f32>( 0.9999 ) ) );
	nodeVar8 = ( ( vec3<f32>( 1.0, 1.0, 1.0 ) + nodeVar7 ) / ( vec3<f32>( 1.0, 1.0, 1.0 ) - nodeVar7 ) );
	nodeVar9 = ( ( nodeVar8 - vec3<f32>( nodeVar0 ) ) / ( nodeVar8 + vec3<f32>( nodeVar0 ) ) );
	nodeVar10 = sqrt( nodeVar2 );
	nodeVar11 = exp2( ( ( ( nodeVar10 * -5.55473 ) - 6.98316 ) * nodeVar10 ) );
	nodeVar12 = ( ( ( nodeVar9 * nodeVar9 ) * vec3<f32>( ( 1.0 - nodeVar11 ) ) ) + vec3<f32>( ( 1.0 * nodeVar11 ) ) );
	nodeVar13 = clamp( ( vec3<f32>( nodeVar5 ) * nodeVar12 ), vec3<f32>( 0.00001 ), vec3<f32>( 0.9999 ) );
	nodeVar14 = ( ( vec3<f32>( ( nodeVar6 * nodeVar6 ) ) * nodeVar12 ) / ( vec3<f32>( 1.0, 1.0, 1.0 ) - nodeVar13 ) );
	nodeVar15 = ( vec3<f32>( nodeVar5 ) + nodeVar14 );
	nodeVar16 = ( nodeVar14 - vec3<f32>( nodeVar6 ) );

	for ( var m : i32 = 1; m <= 2; m ++ ) {

		nodeVar16 = ( nodeVar16 * sqrt( nodeVar13 ) );
		nodeVar17 = ( ( f32( m ) * ( ( ( nodeVar0 * thinFilmThickness ) * nodeVar10 ) * 2.0 ) ) * 6.283185307179586e-9 );

		if ( ( nodeVar0 < outsideIOR ) ) {

			nodeVar18 = 3.141592653589793;

		} else {

			nodeVar18 = 0.0;

		}


		if ( ( nodeVar8.x < nodeVar0 ) ) {

			nodeVar19 = 3.141592653589793;

		} else {

			nodeVar19 = 0.0;

		}


		if ( ( nodeVar8.y < nodeVar0 ) ) {

			nodeVar20 = 3.141592653589793;

		} else {

			nodeVar20 = 0.0;

		}


		if ( ( nodeVar8.z < nodeVar0 ) ) {

			nodeVar21 = 3.141592653589793;

		} else {

			nodeVar21 = 0.0;

		}

		nodeVar22 = ( vec3<f32>( f32( m ) ) * ( vec3<f32>( ( 3.141592653589793 - nodeVar18 ) ) + vec3<f32>( nodeVar19, nodeVar20, nodeVar21 ) ) );
		nodeVar23 = ( ( ( vec3<f32>( 5.4856e-13, 4.4201e-13, 5.2481e-13 ) * sqrt( ( vec3<f32>( 4327800000.0, 9304600000.0, 6612100000.0 ) * vec3<f32>( 6.283185307179586 ) ) ) ) * cos( ( ( vec3<f32>( 1681000.0, 1795300.0, 2208400.0 ) * vec3<f32>( nodeVar17 ) ) + nodeVar22 ) ) ) * exp( ( vec3<f32>( ( - ( nodeVar17 * nodeVar17 ) ) ) * vec3<f32>( 4327800000.0, 9304600000.0, 6612100000.0 ) ) ) );
		nodeVar15 = ( nodeVar15 + ( nodeVar16 * ( ( mat3x3<f32>( 3.2404542, -0.969266, 0.0556434, -1.5371385, 1.8760108, -0.2040259, -0.4985314, 0.041556, 1.0572252 ) * ( vec3<f32>( ( nodeVar23.x + ( ( 1.6440828550896444e-8 * cos( ( ( nodeVar17 * 2239900.0 ) + nodeVar22.x ) ) ) * exp( ( ( nodeVar17 * nodeVar17 ) * -4528200000.0 ) ) ) ), nodeVar23.y, nodeVar23.z ) / vec3<f32>( 1.0685e-7 ) ) ) * vec3<f32>( 2.0 ) ) ) );

	}


	return max( nodeVar15, vec3<f32>( 0.0, 0.0, 0.0 ) );

}


fn Schlick_to_F0 ( f : vec3<f32>, f90 : f32, dotVH : f32 ) -> vec3<f32> {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;

	nodeVar0 = clamp( ( 1.0 - dotVH ), 0.0, 1.0 );
	nodeVar1 = ( nodeVar0 * nodeVar0 );
	nodeVar2 = clamp( ( ( nodeVar0 * nodeVar1 ) * nodeVar1 ), 0.0, 0.9999 );

	return ( ( f - ( vec3<f32>( f90 ) * vec3<f32>( nodeVar2 ) ) ) / vec3<f32>( ( 1.0 - nodeVar2 ) ) );

}




@fragment
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) v_positionViewDirection : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( vec3<f32>( 0.1, 0.1, 0.1 ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform1;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( 0.1, 0.0525 ) + max( max( nodeVar0.x, nodeVar0.y ), nodeVar0.z ) ), 1.0 );
	IOR = object.nodeUniform4;
	nodeVar1 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar1 * nodeVar1 ) ) * vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform5 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform5, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform1 ) ) );
	Iridescence = 1.0;
	IridescenceIOR = clamp( 3.5, 1.0, 2.333 );
	IridescenceThickness = 200.0;
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar2 = dot( normalView, positionViewDirection );
	nodeVar3 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( nodeVar2, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar3;
	nodeVar4 = ( dfg.x + dfg.y );
	nodeVar5 = ( 1.0 / nodeVar4 );
	nodeVar6 = nodeVar5;
	nodeVar7 = ( nodeVar6 - 1.0 );
	nodeVar8 = ( SpecularColorBlended * vec3<f32>( nodeVar7 ) );
	nodeVar9 = ( nodeVar8 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar9;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar10 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform10 );
	nodeVar11 = floor( nodeVar10 );
	nodeVar12 = nodeVar11;
	nodeVar13 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar14 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar13.x, ( - nodeVar13.y ), nodeVar13.z ), 1.0 ) ).xyz );
	nodeVar15 = max( ( 4.0 - nodeVar12 ), 0.0 );
	nodeVar12 = max( nodeVar12, 4.0 );
	nodeVar16 = exp2( nodeVar12 );
	nodeVar17 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar13.x, ( - nodeVar13.y ), nodeVar13.z ), 1.0 ) ).xyz, nodeVar14 ) * vec2<f32>( ( nodeVar16 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar14 > 2.0 ) ) {

		nodeVar17.y = ( nodeVar17.y + nodeVar16 );
		nodeVar14 = ( nodeVar14 - 3.0 );
		

	}

	nodeVar17.x = ( nodeVar17.x + ( nodeVar14 * nodeVar16 ) );
	nodeVar17.x = ( nodeVar17.x + ( nodeVar15 * ( 3.0 * 16.0 ) ) );
	nodeVar17.y = ( nodeVar17.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar16 ) ) );
	nodeVar17.x = ( nodeVar17.x * object.nodeUniform13 );
	nodeVar17.y = ( nodeVar17.y * object.nodeUniform14 );
	nodeVar18 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar17, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar19 = nodeVar18.xyz;
	nodeVar20 = fract( nodeVar10 );

	if ( ( nodeVar20 != 0.0 ) ) {

		nodeVar21 = ( nodeVar11 + 1.0 );
		nodeVar22 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar13.x, ( - nodeVar13.y ), nodeVar13.z ), 1.0 ) ).xyz );
		nodeVar23 = max( ( 4.0 - nodeVar21 ), 0.0 );
		nodeVar21 = max( nodeVar21, 4.0 );
		nodeVar24 = exp2( nodeVar21 );
		nodeVar25 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar13.x, ( - nodeVar13.y ), nodeVar13.z ), 1.0 ) ).xyz, nodeVar22 ) * vec2<f32>( ( nodeVar24 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar22 > 2.0 ) ) {

			nodeVar25.y = ( nodeVar25.y + nodeVar24 );
			nodeVar22 = ( nodeVar22 - 3.0 );
			

		}

		nodeVar25.x = ( nodeVar25.x + ( nodeVar22 * nodeVar24 ) );
		nodeVar25.x = ( nodeVar25.x + ( nodeVar23 * ( 3.0 * 16.0 ) ) );
		nodeVar25.y = ( nodeVar25.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar24 ) ) );
		nodeVar25.x = ( nodeVar25.x * object.nodeUniform13 );
		nodeVar25.y = ( nodeVar25.y * object.nodeUniform14 );
		nodeVar26 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar25, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar27 = nodeVar26.xyz;
		nodeVar19 = mix( nodeVar19, nodeVar27, nodeVar20 );
		

	}

	nodeVar28 = ( radiance + ( nodeVar19 * vec3<f32>( object.nodeUniform16 ) ) );
	radiance = nodeVar28;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar29 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform10 );
	nodeVar30 = floor( nodeVar29 );
	nodeVar31 = nodeVar30;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar32 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar33 = max( ( 4.0 - nodeVar31 ), 0.0 );
	nodeVar31 = max( nodeVar31, 4.0 );
	nodeVar34 = exp2( nodeVar31 );
	nodeVar35 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar32 ) * vec2<f32>( ( nodeVar34 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar32 > 2.0 ) ) {

		nodeVar35.y = ( nodeVar35.y + nodeVar34 );
		nodeVar32 = ( nodeVar32 - 3.0 );
		

	}

	nodeVar35.x = ( nodeVar35.x + ( nodeVar32 * nodeVar34 ) );
	nodeVar35.x = ( nodeVar35.x + ( nodeVar33 * ( 3.0 * 16.0 ) ) );
	nodeVar35.y = ( nodeVar35.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar34 ) ) );
	nodeVar35.x = ( nodeVar35.x * object.nodeUniform13 );
	nodeVar35.y = ( nodeVar35.y * object.nodeUniform14 );
	nodeVar36 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar35, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar37 = nodeVar36.xyz;
	nodeVar38 = fract( nodeVar29 );

	if ( ( nodeVar38 != 0.0 ) ) {

		nodeVar39 = ( nodeVar30 + 1.0 );
		nodeVar40 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar41 = max( ( 4.0 - nodeVar39 ), 0.0 );
		nodeVar39 = max( nodeVar39, 4.0 );
		nodeVar42 = exp2( nodeVar39 );
		nodeVar43 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar40 ) * vec2<f32>( ( nodeVar42 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar40 > 2.0 ) ) {

			nodeVar43.y = ( nodeVar43.y + nodeVar42 );
			nodeVar40 = ( nodeVar40 - 3.0 );
			

		}

		nodeVar43.x = ( nodeVar43.x + ( nodeVar40 * nodeVar42 ) );
		nodeVar43.x = ( nodeVar43.x + ( nodeVar41 * ( 3.0 * 16.0 ) ) );
		nodeVar43.y = ( nodeVar43.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar42 ) ) );
		nodeVar43.x = ( nodeVar43.x * object.nodeUniform13 );
		nodeVar43.y = ( nodeVar43.y * object.nodeUniform14 );
		nodeVar44 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar43, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar45 = nodeVar44.xyz;
		nodeVar37 = mix( nodeVar37, nodeVar45, nodeVar38 );
		

	}

	nodeVar46 = ( iblIrradiance + ( ( nodeVar37 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform16 ) ) );
	iblIrradiance = nodeVar46;
	nodeVar47 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar48 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar49 = dot( normalView, positionViewDirection );
	nodeVar50 = clamp( nodeVar49, 0.0, 1.0 );
	nodeVar51 = evalIridescence( 1.0, IridescenceIOR, nodeVar50, IridescenceThickness, SpecularColor );
	nodeVar52 = Schlick_to_F0( nodeVar51, 1.0, nodeVar50 );
	nodeVar53 = mix( SpecularColor, nodeVar52, Iridescence );
	nodeVar54 = ( nodeVar53 * vec3<f32>( dfg.x ) );
	nodeVar55 = ( SpecularF90 * dfg.y );
	nodeVar56 = ( nodeVar54 + vec3<f32>( nodeVar55 ) );
	nodeVar57 = ( nodeVar47 + nodeVar56 );
	nodeVar47 = nodeVar57;
	nodeVar58 = ( vec3<f32>( 1.0 ) - nodeVar53 );
	nodeVar59 = nodeVar58;
	nodeVar60 = ( nodeVar59 * vec3<f32>( 0.047619 ) );
	nodeVar61 = ( nodeVar53 + nodeVar60 );
	nodeVar62 = ( nodeVar56 * nodeVar61 );
	nodeVar63 = ( dfg.x + dfg.y );
	nodeVar64 = ( 1.0 - nodeVar63 );
	nodeVar65 = nodeVar64;
	nodeVar66 = ( vec3<f32>( nodeVar65 ) * nodeVar61 );
	nodeVar67 = ( vec3<f32>( 1.0 ) - nodeVar66 );
	nodeVar68 = nodeVar67;
	nodeVar69 = ( nodeVar62 / nodeVar68 );
	nodeVar70 = ( nodeVar69 * vec3<f32>( nodeVar65 ) );
	nodeVar71 = ( nodeVar48 + nodeVar70 );
	nodeVar48 = nodeVar71;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar72 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar73 = ( irradiance * nodeVar72 );
	nodeVar74 = ( nodeVar47 + nodeVar48 );
	nodeVar75 = ( vec3<f32>( 1.0 ) - nodeVar74 );
	nodeVar76 = nodeVar75;
	nodeVar77 = ( nodeVar73 * nodeVar76 );
	nodeVar78 = nodeVar77;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar79 = ( indirectDiffuse + nodeVar78 );
	indirectDiffuse = nodeVar79;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar80 = mix( SpecularColor, nodeVar52, Iridescence );
	nodeVar81 = ( nodeVar80 * vec3<f32>( dfg.x ) );
	nodeVar82 = ( SpecularF90 * dfg.y );
	nodeVar83 = ( nodeVar81 + vec3<f32>( nodeVar82 ) );
	nodeVar84 = ( singleScatteringDielectric + nodeVar83 );
	singleScatteringDielectric = nodeVar84;
	nodeVar85 = ( vec3<f32>( 1.0 ) - nodeVar80 );
	nodeVar86 = nodeVar85;
	nodeVar87 = ( nodeVar86 * vec3<f32>( 0.047619 ) );
	nodeVar88 = ( nodeVar80 + nodeVar87 );
	nodeVar89 = ( nodeVar83 * nodeVar88 );
	nodeVar90 = ( dfg.x + dfg.y );
	nodeVar91 = ( 1.0 - nodeVar90 );
	nodeVar92 = nodeVar91;
	nodeVar93 = ( vec3<f32>( nodeVar92 ) * nodeVar88 );
	nodeVar94 = ( vec3<f32>( 1.0 ) - nodeVar93 );
	nodeVar95 = nodeVar94;
	nodeVar96 = ( nodeVar89 / nodeVar95 );
	nodeVar97 = ( nodeVar96 * vec3<f32>( nodeVar92 ) );
	nodeVar98 = ( multiScatteringDielectric + nodeVar97 );
	multiScatteringDielectric = nodeVar98;
	nodeVar99 = evalIridescence( 1.0, IridescenceIOR, nodeVar50, IridescenceThickness, DiffuseColor.xyz );
	nodeVar100 = Schlick_to_F0( nodeVar99, 1.0, nodeVar50 );
	nodeVar101 = mix( DiffuseColor.xyz, nodeVar100, Iridescence );
	nodeVar102 = ( nodeVar101 * vec3<f32>( dfg.x ) );
	nodeVar103 = ( SpecularF90 * dfg.y );
	nodeVar104 = ( nodeVar102 + vec3<f32>( nodeVar103 ) );
	nodeVar105 = ( singleScatteringMetallic + nodeVar104 );
	singleScatteringMetallic = nodeVar105;
	nodeVar106 = ( vec3<f32>( 1.0 ) - nodeVar101 );
	nodeVar107 = nodeVar106;
	nodeVar108 = ( nodeVar107 * vec3<f32>( 0.047619 ) );
	nodeVar109 = ( nodeVar101 + nodeVar108 );
	nodeVar110 = ( nodeVar104 * nodeVar109 );
	nodeVar111 = ( dfg.x + dfg.y );
	nodeVar112 = ( 1.0 - nodeVar111 );
	nodeVar113 = nodeVar112;
	nodeVar114 = ( vec3<f32>( nodeVar113 ) * nodeVar109 );
	nodeVar115 = ( vec3<f32>( 1.0 ) - nodeVar114 );
	nodeVar116 = nodeVar115;
	nodeVar117 = ( nodeVar110 / nodeVar116 );
	nodeVar118 = ( nodeVar117 * vec3<f32>( nodeVar113 ) );
	nodeVar119 = ( multiScatteringMetallic + nodeVar118 );
	multiScatteringMetallic = nodeVar119;
	nodeVar120 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar121 = ( radiance * nodeVar120 );
	nodeVar122 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar123 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar124 = ( nodeVar122 * nodeVar123 );
	nodeVar125 = ( nodeVar121 + nodeVar124 );
	nodeVar126 = nodeVar125;
	nodeVar127 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar128 = ( vec3<f32>( 1.0 ) - nodeVar127 );
	nodeVar129 = nodeVar128;
	nodeVar130 = ( DiffuseContribution * nodeVar129 );
	nodeVar131 = ( nodeVar130 * nodeVar123 );
	nodeVar132 = nodeVar131;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar133 = ( indirectSpecular + nodeVar126 );
	indirectSpecular = nodeVar133;
	nodeVar134 = ( indirectDiffuse + nodeVar132 );
	indirectDiffuse = nodeVar134;
	ambientOcclusion = 1.0;
	nodeVar135 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar135;
	nodeVar136 = dot( normalView, positionViewDirection );
	nodeVar137 = ( clamp( nodeVar136, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar138 = ( Roughness * -16.0 );
	nodeVar139 = ( 1.0 - nodeVar138 );
	nodeVar140 = nodeVar139;
	nodeVar141 = ( - nodeVar140 );
	nodeVar142 = exp2( nodeVar141 );
	nodeVar143 = pow( nodeVar137, nodeVar142 );
	nodeVar144 = ( 1.0 - nodeVar143 );
	nodeVar145 = nodeVar144;
	nodeVar146 = ( ambientOcclusion - nodeVar145 );
	nodeVar147 = ( indirectSpecular * vec3<f32>( clamp( nodeVar146, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar147;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar148 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar148;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar149 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar149;
	nodeVar150 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar150;
	nodeVar151 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar151;

	// result

	output.color = nodeVar151;

	return output;

}
