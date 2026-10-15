// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>,
	@builtin( frag_depth ) depth : f32
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform2_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform2 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform10_sampler : sampler;
@binding( 10 ) @group( 1 ) var nodeUniform10 : texture_2d<f32>;
@binding( 11 ) @group( 1 ) var nodeUniform11_sampler : sampler;
@binding( 12 ) @group( 1 ) var nodeUniform11 : texture_2d<f32>;
@binding( 14 ) @group( 1 ) var nodeUniform13_sampler : sampler;
@binding( 15 ) @group( 1 ) var nodeUniform13 : texture_2d<f32>;
@binding( 16 ) @group( 1 ) var nodeUniform14_sampler : sampler;
@binding( 17 ) @group( 1 ) var nodeUniform14 : texture_2d<f32>;
@binding( 18 ) @group( 1 ) var nodeUniform16_sampler : sampler;
@binding( 19 ) @group( 1 ) var nodeUniform16 : texture_2d<f32>;
@binding( 20 ) @group( 1 ) var nodeUniform22_sampler : sampler;
@binding( 21 ) @group( 1 ) var nodeUniform22 : texture_2d<f32>;

struct NodeBuffer_1005Struct {
	value : array< u32 >
};
@binding( 0 ) @group( 1 )
var<storage, read> NodeBuffer_1005 : NodeBuffer_1005Struct;

struct NodeBuffer_995Struct {
	value : array< vec2<f32> >
};
@binding( 3 ) @group( 1 )
var<storage, read> NodeBuffer_995 : NodeBuffer_995Struct;

struct NodeBuffer_996Struct {
	value : array< u32 >
};
@binding( 4 ) @group( 1 )
var<storage, read> NodeBuffer_996 : NodeBuffer_996Struct;

struct NodeBuffer_1012Struct {
	value : array< mat4x4<f32> >
};
@binding( 6 ) @group( 1 )
var<storage, read> NodeBuffer_1012 : NodeBuffer_1012Struct;

struct NodeBuffer_1007Struct {
	value : array< u32 >
};
@binding( 7 ) @group( 1 )
var<storage, read> NodeBuffer_1007 : NodeBuffer_1007Struct;

struct NodeBuffer_993Struct {
	value : array< vec4<f32> >
};
@binding( 8 ) @group( 1 )
var<storage, read> NodeBuffer_993 : NodeBuffer_993Struct;

struct NodeBuffer_994Struct {
	value : array< vec4<f32> >
};
@binding( 13 ) @group( 1 )
var<storage, read> NodeBuffer_994 : NodeBuffer_994Struct;

