// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform7_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform7 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform9_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform9 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform11_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform11 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform5 : vec3<f32>,
	nodeUniform6 : f32,
	nodeUniform8 : mat4x4<f32>,
	nodeUniform10 : mat3x3<f32>,
	nodeUniform12 : mat3x3<f32>,
	nodeUniform17 : mat3x3<f32>,
	nodeUniform18 : mat3x3<f32>,
	nodeUniform23 : mat3x3<f32>,
	nodeUniform24 : mat3x3<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform19 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform26 : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> nodeVar0 : vec2<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
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
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec2<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : mat3x3<f32>;
var<private> nodeVar19 : mat3x3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
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
var<private> nodeVar36 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : vec2<f32>;
var<private> nodeVar43 : vec4<f32>;
var<private> nodeVar44 : vec4<f32>;
var<private> nodeVar45 : vec4<f32>;
var<private> nodeVar46 : vec4<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : mat3x3<f32>;
var<private> nodeVar50 : mat3x3<f32>;
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
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec2<f32>;
var<private> nodeVar74 : vec4<f32>;
var<private> nodeVar75 : vec4<f32>;
var<private> nodeVar76 : vec4<f32>;
var<private> nodeVar77 : vec4<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : mat3x3<f32>;
var<private> nodeVar81 : mat3x3<f32>;
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
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : f32;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : f32;
var<private> nodeVar159 : f32;
var<private> nodeVar160 : f32;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : vec3<f32>;
var<private> nodeVar177 : vec3<f32>;
var<private> nodeVar178 : vec3<f32>;
var<private> nodeVar179 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar180 : vec3<f32>;
var<private> nodeVar181 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar182 : vec3<f32>;
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
var<private> nodeVar193 : f32;
var<private> nodeVar194 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar195 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar196 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar197 : vec3<f32>;
var<private> nodeVar198 : vec4<f32>;

// codes
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


