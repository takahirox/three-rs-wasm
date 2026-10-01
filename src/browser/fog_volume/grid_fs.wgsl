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
@binding( 3 ) @group( 1 ) var nodeUniform19_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform19 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform17 : mat4x4<f32>,
	nodeUniform16 : vec4<f32>,
	nodeUniform23 : mat4x4<f32>,
	nodeUniform22 : vec4<f32>,
	nodeUniform15 : f32,
	nodeUniform18 : f32,
	nodeUniform20 : f32,
	nodeUniform21 : vec2<f32>,
	nodeUniform24 : f32,
	nodeUniform25 : f32,
	nodeUniform26 : vec2<f32>,
	nodeUniform27 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : vec2<f32>;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : vec2<f32>;
var<private> nodeVar4 : vec2<f32>;
var<private> nodeVar5 : vec2<f32>;
var<private> nodeVar6 : vec2<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec2<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec4<f32>;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : f32;
var<private> shadowValue : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec4<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec2<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : vec2<f32>;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec2<f32>;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec2<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : vec2<f32>;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : vec4<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : vec2<f32>;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : vec2<f32>;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : vec2<f32>;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : vec2<f32>;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : vec2<f32>;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : f32;
var<private> nodeVar121 : f32;
var<private> nodeVar122 : f32;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : f32;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : f32;
var<private> nodeVar164 : f32;
var<private> nodeVar165 : f32;
var<private> nodeVar166 : f32;
var<private> nodeVar167 : f32;
var<private> nodeVar168 : f32;
var<private> nodeVar169 : f32;
var<private> nodeVar170 : f32;
var<private> nodeVar171 : f32;
var<private> nodeVar172 : f32;
var<private> nodeVar173 : f32;
var<private> nodeVar174 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar176 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar177 : vec3<f32>;
var<private> nodeVar178 : vec4<f32>;