struct renderStruct {
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform1 : vec2<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform5 : mat4x4<f32>,
	nodeUniform9 : f32,
	nodeUniform17 : f32,
	nodeUniform18 : mat4x4<f32>,
	nodeUniform20 : f32,
	nodeUniform21 : f32,
	nodeUniform23 : f32
};
@binding( 5 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : f32;
var<private> nodeVar1 : u32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar4 : u32;
var<private> nodeVar5 : vec2<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec2<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : vec2<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec2<f32>;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : vec2<f32>;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : vec4<f32>;
var<private> AmbientOcclusion : f32;
var<private> nodeVar38 : vec4<f32>;
var<private> Metalness : f32;
var<private> nodeVar39 : vec4<f32>;
var<private> Roughness : f32;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> nodeVar43 : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar44 : vec2<f32>;
var<private> nodeVar45 : vec2<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec4<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec2<f32>;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : vec2<f32>;
var<private> nodeVar66 : vec4<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : vec2<f32>;
var<private> nodeVar74 : vec4<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : vec2<f32>;
var<private> nodeVar84 : vec4<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : vec2<f32>;
var<private> nodeVar92 : vec4<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : f32;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : f32;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : f32;
var<private> nodeVar177 : f32;
var<private> nodeVar178 : f32;
var<private> nodeVar179 : f32;
var<private> nodeVar180 : f32;
var<private> nodeVar181 : f32;
var<private> nodeVar182 : f32;
var<private> nodeVar183 : f32;
var<private> nodeVar184 : f32;
var<private> nodeVar185 : f32;
var<private> nodeVar186 : f32;
var<private> nodeVar187 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar188 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar189 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : vec4<f32>;

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
fn main( @builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = ( render.nodeUniform1.y - fragCoord.xy.y );
	nodeVar1 = ( ( u32( nodeVar0 ) * u32( render.nodeUniform1.x ) ) + u32( fragCoord.xy.x ) );
	nodeVar2 = ( f32( ( NodeBuffer_1005.value[ nodeVar1 ] >> 16u ) ) / 65535.0 );
	nodeVar3 = ( nodeVar2 * nodeVar2 );
	output.depth = ( 1.0 - ( nodeVar3 * nodeVar3 ) );

	if ( ( f32( ( NodeBuffer_1005.value[ nodeVar1 ] >> 16u ) ) == 0.0 ) ) {

		discard;
		

	}

	nodeVar4 = ( NodeBuffer_1005.value[ nodeVar1 ] & 65535u );
	nodeVar5 = vec2<f32>( fragCoord.xy.x, nodeVar0 );
	nodeVar6 = ( object.nodeUniform5 * vec4<f32>( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 1u ) ] ] ).xyz, 1.0 ) );
	nodeVar7 = ( ( ( ( nodeVar6.xyz / vec3<f32>( nodeVar6.w ) ).xy + vec2<f32>( 1.0 ) ) * vec2<f32>( 0.5 ) ) * vec2<f32>( render.nodeUniform1.x, render.nodeUniform1.y ) );
	nodeVar8 = ( object.nodeUniform5 * vec4<f32>( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 2u ) ] ] ).xyz, 1.0 ) );
	nodeVar9 = ( ( ( ( nodeVar8.xyz / vec3<f32>( nodeVar8.w ) ).xy + vec2<f32>( 1.0 ) ) * vec2<f32>( 0.5 ) ) * vec2<f32>( render.nodeUniform1.x, render.nodeUniform1.y ) );
	nodeVar10 = ( ( ( nodeVar5.y - nodeVar7.y ) * ( nodeVar9.x - nodeVar7.x ) ) - ( ( nodeVar5.x - nodeVar7.x ) * ( nodeVar9.y - nodeVar7.y ) ) );
	nodeVar12 = ( object.nodeUniform5 * vec4<f32>( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 0u ) ] ] ).xyz, 1.0 ) );
	nodeVar13 = ( ( ( ( nodeVar12.xyz / vec3<f32>( nodeVar12.w ) ).xy + vec2<f32>( 1.0 ) ) * vec2<f32>( 0.5 ) ) * vec2<f32>( render.nodeUniform1.x, render.nodeUniform1.y ) );
	nodeVar14 = ( ( ( nodeVar9.y - nodeVar13.y ) * ( nodeVar7.x - nodeVar13.x ) ) - ( ( nodeVar9.x - nodeVar13.x ) * ( nodeVar7.y - nodeVar13.y ) ) );

	if ( ( nodeVar14 == 0.0 ) ) {

		nodeVar11 = 1.0;

	} else {

		nodeVar11 = nodeVar14;

	}

	nodeVar15 = ( nodeVar10 / nodeVar11 );
	nodeVar17 = ( ( ( nodeVar5.y - nodeVar9.y ) * ( nodeVar13.x - nodeVar9.x ) ) - ( ( nodeVar5.x - nodeVar9.x ) * ( nodeVar13.y - nodeVar9.y ) ) );
	nodeVar18 = ( nodeVar17 / nodeVar11 );
	nodeVar19 = ( ( ( nodeVar5.y - nodeVar13.y ) * ( nodeVar7.x - nodeVar13.x ) ) - ( ( nodeVar5.x - nodeVar13.x ) * ( nodeVar7.y - nodeVar13.y ) ) );
	nodeVar20 = ( nodeVar19 / nodeVar11 );
	nodeVar21 = ( ( ( nodeVar15 / nodeVar12.w ) + ( nodeVar18 / nodeVar6.w ) ) + ( nodeVar20 / nodeVar8.w ) );

	if ( ( nodeVar21 == 0.0 ) ) {

		nodeVar16 = 1.0;

	} else {

		nodeVar16 = nodeVar21;

	}

	nodeVar22 = ( ( nodeVar15 / nodeVar12.w ) / nodeVar16 );
	nodeVar23 = ( ( nodeVar18 / nodeVar6.w ) / nodeVar16 );
	nodeVar24 = ( ( nodeVar20 / nodeVar8.w ) / nodeVar16 );
	nodeVar25 = ( ( ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 0u ) ] ] * vec2<f32>( nodeVar22 ) ) + ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 1u ) ] ] * vec2<f32>( nodeVar23 ) ) ) + ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 2u ) ] ] * vec2<f32>( nodeVar24 ) ) );
	nodeVar26 = ( nodeVar9.y - nodeVar7.y );
	nodeVar27 = ( 1.0 / nodeVar12.w );
	nodeVar28 = ( nodeVar13.y - nodeVar9.y );
	nodeVar29 = ( 1.0 / nodeVar6.w );
	nodeVar30 = ( nodeVar7.y - nodeVar13.y );
	nodeVar31 = ( 1.0 / nodeVar8.w );
	nodeVar33 = ( ( ( nodeVar10 * nodeVar27 ) + ( nodeVar17 * nodeVar29 ) ) + ( nodeVar19 * nodeVar31 ) );

	if ( ( nodeVar33 == 0.0 ) ) {

		nodeVar32 = 1.0;

	} else {

		nodeVar32 = nodeVar33;

	}

	nodeVar34 = ( nodeVar7.x - nodeVar9.x );
	nodeVar35 = ( nodeVar9.x - nodeVar13.x );
	nodeVar36 = ( nodeVar13.x - nodeVar7.x );
	nodeVar37 = textureSampleGrad( nodeUniform2, nodeUniform2_sampler, nodeVar25, ( ( ( ( vec2<f32>( ( nodeVar26 * nodeVar27 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 0u ) ] ] - nodeVar25 ) ) + ( vec2<f32>( ( nodeVar28 * nodeVar29 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 1u ) ] ] - nodeVar25 ) ) ) + ( vec2<f32>( ( nodeVar30 * nodeVar31 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 2u ) ] ] - nodeVar25 ) ) ) / vec2<f32>( nodeVar32 ) ), ( ( ( ( vec2<f32>( ( nodeVar34 * nodeVar27 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 0u ) ] ] - nodeVar25 ) ) + ( vec2<f32>( ( nodeVar35 * nodeVar29 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 1u ) ] ] - nodeVar25 ) ) ) + ( vec2<f32>( ( nodeVar36 * nodeVar31 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 2u ) ] ] - nodeVar25 ) ) ) / nodeVar32 ) );
	DiffuseColor = nodeVar37;
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform9 );
	DiffuseColor.w = 1.0;
	nodeVar38 = textureSampleGrad( nodeUniform10, nodeUniform10_sampler, nodeVar25, ( ( ( ( vec2<f32>( ( nodeVar26 * nodeVar27 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 0u ) ] ] - nodeVar25 ) ) + ( vec2<f32>( ( nodeVar28 * nodeVar29 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 1u ) ] ] - nodeVar25 ) ) ) + ( vec2<f32>( ( nodeVar30 * nodeVar31 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 2u ) ] ] - nodeVar25 ) ) ) / nodeVar32 ), ( ( ( ( vec2<f32>( ( nodeVar34 * nodeVar27 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 0u ) ] ] - nodeVar25 ) ) + ( vec2<f32>( ( nodeVar35 * nodeVar29 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 1u ) ] ] - nodeVar25 ) ) ) + ( vec2<f32>( ( nodeVar36 * nodeVar31 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 2u ) ] ] - nodeVar25 ) ) ) / nodeVar32 ) );
	AmbientOcclusion = nodeVar38.x;
	nodeVar39 = textureSampleGrad( nodeUniform11, nodeUniform11_sampler, nodeVar25, ( ( ( ( vec2<f32>( ( nodeVar26 * nodeVar27 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 0u ) ] ] - nodeVar25 ) ) + ( vec2<f32>( ( nodeVar28 * nodeVar29 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 1u ) ] ] - nodeVar25 ) ) ) + ( vec2<f32>( ( nodeVar30 * nodeVar31 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 2u ) ] ] - nodeVar25 ) ) ) / nodeVar32 ), ( ( ( ( vec2<f32>( ( nodeVar34 * nodeVar27 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 0u ) ] ] - nodeVar25 ) ) + ( vec2<f32>( ( nodeVar35 * nodeVar29 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 1u ) ] ] - nodeVar25 ) ) ) + ( vec2<f32>( ( nodeVar36 * nodeVar31 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 2u ) ] ] - nodeVar25 ) ) ) / nodeVar32 ) );
	Metalness = nodeVar39.z;
	nodeVar40 = normalize( ( ( ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * vec4<f32>( NodeBuffer_994.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 0u ) ] ].xyz, 0.0 ) ).xyz * vec3<f32>( nodeVar22 ) ) + ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * vec4<f32>( NodeBuffer_994.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 1u ) ] ].xyz, 0.0 ) ).xyz * vec3<f32>( nodeVar23 ) ) ) + ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * vec4<f32>( NodeBuffer_994.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 2u ) ] ].xyz, 0.0 ) ).xyz * vec3<f32>( nodeVar24 ) ) ) );
	nodeVar41 = ( ( ( ( vec3<f32>( ( nodeVar26 * nodeVar27 ) ) * ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * vec4<f32>( NodeBuffer_994.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 0u ) ] ].xyz, 0.0 ) ).xyz - nodeVar40 ) ) + ( vec3<f32>( ( nodeVar28 * nodeVar29 ) ) * ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * vec4<f32>( NodeBuffer_994.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 1u ) ] ].xyz, 0.0 ) ).xyz - nodeVar40 ) ) ) + ( vec3<f32>( ( nodeVar30 * nodeVar31 ) ) * ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * vec4<f32>( NodeBuffer_994.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 2u ) ] ].xyz, 0.0 ) ).xyz - nodeVar40 ) ) ) / nodeVar32 );
	nodeVar42 = ( ( ( ( vec3<f32>( ( nodeVar34 * nodeVar27 ) ) * ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * vec4<f32>( NodeBuffer_994.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 0u ) ] ].xyz, 0.0 ) ).xyz - nodeVar40 ) ) + ( vec3<f32>( ( nodeVar35 * nodeVar29 ) ) * ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * vec4<f32>( NodeBuffer_994.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 1u ) ] ].xyz, 0.0 ) ).xyz - nodeVar40 ) ) ) + ( vec3<f32>( ( nodeVar36 * nodeVar31 ) ) * ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * vec4<f32>( NodeBuffer_994.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 2u ) ] ].xyz, 0.0 ) ).xyz - nodeVar40 ) ) ) / nodeVar32 );
	Roughness = min( ( max( sqrt( ( ( nodeVar39.y * nodeVar39.y ) + min( ( ( dot( nodeVar41, nodeVar41 ) + dot( nodeVar42, nodeVar42 ) ) * 2.0 ), 0.2 ) ) ), 0.0525 ) + 0.0 ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - nodeVar39.z ) ) );
	nodeVar43 = textureSampleGrad( nodeUniform13, nodeUniform13_sampler, nodeVar25, ( ( ( ( vec2<f32>( ( nodeVar26 * nodeVar27 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 0u ) ] ] - nodeVar25 ) ) + ( vec2<f32>( ( nodeVar28 * nodeVar29 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 1u ) ] ] - nodeVar25 ) ) ) + ( vec2<f32>( ( nodeVar30 * nodeVar31 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 2u ) ] ] - nodeVar25 ) ) ) / nodeVar32 ), ( ( ( ( vec2<f32>( ( nodeVar34 * nodeVar27 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 0u ) ] ] - nodeVar25 ) ) + ( vec2<f32>( ( nodeVar35 * nodeVar29 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 1u ) ] ] - nodeVar25 ) ) ) + ( vec2<f32>( ( nodeVar36 * nodeVar31 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 2u ) ] ] - nodeVar25 ) ) ) / nodeVar32 ) );
	EmissiveColor = nodeVar43.xyz;
	nodeVar44 = ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 2u ) ] ] - NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 0u ) ] ] );
	nodeVar45 = ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 1u ) ] ] - NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 0u ) ] ] );
	nodeVar46 = ( ( ( ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 1u ) ] ] ).xyz - ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 0u ) ] ] ).xyz ) * vec3<f32>( nodeVar44.y ) ) - ( ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 2u ) ] ] ).xyz - ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 0u ) ] ] ).xyz ) * vec3<f32>( nodeVar45.y ) ) ) * vec3<f32>( sign( ( ( nodeVar45.x * nodeVar44.y ) - ( nodeVar45.y * nodeVar44.x ) ) ) ) );
	nodeVar47 = normalize( ( nodeVar46 - ( nodeVar40 * vec3<f32>( dot( nodeVar40, nodeVar46 ) ) ) ) );
	nodeVar48 = textureSampleGrad( nodeUniform16, nodeUniform16_sampler, nodeVar25, ( ( ( ( vec2<f32>( ( nodeVar26 * nodeVar27 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 0u ) ] ] - nodeVar25 ) ) + ( vec2<f32>( ( nodeVar28 * nodeVar29 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 1u ) ] ] - nodeVar25 ) ) ) + ( vec2<f32>( ( nodeVar30 * nodeVar31 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 2u ) ] ] - nodeVar25 ) ) ) / nodeVar32 ), ( ( ( ( vec2<f32>( ( nodeVar34 * nodeVar27 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 0u ) ] ] - nodeVar25 ) ) + ( vec2<f32>( ( nodeVar35 * nodeVar29 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 1u ) ] ] - nodeVar25 ) ) ) + ( vec2<f32>( ( nodeVar36 * nodeVar31 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 2u ) ] ] - nodeVar25 ) ) ) / nodeVar32 ) );
	nodeVar49 = ( ( nodeVar48.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) );
	normalView = normalize( ( render.cameraViewMatrix * vec4<f32>( normalize( ( ( ( nodeVar47 * vec3<f32>( nodeVar49.x ) ) + ( cross( nodeVar40, nodeVar47 ) * vec3<f32>( nodeVar49.y ) ) ) + ( nodeVar40 * vec3<f32>( nodeVar49.z ) ) ) ), 0.0 ) ).xyz );
	nodeVar50 = dot( normalView, normalize( ( - ( render.cameraViewMatrix * vec4<f32>( ( ( ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 0u ) ] ] ).xyz * vec3<f32>( nodeVar22 ) ) + ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 1u ) ] ] ).xyz * vec3<f32>( nodeVar23 ) ) ) + ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 2u ) ] ] ).xyz * vec3<f32>( nodeVar24 ) ) ), 1.0 ) ).xyz ) ) );
	nodeVar51 = textureSample( nodeUniform14, nodeUniform14_sampler, vec2<f32>( Roughness, clamp( nodeVar50, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar51;
	nodeVar52 = ( dfg.x + dfg.y );
	nodeVar53 = ( 1.0 / nodeVar52 );
	nodeVar54 = nodeVar53;
	nodeVar55 = ( nodeVar54 - 1.0 );
	nodeVar56 = ( SpecularColorBlended * vec3<f32>( nodeVar55 ) );
	nodeVar57 = ( nodeVar56 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar57;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar58 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform17 );
	nodeVar59 = floor( nodeVar58 );
	nodeVar60 = nodeVar59;
	nodeVar61 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - normalize( ( - ( render.cameraViewMatrix * vec4<f32>( ( ( ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 0u ) ] ] ).xyz * vec3<f32>( nodeVar22 ) ) + ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 1u ) ] ] ).xyz * vec3<f32>( nodeVar23 ) ) ) + ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 2u ) ] ] ).xyz * vec3<f32>( nodeVar24 ) ) ), 1.0 ) ).xyz ) ) ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar62 = getFace( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( nodeVar61.x, ( - nodeVar61.y ), nodeVar61.z ), 1.0 ) ).xyz );
	nodeVar63 = max( ( 4.0 - nodeVar60 ), 0.0 );
	nodeVar60 = max( nodeVar60, 4.0 );
	nodeVar64 = exp2( nodeVar60 );
	nodeVar65 = ( ( getUV( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( nodeVar61.x, ( - nodeVar61.y ), nodeVar61.z ), 1.0 ) ).xyz, nodeVar62 ) * vec2<f32>( ( nodeVar64 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar62 > 2.0 ) ) {

		nodeVar65.y = ( nodeVar65.y + nodeVar64 );
		nodeVar62 = ( nodeVar62 - 3.0 );
		

	}

	nodeVar65.x = ( nodeVar65.x + ( nodeVar62 * nodeVar64 ) );
	nodeVar65.x = ( nodeVar65.x + ( nodeVar63 * ( 3.0 * 16.0 ) ) );
	nodeVar65.y = ( nodeVar65.y + ( 4.0 * ( exp2( object.nodeUniform17 ) - nodeVar64 ) ) );
	nodeVar65.x = ( nodeVar65.x * object.nodeUniform20 );
	nodeVar65.y = ( nodeVar65.y * object.nodeUniform21 );
	nodeVar66 = textureSampleGrad( nodeUniform22, nodeUniform22_sampler, nodeVar65, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar67 = nodeVar66.xyz;
	nodeVar68 = fract( nodeVar58 );

	if ( ( nodeVar68 != 0.0 ) ) {

		nodeVar69 = ( nodeVar59 + 1.0 );
		nodeVar70 = getFace( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( nodeVar61.x, ( - nodeVar61.y ), nodeVar61.z ), 1.0 ) ).xyz );
		nodeVar71 = max( ( 4.0 - nodeVar69 ), 0.0 );
		nodeVar69 = max( nodeVar69, 4.0 );
		nodeVar72 = exp2( nodeVar69 );
		nodeVar73 = ( ( getUV( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( nodeVar61.x, ( - nodeVar61.y ), nodeVar61.z ), 1.0 ) ).xyz, nodeVar70 ) * vec2<f32>( ( nodeVar72 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar70 > 2.0 ) ) {

			nodeVar73.y = ( nodeVar73.y + nodeVar72 );
			nodeVar70 = ( nodeVar70 - 3.0 );
			

		}

		nodeVar73.x = ( nodeVar73.x + ( nodeVar70 * nodeVar72 ) );
		nodeVar73.x = ( nodeVar73.x + ( nodeVar71 * ( 3.0 * 16.0 ) ) );
		nodeVar73.y = ( nodeVar73.y + ( 4.0 * ( exp2( object.nodeUniform17 ) - nodeVar72 ) ) );
		nodeVar73.x = ( nodeVar73.x * object.nodeUniform20 );
		nodeVar73.y = ( nodeVar73.y * object.nodeUniform21 );
		nodeVar74 = textureSampleGrad( nodeUniform22, nodeUniform22_sampler, nodeVar73, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar75 = nodeVar74.xyz;
		nodeVar67 = mix( nodeVar67, nodeVar75, nodeVar68 );
		

	}

	nodeVar76 = ( radiance + ( nodeVar67 * vec3<f32>( object.nodeUniform23 ) ) );
	radiance = nodeVar76;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar77 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform17 );
	nodeVar78 = floor( nodeVar77 );
	nodeVar79 = nodeVar78;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar80 = getFace( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar81 = max( ( 4.0 - nodeVar79 ), 0.0 );
	nodeVar79 = max( nodeVar79, 4.0 );
	nodeVar82 = exp2( nodeVar79 );
	nodeVar83 = ( ( getUV( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar80 ) * vec2<f32>( ( nodeVar82 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar80 > 2.0 ) ) {

		nodeVar83.y = ( nodeVar83.y + nodeVar82 );
		nodeVar80 = ( nodeVar80 - 3.0 );
		

	}

	nodeVar83.x = ( nodeVar83.x + ( nodeVar80 * nodeVar82 ) );
	nodeVar83.x = ( nodeVar83.x + ( nodeVar81 * ( 3.0 * 16.0 ) ) );
	nodeVar83.y = ( nodeVar83.y + ( 4.0 * ( exp2( object.nodeUniform17 ) - nodeVar82 ) ) );
	nodeVar83.x = ( nodeVar83.x * object.nodeUniform20 );
	nodeVar83.y = ( nodeVar83.y * object.nodeUniform21 );
	nodeVar84 = textureSampleGrad( nodeUniform22, nodeUniform22_sampler, nodeVar83, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar85 = nodeVar84.xyz;
	nodeVar86 = fract( nodeVar77 );

	if ( ( nodeVar86 != 0.0 ) ) {

		nodeVar87 = ( nodeVar78 + 1.0 );
		nodeVar88 = getFace( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar89 = max( ( 4.0 - nodeVar87 ), 0.0 );
		nodeVar87 = max( nodeVar87, 4.0 );
		nodeVar90 = exp2( nodeVar87 );
		nodeVar91 = ( ( getUV( ( object.nodeUniform18 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar88 ) * vec2<f32>( ( nodeVar90 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar88 > 2.0 ) ) {

			nodeVar91.y = ( nodeVar91.y + nodeVar90 );
			nodeVar88 = ( nodeVar88 - 3.0 );
			

		}

		nodeVar91.x = ( nodeVar91.x + ( nodeVar88 * nodeVar90 ) );
		nodeVar91.x = ( nodeVar91.x + ( nodeVar89 * ( 3.0 * 16.0 ) ) );
		nodeVar91.y = ( nodeVar91.y + ( 4.0 * ( exp2( object.nodeUniform17 ) - nodeVar90 ) ) );
		nodeVar91.x = ( nodeVar91.x * object.nodeUniform20 );
		nodeVar91.y = ( nodeVar91.y * object.nodeUniform21 );
		nodeVar92 = textureSampleGrad( nodeUniform22, nodeUniform22_sampler, nodeVar91, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar93 = nodeVar92.xyz;
		nodeVar85 = mix( nodeVar85, nodeVar93, nodeVar86 );
		

	}

	nodeVar94 = ( iblIrradiance + ( ( nodeVar85 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform23 ) ) );
	iblIrradiance = nodeVar94;
	ambientOcclusion = 1.0;
	nodeVar95 = ( ambientOcclusion * AmbientOcclusion );
	ambientOcclusion = nodeVar95;
	nodeVar96 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar97 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar98 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar99 = ( SpecularF90 * dfg.y );
	nodeVar100 = ( nodeVar98 + vec3<f32>( nodeVar99 ) );
	nodeVar101 = ( nodeVar96 + nodeVar100 );
	nodeVar96 = nodeVar101;
	nodeVar102 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar103 = nodeVar102;
	nodeVar104 = ( nodeVar103 * vec3<f32>( 0.047619 ) );
	nodeVar105 = ( SpecularColor + nodeVar104 );
	nodeVar106 = ( nodeVar100 * nodeVar105 );
	nodeVar107 = ( dfg.x + dfg.y );
	nodeVar108 = ( 1.0 - nodeVar107 );
	nodeVar109 = nodeVar108;
	nodeVar110 = ( vec3<f32>( nodeVar109 ) * nodeVar105 );
	nodeVar111 = ( vec3<f32>( 1.0 ) - nodeVar110 );
	nodeVar112 = nodeVar111;
	nodeVar113 = ( nodeVar106 / nodeVar112 );
	nodeVar114 = ( nodeVar113 * vec3<f32>( nodeVar109 ) );
	nodeVar115 = ( nodeVar97 + nodeVar114 );
	nodeVar97 = nodeVar115;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar116 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar117 = ( irradiance * nodeVar116 );
	nodeVar118 = ( nodeVar96 + nodeVar97 );
	nodeVar119 = ( vec3<f32>( 1.0 ) - nodeVar118 );
	nodeVar120 = nodeVar119;
	nodeVar121 = ( nodeVar117 * nodeVar120 );
	nodeVar122 = nodeVar121;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar123 = ( indirectDiffuse + nodeVar122 );
	indirectDiffuse = nodeVar123;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar124 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar125 = ( SpecularF90 * dfg.y );
	nodeVar126 = ( nodeVar124 + vec3<f32>( nodeVar125 ) );
	nodeVar127 = ( singleScatteringDielectric + nodeVar126 );
	singleScatteringDielectric = nodeVar127;
	nodeVar128 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar129 = nodeVar128;
	nodeVar130 = ( nodeVar129 * vec3<f32>( 0.047619 ) );
	nodeVar131 = ( SpecularColor + nodeVar130 );
	nodeVar132 = ( nodeVar126 * nodeVar131 );
	nodeVar133 = ( dfg.x + dfg.y );
	nodeVar134 = ( 1.0 - nodeVar133 );
	nodeVar135 = nodeVar134;
	nodeVar136 = ( vec3<f32>( nodeVar135 ) * nodeVar131 );
	nodeVar137 = ( vec3<f32>( 1.0 ) - nodeVar136 );
	nodeVar138 = nodeVar137;
	nodeVar139 = ( nodeVar132 / nodeVar138 );
	nodeVar140 = ( nodeVar139 * vec3<f32>( nodeVar135 ) );
	nodeVar141 = ( multiScatteringDielectric + nodeVar140 );
	multiScatteringDielectric = nodeVar141;
	nodeVar142 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar143 = ( SpecularF90 * dfg.y );
	nodeVar144 = ( nodeVar142 + vec3<f32>( nodeVar143 ) );
	nodeVar145 = ( singleScatteringMetallic + nodeVar144 );
	singleScatteringMetallic = nodeVar145;
	nodeVar146 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar147 = nodeVar146;
	nodeVar148 = ( nodeVar147 * vec3<f32>( 0.047619 ) );
	nodeVar149 = ( DiffuseColor.xyz + nodeVar148 );
	nodeVar150 = ( nodeVar144 * nodeVar149 );
	nodeVar151 = ( dfg.x + dfg.y );
	nodeVar152 = ( 1.0 - nodeVar151 );
	nodeVar153 = nodeVar152;
	nodeVar154 = ( vec3<f32>( nodeVar153 ) * nodeVar149 );
	nodeVar155 = ( vec3<f32>( 1.0 ) - nodeVar154 );
	nodeVar156 = nodeVar155;
	nodeVar157 = ( nodeVar150 / nodeVar156 );
	nodeVar158 = ( nodeVar157 * vec3<f32>( nodeVar153 ) );
	nodeVar159 = ( multiScatteringMetallic + nodeVar158 );
	multiScatteringMetallic = nodeVar159;
	nodeVar160 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar161 = ( radiance * nodeVar160 );
	nodeVar162 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar163 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar164 = ( nodeVar162 * nodeVar163 );
	nodeVar165 = ( nodeVar161 + nodeVar164 );
	nodeVar166 = nodeVar165;
	nodeVar167 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar168 = ( vec3<f32>( 1.0 ) - nodeVar167 );
	nodeVar169 = nodeVar168;
	nodeVar170 = ( DiffuseContribution * nodeVar169 );
	nodeVar171 = ( nodeVar170 * nodeVar163 );
	nodeVar172 = nodeVar171;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar173 = ( indirectSpecular + nodeVar166 );
	indirectSpecular = nodeVar173;
	nodeVar174 = ( indirectDiffuse + nodeVar172 );
	indirectDiffuse = nodeVar174;
	nodeVar175 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar175;
	nodeVar176 = dot( normalView, normalize( ( - ( render.cameraViewMatrix * vec4<f32>( ( ( ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 0u ) ] ] ).xyz * vec3<f32>( nodeVar22 ) ) + ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 1u ) ] ] ).xyz * vec3<f32>( nodeVar23 ) ) ) + ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar4 * 3u ) + 2u ) ] ] ).xyz * vec3<f32>( nodeVar24 ) ) ), 1.0 ) ).xyz ) ) );
	nodeVar177 = ( clamp( nodeVar176, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar178 = ( Roughness * -16.0 );
	nodeVar179 = ( 1.0 - nodeVar178 );
	nodeVar180 = nodeVar179;
	nodeVar181 = ( - nodeVar180 );
	nodeVar182 = exp2( nodeVar181 );
	nodeVar183 = pow( nodeVar177, nodeVar182 );
	nodeVar184 = ( 1.0 - nodeVar183 );
	nodeVar185 = nodeVar184;
	nodeVar186 = ( ambientOcclusion - nodeVar185 );
	nodeVar187 = ( indirectSpecular * vec3<f32>( clamp( nodeVar186, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar187;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar188 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar188;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar189 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar189;
	nodeVar190 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar190;
	nodeVar191 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar191;

	// result

	output.color = nodeVar191;

	return output;

}
