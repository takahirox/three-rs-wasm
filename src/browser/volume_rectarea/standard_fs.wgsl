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
@binding( 3 ) @group( 1 ) var nodeUniform10_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform10 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform12_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform12 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform11 : mat3x3<f32>,
	nodeUniform13 : mat3x3<f32>,
	nodeUniform18 : mat3x3<f32>,
	nodeUniform19 : mat3x3<f32>,
	nodeUniform24 : mat3x3<f32>,
	nodeUniform25 : mat3x3<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform17 : vec3<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform29 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform27 : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec2<f32>;
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : mat3x3<f32>;
var<private> nodeVar18 : mat3x3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : vec2<f32>;
var<private> nodeVar42 : vec4<f32>;
var<private> nodeVar43 : vec4<f32>;
var<private> nodeVar44 : vec4<f32>;
var<private> nodeVar45 : vec4<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : mat3x3<f32>;
var<private> nodeVar49 : mat3x3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec2<f32>;
var<private> nodeVar73 : vec4<f32>;
var<private> nodeVar74 : vec4<f32>;
var<private> nodeVar75 : vec4<f32>;
var<private> nodeVar76 : vec4<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : mat3x3<f32>;
var<private> nodeVar80 : mat3x3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
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
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : f32;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : f32;
var<private> nodeVar158 : f32;
var<private> nodeVar159 : f32;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : vec3<f32>;
var<private> nodeVar177 : vec3<f32>;
var<private> nodeVar178 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar179 : vec3<f32>;
var<private> nodeVar180 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar181 : vec3<f32>;
var<private> nodeVar182 : f32;
var<private> nodeVar183 : f32;
var<private> nodeVar184 : f32;
var<private> nodeVar185 : f32;
var<private> nodeVar186 : f32;
var<private> nodeVar187 : f32;
var<private> nodeVar188 : f32;
var<private> nodeVar189 : f32;
var<private> nodeVar190 : f32;
var<private> nodeVar191 : f32;
var<private> nodeVar192 : f32;
var<private> nodeVar193 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar194 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar195 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar196 : vec3<f32>;
var<private> nodeVar197 : vec4<f32>;

// codes
fn LTC_Uv ( N : vec3<f32>, V : vec3<f32>, roughness : f32 ) -> vec2<f32> {

	var nodeVar0 : vec2<f32>;

	nodeVar0 = vec2<f32>( roughness, sqrt( ( 1.0 - clamp( dot( N, V ), 0.0, 1.0 ) ) ) );
	nodeVar0 = ( ( nodeVar0 * vec2<f32>( 0.984375 ) ) + vec2<f32>( 0.0078125 ) );

	return nodeVar0;

}


fn LTC_EdgeVectorFormFactor ( v1 : vec3<f32>, v2 : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;

	nodeVar0 = dot( v1, v2 );
	nodeVar1 = abs( nodeVar0 );
	nodeVar2 = ( ( ( ( nodeVar1 * 0.0145206 ) + 0.4965155 ) * nodeVar1 ) + 0.8543985 );
	nodeVar3 = ( ( ( nodeVar1 + 4.1616724 ) * nodeVar1 ) + 3.417594 );

	if ( ( nodeVar0 > 0.0 ) ) {

		nodeVar4 = ( nodeVar2 / nodeVar3 );

	} else {

		nodeVar4 = ( ( inverseSqrt( max( ( 1.0 - ( nodeVar0 * nodeVar0 ) ), 1e-7 ) ) * 0.5 ) - ( nodeVar2 / nodeVar3 ) );

	}


	return ( cross( v1, v2 ) * vec3<f32>( nodeVar4 ) );

}


fn LTC_ClippedSphereFormFactor ( f : vec3<f32> ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = length( f );

	return max( ( ( ( nodeVar0 * nodeVar0 ) + f.z ) / ( nodeVar0 + 1.0 ) ), 0.0 );

}


