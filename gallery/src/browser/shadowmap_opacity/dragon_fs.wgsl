// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );

// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform11_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform11 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform22_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform22 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform24_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform24 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform29_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform29 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform33_sampler : sampler_comparison;
@binding( 10 ) @group( 1 ) var nodeUniform33 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : f32,
	nodeUniform7 : vec3<f32>,
	nodeUniform8 : f32,
	nodeUniform9 : f32,
	nodeUniform10 : f32,
	nodeUniform12 : mat3x3<f32>,
	nodeUniform13 : f32,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform16 : f32,
	nodeUniform19 : mat4x4<f32>,
	nodeUniform21 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform30 : mat4x4<f32>,
	nodeUniform31 : f32,
	nodeUniform32 : f32,
	nodeUniform36 : f32,
	cameraPosition : vec3<f32>,
	nodeUniform23 : vec2<f32>,
	nodeUniform34 : f32,
	nodeUniform35 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> IOR : f32;
var<private> SpecularColor : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> Transmission : f32;
var<private> Thickness : f32;
var<private> nodeVar2 : vec4<f32>;
var<private> AttenuationDistance : f32;
var<private> AttenuationColor : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec2<f32>;
var<private> nodeVar8 : vec2<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec2<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec2<f32>;
var<private> nodeVar14 : vec2<f32>;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec2<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : vec4<f32>;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : vec4<f32>;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : vec4<f32>;
var<private> nodeVar28 : vec2<f32>;
var<private> nodeVar29 : vec2<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec2<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec4<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec4<f32>;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec4<f32>;
var<private> nodeVar41 : vec4<f32>;
var<private> nodeVar42 : vec4<f32>;
var<private> nodeVar43 : vec4<f32>;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : vec2<f32>;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec4<f32>;
var<private> nodeVar57 : vec4<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar61 : vec4<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec4<f32>;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : vec2<f32>;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : vec2<f32>;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : vec2<f32>;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : vec2<f32>;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : vec2<f32>;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : vec4<f32>;
var<private> nodeVar79 : vec4<f32>;
var<private> nodeVar80 : vec4<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec4<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec4<f32>;
var<private> nodeVar92 : vec4<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec4<f32>;
var<private> nodeVar99 : vec4<f32>;
var<private> nodeVar100 : vec4<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
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
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : f32;
var<private> nodeVar157 : f32;
var<private> nodeVar158 : f32;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : vec3<f32>;
var<private> nodeVar177 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar178 : vec3<f32>;
var<private> nodeVar179 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar180 : vec3<f32>;
var<private> nodeVar181 : f32;
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
var<private> nodeVar192 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar193 : vec3<f32>;
var<private> nodeVar194 : vec4<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar195 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar196 : vec3<f32>;
var<private> nodeVar197 : vec4<f32>;

// codes
fn getVolumeTransmissionRay ( n : vec3<f32>, v : vec3<f32>, thickness : f32, ior : f32, modelMatrix : mat4x4<f32> ) -> vec3<f32> {

	

	return ( normalize( refract( ( - v ), normalize( n ), ( 1.0 / ior ) ) ) * ( vec3<f32>( thickness ) * vec3<f32>( length( modelMatrix[ 0u ].xyz ), length( modelMatrix[ 1u ].xyz ), length( modelMatrix[ 2u ].xyz ) ) ) );

}

fn volumeAttenuation ( transmissionDistance : f32, attenuationColor : vec3<f32>, attenuationDistance : f32 ) -> vec3<f32> {

	

	if ( ( attenuationDistance != 0.0 ) ) {

		return exp( ( ( - ( ( - log( attenuationColor ) ) / vec3<f32>( attenuationDistance ) ) ) * vec3<f32>( transmissionDistance ) ) );

	}

	return vec3<f32>( 1.0, 1.0, 1.0 );

}

