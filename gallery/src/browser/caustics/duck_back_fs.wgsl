// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );

// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform20_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform20 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform22_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform22 : texture_2d<f32>;

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
	nodeUniform13 : vec3<f32>,
	nodeUniform14 : f32,
	nodeUniform17 : mat4x4<f32>,
	nodeUniform19 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform25 : f32,
	nodeUniform26 : f32,
	nodeUniform29 : f32,
	nodeUniform30 : f32,
	nodeUniform24 : vec3<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	cameraPosition : vec3<f32>,
	nodeUniform21 : vec2<f32>
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
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec2<f32>;
var<private> nodeVar7 : vec2<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec2<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : vec2<f32>;
var<private> nodeVar13 : vec2<f32>;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec2<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec4<f32>;
var<private> nodeVar25 : vec4<f32>;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : vec2<f32>;
var<private> nodeVar28 : vec2<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec2<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec4<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : vec4<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : vec4<f32>;
var<private> nodeVar40 : vec4<f32>;
var<private> nodeVar41 : vec4<f32>;
var<private> nodeVar42 : vec4<f32>;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : vec2<f32>;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec4<f32>;
var<private> nodeVar58 : vec4<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : f32;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : f32;
var<private> nodeVar129 : f32;
var<private> nodeVar130 : f32;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : f32;
var<private> nodeVar172 : f32;
var<private> nodeVar173 : f32;
var<private> nodeVar174 : f32;
var<private> nodeVar175 : f32;
var<private> nodeVar176 : f32;
var<private> nodeVar177 : f32;
var<private> nodeVar178 : f32;
var<private> nodeVar179 : f32;
var<private> nodeVar180 : f32;
var<private> nodeVar181 : f32;
var<private> nodeVar182 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar183 : vec3<f32>;
var<private> nodeVar184 : vec4<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar185 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> nodeVar187 : vec4<f32>;

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
	@location( 2 ) v_positionWorld : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32> ) -> OutputStruct {

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
	EmissiveColor = ( object.nodeUniform13 * vec3<f32>( object.nodeUniform14 ) );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
	normalView = NORMAL_normalView;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar2 = ( render.cameraPosition - v_positionWorld );
	nodeVar3 = normalize( nodeVar2 );
	nodeVar4 = getVolumeTransmissionRay( normalWorld, nodeVar3, Thickness, IOR, object.nodeUniform19 );
	nodeVar5 = ( render.cameraProjectionMatrix * ( render.cameraViewMatrix * vec4<f32>( ( v_positionWorld + nodeVar4 ), 1.0 ) ) );
	nodeVar6 = ( nodeVar5.xy / vec2<f32>( nodeVar5.w ) );
	nodeVar6 = ( nodeVar6 + vec2<f32>( 1.0 ) );
	nodeVar6 = ( nodeVar6 / vec2<f32>( 2.0 ) );
	nodeVar6 = vec2<f32>( nodeVar6.x, ( 1.0 - nodeVar6.y ) );
	nodeVar7 = textureSample( nodeUniform20, nodeUniform20_sampler, vec2<f32>( Roughness, clamp( dot( normalWorld, nodeVar3 ), 0.0, 1.0 ) ) ).xy;
	nodeVar8 = ( DiffuseContribution * volumeAttenuation( length( nodeVar4 ), AttenuationColor, AttenuationDistance ) );
	let cameraViewport = vec4<f32>( 0.0, 0.0, render.nodeUniform21.x, render.nodeUniform21.y );
	nodeVar9 = ( ( ( nodeVar6 * cameraViewport.zw ) + cameraViewport.xy ) / render.nodeUniform21 );
	nodeVar10 = ( log2( cameraViewport.z ) * applyIorToRoughness( Roughness, IOR ) );
	nodeVar11 = vec4<f32>( ( vec2<f32>( 1.0 ) / vec2<f32>( textureDimensions( nodeUniform22, i32( nodeVar10 ) ) ) ), vec2<f32>( textureDimensions( nodeUniform22, i32( nodeVar10 ) ) ) );
	nodeVar12 = ( ( nodeVar9 * nodeVar11.zw ) + vec2<f32>( 0.5 ) );
	nodeVar13 = fract( nodeVar12 );
	nodeVar14 = ( ( 0.16666666666666666 * ( ( nodeVar13.x * ( ( nodeVar13.x * ( ( - nodeVar13.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar13.x * ( nodeVar13.x * ( ( 3.0 * nodeVar13.x ) - 6.0 ) ) ) + 4.0 ) ) );
	nodeVar15 = floor( nodeVar12 );
	nodeVar16 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar13.x * ( nodeVar13.x * ( ( 3.0 * nodeVar13.x ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar13.x * ( ( nodeVar13.x * ( ( - nodeVar13.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar13.x * ( nodeVar13.x * ( ( 3.0 * nodeVar13.x ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar17 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar13.y * ( nodeVar13.y * ( ( 3.0 * nodeVar13.y ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar13.y * ( ( nodeVar13.y * ( ( - nodeVar13.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar13.y * ( nodeVar13.y * ( ( 3.0 * nodeVar13.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar18 = floor( nodeVar10 );
	nodeVar19 = textureSampleLevel( nodeUniform22, nodeUniform22_sampler, ( ( vec2<f32>( ( nodeVar15.x + nodeVar16 ), ( nodeVar15.y + nodeVar17 ) ) - vec2<f32>( 0.5 ) ) * nodeVar11.xy ), nodeVar18 );
	nodeVar20 = ( ( 0.16666666666666666 * ( ( nodeVar13.x * ( ( nodeVar13.x * ( ( -3.0 * nodeVar13.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar13.x, 3.0 ) ) );
	nodeVar21 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar13.x, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar13.x * ( ( nodeVar13.x * ( ( -3.0 * nodeVar13.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar13.x, 3.0 ) ) ) ) );
	nodeVar22 = textureSampleLevel( nodeUniform22, nodeUniform22_sampler, ( ( vec2<f32>( ( nodeVar15.x + nodeVar21 ), ( nodeVar15.y + nodeVar17 ) ) - vec2<f32>( 0.5 ) ) * nodeVar11.xy ), nodeVar18 );
	nodeVar23 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar13.y, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar13.y * ( ( nodeVar13.y * ( ( -3.0 * nodeVar13.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar13.y, 3.0 ) ) ) ) );
	nodeVar24 = textureSampleLevel( nodeUniform22, nodeUniform22_sampler, ( ( vec2<f32>( ( nodeVar15.x + nodeVar16 ), ( nodeVar15.y + nodeVar23 ) ) - vec2<f32>( 0.5 ) ) * nodeVar11.xy ), nodeVar18 );
	nodeVar25 = textureSampleLevel( nodeUniform22, nodeUniform22_sampler, ( ( vec2<f32>( ( nodeVar15.x + nodeVar21 ), ( nodeVar15.y + nodeVar23 ) ) - vec2<f32>( 0.5 ) ) * nodeVar11.xy ), nodeVar18 );
	nodeVar26 = vec4<f32>( ( vec2<f32>( 1.0 ) / vec2<f32>( textureDimensions( nodeUniform22, i32( ( nodeVar10 + 1.0 ) ) ) ) ), vec2<f32>( textureDimensions( nodeUniform22, i32( ( nodeVar10 + 1.0 ) ) ) ) );
	nodeVar27 = ( ( nodeVar9 * nodeVar26.zw ) + vec2<f32>( 0.5 ) );
	nodeVar28 = fract( nodeVar27 );
	nodeVar29 = ( ( 0.16666666666666666 * ( ( nodeVar28.x * ( ( nodeVar28.x * ( ( - nodeVar28.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar28.x * ( nodeVar28.x * ( ( 3.0 * nodeVar28.x ) - 6.0 ) ) ) + 4.0 ) ) );
	nodeVar30 = floor( nodeVar27 );
	nodeVar31 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar28.x * ( nodeVar28.x * ( ( 3.0 * nodeVar28.x ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar28.x * ( ( nodeVar28.x * ( ( - nodeVar28.x ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar28.x * ( nodeVar28.x * ( ( 3.0 * nodeVar28.x ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar32 = ( -1.0 + ( ( 0.16666666666666666 * ( ( nodeVar28.y * ( nodeVar28.y * ( ( 3.0 * nodeVar28.y ) - 6.0 ) ) ) + 4.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar28.y * ( ( nodeVar28.y * ( ( - nodeVar28.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar28.y * ( nodeVar28.y * ( ( 3.0 * nodeVar28.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) );
	nodeVar33 = ceil( nodeVar10 );
	nodeVar34 = textureSampleLevel( nodeUniform22, nodeUniform22_sampler, ( ( vec2<f32>( ( nodeVar30.x + nodeVar31 ), ( nodeVar30.y + nodeVar32 ) ) - vec2<f32>( 0.5 ) ) * nodeVar26.xy ), nodeVar33 );
	nodeVar35 = ( ( 0.16666666666666666 * ( ( nodeVar28.x * ( ( nodeVar28.x * ( ( -3.0 * nodeVar28.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar28.x, 3.0 ) ) );
	nodeVar36 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar28.x, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar28.x * ( ( nodeVar28.x * ( ( -3.0 * nodeVar28.x ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar28.x, 3.0 ) ) ) ) );
	nodeVar37 = textureSampleLevel( nodeUniform22, nodeUniform22_sampler, ( ( vec2<f32>( ( nodeVar30.x + nodeVar36 ), ( nodeVar30.y + nodeVar32 ) ) - vec2<f32>( 0.5 ) ) * nodeVar26.xy ), nodeVar33 );
	nodeVar38 = ( 1.0 + ( ( 0.16666666666666666 * pow( nodeVar28.y, 3.0 ) ) / ( ( 0.16666666666666666 * ( ( nodeVar28.y * ( ( nodeVar28.y * ( ( -3.0 * nodeVar28.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar28.y, 3.0 ) ) ) ) );
	nodeVar39 = textureSampleLevel( nodeUniform22, nodeUniform22_sampler, ( ( vec2<f32>( ( nodeVar30.x + nodeVar31 ), ( nodeVar30.y + nodeVar38 ) ) - vec2<f32>( 0.5 ) ) * nodeVar26.xy ), nodeVar33 );
	nodeVar40 = textureSampleLevel( nodeUniform22, nodeUniform22_sampler, ( ( vec2<f32>( ( nodeVar30.x + nodeVar36 ), ( nodeVar30.y + nodeVar38 ) ) - vec2<f32>( 0.5 ) ) * nodeVar26.xy ), nodeVar33 );
	nodeVar41 = mix( ( ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar13.y * ( ( nodeVar13.y * ( ( - nodeVar13.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar13.y * ( nodeVar13.y * ( ( 3.0 * nodeVar13.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) * ( ( vec4<f32>( nodeVar14 ) * nodeVar19 ) + ( vec4<f32>( nodeVar20 ) * nodeVar22 ) ) ) + ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar13.y * ( ( nodeVar13.y * ( ( -3.0 * nodeVar13.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar13.y, 3.0 ) ) ) ) * ( ( vec4<f32>( nodeVar14 ) * nodeVar24 ) + ( vec4<f32>( nodeVar20 ) * nodeVar25 ) ) ) ), ( ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar28.y * ( ( nodeVar28.y * ( ( - nodeVar28.y ) + 3.0 ) ) - 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * ( ( nodeVar28.y * ( nodeVar28.y * ( ( 3.0 * nodeVar28.y ) - 6.0 ) ) ) + 4.0 ) ) ) ) * ( ( vec4<f32>( nodeVar29 ) * nodeVar34 ) + ( vec4<f32>( nodeVar35 ) * nodeVar37 ) ) ) + ( vec4<f32>( ( ( 0.16666666666666666 * ( ( nodeVar28.y * ( ( nodeVar28.y * ( ( -3.0 * nodeVar28.y ) + 3.0 ) ) + 3.0 ) ) + 1.0 ) ) + ( 0.16666666666666666 * pow( nodeVar28.y, 3.0 ) ) ) ) * ( ( vec4<f32>( nodeVar29 ) * nodeVar39 ) + ( vec4<f32>( nodeVar35 ) * nodeVar40 ) ) ) ), fract( nodeVar10 ) );
	nodeVar42 = vec4<f32>( ( ( vec3<f32>( 1.0 ) - ( ( SpecularColorBlended * vec3<f32>( nodeVar7.x ) ) + vec3<f32>( ( SpecularF90 * nodeVar7.y ) ) ) ) * ( nodeVar8 * nodeVar41.xyz ) ), ( 1.0 - ( ( 1.0 - nodeVar41.w ) * ( ( ( nodeVar8.x + nodeVar8.y ) + nodeVar8.z ) / 3.0 ) ) ) );
	nodeVar43 = mix( 1.0, nodeVar42.w, Transmission );
	nodeVar44 = ( DiffuseColor.w * nodeVar43 );
	DiffuseColor.w = nodeVar44;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar45 = dot( normalView, positionViewDirection );
	nodeVar46 = textureSample( nodeUniform20, nodeUniform20_sampler, vec2<f32>( Roughness, clamp( nodeVar45, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar46;
	nodeVar47 = ( dfg.x + dfg.y );
	nodeVar48 = ( 1.0 / nodeVar47 );
	nodeVar49 = nodeVar48;
	nodeVar50 = ( nodeVar49 - 1.0 );
	nodeVar51 = ( SpecularColorBlended * vec3<f32>( nodeVar50 ) );
	nodeVar52 = ( nodeVar51 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar52;
	nodeVar53 = ( render.nodeUniform23 - v_positionView );
	nodeVar54 = normalize( nodeVar53 );
	nodeVar55 = dot( normalView, nodeVar54 );
	nodeVar56 = ( render.nodeUniform27 - render.nodeUniform28 );
	nodeVar57 = vec4<f32>( nodeVar56, 0.0 );
	nodeVar58 = ( render.cameraViewMatrix * nodeVar57 );
	nodeVar59 = normalize( nodeVar58.xyz );
	nodeVar60 = nodeVar59;
	nodeVar61 = dot( nodeVar54, nodeVar60 );
	nodeVar62 = smoothstep( render.nodeUniform25, render.nodeUniform26, nodeVar61 );
	nodeVar63 = ( render.nodeUniform24 * vec3<f32>( nodeVar62 ) );

	if ( ( render.nodeUniform29 > 0.0 ) ) {

		nodeVar65 = length( nodeVar53 );
		nodeVar66 = ( nodeVar65 / render.nodeUniform29 );
		nodeVar67 = clamp( ( 1.0 - ( ( ( nodeVar66 * nodeVar66 ) * nodeVar66 ) * nodeVar66 ) ), 0.0, 1.0 );
		nodeVar64 = ( ( 1.0 / max( pow( nodeVar65, render.nodeUniform30 ), 0.01 ) ) * ( nodeVar67 * nodeVar67 ) );

	} else {

		nodeVar64 = ( 1.0 / max( pow( length( nodeVar53 ), render.nodeUniform30 ), 0.01 ) );

	}

	nodeVar68 = ( nodeVar63 * vec3<f32>( nodeVar64 ) );
	nodeVar69 = ( vec3<f32>( clamp( nodeVar55, 0.0, 1.0 ) ) * nodeVar68 );
	nodeVar70 = nodeVar69;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar71 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar72 = ( nodeVar70 * nodeVar71 );
	nodeVar73 = ( nodeVar54 + positionViewDirection );
	nodeVar74 = normalize( nodeVar73 );
	nodeVar75 = dot( positionViewDirection, nodeVar74 );
	nodeVar76 = clamp( nodeVar75, 0.0, 1.0 );
	nodeVar77 = exp2( ( ( ( nodeVar76 * -5.55473 ) - 6.98316 ) * nodeVar76 ) );
	nodeVar78 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar77 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar77 ) ) );
	nodeVar79 = ( vec3<f32>( 1.0 ) - nodeVar78 );
	nodeVar80 = nodeVar79;
	nodeVar81 = ( nodeVar72 * nodeVar80 );
	nodeVar82 = ( directDiffuse + nodeVar81 );
	directDiffuse = nodeVar82;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar83 = normalize( ( nodeVar54 + positionViewDirection ) );
	nodeVar84 = clamp( dot( positionViewDirection, nodeVar83 ), 0.0, 1.0 );
	nodeVar85 = exp2( ( ( ( nodeVar84 * -5.55473 ) - 6.98316 ) * nodeVar84 ) );
	nodeVar86 = ( Roughness * Roughness );
	nodeVar87 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar85 ) ) ) + vec3<f32>( ( 1.0 * nodeVar85 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar86, clamp( dot( normalView, nodeVar54 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar86, clamp( dot( normalView, nodeVar83 ), 0.0, 1.0 ) ) ) );
	nodeVar88 = ( nodeVar70 * nodeVar87 );
	nodeVar89 = ( nodeVar88 * multiScatteringCompensation );
	nodeVar90 = ( directSpecular + nodeVar89 );
	directSpecular = nodeVar90;
	nodeVar91 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar92 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar93 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar94 = ( SpecularF90 * dfg.y );
	nodeVar95 = ( nodeVar93 + vec3<f32>( nodeVar94 ) );
	nodeVar96 = ( nodeVar91 + nodeVar95 );
	nodeVar91 = nodeVar96;
	nodeVar97 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar98 = nodeVar97;
	nodeVar99 = ( nodeVar98 * vec3<f32>( 0.047619 ) );
	nodeVar100 = ( SpecularColor + nodeVar99 );
	nodeVar101 = ( nodeVar95 * nodeVar100 );
	nodeVar102 = ( dfg.x + dfg.y );
	nodeVar103 = ( 1.0 - nodeVar102 );
	nodeVar104 = nodeVar103;
	nodeVar105 = ( vec3<f32>( nodeVar104 ) * nodeVar100 );
	nodeVar106 = ( vec3<f32>( 1.0 ) - nodeVar105 );
	nodeVar107 = nodeVar106;
	nodeVar108 = ( nodeVar101 / nodeVar107 );
	nodeVar109 = ( nodeVar108 * vec3<f32>( nodeVar104 ) );
	nodeVar110 = ( nodeVar92 + nodeVar109 );
	nodeVar92 = nodeVar110;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar111 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar112 = ( irradiance * nodeVar111 );
	nodeVar113 = ( nodeVar91 + nodeVar92 );
	nodeVar114 = ( vec3<f32>( 1.0 ) - nodeVar113 );
	nodeVar115 = nodeVar114;
	nodeVar116 = ( nodeVar112 * nodeVar115 );
	nodeVar117 = nodeVar116;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar118 = ( indirectDiffuse + nodeVar117 );
	indirectDiffuse = nodeVar118;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar119 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar120 = ( SpecularF90 * dfg.y );
	nodeVar121 = ( nodeVar119 + vec3<f32>( nodeVar120 ) );
	nodeVar122 = ( singleScatteringDielectric + nodeVar121 );
	singleScatteringDielectric = nodeVar122;
	nodeVar123 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar124 = nodeVar123;
	nodeVar125 = ( nodeVar124 * vec3<f32>( 0.047619 ) );
	nodeVar126 = ( SpecularColor + nodeVar125 );
	nodeVar127 = ( nodeVar121 * nodeVar126 );
	nodeVar128 = ( dfg.x + dfg.y );
	nodeVar129 = ( 1.0 - nodeVar128 );
	nodeVar130 = nodeVar129;
	nodeVar131 = ( vec3<f32>( nodeVar130 ) * nodeVar126 );
	nodeVar132 = ( vec3<f32>( 1.0 ) - nodeVar131 );
	nodeVar133 = nodeVar132;
	nodeVar134 = ( nodeVar127 / nodeVar133 );
	nodeVar135 = ( nodeVar134 * vec3<f32>( nodeVar130 ) );
	nodeVar136 = ( multiScatteringDielectric + nodeVar135 );
	multiScatteringDielectric = nodeVar136;
	nodeVar137 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar138 = ( SpecularF90 * dfg.y );
	nodeVar139 = ( nodeVar137 + vec3<f32>( nodeVar138 ) );
	nodeVar140 = ( singleScatteringMetallic + nodeVar139 );
	singleScatteringMetallic = nodeVar140;
	nodeVar141 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar142 = nodeVar141;
	nodeVar143 = ( nodeVar142 * vec3<f32>( 0.047619 ) );
	nodeVar144 = ( DiffuseColor.xyz + nodeVar143 );
	nodeVar145 = ( nodeVar139 * nodeVar144 );
	nodeVar146 = ( dfg.x + dfg.y );
	nodeVar147 = ( 1.0 - nodeVar146 );
	nodeVar148 = nodeVar147;
	nodeVar149 = ( vec3<f32>( nodeVar148 ) * nodeVar144 );
	nodeVar150 = ( vec3<f32>( 1.0 ) - nodeVar149 );
	nodeVar151 = nodeVar150;
	nodeVar152 = ( nodeVar145 / nodeVar151 );
	nodeVar153 = ( nodeVar152 * vec3<f32>( nodeVar148 ) );
	nodeVar154 = ( multiScatteringMetallic + nodeVar153 );
	multiScatteringMetallic = nodeVar154;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar155 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar156 = ( radiance * nodeVar155 );
	nodeVar157 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar158 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar159 = ( nodeVar157 * nodeVar158 );
	nodeVar160 = ( nodeVar156 + nodeVar159 );
	nodeVar161 = nodeVar160;
	nodeVar162 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar163 = ( vec3<f32>( 1.0 ) - nodeVar162 );
	nodeVar164 = nodeVar163;
	nodeVar165 = ( DiffuseContribution * nodeVar164 );
	nodeVar166 = ( nodeVar165 * nodeVar158 );
	nodeVar167 = nodeVar166;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar168 = ( indirectSpecular + nodeVar161 );
	indirectSpecular = nodeVar168;
	nodeVar169 = ( indirectDiffuse + nodeVar167 );
	indirectDiffuse = nodeVar169;
	ambientOcclusion = 1.0;
	nodeVar170 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar170;
	nodeVar171 = dot( normalView, positionViewDirection );
	nodeVar172 = ( clamp( nodeVar171, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar173 = ( Roughness * -16.0 );
	nodeVar174 = ( 1.0 - nodeVar173 );
	nodeVar175 = nodeVar174;
	nodeVar176 = ( - nodeVar175 );
	nodeVar177 = exp2( nodeVar176 );
	nodeVar178 = pow( nodeVar172, nodeVar177 );
	nodeVar179 = ( 1.0 - nodeVar178 );
	nodeVar180 = nodeVar179;
	nodeVar181 = ( ambientOcclusion - nodeVar180 );
	nodeVar182 = ( indirectSpecular * vec3<f32>( clamp( nodeVar181, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar182;
	nodeVar183 = ( directDiffuse + indirectDiffuse );
	nodeVar184 = mix( vec4<f32>( nodeVar183, 1.0 ), nodeVar42, Transmission );
	totalDiffuse = nodeVar184.xyz;
	nodeVar185 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar185;
	nodeVar186 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar186;
	nodeVar187 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar187;

	// result

	output.color = nodeVar187;

	return output;

}
