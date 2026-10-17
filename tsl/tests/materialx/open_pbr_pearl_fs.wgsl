// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform9_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform9 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform15_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform15 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform1 : f32,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : mat4x4<f32>,
	nodeUniform7 : vec3<f32>,
	nodeUniform8 : f32,
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
var<private> Clearcoat : f32;
var<private> nodeVar2 : f32;
var<private> ClearcoatRoughness : f32;
var<private> nodeVar3 : vec3<f32>;
var<private> Iridescence : f32;
var<private> IridescenceIOR : f32;
var<private> IridescenceThickness : f32;
var<private> nodeVar4 : vec2<f32>;
var<private> Anisotropy : f32;
var<private> AlphaT : f32;
var<private> AnisotropyT : vec3<f32>;
var<private> tangentView : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> bitangentView : vec3<f32>;
var<private> TBNViewMatrix : mat3x3<f32>;
var<private> AnisotropyB : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> clearcoatRadiance : vec3<f32>;
var<private> clearcoatSpecularDirect : vec3<f32>;
var<private> clearcoatSpecularIndirect : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : vec2<f32>;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : vec2<f32>;
var<private> nodeVar24 : vec4<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec2<f32>;
var<private> nodeVar32 : vec4<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : vec2<f32>;
var<private> nodeVar42 : vec4<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : vec2<f32>;
var<private> nodeVar50 : vec4<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : f32;
var<private> clearcoatNormalView : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : vec2<f32>;
var<private> nodeVar61 : vec4<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : vec2<f32>;
var<private> nodeVar69 : vec4<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : vec2<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : f32;
var<private> nodeVar121 : f32;
var<private> nodeVar122 : f32;
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
var<private> nodeVar133 : f32;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : f32;
var<private> nodeVar168 : f32;
var<private> nodeVar169 : f32;
var<private> nodeVar170 : f32;
var<private> nodeVar171 : f32;
var<private> nodeVar172 : f32;
var<private> nodeVar173 : f32;
var<private> nodeVar174 : f32;
var<private> nodeVar175 : f32;
var<private> nodeVar176 : f32;
var<private> nodeVar177 : f32;
var<private> nodeVar178 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar179 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar180 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar181 : vec3<f32>;
var<private> nodeVar182 : f32;
var<private> nodeVar183 : f32;
var<private> nodeVar184 : f32;
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> nodeVar187 : vec3<f32>;
var<private> nodeVar188 : vec3<f32>;
var<private> nodeVar189 : vec3<f32>;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : vec3<f32>;
var<private> nodeVar193 : vec4<f32>;

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
	@location( 1 ) v_tangentView : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) nodeVarying7 : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( ( vec3<f32>( 1.0 ) * vec3<f32>( 0.8, 0.75, 0.7 ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform1;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( 0.35, 0.0525 ) + max( max( nodeVar0.x, nodeVar0.y ), nodeVar0.z ) ), 1.0 );
	IOR = object.nodeUniform4;
	nodeVar1 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar1 * nodeVar1 ) ) * vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform5 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform5, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform1 ) ) );
	nodeVar2 = ( ( 1.68 - 1.0 ) / ( 1.68 + 1.0 ) );
	Clearcoat = clamp( ( 1.0 * ( ( nodeVar2 * nodeVar2 ) / 0.04 ) ), 0.0, 1.0 );
	nodeVar3 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	ClearcoatRoughness = min( ( max( 0.15, 0.0525 ) + max( max( nodeVar3.x, nodeVar3.y ), nodeVar3.z ) ), 1.0 );
	Iridescence = 1.0;
	IridescenceIOR = 2.0;
	IridescenceThickness = ( 0.42 * 1000.0 );
	nodeVar4 = ( vec2<f32>( cos( 0.0 ), sin( 0.0 ) ) * vec2<f32>( 0.0 ) );
	Anisotropy = length( nodeVar4 );

	if ( ( Anisotropy == 0.0 ) ) {

		nodeVar4 = vec2<f32>( 1.0, 0.0 );
		

	} else {

		nodeVar4 = ( nodeVar4 / vec2<f32>( Anisotropy ) );
		Anisotropy = clamp( Anisotropy, 0.0, 1.0 );
		

	}

	AlphaT = mix( ( Roughness * Roughness ), 1.0, ( Anisotropy * Anisotropy ) );
	tangentView = normalize( v_tangentView );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	bitangentView = normalize( ( cross( normalView, tangentView ) * vec3<f32>( nodeVarying7.w ) ) );
	TBNViewMatrix = mat3x3<f32>( tangentView, bitangentView, normalView );
	AnisotropyT = ( ( TBNViewMatrix[ 0u ] * vec3<f32>( nodeVar4.x ) ) + ( TBNViewMatrix[ 1u ] * vec3<f32>( nodeVar4.y ) ) );
	AnisotropyB = ( ( TBNViewMatrix[ 1u ] * vec3<f32>( nodeVar4.x ) ) - ( TBNViewMatrix[ 0u ] * vec3<f32>( nodeVar4.y ) ) );
	EmissiveColor = ( object.nodeUniform7 * vec3<f32>( object.nodeUniform8 ) );
	clearcoatRadiance = vec3<f32>( 0.0, 0.0, 0.0 );
	clearcoatSpecularDirect = vec3<f32>( 0.0, 0.0, 0.0 );
	clearcoatSpecularIndirect = vec3<f32>( 0.0, 0.0, 0.0 );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar5 = dot( normalView, positionViewDirection );
	nodeVar6 = textureSample( nodeUniform9, nodeUniform9_sampler, vec2<f32>( Roughness, clamp( nodeVar5, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar6;
	nodeVar7 = ( dfg.x + dfg.y );
	nodeVar8 = ( 1.0 / nodeVar7 );
	nodeVar9 = nodeVar8;
	nodeVar10 = ( nodeVar9 - 1.0 );
	nodeVar11 = ( SpecularColorBlended * vec3<f32>( nodeVar10 ) );
	nodeVar12 = ( nodeVar11 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar12;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar13 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform10 );
	nodeVar14 = floor( nodeVar13 );
	nodeVar15 = nodeVar14;
	nodeVar16 = ( 1.0 - ( Anisotropy * ( 1.0 - Roughness ) ) );
	nodeVar17 = ( nodeVar16 * nodeVar16 );
	nodeVar18 = normalize( mix( normalize( cross( cross( AnisotropyB, positionViewDirection ), AnisotropyB ) ), normalView, ( nodeVar17 * nodeVar17 ) ) );
	nodeVar19 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), nodeVar18 ), nodeVar18, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar20 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar19.x, ( - nodeVar19.y ), nodeVar19.z ), 1.0 ) ).xyz );
	nodeVar21 = max( ( 4.0 - nodeVar15 ), 0.0 );
	nodeVar15 = max( nodeVar15, 4.0 );
	nodeVar22 = exp2( nodeVar15 );
	nodeVar23 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar19.x, ( - nodeVar19.y ), nodeVar19.z ), 1.0 ) ).xyz, nodeVar20 ) * vec2<f32>( ( nodeVar22 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar20 > 2.0 ) ) {

		nodeVar23.y = ( nodeVar23.y + nodeVar22 );
		nodeVar20 = ( nodeVar20 - 3.0 );
		

	}

	nodeVar23.x = ( nodeVar23.x + ( nodeVar20 * nodeVar22 ) );
	nodeVar23.x = ( nodeVar23.x + ( nodeVar21 * ( 3.0 * 16.0 ) ) );
	nodeVar23.y = ( nodeVar23.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar22 ) ) );
	nodeVar23.x = ( nodeVar23.x * object.nodeUniform13 );
	nodeVar23.y = ( nodeVar23.y * object.nodeUniform14 );
	nodeVar24 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar23, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar25 = nodeVar24.xyz;
	nodeVar26 = fract( nodeVar13 );

	if ( ( nodeVar26 != 0.0 ) ) {

		nodeVar27 = ( nodeVar14 + 1.0 );
		nodeVar28 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar19.x, ( - nodeVar19.y ), nodeVar19.z ), 1.0 ) ).xyz );
		nodeVar29 = max( ( 4.0 - nodeVar27 ), 0.0 );
		nodeVar27 = max( nodeVar27, 4.0 );
		nodeVar30 = exp2( nodeVar27 );
		nodeVar31 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar19.x, ( - nodeVar19.y ), nodeVar19.z ), 1.0 ) ).xyz, nodeVar28 ) * vec2<f32>( ( nodeVar30 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar28 > 2.0 ) ) {

			nodeVar31.y = ( nodeVar31.y + nodeVar30 );
			nodeVar28 = ( nodeVar28 - 3.0 );
			

		}

		nodeVar31.x = ( nodeVar31.x + ( nodeVar28 * nodeVar30 ) );
		nodeVar31.x = ( nodeVar31.x + ( nodeVar29 * ( 3.0 * 16.0 ) ) );
		nodeVar31.y = ( nodeVar31.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar30 ) ) );
		nodeVar31.x = ( nodeVar31.x * object.nodeUniform13 );
		nodeVar31.y = ( nodeVar31.y * object.nodeUniform14 );
		nodeVar32 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar31, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar33 = nodeVar32.xyz;
		nodeVar25 = mix( nodeVar25, nodeVar33, nodeVar26 );
		

	}

	nodeVar34 = ( radiance + ( nodeVar25 * vec3<f32>( object.nodeUniform16 ) ) );
	radiance = nodeVar34;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar35 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform10 );
	nodeVar36 = floor( nodeVar35 );
	nodeVar37 = nodeVar36;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar38 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar39 = max( ( 4.0 - nodeVar37 ), 0.0 );
	nodeVar37 = max( nodeVar37, 4.0 );
	nodeVar40 = exp2( nodeVar37 );
	nodeVar41 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar38 ) * vec2<f32>( ( nodeVar40 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar38 > 2.0 ) ) {

		nodeVar41.y = ( nodeVar41.y + nodeVar40 );
		nodeVar38 = ( nodeVar38 - 3.0 );
		

	}

	nodeVar41.x = ( nodeVar41.x + ( nodeVar38 * nodeVar40 ) );
	nodeVar41.x = ( nodeVar41.x + ( nodeVar39 * ( 3.0 * 16.0 ) ) );
	nodeVar41.y = ( nodeVar41.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar40 ) ) );
	nodeVar41.x = ( nodeVar41.x * object.nodeUniform13 );
	nodeVar41.y = ( nodeVar41.y * object.nodeUniform14 );
	nodeVar42 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar41, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar43 = nodeVar42.xyz;
	nodeVar44 = fract( nodeVar35 );

	if ( ( nodeVar44 != 0.0 ) ) {

		nodeVar45 = ( nodeVar36 + 1.0 );
		nodeVar46 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar47 = max( ( 4.0 - nodeVar45 ), 0.0 );
		nodeVar45 = max( nodeVar45, 4.0 );
		nodeVar48 = exp2( nodeVar45 );
		nodeVar49 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar46 ) * vec2<f32>( ( nodeVar48 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar46 > 2.0 ) ) {

			nodeVar49.y = ( nodeVar49.y + nodeVar48 );
			nodeVar46 = ( nodeVar46 - 3.0 );
			

		}

		nodeVar49.x = ( nodeVar49.x + ( nodeVar46 * nodeVar48 ) );
		nodeVar49.x = ( nodeVar49.x + ( nodeVar47 * ( 3.0 * 16.0 ) ) );
		nodeVar49.y = ( nodeVar49.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar48 ) ) );
		nodeVar49.x = ( nodeVar49.x * object.nodeUniform13 );
		nodeVar49.y = ( nodeVar49.y * object.nodeUniform14 );
		nodeVar50 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar49, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar51 = nodeVar50.xyz;
		nodeVar43 = mix( nodeVar43, nodeVar51, nodeVar44 );
		

	}

	nodeVar52 = ( iblIrradiance + ( ( nodeVar43 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform16 ) ) );
	iblIrradiance = nodeVar52;
	nodeVar53 = clamp( roughnessToMip( ClearcoatRoughness ), -2.0, object.nodeUniform10 );
	nodeVar54 = floor( nodeVar53 );
	nodeVar55 = nodeVar54;
	clearcoatNormalView = NORMAL_normalView;
	nodeVar56 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), clearcoatNormalView ), clearcoatNormalView, ( ( ( ClearcoatRoughness * ClearcoatRoughness ) * ClearcoatRoughness ) * ClearcoatRoughness ) ) ), 0.0 ) ).xyz );
	nodeVar57 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar56.x, ( - nodeVar56.y ), nodeVar56.z ), 1.0 ) ).xyz );
	nodeVar58 = max( ( 4.0 - nodeVar55 ), 0.0 );
	nodeVar55 = max( nodeVar55, 4.0 );
	nodeVar59 = exp2( nodeVar55 );
	nodeVar60 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar56.x, ( - nodeVar56.y ), nodeVar56.z ), 1.0 ) ).xyz, nodeVar57 ) * vec2<f32>( ( nodeVar59 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar57 > 2.0 ) ) {

		nodeVar60.y = ( nodeVar60.y + nodeVar59 );
		nodeVar57 = ( nodeVar57 - 3.0 );
		

	}

	nodeVar60.x = ( nodeVar60.x + ( nodeVar57 * nodeVar59 ) );
	nodeVar60.x = ( nodeVar60.x + ( nodeVar58 * ( 3.0 * 16.0 ) ) );
	nodeVar60.y = ( nodeVar60.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar59 ) ) );
	nodeVar60.x = ( nodeVar60.x * object.nodeUniform13 );
	nodeVar60.y = ( nodeVar60.y * object.nodeUniform14 );
	nodeVar61 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar60, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar62 = nodeVar61.xyz;
	nodeVar63 = fract( nodeVar53 );

	if ( ( nodeVar63 != 0.0 ) ) {

		nodeVar64 = ( nodeVar54 + 1.0 );
		nodeVar65 = getFace( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar56.x, ( - nodeVar56.y ), nodeVar56.z ), 1.0 ) ).xyz );
		nodeVar66 = max( ( 4.0 - nodeVar64 ), 0.0 );
		nodeVar64 = max( nodeVar64, 4.0 );
		nodeVar67 = exp2( nodeVar64 );
		nodeVar68 = ( ( getUV( ( object.nodeUniform11 * vec4<f32>( vec3<f32>( nodeVar56.x, ( - nodeVar56.y ), nodeVar56.z ), 1.0 ) ).xyz, nodeVar65 ) * vec2<f32>( ( nodeVar67 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar65 > 2.0 ) ) {

			nodeVar68.y = ( nodeVar68.y + nodeVar67 );
			nodeVar65 = ( nodeVar65 - 3.0 );
			

		}

		nodeVar68.x = ( nodeVar68.x + ( nodeVar65 * nodeVar67 ) );
		nodeVar68.x = ( nodeVar68.x + ( nodeVar66 * ( 3.0 * 16.0 ) ) );
		nodeVar68.y = ( nodeVar68.y + ( 4.0 * ( exp2( object.nodeUniform10 ) - nodeVar67 ) ) );
		nodeVar68.x = ( nodeVar68.x * object.nodeUniform13 );
		nodeVar68.y = ( nodeVar68.y * object.nodeUniform14 );
		nodeVar69 = textureSampleGrad( nodeUniform15, nodeUniform15_sampler, nodeVar68, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar70 = nodeVar69.xyz;
		nodeVar62 = mix( nodeVar62, nodeVar70, nodeVar63 );
		

	}

	nodeVar71 = ( clearcoatRadiance + ( nodeVar62 * vec3<f32>( object.nodeUniform16 ) ) );
	clearcoatRadiance = nodeVar71;
	nodeVar72 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar73 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar74 = dot( normalView, positionViewDirection );
	nodeVar75 = clamp( nodeVar74, 0.0, 1.0 );
	nodeVar76 = evalIridescence( 1.0, IridescenceIOR, nodeVar75, IridescenceThickness, SpecularColor );
	nodeVar77 = Schlick_to_F0( nodeVar76, 1.0, nodeVar75 );
	nodeVar78 = mix( SpecularColor, nodeVar77, Iridescence );
	nodeVar79 = ( nodeVar78 * vec3<f32>( dfg.x ) );
	nodeVar80 = ( SpecularF90 * dfg.y );
	nodeVar81 = ( nodeVar79 + vec3<f32>( nodeVar80 ) );
	nodeVar82 = ( nodeVar72 + nodeVar81 );
	nodeVar72 = nodeVar82;
	nodeVar83 = ( vec3<f32>( 1.0 ) - nodeVar78 );
	nodeVar84 = nodeVar83;
	nodeVar85 = ( nodeVar84 * vec3<f32>( 0.047619 ) );
	nodeVar86 = ( nodeVar78 + nodeVar85 );
	nodeVar87 = ( nodeVar81 * nodeVar86 );
	nodeVar88 = ( dfg.x + dfg.y );
	nodeVar89 = ( 1.0 - nodeVar88 );
	nodeVar90 = nodeVar89;
	nodeVar91 = ( vec3<f32>( nodeVar90 ) * nodeVar86 );
	nodeVar92 = ( vec3<f32>( 1.0 ) - nodeVar91 );
	nodeVar93 = nodeVar92;
	nodeVar94 = ( nodeVar87 / nodeVar93 );
	nodeVar95 = ( nodeVar94 * vec3<f32>( nodeVar90 ) );
	nodeVar96 = ( nodeVar73 + nodeVar95 );
	nodeVar73 = nodeVar96;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar97 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar98 = ( irradiance * nodeVar97 );
	nodeVar99 = ( nodeVar72 + nodeVar73 );
	nodeVar100 = ( vec3<f32>( 1.0 ) - nodeVar99 );
	nodeVar101 = nodeVar100;
	nodeVar102 = ( nodeVar98 * nodeVar101 );
	nodeVar103 = nodeVar102;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar104 = ( indirectDiffuse + nodeVar103 );
	indirectDiffuse = nodeVar104;
	nodeVar105 = dot( clearcoatNormalView, positionViewDirection );
	nodeVar106 = textureSample( nodeUniform9, nodeUniform9_sampler, vec2<f32>( ClearcoatRoughness, clamp( nodeVar105, 0.0, 1.0 ) ) ).xy;
	nodeVar107 = ( ( vec3<f32>( 0.04, 0.04, 0.04 ) * vec3<f32>( nodeVar106.x ) ) + vec3<f32>( ( 1.0 * nodeVar106.y ) ) );
	nodeVar108 = ( clearcoatRadiance * nodeVar107 );
	nodeVar109 = ( clearcoatSpecularIndirect + nodeVar108 );
	clearcoatSpecularIndirect = nodeVar109;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar110 = mix( SpecularColor, nodeVar77, Iridescence );
	nodeVar111 = ( nodeVar110 * vec3<f32>( dfg.x ) );
	nodeVar112 = ( SpecularF90 * dfg.y );
	nodeVar113 = ( nodeVar111 + vec3<f32>( nodeVar112 ) );
	nodeVar114 = ( singleScatteringDielectric + nodeVar113 );
	singleScatteringDielectric = nodeVar114;
	nodeVar115 = ( vec3<f32>( 1.0 ) - nodeVar110 );
	nodeVar116 = nodeVar115;
	nodeVar117 = ( nodeVar116 * vec3<f32>( 0.047619 ) );
	nodeVar118 = ( nodeVar110 + nodeVar117 );
	nodeVar119 = ( nodeVar113 * nodeVar118 );
	nodeVar120 = ( dfg.x + dfg.y );
	nodeVar121 = ( 1.0 - nodeVar120 );
	nodeVar122 = nodeVar121;
	nodeVar123 = ( vec3<f32>( nodeVar122 ) * nodeVar118 );
	nodeVar124 = ( vec3<f32>( 1.0 ) - nodeVar123 );
	nodeVar125 = nodeVar124;
	nodeVar126 = ( nodeVar119 / nodeVar125 );
	nodeVar127 = ( nodeVar126 * vec3<f32>( nodeVar122 ) );
	nodeVar128 = ( multiScatteringDielectric + nodeVar127 );
	multiScatteringDielectric = nodeVar128;
	nodeVar129 = evalIridescence( 1.0, IridescenceIOR, nodeVar75, IridescenceThickness, DiffuseColor.xyz );
	nodeVar130 = Schlick_to_F0( nodeVar129, 1.0, nodeVar75 );
	nodeVar131 = mix( DiffuseColor.xyz, nodeVar130, Iridescence );
	nodeVar132 = ( nodeVar131 * vec3<f32>( dfg.x ) );
	nodeVar133 = ( SpecularF90 * dfg.y );
	nodeVar134 = ( nodeVar132 + vec3<f32>( nodeVar133 ) );
	nodeVar135 = ( singleScatteringMetallic + nodeVar134 );
	singleScatteringMetallic = nodeVar135;
	nodeVar136 = ( vec3<f32>( 1.0 ) - nodeVar131 );
	nodeVar137 = nodeVar136;
	nodeVar138 = ( nodeVar137 * vec3<f32>( 0.047619 ) );
	nodeVar139 = ( nodeVar131 + nodeVar138 );
	nodeVar140 = ( nodeVar134 * nodeVar139 );
	nodeVar141 = ( dfg.x + dfg.y );
	nodeVar142 = ( 1.0 - nodeVar141 );
	nodeVar143 = nodeVar142;
	nodeVar144 = ( vec3<f32>( nodeVar143 ) * nodeVar139 );
	nodeVar145 = ( vec3<f32>( 1.0 ) - nodeVar144 );
	nodeVar146 = nodeVar145;
	nodeVar147 = ( nodeVar140 / nodeVar146 );
	nodeVar148 = ( nodeVar147 * vec3<f32>( nodeVar143 ) );
	nodeVar149 = ( multiScatteringMetallic + nodeVar148 );
	multiScatteringMetallic = nodeVar149;
	nodeVar150 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar151 = ( radiance * nodeVar150 );
	nodeVar152 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar153 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar154 = ( nodeVar152 * nodeVar153 );
	nodeVar155 = ( nodeVar151 + nodeVar154 );
	nodeVar156 = nodeVar155;
	nodeVar157 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar158 = ( vec3<f32>( 1.0 ) - nodeVar157 );
	nodeVar159 = nodeVar158;
	nodeVar160 = ( DiffuseContribution * nodeVar159 );
	nodeVar161 = ( nodeVar160 * nodeVar153 );
	nodeVar162 = nodeVar161;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar163 = ( indirectSpecular + nodeVar156 );
	indirectSpecular = nodeVar163;
	nodeVar164 = ( indirectDiffuse + nodeVar162 );
	indirectDiffuse = nodeVar164;
	ambientOcclusion = 1.0;
	nodeVar165 = ( clearcoatSpecularIndirect * vec3<f32>( ambientOcclusion ) );
	clearcoatSpecularIndirect = nodeVar165;
	nodeVar166 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar166;
	nodeVar167 = dot( normalView, positionViewDirection );
	nodeVar168 = ( clamp( nodeVar167, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar169 = ( Roughness * -16.0 );
	nodeVar170 = ( 1.0 - nodeVar169 );
	nodeVar171 = nodeVar170;
	nodeVar172 = ( - nodeVar171 );
	nodeVar173 = exp2( nodeVar172 );
	nodeVar174 = pow( nodeVar168, nodeVar173 );
	nodeVar175 = ( 1.0 - nodeVar174 );
	nodeVar176 = nodeVar175;
	nodeVar177 = ( ambientOcclusion - nodeVar176 );
	nodeVar178 = ( indirectSpecular * vec3<f32>( clamp( nodeVar177, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar178;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar179 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar179;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar180 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar180;
	nodeVar181 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar181;
	nodeVar182 = dot( clearcoatNormalView, positionViewDirection );
	nodeVar183 = clamp( nodeVar182, 0.0, 1.0 );
	nodeVar184 = exp2( ( ( ( nodeVar183 * -5.55473 ) - 6.98316 ) * nodeVar183 ) );
	nodeVar185 = ( ( vec3<f32>( 0.04, 0.04, 0.04 ) * vec3<f32>( ( 1.0 - nodeVar184 ) ) ) + vec3<f32>( ( 1.0 * nodeVar184 ) ) );
	nodeVar186 = ( vec3<f32>( Clearcoat ) * nodeVar185 );
	nodeVar187 = ( vec3<f32>( 1.0 ) - nodeVar186 );
	nodeVar188 = nodeVar187;
	nodeVar189 = ( outgoingLight * nodeVar188 );
	nodeVar190 = ( clearcoatSpecularDirect + clearcoatSpecularIndirect );
	nodeVar191 = ( nodeVar190 * vec3<f32>( Clearcoat ) );
	nodeVar192 = ( nodeVar189 + nodeVar191 );
	outgoingLight = nodeVar192;
	nodeVar193 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar193;

	// result

	output.color = nodeVar193;

	return output;

}
