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
@binding( 1 ) @group( 2 ) var nodeUniform1_sampler : sampler;
@binding( 2 ) @group( 2 ) var nodeUniform1 : texture_2d<f32>;
@binding( 3 ) @group( 2 ) var nodeUniform11_sampler : sampler;
@binding( 4 ) @group( 2 ) var nodeUniform11 : texture_2d<f32>;
@binding( 5 ) @group( 2 ) var nodeUniform20_sampler : sampler;
@binding( 6 ) @group( 2 ) var nodeUniform20 : texture_3d<f32>;

struct cameraViewMatricesStruct {
	value : array< mat4x4<f32>, 48 >
};
@binding( 0 ) @group( 0 )
var<uniform> cameraViewMatrices : cameraViewMatricesStruct;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : f32,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform17 : vec3<f32>,
	nodeUniform18 : vec3<f32>
};
@binding( 0 ) @group( 2 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform21 : f32,
	nodeUniform19 : f32,
	nodeUniform15 : vec3<f32>,
	nodeUniform14 : vec3<f32>
};
@binding( 1 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar1 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> lightProbeIrradiance : vec3<f32>;
var<private> lightProbePriority : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
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
fn main( @location( 0 ) @interpolate(flat, either) v_cameraIndex : u32,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32>,
	@location( 4 ) nodeVarying8 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying8, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform4;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform5, 0.045 ) + max( max( nodeConst0.x, nodeConst0.y ), nodeConst0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform4 ) ) );
	EmissiveColor = ( object.nodeUniform9 * vec3<f32>( object.nodeUniform10 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar1 = textureSample( nodeUniform11, nodeUniform11_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar1;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst1 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst1 );
	let nodeConst2 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst3 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst1 * nodeConst2 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst3 ) * nodeConst2 ) ) ) * vec3<f32>( nodeConst3 ) ) );
	let nodeConst4 = normalize( ( cameraViewMatrices.value[ v_cameraIndex ] * vec4<f32>( render.nodeUniform14, 0.0 ) ).xyz );
	nodeVar2 = ( vec3<f32>( clamp( dot( normalView, nodeConst4 ), 0.0, 1.0 ) ) * render.nodeUniform15 );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst5 = clamp( dot( positionViewDirection, normalize( ( nodeConst4 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst6 = exp2( ( ( ( nodeConst5 * -5.55473 ) - 6.98316 ) * nodeConst5 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar2 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst6 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst6 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst7 = normalize( ( nodeConst4 + positionViewDirection ) );
	let nodeConst8 = clamp( dot( positionViewDirection, nodeConst7 ), 0.0, 1.0 );
	let nodeConst9 = exp2( ( ( ( nodeConst8 * -5.55473 ) - 6.98316 ) * nodeConst8 ) );
	let nodeConst10 = max( Roughness, 0.045 );
	let nodeConst11 = ( nodeConst10 * nodeConst10 );
	directSpecular = ( directSpecular + ( ( nodeVar2 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst9 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst9 ) ) ) * vec3<f32>( fn12( nodeConst11, clamp( dot( normalView, nodeConst4 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn14( nodeConst11, clamp( dot( normalView, nodeConst7 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	lightProbeIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	lightProbePriority = ( - 1000000.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * cameraViewMatrices.value[ v_cameraIndex ] ).xyz );
	let nodeConst12 = ( ( object.nodeUniform16 - object.nodeUniform17 ) / object.nodeUniform18 );
	let nodeConst13 = ( v_positionWorld + ( ( normalWorld * nodeConst12 ) * vec3<f32>( 0.5 ) ) );
	let nodeConst14 = ( nodeConst12 * vec3<f32>( 0.01 ) );

	if ( ( ( all( ( nodeConst13 >= ( object.nodeUniform17 - nodeConst14 ) ) ) && all( ( nodeConst13 <= ( object.nodeUniform16 + nodeConst14 ) ) ) ) && ( render.nodeUniform19 >= lightProbePriority ) ) ) {

		let nodeConst15 = ( object.nodeUniform16 - object.nodeUniform17 );
		let nodeConst16 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform18 );
		let nodeConst17 = clamp( ( ( ( v_positionWorld + ( ( normalWorld * ( nodeConst15 / object.nodeUniform18 ) ) * vec3<f32>( 0.5 ) ) ) - object.nodeUniform17 ) / nodeConst15 ), nodeConst16, ( vec3<f32>( 1.0 ) - nodeConst16 ) );
		let nodeConst18 = ( ( nodeConst17.z * object.nodeUniform18.z ) + 1.0 );
		let nodeConst19 = ( object.nodeUniform18.z + 2.0 );
		let nodeConst20 = ( nodeConst19 * 7.0 );
		nodeVar3 = textureSample( nodeUniform20, nodeUniform20_sampler, vec3<f32>( nodeConst17.xy, ( ( nodeConst18 + ( nodeConst19 * 0.0 ) ) / nodeConst20 ) ) );
		nodeVar4 = textureSample( nodeUniform20, nodeUniform20_sampler, vec3<f32>( nodeConst17.xy, ( ( nodeConst18 + ( nodeConst19 * 1.0 ) ) / nodeConst20 ) ) );
		nodeVar5 = textureSample( nodeUniform20, nodeUniform20_sampler, vec3<f32>( nodeConst17.xy, ( ( nodeConst18 + ( nodeConst19 * 2.0 ) ) / nodeConst20 ) ) );
		nodeVar6 = textureSample( nodeUniform20, nodeUniform20_sampler, vec3<f32>( nodeConst17.xy, ( ( nodeConst18 + ( nodeConst19 * 3.0 ) ) / nodeConst20 ) ) );
		nodeVar7 = textureSample( nodeUniform20, nodeUniform20_sampler, vec3<f32>( nodeConst17.xy, ( ( nodeConst18 + ( nodeConst19 * 4.0 ) ) / nodeConst20 ) ) );
		nodeVar8 = textureSample( nodeUniform20, nodeUniform20_sampler, vec3<f32>( nodeConst17.xy, ( ( nodeConst18 + ( nodeConst19 * 5.0 ) ) / nodeConst20 ) ) );
		nodeVar9 = textureSample( nodeUniform20, nodeUniform20_sampler, vec3<f32>( nodeConst17.xy, ( ( nodeConst18 + ( nodeConst19 * 6.0 ) ) / nodeConst20 ) ) );
		let nodeConst21 = array< vec3<f32>, 9 >( nodeVar3.xyz, vec3<f32>( nodeVar3.w, nodeVar4.xy ), vec3<f32>( nodeVar4.zw, nodeVar5.x ), nodeVar5.yzw, nodeVar6.xyz, vec3<f32>( nodeVar6.w, nodeVar7.xy ), vec3<f32>( nodeVar7.zw, nodeVar8.x ), nodeVar8.yzw, nodeVar9.xyz );
		let nodeConst22 = ( max( ( ( ( ( ( ( ( ( ( nodeConst21[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeConst21[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeConst21[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst21[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeConst21[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeConst21[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeConst21[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeConst21[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst21[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ) * vec3<f32>( render.nodeUniform21 ) );
		lightProbeIrradiance = nodeConst22;
		lightProbePriority = render.nodeUniform19;
		

	}

	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + lightProbeIrradiance );
	nodeVar10 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar10 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst23 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst23 );
	let nodeConst24 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst25 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst23 * nodeConst24 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst25 ) * nodeConst24 ) ) ) * vec3<f32>( nodeConst25 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst26 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar11 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst26 ) );
	nodeVar12 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst26 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar11 );
	indirectDiffuse = ( indirectDiffuse + nodeVar12 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst27 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst27;

	// result

	output.color = nodeConst27;

	return output;

}
