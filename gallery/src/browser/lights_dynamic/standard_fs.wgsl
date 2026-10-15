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

struct NodeBuffer_1163Struct {
	value : array< vec4<f32>, MAX_LIGHTS >
};
@binding( 1 ) @group( 0 )
var<uniform> NodeBuffer_1163 : NodeBuffer_1163Struct;

struct NodeBuffer_1162Struct {
	value : array< vec4<f32>, MAX_LIGHTS >
};
@binding( 2 ) @group( 0 )
var<uniform> NodeBuffer_1162 : NodeBuffer_1162Struct;

struct NodeBuffer_1164Struct {
	value : array< vec4<f32>, MAX_LIGHTS >
};
@binding( 3 ) @group( 0 )
var<uniform> NodeBuffer_1164 : NodeBuffer_1164Struct;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform11 : i32
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
var<private> irradiance : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> dynPointDiffuse : vec3<f32>;
var<private> dynPointSpecular : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
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
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
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
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : f32;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec4<f32>;

// codes
fn V_GGX_SmithCorrelated ( alpha : f32, dotNL : f32, dotNV : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = ( alpha * alpha );

	return ( 0.5 / max( ( ( dotNL * sqrt( ( nodeVar0 + ( ( 1.0 - nodeVar0 ) * ( dotNV * dotNV ) ) ) ) ) + ( dotNV * sqrt( ( nodeVar0 + ( ( 1.0 - nodeVar0 ) * ( dotNL * dotNL ) ) ) ) ) ), 0.000001 ) );

}

