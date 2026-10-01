// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform13_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform13 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform22_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform22 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : vec3<f32>,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform8 : vec3<f32>,
	nodeUniform9 : f32,
	nodeUniform10 : f32,
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform15 : mat3x3<f32>,
	nodeUniform16 : mat4x4<f32>,
	nodeUniform17 : f32,
	nodeUniform18 : mat4x4<f32>,
	nodeUniform20 : f32,
	nodeUniform21 : f32,
	nodeUniform23 : f32
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
var<private> IOR : f32;
var<private> SpecularColor : vec3<f32>;
var<private> nodeVar4 : f32;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> Sheen : vec3<f32>;
var<private> SheenRoughness : f32;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> sheenSpecularDirect : vec3<f32>;
var<private> sheenSpecularIndirect : vec3<f32>;
var<private> normalView : vec3<f32>;
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
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec2<f32>;
var<private> nodeVar21 : vec4<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec2<f32>;
var<private> nodeVar29 : vec4<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec2<f32>;
var<private> nodeVar39 : vec4<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : vec2<f32>;
var<private> nodeVar47 : vec4<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : f32;
var<private> nodeVar122 : f32;
var<private> nodeVar123 : f32;
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
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : f32;
var<private> nodeVar158 : f32;
var<private> nodeVar159 : f32;
var<private> nodeVar160 : f32;
var<private> nodeVar161 : f32;
var<private> nodeVar162 : f32;
var<private> nodeVar163 : f32;
var<private> nodeVar164 : f32;
var<private> nodeVar165 : f32;
var<private> nodeVar166 : f32;
var<private> nodeVar167 : f32;
var<private> nodeVar168 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : vec4<f32>;

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