fn applyIorToRoughness ( roughness : f32, ior : f32 ) -> f32 {

	

	return ( roughness * clamp( ( ( ior * 2.0 ) - 2.0 ), 0.0, 1.0 ) );

}

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
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) nodeVarying7 : vec2<f32>,
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
	IOR = object.nodeUniform6;
	nodeVar1 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeVar1 * nodeVar1 ) ) * object.nodeUniform7 ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform8 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform8, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	Transmission = object.nodeUniform9;
	nodeVar2 = textureSample( nodeUniform11, nodeUniform11_sampler, ( object.nodeUniform12 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	Thickness = ( object.nodeUniform10 * nodeVar2.y );
	AttenuationDistance = object.nodeUniform13;
	AttenuationColor = object.nodeUniform14;
	EmissiveColor = ( object.nodeUniform15 * vec3<f32>( object.nodeUniform16 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar3 = ( render.cameraPosition - v_positionWorld );
	nodeVar4 = normalize( nodeVar3 );
	nodeVar5 = getVolumeTransmissionRay( normalWorld, nodeVar4, Thickness, IOR, object.nodeUniform21 );
	nodeVar6 = ( render.cameraProjectionMatrix * ( render.cameraViewMatrix * vec4<f32>( ( v_positionWorld + nodeVar5 ), 1.0 ) ) );
	nodeVar7 = ( nodeVar6.xy / vec2<f32>( nodeVar6.w ) );
	nodeVar7 = ( nodeVar7 + vec2<f32>( 1.0 ) );
	nodeVar7 = ( nodeVar7 / vec2<f32>( 2.0 ) );
	nodeVar7 = vec2<f32>( nodeVar7.x, ( 1.0 - nodeVar7.y ) );
	nodeVar8 = textureSample( nodeUniform22, nodeUniform22_sampler, vec2<f32>( Roughness, clamp( dot( normalWorld, nodeVar4 ), 0.0, 1.0 ) ) ).xy;
	nodeVar9 = ( DiffuseContribution * volumeAttenuation( length( nodeVar5 ), AttenuationColor, AttenuationDistance ) );
	let cameraViewport = vec4<f32>( 0.0, 0.0, render.nodeUniform23.x, render.nodeUniform23.y );
	nodeVar10 = ( ( ( nodeVar7 * cameraViewport.zw ) + cameraViewport.xy ) / render.nodeUniform23 );
	nodeVar11 = ( log2( cameraViewport.z ) * applyIorToRoughness( Roughness, IOR ) );
	nodeVar12 = vec4<f32>( ( vec2<f32>( 1.0 ) / vec2<f32>( textureDimensions( nodeUniform24, i32( nodeVar11 ) ) ) ), vec2<f32>( textureDimensions( nodeUniform24, i32( nodeVar11 ) ) ) );
	nodeVar13 = ( ( nodeVar10 * nodeVar12.zw ) + vec2<f32>( 0.5 ) );
	nodeVar14 = fract( nodeVar13 );
	nodeVar15 = ( ( 0.16666666666666666 * ( ( nodeVar14.x * ( ( nodeVar14.x * ( ( - nodeVar14.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar14.x * ( nodeVar14.x * ( ( 3.0 * nodeVar14.x ) - 6.0 ) ) ) + 4.0 ) ) );
	nodeVar16 = floor( nodeVar13 );
	nodeVar17 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar14.x * ( nodeVar14.x * ( ( 3.0 * nodeVar14.x ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar14.x * ( ( nodeVar14.x * ( ( - nodeVar14.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar14.x * ( nodeVar14.x * ( ( 3.0 * nodeVar14.x ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar18 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar14.y * ( nodeVar14.y * ( ( 3.0 * nodeVar14.y ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar14.y * ( ( nodeVar14.y * ( ( - nodeVar14.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar14.y * ( nodeVar14.y * ( ( 3.0 * nodeVar14.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar19 = floor( nodeVar11 );
	nodeVar20 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, ( ( vec2<f32>( ( nodeVar16.x + nodeVar17 ), ( nodeVar16.y + nodeVar18 ) ) - vec2<f32>( 0.5 ) ) * nodeVar12.xy ), nodeVar19 );
	nodeVar21 = ( ( 0.16666666666666666 * ( ( nodeVar14.x * ( ( nodeVar14.x * ( ( -3.0 * nodeVar14.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar14.x, 3.0 ) ) );
	nodeVar22 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar14.x, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar14.x * ( ( nodeVar14.x * ( ( -3.0 * nodeVar14.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar14.x, 3.0 ) ) ) ) );
	nodeVar23 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, ( ( vec2<f32>( ( nodeVar16.x + nodeVar22 ), ( nodeVar16.y + nodeVar18 ) ) - vec2<f32>( 0.5 ) ) * nodeVar12.xy ), nodeVar19 );
	nodeVar24 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar14.y, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar14.y * ( ( nodeVar14.y * ( ( -3.0 * nodeVar14.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar14.y, 3.0 ) ) ) ) );
	nodeVar25 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, ( ( vec2<f32>( ( nodeVar16.x + nodeVar17 ), ( nodeVar16.y + nodeVar24 ) ) - vec2<f32>( 0.5 ) ) * nodeVar12.xy ), nodeVar19 );
	nodeVar26 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, ( ( vec2<f32>( ( nodeVar16.x + nodeVar22 ), ( nodeVar16.y + nodeVar24 ) ) - vec2<f32>( 0.5 ) ) * nodeVar12.xy ), nodeVar19 );
	nodeVar27 = vec4<f32>( ( vec2<f32>( 1.0 ) / vec2<f32>( textureDimensions( nodeUniform24, i32( ( nodeVar11 + 1.0 ) ) ) ) ), vec2<f32>( textureDimensions( nodeUniform24, i32( ( nodeVar11 + 1.0 ) ) ) ) );
	nodeVar28 = ( ( nodeVar10 * nodeVar27.zw ) + vec2<f32>( 0.5 ) );
	nodeVar29 = fract( nodeVar28 );
	nodeVar30 = ( ( 0.16666666666666666 * ( ( nodeVar29.x * ( ( nodeVar29.x * ( ( - nodeVar29.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar29.x * ( nodeVar29.x * ( ( 3.0 * nodeVar29.x ) - 6.0 ) ) ) + 4.0 ) ) );
	nodeVar31 = floor( nodeVar28 );
	nodeVar32 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar29.x * ( nodeVar29.x * ( ( 3.0 * nodeVar29.x ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar29.x * ( ( nodeVar29.x * ( ( - nodeVar29.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar29.x * ( nodeVar29.x * ( ( 3.0 * nodeVar29.x ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar33 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar29.y * ( nodeVar29.y * ( ( 3.0 * nodeVar29.y ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar29.y * ( ( nodeVar29.y * ( ( - nodeVar29.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar29.y * ( nodeVar29.y * ( ( 3.0 * nodeVar29.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar34 = ceil( nodeVar11 );
	nodeVar35 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, ( ( vec2<f32>( ( nodeVar31.x + nodeVar32 ), ( nodeVar31.y + nodeVar33 ) ) - vec2<f32>( 0.5 ) ) * nodeVar27.xy ), nodeVar34 );
	nodeVar36 = ( ( 0.16666666666666666 * ( ( nodeVar29.x * ( ( nodeVar29.x * ( ( -3.0 * nodeVar29.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar29.x, 3.0 ) ) );
	nodeVar37 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar29.x, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar29.x * ( ( nodeVar29.x * ( ( -3.0 * nodeVar29.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar29.x, 3.0 ) ) ) ) );
	nodeVar38 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, ( ( vec2<f32>( ( nodeVar31.x + nodeVar37 ), ( nodeVar31.y + nodeVar33 ) ) - vec2<f32>( 0.5 ) ) * nodeVar27.xy ), nodeVar34 );
	nodeVar39 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar29.y, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar29.y * ( ( nodeVar29.y * ( ( -3.0 * nodeVar29.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar29.y, 3.0 ) ) ) ) );
	nodeVar40 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, ( ( vec2<f32>( ( nodeVar31.x + nodeVar32 ), ( nodeVar31.y + nodeVar39 ) ) - vec2<f32>( 0.5 ) ) * nodeVar27.xy ), nodeVar34 );
	nodeVar41 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, ( ( vec2<f32>( ( nodeVar31.x + nodeVar37 ), ( nodeVar31.y + nodeVar39 ) ) - vec2<f32>( 0.5 ) ) * nodeVar27.xy ), nodeVar34 );
	nodeVar42 = mix( ( ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar14.y * ( ( nodeVar14.y * ( ( - nodeVar14.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar14.y * ( nodeVar14.y * ( ( 3.0 * nodeVar14.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) * ( ( vec4<f32>( nodeVar15 ) * nodeVar20 ) + ( vec4<f32>( nodeVar21 ) * nodeVar23 ) ) ) + ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar14.y * ( ( nodeVar14.y * ( ( -3.0 * nodeVar14.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar14.y, 3.0 ) ) ) ) * ( ( vec4<f32>( nodeVar15 ) * nodeVar25 ) + ( vec4<f32>( nodeVar21 ) * nodeVar26 ) ) ) ), ( ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar29.y * ( ( nodeVar29.y * ( ( - nodeVar29.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar29.y * ( nodeVar29.y * ( ( 3.0 * nodeVar29.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) * ( ( vec4<f32>( nodeVar30 ) * nodeVar35 ) + ( vec4<f32>( nodeVar36 ) * nodeVar38 ) ) ) + ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar29.y * ( ( nodeVar29.y * ( ( -3.0 * nodeVar29.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar29.y, 3.0 ) ) ) ) * ( ( vec4<f32>( nodeVar30 ) * nodeVar40 ) + ( vec4<f32>( nodeVar36 ) * nodeVar41 ) ) ) ), fract( nodeVar11 ) );
	nodeVar43 = vec4<f32>( ( ( vec3<f32>( 1.0 ) - ( ( SpecularColorBlended * vec3<f32>( nodeVar8.x ) ) + vec3<f32>( ( SpecularF90 * nodeVar8.y ) ) ) ) * ( nodeVar9 * nodeVar42.xyz ) ), ( 1.0 - ( ( 1.0 - nodeVar42.w ) * ( ( ( nodeVar9.x + nodeVar9.y ) + nodeVar9.z ) / 3.0 ) ) ) );
	nodeVar44 = mix( 1.0, nodeVar43.w, Transmission );
	nodeVar45 = ( DiffuseColor.w * nodeVar44 );
	DiffuseColor.w = nodeVar45;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar46 = dot( normalView, positionViewDirection );
	nodeVar47 = textureSample( nodeUniform22, nodeUniform22_sampler, vec2<f32>( Roughness, clamp( nodeVar46, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar47;
	nodeVar48 = ( dfg.x + dfg.y );
	nodeVar49 = ( 1.0 / nodeVar48 );
	nodeVar50 = nodeVar49;
	nodeVar51 = ( nodeVar50 - 1.0 );
	nodeVar52 = ( SpecularColorBlended * vec3<f32>( nodeVar51 ) );
	nodeVar53 = ( nodeVar52 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar53;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar54 = ( irradiance + render.nodeUniform25 );
	irradiance = nodeVar54;
	nodeVar55 = ( render.nodeUniform26 - render.nodeUniform27 );
	nodeVar56 = vec4<f32>( nodeVar55, 0.0 );
	nodeVar57 = ( render.cameraViewMatrix * nodeVar56 );
	nodeVar58 = normalize( nodeVar57.xyz );
	nodeVar59 = nodeVar58;
	nodeVar60 = dot( normalView, nodeVar59 );
	shadowPositionWorld = v_positionWorld;
	nodeVar61 = ( render.nodeUniform30 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform31 ) ) ), 1.0 ) );
	nodeVar62 = ( nodeVar61.xyz / vec3<f32>( nodeVar61.w ) );
	nodeVar63 = vec3<f32>( nodeVar62.x, ( 1.0 - nodeVar62.y ), ( nodeVar62.z + render.nodeUniform32 ) );
	nodeVar64 = textureSample( nodeUniform29, nodeUniform29_sampler, nodeVar63.xy );

	if ( ( ( ( ( ( nodeVar63.x >= 0.0 ) && ( nodeVar63.x <= 1.0 ) ) && ( nodeVar63.y >= 0.0 ) ) && ( nodeVar63.y <= 1.0 ) ) && ( nodeVar63.z <= 1.0 ) ) ) {

		nodeVar66 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar67 = ( render.nodeUniform34 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform35 ).x );
		nodeVar68 = ( nodeVar63.xy + ( vogelDiskSample( 0, 5, nodeVar66 ) * vec2<f32>( nodeVar67 ) ) );
		nodeVar69 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeVar68, nodeVar63.z );
		nodeVar70 = ( nodeVar63.xy + ( vogelDiskSample( 1, 5, nodeVar66 ) * vec2<f32>( nodeVar67 ) ) );
		nodeVar71 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeVar70, nodeVar63.z );
		nodeVar72 = ( nodeVar63.xy + ( vogelDiskSample( 2, 5, nodeVar66 ) * vec2<f32>( nodeVar67 ) ) );
		nodeVar73 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeVar72, nodeVar63.z );
		nodeVar74 = ( nodeVar63.xy + ( vogelDiskSample( 3, 5, nodeVar66 ) * vec2<f32>( nodeVar67 ) ) );
		nodeVar75 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeVar74, nodeVar63.z );
		nodeVar76 = ( nodeVar63.xy + ( vogelDiskSample( 4, 5, nodeVar66 ) * vec2<f32>( nodeVar67 ) ) );
		nodeVar77 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeVar76, nodeVar63.z );
		nodeVar65 = ( ( ( ( ( nodeVar69 + nodeVar71 ) + nodeVar73 ) + nodeVar75 ) + nodeVar77 ) * 0.2 );

	} else {

		nodeVar65 = 1.0;

	}

	nodeVar78 = mix( vec4<f32>( 1.0 ), mix( nodeVar64, vec4<f32>( 1.0 ), vec4<f32>( nodeVar65 ) ), ( render.nodeUniform36 * nodeVar64.w ) );
	nodeVar79 = ( vec4<f32>( clamp( nodeVar60, 0.0, 1.0 ) ) * ( vec4<f32>( render.nodeUniform28, 1.0 ) * nodeVar78 ) );
	nodeVar80 = nodeVar79;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar81 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar82 = ( nodeVar80 * vec4<f32>( nodeVar81, 1.0 ) );
	nodeVar83 = ( nodeVar59 + positionViewDirection );
	nodeVar84 = normalize( nodeVar83 );
	nodeVar85 = dot( positionViewDirection, nodeVar84 );
	nodeVar86 = clamp( nodeVar85, 0.0, 1.0 );
	nodeVar87 = exp2( ( ( ( nodeVar86 * -5.55473 ) - 6.98316 ) * nodeVar86 ) );
	nodeVar88 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar87 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar87 ) ) );
	nodeVar89 = ( vec3<f32>( 1.0 ) - nodeVar88 );
	nodeVar90 = nodeVar89;
	nodeVar91 = ( nodeVar82 * vec4<f32>( nodeVar90, 1.0 ) );
	nodeVar92 = ( vec4<f32>( directDiffuse, 1.0 ) + nodeVar91 );
	directDiffuse = nodeVar92.xyz;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar93 = normalize( ( nodeVar59 + positionViewDirection ) );
	nodeVar94 = clamp( dot( positionViewDirection, nodeVar93 ), 0.0, 1.0 );
	nodeVar95 = exp2( ( ( ( nodeVar94 * -5.55473 ) - 6.98316 ) * nodeVar94 ) );
	nodeVar96 = ( Roughness * Roughness );
	nodeVar97 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar95 ) ) ) + vec3<f32>( ( 1.0 * nodeVar95 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar96, clamp( dot( normalView, nodeVar59 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar96, clamp( dot( normalView, nodeVar93 ), 0.0, 1.0 ) ) ) );
	nodeVar98 = ( nodeVar80 * vec4<f32>( nodeVar97, 1.0 ) );
	nodeVar99 = ( nodeVar98 * vec4<f32>( multiScatteringCompensation, 1.0 ) );
	nodeVar100 = ( vec4<f32>( directSpecular, 1.0 ) + nodeVar99 );
	directSpecular = nodeVar100.xyz;
	nodeVar101 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar102 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar103 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar104 = ( SpecularF90 * dfg.y );
	nodeVar105 = ( nodeVar103 + vec3<f32>( nodeVar104 ) );
	nodeVar106 = ( nodeVar101 + nodeVar105 );
	nodeVar101 = nodeVar106;
	nodeVar107 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar108 = nodeVar107;
	nodeVar109 = ( nodeVar108 * vec3<f32>( 0.047619 ) );
	nodeVar110 = ( SpecularColor + nodeVar109 );
	nodeVar111 = ( nodeVar105 * nodeVar110 );
	nodeVar112 = ( dfg.x + dfg.y );
	nodeVar113 = ( 1.0 - nodeVar112 );
	nodeVar114 = nodeVar113;
	nodeVar115 = ( vec3<f32>( nodeVar114 ) * nodeVar110 );
	nodeVar116 = ( vec3<f32>( 1.0 ) - nodeVar115 );
	nodeVar117 = nodeVar116;
	nodeVar118 = ( nodeVar111 / nodeVar117 );
	nodeVar119 = ( nodeVar118 * vec3<f32>( nodeVar114 ) );
	nodeVar120 = ( nodeVar102 + nodeVar119 );
	nodeVar102 = nodeVar120;
	nodeVar121 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar122 = ( irradiance * nodeVar121 );
	nodeVar123 = ( nodeVar101 + nodeVar102 );
	nodeVar124 = ( vec3<f32>( 1.0 ) - nodeVar123 );
	nodeVar125 = nodeVar124;
	nodeVar126 = ( nodeVar122 * nodeVar125 );
	nodeVar127 = nodeVar126;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar128 = ( indirectDiffuse + nodeVar127 );
	indirectDiffuse = nodeVar128;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar129 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar130 = ( SpecularF90 * dfg.y );
	nodeVar131 = ( nodeVar129 + vec3<f32>( nodeVar130 ) );
	nodeVar132 = ( singleScatteringDielectric + nodeVar131 );
	singleScatteringDielectric = nodeVar132;
	nodeVar133 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar134 = nodeVar133;
	nodeVar135 = ( nodeVar134 * vec3<f32>( 0.047619 ) );
	nodeVar136 = ( SpecularColor + nodeVar135 );
	nodeVar137 = ( nodeVar131 * nodeVar136 );
	nodeVar138 = ( dfg.x + dfg.y );
	nodeVar139 = ( 1.0 - nodeVar138 );
	nodeVar140 = nodeVar139;
	nodeVar141 = ( vec3<f32>( nodeVar140 ) * nodeVar136 );
	nodeVar142 = ( vec3<f32>( 1.0 ) - nodeVar141 );
	nodeVar143 = nodeVar142;
	nodeVar144 = ( nodeVar137 / nodeVar143 );
	nodeVar145 = ( nodeVar144 * vec3<f32>( nodeVar140 ) );
	nodeVar146 = ( multiScatteringDielectric + nodeVar145 );
	multiScatteringDielectric = nodeVar146;
	nodeVar147 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar148 = ( SpecularF90 * dfg.y );
	nodeVar149 = ( nodeVar147 + vec3<f32>( nodeVar148 ) );
	nodeVar150 = ( singleScatteringMetallic + nodeVar149 );
	singleScatteringMetallic = nodeVar150;
	nodeVar151 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar152 = nodeVar151;
	nodeVar153 = ( nodeVar152 * vec3<f32>( 0.047619 ) );
	nodeVar154 = ( DiffuseColor.xyz + nodeVar153 );
	nodeVar155 = ( nodeVar149 * nodeVar154 );
	nodeVar156 = ( dfg.x + dfg.y );
	nodeVar157 = ( 1.0 - nodeVar156 );
	nodeVar158 = nodeVar157;
	nodeVar159 = ( vec3<f32>( nodeVar158 ) * nodeVar154 );
	nodeVar160 = ( vec3<f32>( 1.0 ) - nodeVar159 );
	nodeVar161 = nodeVar160;
	nodeVar162 = ( nodeVar155 / nodeVar161 );
	nodeVar163 = ( nodeVar162 * vec3<f32>( nodeVar158 ) );
	nodeVar164 = ( multiScatteringMetallic + nodeVar163 );
	multiScatteringMetallic = nodeVar164;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar165 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar166 = ( radiance * nodeVar165 );
	nodeVar167 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar168 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar169 = ( nodeVar167 * nodeVar168 );
	nodeVar170 = ( nodeVar166 + nodeVar169 );
	nodeVar171 = nodeVar170;
	nodeVar172 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar173 = ( vec3<f32>( 1.0 ) - nodeVar172 );
	nodeVar174 = nodeVar173;
	nodeVar175 = ( DiffuseContribution * nodeVar174 );
	nodeVar176 = ( nodeVar175 * nodeVar168 );
	nodeVar177 = nodeVar176;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar178 = ( indirectSpecular + nodeVar171 );
	indirectSpecular = nodeVar178;
	nodeVar179 = ( indirectDiffuse + nodeVar177 );
	indirectDiffuse = nodeVar179;
	ambientOcclusion = 1.0;
	nodeVar180 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar180;
	nodeVar181 = dot( normalView, positionViewDirection );
	nodeVar182 = ( clamp( nodeVar181, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar183 = ( Roughness * -16.0 );
	nodeVar184 = ( 1.0 - nodeVar183 );
	nodeVar185 = nodeVar184;
	nodeVar186 = ( - nodeVar185 );
	nodeVar187 = exp2( nodeVar186 );
	nodeVar188 = pow( nodeVar182, nodeVar187 );
	nodeVar189 = ( 1.0 - nodeVar188 );
	nodeVar190 = nodeVar189;
	nodeVar191 = ( ambientOcclusion - nodeVar190 );
	nodeVar192 = ( indirectSpecular * vec3<f32>( clamp( nodeVar191, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar192;
	nodeVar193 = ( directDiffuse + indirectDiffuse );
	nodeVar194 = mix( vec4<f32>( nodeVar193, 1.0 ), nodeVar43, Transmission );
	totalDiffuse = nodeVar194.xyz;
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
