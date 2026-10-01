// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform13_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform13 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform22_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform22 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform24_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform24 : texture_2d<f32>;

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
	nodeUniform11 : f32,
	nodeUniform12 : vec3<f32>,
	nodeUniform14 : mat4x4<f32>,
	nodeUniform15 : f32,
	nodeUniform16 : vec3<f32>,
	nodeUniform21 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform26 : f32,
	nodeUniform27 : f32,
	nodeUniform30 : f32,
	nodeUniform31 : f32,
	nodeUniform25 : vec3<f32>,
	nodeUniform17 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform29 : vec3<f32>,
	cameraPosition : vec3<f32>,
	nodeUniform23 : vec2<f32>
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
var<private> AttenuationDistance : f32;
var<private> AttenuationColor : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec2<f32>;
var<private> nodeVar14 : vec2<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec2<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : vec4<f32>;
var<private> nodeVar19 : vec2<f32>;
var<private> nodeVar20 : vec2<f32>;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec2<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec4<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec4<f32>;
var<private> nodeVar32 : vec4<f32>;
var<private> nodeVar33 : vec4<f32>;
var<private> nodeVar34 : vec2<f32>;
var<private> nodeVar35 : vec2<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : vec2<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : vec4<f32>;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : vec4<f32>;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : vec4<f32>;
var<private> nodeVar47 : vec4<f32>;
var<private> nodeVar48 : vec4<f32>;
var<private> nodeVar49 : vec4<f32>;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : vec2<f32>;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec4<f32>;
var<private> nodeVar65 : vec4<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
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
var<private> irradiance : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : f32;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : f32;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : f32;
var<private> nodeVar155 : f32;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar177 : vec3<f32>;
var<private> nodeVar178 : f32;
var<private> nodeVar179 : f32;
var<private> nodeVar180 : f32;
var<private> nodeVar181 : f32;
var<private> nodeVar182 : f32;
var<private> nodeVar183 : f32;
var<private> nodeVar184 : f32;
var<private> nodeVar185 : f32;
var<private> nodeVar186 : f32;
var<private> nodeVar187 : f32;
var<private> nodeVar188 : f32;
var<private> nodeVar189 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : vec4<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar192 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar193 : vec3<f32>;
var<private> nodeVar194 : vec4<f32>;

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
	@location( 3 ) v_positionWorld : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
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
	Thickness = object.nodeUniform10;
	AttenuationDistance = object.nodeUniform11;
	AttenuationColor = object.nodeUniform12;
	positionViewDirection = normalize( v_positionViewDirection );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
	normalView = NORMAL_normalView;
	nodeVar2 = ( normalize( refract( ( - positionViewDirection ), normalView, ( 1.0 / 1.5 ) ) ).xy * vec2<f32>( 0.6 ) );
	nodeVar3 = ( pow( normalView.z, -0.9 ) * 0.004 );
	nodeVar4 = textureSample( nodeUniform13, nodeUniform13_sampler, ( nodeVar2 + vec2<f32>( ( - nodeVar3 ), 0.0 ) ) );
	nodeVar5 = textureSample( nodeUniform13, nodeUniform13_sampler, ( nodeVar2 + vec2<f32>( 0.0, ( - nodeVar3 ) ) ) );
	nodeVar6 = textureSample( nodeUniform13, nodeUniform13_sampler, ( nodeVar2 + vec2<f32>( nodeVar3, nodeVar3 ) ) );
	nodeVar7 = pow( normalView.z, object.nodeUniform15 );
	nodeVar8 = ( ( ( vec3<f32>( nodeVar4.x, nodeVar5.y, nodeVar6.z ) * vec3<f32>( ( nodeVar7 * 60.0 ) ) ) + vec3<f32>( nodeVar7 ) ) * object.nodeUniform16 );
	EmissiveColor = ( ( nodeVar8 * vec3<f32>( ( pow( clamp( dot( positionViewDirection, ( - normalize( ( render.nodeUniform17 - v_positionView ) ) ) ), 0.0, 1.0 ), 3.0 ) + 0.1 ) ) ) * vec3<f32>( 0.02 ) );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar9 = ( render.cameraPosition - v_positionWorld );
	nodeVar10 = normalize( nodeVar9 );
	nodeVar11 = getVolumeTransmissionRay( normalWorld, nodeVar10, Thickness, IOR, object.nodeUniform21 );
	nodeVar12 = ( render.cameraProjectionMatrix * ( render.cameraViewMatrix * vec4<f32>( ( v_positionWorld + nodeVar11 ), 1.0 ) ) );
	nodeVar13 = ( nodeVar12.xy / vec2<f32>( nodeVar12.w ) );
	nodeVar13 = ( nodeVar13 + vec2<f32>( 1.0 ) );
	nodeVar13 = ( nodeVar13 / vec2<f32>( 2.0 ) );
	nodeVar13 = vec2<f32>( nodeVar13.x, ( 1.0 - nodeVar13.y ) );
	nodeVar14 = textureSample( nodeUniform22, nodeUniform22_sampler, vec2<f32>( Roughness, clamp( dot( normalWorld, nodeVar10 ), 0.0, 1.0 ) ) ).xy;
	nodeVar15 = ( DiffuseContribution * volumeAttenuation( length( nodeVar11 ), AttenuationColor, AttenuationDistance ) );
	let cameraViewport = vec4<f32>( 0.0, 0.0, render.nodeUniform23.x, render.nodeUniform23.y );
	nodeVar16 = ( ( ( nodeVar13 * cameraViewport.zw ) + cameraViewport.xy ) / render.nodeUniform23 );
	nodeVar17 = ( log2( cameraViewport.z ) * applyIorToRoughness( Roughness, IOR ) );
	nodeVar18 = vec4<f32>( ( vec2<f32>( 1.0 ) / vec2<f32>( textureDimensions( nodeUniform24, i32( nodeVar17 ) ) ) ), vec2<f32>( textureDimensions( nodeUniform24, i32( nodeVar17 ) ) ) );
	nodeVar19 = ( ( nodeVar16 * nodeVar18.zw ) + vec2<f32>( 0.5 ) );
	nodeVar20 = fract( nodeVar19 );
	nodeVar21 = ( ( 0.16666666666666666 * ( ( nodeVar20.x * ( ( nodeVar20.x * ( ( - nodeVar20.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar20.x * ( nodeVar20.x * ( ( 3.0 * nodeVar20.x ) - 6.0 ) ) ) + 4.0 ) ) );
	nodeVar22 = floor( nodeVar19 );
	nodeVar23 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar20.x * ( nodeVar20.x * ( ( 3.0 * nodeVar20.x ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar20.x * ( ( nodeVar20.x * ( ( - nodeVar20.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar20.x * ( nodeVar20.x * ( ( 3.0 * nodeVar20.x ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar24 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar20.y * ( nodeVar20.y * ( ( 3.0 * nodeVar20.y ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar20.y * ( ( nodeVar20.y * ( ( - nodeVar20.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar20.y * ( nodeVar20.y * ( ( 3.0 * nodeVar20.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar25 = floor( nodeVar17 );
	nodeVar26 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, ( ( vec2<f32>( ( nodeVar22.x + nodeVar23 ), ( nodeVar22.y + nodeVar24 ) ) - vec2<f32>( 0.5 ) ) * nodeVar18.xy ), nodeVar25 );
	nodeVar27 = ( ( 0.16666666666666666 * ( ( nodeVar20.x * ( ( nodeVar20.x * ( ( -3.0 * nodeVar20.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar20.x, 3.0 ) ) );
	nodeVar28 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar20.x, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar20.x * ( ( nodeVar20.x * ( ( -3.0 * nodeVar20.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar20.x, 3.0 ) ) ) ) );
	nodeVar29 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, ( ( vec2<f32>( ( nodeVar22.x + nodeVar28 ), ( nodeVar22.y + nodeVar24 ) ) - vec2<f32>( 0.5 ) ) * nodeVar18.xy ), nodeVar25 );
	nodeVar30 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar20.y, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar20.y * ( ( nodeVar20.y * ( ( -3.0 * nodeVar20.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar20.y, 3.0 ) ) ) ) );
	nodeVar31 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, ( ( vec2<f32>( ( nodeVar22.x + nodeVar23 ), ( nodeVar22.y + nodeVar30 ) ) - vec2<f32>( 0.5 ) ) * nodeVar18.xy ), nodeVar25 );
	nodeVar32 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, ( ( vec2<f32>( ( nodeVar22.x + nodeVar28 ), ( nodeVar22.y + nodeVar30 ) ) - vec2<f32>( 0.5 ) ) * nodeVar18.xy ), nodeVar25 );
	nodeVar33 = vec4<f32>( ( vec2<f32>( 1.0 ) / vec2<f32>( textureDimensions( nodeUniform24, i32( ( nodeVar17 + 1.0 ) ) ) ) ), vec2<f32>( textureDimensions( nodeUniform24, i32( ( nodeVar17 + 1.0 ) ) ) ) );
	nodeVar34 = ( ( nodeVar16 * nodeVar33.zw ) + vec2<f32>( 0.5 ) );
	nodeVar35 = fract( nodeVar34 );
	nodeVar36 = ( ( 0.16666666666666666 * ( ( nodeVar35.x * ( ( nodeVar35.x * ( ( - nodeVar35.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar35.x * ( nodeVar35.x * ( ( 3.0 * nodeVar35.x ) - 6.0 ) ) ) + 4.0 ) ) );
	nodeVar37 = floor( nodeVar34 );
	nodeVar38 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar35.x * ( nodeVar35.x * ( ( 3.0 * nodeVar35.x ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar35.x * ( ( nodeVar35.x * ( ( - nodeVar35.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar35.x * ( nodeVar35.x * ( ( 3.0 * nodeVar35.x ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar39 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar35.y * ( nodeVar35.y * ( ( 3.0 * nodeVar35.y ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar35.y * ( ( nodeVar35.y * ( ( - nodeVar35.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar35.y * ( nodeVar35.y * ( ( 3.0 * nodeVar35.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar40 = ceil( nodeVar17 );
	nodeVar41 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, ( ( vec2<f32>( ( nodeVar37.x + nodeVar38 ), ( nodeVar37.y + nodeVar39 ) ) - vec2<f32>( 0.5 ) ) * nodeVar33.xy ), nodeVar40 );
	nodeVar42 = ( ( 0.16666666666666666 * ( ( nodeVar35.x * ( ( nodeVar35.x * ( ( -3.0 * nodeVar35.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar35.x, 3.0 ) ) );
	nodeVar43 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar35.x, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar35.x * ( ( nodeVar35.x * ( ( -3.0 * nodeVar35.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar35.x, 3.0 ) ) ) ) );
	nodeVar44 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, ( ( vec2<f32>( ( nodeVar37.x + nodeVar43 ), ( nodeVar37.y + nodeVar39 ) ) - vec2<f32>( 0.5 ) ) * nodeVar33.xy ), nodeVar40 );
	nodeVar45 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar35.y, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar35.y * ( ( nodeVar35.y * ( ( -3.0 * nodeVar35.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar35.y, 3.0 ) ) ) ) );
	nodeVar46 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, ( ( vec2<f32>( ( nodeVar37.x + nodeVar38 ), ( nodeVar37.y + nodeVar45 ) ) - vec2<f32>( 0.5 ) ) * nodeVar33.xy ), nodeVar40 );
	nodeVar47 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, ( ( vec2<f32>( ( nodeVar37.x + nodeVar43 ), ( nodeVar37.y + nodeVar45 ) ) - vec2<f32>( 0.5 ) ) * nodeVar33.xy ), nodeVar40 );
	nodeVar48 = mix( ( ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar20.y * ( ( nodeVar20.y * ( ( - nodeVar20.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar20.y * ( nodeVar20.y * ( ( 3.0 * nodeVar20.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) * ( ( vec4<f32>( nodeVar21 ) * nodeVar26 ) + ( vec4<f32>( nodeVar27 ) * nodeVar29 ) ) ) + ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar20.y * ( ( nodeVar20.y * ( ( -3.0 * nodeVar20.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar20.y, 3.0 ) ) ) ) * ( ( vec4<f32>( nodeVar21 ) * nodeVar31 ) + ( vec4<f32>( nodeVar27 ) * nodeVar32 ) ) ) ), ( ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar35.y * ( ( nodeVar35.y * ( ( - nodeVar35.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar35.y * ( nodeVar35.y * ( ( 3.0 * nodeVar35.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) * ( ( vec4<f32>( nodeVar36 ) * nodeVar41 ) + ( vec4<f32>( nodeVar42 ) * nodeVar44 ) ) ) + ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar35.y * ( ( nodeVar35.y * ( ( -3.0 * nodeVar35.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar35.y, 3.0 ) ) ) ) * ( ( vec4<f32>( nodeVar36 ) * nodeVar46 ) + ( vec4<f32>( nodeVar42 ) * nodeVar47 ) ) ) ), fract( nodeVar17 ) );
	nodeVar49 = vec4<f32>( ( ( vec3<f32>( 1.0 ) - ( ( SpecularColorBlended * vec3<f32>( nodeVar14.x ) ) + vec3<f32>( ( SpecularF90 * nodeVar14.y ) ) ) ) * ( nodeVar15 * nodeVar48.xyz ) ), ( 1.0 - ( ( 1.0 - nodeVar48.w ) * ( ( ( nodeVar15.x + nodeVar15.y ) + nodeVar15.z ) / 3.0 ) ) ) );
	nodeVar50 = mix( 1.0, nodeVar49.w, Transmission );
	nodeVar51 = ( DiffuseColor.w * nodeVar50 );
	DiffuseColor.w = nodeVar51;
	nodeVar52 = dot( normalView, positionViewDirection );
	nodeVar53 = textureSample( nodeUniform22, nodeUniform22_sampler, vec2<f32>( Roughness, clamp( nodeVar52, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar53;
	nodeVar54 = ( dfg.x + dfg.y );
	nodeVar55 = ( 1.0 / nodeVar54 );
	nodeVar56 = nodeVar55;
	nodeVar57 = ( nodeVar56 - 1.0 );
	nodeVar58 = ( SpecularColorBlended * vec3<f32>( nodeVar57 ) );
	nodeVar59 = ( nodeVar58 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar59;
	nodeVar60 = ( render.nodeUniform17 - v_positionView );
	nodeVar61 = normalize( nodeVar60 );
	nodeVar62 = dot( normalView, nodeVar61 );
	nodeVar63 = ( render.nodeUniform28 - render.nodeUniform29 );
	nodeVar64 = vec4<f32>( nodeVar63, 0.0 );
	nodeVar65 = ( render.cameraViewMatrix * nodeVar64 );
	nodeVar66 = normalize( nodeVar65.xyz );
	nodeVar67 = nodeVar66;
	nodeVar68 = dot( nodeVar61, nodeVar67 );
	nodeVar69 = smoothstep( render.nodeUniform26, render.nodeUniform27, nodeVar68 );
	nodeVar70 = ( render.nodeUniform25 * vec3<f32>( nodeVar69 ) );

	if ( ( render.nodeUniform30 > 0.0 ) ) {

		nodeVar72 = length( nodeVar60 );
		nodeVar73 = ( nodeVar72 / render.nodeUniform30 );
		nodeVar74 = clamp( ( 1.0 - ( ( ( nodeVar73 * nodeVar73 ) * nodeVar73 ) * nodeVar73 ) ), 0.0, 1.0 );
		nodeVar71 = ( ( 1.0 / max( pow( nodeVar72, render.nodeUniform31 ), 0.01 ) ) * ( nodeVar74 * nodeVar74 ) );

	} else {

		nodeVar71 = ( 1.0 / max( pow( length( nodeVar60 ), render.nodeUniform31 ), 0.01 ) );

	}

	nodeVar75 = ( nodeVar70 * vec3<f32>( nodeVar71 ) );
	nodeVar76 = ( vec3<f32>( clamp( nodeVar62, 0.0, 1.0 ) ) * nodeVar75 );
	nodeVar77 = nodeVar76;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar78 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar79 = ( nodeVar77 * nodeVar78 );
	nodeVar80 = ( nodeVar61 + positionViewDirection );
	nodeVar81 = normalize( nodeVar80 );
	nodeVar82 = dot( positionViewDirection, nodeVar81 );
	nodeVar83 = clamp( nodeVar82, 0.0, 1.0 );
	nodeVar84 = exp2( ( ( ( nodeVar83 * -5.55473 ) - 6.98316 ) * nodeVar83 ) );
	nodeVar85 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar84 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar84 ) ) );
	nodeVar86 = ( vec3<f32>( 1.0 ) - nodeVar85 );
	nodeVar87 = nodeVar86;
	nodeVar88 = ( nodeVar79 * nodeVar87 );
	nodeVar89 = ( directDiffuse + nodeVar88 );
	directDiffuse = nodeVar89;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar90 = normalize( ( nodeVar61 + positionViewDirection ) );
	nodeVar91 = clamp( dot( positionViewDirection, nodeVar90 ), 0.0, 1.0 );
	nodeVar92 = exp2( ( ( ( nodeVar91 * -5.55473 ) - 6.98316 ) * nodeVar91 ) );
	nodeVar93 = ( Roughness * Roughness );
	nodeVar94 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar92 ) ) ) + vec3<f32>( ( 1.0 * nodeVar92 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar93, clamp( dot( normalView, nodeVar61 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar93, clamp( dot( normalView, nodeVar90 ), 0.0, 1.0 ) ) ) );
	nodeVar95 = ( nodeVar77 * nodeVar94 );
	nodeVar96 = ( nodeVar95 * multiScatteringCompensation );
	nodeVar97 = ( directSpecular + nodeVar96 );
	directSpecular = nodeVar97;
	nodeVar98 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar99 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar100 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar101 = ( SpecularF90 * dfg.y );
	nodeVar102 = ( nodeVar100 + vec3<f32>( nodeVar101 ) );
	nodeVar103 = ( nodeVar98 + nodeVar102 );
	nodeVar98 = nodeVar103;
	nodeVar104 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar105 = nodeVar104;
	nodeVar106 = ( nodeVar105 * vec3<f32>( 0.047619 ) );
	nodeVar107 = ( SpecularColor + nodeVar106 );
	nodeVar108 = ( nodeVar102 * nodeVar107 );
	nodeVar109 = ( dfg.x + dfg.y );
	nodeVar110 = ( 1.0 - nodeVar109 );
	nodeVar111 = nodeVar110;
	nodeVar112 = ( vec3<f32>( nodeVar111 ) * nodeVar107 );
	nodeVar113 = ( vec3<f32>( 1.0 ) - nodeVar112 );
	nodeVar114 = nodeVar113;
	nodeVar115 = ( nodeVar108 / nodeVar114 );
	nodeVar116 = ( nodeVar115 * vec3<f32>( nodeVar111 ) );
	nodeVar117 = ( nodeVar99 + nodeVar116 );
	nodeVar99 = nodeVar117;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar118 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar119 = ( irradiance * nodeVar118 );
	nodeVar120 = ( nodeVar98 + nodeVar99 );
	nodeVar121 = ( vec3<f32>( 1.0 ) - nodeVar120 );
	nodeVar122 = nodeVar121;
	nodeVar123 = ( nodeVar119 * nodeVar122 );
	nodeVar124 = nodeVar123;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar125 = ( indirectDiffuse + nodeVar124 );
	indirectDiffuse = nodeVar125;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar126 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar127 = ( SpecularF90 * dfg.y );
	nodeVar128 = ( nodeVar126 + vec3<f32>( nodeVar127 ) );
	nodeVar129 = ( singleScatteringDielectric + nodeVar128 );
	singleScatteringDielectric = nodeVar129;
	nodeVar130 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar131 = nodeVar130;
	nodeVar132 = ( nodeVar131 * vec3<f32>( 0.047619 ) );
	nodeVar133 = ( SpecularColor + nodeVar132 );
	nodeVar134 = ( nodeVar128 * nodeVar133 );
	nodeVar135 = ( dfg.x + dfg.y );
	nodeVar136 = ( 1.0 - nodeVar135 );
	nodeVar137 = nodeVar136;
	nodeVar138 = ( vec3<f32>( nodeVar137 ) * nodeVar133 );
	nodeVar139 = ( vec3<f32>( 1.0 ) - nodeVar138 );
	nodeVar140 = nodeVar139;
	nodeVar141 = ( nodeVar134 / nodeVar140 );
	nodeVar142 = ( nodeVar141 * vec3<f32>( nodeVar137 ) );
	nodeVar143 = ( multiScatteringDielectric + nodeVar142 );
	multiScatteringDielectric = nodeVar143;
	nodeVar144 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar145 = ( SpecularF90 * dfg.y );
	nodeVar146 = ( nodeVar144 + vec3<f32>( nodeVar145 ) );
	nodeVar147 = ( singleScatteringMetallic + nodeVar146 );
	singleScatteringMetallic = nodeVar147;
	nodeVar148 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar149 = nodeVar148;
	nodeVar150 = ( nodeVar149 * vec3<f32>( 0.047619 ) );
	nodeVar151 = ( DiffuseColor.xyz + nodeVar150 );
	nodeVar152 = ( nodeVar146 * nodeVar151 );
	nodeVar153 = ( dfg.x + dfg.y );
	nodeVar154 = ( 1.0 - nodeVar153 );
	nodeVar155 = nodeVar154;
	nodeVar156 = ( vec3<f32>( nodeVar155 ) * nodeVar151 );
	nodeVar157 = ( vec3<f32>( 1.0 ) - nodeVar156 );
	nodeVar158 = nodeVar157;
	nodeVar159 = ( nodeVar152 / nodeVar158 );
	nodeVar160 = ( nodeVar159 * vec3<f32>( nodeVar155 ) );
	nodeVar161 = ( multiScatteringMetallic + nodeVar160 );
	multiScatteringMetallic = nodeVar161;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar162 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar163 = ( radiance * nodeVar162 );
	nodeVar164 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar165 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar166 = ( nodeVar164 * nodeVar165 );
	nodeVar167 = ( nodeVar163 + nodeVar166 );
	nodeVar168 = nodeVar167;
	nodeVar169 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar170 = ( vec3<f32>( 1.0 ) - nodeVar169 );
	nodeVar171 = nodeVar170;
	nodeVar172 = ( DiffuseContribution * nodeVar171 );
	nodeVar173 = ( nodeVar172 * nodeVar165 );
	nodeVar174 = nodeVar173;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar175 = ( indirectSpecular + nodeVar168 );
	indirectSpecular = nodeVar175;
	nodeVar176 = ( indirectDiffuse + nodeVar174 );
	indirectDiffuse = nodeVar176;
	ambientOcclusion = 1.0;
	nodeVar177 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar177;
	nodeVar178 = dot( normalView, positionViewDirection );
	nodeVar179 = ( clamp( nodeVar178, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar180 = ( Roughness * -16.0 );
	nodeVar181 = ( 1.0 - nodeVar180 );
	nodeVar182 = nodeVar181;
	nodeVar183 = ( - nodeVar182 );
	nodeVar184 = exp2( nodeVar183 );
	nodeVar185 = pow( nodeVar179, nodeVar184 );
	nodeVar186 = ( 1.0 - nodeVar185 );
	nodeVar187 = nodeVar186;
	nodeVar188 = ( ambientOcclusion - nodeVar187 );
	nodeVar189 = ( indirectSpecular * vec3<f32>( clamp( nodeVar188, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar189;
	nodeVar190 = ( directDiffuse + indirectDiffuse );
	nodeVar191 = mix( vec4<f32>( nodeVar190, 1.0 ), nodeVar49, Transmission );
	totalDiffuse = nodeVar191.xyz;
	nodeVar192 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar192;
	nodeVar193 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar193;
	nodeVar194 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar194;

	// result

	output.color = nodeVar194;

	return output;

}