fn D_GGX ( alpha : f32, dotNH : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;

	nodeVar0 = ( alpha * alpha );
	nodeVar1 = ( 1.0 - ( ( dotNH * dotNH ) * ( 1.0 - nodeVar0 ) ) );

	return ( ( nodeVar0 / ( nodeVar1 * nodeVar1 ) ) * 0.3183098861837907 );

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
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar9 = ( irradiance + render.nodeUniform10 );
	irradiance = nodeVar9;
	dynPointDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	dynPointSpecular = vec3<f32>( 0.0, 0.0, 0.0 );

	for ( var i : i32 = 0; i < render.nodeUniform11; i ++ ) {

		nodeVar10 = ( NodeBuffer_1163.value[ i ].xyz - v_positionView );
		nodeVar11 = normalize( nodeVar10 );

		if ( ( NodeBuffer_1163.value[ i ].w > 0.0 ) ) {

			nodeVar13 = length( nodeVar10 );
			nodeVar14 = ( nodeVar13 / NodeBuffer_1163.value[ i ].w );
			nodeVar15 = clamp( ( 1.0 - ( ( ( nodeVar14 * nodeVar14 ) * nodeVar14 ) * nodeVar14 ) ), 0.0, 1.0 );
			nodeVar12 = ( ( 1.0 / max( pow( nodeVar13, NodeBuffer_1164.value[ i ].x ), 0.01 ) ) * ( nodeVar15 * nodeVar15 ) );

		} else {

			nodeVar12 = ( 1.0 / max( pow( length( nodeVar10 ), NodeBuffer_1164.value[ i ].x ), 0.01 ) );

		}

		nodeVar16 = ( NodeBuffer_1162.value[ i ].xyz * vec3<f32>( nodeVar12 ) );
		nodeVar17 = ( vec3<f32>( clamp( dot( normalView, nodeVar11 ), 0.0, 1.0 ) ) * nodeVar16 );
		nodeVar18 = clamp( dot( positionViewDirection, normalize( ( nodeVar11 + positionViewDirection ) ) ), 0.0, 1.0 );
		nodeVar19 = exp2( ( ( ( nodeVar18 * -5.55473 ) - 6.98316 ) * nodeVar18 ) );
		dynPointDiffuse = ( dynPointDiffuse + ( ( nodeVar17 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar19 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar19 ) ) ) ) ) );
		nodeVar20 = normalize( ( nodeVar11 + positionViewDirection ) );
		nodeVar21 = clamp( dot( positionViewDirection, nodeVar20 ), 0.0, 1.0 );
		nodeVar22 = exp2( ( ( ( nodeVar21 * -5.55473 ) - 6.98316 ) * nodeVar21 ) );
		nodeVar23 = ( Roughness * Roughness );
		dynPointSpecular = ( dynPointSpecular + ( ( nodeVar17 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar22 ) ) ) + vec3<f32>( ( 1.0 * nodeVar22 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar23, clamp( dot( normalView, nodeVar11 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar23, clamp( dot( normalView, nodeVar20 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );

	}

	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar24 = ( directDiffuse + dynPointDiffuse );
	directDiffuse = nodeVar24;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar25 = ( directSpecular + dynPointSpecular );
	directSpecular = nodeVar25;
	nodeVar26 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar27 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar28 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar29 = ( SpecularF90 * dfg.y );
	nodeVar30 = ( nodeVar28 + vec3<f32>( nodeVar29 ) );
	nodeVar31 = ( nodeVar26 + nodeVar30 );
	nodeVar26 = nodeVar31;
	nodeVar32 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar33 = nodeVar32;
	nodeVar34 = ( nodeVar33 * vec3<f32>( 0.047619 ) );
	nodeVar35 = ( SpecularColor + nodeVar34 );
	nodeVar36 = ( nodeVar30 * nodeVar35 );
	nodeVar37 = ( dfg.x + dfg.y );
	nodeVar38 = ( 1.0 - nodeVar37 );
	nodeVar39 = nodeVar38;
	nodeVar40 = ( vec3<f32>( nodeVar39 ) * nodeVar35 );
	nodeVar41 = ( vec3<f32>( 1.0 ) - nodeVar40 );
	nodeVar42 = nodeVar41;
	nodeVar43 = ( nodeVar36 / nodeVar42 );
	nodeVar44 = ( nodeVar43 * vec3<f32>( nodeVar39 ) );
	nodeVar45 = ( nodeVar27 + nodeVar44 );
	nodeVar27 = nodeVar45;
	nodeVar46 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar47 = ( irradiance * nodeVar46 );
	nodeVar48 = ( nodeVar26 + nodeVar27 );
	nodeVar49 = ( vec3<f32>( 1.0 ) - nodeVar48 );
	nodeVar50 = nodeVar49;
	nodeVar51 = ( nodeVar47 * nodeVar50 );
	nodeVar52 = nodeVar51;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar53 = ( indirectDiffuse + nodeVar52 );
	indirectDiffuse = nodeVar53;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar54 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar55 = ( SpecularF90 * dfg.y );
	nodeVar56 = ( nodeVar54 + vec3<f32>( nodeVar55 ) );
	nodeVar57 = ( singleScatteringDielectric + nodeVar56 );
	singleScatteringDielectric = nodeVar57;
	nodeVar58 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar59 = nodeVar58;
	nodeVar60 = ( nodeVar59 * vec3<f32>( 0.047619 ) );
	nodeVar61 = ( SpecularColor + nodeVar60 );
	nodeVar62 = ( nodeVar56 * nodeVar61 );
	nodeVar63 = ( dfg.x + dfg.y );
	nodeVar64 = ( 1.0 - nodeVar63 );
	nodeVar65 = nodeVar64;
	nodeVar66 = ( vec3<f32>( nodeVar65 ) * nodeVar61 );
	nodeVar67 = ( vec3<f32>( 1.0 ) - nodeVar66 );
	nodeVar68 = nodeVar67;
	nodeVar69 = ( nodeVar62 / nodeVar68 );
	nodeVar70 = ( nodeVar69 * vec3<f32>( nodeVar65 ) );
	nodeVar71 = ( multiScatteringDielectric + nodeVar70 );
	multiScatteringDielectric = nodeVar71;
	nodeVar72 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar73 = ( SpecularF90 * dfg.y );
	nodeVar74 = ( nodeVar72 + vec3<f32>( nodeVar73 ) );
	nodeVar75 = ( singleScatteringMetallic + nodeVar74 );
	singleScatteringMetallic = nodeVar75;
	nodeVar76 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar77 = nodeVar76;
	nodeVar78 = ( nodeVar77 * vec3<f32>( 0.047619 ) );
	nodeVar79 = ( DiffuseColor.xyz + nodeVar78 );
	nodeVar80 = ( nodeVar74 * nodeVar79 );
	nodeVar81 = ( dfg.x + dfg.y );
	nodeVar82 = ( 1.0 - nodeVar81 );
	nodeVar83 = nodeVar82;
	nodeVar84 = ( vec3<f32>( nodeVar83 ) * nodeVar79 );
	nodeVar85 = ( vec3<f32>( 1.0 ) - nodeVar84 );
	nodeVar86 = nodeVar85;
	nodeVar87 = ( nodeVar80 / nodeVar86 );
	nodeVar88 = ( nodeVar87 * vec3<f32>( nodeVar83 ) );
	nodeVar89 = ( multiScatteringMetallic + nodeVar88 );
	multiScatteringMetallic = nodeVar89;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar90 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar91 = ( radiance * nodeVar90 );
	nodeVar92 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar93 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar94 = ( nodeVar92 * nodeVar93 );
	nodeVar95 = ( nodeVar91 + nodeVar94 );
	nodeVar96 = nodeVar95;
	nodeVar97 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar98 = ( vec3<f32>( 1.0 ) - nodeVar97 );
	nodeVar99 = nodeVar98;
	nodeVar100 = ( DiffuseContribution * nodeVar99 );
	nodeVar101 = ( nodeVar100 * nodeVar93 );
	nodeVar102 = nodeVar101;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar103 = ( indirectSpecular + nodeVar96 );
	indirectSpecular = nodeVar103;
	nodeVar104 = ( indirectDiffuse + nodeVar102 );
	indirectDiffuse = nodeVar104;
	ambientOcclusion = 1.0;
	nodeVar105 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar105;
	nodeVar106 = dot( normalView, positionViewDirection );
	nodeVar107 = ( clamp( nodeVar106, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar108 = ( Roughness * -16.0 );
	nodeVar109 = ( 1.0 - nodeVar108 );
	nodeVar110 = nodeVar109;
	nodeVar111 = ( - nodeVar110 );
	nodeVar112 = exp2( nodeVar111 );
	nodeVar113 = pow( nodeVar107, nodeVar112 );
	nodeVar114 = ( 1.0 - nodeVar113 );
	nodeVar115 = nodeVar114;
	nodeVar116 = ( ambientOcclusion - nodeVar115 );
	nodeVar117 = ( indirectSpecular * vec3<f32>( clamp( nodeVar116, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar117;
	nodeVar118 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar118;
	nodeVar119 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar119;
	nodeVar120 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar120;
	nodeVar121 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar121;

	// result

	output.color = nodeVar121;

	return output;

}