fn tsl_mod_float( x : f32, y : f32 ) -> f32 { return x - y * floor( x / y ); }
fn LTC_Uv ( N : vec3<f32>, V : vec3<f32>, roughness : f32 ) -> vec2<f32> {

	var nodeVar0 : vec2<f32>;

	nodeVar0 = vec2<f32>( roughness, sqrt( ( 1.0 - clamp( dot( N, V ), 0.0, 1.0 ) ) ) );
	nodeVar0 = ( ( nodeVar0 * vec2<f32>( 0.984375 ) ) + vec2<f32>( 0.0078125 ) );

	return nodeVar0;

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
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) nodeVarying6 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform2;
	nodeVar0 = ( ( nodeVarying6 * vec2<f32>( 400.0 ) ) * vec2<f32>( 2.0 ) );
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar1 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( sign( tsl_mod_float( ( floor( nodeVar0.x ) + floor( nodeVar0.y ) ), 2.0 ) ), 0.0525 ) + max( max( nodeVar1.x, nodeVar1.y ), nodeVar1.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	EmissiveColor = ( object.nodeUniform5 * vec3<f32>( object.nodeUniform6 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar2 = dot( normalView, positionViewDirection );
	nodeVar3 = textureSample( nodeUniform7, nodeUniform7_sampler, vec2<f32>( Roughness, clamp( nodeVar2, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar3;
	nodeVar4 = ( dfg.x + dfg.y );
	nodeVar5 = ( 1.0 / nodeVar4 );
	nodeVar6 = nodeVar5;
	nodeVar7 = ( nodeVar6 - 1.0 );
	nodeVar8 = ( SpecularColorBlended * vec3<f32>( nodeVar7 ) );
	nodeVar9 = ( nodeVar8 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar9;
	nodeVar10 = v_positionView;
	nodeVar11 = LTC_Uv( normalView, positionViewDirection, Roughness );
	nodeVar12 = textureSample( nodeUniform9, nodeUniform9_sampler, ( object.nodeUniform10 * vec3<f32>( nodeVar11, 1.0 ) ).xy );
	nodeVar13 = nodeVar12;
	nodeVar14 = textureSample( nodeUniform11, nodeUniform11_sampler, ( object.nodeUniform12 * vec3<f32>( nodeVar11, 1.0 ) ).xy );
	nodeVar15 = nodeVar14;
	nodeVar16 = vec3<f32>( nodeVar13.x, 0.0, nodeVar13.y );
	nodeVar17 = vec3<f32>( nodeVar13.z, 0.0, nodeVar13.w );
	nodeVar18 = mat3x3<f32>( nodeVar16, vec3<f32>( 0.0, 1.0, 0.0 ), nodeVar17 );
	nodeVar19 = nodeVar18;
	nodeVar20 = ( SpecularColorBlended * vec3<f32>( nodeVar15.x ) );
	nodeVar21 = ( vec3<f32>( SpecularF90 ) - SpecularColorBlended );
	nodeVar22 = ( nodeVar21 * vec3<f32>( nodeVar15.y ) );
	nodeVar23 = ( nodeVar20 + nodeVar22 );
	nodeVar24 = nodeVar23;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar25 = ( render.nodeUniform13 * nodeVar24 );
	nodeVar26 = ( render.nodeUniform14 + render.nodeUniform15 );
	nodeVar27 = ( nodeVar26 - render.nodeUniform16 );
	nodeVar28 = ( render.nodeUniform14 - render.nodeUniform15 );
	nodeVar29 = ( nodeVar28 - render.nodeUniform16 );
	nodeVar30 = ( render.nodeUniform14 - render.nodeUniform15 );
	nodeVar31 = ( nodeVar30 + render.nodeUniform16 );
	nodeVar32 = ( render.nodeUniform14 + render.nodeUniform15 );
	nodeVar33 = ( nodeVar32 + render.nodeUniform16 );
	nodeVar34 = LTC_Evaluate( normalView, positionViewDirection, nodeVar10, nodeVar19, nodeVar27, nodeVar29, nodeVar31, nodeVar33 );
	nodeVar35 = ( nodeVar25 * nodeVar34 );
	nodeVar36 = ( directSpecular + nodeVar35 );
	directSpecular = nodeVar36;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar37 = ( render.nodeUniform13 * DiffuseContribution );
	nodeVar38 = LTC_Evaluate( normalView, positionViewDirection, nodeVar10, mat3x3<f32>( 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0 ), nodeVar27, nodeVar29, nodeVar31, nodeVar33 );
	nodeVar39 = ( nodeVar37 * nodeVar38 );
	nodeVar40 = ( directDiffuse + nodeVar39 );
	directDiffuse = nodeVar40;
	nodeVar41 = v_positionView;
	nodeVar42 = LTC_Uv( normalView, positionViewDirection, Roughness );
	nodeVar43 = textureSample( nodeUniform9, nodeUniform9_sampler, ( object.nodeUniform17 * vec3<f32>( nodeVar42, 1.0 ) ).xy );
	nodeVar44 = nodeVar43;
	nodeVar45 = textureSample( nodeUniform11, nodeUniform11_sampler, ( object.nodeUniform18 * vec3<f32>( nodeVar42, 1.0 ) ).xy );
	nodeVar46 = nodeVar45;
	nodeVar47 = vec3<f32>( nodeVar44.x, 0.0, nodeVar44.y );
	nodeVar48 = vec3<f32>( nodeVar44.z, 0.0, nodeVar44.w );
	nodeVar49 = mat3x3<f32>( nodeVar47, vec3<f32>( 0.0, 1.0, 0.0 ), nodeVar48 );
	nodeVar50 = nodeVar49;
	nodeVar51 = ( SpecularColorBlended * vec3<f32>( nodeVar46.x ) );
	nodeVar52 = ( vec3<f32>( SpecularF90 ) - SpecularColorBlended );
	nodeVar53 = ( nodeVar52 * vec3<f32>( nodeVar46.y ) );
	nodeVar54 = ( nodeVar51 + nodeVar53 );
	nodeVar55 = nodeVar54;
	nodeVar56 = ( render.nodeUniform19 * nodeVar55 );
	nodeVar57 = ( render.nodeUniform20 + render.nodeUniform21 );
	nodeVar58 = ( nodeVar57 - render.nodeUniform22 );
	nodeVar59 = ( render.nodeUniform20 - render.nodeUniform21 );
	nodeVar60 = ( nodeVar59 - render.nodeUniform22 );
	nodeVar61 = ( render.nodeUniform20 - render.nodeUniform21 );
	nodeVar62 = ( nodeVar61 + render.nodeUniform22 );
	nodeVar63 = ( render.nodeUniform20 + render.nodeUniform21 );
	nodeVar64 = ( nodeVar63 + render.nodeUniform22 );
	nodeVar65 = LTC_Evaluate( normalView, positionViewDirection, nodeVar41, nodeVar50, nodeVar58, nodeVar60, nodeVar62, nodeVar64 );
	nodeVar66 = ( nodeVar56 * nodeVar65 );
	nodeVar67 = ( directSpecular + nodeVar66 );
	directSpecular = nodeVar67;
	nodeVar68 = ( render.nodeUniform19 * DiffuseContribution );
	nodeVar69 = LTC_Evaluate( normalView, positionViewDirection, nodeVar41, mat3x3<f32>( 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0 ), nodeVar58, nodeVar60, nodeVar62, nodeVar64 );
	nodeVar70 = ( nodeVar68 * nodeVar69 );
	nodeVar71 = ( directDiffuse + nodeVar70 );
	directDiffuse = nodeVar71;
	nodeVar72 = v_positionView;
	nodeVar73 = LTC_Uv( normalView, positionViewDirection, Roughness );
	nodeVar74 = textureSample( nodeUniform9, nodeUniform9_sampler, ( object.nodeUniform23 * vec3<f32>( nodeVar73, 1.0 ) ).xy );
	nodeVar75 = nodeVar74;
	nodeVar76 = textureSample( nodeUniform11, nodeUniform11_sampler, ( object.nodeUniform24 * vec3<f32>( nodeVar73, 1.0 ) ).xy );
	nodeVar77 = nodeVar76;
	nodeVar78 = vec3<f32>( nodeVar75.x, 0.0, nodeVar75.y );
	nodeVar79 = vec3<f32>( nodeVar75.z, 0.0, nodeVar75.w );
	nodeVar80 = mat3x3<f32>( nodeVar78, vec3<f32>( 0.0, 1.0, 0.0 ), nodeVar79 );
	nodeVar81 = nodeVar80;
	nodeVar82 = ( SpecularColorBlended * vec3<f32>( nodeVar77.x ) );
	nodeVar83 = ( vec3<f32>( SpecularF90 ) - SpecularColorBlended );
	nodeVar84 = ( nodeVar83 * vec3<f32>( nodeVar77.y ) );
	nodeVar85 = ( nodeVar82 + nodeVar84 );
	nodeVar86 = nodeVar85;
	nodeVar87 = ( render.nodeUniform25 * nodeVar86 );
	nodeVar88 = ( render.nodeUniform26 + render.nodeUniform27 );
	nodeVar89 = ( nodeVar88 - render.nodeUniform28 );
	nodeVar90 = ( render.nodeUniform26 - render.nodeUniform27 );
	nodeVar91 = ( nodeVar90 - render.nodeUniform28 );
	nodeVar92 = ( render.nodeUniform26 - render.nodeUniform27 );
	nodeVar93 = ( nodeVar92 + render.nodeUniform28 );
	nodeVar94 = ( render.nodeUniform26 + render.nodeUniform27 );
	nodeVar95 = ( nodeVar94 + render.nodeUniform28 );
	nodeVar96 = LTC_Evaluate( normalView, positionViewDirection, nodeVar72, nodeVar81, nodeVar89, nodeVar91, nodeVar93, nodeVar95 );
	nodeVar97 = ( nodeVar87 * nodeVar96 );
	nodeVar98 = ( directSpecular + nodeVar97 );
	directSpecular = nodeVar98;
	nodeVar99 = ( render.nodeUniform25 * DiffuseContribution );
	nodeVar100 = LTC_Evaluate( normalView, positionViewDirection, nodeVar72, mat3x3<f32>( 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0 ), nodeVar89, nodeVar91, nodeVar93, nodeVar95 );
	nodeVar101 = ( nodeVar99 * nodeVar100 );
	nodeVar102 = ( directDiffuse + nodeVar101 );
	directDiffuse = nodeVar102;
	nodeVar103 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar104 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar105 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar106 = ( SpecularF90 * dfg.y );
	nodeVar107 = ( nodeVar105 + vec3<f32>( nodeVar106 ) );
	nodeVar108 = ( nodeVar103 + nodeVar107 );
	nodeVar103 = nodeVar108;
	nodeVar109 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar110 = nodeVar109;
	nodeVar111 = ( nodeVar110 * vec3<f32>( 0.047619 ) );
	nodeVar112 = ( SpecularColor + nodeVar111 );
	nodeVar113 = ( nodeVar107 * nodeVar112 );
	nodeVar114 = ( dfg.x + dfg.y );
	nodeVar115 = ( 1.0 - nodeVar114 );
	nodeVar116 = nodeVar115;
	nodeVar117 = ( vec3<f32>( nodeVar116 ) * nodeVar112 );
	nodeVar118 = ( vec3<f32>( 1.0 ) - nodeVar117 );
	nodeVar119 = nodeVar118;
	nodeVar120 = ( nodeVar113 / nodeVar119 );
	nodeVar121 = ( nodeVar120 * vec3<f32>( nodeVar116 ) );
	nodeVar122 = ( nodeVar104 + nodeVar121 );
	nodeVar104 = nodeVar122;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar123 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar124 = ( irradiance * nodeVar123 );
	nodeVar125 = ( nodeVar103 + nodeVar104 );
	nodeVar126 = ( vec3<f32>( 1.0 ) - nodeVar125 );
	nodeVar127 = nodeVar126;
	nodeVar128 = ( nodeVar124 * nodeVar127 );
	nodeVar129 = nodeVar128;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar130 = ( indirectDiffuse + nodeVar129 );
	indirectDiffuse = nodeVar130;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar131 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar132 = ( SpecularF90 * dfg.y );
	nodeVar133 = ( nodeVar131 + vec3<f32>( nodeVar132 ) );
	nodeVar134 = ( singleScatteringDielectric + nodeVar133 );
	singleScatteringDielectric = nodeVar134;
	nodeVar135 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar136 = nodeVar135;
	nodeVar137 = ( nodeVar136 * vec3<f32>( 0.047619 ) );
	nodeVar138 = ( SpecularColor + nodeVar137 );
	nodeVar139 = ( nodeVar133 * nodeVar138 );
	nodeVar140 = ( dfg.x + dfg.y );
	nodeVar141 = ( 1.0 - nodeVar140 );
	nodeVar142 = nodeVar141;
	nodeVar143 = ( vec3<f32>( nodeVar142 ) * nodeVar138 );
	nodeVar144 = ( vec3<f32>( 1.0 ) - nodeVar143 );
	nodeVar145 = nodeVar144;
	nodeVar146 = ( nodeVar139 / nodeVar145 );
	nodeVar147 = ( nodeVar146 * vec3<f32>( nodeVar142 ) );
	nodeVar148 = ( multiScatteringDielectric + nodeVar147 );
	multiScatteringDielectric = nodeVar148;
	nodeVar149 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar150 = ( SpecularF90 * dfg.y );
	nodeVar151 = ( nodeVar149 + vec3<f32>( nodeVar150 ) );
	nodeVar152 = ( singleScatteringMetallic + nodeVar151 );
	singleScatteringMetallic = nodeVar152;
	nodeVar153 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar154 = nodeVar153;
	nodeVar155 = ( nodeVar154 * vec3<f32>( 0.047619 ) );
	nodeVar156 = ( DiffuseColor.xyz + nodeVar155 );
	nodeVar157 = ( nodeVar151 * nodeVar156 );
	nodeVar158 = ( dfg.x + dfg.y );
	nodeVar159 = ( 1.0 - nodeVar158 );
	nodeVar160 = nodeVar159;
	nodeVar161 = ( vec3<f32>( nodeVar160 ) * nodeVar156 );
	nodeVar162 = ( vec3<f32>( 1.0 ) - nodeVar161 );
	nodeVar163 = nodeVar162;
	nodeVar164 = ( nodeVar157 / nodeVar163 );
	nodeVar165 = ( nodeVar164 * vec3<f32>( nodeVar160 ) );
	nodeVar166 = ( multiScatteringMetallic + nodeVar165 );
	multiScatteringMetallic = nodeVar166;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar167 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar168 = ( radiance * nodeVar167 );
	nodeVar169 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar170 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar171 = ( nodeVar169 * nodeVar170 );
	nodeVar172 = ( nodeVar168 + nodeVar171 );
	nodeVar173 = nodeVar172;
	nodeVar174 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar175 = ( vec3<f32>( 1.0 ) - nodeVar174 );
	nodeVar176 = nodeVar175;
	nodeVar177 = ( DiffuseContribution * nodeVar176 );
	nodeVar178 = ( nodeVar177 * nodeVar170 );
	nodeVar179 = nodeVar178;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar180 = ( indirectSpecular + nodeVar173 );
	indirectSpecular = nodeVar180;
	nodeVar181 = ( indirectDiffuse + nodeVar179 );
	indirectDiffuse = nodeVar181;
	ambientOcclusion = 1.0;
	nodeVar182 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar182;
	nodeVar183 = dot( normalView, positionViewDirection );
	nodeVar184 = ( clamp( nodeVar183, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar185 = ( Roughness * -16.0 );
	nodeVar186 = ( 1.0 - nodeVar185 );
	nodeVar187 = nodeVar186;
	nodeVar188 = ( - nodeVar187 );
	nodeVar189 = exp2( nodeVar188 );
	nodeVar190 = pow( nodeVar184, nodeVar189 );
	nodeVar191 = ( 1.0 - nodeVar190 );
	nodeVar192 = nodeVar191;
	nodeVar193 = ( ambientOcclusion - nodeVar192 );
	nodeVar194 = ( indirectSpecular * vec3<f32>( clamp( nodeVar193, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar194;
	nodeVar195 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar195;
	nodeVar196 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar196;
	nodeVar197 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar197;
	nodeVar198 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar198;

	// result

	output.color = nodeVar198;

	return output;

}