fn LTC_Evaluate ( N : vec3<f32>, V : vec3<f32>, P : vec3<f32>, mInv : mat3x3<f32>, p0 : vec3<f32>, p1 : vec3<f32>, p2 : vec3<f32>, p3 : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : vec3<f32>;
	var nodeVar2 : vec3<f32>;
	var nodeVar3 : vec3<f32>;
	var nodeVar4 : mat3x3<f32>;
	var nodeVar5 : vec3<f32>;
	var nodeVar6 : vec3<f32>;
	var nodeVar7 : vec3<f32>;
	var nodeVar8 : vec3<f32>;
	var nodeVar9 : vec3<f32>;

	nodeVar0 = ( p1 - p0 );
	nodeVar1 = ( p3 - p0 );
	nodeVar2 = vec3<f32>( 0.0, 0.0, 0.0 );

	if ( ( dot( cross( nodeVar0, nodeVar1 ), ( P - p0 ) ) >= 0.0 ) ) {

		nodeVar3 = normalize( ( V - ( N * vec3<f32>( dot( V, N ) ) ) ) );
		nodeVar4 = ( mInv * transpose( mat3x3<f32>( nodeVar3, ( - cross( N, nodeVar3 ) ), N ) ) );
		nodeVar5 = normalize( ( nodeVar4 * ( p0 - P ) ) );
		nodeVar6 = normalize( ( nodeVar4 * ( p1 - P ) ) );
		nodeVar7 = normalize( ( nodeVar4 * ( p2 - P ) ) );
		nodeVar8 = normalize( ( nodeVar4 * ( p3 - P ) ) );
		nodeVar9 = vec3<f32>( 0.0, 0.0, 0.0 );
		nodeVar9 = ( nodeVar9 + LTC_EdgeVectorFormFactor( nodeVar5, nodeVar6 ) );
		nodeVar9 = ( nodeVar9 + LTC_EdgeVectorFormFactor( nodeVar6, nodeVar7 ) );
		nodeVar9 = ( nodeVar9 + LTC_EdgeVectorFormFactor( nodeVar7, nodeVar8 ) );
		nodeVar9 = ( nodeVar9 + LTC_EdgeVectorFormFactor( nodeVar8, nodeVar5 ) );
		nodeVar2 = vec3<f32>( LTC_ClippedSphereFormFactor( nodeVar9 ) );
		

	}


	return nodeVar2;

}




