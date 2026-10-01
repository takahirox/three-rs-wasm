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
	nodeUniform14 : f32,
	nodeUniform21 : f32,
	nodeUniform26 : mat4x4<f32>,
	nodeUniform27 : f32,
	nodeUniform34 : f32,
	nodeUniform16 : f32,
	nodeUniform15 : f32,
	nodeUniform17 : f32,
	nodeUniform19 : f32,
	nodeUniform20 : vec2<f32>,
	nodeUniform29 : f32,
	nodeUniform28 : f32,
	nodeUniform30 : f32,
	nodeUniform32 : f32,
	nodeUniform33 : vec2<f32>
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
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : vec2<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : vec2<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : vec2<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : vec2<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : vec2<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : f32;
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
var<private> nodeVar116 : f32;
var<private> nodeVar117 : f32;
var<private> nodeVar118 : f32;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : f32;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : f32;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : f32;
var<private> nodeVar161 : f32;
var<private> nodeVar162 : f32;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : f32;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : vec3<f32>;
var<private> nodeVar177 : vec3<f32>;
var<private> nodeVar178 : f32;
var<private> nodeVar179 : f32;
var<private> nodeVar180 : f32;
var<private> nodeVar181 : vec3<f32>;
var<private> nodeVar182 : vec3<f32>;
var<private> nodeVar183 : vec3<f32>;
var<private> nodeVar184 : vec3<f32>;
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar187 : vec3<f32>;
var<private> nodeVar188 : vec3<f32>;
var<private> nodeVar189 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : vec3<f32>;
var<private> nodeVar193 : vec3<f32>;
var<private> nodeVar194 : vec3<f32>;
var<private> nodeVar195 : vec3<f32>;
var<private> nodeVar196 : vec3<f32>;
var<private> nodeVar197 : vec3<f32>;
var<private> nodeVar198 : vec3<f32>;
var<private> nodeVar199 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar200 : vec3<f32>;
var<private> nodeVar201 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar202 : vec3<f32>;
var<private> nodeVar203 : f32;
var<private> nodeVar204 : f32;
var<private> nodeVar205 : f32;
var<private> nodeVar206 : f32;
var<private> nodeVar207 : f32;
var<private> nodeVar208 : f32;
var<private> nodeVar209 : f32;
var<private> nodeVar210 : f32;
var<private> nodeVar211 : f32;
var<private> nodeVar212 : f32;
var<private> nodeVar213 : f32;
var<private> nodeVar214 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar215 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar216 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar217 : vec3<f32>;
var<private> nodeVar218 : vec4<f32>;

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

	if ( ( render.nodeUniform22 > 0.0 ) ) {

		nodeVar40 = length( nodeVar9 );
		nodeVar41 = ( nodeVar40 / render.nodeUniform22 );
		nodeVar42 = clamp( ( 1.0 - ( ( ( nodeVar41 * nodeVar41 ) * nodeVar41 ) * nodeVar41 ) ), 0.0, 1.0 );
		nodeVar39 = ( ( 1.0 / max( pow( nodeVar40, render.nodeUniform23 ), 0.01 ) ) * ( nodeVar42 * nodeVar42 ) );

	} else {

		nodeVar39 = ( 1.0 / max( pow( length( nodeVar9 ), render.nodeUniform23 ), 0.01 ) );

	}

	nodeVar43 = ( ( render.nodeUniform11 * vec3<f32>( nodeVar38 ) ) * vec3<f32>( nodeVar39 ) );
	nodeVar44 = ( vec3<f32>( clamp( nodeVar11, 0.0, 1.0 ) ) * nodeVar43 );
	nodeVar45 = nodeVar44;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar46 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar47 = ( nodeVar45 * nodeVar46 );
	nodeVar48 = ( nodeVar10 + positionViewDirection );
	nodeVar49 = normalize( nodeVar48 );
	nodeVar50 = dot( positionViewDirection, nodeVar49 );
	nodeVar51 = clamp( nodeVar50, 0.0, 1.0 );
	nodeVar52 = exp2( ( ( ( nodeVar51 * -5.55473 ) - 6.98316 ) * nodeVar51 ) );
	nodeVar53 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar52 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar52 ) ) );
	nodeVar54 = ( vec3<f32>( 1.0 ) - nodeVar53 );
	nodeVar55 = nodeVar54;
	nodeVar56 = ( nodeVar47 * nodeVar55 );
	nodeVar57 = ( directDiffuse + nodeVar56 );
	directDiffuse = nodeVar57;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar58 = normalize( ( nodeVar10 + positionViewDirection ) );
	nodeVar59 = clamp( dot( positionViewDirection, nodeVar58 ), 0.0, 1.0 );
	nodeVar60 = exp2( ( ( ( nodeVar59 * -5.55473 ) - 6.98316 ) * nodeVar59 ) );
	nodeVar61 = ( Roughness * Roughness );
	nodeVar62 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar60 ) ) ) + vec3<f32>( ( 1.0 * nodeVar60 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar61, clamp( dot( normalView, nodeVar10 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar61, clamp( dot( normalView, nodeVar58 ), 0.0, 1.0 ) ) ) );
	nodeVar63 = ( nodeVar45 * nodeVar62 );
	nodeVar64 = ( nodeVar63 * multiScatteringCompensation );
	nodeVar65 = ( directSpecular + nodeVar64 );
	directSpecular = nodeVar65;
	nodeVar66 = ( render.nodeUniform24 - v_positionView );
	nodeVar67 = normalize( nodeVar66 );
	nodeVar68 = dot( normalView, nodeVar67 );
	shadowPositionWorld = v_positionWorld;
	let nodeConst2 = ( render.nodeUniform26 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform27 ) ) ), 1.0 ) ).xyz;
	let nodeConst3 = abs( nodeConst2 );
	nodeVar69 = 1.0;
	nodeVar70 = max( max( nodeConst3.x, nodeConst3.y ), nodeConst3.z );

	if ( ( ( ( nodeVar70 - render.nodeUniform28 ) <= 0.0 ) && ( ( nodeVar70 - render.nodeUniform29 ) >= 0.0 ) ) ) {

		nodeVar71 = ( - nodeVar70 );
		nodeVar72 = ( ( ( render.nodeUniform29 + nodeVar71 ) * render.nodeUniform28 ) / ( ( render.nodeUniform28 - render.nodeUniform29 ) * nodeVar71 ) );
		nodeVar72 = ( nodeVar72 + render.nodeUniform30 );
		nodeVar73 = normalize( nodeConst2 );
		nodeVar75 = abs( nodeVar73 );

		if ( ( nodeVar75.x > nodeVar75.z ) ) {

			nodeVar74 = vec3<f32>( 0.0, 1.0, 0.0 );

		} else {

			nodeVar74 = vec3<f32>( 1.0, 0.0, 0.0 );

		}

		nodeVar76 = normalize( cross( nodeVar73, nodeVar74 ) );
		nodeVar77 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar78 = vogelDiskSample( 0, 5, nodeVar77 );
		nodeVar79 = cross( nodeVar73, nodeVar76 );
		nodeVar80 = ( render.nodeUniform32 / render.nodeUniform33.x );
		nodeVar81 = ( nodeVar73 + ( ( ( nodeVar76 * vec3<f32>( nodeVar78.x ) ) + ( nodeVar79 * vec3<f32>( nodeVar78.y ) ) ) * vec3<f32>( nodeVar80 ) ) );
		nodeVar82 = textureSampleCompare( nodeUniform31, nodeUniform31_sampler, vec3<f32>( nodeVar81.x, ( - nodeVar81.y ), nodeVar81.z ), nodeVar72 );
		nodeVar83 = vogelDiskSample( 1, 5, nodeVar77 );
		nodeVar84 = ( nodeVar73 + ( ( ( nodeVar76 * vec3<f32>( nodeVar83.x ) ) + ( nodeVar79 * vec3<f32>( nodeVar83.y ) ) ) * vec3<f32>( nodeVar80 ) ) );
		nodeVar85 = textureSampleCompare( nodeUniform31, nodeUniform31_sampler, vec3<f32>( nodeVar84.x, ( - nodeVar84.y ), nodeVar84.z ), nodeVar72 );
		nodeVar86 = vogelDiskSample( 2, 5, nodeVar77 );
		nodeVar87 = ( nodeVar73 + ( ( ( nodeVar76 * vec3<f32>( nodeVar86.x ) ) + ( nodeVar79 * vec3<f32>( nodeVar86.y ) ) ) * vec3<f32>( nodeVar80 ) ) );
		nodeVar88 = textureSampleCompare( nodeUniform31, nodeUniform31_sampler, vec3<f32>( nodeVar87.x, ( - nodeVar87.y ), nodeVar87.z ), nodeVar72 );
		nodeVar89 = vogelDiskSample( 3, 5, nodeVar77 );
		nodeVar90 = ( nodeVar73 + ( ( ( nodeVar76 * vec3<f32>( nodeVar89.x ) ) + ( nodeVar79 * vec3<f32>( nodeVar89.y ) ) ) * vec3<f32>( nodeVar80 ) ) );
		nodeVar91 = textureSampleCompare( nodeUniform31, nodeUniform31_sampler, vec3<f32>( nodeVar90.x, ( - nodeVar90.y ), nodeVar90.z ), nodeVar72 );
		nodeVar92 = vogelDiskSample( 4, 5, nodeVar77 );
		nodeVar93 = ( nodeVar73 + ( ( ( nodeVar76 * vec3<f32>( nodeVar92.x ) ) + ( nodeVar79 * vec3<f32>( nodeVar92.y ) ) ) * vec3<f32>( nodeVar80 ) ) );
		nodeVar94 = textureSampleCompare( nodeUniform31, nodeUniform31_sampler, vec3<f32>( nodeVar93.x, ( - nodeVar93.y ), nodeVar93.z ), nodeVar72 );
		nodeVar69 = ( ( ( ( ( nodeVar82 + nodeVar85 ) + nodeVar88 ) + nodeVar91 ) + nodeVar94 ) * 0.2 );
		

	}

	nodeVar95 = mix( 1.0, nodeVar69, render.nodeUniform34 );

	if ( ( render.nodeUniform35 > 0.0 ) ) {

		nodeVar97 = length( nodeVar66 );
		nodeVar98 = ( nodeVar97 / render.nodeUniform35 );
		nodeVar99 = clamp( ( 1.0 - ( ( ( nodeVar98 * nodeVar98 ) * nodeVar98 ) * nodeVar98 ) ), 0.0, 1.0 );
		nodeVar96 = ( ( 1.0 / max( pow( nodeVar97, render.nodeUniform36 ), 0.01 ) ) * ( nodeVar99 * nodeVar99 ) );

	} else {

		nodeVar96 = ( 1.0 / max( pow( length( nodeVar66 ), render.nodeUniform36 ), 0.01 ) );

	}

	nodeVar100 = ( ( render.nodeUniform25 * vec3<f32>( nodeVar95 ) ) * vec3<f32>( nodeVar96 ) );
	nodeVar101 = ( vec3<f32>( clamp( nodeVar68, 0.0, 1.0 ) ) * nodeVar100 );
	nodeVar102 = nodeVar101;
	nodeVar103 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar104 = ( nodeVar102 * nodeVar103 );
	nodeVar105 = ( nodeVar67 + positionViewDirection );
	nodeVar106 = normalize( nodeVar105 );
	nodeVar107 = dot( positionViewDirection, nodeVar106 );
	nodeVar108 = clamp( nodeVar107, 0.0, 1.0 );
	nodeVar109 = exp2( ( ( ( nodeVar108 * -5.55473 ) - 6.98316 ) * nodeVar108 ) );
	nodeVar110 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar109 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar109 ) ) );
	nodeVar111 = ( vec3<f32>( 1.0 ) - nodeVar110 );
	nodeVar112 = nodeVar111;
	nodeVar113 = ( nodeVar104 * nodeVar112 );
	nodeVar114 = ( directDiffuse + nodeVar113 );
	directDiffuse = nodeVar114;
	nodeVar115 = normalize( ( nodeVar67 + positionViewDirection ) );
	nodeVar116 = clamp( dot( positionViewDirection, nodeVar115 ), 0.0, 1.0 );
	nodeVar117 = exp2( ( ( ( nodeVar116 * -5.55473 ) - 6.98316 ) * nodeVar116 ) );
	nodeVar118 = ( Roughness * Roughness );
	nodeVar119 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar117 ) ) ) + vec3<f32>( ( 1.0 * nodeVar117 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar118, clamp( dot( normalView, nodeVar67 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar118, clamp( dot( normalView, nodeVar115 ), 0.0, 1.0 ) ) ) );
	nodeVar120 = ( nodeVar102 * nodeVar119 );
	nodeVar121 = ( nodeVar120 * multiScatteringCompensation );
	nodeVar122 = ( directSpecular + nodeVar121 );
	directSpecular = nodeVar122;
	nodeVar123 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar124 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar125 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar126 = ( SpecularF90 * dfg.y );
	nodeVar127 = ( nodeVar125 + vec3<f32>( nodeVar126 ) );
	nodeVar128 = ( nodeVar123 + nodeVar127 );
	nodeVar123 = nodeVar128;
	nodeVar129 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar130 = nodeVar129;
	nodeVar131 = ( nodeVar130 * vec3<f32>( 0.047619 ) );
	nodeVar132 = ( SpecularColor + nodeVar131 );
	nodeVar133 = ( nodeVar127 * nodeVar132 );
	nodeVar134 = ( dfg.x + dfg.y );
	nodeVar135 = ( 1.0 - nodeVar134 );
	nodeVar136 = nodeVar135;
	nodeVar137 = ( vec3<f32>( nodeVar136 ) * nodeVar132 );
	nodeVar138 = ( vec3<f32>( 1.0 ) - nodeVar137 );
	nodeVar139 = nodeVar138;
	nodeVar140 = ( nodeVar133 / nodeVar139 );
	nodeVar141 = ( nodeVar140 * vec3<f32>( nodeVar136 ) );
	nodeVar142 = ( nodeVar124 + nodeVar141 );
	nodeVar124 = nodeVar142;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar143 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar144 = ( irradiance * nodeVar143 );
	nodeVar145 = ( nodeVar123 + nodeVar124 );
	nodeVar146 = ( vec3<f32>( 1.0 ) - nodeVar145 );
	nodeVar147 = nodeVar146;
	nodeVar148 = ( nodeVar144 * nodeVar147 );
	nodeVar149 = nodeVar148;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar150 = ( indirectDiffuse + nodeVar149 );
	indirectDiffuse = nodeVar150;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar151 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar152 = ( SpecularF90 * dfg.y );
	nodeVar153 = ( nodeVar151 + vec3<f32>( nodeVar152 ) );
	nodeVar154 = ( singleScatteringDielectric + nodeVar153 );
	singleScatteringDielectric = nodeVar154;
	nodeVar155 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar156 = nodeVar155;
	nodeVar157 = ( nodeVar156 * vec3<f32>( 0.047619 ) );
	nodeVar158 = ( SpecularColor + nodeVar157 );
	nodeVar159 = ( nodeVar153 * nodeVar158 );
	nodeVar160 = ( dfg.x + dfg.y );
	nodeVar161 = ( 1.0 - nodeVar160 );
	nodeVar162 = nodeVar161;
	nodeVar163 = ( vec3<f32>( nodeVar162 ) * nodeVar158 );
	nodeVar164 = ( vec3<f32>( 1.0 ) - nodeVar163 );
	nodeVar165 = nodeVar164;
	nodeVar166 = ( nodeVar159 / nodeVar165 );
	nodeVar167 = ( nodeVar166 * vec3<f32>( nodeVar162 ) );
	nodeVar168 = ( multiScatteringDielectric + nodeVar167 );
	multiScatteringDielectric = nodeVar168;
	nodeVar169 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar170 = ( SpecularF90 * dfg.y );
	nodeVar171 = ( nodeVar169 + vec3<f32>( nodeVar170 ) );
	nodeVar172 = ( singleScatteringMetallic + nodeVar171 );
	singleScatteringMetallic = nodeVar172;
	nodeVar173 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar174 = nodeVar173;
	nodeVar175 = ( nodeVar174 * vec3<f32>( 0.047619 ) );
	nodeVar176 = ( DiffuseColor.xyz + nodeVar175 );
	nodeVar177 = ( nodeVar171 * nodeVar176 );
	nodeVar178 = ( dfg.x + dfg.y );
	nodeVar179 = ( 1.0 - nodeVar178 );
	nodeVar180 = nodeVar179;
	nodeVar181 = ( vec3<f32>( nodeVar180 ) * nodeVar176 );
	nodeVar182 = ( vec3<f32>( 1.0 ) - nodeVar181 );
	nodeVar183 = nodeVar182;
	nodeVar184 = ( nodeVar177 / nodeVar183 );
	nodeVar185 = ( nodeVar184 * vec3<f32>( nodeVar180 ) );
	nodeVar186 = ( multiScatteringMetallic + nodeVar185 );
	multiScatteringMetallic = nodeVar186;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar187 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar188 = ( radiance * nodeVar187 );
	nodeVar189 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar190 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar191 = ( nodeVar189 * nodeVar190 );
	nodeVar192 = ( nodeVar188 + nodeVar191 );
	nodeVar193 = nodeVar192;
	nodeVar194 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar195 = ( vec3<f32>( 1.0 ) - nodeVar194 );
	nodeVar196 = nodeVar195;
	nodeVar197 = ( DiffuseContribution * nodeVar196 );
	nodeVar198 = ( nodeVar197 * nodeVar190 );
	nodeVar199 = nodeVar198;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar200 = ( indirectSpecular + nodeVar193 );
	indirectSpecular = nodeVar200;
	nodeVar201 = ( indirectDiffuse + nodeVar199 );
	indirectDiffuse = nodeVar201;
	ambientOcclusion = 1.0;
	nodeVar202 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar202;
	nodeVar203 = dot( normalView, positionViewDirection );
	nodeVar204 = ( clamp( nodeVar203, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar205 = ( Roughness * -16.0 );
	nodeVar206 = ( 1.0 - nodeVar205 );
	nodeVar207 = nodeVar206;
	nodeVar208 = ( - nodeVar207 );
	nodeVar209 = exp2( nodeVar208 );
	nodeVar210 = pow( nodeVar204, nodeVar209 );
	nodeVar211 = ( 1.0 - nodeVar210 );
	nodeVar212 = nodeVar211;
	nodeVar213 = ( ambientOcclusion - nodeVar212 );
	nodeVar214 = ( indirectSpecular * vec3<f32>( clamp( nodeVar213, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar214;
	nodeVar215 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar215;
	nodeVar216 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar216;
	nodeVar217 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar217;
	nodeVar218 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar218;

	// result

	output.color = nodeVar218;

	return output;

}
