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
@binding( 5 ) @group( 1 ) var nodeUniform24_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform24 : texture_3d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform28 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform22 : f32,
	nodeUniform23 : f32,
	nodeUniform10 : vec3<f32>,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform14 : f32,
	nodeUniform21 : f32,
	nodeUniform16 : f32,
	nodeUniform15 : f32,
	nodeUniform17 : f32,
	nodeUniform19 : f32,
	nodeUniform20 : vec2<f32>
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
var<private> irradiance : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec4<f32>;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec4<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec4<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec4<f32>;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec4<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : f32;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec4<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : f32;
var<private> nodeVar118 : f32;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec4<f32>;
var<private> nodeVar121 : array< vec3<f32>, 9 >;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : f32;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : f32;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : f32;
var<private> nodeVar164 : f32;
var<private> nodeVar165 : f32;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : f32;
var<private> nodeVar174 : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : vec3<f32>;
var<private> nodeVar177 : vec3<f32>;
var<private> nodeVar178 : vec3<f32>;
var<private> nodeVar179 : vec3<f32>;
var<private> nodeVar180 : vec3<f32>;
var<private> nodeVar181 : f32;
var<private> nodeVar182 : f32;
var<private> nodeVar183 : f32;
var<private> nodeVar184 : vec3<f32>;
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> nodeVar187 : vec3<f32>;
var<private> nodeVar188 : vec3<f32>;
var<private> nodeVar189 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar193 : vec3<f32>;
var<private> nodeVar194 : vec3<f32>;
var<private> nodeVar195 : vec3<f32>;
var<private> nodeVar196 : vec3<f32>;
var<private> nodeVar197 : vec3<f32>;
var<private> nodeVar198 : vec3<f32>;
var<private> nodeVar199 : vec3<f32>;
var<private> nodeVar200 : vec3<f32>;
var<private> nodeVar201 : vec3<f32>;
var<private> nodeVar202 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar203 : vec3<f32>;
var<private> nodeVar204 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar205 : vec3<f32>;
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
var<private> nodeVar216 : f32;
var<private> nodeVar217 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar218 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar219 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar220 : vec3<f32>;
var<private> nodeVar221 : vec4<f32>;

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
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar66 = ( object.nodeUniform25 - object.nodeUniform26 );
	nodeVar67 = ( object.nodeUniform27 - vec3<f32>( 1.0 ) );
	nodeVar68 = ( nodeVar66 / nodeVar67 );
	nodeVar69 = ( normalWorld * nodeVar68 );
	nodeVar70 = ( nodeVar69 * vec3<f32>( 0.5 ) );
	nodeVar71 = ( v_positionWorld + nodeVar70 );
	nodeVar72 = ( nodeVar71 - object.nodeUniform26 );
	nodeVar73 = ( nodeVar72 / nodeVar66 );
	nodeVar74 = ( clamp( nodeVar73, vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) ) * nodeVar67 );
	nodeVar75 = ( nodeVar74 / object.nodeUniform27 );
	nodeVar76 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform27 );
	nodeVar77 = ( nodeVar75 + nodeVar76 );
	nodeVar78 = ( nodeVar77.z * object.nodeUniform27.z );
	nodeVar79 = ( nodeVar78 + 1.0 );
	nodeVar80 = ( object.nodeUniform27.z + 2.0 );
	nodeVar81 = ( nodeVar80 * 0.0 );
	nodeVar82 = ( nodeVar79 + nodeVar81 );
	nodeVar83 = ( nodeVar80 * 7.0 );
	nodeVar84 = ( nodeVar82 / nodeVar83 );
	nodeVar85 = vec3<f32>( nodeVar77.xy, nodeVar84 );
	nodeVar86 = textureSample( nodeUniform24, nodeUniform24_sampler, nodeVar85 );
	nodeVar87 = ( nodeVar80 * 1.0 );
	nodeVar88 = ( nodeVar79 + nodeVar87 );
	nodeVar89 = ( nodeVar88 / nodeVar83 );
	nodeVar90 = vec3<f32>( nodeVar77.xy, nodeVar89 );
	nodeVar91 = textureSample( nodeUniform24, nodeUniform24_sampler, nodeVar90 );
	nodeVar92 = vec3<f32>( nodeVar86.w, nodeVar91.xy );
	nodeVar93 = ( nodeVar80 * 2.0 );
	nodeVar94 = ( nodeVar79 + nodeVar93 );
	nodeVar95 = ( nodeVar94 / nodeVar83 );
	nodeVar96 = vec3<f32>( nodeVar77.xy, nodeVar95 );
	nodeVar97 = textureSample( nodeUniform24, nodeUniform24_sampler, nodeVar96 );
	nodeVar98 = vec3<f32>( nodeVar91.zw, nodeVar97.x );
	nodeVar99 = ( nodeVar80 * 3.0 );
	nodeVar100 = ( nodeVar79 + nodeVar99 );
	nodeVar101 = ( nodeVar100 / nodeVar83 );
	nodeVar102 = vec3<f32>( nodeVar77.xy, nodeVar101 );
	nodeVar103 = textureSample( nodeUniform24, nodeUniform24_sampler, nodeVar102 );
	nodeVar104 = ( nodeVar80 * 4.0 );
	nodeVar105 = ( nodeVar79 + nodeVar104 );
	nodeVar106 = ( nodeVar105 / nodeVar83 );
	nodeVar107 = vec3<f32>( nodeVar77.xy, nodeVar106 );
	nodeVar108 = textureSample( nodeUniform24, nodeUniform24_sampler, nodeVar107 );
	nodeVar109 = vec3<f32>( nodeVar103.w, nodeVar108.xy );
	nodeVar110 = ( nodeVar80 * 5.0 );
	nodeVar111 = ( nodeVar79 + nodeVar110 );
	nodeVar112 = ( nodeVar111 / nodeVar83 );
	nodeVar113 = vec3<f32>( nodeVar77.xy, nodeVar112 );
	nodeVar114 = textureSample( nodeUniform24, nodeUniform24_sampler, nodeVar113 );
	nodeVar115 = vec3<f32>( nodeVar108.zw, nodeVar114.x );
	nodeVar116 = ( nodeVar80 * 6.0 );
	nodeVar117 = ( nodeVar79 + nodeVar116 );
	nodeVar118 = ( nodeVar117 / nodeVar83 );
	nodeVar119 = vec3<f32>( nodeVar77.xy, nodeVar118 );
	nodeVar120 = textureSample( nodeUniform24, nodeUniform24_sampler, nodeVar119 );
	nodeVar121 = array< vec3<f32>, 9 >( nodeVar86.xyz, nodeVar92, nodeVar98, nodeVar97.yzw, nodeVar103.xyz, nodeVar109, nodeVar115, nodeVar114.yzw, nodeVar120.xyz );
	nodeVar122 = ( ( ( ( ( ( ( ( ( nodeVar121[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeVar121[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeVar121[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar121[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeVar121[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeVar121[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeVar121[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeVar121[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar121[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) );
	nodeVar123 = max( nodeVar122, vec3<f32>( 0.0, 0.0, 0.0 ) );
	nodeVar124 = ( nodeVar123 * vec3<f32>( object.nodeUniform28 ) );
	nodeVar125 = ( irradiance + nodeVar124 );
	irradiance = nodeVar125;
	nodeVar126 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar127 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar128 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar129 = ( SpecularF90 * dfg.y );
	nodeVar130 = ( nodeVar128 + vec3<f32>( nodeVar129 ) );
	nodeVar131 = ( nodeVar126 + nodeVar130 );
	nodeVar126 = nodeVar131;
	nodeVar132 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar133 = nodeVar132;
	nodeVar134 = ( nodeVar133 * vec3<f32>( 0.047619 ) );
	nodeVar135 = ( SpecularColor + nodeVar134 );
	nodeVar136 = ( nodeVar130 * nodeVar135 );
	nodeVar137 = ( dfg.x + dfg.y );
	nodeVar138 = ( 1.0 - nodeVar137 );
	nodeVar139 = nodeVar138;
	nodeVar140 = ( vec3<f32>( nodeVar139 ) * nodeVar135 );
	nodeVar141 = ( vec3<f32>( 1.0 ) - nodeVar140 );
	nodeVar142 = nodeVar141;
	nodeVar143 = ( nodeVar136 / nodeVar142 );
	nodeVar144 = ( nodeVar143 * vec3<f32>( nodeVar139 ) );
	nodeVar145 = ( nodeVar127 + nodeVar144 );
	nodeVar127 = nodeVar145;
	nodeVar146 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar147 = ( irradiance * nodeVar146 );
	nodeVar148 = ( nodeVar126 + nodeVar127 );
	nodeVar149 = ( vec3<f32>( 1.0 ) - nodeVar148 );
	nodeVar150 = nodeVar149;
	nodeVar151 = ( nodeVar147 * nodeVar150 );
	nodeVar152 = nodeVar151;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar153 = ( indirectDiffuse + nodeVar152 );
	indirectDiffuse = nodeVar153;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar154 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar155 = ( SpecularF90 * dfg.y );
	nodeVar156 = ( nodeVar154 + vec3<f32>( nodeVar155 ) );
	nodeVar157 = ( singleScatteringDielectric + nodeVar156 );
	singleScatteringDielectric = nodeVar157;
	nodeVar158 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar159 = nodeVar158;
	nodeVar160 = ( nodeVar159 * vec3<f32>( 0.047619 ) );
	nodeVar161 = ( SpecularColor + nodeVar160 );
	nodeVar162 = ( nodeVar156 * nodeVar161 );
	nodeVar163 = ( dfg.x + dfg.y );
	nodeVar164 = ( 1.0 - nodeVar163 );
	nodeVar165 = nodeVar164;
	nodeVar166 = ( vec3<f32>( nodeVar165 ) * nodeVar161 );
	nodeVar167 = ( vec3<f32>( 1.0 ) - nodeVar166 );
	nodeVar168 = nodeVar167;
	nodeVar169 = ( nodeVar162 / nodeVar168 );
	nodeVar170 = ( nodeVar169 * vec3<f32>( nodeVar165 ) );
	nodeVar171 = ( multiScatteringDielectric + nodeVar170 );
	multiScatteringDielectric = nodeVar171;
	nodeVar172 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar173 = ( SpecularF90 * dfg.y );
	nodeVar174 = ( nodeVar172 + vec3<f32>( nodeVar173 ) );
	nodeVar175 = ( singleScatteringMetallic + nodeVar174 );
	singleScatteringMetallic = nodeVar175;
	nodeVar176 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar177 = nodeVar176;
	nodeVar178 = ( nodeVar177 * vec3<f32>( 0.047619 ) );
	nodeVar179 = ( DiffuseColor.xyz + nodeVar178 );
	nodeVar180 = ( nodeVar174 * nodeVar179 );
	nodeVar181 = ( dfg.x + dfg.y );
	nodeVar182 = ( 1.0 - nodeVar181 );
	nodeVar183 = nodeVar182;
	nodeVar184 = ( vec3<f32>( nodeVar183 ) * nodeVar179 );
	nodeVar185 = ( vec3<f32>( 1.0 ) - nodeVar184 );
	nodeVar186 = nodeVar185;
	nodeVar187 = ( nodeVar180 / nodeVar186 );
	nodeVar188 = ( nodeVar187 * vec3<f32>( nodeVar183 ) );
	nodeVar189 = ( multiScatteringMetallic + nodeVar188 );
	multiScatteringMetallic = nodeVar189;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar190 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar191 = ( radiance * nodeVar190 );
	nodeVar192 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar193 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar194 = ( nodeVar192 * nodeVar193 );
	nodeVar195 = ( nodeVar191 + nodeVar194 );
	nodeVar196 = nodeVar195;
	nodeVar197 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar198 = ( vec3<f32>( 1.0 ) - nodeVar197 );
	nodeVar199 = nodeVar198;
	nodeVar200 = ( DiffuseContribution * nodeVar199 );
	nodeVar201 = ( nodeVar200 * nodeVar193 );
	nodeVar202 = nodeVar201;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar203 = ( indirectSpecular + nodeVar196 );
	indirectSpecular = nodeVar203;
	nodeVar204 = ( indirectDiffuse + nodeVar202 );
	indirectDiffuse = nodeVar204;
	ambientOcclusion = 1.0;
	nodeVar205 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar205;
	nodeVar206 = dot( normalView, positionViewDirection );
	nodeVar207 = ( clamp( nodeVar206, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar208 = ( Roughness * -16.0 );
	nodeVar209 = ( 1.0 - nodeVar208 );
	nodeVar210 = nodeVar209;
	nodeVar211 = ( - nodeVar210 );
	nodeVar212 = exp2( nodeVar211 );
	nodeVar213 = pow( nodeVar207, nodeVar212 );
	nodeVar214 = ( 1.0 - nodeVar213 );
	nodeVar215 = nodeVar214;
	nodeVar216 = ( ambientOcclusion - nodeVar215 );
	nodeVar217 = ( indirectSpecular * vec3<f32>( clamp( nodeVar216, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar217;
	nodeVar218 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar218;
	nodeVar219 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar219;
	nodeVar220 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar220;
	nodeVar221 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar221;

	// result

	output.color = nodeVar221;

	return output;

}
