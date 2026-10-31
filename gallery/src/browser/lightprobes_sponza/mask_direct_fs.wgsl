// Three.js r187 - Node System

// global
diagnostic( off, derivative_uniformity );


// directives


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform1_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform6_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform6 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform14_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform14 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform16_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform16 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform8 : f32,
	nodeUniform9 : mat3x3<f32>,
	nodeUniform11 : mat3x3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform13 : f32,
	nodeUniform15 : mat4x4<f32>,
	nodeUniform17 : mat3x3<f32>,
	nodeUniform18 : vec2<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform20 : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> Metalness : f32;
var<private> nodeVar1 : vec4<f32>;
var<private> Roughness : f32;
var<private> nodeVar2 : vec4<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_tangentView : vec3<f32>;
var<private> NORMAL_bitangentView : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> NORMAL_TBNViewMatrix : mat3x3<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar4 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn12 ( alpha : f32, dotNL : f32, dotNV : f32 ) -> f32 {

	

	let nodeConst0 = ( alpha * alpha );

	return ( 0.5 / max( ( ( dotNL * sqrt( ( nodeConst0 + ( ( 1.0 - nodeConst0 ) * ( dotNV * dotNV ) ) ) ) ) + ( dotNV * sqrt( ( nodeConst0 + ( ( 1.0 - nodeConst0 ) * ( dotNL * dotNL ) ) ) ) ) ), 0.000001 ) );

}


fn fn14 ( alpha : f32, dotNH : f32 ) -> f32 {

	

	let nodeConst0 = ( alpha * alpha );
	let nodeConst1 = ( 1.0 - ( ( dotNH * dotNH ) * ( 1.0 - nodeConst0 ) ) );

	return ( ( nodeConst0 / ( nodeConst1 * nodeConst1 ) ) * 0.3183098861837907 );

}




@fragment
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) v_tangentView : vec3<f32>,
	@location( 2 ) v_bitangentView : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) NORMAL_v_bitangentView : vec3<f32>,
	@location( 5 ) nodeVarying9 : vec2<f32>,
	@builtin( front_facing ) isFront : bool ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying9, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );

	if ( ( DiffuseColor.w <= object.nodeUniform4 ) ) {

		discard;
		

	}

	DiffuseColor.w = 1.0;
	nodeVar1 = textureSample( nodeUniform6, nodeUniform6_sampler, ( object.nodeUniform7 * vec3<f32>( nodeVarying9, 1.0 ) ).xy );
	Metalness = ( object.nodeUniform5 * nodeVar1.z );
	nodeVar2 = textureSample( nodeUniform6, nodeUniform6_sampler, ( object.nodeUniform9 * vec3<f32>( nodeVarying9, 1.0 ) ).xy );
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst1 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( object.nodeUniform8 * nodeVar2.y ), 0.045 ) + max( max( nodeConst1.x, nodeConst1.y ), nodeConst1.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - ( object.nodeUniform5 * nodeVar1.z ) ) ) );
	EmissiveColor = ( object.nodeUniform12 * vec3<f32>( object.nodeUniform13 ) );
	let nodeConst2 = ( ( f32( isFront ) * 2.0 ) - 1.0 );
	NORMAL_tangentView = ( normalize( v_tangentView ) * vec3<f32>( nodeConst2 ) );
	NORMAL_bitangentView = ( normalize( NORMAL_v_bitangentView ) * vec3<f32>( nodeConst2 ) );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( nodeConst2 ) );
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar3 = textureSample( nodeUniform16, nodeUniform16_sampler, ( object.nodeUniform17 * vec3<f32>( nodeVarying9, 1.0 ) ).xy );
	let nodeConst4 = ( ( nodeVar3 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeConst4.xy * object.nodeUniform18 ), nodeConst4.z ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar4 = textureSample( nodeUniform14, nodeUniform14_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar4;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst5 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst5 );
	let nodeConst6 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst7 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst5 * nodeConst6 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst7 ) * nodeConst6 ) ) ) * vec3<f32>( nodeConst7 ) ) );
	let nodeConst8 = normalize( ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform20, 0.0 ) ).xyz );
	nodeVar5 = ( vec3<f32>( clamp( dot( normalView, nodeConst8 ), 0.0, 1.0 ) ) * render.nodeUniform21 );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst9 = clamp( dot( positionViewDirection, normalize( ( nodeConst8 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst10 = exp2( ( ( ( nodeConst9 * -5.55473 ) - 6.98316 ) * nodeConst9 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar5 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst10 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst10 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst11 = normalize( ( nodeConst8 + positionViewDirection ) );
	let nodeConst12 = clamp( dot( positionViewDirection, nodeConst11 ), 0.0, 1.0 );
	let nodeConst13 = exp2( ( ( ( nodeConst12 * -5.55473 ) - 6.98316 ) * nodeConst12 ) );
	let nodeConst14 = max( Roughness, 0.045 );
	let nodeConst15 = ( nodeConst14 * nodeConst14 );
	directSpecular = ( directSpecular + ( ( nodeVar5 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst13 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst13 ) ) ) * vec3<f32>( fn12( nodeConst15, clamp( dot( normalView, nodeConst8 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn14( nodeConst15, clamp( dot( normalView, nodeConst11 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar6 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar6 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst16 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst16 );
	let nodeConst17 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst18 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst16 * nodeConst17 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst18 ) * nodeConst17 ) ) ) * vec3<f32>( nodeConst18 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst19 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar7 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst19 ) );
	nodeVar8 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst19 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar7 );
	indirectDiffuse = ( indirectDiffuse + nodeVar8 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst20 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst20;

	// result

	output.color = nodeConst20;

	return output;

}
