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
@binding( 3 ) @group( 1 ) var nodeUniform18_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform18 : texture_depth_cube;
@binding( 5 ) @group( 1 ) var nodeUniform29_sampler : sampler_comparison;
@binding( 6 ) @group( 1 ) var nodeUniform29 : texture_depth_2d;
@binding( 7 ) @group( 1 ) var nodeUniform39_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform39 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform40 : mat4x4<f32>,
	nodeUniform41 : mat4x4<f32>,
	nodeUniform43 : mat4x4<f32>,
	nodeUniform44 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform42 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform22 : f32,
	nodeUniform23 : f32,
	nodeUniform33 : f32,
	nodeUniform34 : f32,
	nodeUniform37 : f32,
	nodeUniform38 : f32,
	nodeUniform26 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform35 : vec3<f32>,
	nodeUniform36 : vec3<f32>,
	nodeUniform25 : mat4x4<f32>,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform16 : f32,
	nodeUniform15 : f32,
	nodeUniform14 : f32,
	nodeUniform17 : f32,
	nodeUniform19 : f32,
	nodeUniform20 : vec2<f32>,
	nodeUniform21 : f32,
	nodeUniform27 : f32,
	nodeUniform28 : f32,
	nodeUniform30 : f32,
	nodeUniform31 : vec2<f32>,
	nodeUniform32 : f32
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
var<private> nodeVar70 : vec4<f32>;
var<private> nodeVar71 : vec4<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<bool>;
var<private> nodeVar77 : bool;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : vec4<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : vec2<f32>;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : vec2<f32>;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : vec2<f32>;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : vec2<f32>;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : vec2<f32>;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : vec4<f32>;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : vec4<f32>;
var<private> nodeVar105 : vec4<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec4<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : f32;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec4<f32>;
var<private> nodeVar117 : vec4<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : f32;
var<private> nodeVar120 : f32;
var<private> nodeVar121 : f32;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec4<f32>;
var<private> nodeVar124 : vec4<f32>;
var<private> nodeVar125 : vec4<f32>;
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
var<private> irradiance : vec3<f32>;
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
var<private> modelViewMatrix : mat4x4<f32>;
var<private> nodeVar221 : vec4<f32>;
var<private> nodeVar222 : vec4<f32>;
var<private> nodeVar223 : vec2<f32>;

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
	@location( 1 ) positionLocal : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32>,
	@location( 4 ) v_positionViewDirection : vec3<f32>,
	@location( 5 ) positionPrevious : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputType {

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
	nodeVar71 = ( render.nodeUniform25 * vec4<f32>( v_positionWorld, 1.0 ) );
	nodeVar72 = ( nodeVar71.xyz / vec3<f32>( nodeVar71.w ) );
	nodeVar73 = ( nodeVar72 * vec3<f32>( 2.0 ) );
	nodeVar74 = ( nodeVar73 - vec3<f32>( 1.0 ) );
	nodeVar75 = abs( nodeVar74 );
	nodeVar76 = ( nodeVar75 < vec3<f32>( 1.0 ) );
	nodeVar77 = all( nodeVar76 );

	if ( nodeVar77 ) {

		shadowPositionWorld = v_positionWorld;
		nodeVar79 = ( render.nodeUniform25 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform27 ) ) ), 1.0 ) );
		nodeVar80 = ( nodeVar79.xyz / vec3<f32>( nodeVar79.w ) );
		nodeVar81 = vec3<f32>( nodeVar80.x, ( 1.0 - nodeVar80.y ), ( nodeVar80.z + render.nodeUniform28 ) );

		if ( ( ( ( ( ( nodeVar81.x >= 0.0 ) && ( nodeVar81.x <= 1.0 ) ) && ( nodeVar81.y >= 0.0 ) ) && ( nodeVar81.y <= 1.0 ) ) && ( nodeVar81.z <= 1.0 ) ) ) {

			nodeVar82 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar83 = ( render.nodeUniform30 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform31 ).x );
			nodeVar84 = ( nodeVar81.xy + ( vogelDiskSample( 0, 5, nodeVar82 ) * vec2<f32>( nodeVar83 ) ) );
			nodeVar85 = textureSampleCompare( nodeUniform29, nodeUniform29_sampler, nodeVar84, nodeVar81.z );
			nodeVar86 = ( nodeVar81.xy + ( vogelDiskSample( 1, 5, nodeVar82 ) * vec2<f32>( nodeVar83 ) ) );
			nodeVar87 = textureSampleCompare( nodeUniform29, nodeUniform29_sampler, nodeVar86, nodeVar81.z );
			nodeVar88 = ( nodeVar81.xy + ( vogelDiskSample( 2, 5, nodeVar82 ) * vec2<f32>( nodeVar83 ) ) );
			nodeVar89 = textureSampleCompare( nodeUniform29, nodeUniform29_sampler, nodeVar88, nodeVar81.z );
			nodeVar90 = ( nodeVar81.xy + ( vogelDiskSample( 3, 5, nodeVar82 ) * vec2<f32>( nodeVar83 ) ) );
			nodeVar91 = textureSampleCompare( nodeUniform29, nodeUniform29_sampler, nodeVar90, nodeVar81.z );
			nodeVar92 = ( nodeVar81.xy + ( vogelDiskSample( 4, 5, nodeVar82 ) * vec2<f32>( nodeVar83 ) ) );
			nodeVar93 = textureSampleCompare( nodeUniform29, nodeUniform29_sampler, nodeVar92, nodeVar81.z );
			nodeVar78 = ( ( ( ( ( nodeVar85 + nodeVar87 ) + nodeVar89 ) + nodeVar91 ) + nodeVar93 ) * 0.2 );

		} else {

			nodeVar78 = 1.0;

		}

		nodeVar94 = mix( 1.0, nodeVar78, render.nodeUniform32 );

		if ( ( render.nodeUniform37 > 0.0 ) ) {

			nodeVar96 = length( nodeVar67 );
			nodeVar97 = ( nodeVar96 / render.nodeUniform37 );
			nodeVar98 = clamp( ( 1.0 - ( ( ( nodeVar97 * nodeVar97 ) * nodeVar97 ) * nodeVar97 ) ), 0.0, 1.0 );
			nodeVar95 = ( ( 1.0 / max( pow( nodeVar96, render.nodeUniform38 ), 0.01 ) ) * ( nodeVar98 * nodeVar98 ) );

		} else {

			nodeVar95 = ( 1.0 / max( pow( length( nodeVar67 ), render.nodeUniform38 ), 0.01 ) );

		}

		nodeVar99 = textureSample( nodeUniform39, nodeUniform39_sampler, nodeVar72.xy );
		nodeVar70 = ( vec4<f32>( ( ( ( render.nodeUniform26 * vec3<f32>( nodeVar94 ) ) * vec3<f32>( smoothstep( render.nodeUniform33, render.nodeUniform34, dot( nodeVar68, normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform35 - render.nodeUniform36 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( nodeVar95 ) ), 1.0 ) * nodeVar99 );

	} else {

		shadowPositionWorld = v_positionWorld;
		nodeVar94 = mix( 1.0, nodeVar78, render.nodeUniform32 );
		nodeVar94 = mix( 1.0, nodeVar78, render.nodeUniform32 );

		if ( ( render.nodeUniform37 > 0.0 ) ) {

			nodeVar101 = length( nodeVar67 );
			nodeVar102 = ( nodeVar101 / render.nodeUniform37 );
			nodeVar103 = clamp( ( 1.0 - ( ( ( nodeVar102 * nodeVar102 ) * nodeVar102 ) * nodeVar102 ) ), 0.0, 1.0 );
			nodeVar100 = ( ( 1.0 / max( pow( nodeVar101, render.nodeUniform38 ), 0.01 ) ) * ( nodeVar103 * nodeVar103 ) );

		} else {

			nodeVar100 = ( 1.0 / max( pow( length( nodeVar67 ), render.nodeUniform38 ), 0.01 ) );

		}

		nodeVar70 = vec4<f32>( ( ( ( render.nodeUniform26 * vec3<f32>( nodeVar94 ) ) * vec3<f32>( smoothstep( render.nodeUniform33, render.nodeUniform34, dot( nodeVar68, normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform35 - render.nodeUniform36 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( nodeVar100 ) ), 1.0 );

	}

	nodeVar104 = ( vec4<f32>( clamp( nodeVar69, 0.0, 1.0 ) ) * nodeVar70 );
	nodeVar105 = nodeVar104;
	nodeVar106 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar107 = ( nodeVar105 * vec4<f32>( nodeVar106, 1.0 ) );
	nodeVar108 = ( nodeVar68 + positionViewDirection );
	nodeVar109 = normalize( nodeVar108 );
	nodeVar110 = dot( positionViewDirection, nodeVar109 );
	nodeVar111 = clamp( nodeVar110, 0.0, 1.0 );
	nodeVar112 = exp2( ( ( ( nodeVar111 * -5.55473 ) - 6.98316 ) * nodeVar111 ) );
	nodeVar113 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar112 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar112 ) ) );
	nodeVar114 = ( vec3<f32>( 1.0 ) - nodeVar113 );
	nodeVar115 = nodeVar114;
	nodeVar116 = ( nodeVar107 * vec4<f32>( nodeVar115, 1.0 ) );
	nodeVar117 = ( vec4<f32>( directDiffuse, 1.0 ) + nodeVar116 );
	directDiffuse = nodeVar117.xyz;
	nodeVar118 = normalize( ( nodeVar68 + positionViewDirection ) );
	nodeVar119 = clamp( dot( positionViewDirection, nodeVar118 ), 0.0, 1.0 );
	nodeVar120 = exp2( ( ( ( nodeVar119 * -5.55473 ) - 6.98316 ) * nodeVar119 ) );
	nodeVar121 = ( Roughness * Roughness );
	nodeVar122 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar120 ) ) ) + vec3<f32>( ( 1.0 * nodeVar120 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar121, clamp( dot( normalView, nodeVar68 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar121, clamp( dot( normalView, nodeVar118 ), 0.0, 1.0 ) ) ) );
	nodeVar123 = ( nodeVar105 * vec4<f32>( nodeVar122, 1.0 ) );
	nodeVar124 = ( nodeVar123 * vec4<f32>( multiScatteringCompensation, 1.0 ) );
	nodeVar125 = ( vec4<f32>( directSpecular, 1.0 ) + nodeVar124 );
	directSpecular = nodeVar125.xyz;
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
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
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
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	output.m0 = Output;
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform41 );
	nodeVar221 = ( ( object.nodeUniform40 * modelViewMatrix ) * vec4<f32>( positionLocal, 1.0 ) );
	nodeVar222 = ( ( render.nodeUniform42 * ( object.nodeUniform43 * object.nodeUniform44 ) ) * vec4<f32>( positionPrevious, 1.0 ) );
	nodeVar223 = ( ( nodeVar221.xy / vec2<f32>( nodeVar221.w ) ) - ( nodeVar222.xy / vec2<f32>( nodeVar222.w ) ) );
	output.m1 = vec4<f32>( vec3<f32>( nodeVar223, 0.0 ), 1.0 );

	// result

	return output;

}