@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform2;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform3, 0.0525 ) + max( max( nodeVar0.x, nodeVar0.y ), nodeVar0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar1 = dot( normalView, positionViewDirection );
	nodeVar2 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( nodeVar1, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar2;
	nodeVar3 = ( dfg.x + dfg.y );
	nodeVar4 = ( 1.0 / nodeVar3 );
	nodeVar5 = nodeVar4;
	nodeVar6 = ( nodeVar5 - 1.0 );
	nodeVar7 = ( SpecularColorBlended * vec3<f32>( nodeVar6 ) );
	nodeVar8 = ( nodeVar7 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar8;
	nodeVar9 = v_positionView;
	nodeVar10 = LTC_Uv( normalView, positionViewDirection, Roughness );
	nodeVar11 = textureSample( nodeUniform10, nodeUniform10_sampler, ( object.nodeUniform11 * vec3<f32>( nodeVar10, 1.0 ) ).xy );
	nodeVar12 = nodeVar11;
	nodeVar13 = textureSample( nodeUniform12, nodeUniform12_sampler, ( object.nodeUniform13 * vec3<f32>( nodeVar10, 1.0 ) ).xy );
	nodeVar14 = nodeVar13;
	nodeVar15 = vec3<f32>( nodeVar12.x, 0.0, nodeVar12.y );
	nodeVar16 = vec3<f32>( nodeVar12.z, 0.0, nodeVar12.w );
	nodeVar17 = mat3x3<f32>( nodeVar15, vec3<f32>( 0.0, 1.0, 0.0 ), nodeVar16 );
	nodeVar18 = nodeVar17;
	nodeVar19 = ( SpecularColorBlended * vec3<f32>( nodeVar14.x ) );
	nodeVar20 = ( vec3<f32>( SpecularF90 ) - SpecularColorBlended );
	nodeVar21 = ( nodeVar20 * vec3<f32>( nodeVar14.y ) );
	nodeVar22 = ( nodeVar19 + nodeVar21 );
	nodeVar23 = nodeVar22;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar24 = ( render.nodeUniform14 * nodeVar23 );
	nodeVar25 = ( render.nodeUniform15 + render.nodeUniform16 );
	nodeVar26 = ( nodeVar25 - render.nodeUniform17 );
	nodeVar27 = ( render.nodeUniform15 - render.nodeUniform16 );
	nodeVar28 = ( nodeVar27 - render.nodeUniform17 );
	nodeVar29 = ( render.nodeUniform15 - render.nodeUniform16 );
	nodeVar30 = ( nodeVar29 + render.nodeUniform17 );
	nodeVar31 = ( render.nodeUniform15 + render.nodeUniform16 );
	nodeVar32 = ( nodeVar31 + render.nodeUniform17 );
	nodeVar33 = LTC_Evaluate( normalView, positionViewDirection, nodeVar9, nodeVar18, nodeVar26, nodeVar28, nodeVar30, nodeVar32 );
	nodeVar34 = ( nodeVar24 * nodeVar33 );
	nodeVar35 = ( directSpecular + nodeVar34 );
	directSpecular = nodeVar35;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar36 = ( render.nodeUniform14 * DiffuseContribution );
	nodeVar37 = LTC_Evaluate( normalView, positionViewDirection, nodeVar9, mat3x3<f32>( 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0 ), nodeVar26, nodeVar28, nodeVar30, nodeVar32 );
	nodeVar38 = ( nodeVar36 * nodeVar37 );
	nodeVar39 = ( directDiffuse + nodeVar38 );
	directDiffuse = nodeVar39;
	nodeVar40 = v_positionView;
	nodeVar41 = LTC_Uv( normalView, positionViewDirection, Roughness );
	nodeVar42 = textureSample( nodeUniform10, nodeUniform10_sampler, ( object.nodeUniform18 * vec3<f32>( nodeVar41, 1.0 ) ).xy );
	nodeVar43 = nodeVar42;
	nodeVar44 = textureSample( nodeUniform12, nodeUniform12_sampler, ( object.nodeUniform19 * vec3<f32>( nodeVar41, 1.0 ) ).xy );
	nodeVar45 = nodeVar44;
	nodeVar46 = vec3<f32>( nodeVar43.x, 0.0, nodeVar43.y );
	nodeVar47 = vec3<f32>( nodeVar43.z, 0.0, nodeVar43.w );
	nodeVar48 = mat3x3<f32>( nodeVar46, vec3<f32>( 0.0, 1.0, 0.0 ), nodeVar47 );
	nodeVar49 = nodeVar48;
	nodeVar50 = ( SpecularColorBlended * vec3<f32>( nodeVar45.x ) );
	nodeVar51 = ( vec3<f32>( SpecularF90 ) - SpecularColorBlended );
	nodeVar52 = ( nodeVar51 * vec3<f32>( nodeVar45.y ) );
	nodeVar53 = ( nodeVar50 + nodeVar52 );
	nodeVar54 = nodeVar53;
	nodeVar55 = ( render.nodeUniform20 * nodeVar54 );
	nodeVar56 = ( render.nodeUniform21 + render.nodeUniform22 );
	nodeVar57 = ( nodeVar56 - render.nodeUniform23 );
	nodeVar58 = ( render.nodeUniform21 - render.nodeUniform22 );
	nodeVar59 = ( nodeVar58 - render.nodeUniform23 );
	nodeVar60 = ( render.nodeUniform21 - render.nodeUniform22 );
	nodeVar61 = ( nodeVar60 + render.nodeUniform23 );
	nodeVar62 = ( render.nodeUniform21 + render.nodeUniform22 );
	nodeVar63 = ( nodeVar62 + render.nodeUniform23 );
	nodeVar64 = LTC_Evaluate( normalView, positionViewDirection, nodeVar40, nodeVar49, nodeVar57, nodeVar59, nodeVar61, nodeVar63 );
	nodeVar65 = ( nodeVar55 * nodeVar64 );
	nodeVar66 = ( directSpecular + nodeVar65 );
	directSpecular = nodeVar66;
	nodeVar67 = ( render.nodeUniform20 * DiffuseContribution );
	nodeVar68 = LTC_Evaluate( normalView, positionViewDirection, nodeVar40, mat3x3<f32>( 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0 ), nodeVar57, nodeVar59, nodeVar61, nodeVar63 );
	nodeVar69 = ( nodeVar67 * nodeVar68 );
	nodeVar70 = ( directDiffuse + nodeVar69 );
	directDiffuse = nodeVar70;
	nodeVar71 = v_positionView;
	nodeVar72 = LTC_Uv( normalView, positionViewDirection, Roughness );
	nodeVar73 = textureSample( nodeUniform10, nodeUniform10_sampler, ( object.nodeUniform24 * vec3<f32>( nodeVar72, 1.0 ) ).xy );
	nodeVar74 = nodeVar73;
	nodeVar75 = textureSample( nodeUniform12, nodeUniform12_sampler, ( object.nodeUniform25 * vec3<f32>( nodeVar72, 1.0 ) ).xy );
	nodeVar76 = nodeVar75;
	nodeVar77 = vec3<f32>( nodeVar74.x, 0.0, nodeVar74.y );
	nodeVar78 = vec3<f32>( nodeVar74.z, 0.0, nodeVar74.w );
	nodeVar79 = mat3x3<f32>( nodeVar77, vec3<f32>( 0.0, 1.0, 0.0 ), nodeVar78 );
	nodeVar80 = nodeVar79;
	nodeVar81 = ( SpecularColorBlended * vec3<f32>( nodeVar76.x ) );
	nodeVar82 = ( vec3<f32>( SpecularF90 ) - SpecularColorBlended );
	nodeVar83 = ( nodeVar82 * vec3<f32>( nodeVar76.y ) );
	nodeVar84 = ( nodeVar81 + nodeVar83 );
	nodeVar85 = nodeVar84;
	nodeVar86 = ( render.nodeUniform26 * nodeVar85 );
	nodeVar87 = ( render.nodeUniform27 + render.nodeUniform28 );
	nodeVar88 = ( nodeVar87 - render.nodeUniform29 );
	nodeVar89 = ( render.nodeUniform27 - render.nodeUniform28 );
	nodeVar90 = ( nodeVar89 - render.nodeUniform29 );
	nodeVar91 = ( render.nodeUniform27 - render.nodeUniform28 );
	nodeVar92 = ( nodeVar91 + render.nodeUniform29 );
	nodeVar93 = ( render.nodeUniform27 + render.nodeUniform28 );
	nodeVar94 = ( nodeVar93 + render.nodeUniform29 );
	nodeVar95 = LTC_Evaluate( normalView, positionViewDirection, nodeVar71, nodeVar80, nodeVar88, nodeVar90, nodeVar92, nodeVar94 );
	nodeVar96 = ( nodeVar86 * nodeVar95 );
	nodeVar97 = ( directSpecular + nodeVar96 );
	directSpecular = nodeVar97;
	nodeVar98 = ( render.nodeUniform26 * DiffuseContribution );
	nodeVar99 = LTC_Evaluate( normalView, positionViewDirection, nodeVar71, mat3x3<f32>( 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0 ), nodeVar88, nodeVar90, nodeVar92, nodeVar94 );
	nodeVar100 = ( nodeVar98 * nodeVar99 );
	nodeVar101 = ( directDiffuse + nodeVar100 );
	directDiffuse = nodeVar101;
	nodeVar102 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar103 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar104 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar105 = ( SpecularF90 * dfg.y );
	nodeVar106 = ( nodeVar104 + vec3<f32>( nodeVar105 ) );
	nodeVar107 = ( nodeVar102 + nodeVar106 );
	nodeVar102 = nodeVar107;
	nodeVar108 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar109 = nodeVar108;
	nodeVar110 = ( nodeVar109 * vec3<f32>( 0.047619 ) );
	nodeVar111 = ( SpecularColor + nodeVar110 );
	nodeVar112 = ( nodeVar106 * nodeVar111 );
	nodeVar113 = ( dfg.x + dfg.y );
	nodeVar114 = ( 1.0 - nodeVar113 );
	nodeVar115 = nodeVar114;
	nodeVar116 = ( vec3<f32>( nodeVar115 ) * nodeVar111 );
	nodeVar117 = ( vec3<f32>( 1.0 ) - nodeVar116 );
	nodeVar118 = nodeVar117;
	nodeVar119 = ( nodeVar112 / nodeVar118 );
	nodeVar120 = ( nodeVar119 * vec3<f32>( nodeVar115 ) );
	nodeVar121 = ( nodeVar103 + nodeVar120 );
	nodeVar103 = nodeVar121;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar122 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar123 = ( irradiance * nodeVar122 );
	nodeVar124 = ( nodeVar102 + nodeVar103 );
	nodeVar125 = ( vec3<f32>( 1.0 ) - nodeVar124 );
	nodeVar126 = nodeVar125;
	nodeVar127 = ( nodeVar123 * nodeVar126 );
	nodeVar128 = nodeVar127;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar129 = ( indirectDiffuse + nodeVar128 );
	indirectDiffuse = nodeVar129;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar130 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar131 = ( SpecularF90 * dfg.y );
	nodeVar132 = ( nodeVar130 + vec3<f32>( nodeVar131 ) );
	nodeVar133 = ( singleScatteringDielectric + nodeVar132 );
	singleScatteringDielectric = nodeVar133;
	nodeVar134 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar135 = nodeVar134;
	nodeVar136 = ( nodeVar135 * vec3<f32>( 0.047619 ) );
	nodeVar137 = ( SpecularColor + nodeVar136 );
	nodeVar138 = ( nodeVar132 * nodeVar137 );
	nodeVar139 = ( dfg.x + dfg.y );
	nodeVar140 = ( 1.0 - nodeVar139 );
	nodeVar141 = nodeVar140;
	nodeVar142 = ( vec3<f32>( nodeVar141 ) * nodeVar137 );
	nodeVar143 = ( vec3<f32>( 1.0 ) - nodeVar142 );
	nodeVar144 = nodeVar143;
	nodeVar145 = ( nodeVar138 / nodeVar144 );
	nodeVar146 = ( nodeVar145 * vec3<f32>( nodeVar141 ) );
	nodeVar147 = ( multiScatteringDielectric + nodeVar146 );
	multiScatteringDielectric = nodeVar147;
	nodeVar148 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar149 = ( SpecularF90 * dfg.y );
	nodeVar150 = ( nodeVar148 + vec3<f32>( nodeVar149 ) );
	nodeVar151 = ( singleScatteringMetallic + nodeVar150 );
	singleScatteringMetallic = nodeVar151;
	nodeVar152 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar153 = nodeVar152;
	nodeVar154 = ( nodeVar153 * vec3<f32>( 0.047619 ) );
	nodeVar155 = ( DiffuseColor.xyz + nodeVar154 );
	nodeVar156 = ( nodeVar150 * nodeVar155 );
	nodeVar157 = ( dfg.x + dfg.y );
	nodeVar158 = ( 1.0 - nodeVar157 );
	nodeVar159 = nodeVar158;
	nodeVar160 = ( vec3<f32>( nodeVar159 ) * nodeVar155 );
	nodeVar161 = ( vec3<f32>( 1.0 ) - nodeVar160 );
	nodeVar162 = nodeVar161;
	nodeVar163 = ( nodeVar156 / nodeVar162 );
	nodeVar164 = ( nodeVar163 * vec3<f32>( nodeVar159 ) );
	nodeVar165 = ( multiScatteringMetallic + nodeVar164 );
	multiScatteringMetallic = nodeVar165;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar166 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar167 = ( radiance * nodeVar166 );
	nodeVar168 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar169 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar170 = ( nodeVar168 * nodeVar169 );
	nodeVar171 = ( nodeVar167 + nodeVar170 );
	nodeVar172 = nodeVar171;
	nodeVar173 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar174 = ( vec3<f32>( 1.0 ) - nodeVar173 );
	nodeVar175 = nodeVar174;
	nodeVar176 = ( DiffuseContribution * nodeVar175 );
	nodeVar177 = ( nodeVar176 * nodeVar169 );
	nodeVar178 = nodeVar177;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar179 = ( indirectSpecular + nodeVar172 );
	indirectSpecular = nodeVar179;
	nodeVar180 = ( indirectDiffuse + nodeVar178 );
	indirectDiffuse = nodeVar180;
	ambientOcclusion = 1.0;
	nodeVar181 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar181;
	nodeVar182 = dot( normalView, positionViewDirection );
	nodeVar183 = ( clamp( nodeVar182, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar184 = ( Roughness * -16.0 );
	nodeVar185 = ( 1.0 - nodeVar184 );
	nodeVar186 = nodeVar185;
	nodeVar187 = ( - nodeVar186 );
	nodeVar188 = exp2( nodeVar187 );
	nodeVar189 = pow( nodeVar183, nodeVar188 );
	nodeVar190 = ( 1.0 - nodeVar189 );
	nodeVar191 = nodeVar190;
	nodeVar192 = ( ambientOcclusion - nodeVar191 );
	nodeVar193 = ( indirectSpecular * vec3<f32>( clamp( nodeVar192, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar193;
	nodeVar194 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar194;
	nodeVar195 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar195;
	nodeVar196 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar196;
	nodeVar197 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar197;

	// result

	output.color = nodeVar197;

	return output;

}
