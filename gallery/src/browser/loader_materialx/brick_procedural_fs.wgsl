// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 1 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform0 : texture_2d<f32>;
@binding( 2 ) @group( 1 ) var nodeUniform1_sampler : sampler;
@binding( 3 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;
@binding( 4 ) @group( 1 ) var nodeUniform2_sampler : sampler;
@binding( 5 ) @group( 1 ) var nodeUniform2 : texture_2d<f32>;
@binding( 6 ) @group( 1 ) var nodeUniform3_sampler : sampler;
@binding( 7 ) @group( 1 ) var nodeUniform3 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform6_sampler : sampler;
@binding( 10 ) @group( 1 ) var nodeUniform6 : texture_2d<f32>;
@binding( 11 ) @group( 1 ) var nodeUniform13_sampler : sampler;
@binding( 12 ) @group( 1 ) var nodeUniform13 : texture_2d<f32>;
@binding( 13 ) @group( 1 ) var nodeUniform18_sampler : sampler;
@binding( 14 ) @group( 1 ) var nodeUniform18 : texture_2d<f32>;
@binding( 15 ) @group( 1 ) var nodeUniform24_sampler : sampler;
@binding( 16 ) @group( 1 ) var nodeUniform24 : texture_2d<f32>;

struct objectStruct {
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform9 : f32,
	nodeUniform10 : f32,
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform15 : mat3x3<f32>,
	nodeUniform17 : mat4x4<f32>,
	nodeUniform19 : f32,
	nodeUniform20 : mat4x4<f32>,
	nodeUniform22 : f32,
	nodeUniform23 : f32,
	nodeUniform25 : f32
};
@binding( 8 ) @group( 1 )
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
var<private> nodeVar0 : vec2<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec2<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec2<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> nodeVar8 : vec2<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> IOR : f32;
var<private> SpecularColor : vec3<f32>;
var<private> nodeVar12 : f32;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_tangentWorld : vec3<f32>;
var<private> nodeVar14 : vec2<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> NORMAL_bitangentWorld : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> NORMAL_normalWorld : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec2<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec2<f32>;
var<private> nodeVar34 : vec4<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : vec2<f32>;
var<private> nodeVar42 : vec4<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec2<f32>;
var<private> nodeVar52 : vec4<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : vec2<f32>;
var<private> nodeVar60 : vec4<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : f32;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : f32;
var<private> nodeVar119 : f32;
var<private> nodeVar120 : f32;
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
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec4<f32>;

// codes
fn mx_rgbtohsv ( c : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;
	var nodeVar7 : f32;
	var nodeVar8 : f32;
	var nodeVar9 : f32;

	nodeVar0 = c;
	nodeVar1 = nodeVar0.x;
	nodeVar2 = nodeVar0.y;
	nodeVar3 = nodeVar0.z;
	nodeVar4 = min( nodeVar1, min( nodeVar2, nodeVar3 ) );
	nodeVar5 = max( nodeVar1, max( nodeVar2, nodeVar3 ) );
	nodeVar6 = ( nodeVar5 - nodeVar4 );
	nodeVar7 = 0.0;
	nodeVar8 = 0.0;
	nodeVar9 = 0.0;
	nodeVar9 = nodeVar5;

	if ( ( nodeVar5 > 0.0 ) ) {

		nodeVar8 = ( nodeVar6 / nodeVar5 );
		

	} else {

		nodeVar8 = 0.0;
		

	}


	if ( ( nodeVar8 <= 0.0 ) ) {

		nodeVar7 = 0.0;
		

	} else {


		if ( ( nodeVar1 >= nodeVar5 ) ) {

			nodeVar7 = ( ( nodeVar2 - nodeVar3 ) / nodeVar6 );
			

		} else {


			if ( ( nodeVar2 >= nodeVar5 ) ) {

				nodeVar7 = ( 2.0 + ( ( nodeVar3 - nodeVar1 ) / nodeVar6 ) );
				

			} else {

				nodeVar7 = ( 4.0 + ( ( nodeVar1 - nodeVar2 ) / nodeVar6 ) );
				

			}

			

		}

		nodeVar7 = ( nodeVar7 * 0.16666666666666666 );

		if ( ( nodeVar7 < 0.0 ) ) {

			nodeVar7 = ( nodeVar7 + 1.0 );
			

		}

		

	}


	return vec3<f32>( nodeVar7, nodeVar8, nodeVar9 );

}


fn mx_hsvtorgb ( hsv : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;

	nodeVar0 = vec3<f32>( 0.0, 0.0, 0.0 );

	if ( ( hsv.y < 0.0001 ) ) {

		nodeVar0 = vec3<f32>( hsv.z, hsv.z, hsv.z );
		

	} else {

		nodeVar1 = ( ( hsv.x - floor( hsv.x ) ) * 6.0 );

		if ( ( i32( trunc( nodeVar1 ) ) == 0 ) ) {

			nodeVar0 = vec3<f32>( hsv.z, ( hsv.z * ( 1.0 - ( hsv.y * ( 1.0 - ( nodeVar1 - f32( i32( trunc( nodeVar1 ) ) ) ) ) ) ) ), ( hsv.z * ( 1.0 - hsv.y ) ) );
			

		} else {


			if ( ( i32( trunc( nodeVar1 ) ) == 1 ) ) {

				nodeVar2 = ( nodeVar1 - f32( i32( trunc( nodeVar1 ) ) ) );
				nodeVar3 = ( hsv.z * ( 1.0 - hsv.y ) );
				nodeVar0 = vec3<f32>( ( hsv.z * ( 1.0 - ( hsv.y * nodeVar2 ) ) ), hsv.z, nodeVar3 );
				

			} else {


				if ( ( i32( trunc( nodeVar1 ) ) == 2 ) ) {

					nodeVar4 = ( hsv.z * ( 1.0 - ( hsv.y * ( 1.0 - nodeVar2 ) ) ) );
					nodeVar0 = vec3<f32>( nodeVar3, hsv.z, nodeVar4 );
					

				} else {


					if ( ( i32( trunc( nodeVar1 ) ) == 3 ) ) {

						nodeVar5 = ( hsv.z * ( 1.0 - ( hsv.y * nodeVar2 ) ) );
						nodeVar0 = vec3<f32>( nodeVar3, nodeVar5, hsv.z );
						

					} else {


						if ( ( i32( trunc( nodeVar1 ) ) == 4 ) ) {

							nodeVar0 = vec3<f32>( nodeVar4, nodeVar3, hsv.z );
							

						} else {

							nodeVar0 = vec3<f32>( hsv.z, nodeVar3, nodeVar5 );
							

						}

						

					}

					

				}

				

			}

			

		}

		

	}


	return nodeVar0;

}


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




@fragment
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) v_tangentWorld : vec3<f32>,
	@location( 2 ) v_bitangentWorld : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) NORMAL_v_tangentWorld : vec3<f32>,
	@location( 5 ) NORMAL_v_bitangentWorld : vec3<f32>,
	@location( 6 ) nodeVarying12 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = ( ( vec2<f32>( nodeVarying12[ 0u ], ( 1.0 - nodeVarying12[ 1u ] ) ) * vec2<f32>( 3.0, 3.0 ) ) + vec2<f32>( 0.0 ) );
	nodeVar1 = textureSample( nodeUniform0, nodeUniform0_sampler, vec2<f32>( nodeVar0[ 0u ], ( 1.0 - nodeVar0[ 1u ] ) ) );
	nodeVar2 = ( ( vec2<f32>( nodeVarying12[ 0u ], ( 1.0 - nodeVarying12[ 1u ] ) ) * vec2<f32>( 3.0, 3.0 ) ) + vec2<f32>( 0.0 ) );
	nodeVar3 = textureSample( nodeUniform1, nodeUniform1_sampler, vec2<f32>( nodeVar2[ 0u ], ( 1.0 - nodeVar2[ 1u ] ) ) );
	nodeVar4 = ( ( vec2<f32>( nodeVarying12[ 0u ], ( 1.0 - nodeVarying12[ 1u ] ) ) * vec2<f32>( 3.0, 3.0 ) ) + vec2<f32>( 0.0 ) );
	nodeVar5 = textureSample( nodeUniform2, nodeUniform2_sampler, vec2<f32>( nodeVar4[ 0u ], ( 1.0 - nodeVar4[ 1u ] ) ) );
	nodeVar6 = ( ( vec2<f32>( nodeVarying12[ 0u ], ( 1.0 - nodeVarying12[ 1u ] ) ) * vec2<f32>( 3.0, 3.0 ) ) + vec2<f32>( 0.0 ) );
	nodeVar7 = textureSample( nodeUniform3, nodeUniform3_sampler, vec2<f32>( nodeVar6[ 0u ], ( 1.0 - nodeVar6[ 1u ] ) ) );
	DiffuseColor = vec4<f32>( min( max( mix( ( vec3<f32>( 0.263273, 0.263273, 0.263273 ) * vec3<f32>( nodeVar1.x ) ), ( mix( mx_hsvtorgb( ( vec3<f32>( ( ( ( ( 0.083 * nodeVar3.x ) + nodeVar1.x ) - 0.35 ) * 0.083 ), 0.0, ( ( ( 0.083 * nodeVar3.x ) + nodeVar1.x ) * ( 0.787 * nodeVar3.x ) ) ) + mx_rgbtohsv( vec3<f32>( 0.661876, 0.19088, 0.0 ) ) ) ), vec3<f32>( 0.56372, 0.56372, 0.56372 ), ( 0.248 * nodeVar5.x ) ) * vec3<f32>( nodeVar1.x ) ), nodeVar7.x ), vec3<f32>( 0.0 ) ), vec3<f32>( 1.0 ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform4 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform5;
	nodeVar8 = ( ( vec2<f32>( nodeVarying12[ 0u ], ( 1.0 - nodeVarying12[ 1u ] ) ) * vec2<f32>( 3.0, 3.0 ) ) + vec2<f32>( 0.0 ) );
	nodeVar9 = textureSample( nodeUniform6, nodeUniform6_sampler, vec2<f32>( nodeVar8[ 0u ], ( 1.0 - nodeVar8[ 1u ] ) ) );
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar11 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( ( 0.853 / max( nodeVar7.x, 0.00001 ) ) * nodeVar9.x ), 0.0525 ) + max( max( nodeVar11.x, nodeVar11.y ), nodeVar11.z ) ), 1.0 );
	IOR = object.nodeUniform9;
	nodeVar12 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar12 * nodeVar12 ) ) * vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform10 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform10, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform5 ) ) );
	EmissiveColor = ( object.nodeUniform11 * vec3<f32>( object.nodeUniform12 ) );
	NORMAL_tangentWorld = normalize( NORMAL_v_tangentWorld );
	nodeVar14 = ( ( vec2<f32>( nodeVarying12[ 0u ], ( 1.0 - nodeVarying12[ 1u ] ) ) * vec2<f32>( 3.0, 3.0 ) ) + vec2<f32>( 0.0 ) );
	nodeVar15 = textureSample( nodeUniform18, nodeUniform18_sampler, vec2<f32>( nodeVar14[ 0u ], ( 1.0 - nodeVar14[ 1u ] ) ) );
	nodeVar16 = vec3<f32>( nodeVar15.xyz[ 0u ], nodeVar15.xyz[ 1u ], nodeVar15.xyz[ 2u ] );
	nodeVar17 = mix( ( ( nodeVar16 * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 0.0, 0.0, 1.0 ), f32( ( dot( nodeVar16, nodeVar16 ) == 0.0 ) ) );
	NORMAL_bitangentWorld = normalize( NORMAL_v_bitangentWorld );
	NORMAL_normalView = normalViewGeometry;
	NORMAL_normalWorld = normalize( ( vec4<f32>( NORMAL_normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	normalView = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform15 * normalize( ( ( ( NORMAL_tangentWorld * vec3<f32>( ( nodeVar17[ 0u ] * vec2<f32>( 1.0 )[ 0u ] ) ) ) + ( NORMAL_bitangentWorld * vec3<f32>( ( nodeVar17[ 1u ] * vec2<f32>( 1.0 )[ 1u ] ) ) ) ) + ( NORMAL_normalWorld * vec3<f32>( nodeVar17[ 2u ] ) ) ) ) ), 0.0 ) ).xyz );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar18 = dot( normalView, positionViewDirection );
	nodeVar19 = textureSample( nodeUniform13, nodeUniform13_sampler, vec2<f32>( Roughness, clamp( nodeVar18, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar19;
	nodeVar20 = ( dfg.x + dfg.y );
	nodeVar21 = ( 1.0 / nodeVar20 );
	nodeVar22 = nodeVar21;
	nodeVar23 = ( nodeVar22 - 1.0 );
	nodeVar24 = ( SpecularColorBlended * vec3<f32>( nodeVar23 ) );
	nodeVar25 = ( nodeVar24 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar25;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar26 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform19 );
	nodeVar27 = floor( nodeVar26 );
	nodeVar28 = nodeVar27;
	nodeVar29 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar30 = getFace( ( object.nodeUniform20 * vec4<f32>( vec3<f32>( nodeVar29.x, ( - nodeVar29.y ), nodeVar29.z ), 1.0 ) ).xyz );
	nodeVar31 = max( ( 4.0 - nodeVar28 ), 0.0 );
	nodeVar28 = max( nodeVar28, 4.0 );
	nodeVar32 = exp2( nodeVar28 );
	nodeVar33 = ( ( getUV( ( object.nodeUniform20 * vec4<f32>( vec3<f32>( nodeVar29.x, ( - nodeVar29.y ), nodeVar29.z ), 1.0 ) ).xyz, nodeVar30 ) * vec2<f32>( ( nodeVar32 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar30 > 2.0 ) ) {

		nodeVar33.y = ( nodeVar33.y + nodeVar32 );
		nodeVar30 = ( nodeVar30 - 3.0 );
		

	}

	nodeVar33.x = ( nodeVar33.x + ( nodeVar30 * nodeVar32 ) );
	nodeVar33.x = ( nodeVar33.x + ( nodeVar31 * ( 3.0 * 16.0 ) ) );
	nodeVar33.y = ( nodeVar33.y + ( 4.0 * ( exp2( object.nodeUniform19 ) - nodeVar32 ) ) );
	nodeVar33.x = ( nodeVar33.x * object.nodeUniform22 );
	nodeVar33.y = ( nodeVar33.y * object.nodeUniform23 );
	nodeVar34 = textureSampleGrad( nodeUniform24, nodeUniform24_sampler, nodeVar33, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar35 = nodeVar34.xyz;
	nodeVar36 = fract( nodeVar26 );

	if ( ( nodeVar36 != 0.0 ) ) {

		nodeVar37 = ( nodeVar27 + 1.0 );
		nodeVar38 = getFace( ( object.nodeUniform20 * vec4<f32>( vec3<f32>( nodeVar29.x, ( - nodeVar29.y ), nodeVar29.z ), 1.0 ) ).xyz );
		nodeVar39 = max( ( 4.0 - nodeVar37 ), 0.0 );
		nodeVar37 = max( nodeVar37, 4.0 );
		nodeVar40 = exp2( nodeVar37 );
		nodeVar41 = ( ( getUV( ( object.nodeUniform20 * vec4<f32>( vec3<f32>( nodeVar29.x, ( - nodeVar29.y ), nodeVar29.z ), 1.0 ) ).xyz, nodeVar38 ) * vec2<f32>( ( nodeVar40 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar38 > 2.0 ) ) {

			nodeVar41.y = ( nodeVar41.y + nodeVar40 );
			nodeVar38 = ( nodeVar38 - 3.0 );
			

		}

		nodeVar41.x = ( nodeVar41.x + ( nodeVar38 * nodeVar40 ) );
		nodeVar41.x = ( nodeVar41.x + ( nodeVar39 * ( 3.0 * 16.0 ) ) );
		nodeVar41.y = ( nodeVar41.y + ( 4.0 * ( exp2( object.nodeUniform19 ) - nodeVar40 ) ) );
		nodeVar41.x = ( nodeVar41.x * object.nodeUniform22 );
		nodeVar41.y = ( nodeVar41.y * object.nodeUniform23 );
		nodeVar42 = textureSampleGrad( nodeUniform24, nodeUniform24_sampler, nodeVar41, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar43 = nodeVar42.xyz;
		nodeVar35 = mix( nodeVar35, nodeVar43, nodeVar36 );
		

	}

	nodeVar44 = ( radiance + ( nodeVar35 * vec3<f32>( object.nodeUniform25 ) ) );
	radiance = nodeVar44;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar45 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform19 );
	nodeVar46 = floor( nodeVar45 );
	nodeVar47 = nodeVar46;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar48 = getFace( ( object.nodeUniform20 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar49 = max( ( 4.0 - nodeVar47 ), 0.0 );
	nodeVar47 = max( nodeVar47, 4.0 );
	nodeVar50 = exp2( nodeVar47 );
	nodeVar51 = ( ( getUV( ( object.nodeUniform20 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar48 ) * vec2<f32>( ( nodeVar50 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar48 > 2.0 ) ) {

		nodeVar51.y = ( nodeVar51.y + nodeVar50 );
		nodeVar48 = ( nodeVar48 - 3.0 );
		

	}

	nodeVar51.x = ( nodeVar51.x + ( nodeVar48 * nodeVar50 ) );
	nodeVar51.x = ( nodeVar51.x + ( nodeVar49 * ( 3.0 * 16.0 ) ) );
	nodeVar51.y = ( nodeVar51.y + ( 4.0 * ( exp2( object.nodeUniform19 ) - nodeVar50 ) ) );
	nodeVar51.x = ( nodeVar51.x * object.nodeUniform22 );
	nodeVar51.y = ( nodeVar51.y * object.nodeUniform23 );
	nodeVar52 = textureSampleGrad( nodeUniform24, nodeUniform24_sampler, nodeVar51, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar53 = nodeVar52.xyz;
	nodeVar54 = fract( nodeVar45 );

	if ( ( nodeVar54 != 0.0 ) ) {

		nodeVar55 = ( nodeVar46 + 1.0 );
		nodeVar56 = getFace( ( object.nodeUniform20 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar57 = max( ( 4.0 - nodeVar55 ), 0.0 );
		nodeVar55 = max( nodeVar55, 4.0 );
		nodeVar58 = exp2( nodeVar55 );
		nodeVar59 = ( ( getUV( ( object.nodeUniform20 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar56 ) * vec2<f32>( ( nodeVar58 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar56 > 2.0 ) ) {

			nodeVar59.y = ( nodeVar59.y + nodeVar58 );
			nodeVar56 = ( nodeVar56 - 3.0 );
			

		}

		nodeVar59.x = ( nodeVar59.x + ( nodeVar56 * nodeVar58 ) );
		nodeVar59.x = ( nodeVar59.x + ( nodeVar57 * ( 3.0 * 16.0 ) ) );
		nodeVar59.y = ( nodeVar59.y + ( 4.0 * ( exp2( object.nodeUniform19 ) - nodeVar58 ) ) );
		nodeVar59.x = ( nodeVar59.x * object.nodeUniform22 );
		nodeVar59.y = ( nodeVar59.y * object.nodeUniform23 );
		nodeVar60 = textureSampleGrad( nodeUniform24, nodeUniform24_sampler, nodeVar59, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar61 = nodeVar60.xyz;
		nodeVar53 = mix( nodeVar53, nodeVar61, nodeVar54 );
		

	}

	nodeVar62 = ( iblIrradiance + ( ( nodeVar53 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform25 ) ) );
	iblIrradiance = nodeVar62;
	nodeVar63 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar64 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar65 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar66 = ( SpecularF90 * dfg.y );
	nodeVar67 = ( nodeVar65 + vec3<f32>( nodeVar66 ) );
	nodeVar68 = ( nodeVar63 + nodeVar67 );
	nodeVar63 = nodeVar68;
	nodeVar69 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar70 = nodeVar69;
	nodeVar71 = ( nodeVar70 * vec3<f32>( 0.047619 ) );
	nodeVar72 = ( SpecularColor + nodeVar71 );
	nodeVar73 = ( nodeVar67 * nodeVar72 );
	nodeVar74 = ( dfg.x + dfg.y );
	nodeVar75 = ( 1.0 - nodeVar74 );
	nodeVar76 = nodeVar75;
	nodeVar77 = ( vec3<f32>( nodeVar76 ) * nodeVar72 );
	nodeVar78 = ( vec3<f32>( 1.0 ) - nodeVar77 );
	nodeVar79 = nodeVar78;
	nodeVar80 = ( nodeVar73 / nodeVar79 );
	nodeVar81 = ( nodeVar80 * vec3<f32>( nodeVar76 ) );
	nodeVar82 = ( nodeVar64 + nodeVar81 );
	nodeVar64 = nodeVar82;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar83 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar84 = ( irradiance * nodeVar83 );
	nodeVar85 = ( nodeVar63 + nodeVar64 );
	nodeVar86 = ( vec3<f32>( 1.0 ) - nodeVar85 );
	nodeVar87 = nodeVar86;
	nodeVar88 = ( nodeVar84 * nodeVar87 );
	nodeVar89 = nodeVar88;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar90 = ( indirectDiffuse + nodeVar89 );
	indirectDiffuse = nodeVar90;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar91 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar92 = ( SpecularF90 * dfg.y );
	nodeVar93 = ( nodeVar91 + vec3<f32>( nodeVar92 ) );
	nodeVar94 = ( singleScatteringDielectric + nodeVar93 );
	singleScatteringDielectric = nodeVar94;
	nodeVar95 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar96 = nodeVar95;
	nodeVar97 = ( nodeVar96 * vec3<f32>( 0.047619 ) );
	nodeVar98 = ( SpecularColor + nodeVar97 );
	nodeVar99 = ( nodeVar93 * nodeVar98 );
	nodeVar100 = ( dfg.x + dfg.y );
	nodeVar101 = ( 1.0 - nodeVar100 );
	nodeVar102 = nodeVar101;
	nodeVar103 = ( vec3<f32>( nodeVar102 ) * nodeVar98 );
	nodeVar104 = ( vec3<f32>( 1.0 ) - nodeVar103 );
	nodeVar105 = nodeVar104;
	nodeVar106 = ( nodeVar99 / nodeVar105 );
	nodeVar107 = ( nodeVar106 * vec3<f32>( nodeVar102 ) );
	nodeVar108 = ( multiScatteringDielectric + nodeVar107 );
	multiScatteringDielectric = nodeVar108;
	nodeVar109 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar110 = ( SpecularF90 * dfg.y );
	nodeVar111 = ( nodeVar109 + vec3<f32>( nodeVar110 ) );
	nodeVar112 = ( singleScatteringMetallic + nodeVar111 );
	singleScatteringMetallic = nodeVar112;
	nodeVar113 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar114 = nodeVar113;
	nodeVar115 = ( nodeVar114 * vec3<f32>( 0.047619 ) );
	nodeVar116 = ( DiffuseColor.xyz + nodeVar115 );
	nodeVar117 = ( nodeVar111 * nodeVar116 );
	nodeVar118 = ( dfg.x + dfg.y );
	nodeVar119 = ( 1.0 - nodeVar118 );
	nodeVar120 = nodeVar119;
	nodeVar121 = ( vec3<f32>( nodeVar120 ) * nodeVar116 );
	nodeVar122 = ( vec3<f32>( 1.0 ) - nodeVar121 );
	nodeVar123 = nodeVar122;
	nodeVar124 = ( nodeVar117 / nodeVar123 );
	nodeVar125 = ( nodeVar124 * vec3<f32>( nodeVar120 ) );
	nodeVar126 = ( multiScatteringMetallic + nodeVar125 );
	multiScatteringMetallic = nodeVar126;
	nodeVar127 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar128 = ( radiance * nodeVar127 );
	nodeVar129 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar130 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar131 = ( nodeVar129 * nodeVar130 );
	nodeVar132 = ( nodeVar128 + nodeVar131 );
	nodeVar133 = nodeVar132;
	nodeVar134 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar135 = ( vec3<f32>( 1.0 ) - nodeVar134 );
	nodeVar136 = nodeVar135;
	nodeVar137 = ( DiffuseContribution * nodeVar136 );
	nodeVar138 = ( nodeVar137 * nodeVar130 );
	nodeVar139 = nodeVar138;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar140 = ( indirectSpecular + nodeVar133 );
	indirectSpecular = nodeVar140;
	nodeVar141 = ( indirectDiffuse + nodeVar139 );
	indirectDiffuse = nodeVar141;
	ambientOcclusion = 1.0;
	nodeVar142 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar142;
	nodeVar143 = dot( normalView, positionViewDirection );
	nodeVar144 = ( clamp( nodeVar143, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar145 = ( Roughness * -16.0 );
	nodeVar146 = ( 1.0 - nodeVar145 );
	nodeVar147 = nodeVar146;
	nodeVar148 = ( - nodeVar147 );
	nodeVar149 = exp2( nodeVar148 );
	nodeVar150 = pow( nodeVar144, nodeVar149 );
	nodeVar151 = ( 1.0 - nodeVar150 );
	nodeVar152 = nodeVar151;
	nodeVar153 = ( ambientOcclusion - nodeVar152 );
	nodeVar154 = ( indirectSpecular * vec3<f32>( clamp( nodeVar153, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar154;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar155 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar155;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar156 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar156;
	nodeVar157 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar157;
	nodeVar158 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar158;

	// result

	output.color = nodeVar158;

	return output;

}