@fragment
fn main( @location( 0 ) nodeVarying3 : vec3<f32>,
	@location( 1 ) v_positionViewDirection : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform1, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform2 );
	Metalness = object.nodeUniform3;
	Roughness = min( ( max( object.nodeUniform4, 0.0525 ) + 0.0 ), 1.0 );
	IOR = object.nodeUniform5;
	nodeVar4 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar4 * nodeVar4 ) ) * object.nodeUniform6 ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform7 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform7, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform3 ) ) );
	Sheen = ( object.nodeUniform8 * vec3<f32>( object.nodeUniform9 ) );
	SheenRoughness = clamp( object.nodeUniform10, 0.0001, 1.0 );
	EmissiveColor = ( object.nodeUniform11 * vec3<f32>( object.nodeUniform12 ) );
	sheenSpecularDirect = vec3<f32>( 0.0, 0.0, 0.0 );
	sheenSpecularIndirect = vec3<f32>( 0.0, 0.0, 0.0 );
	normalView = nodeVarying3;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar5 = dot( normalView, positionViewDirection );
	nodeVar6 = textureSample( nodeUniform13, nodeUniform13_sampler, vec2<f32>( Roughness, clamp( nodeVar5, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar6;
	nodeVar7 = ( dfg.x + dfg.y );
	nodeVar8 = ( 1.0 / nodeVar7 );
	nodeVar9 = nodeVar8;
	nodeVar10 = ( nodeVar9 - 1.0 );
	nodeVar11 = ( SpecularColorBlended * vec3<f32>( nodeVar10 ) );
	nodeVar12 = ( nodeVar11 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar12;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar13 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform17 );
	nodeVar14 = floor( nodeVar13 );
	nodeVar15 = nodeVar14;
	nodeVar16 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar17 = getFace( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( nodeVar16.x, ( - nodeVar16.y ), nodeVar16.z ), 1.0 ) ).xyz );
	nodeVar18 = max( ( 4.0 - nodeVar15 ), 0.0 );
	nodeVar15 = max( nodeVar15, 4.0 );
	nodeVar19 = exp2( nodeVar15 );
	nodeVar20 = ( ( getUV( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( nodeVar16.x, ( - nodeVar16.y ), nodeVar16.z ), 1.0 ) ).xyz, nodeVar17 ) * vec2<f32>( ( nodeVar19 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar17 > 2.0 ) ) {

		nodeVar20.y = ( nodeVar20.y + nodeVar19 );
		nodeVar17 = ( nodeVar17 - 3.0 );
		

	}

	nodeVar20.x = ( nodeVar20.x + ( nodeVar17 * nodeVar19 ) );
	nodeVar20.x = ( nodeVar20.x + ( nodeVar18 * ( 3.0 * 16.0 ) ) );
	nodeVar20.y = ( nodeVar20.y + ( 4.0 * ( exp2( object.nodeUniform17 ) - nodeVar19 ) ) );
	nodeVar20.x = ( nodeVar20.x * object.nodeUniform20 );
	nodeVar20.y = ( nodeVar20.y * object.nodeUniform21 );
	nodeVar21 = textureSampleGrad( nodeUniform22, nodeUniform22_sampler, nodeVar20, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar22 = nodeVar21.xyz;
	nodeVar23 = fract( nodeVar13 );

	if ( ( nodeVar23 != 0.0 ) ) {

		nodeVar24 = ( nodeVar14 + 1.0 );
		nodeVar25 = getFace( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( nodeVar16.x, ( - nodeVar16.y ), nodeVar16.z ), 1.0 ) ).xyz );
		nodeVar26 = max( ( 4.0 - nodeVar24 ), 0.0 );
		nodeVar24 = max( nodeVar24, 4.0 );
		nodeVar27 = exp2( nodeVar24 );
		nodeVar28 = ( ( getUV( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( nodeVar16.x, ( - nodeVar16.y ), nodeVar16.z ), 1.0 ) ).xyz, nodeVar25 ) * vec2<f32>( ( nodeVar27 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar25 > 2.0 ) ) {

			nodeVar28.y = ( nodeVar28.y + nodeVar27 );
			nodeVar25 = ( nodeVar25 - 3.0 );
			

		}

		nodeVar28.x = ( nodeVar28.x + ( nodeVar25 * nodeVar27 ) );
		nodeVar28.x = ( nodeVar28.x + ( nodeVar26 * ( 3.0 * 16.0 ) ) );
		nodeVar28.y = ( nodeVar28.y + ( 4.0 * ( exp2( object.nodeUniform17 ) - nodeVar27 ) ) );
		nodeVar28.x = ( nodeVar28.x * object.nodeUniform20 );
		nodeVar28.y = ( nodeVar28.y * object.nodeUniform21 );
		nodeVar29 = textureSampleGrad( nodeUniform22, nodeUniform22_sampler, nodeVar28, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar30 = nodeVar29.xyz;
		nodeVar22 = mix( nodeVar22, nodeVar30, nodeVar23 );
		

	}

	nodeVar31 = ( radiance + ( nodeVar22 * vec3<f32>( object.nodeUniform23 ) ) );
	radiance = nodeVar31;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar32 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform17 );
	nodeVar33 = floor( nodeVar32 );
	nodeVar34 = nodeVar33;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar35 = getFace( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar36 = max( ( 4.0 - nodeVar34 ), 0.0 );
	nodeVar34 = max( nodeVar34, 4.0 );
	nodeVar37 = exp2( nodeVar34 );
	nodeVar38 = ( ( getUV( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar35 ) * vec2<f32>( ( nodeVar37 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar35 > 2.0 ) ) {

		nodeVar38.y = ( nodeVar38.y + nodeVar37 );
		nodeVar35 = ( nodeVar35 - 3.0 );
		

	}

	nodeVar38.x = ( nodeVar38.x + ( nodeVar35 * nodeVar37 ) );
	nodeVar38.x = ( nodeVar38.x + ( nodeVar36 * ( 3.0 * 16.0 ) ) );
	nodeVar38.y = ( nodeVar38.y + ( 4.0 * ( exp2( object.nodeUniform17 ) - nodeVar37 ) ) );
	nodeVar38.x = ( nodeVar38.x * object.nodeUniform20 );
	nodeVar38.y = ( nodeVar38.y * object.nodeUniform21 );
	nodeVar39 = textureSampleGrad( nodeUniform22, nodeUniform22_sampler, nodeVar38, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar40 = nodeVar39.xyz;
	nodeVar41 = fract( nodeVar32 );

	if ( ( nodeVar41 != 0.0 ) ) {

		nodeVar42 = ( nodeVar33 + 1.0 );
		nodeVar43 = getFace( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar44 = max( ( 4.0 - nodeVar42 ), 0.0 );
		nodeVar42 = max( nodeVar42, 4.0 );
		nodeVar45 = exp2( nodeVar42 );
		nodeVar46 = ( ( getUV( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar43 ) * vec2<f32>( ( nodeVar45 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar43 > 2.0 ) ) {

			nodeVar46.y = ( nodeVar46.y + nodeVar45 );
			nodeVar43 = ( nodeVar43 - 3.0 );
			

		}

		nodeVar46.x = ( nodeVar46.x + ( nodeVar43 * nodeVar45 ) );
		nodeVar46.x = ( nodeVar46.x + ( nodeVar44 * ( 3.0 * 16.0 ) ) );
		nodeVar46.y = ( nodeVar46.y + ( 4.0 * ( exp2( object.nodeUniform17 ) - nodeVar45 ) ) );
		nodeVar46.x = ( nodeVar46.x * object.nodeUniform20 );
		nodeVar46.y = ( nodeVar46.y * object.nodeUniform21 );
		nodeVar47 = textureSampleGrad( nodeUniform22, nodeUniform22_sampler, nodeVar46, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar48 = nodeVar47.xyz;
		nodeVar40 = mix( nodeVar40, nodeVar48, nodeVar41 );
		

	}

	nodeVar49 = ( iblIrradiance + ( ( nodeVar40 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform23 ) ) );
	iblIrradiance = nodeVar49;
	nodeVar50 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar51 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar52 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar53 = ( SpecularF90 * dfg.y );
	nodeVar54 = ( nodeVar52 + vec3<f32>( nodeVar53 ) );
	nodeVar55 = ( nodeVar50 + nodeVar54 );
	nodeVar50 = nodeVar55;
	nodeVar56 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar57 = nodeVar56;
	nodeVar58 = ( nodeVar57 * vec3<f32>( 0.047619 ) );
	nodeVar59 = ( SpecularColor + nodeVar58 );
	nodeVar60 = ( nodeVar54 * nodeVar59 );
	nodeVar61 = ( dfg.x + dfg.y );
	nodeVar62 = ( 1.0 - nodeVar61 );
	nodeVar63 = nodeVar62;
	nodeVar64 = ( vec3<f32>( nodeVar63 ) * nodeVar59 );
	nodeVar65 = ( vec3<f32>( 1.0 ) - nodeVar64 );
	nodeVar66 = nodeVar65;
	nodeVar67 = ( nodeVar60 / nodeVar66 );
	nodeVar68 = ( nodeVar67 * vec3<f32>( nodeVar63 ) );
	nodeVar69 = ( nodeVar51 + nodeVar68 );
	nodeVar51 = nodeVar69;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar70 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar71 = ( irradiance * nodeVar70 );
	nodeVar72 = ( nodeVar50 + nodeVar51 );
	nodeVar73 = ( vec3<f32>( 1.0 ) - nodeVar72 );
	nodeVar74 = nodeVar73;
	nodeVar75 = ( nodeVar71 * nodeVar74 );
	nodeVar76 = nodeVar75;
	nodeVar77 = ( SheenRoughness * SheenRoughness );
	nodeVar78 = ( 1.0 / ( SheenRoughness + 0.1 ) );
	nodeVar79 = clamp( exp( ( ( ( ( ( -1.9362 + ( SheenRoughness * 1.0678 ) ) + ( nodeVar77 * 0.4573 ) ) - ( nodeVar78 * 0.8469 ) ) * clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) + ( ( ( -0.6014 + ( SheenRoughness * 0.5538 ) ) - ( nodeVar77 * 0.467 ) ) - ( nodeVar78 * 0.1255 ) ) ) ), 0.0, 1.0 );
	nodeVar80 = ( ( ( irradiance * Sheen ) * vec3<f32>( nodeVar79 ) ) * vec3<f32>( 0.3183098861837907 ) );
	nodeVar81 = ( sheenSpecularIndirect + nodeVar80 );
	sheenSpecularIndirect = nodeVar81;
	nodeVar82 = max( Sheen.x, Sheen.y );
	nodeVar83 = max( nodeVar82, Sheen.z );
	nodeVar84 = ( nodeVar83 * nodeVar79 );
	nodeVar85 = ( 1.0 - nodeVar84 );
	nodeVar86 = nodeVar85;
	nodeVar87 = ( nodeVar76 * vec3<f32>( nodeVar86 ) );
	nodeVar76 = nodeVar87;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar88 = ( indirectDiffuse + nodeVar76 );
	indirectDiffuse = nodeVar88;
	nodeVar89 = ( SheenRoughness * SheenRoughness );
	nodeVar90 = ( 1.0 / ( SheenRoughness + 0.1 ) );
	nodeVar91 = clamp( exp( ( ( ( ( ( -1.9362 + ( SheenRoughness * 1.0678 ) ) + ( nodeVar89 * 0.4573 ) ) - ( nodeVar90 * 0.8469 ) ) * clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) + ( ( ( -0.6014 + ( SheenRoughness * 0.5538 ) ) - ( nodeVar89 * 0.467 ) ) - ( nodeVar90 * 0.1255 ) ) ) ), 0.0, 1.0 );
	nodeVar92 = ( ( ( iblIrradiance * Sheen ) * vec3<f32>( nodeVar91 ) ) * vec3<f32>( 0.3183098861837907 ) );
	nodeVar93 = ( sheenSpecularIndirect + nodeVar92 );
	sheenSpecularIndirect = nodeVar93;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar94 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar95 = ( SpecularF90 * dfg.y );
	nodeVar96 = ( nodeVar94 + vec3<f32>( nodeVar95 ) );
	nodeVar97 = ( singleScatteringDielectric + nodeVar96 );
	singleScatteringDielectric = nodeVar97;
	nodeVar98 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar99 = nodeVar98;
	nodeVar100 = ( nodeVar99 * vec3<f32>( 0.047619 ) );
	nodeVar101 = ( SpecularColor + nodeVar100 );
	nodeVar102 = ( nodeVar96 * nodeVar101 );
	nodeVar103 = ( dfg.x + dfg.y );
	nodeVar104 = ( 1.0 - nodeVar103 );
	nodeVar105 = nodeVar104;
	nodeVar106 = ( vec3<f32>( nodeVar105 ) * nodeVar101 );
	nodeVar107 = ( vec3<f32>( 1.0 ) - nodeVar106 );
	nodeVar108 = nodeVar107;
	nodeVar109 = ( nodeVar102 / nodeVar108 );
	nodeVar110 = ( nodeVar109 * vec3<f32>( nodeVar105 ) );
	nodeVar111 = ( multiScatteringDielectric + nodeVar110 );
	multiScatteringDielectric = nodeVar111;
	nodeVar112 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar113 = ( SpecularF90 * dfg.y );
	nodeVar114 = ( nodeVar112 + vec3<f32>( nodeVar113 ) );
	nodeVar115 = ( singleScatteringMetallic + nodeVar114 );
	singleScatteringMetallic = nodeVar115;
	nodeVar116 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar117 = nodeVar116;
	nodeVar118 = ( nodeVar117 * vec3<f32>( 0.047619 ) );
	nodeVar119 = ( DiffuseColor.xyz + nodeVar118 );
	nodeVar120 = ( nodeVar114 * nodeVar119 );
	nodeVar121 = ( dfg.x + dfg.y );
	nodeVar122 = ( 1.0 - nodeVar121 );
	nodeVar123 = nodeVar122;
	nodeVar124 = ( vec3<f32>( nodeVar123 ) * nodeVar119 );
	nodeVar125 = ( vec3<f32>( 1.0 ) - nodeVar124 );
	nodeVar126 = nodeVar125;
	nodeVar127 = ( nodeVar120 / nodeVar126 );
	nodeVar128 = ( nodeVar127 * vec3<f32>( nodeVar123 ) );
	nodeVar129 = ( multiScatteringMetallic + nodeVar128 );
	multiScatteringMetallic = nodeVar129;
	nodeVar130 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar131 = ( radiance * nodeVar130 );
	nodeVar132 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar133 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar134 = ( nodeVar132 * nodeVar133 );
	nodeVar135 = ( nodeVar131 + nodeVar134 );
	nodeVar136 = nodeVar135;
	nodeVar137 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar138 = ( vec3<f32>( 1.0 ) - nodeVar137 );
	nodeVar139 = nodeVar138;
	nodeVar140 = ( DiffuseContribution * nodeVar139 );
	nodeVar141 = ( nodeVar140 * nodeVar133 );
	nodeVar142 = nodeVar141;
	nodeVar143 = max( Sheen.x, Sheen.y );
	nodeVar144 = max( nodeVar143, Sheen.z );
	nodeVar145 = ( SheenRoughness * SheenRoughness );
	nodeVar146 = ( 1.0 / ( SheenRoughness + 0.1 ) );
	nodeVar147 = clamp( exp( ( ( ( ( ( -1.9362 + ( SheenRoughness * 1.0678 ) ) + ( nodeVar145 * 0.4573 ) ) - ( nodeVar146 * 0.8469 ) ) * clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) + ( ( ( -0.6014 + ( SheenRoughness * 0.5538 ) ) - ( nodeVar145 * 0.467 ) ) - ( nodeVar146 * 0.1255 ) ) ) ), 0.0, 1.0 );
	nodeVar148 = ( nodeVar144 * nodeVar147 );
	nodeVar149 = ( 1.0 - nodeVar148 );
	nodeVar150 = nodeVar149;
	nodeVar151 = ( nodeVar136 * vec3<f32>( nodeVar150 ) );
	nodeVar136 = nodeVar151;
	nodeVar152 = ( nodeVar142 * vec3<f32>( nodeVar150 ) );
	nodeVar142 = nodeVar152;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar153 = ( indirectSpecular + nodeVar136 );
	indirectSpecular = nodeVar153;
	nodeVar154 = ( indirectDiffuse + nodeVar142 );
	indirectDiffuse = nodeVar154;
	ambientOcclusion = 1.0;
	nodeVar155 = ( sheenSpecularIndirect * vec3<f32>( ambientOcclusion ) );
	sheenSpecularIndirect = nodeVar155;
	nodeVar156 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar156;
	nodeVar157 = dot( normalView, positionViewDirection );
	nodeVar158 = ( clamp( nodeVar157, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar159 = ( Roughness * -16.0 );
	nodeVar160 = ( 1.0 - nodeVar159 );
	nodeVar161 = nodeVar160;
	nodeVar162 = ( - nodeVar161 );
	nodeVar163 = exp2( nodeVar162 );
	nodeVar164 = pow( nodeVar158, nodeVar163 );
	nodeVar165 = ( 1.0 - nodeVar164 );
	nodeVar166 = nodeVar165;
	nodeVar167 = ( ambientOcclusion - nodeVar166 );
	nodeVar168 = ( indirectSpecular * vec3<f32>( clamp( nodeVar167, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar168;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar169 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar169;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar170 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar170;
	nodeVar171 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar171;
	nodeVar172 = ( ( outgoingLight + sheenSpecularDirect ) + sheenSpecularIndirect );
	outgoingLight = nodeVar172;
	nodeVar173 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar173;

	// result

	output.color = nodeVar173;

	return output;

}
