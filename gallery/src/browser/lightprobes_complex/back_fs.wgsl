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
@binding( 3 ) @group( 1 ) var nodeUniform18_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform18 : texture_depth_cube;
@binding( 5 ) @group( 1 ) var nodeUniform31_sampler : sampler_comparison;
@binding( 6 ) @group( 1 ) var nodeUniform31 : texture_depth_cube;

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
	nodeUniform11 : vec3<f32>,
	nodeUniform22 : f32,
	nodeUniform23 : f32,
	nodeUniform25 : vec3<f32>,
	nodeUniform35 : f32,
	nodeUniform36 : f32,
	nodeUniform10 : vec3<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform16 : f32,
	nodeUniform15 : f32,
	nodeUniform14 : f32,
	nodeUniform17 : f32,
	nodeUniform19 : f32,
	nodeUniform20 : vec2<f32>,
	nodeUniform21 : f32,
	nodeUniform26 : mat4x4<f32>,
	nodeUniform29 : f32,
	nodeUniform28 : f32,
	nodeUniform27 : f32,
	nodeUniform30 : f32,
	nodeUniform32 : f32,
	nodeUniform33 : vec2<f32>,
	nodeUniform34 : f32
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
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : vec2<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec2<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec2<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : vec2<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec2<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : vec2<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : vec2<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : vec2<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : vec2<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : vec2<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : f32;
var<private> nodeVar111 : f32;
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
var<private> irradiance : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : f32;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : f32;
var<private> nodeVar163 : f32;
var<private> nodeVar164 : f32;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : f32;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : vec3<f32>;
var<private> nodeVar177 : vec3<f32>;
var<private> nodeVar178 : vec3<f32>;
var<private> nodeVar179 : vec3<f32>;
var<private> nodeVar180 : f32;
var<private> nodeVar181 : f32;
var<private> nodeVar182 : f32;
var<private> nodeVar183 : vec3<f32>;
var<private> nodeVar184 : vec3<f32>;
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> nodeVar187 : vec3<f32>;
var<private> nodeVar188 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar189 : vec3<f32>;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar192 : vec3<f32>;
var<private> nodeVar193 : vec3<f32>;
var<private> nodeVar194 : vec3<f32>;
var<private> nodeVar195 : vec3<f32>;
var<private> nodeVar196 : vec3<f32>;
var<private> nodeVar197 : vec3<f32>;
var<private> nodeVar198 : vec3<f32>;
var<private> nodeVar199 : vec3<f32>;
var<private> nodeVar200 : vec3<f32>;
var<private> nodeVar201 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar202 : vec3<f32>;
var<private> nodeVar203 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar204 : vec3<f32>;
var<private> nodeVar205 : f32;
var<private> nodeVar206 : f32;
var<private> nodeVar207 : f32;
var<private> nodeVar208 : f32;
var<private> nodeVar209 : f32;
var<private> nodeVar210 : f32;
var<private> nodeVar211 : f32;
var<private> nodeVar212 : f32;
var<private> nodeVar213 : f32;
var<private> nodeVar214 : f32;
var<private> nodeVar215 : f32;
var<private> nodeVar216 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar217 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar218 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar219 : vec3<f32>;
var<private> nodeVar220 : vec4<f32>;