// codes
fn tsl_mod_float( x : f32, y : f32 ) -> f32 { return x - y * floor( x / y ); }
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
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar1 = floor( v_positionWorld.xz );

	if ( ( abs( tsl_mod_float( ( nodeVar1.x + nodeVar1.y ), 2.0 ) ) > 0.5 ) ) {

		nodeVar0 = vec3<f32>( 0.04373502925049377, 0.04817182422013895, 0.061246054224174035 );

	} else {

		nodeVar0 = vec3<f32>( 0.033104766565152086, 0.036889450395083165, 0.04666508633021928 );

	}

	nodeVar2 = ( v_positionWorld.xz * vec2<f32>( 5.0 ) );
	nodeVar3 = ( ( vec2<f32>( 0.5 ) - abs( ( fract( nodeVar2 ) - vec2<f32>( 0.5 ) ) ) ) / max( fwidth( nodeVar2 ), vec2<f32>( 0.0001 ) ) );
	nodeVar4 = ( ( vec2<f32>( 0.5 ) - abs( ( fract( v_positionWorld.xz ) - vec2<f32>( 0.5 ) ) ) ) / max( fwidth( v_positionWorld.xz ), vec2<f32>( 0.0001 ) ) );
	nodeVar5 = abs( ( v_positionWorld.xz - round( v_positionWorld.xz ) ) );
	nodeVar6 = max( fwidth( v_positionWorld.xz ), vec2<f32>( 0.0001 ) );
	DiffuseColor = vec4<f32>( mix( mix( mix( nodeVar0, vec3<f32>( 0.07421356837213867, 0.08228270712149792, 0.10702310296918527 ), max( smoothstep( 0.9, 0.0, nodeVar3.x ), smoothstep( 0.9, 0.0, nodeVar3.y ) ) ), vec3<f32>( 0.7304607400847158, 0.7304607400847158, 0.7304607400847158 ), max( smoothstep( 1.4, 0.0, nodeVar4.x ), smoothstep( 1.4, 0.0, nodeVar4.y ) ) ), vec3<f32>( 1.0, 1.0, 1.0 ), max( ( smoothstep( 0.08, 0.07, nodeVar5.x ) * smoothstep( ( nodeVar6.y * 1.5 ), 0.0, nodeVar5.y ) ), ( smoothstep( 0.08, 0.07, nodeVar5.y ) * smoothstep( ( nodeVar6.x * 1.5 ), 0.0, nodeVar5.x ) ) ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform2;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar7 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform3, 0.0525 ) + max( max( nodeVar7.x, nodeVar7.y ), nodeVar7.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar8 = dot( normalView, positionViewDirection );
	nodeVar9 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( nodeVar8, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar9;
	nodeVar10 = ( dfg.x + dfg.y );
	nodeVar11 = ( 1.0 / nodeVar10 );
	nodeVar12 = nodeVar11;
	nodeVar13 = ( nodeVar12 - 1.0 );
	nodeVar14 = ( SpecularColorBlended * vec3<f32>( nodeVar13 ) );
	nodeVar15 = ( nodeVar14 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar15;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar16 = dot( normalWorld, normalize( render.nodeUniform12 ) );
	nodeVar17 = ( nodeVar16 * 0.5 );
	nodeVar18 = ( nodeVar17 + 0.5 );
	nodeVar19 = mix( render.nodeUniform9, render.nodeUniform10, nodeVar18 );
	nodeVar20 = ( irradiance + nodeVar19 );
	irradiance = nodeVar20;
	nodeVar21 = vec4<f32>( render.nodeUniform13, 0.0 );
	nodeVar22 = ( render.cameraViewMatrix * nodeVar21 );
	nodeVar23 = normalize( nodeVar22.xyz );
	nodeVar24 = nodeVar23;
	nodeVar25 = dot( normalView, nodeVar24 );
	shadowPositionWorld = v_positionWorld;
	nodeVar26 = vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform15 ) ) ), 1.0 );
	nodeVar27 = ( - v_positionView.z );
	shadowValue = 1.0;

	if ( ( ( nodeVar27 >= render.nodeUniform16.x ) && ( nodeVar27 < render.nodeUniform16.y ) ) ) {

		nodeVar29 = ( render.nodeUniform17 * nodeVar26 );
		nodeVar30 = ( nodeVar29.xyz / vec3<f32>( nodeVar29.w ) );
		nodeVar31 = vec3<f32>( nodeVar30.x, ( 1.0 - nodeVar30.y ), ( nodeVar30.z + render.nodeUniform18 ) );

		if ( ( ( ( ( ( nodeVar31.x >= 0.0 ) && ( nodeVar31.x <= 1.0 ) ) && ( nodeVar31.y >= 0.0 ) ) && ( nodeVar31.y <= 1.0 ) ) && ( nodeVar31.z <= 1.0 ) ) ) {

			nodeVar32 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar33 = ( render.nodeUniform20 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform21 ).x );
			nodeVar34 = ( nodeVar31.xy + ( vogelDiskSample( 0, 5, nodeVar32 ) * vec2<f32>( nodeVar33 ) ) );
			nodeVar35 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar34, nodeVar31.z );
			nodeVar36 = ( nodeVar31.xy + ( vogelDiskSample( 1, 5, nodeVar32 ) * vec2<f32>( nodeVar33 ) ) );
			nodeVar37 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar36, nodeVar31.z );
			nodeVar38 = ( nodeVar31.xy + ( vogelDiskSample( 2, 5, nodeVar32 ) * vec2<f32>( nodeVar33 ) ) );
			nodeVar39 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar38, nodeVar31.z );
			nodeVar40 = ( nodeVar31.xy + ( vogelDiskSample( 3, 5, nodeVar32 ) * vec2<f32>( nodeVar33 ) ) );
			nodeVar41 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar40, nodeVar31.z );
			nodeVar42 = ( nodeVar31.xy + ( vogelDiskSample( 4, 5, nodeVar32 ) * vec2<f32>( nodeVar33 ) ) );
			nodeVar43 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar42, nodeVar31.z );
			nodeVar28 = ( ( ( ( ( nodeVar35 + nodeVar37 ) + nodeVar39 ) + nodeVar41 ) + nodeVar43 ) * 0.2 );

		} else {

			nodeVar28 = 1.0;

		}

		shadowValue = mix( nodeVar28, shadowValue, smoothstep( render.nodeUniform16.z, render.nodeUniform16.y, nodeVar27 ) );
		

	}


	if ( ( ( nodeVar27 >= render.nodeUniform22.x ) && ( nodeVar27 < render.nodeUniform22.y ) ) ) {

		nodeVar45 = ( render.nodeUniform23 * nodeVar26 );
		nodeVar46 = ( nodeVar45.xyz / vec3<f32>( nodeVar45.w ) );
		nodeVar47 = vec3<f32>( nodeVar46.x, ( 1.0 - nodeVar46.y ), ( nodeVar46.z + render.nodeUniform24 ) );

		if ( ( ( ( ( ( nodeVar47.x >= 0.0 ) && ( nodeVar47.x <= 1.0 ) ) && ( nodeVar47.y >= 0.0 ) ) && ( nodeVar47.y <= 1.0 ) ) && ( nodeVar47.z <= 1.0 ) ) ) {

			nodeVar48 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar49 = ( render.nodeUniform25 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform26 ).x );
			nodeVar50 = ( nodeVar47.xy + ( vogelDiskSample( 0, 5, nodeVar48 ) * vec2<f32>( nodeVar49 ) ) );
			nodeVar51 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar50, nodeVar47.z );
			nodeVar52 = ( nodeVar47.xy + ( vogelDiskSample( 1, 5, nodeVar48 ) * vec2<f32>( nodeVar49 ) ) );
			nodeVar53 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar52, nodeVar47.z );
			nodeVar54 = ( nodeVar47.xy + ( vogelDiskSample( 2, 5, nodeVar48 ) * vec2<f32>( nodeVar49 ) ) );
			nodeVar55 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar54, nodeVar47.z );
			nodeVar56 = ( nodeVar47.xy + ( vogelDiskSample( 3, 5, nodeVar48 ) * vec2<f32>( nodeVar49 ) ) );
			nodeVar57 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar56, nodeVar47.z );
			nodeVar58 = ( nodeVar47.xy + ( vogelDiskSample( 4, 5, nodeVar48 ) * vec2<f32>( nodeVar49 ) ) );
			nodeVar59 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar58, nodeVar47.z );
			nodeVar44 = ( ( ( ( ( nodeVar51 + nodeVar53 ) + nodeVar55 ) + nodeVar57 ) + nodeVar59 ) * 0.2 );

		} else {

			nodeVar44 = 1.0;

		}

		shadowValue = mix( nodeVar44, shadowValue, smoothstep( render.nodeUniform22.z, render.nodeUniform22.y, nodeVar27 ) );
		

	}

	nodeVar60 = mix( 1.0, shadowValue, render.nodeUniform27 );
	nodeVar61 = ( vec3<f32>( clamp( nodeVar25, 0.0, 1.0 ) ) * ( render.nodeUniform14 * vec3<f32>( nodeVar60 ) ) );
	nodeVar62 = nodeVar61;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar63 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar64 = ( nodeVar62 * nodeVar63 );
	nodeVar65 = ( nodeVar24 + positionViewDirection );
	nodeVar66 = normalize( nodeVar65 );
	nodeVar67 = dot( positionViewDirection, nodeVar66 );
	nodeVar68 = clamp( nodeVar67, 0.0, 1.0 );
	nodeVar69 = exp2( ( ( ( nodeVar68 * -5.55473 ) - 6.98316 ) * nodeVar68 ) );
	nodeVar70 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar69 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar69 ) ) );
	nodeVar71 = ( vec3<f32>( 1.0 ) - nodeVar70 );
	nodeVar72 = nodeVar71;
	nodeVar73 = ( nodeVar64 * nodeVar72 );
	nodeVar74 = ( directDiffuse + nodeVar73 );
	directDiffuse = nodeVar74;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar75 = normalize( ( nodeVar24 + positionViewDirection ) );
	nodeVar76 = clamp( dot( positionViewDirection, nodeVar75 ), 0.0, 1.0 );
	nodeVar77 = exp2( ( ( ( nodeVar76 * -5.55473 ) - 6.98316 ) * nodeVar76 ) );
	nodeVar78 = ( Roughness * Roughness );
	nodeVar79 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar77 ) ) ) + vec3<f32>( ( 1.0 * nodeVar77 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar78, clamp( dot( normalView, nodeVar24 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar78, clamp( dot( normalView, nodeVar75 ), 0.0, 1.0 ) ) ) );
	nodeVar80 = ( nodeVar62 * nodeVar79 );
	nodeVar81 = ( nodeVar80 * multiScatteringCompensation );
	nodeVar82 = ( directSpecular + nodeVar81 );
	directSpecular = nodeVar82;
	nodeVar83 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar84 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar85 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar86 = ( SpecularF90 * dfg.y );
	nodeVar87 = ( nodeVar85 + vec3<f32>( nodeVar86 ) );
	nodeVar88 = ( nodeVar83 + nodeVar87 );
	nodeVar83 = nodeVar88;
	nodeVar89 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar90 = nodeVar89;
	nodeVar91 = ( nodeVar90 * vec3<f32>( 0.047619 ) );
	nodeVar92 = ( SpecularColor + nodeVar91 );
	nodeVar93 = ( nodeVar87 * nodeVar92 );
	nodeVar94 = ( dfg.x + dfg.y );
	nodeVar95 = ( 1.0 - nodeVar94 );
	nodeVar96 = nodeVar95;
	nodeVar97 = ( vec3<f32>( nodeVar96 ) * nodeVar92 );
	nodeVar98 = ( vec3<f32>( 1.0 ) - nodeVar97 );
	nodeVar99 = nodeVar98;
	nodeVar100 = ( nodeVar93 / nodeVar99 );
	nodeVar101 = ( nodeVar100 * vec3<f32>( nodeVar96 ) );
	nodeVar102 = ( nodeVar84 + nodeVar101 );
	nodeVar84 = nodeVar102;
	nodeVar103 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar104 = ( irradiance * nodeVar103 );
	nodeVar105 = ( nodeVar83 + nodeVar84 );
	nodeVar106 = ( vec3<f32>( 1.0 ) - nodeVar105 );
	nodeVar107 = nodeVar106;
	nodeVar108 = ( nodeVar104 * nodeVar107 );
	nodeVar109 = nodeVar108;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar110 = ( indirectDiffuse + nodeVar109 );
	indirectDiffuse = nodeVar110;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar111 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar112 = ( SpecularF90 * dfg.y );
	nodeVar113 = ( nodeVar111 + vec3<f32>( nodeVar112 ) );
	nodeVar114 = ( singleScatteringDielectric + nodeVar113 );
	singleScatteringDielectric = nodeVar114;
	nodeVar115 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar116 = nodeVar115;
	nodeVar117 = ( nodeVar116 * vec3<f32>( 0.047619 ) );
	nodeVar118 = ( SpecularColor + nodeVar117 );
	nodeVar119 = ( nodeVar113 * nodeVar118 );
	nodeVar120 = ( dfg.x + dfg.y );
	nodeVar121 = ( 1.0 - nodeVar120 );
	nodeVar122 = nodeVar121;
	nodeVar123 = ( vec3<f32>( nodeVar122 ) * nodeVar118 );
	nodeVar124 = ( vec3<f32>( 1.0 ) - nodeVar123 );
	nodeVar125 = nodeVar124;
	nodeVar126 = ( nodeVar119 / nodeVar125 );
	nodeVar127 = ( nodeVar126 * vec3<f32>( nodeVar122 ) );
	nodeVar128 = ( multiScatteringDielectric + nodeVar127 );
	multiScatteringDielectric = nodeVar128;
	nodeVar129 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar130 = ( SpecularF90 * dfg.y );
	nodeVar131 = ( nodeVar129 + vec3<f32>( nodeVar130 ) );
	nodeVar132 = ( singleScatteringMetallic + nodeVar131 );
	singleScatteringMetallic = nodeVar132;
	nodeVar133 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar134 = nodeVar133;
	nodeVar135 = ( nodeVar134 * vec3<f32>( 0.047619 ) );
	nodeVar136 = ( DiffuseColor.xyz + nodeVar135 );
	nodeVar137 = ( nodeVar131 * nodeVar136 );
	nodeVar138 = ( dfg.x + dfg.y );
	nodeVar139 = ( 1.0 - nodeVar138 );
	nodeVar140 = nodeVar139;
	nodeVar141 = ( vec3<f32>( nodeVar140 ) * nodeVar136 );
	nodeVar142 = ( vec3<f32>( 1.0 ) - nodeVar141 );
	nodeVar143 = nodeVar142;
	nodeVar144 = ( nodeVar137 / nodeVar143 );
	nodeVar145 = ( nodeVar144 * vec3<f32>( nodeVar140 ) );
	nodeVar146 = ( multiScatteringMetallic + nodeVar145 );
	multiScatteringMetallic = nodeVar146;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar147 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar148 = ( radiance * nodeVar147 );
	nodeVar149 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar150 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar151 = ( nodeVar149 * nodeVar150 );
	nodeVar152 = ( nodeVar148 + nodeVar151 );
	nodeVar153 = nodeVar152;
	nodeVar154 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar155 = ( vec3<f32>( 1.0 ) - nodeVar154 );
	nodeVar156 = nodeVar155;
	nodeVar157 = ( DiffuseContribution * nodeVar156 );
	nodeVar158 = ( nodeVar157 * nodeVar150 );
	nodeVar159 = nodeVar158;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar160 = ( indirectSpecular + nodeVar153 );
	indirectSpecular = nodeVar160;
	nodeVar161 = ( indirectDiffuse + nodeVar159 );
	indirectDiffuse = nodeVar161;
	ambientOcclusion = 1.0;
	nodeVar162 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar162;
	nodeVar163 = dot( normalView, positionViewDirection );
	nodeVar164 = ( clamp( nodeVar163, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar165 = ( Roughness * -16.0 );
	nodeVar166 = ( 1.0 - nodeVar165 );
	nodeVar167 = nodeVar166;
	nodeVar168 = ( - nodeVar167 );
	nodeVar169 = exp2( nodeVar168 );
	nodeVar170 = pow( nodeVar164, nodeVar169 );
	nodeVar171 = ( 1.0 - nodeVar170 );
	nodeVar172 = nodeVar171;
	nodeVar173 = ( ambientOcclusion - nodeVar172 );
	nodeVar174 = ( indirectSpecular * vec3<f32>( clamp( nodeVar173, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar174;
	nodeVar175 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar175;
	nodeVar176 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar176;
	nodeVar177 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar177;
	nodeVar178 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar178;

	// result

	output.color = nodeVar178;

	return output;

}
