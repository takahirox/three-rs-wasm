// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputType {
	@location( 0 ) m0 : vec4<f32>,
	@location( 1 ) m1 : vec4<f32>,
	
};
var<private> output : OutputType;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform8 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform24_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform24 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform25 : mat4x4<f32>,
	nodeUniform26 : mat4x4<f32>,
	nodeUniform28 : mat4x4<f32>,
	nodeUniform29 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform27 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform13 : f32,
	nodeUniform17 : f32,
	nodeUniform18 : f32,
	nodeUniform22 : f32,
	nodeUniform23 : f32,
	nodeUniform16 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform15 : mat4x4<f32>
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
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : vec4<f32>;
var<private> nodeVar43 : vec4<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<bool>;
var<private> nodeVar49 : bool;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : vec4<f32>;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : vec4<f32>;
var<private> nodeVar60 : vec4<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec4<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec4<f32>;
var<private> nodeVar72 : vec4<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec4<f32>;
var<private> nodeVar79 : vec4<f32>;
var<private> nodeVar80 : vec4<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
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
var<private> nodeVar128 : f32;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : f32;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
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
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : f32;
var<private> nodeVar162 : f32;
var<private> nodeVar163 : f32;
var<private> nodeVar164 : f32;
var<private> nodeVar165 : f32;
var<private> nodeVar166 : f32;
var<private> nodeVar167 : f32;
var<private> nodeVar168 : f32;
var<private> nodeVar169 : f32;
var<private> nodeVar170 : f32;
var<private> nodeVar171 : f32;
var<private> nodeVar172 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar174 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> nodeVar176 : vec4<f32>;
var<private> nodeVar177 : vec4<f32>;
var<private> nodeVar178 : vec2<f32>;

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
	@location( 1 ) positionLocal : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32>,
	@location( 4 ) v_positionViewDirection : vec3<f32>,
	@location( 5 ) positionPrevious : vec3<f32>,
	@builtin( front_facing ) isFront : bool ) -> OutputType {

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
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( ( ( f32( isFront ) * 2.0 ) - 1.0 ) ) );
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
	nodeVar9 = ( render.nodeUniform10 - v_positionView );
	nodeVar10 = normalize( nodeVar9 );
	nodeVar11 = dot( normalView, nodeVar10 );

	if ( ( render.nodeUniform12 > 0.0 ) ) {

		nodeVar13 = length( nodeVar9 );
		nodeVar14 = ( nodeVar13 / render.nodeUniform12 );
		nodeVar15 = clamp( ( 1.0 - ( ( ( nodeVar14 * nodeVar14 ) * nodeVar14 ) * nodeVar14 ) ), 0.0, 1.0 );
		nodeVar12 = ( ( 1.0 / max( pow( nodeVar13, render.nodeUniform13 ), 0.01 ) ) * ( nodeVar15 * nodeVar15 ) );

	} else {

		nodeVar12 = ( 1.0 / max( pow( length( nodeVar9 ), render.nodeUniform13 ), 0.01 ) );

	}

	nodeVar16 = ( render.nodeUniform11 * vec3<f32>( nodeVar12 ) );
	nodeVar17 = ( vec3<f32>( clamp( nodeVar11, 0.0, 1.0 ) ) * nodeVar16 );
	nodeVar18 = nodeVar17;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar19 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar20 = ( nodeVar18 * nodeVar19 );
	nodeVar21 = ( nodeVar10 + positionViewDirection );
	nodeVar22 = normalize( nodeVar21 );
	nodeVar23 = dot( positionViewDirection, nodeVar22 );
	nodeVar24 = clamp( nodeVar23, 0.0, 1.0 );
	nodeVar25 = exp2( ( ( ( nodeVar24 * -5.55473 ) - 6.98316 ) * nodeVar24 ) );
	nodeVar26 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar25 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar25 ) ) );
	nodeVar27 = ( vec3<f32>( 1.0 ) - nodeVar26 );
	nodeVar28 = nodeVar27;
	nodeVar29 = ( nodeVar20 * nodeVar28 );
	nodeVar30 = ( directDiffuse + nodeVar29 );
	directDiffuse = nodeVar30;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar31 = normalize( ( nodeVar10 + positionViewDirection ) );
	nodeVar32 = clamp( dot( positionViewDirection, nodeVar31 ), 0.0, 1.0 );
	nodeVar33 = exp2( ( ( ( nodeVar32 * -5.55473 ) - 6.98316 ) * nodeVar32 ) );
	nodeVar34 = ( Roughness * Roughness );
	nodeVar35 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar33 ) ) ) + vec3<f32>( ( 1.0 * nodeVar33 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar34, clamp( dot( normalView, nodeVar10 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar34, clamp( dot( normalView, nodeVar31 ), 0.0, 1.0 ) ) ) );
	nodeVar36 = ( nodeVar18 * nodeVar35 );
	nodeVar37 = ( nodeVar36 * multiScatteringCompensation );
	nodeVar38 = ( directSpecular + nodeVar37 );
	directSpecular = nodeVar38;
	nodeVar39 = ( render.nodeUniform14 - v_positionView );
	nodeVar40 = normalize( nodeVar39 );
	nodeVar41 = dot( normalView, nodeVar40 );
	nodeVar43 = ( render.nodeUniform15 * vec4<f32>( v_positionWorld, 1.0 ) );
	nodeVar44 = ( nodeVar43.xyz / vec3<f32>( nodeVar43.w ) );
	nodeVar45 = ( nodeVar44 * vec3<f32>( 2.0 ) );
	nodeVar46 = ( nodeVar45 - vec3<f32>( 1.0 ) );
	nodeVar47 = abs( nodeVar46 );
	nodeVar48 = ( nodeVar47 < vec3<f32>( 1.0 ) );
	nodeVar49 = all( nodeVar48 );

	if ( nodeVar49 ) {


		if ( ( render.nodeUniform22 > 0.0 ) ) {

			nodeVar51 = length( nodeVar39 );
			nodeVar52 = ( nodeVar51 / render.nodeUniform22 );
			nodeVar53 = clamp( ( 1.0 - ( ( ( nodeVar52 * nodeVar52 ) * nodeVar52 ) * nodeVar52 ) ), 0.0, 1.0 );
			nodeVar50 = ( ( 1.0 / max( pow( nodeVar51, render.nodeUniform23 ), 0.01 ) ) * ( nodeVar53 * nodeVar53 ) );

		} else {

			nodeVar50 = ( 1.0 / max( pow( length( nodeVar39 ), render.nodeUniform23 ), 0.01 ) );

		}

		nodeVar54 = textureSample( nodeUniform24, nodeUniform24_sampler, nodeVar44.xy );
		nodeVar42 = ( vec4<f32>( ( ( render.nodeUniform16 * vec3<f32>( smoothstep( render.nodeUniform17, render.nodeUniform18, dot( nodeVar40, normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform20 - render.nodeUniform21 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( nodeVar50 ) ), 1.0 ) * nodeVar54 );

	} else {


		if ( ( render.nodeUniform22 > 0.0 ) ) {

			nodeVar56 = length( nodeVar39 );
			nodeVar57 = ( nodeVar56 / render.nodeUniform22 );
			nodeVar58 = clamp( ( 1.0 - ( ( ( nodeVar57 * nodeVar57 ) * nodeVar57 ) * nodeVar57 ) ), 0.0, 1.0 );
			nodeVar55 = ( ( 1.0 / max( pow( nodeVar56, render.nodeUniform23 ), 0.01 ) ) * ( nodeVar58 * nodeVar58 ) );

		} else {

			nodeVar55 = ( 1.0 / max( pow( length( nodeVar39 ), render.nodeUniform23 ), 0.01 ) );

		}

		nodeVar42 = vec4<f32>( ( ( render.nodeUniform16 * vec3<f32>( smoothstep( render.nodeUniform17, render.nodeUniform18, dot( nodeVar40, normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform20 - render.nodeUniform21 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( nodeVar55 ) ), 1.0 );

	}

	nodeVar59 = ( vec4<f32>( clamp( nodeVar41, 0.0, 1.0 ) ) * nodeVar42 );
	nodeVar60 = nodeVar59;
	nodeVar61 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar62 = ( nodeVar60 * vec4<f32>( nodeVar61, 1.0 ) );
	nodeVar63 = ( nodeVar40 + positionViewDirection );
	nodeVar64 = normalize( nodeVar63 );
	nodeVar65 = dot( positionViewDirection, nodeVar64 );
	nodeVar66 = clamp( nodeVar65, 0.0, 1.0 );
	nodeVar67 = exp2( ( ( ( nodeVar66 * -5.55473 ) - 6.98316 ) * nodeVar66 ) );
	nodeVar68 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar67 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar67 ) ) );
	nodeVar69 = ( vec3<f32>( 1.0 ) - nodeVar68 );
	nodeVar70 = nodeVar69;
	nodeVar71 = ( nodeVar62 * vec4<f32>( nodeVar70, 1.0 ) );
	nodeVar72 = ( vec4<f32>( directDiffuse, 1.0 ) + nodeVar71 );
	directDiffuse = nodeVar72.xyz;
	nodeVar73 = normalize( ( nodeVar40 + positionViewDirection ) );
	nodeVar74 = clamp( dot( positionViewDirection, nodeVar73 ), 0.0, 1.0 );
	nodeVar75 = exp2( ( ( ( nodeVar74 * -5.55473 ) - 6.98316 ) * nodeVar74 ) );
	nodeVar76 = ( Roughness * Roughness );
	nodeVar77 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar75 ) ) ) + vec3<f32>( ( 1.0 * nodeVar75 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar76, clamp( dot( normalView, nodeVar40 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar76, clamp( dot( normalView, nodeVar73 ), 0.0, 1.0 ) ) ) );
	nodeVar78 = ( nodeVar60 * vec4<f32>( nodeVar77, 1.0 ) );
	nodeVar79 = ( nodeVar78 * vec4<f32>( multiScatteringCompensation, 1.0 ) );
	nodeVar80 = ( vec4<f32>( directSpecular, 1.0 ) + nodeVar79 );
	directSpecular = nodeVar80.xyz;
	nodeVar81 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar82 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar83 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar84 = ( SpecularF90 * dfg.y );
	nodeVar85 = ( nodeVar83 + vec3<f32>( nodeVar84 ) );
	nodeVar86 = ( nodeVar81 + nodeVar85 );
	nodeVar81 = nodeVar86;
	nodeVar87 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar88 = nodeVar87;
	nodeVar89 = ( nodeVar88 * vec3<f32>( 0.047619 ) );
	nodeVar90 = ( SpecularColor + nodeVar89 );
	nodeVar91 = ( nodeVar85 * nodeVar90 );
	nodeVar92 = ( dfg.x + dfg.y );
	nodeVar93 = ( 1.0 - nodeVar92 );
	nodeVar94 = nodeVar93;
	nodeVar95 = ( vec3<f32>( nodeVar94 ) * nodeVar90 );
	nodeVar96 = ( vec3<f32>( 1.0 ) - nodeVar95 );
	nodeVar97 = nodeVar96;
	nodeVar98 = ( nodeVar91 / nodeVar97 );
	nodeVar99 = ( nodeVar98 * vec3<f32>( nodeVar94 ) );
	nodeVar100 = ( nodeVar82 + nodeVar99 );
	nodeVar82 = nodeVar100;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar101 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar102 = ( irradiance * nodeVar101 );
	nodeVar103 = ( nodeVar81 + nodeVar82 );
	nodeVar104 = ( vec3<f32>( 1.0 ) - nodeVar103 );
	nodeVar105 = nodeVar104;
	nodeVar106 = ( nodeVar102 * nodeVar105 );
	nodeVar107 = nodeVar106;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar108 = ( indirectDiffuse + nodeVar107 );
	indirectDiffuse = nodeVar108;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar109 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar110 = ( SpecularF90 * dfg.y );
	nodeVar111 = ( nodeVar109 + vec3<f32>( nodeVar110 ) );
	nodeVar112 = ( singleScatteringDielectric + nodeVar111 );
	singleScatteringDielectric = nodeVar112;
	nodeVar113 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar114 = nodeVar113;
	nodeVar115 = ( nodeVar114 * vec3<f32>( 0.047619 ) );
	nodeVar116 = ( SpecularColor + nodeVar115 );
	nodeVar117 = ( nodeVar111 * nodeVar116 );
	nodeVar118 = ( dfg.x + dfg.y );
	nodeVar119 = ( 1.0 - nodeVar118 );
	nodeVar120 = nodeVar119;
	nodeVar121 = ( vec3<f32>( nodeVar120 ) * nodeVar116 );
	nodeVar122 = ( vec3<f32>( 1.0 ) - nodeVar121 );
	nodeVar123 = nodeVar122;
	nodeVar124 = ( nodeVar117 / nodeVar123 );
	nodeVar125 = ( nodeVar124 * vec3<f32>( nodeVar120 ) );
	nodeVar126 = ( multiScatteringDielectric + nodeVar125 );
	multiScatteringDielectric = nodeVar126;
	nodeVar127 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar128 = ( SpecularF90 * dfg.y );
	nodeVar129 = ( nodeVar127 + vec3<f32>( nodeVar128 ) );
	nodeVar130 = ( singleScatteringMetallic + nodeVar129 );
	singleScatteringMetallic = nodeVar130;
	nodeVar131 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar132 = nodeVar131;
	nodeVar133 = ( nodeVar132 * vec3<f32>( 0.047619 ) );
	nodeVar134 = ( DiffuseColor.xyz + nodeVar133 );
	nodeVar135 = ( nodeVar129 * nodeVar134 );
	nodeVar136 = ( dfg.x + dfg.y );
	nodeVar137 = ( 1.0 - nodeVar136 );
	nodeVar138 = nodeVar137;
	nodeVar139 = ( vec3<f32>( nodeVar138 ) * nodeVar134 );
	nodeVar140 = ( vec3<f32>( 1.0 ) - nodeVar139 );
	nodeVar141 = nodeVar140;
	nodeVar142 = ( nodeVar135 / nodeVar141 );
	nodeVar143 = ( nodeVar142 * vec3<f32>( nodeVar138 ) );
	nodeVar144 = ( multiScatteringMetallic + nodeVar143 );
	multiScatteringMetallic = nodeVar144;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar145 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar146 = ( radiance * nodeVar145 );
	nodeVar147 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar148 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar149 = ( nodeVar147 * nodeVar148 );
	nodeVar150 = ( nodeVar146 + nodeVar149 );
	nodeVar151 = nodeVar150;
	nodeVar152 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar153 = ( vec3<f32>( 1.0 ) - nodeVar152 );
	nodeVar154 = nodeVar153;
	nodeVar155 = ( DiffuseContribution * nodeVar154 );
	nodeVar156 = ( nodeVar155 * nodeVar148 );
	nodeVar157 = nodeVar156;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar158 = ( indirectSpecular + nodeVar151 );
	indirectSpecular = nodeVar158;
	nodeVar159 = ( indirectDiffuse + nodeVar157 );
	indirectDiffuse = nodeVar159;
	ambientOcclusion = 1.0;
	nodeVar160 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar160;
	nodeVar161 = dot( normalView, positionViewDirection );
	nodeVar162 = ( clamp( nodeVar161, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar163 = ( Roughness * -16.0 );
	nodeVar164 = ( 1.0 - nodeVar163 );
	nodeVar165 = nodeVar164;
	nodeVar166 = ( - nodeVar165 );
	nodeVar167 = exp2( nodeVar166 );
	nodeVar168 = pow( nodeVar162, nodeVar167 );
	nodeVar169 = ( 1.0 - nodeVar168 );
	nodeVar170 = nodeVar169;
	nodeVar171 = ( ambientOcclusion - nodeVar170 );
	nodeVar172 = ( indirectSpecular * vec3<f32>( clamp( nodeVar171, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar172;
	nodeVar173 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar173;
	nodeVar174 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar174;
	nodeVar175 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar175;
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	output.m0 = Output;
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform26 );
	nodeVar176 = ( ( object.nodeUniform25 * modelViewMatrix ) * vec4<f32>( positionLocal, 1.0 ) );
	nodeVar177 = ( ( render.nodeUniform27 * ( object.nodeUniform28 * object.nodeUniform29 ) ) * vec4<f32>( positionPrevious, 1.0 ) );
	nodeVar178 = ( ( nodeVar176.xy / vec2<f32>( nodeVar176.w ) ) - ( nodeVar177.xy / vec2<f32>( nodeVar177.w ) ) );
	output.m1 = vec4<f32>( vec3<f32>( nodeVar178, 0.0 ), 1.0 );

	// result

	return output;

}