// codes
fn interleavedGradientNoise ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn vogelDiskSample ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	var nodeVar0 : f32;

	nodeVar0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeVar0 ), sin( nodeVar0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

}


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
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

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
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
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
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst0 = ( render.nodeUniform12 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform14 ) ) ), 1.0 ) ).xyz;
	let nodeConst1 = abs( nodeConst0 );
	nodeVar12 = 1.0;
	nodeVar13 = max( max( nodeConst1.x, nodeConst1.y ), nodeConst1.z );

	if ( ( ( ( nodeVar13 - render.nodeUniform15 ) <= 0.0 ) && ( ( nodeVar13 - render.nodeUniform16 ) >= 0.0 ) ) ) {

		nodeVar14 = ( - nodeVar13 );
		nodeVar15 = ( ( ( render.nodeUniform16 + nodeVar14 ) * render.nodeUniform15 ) / ( ( render.nodeUniform15 - render.nodeUniform16 ) * nodeVar14 ) );
		nodeVar15 = ( nodeVar15 + render.nodeUniform17 );
		nodeVar16 = normalize( nodeConst0 );
		nodeVar18 = abs( nodeVar16 );

		if ( ( nodeVar18.x > nodeVar18.z ) ) {

			nodeVar17 = vec3<f32>( 0.0, 1.0, 0.0 );

		} else {

			nodeVar17 = vec3<f32>( 1.0, 0.0, 0.0 );

		}

		nodeVar19 = normalize( cross( nodeVar16, nodeVar17 ) );
		nodeVar20 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar21 = vogelDiskSample( 0, 5, nodeVar20 );
		nodeVar22 = cross( nodeVar16, nodeVar19 );
		nodeVar23 = ( render.nodeUniform19 / render.nodeUniform20.x );
		nodeVar24 = ( nodeVar16 + ( ( ( nodeVar19 * vec3<f32>( nodeVar21.x ) ) + ( nodeVar22 * vec3<f32>( nodeVar21.y ) ) ) * vec3<f32>( nodeVar23 ) ) );
		nodeVar25 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, vec3<f32>( nodeVar24.x, ( - nodeVar24.y ), nodeVar24.z ), nodeVar15 );
		nodeVar26 = vogelDiskSample( 1, 5, nodeVar20 );
		nodeVar27 = ( nodeVar16 + ( ( ( nodeVar19 * vec3<f32>( nodeVar26.x ) ) + ( nodeVar22 * vec3<f32>( nodeVar26.y ) ) ) * vec3<f32>( nodeVar23 ) ) );
		nodeVar28 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, vec3<f32>( nodeVar27.x, ( - nodeVar27.y ), nodeVar27.z ), nodeVar15 );
		nodeVar29 = vogelDiskSample( 2, 5, nodeVar20 );
		nodeVar30 = ( nodeVar16 + ( ( ( nodeVar19 * vec3<f32>( nodeVar29.x ) ) + ( nodeVar22 * vec3<f32>( nodeVar29.y ) ) ) * vec3<f32>( nodeVar23 ) ) );
		nodeVar31 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, vec3<f32>( nodeVar30.x, ( - nodeVar30.y ), nodeVar30.z ), nodeVar15 );
		nodeVar32 = vogelDiskSample( 3, 5, nodeVar20 );
		nodeVar33 = ( nodeVar16 + ( ( ( nodeVar19 * vec3<f32>( nodeVar32.x ) ) + ( nodeVar22 * vec3<f32>( nodeVar32.y ) ) ) * vec3<f32>( nodeVar23 ) ) );
		nodeVar34 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, vec3<f32>( nodeVar33.x, ( - nodeVar33.y ), nodeVar33.z ), nodeVar15 );
		nodeVar35 = vogelDiskSample( 4, 5, nodeVar20 );
		nodeVar36 = ( nodeVar16 + ( ( ( nodeVar19 * vec3<f32>( nodeVar35.x ) ) + ( nodeVar22 * vec3<f32>( nodeVar35.y ) ) ) * vec3<f32>( nodeVar23 ) ) );
		nodeVar37 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, vec3<f32>( nodeVar36.x, ( - nodeVar36.y ), nodeVar36.z ), nodeVar15 );
		nodeVar12 = ( ( ( ( ( nodeVar25 + nodeVar28 ) + nodeVar31 ) + nodeVar34 ) + nodeVar37 ) * 0.2 );
		

	}

	nodeVar38 = mix( 1.0, nodeVar12, render.nodeUniform21 );
	nodeVar39 = ( render.nodeUniform11 * vec3<f32>( nodeVar38 ) );

	if ( ( render.nodeUniform22 > 0.0 ) ) {

		nodeVar41 = length( nodeVar9 );
		nodeVar42 = ( nodeVar41 / render.nodeUniform22 );
		nodeVar43 = clamp( ( 1.0 - ( ( ( nodeVar42 * nodeVar42 ) * nodeVar42 ) * nodeVar42 ) ), 0.0, 1.0 );
		nodeVar40 = ( ( 1.0 / max( pow( nodeVar41, render.nodeUniform23 ), 0.01 ) ) * ( nodeVar43 * nodeVar43 ) );

	} else {

		nodeVar40 = ( 1.0 / max( pow( length( nodeVar9 ), render.nodeUniform23 ), 0.01 ) );

	}

	nodeVar44 = ( nodeVar39 * vec3<f32>( nodeVar40 ) );
	nodeVar45 = ( vec3<f32>( clamp( nodeVar11, 0.0, 1.0 ) ) * nodeVar44 );
	nodeVar46 = nodeVar45;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar47 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar48 = ( nodeVar46 * nodeVar47 );
	nodeVar49 = ( nodeVar10 + positionViewDirection );
	nodeVar50 = normalize( nodeVar49 );
	nodeVar51 = dot( positionViewDirection, nodeVar50 );
	nodeVar52 = clamp( nodeVar51, 0.0, 1.0 );
	nodeVar53 = exp2( ( ( ( nodeVar52 * -5.55473 ) - 6.98316 ) * nodeVar52 ) );
	nodeVar54 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar53 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar53 ) ) );
	nodeVar55 = ( vec3<f32>( 1.0 ) - nodeVar54 );
	nodeVar56 = nodeVar55;
	nodeVar57 = ( nodeVar48 * nodeVar56 );
	nodeVar58 = ( directDiffuse + nodeVar57 );
	directDiffuse = nodeVar58;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar59 = normalize( ( nodeVar10 + positionViewDirection ) );
	nodeVar60 = clamp( dot( positionViewDirection, nodeVar59 ), 0.0, 1.0 );
	nodeVar61 = exp2( ( ( ( nodeVar60 * -5.55473 ) - 6.98316 ) * nodeVar60 ) );
	nodeVar62 = ( Roughness * Roughness );
	nodeVar63 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar61 ) ) ) + vec3<f32>( ( 1.0 * nodeVar61 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar62, clamp( dot( normalView, nodeVar10 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar62, clamp( dot( normalView, nodeVar59 ), 0.0, 1.0 ) ) ) );
	nodeVar64 = ( nodeVar46 * nodeVar63 );
	nodeVar65 = ( nodeVar64 * multiScatteringCompensation );
	nodeVar66 = ( directSpecular + nodeVar65 );
	directSpecular = nodeVar66;
	nodeVar67 = ( render.nodeUniform24 - v_positionView );
	nodeVar68 = normalize( nodeVar67 );
	nodeVar69 = dot( normalView, nodeVar68 );
	shadowPositionWorld = v_positionWorld;
	let nodeConst2 = ( render.nodeUniform26 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform27 ) ) ), 1.0 ) ).xyz;
	let nodeConst3 = abs( nodeConst2 );
	nodeVar70 = 1.0;
	nodeVar71 = max( max( nodeConst3.x, nodeConst3.y ), nodeConst3.z );

	if ( ( ( ( nodeVar71 - render.nodeUniform28 ) <= 0.0 ) && ( ( nodeVar71 - render.nodeUniform29 ) >= 0.0 ) ) ) {

		nodeVar72 = ( - nodeVar71 );
		nodeVar73 = ( ( ( render.nodeUniform29 + nodeVar72 ) * render.nodeUniform28 ) / ( ( render.nodeUniform28 - render.nodeUniform29 ) * nodeVar72 ) );
		nodeVar73 = ( nodeVar73 + render.nodeUniform30 );
		nodeVar74 = normalize( nodeConst2 );
		nodeVar76 = abs( nodeVar74 );

		if ( ( nodeVar76.x > nodeVar76.z ) ) {

			nodeVar75 = vec3<f32>( 0.0, 1.0, 0.0 );

		} else {

			nodeVar75 = vec3<f32>( 1.0, 0.0, 0.0 );

		}

		nodeVar77 = normalize( cross( nodeVar74, nodeVar75 ) );
		nodeVar78 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar79 = vogelDiskSample( 0, 5, nodeVar78 );
		nodeVar80 = cross( nodeVar74, nodeVar77 );
		nodeVar81 = ( render.nodeUniform32 / render.nodeUniform33.x );
		nodeVar82 = ( nodeVar74 + ( ( ( nodeVar77 * vec3<f32>( nodeVar79.x ) ) + ( nodeVar80 * vec3<f32>( nodeVar79.y ) ) ) * vec3<f32>( nodeVar81 ) ) );
		nodeVar83 = textureSampleCompare( nodeUniform31, nodeUniform31_sampler, vec3<f32>( nodeVar82.x, ( - nodeVar82.y ), nodeVar82.z ), nodeVar73 );
		nodeVar84 = vogelDiskSample( 1, 5, nodeVar78 );
		nodeVar85 = ( nodeVar74 + ( ( ( nodeVar77 * vec3<f32>( nodeVar84.x ) ) + ( nodeVar80 * vec3<f32>( nodeVar84.y ) ) ) * vec3<f32>( nodeVar81 ) ) );
		nodeVar86 = textureSampleCompare( nodeUniform31, nodeUniform31_sampler, vec3<f32>( nodeVar85.x, ( - nodeVar85.y ), nodeVar85.z ), nodeVar73 );
		nodeVar87 = vogelDiskSample( 2, 5, nodeVar78 );
		nodeVar88 = ( nodeVar74 + ( ( ( nodeVar77 * vec3<f32>( nodeVar87.x ) ) + ( nodeVar80 * vec3<f32>( nodeVar87.y ) ) ) * vec3<f32>( nodeVar81 ) ) );
		nodeVar89 = textureSampleCompare( nodeUniform31, nodeUniform31_sampler, vec3<f32>( nodeVar88.x, ( - nodeVar88.y ), nodeVar88.z ), nodeVar73 );
		nodeVar90 = vogelDiskSample( 3, 5, nodeVar78 );
		nodeVar91 = ( nodeVar74 + ( ( ( nodeVar77 * vec3<f32>( nodeVar90.x ) ) + ( nodeVar80 * vec3<f32>( nodeVar90.y ) ) ) * vec3<f32>( nodeVar81 ) ) );
		nodeVar92 = textureSampleCompare( nodeUniform31, nodeUniform31_sampler, vec3<f32>( nodeVar91.x, ( - nodeVar91.y ), nodeVar91.z ), nodeVar73 );
		nodeVar93 = vogelDiskSample( 4, 5, nodeVar78 );
		nodeVar94 = ( nodeVar74 + ( ( ( nodeVar77 * vec3<f32>( nodeVar93.x ) ) + ( nodeVar80 * vec3<f32>( nodeVar93.y ) ) ) * vec3<f32>( nodeVar81 ) ) );
		nodeVar95 = textureSampleCompare( nodeUniform31, nodeUniform31_sampler, vec3<f32>( nodeVar94.x, ( - nodeVar94.y ), nodeVar94.z ), nodeVar73 );
		nodeVar70 = ( ( ( ( ( nodeVar83 + nodeVar86 ) + nodeVar89 ) + nodeVar92 ) + nodeVar95 ) * 0.2 );
		

	}

	nodeVar96 = mix( 1.0, nodeVar70, render.nodeUniform34 );
	nodeVar97 = ( render.nodeUniform25 * vec3<f32>( nodeVar96 ) );

	if ( ( render.nodeUniform35 > 0.0 ) ) {

		nodeVar99 = length( nodeVar67 );
		nodeVar100 = ( nodeVar99 / render.nodeUniform35 );
		nodeVar101 = clamp( ( 1.0 - ( ( ( nodeVar100 * nodeVar100 ) * nodeVar100 ) * nodeVar100 ) ), 0.0, 1.0 );
		nodeVar98 = ( ( 1.0 / max( pow( nodeVar99, render.nodeUniform36 ), 0.01 ) ) * ( nodeVar101 * nodeVar101 ) );

	} else {

		nodeVar98 = ( 1.0 / max( pow( length( nodeVar67 ), render.nodeUniform36 ), 0.01 ) );

	}

	nodeVar102 = ( nodeVar97 * vec3<f32>( nodeVar98 ) );
	nodeVar103 = ( vec3<f32>( clamp( nodeVar69, 0.0, 1.0 ) ) * nodeVar102 );
	nodeVar104 = nodeVar103;
	nodeVar105 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar106 = ( nodeVar104 * nodeVar105 );
	nodeVar107 = ( nodeVar68 + positionViewDirection );
	nodeVar108 = normalize( nodeVar107 );
	nodeVar109 = dot( positionViewDirection, nodeVar108 );
	nodeVar110 = clamp( nodeVar109, 0.0, 1.0 );
	nodeVar111 = exp2( ( ( ( nodeVar110 * -5.55473 ) - 6.98316 ) * nodeVar110 ) );
	nodeVar112 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar111 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar111 ) ) );
	nodeVar113 = ( vec3<f32>( 1.0 ) - nodeVar112 );
	nodeVar114 = nodeVar113;
	nodeVar115 = ( nodeVar106 * nodeVar114 );
	nodeVar116 = ( directDiffuse + nodeVar115 );
	directDiffuse = nodeVar116;
	nodeVar117 = normalize( ( nodeVar68 + positionViewDirection ) );
	nodeVar118 = clamp( dot( positionViewDirection, nodeVar117 ), 0.0, 1.0 );
	nodeVar119 = exp2( ( ( ( nodeVar118 * -5.55473 ) - 6.98316 ) * nodeVar118 ) );
	nodeVar120 = ( Roughness * Roughness );
	nodeVar121 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar119 ) ) ) + vec3<f32>( ( 1.0 * nodeVar119 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar120, clamp( dot( normalView, nodeVar68 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar120, clamp( dot( normalView, nodeVar117 ), 0.0, 1.0 ) ) ) );
	nodeVar122 = ( nodeVar104 * nodeVar121 );
	nodeVar123 = ( nodeVar122 * multiScatteringCompensation );
	nodeVar124 = ( directSpecular + nodeVar123 );
	directSpecular = nodeVar124;
	nodeVar125 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar126 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar127 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar128 = ( SpecularF90 * dfg.y );
	nodeVar129 = ( nodeVar127 + vec3<f32>( nodeVar128 ) );
	nodeVar130 = ( nodeVar125 + nodeVar129 );
	nodeVar125 = nodeVar130;
	nodeVar131 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar132 = nodeVar131;
	nodeVar133 = ( nodeVar132 * vec3<f32>( 0.047619 ) );
	nodeVar134 = ( SpecularColor + nodeVar133 );
	nodeVar135 = ( nodeVar129 * nodeVar134 );
	nodeVar136 = ( dfg.x + dfg.y );
	nodeVar137 = ( 1.0 - nodeVar136 );
	nodeVar138 = nodeVar137;
	nodeVar139 = ( vec3<f32>( nodeVar138 ) * nodeVar134 );
	nodeVar140 = ( vec3<f32>( 1.0 ) - nodeVar139 );
	nodeVar141 = nodeVar140;
	nodeVar142 = ( nodeVar135 / nodeVar141 );
	nodeVar143 = ( nodeVar142 * vec3<f32>( nodeVar138 ) );
	nodeVar144 = ( nodeVar126 + nodeVar143 );
	nodeVar126 = nodeVar144;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar145 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar146 = ( irradiance * nodeVar145 );
	nodeVar147 = ( nodeVar125 + nodeVar126 );
	nodeVar148 = ( vec3<f32>( 1.0 ) - nodeVar147 );
	nodeVar149 = nodeVar148;
	nodeVar150 = ( nodeVar146 * nodeVar149 );
	nodeVar151 = nodeVar150;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar152 = ( indirectDiffuse + nodeVar151 );
	indirectDiffuse = nodeVar152;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar153 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar154 = ( SpecularF90 * dfg.y );
	nodeVar155 = ( nodeVar153 + vec3<f32>( nodeVar154 ) );
	nodeVar156 = ( singleScatteringDielectric + nodeVar155 );
	singleScatteringDielectric = nodeVar156;
	nodeVar157 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar158 = nodeVar157;
	nodeVar159 = ( nodeVar158 * vec3<f32>( 0.047619 ) );
	nodeVar160 = ( SpecularColor + nodeVar159 );
	nodeVar161 = ( nodeVar155 * nodeVar160 );
	nodeVar162 = ( dfg.x + dfg.y );
	nodeVar163 = ( 1.0 - nodeVar162 );
	nodeVar164 = nodeVar163;
	nodeVar165 = ( vec3<f32>( nodeVar164 ) * nodeVar160 );
	nodeVar166 = ( vec3<f32>( 1.0 ) - nodeVar165 );
	nodeVar167 = nodeVar166;
	nodeVar168 = ( nodeVar161 / nodeVar167 );
	nodeVar169 = ( nodeVar168 * vec3<f32>( nodeVar164 ) );
	nodeVar170 = ( multiScatteringDielectric + nodeVar169 );
	multiScatteringDielectric = nodeVar170;
	nodeVar171 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar172 = ( SpecularF90 * dfg.y );
	nodeVar173 = ( nodeVar171 + vec3<f32>( nodeVar172 ) );
	nodeVar174 = ( singleScatteringMetallic + nodeVar173 );
	singleScatteringMetallic = nodeVar174;
	nodeVar175 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar176 = nodeVar175;
	nodeVar177 = ( nodeVar176 * vec3<f32>( 0.047619 ) );
	nodeVar178 = ( DiffuseColor.xyz + nodeVar177 );
	nodeVar179 = ( nodeVar173 * nodeVar178 );
	nodeVar180 = ( dfg.x + dfg.y );
	nodeVar181 = ( 1.0 - nodeVar180 );
	nodeVar182 = nodeVar181;
	nodeVar183 = ( vec3<f32>( nodeVar182 ) * nodeVar178 );
	nodeVar184 = ( vec3<f32>( 1.0 ) - nodeVar183 );
	nodeVar185 = nodeVar184;
	nodeVar186 = ( nodeVar179 / nodeVar185 );
	nodeVar187 = ( nodeVar186 * vec3<f32>( nodeVar182 ) );
	nodeVar188 = ( multiScatteringMetallic + nodeVar187 );
	multiScatteringMetallic = nodeVar188;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar189 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar190 = ( radiance * nodeVar189 );
	nodeVar191 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar192 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar193 = ( nodeVar191 * nodeVar192 );
	nodeVar194 = ( nodeVar190 + nodeVar193 );
	nodeVar195 = nodeVar194;
	nodeVar196 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar197 = ( vec3<f32>( 1.0 ) - nodeVar196 );
	nodeVar198 = nodeVar197;
	nodeVar199 = ( DiffuseContribution * nodeVar198 );
	nodeVar200 = ( nodeVar199 * nodeVar192 );
	nodeVar201 = nodeVar200;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar202 = ( indirectSpecular + nodeVar195 );
	indirectSpecular = nodeVar202;
	nodeVar203 = ( indirectDiffuse + nodeVar201 );
	indirectDiffuse = nodeVar203;
	ambientOcclusion = 1.0;
	nodeVar204 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar204;
	nodeVar205 = dot( normalView, positionViewDirection );
	nodeVar206 = ( clamp( nodeVar205, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar207 = ( Roughness * -16.0 );
	nodeVar208 = ( 1.0 - nodeVar207 );
	nodeVar209 = nodeVar208;
	nodeVar210 = ( - nodeVar209 );
	nodeVar211 = exp2( nodeVar210 );
	nodeVar212 = pow( nodeVar206, nodeVar211 );
	nodeVar213 = ( 1.0 - nodeVar212 );
	nodeVar214 = nodeVar213;
	nodeVar215 = ( ambientOcclusion - nodeVar214 );
	nodeVar216 = ( indirectSpecular * vec3<f32>( clamp( nodeVar215, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar216;
	nodeVar217 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar217;
	nodeVar218 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar218;
	nodeVar219 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar219;
	nodeVar220 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar220;

	// result

	output.color = nodeVar220;

	return output;

}
