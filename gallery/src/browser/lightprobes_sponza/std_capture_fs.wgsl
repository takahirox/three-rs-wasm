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
@binding( 3 ) @group( 2 ) var nodeUniform5_sampler : sampler;
@binding( 4 ) @group( 2 ) var nodeUniform5 : texture_2d<f32>;
@binding( 5 ) @group( 2 ) var nodeUniform14_sampler : sampler;
@binding( 6 ) @group( 2 ) var nodeUniform14 : texture_2d<f32>;
@binding( 7 ) @group( 2 ) var nodeUniform16_sampler : sampler;
@binding( 8 ) @group( 2 ) var nodeUniform16 : texture_2d<f32>;
@binding( 9 ) @group( 2 ) var nodeUniform26_sampler : sampler;
@binding( 10 ) @group( 2 ) var nodeUniform26 : texture_3d<f32>;

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
	nodeUniform6 : mat3x3<f32>,
	nodeUniform7 : f32,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform11 : mat3x3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform13 : f32,
	nodeUniform15 : mat4x4<f32>,
	nodeUniform17 : mat3x3<f32>,
	nodeUniform18 : vec2<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform24 : vec3<f32>
};
@binding( 0 ) @group( 2 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform27 : f32,
	nodeUniform25 : f32,
	nodeUniform21 : vec3<f32>,
	nodeUniform20 : vec3<f32>
};
@binding( 1 ) @group( 0 )
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
var<private> lightProbeIrradiance : vec3<f32>;
var<private> lightProbePriority : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
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
	@location( 2 ) v_tangentView : vec3<f32>,
	@location( 3 ) v_bitangentView : vec3<f32>,
	@location( 4 ) v_positionViewDirection : vec3<f32>,
	@location( 5 ) v_positionWorld : vec3<f32>,
	@location( 6 ) NORMAL_v_bitangentView : vec3<f32>,
	@location( 7 ) nodeVarying11 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying11, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	nodeVar1 = textureSample( nodeUniform5, nodeUniform5_sampler, ( object.nodeUniform6 * vec3<f32>( nodeVarying11, 1.0 ) ).xy );
	Metalness = ( object.nodeUniform4 * nodeVar1.z );
	nodeVar2 = textureSample( nodeUniform5, nodeUniform5_sampler, ( object.nodeUniform8 * vec3<f32>( nodeVarying11, 1.0 ) ).xy );
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst1 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( object.nodeUniform7 * nodeVar2.y ), 0.045 ) + max( max( nodeConst1.x, nodeConst1.y ), nodeConst1.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - ( object.nodeUniform4 * nodeVar1.z ) ) ) );
	EmissiveColor = ( object.nodeUniform12 * vec3<f32>( object.nodeUniform13 ) );
	NORMAL_tangentView = normalize( v_tangentView );
	NORMAL_bitangentView = normalize( NORMAL_v_bitangentView );
	NORMAL_normalView = normalViewGeometry;
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar3 = textureSample( nodeUniform16, nodeUniform16_sampler, ( object.nodeUniform17 * vec3<f32>( nodeVarying11, 1.0 ) ).xy );
	let nodeConst2 = ( ( nodeVar3 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeConst2.xy * object.nodeUniform18 ), nodeConst2.z ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar4 = textureSample( nodeUniform14, nodeUniform14_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar4;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst3 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst3 );
	let nodeConst4 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst5 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst3 * nodeConst4 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst5 ) * nodeConst4 ) ) ) * vec3<f32>( nodeConst5 ) ) );
	let nodeConst6 = normalize( ( cameraViewMatrices.value[ v_cameraIndex ] * vec4<f32>( render.nodeUniform20, 0.0 ) ).xyz );
	nodeVar5 = ( vec3<f32>( clamp( dot( normalView, nodeConst6 ), 0.0, 1.0 ) ) * render.nodeUniform21 );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst7 = clamp( dot( positionViewDirection, normalize( ( nodeConst6 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst8 = exp2( ( ( ( nodeConst7 * -5.55473 ) - 6.98316 ) * nodeConst7 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar5 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst8 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst8 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst9 = normalize( ( nodeConst6 + positionViewDirection ) );
	let nodeConst10 = clamp( dot( positionViewDirection, nodeConst9 ), 0.0, 1.0 );
	let nodeConst11 = exp2( ( ( ( nodeConst10 * -5.55473 ) - 6.98316 ) * nodeConst10 ) );
	let nodeConst12 = max( Roughness, 0.045 );
	let nodeConst13 = ( nodeConst12 * nodeConst12 );
	directSpecular = ( directSpecular + ( ( nodeVar5 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst11 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst11 ) ) ) * vec3<f32>( fn12( nodeConst13, clamp( dot( normalView, nodeConst6 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn14( nodeConst13, clamp( dot( normalView, nodeConst9 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	lightProbeIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	lightProbePriority = ( - 1000000.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * cameraViewMatrices.value[ v_cameraIndex ] ).xyz );
	let nodeConst14 = ( ( object.nodeUniform22 - object.nodeUniform23 ) / object.nodeUniform24 );
	let nodeConst15 = ( v_positionWorld + ( ( normalWorld * nodeConst14 ) * vec3<f32>( 0.5 ) ) );
	let nodeConst16 = ( nodeConst14 * vec3<f32>( 0.01 ) );

	if ( ( ( all( ( nodeConst15 >= ( object.nodeUniform23 - nodeConst16 ) ) ) && all( ( nodeConst15 <= ( object.nodeUniform22 + nodeConst16 ) ) ) ) && ( render.nodeUniform25 >= lightProbePriority ) ) ) {

		let nodeConst17 = ( object.nodeUniform22 - object.nodeUniform23 );
		let nodeConst18 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform24 );
		let nodeConst19 = clamp( ( ( ( v_positionWorld + ( ( normalWorld * ( nodeConst17 / object.nodeUniform24 ) ) * vec3<f32>( 0.5 ) ) ) - object.nodeUniform23 ) / nodeConst17 ), nodeConst18, ( vec3<f32>( 1.0 ) - nodeConst18 ) );
		let nodeConst20 = ( ( nodeConst19.z * object.nodeUniform24.z ) + 1.0 );
		let nodeConst21 = ( object.nodeUniform24.z + 2.0 );
		let nodeConst22 = ( nodeConst21 * 7.0 );
		nodeVar6 = textureSample( nodeUniform26, nodeUniform26_sampler, vec3<f32>( nodeConst19.xy, ( ( nodeConst20 + ( nodeConst21 * 0.0 ) ) / nodeConst22 ) ) );
		nodeVar7 = textureSample( nodeUniform26, nodeUniform26_sampler, vec3<f32>( nodeConst19.xy, ( ( nodeConst20 + ( nodeConst21 * 1.0 ) ) / nodeConst22 ) ) );
		nodeVar8 = textureSample( nodeUniform26, nodeUniform26_sampler, vec3<f32>( nodeConst19.xy, ( ( nodeConst20 + ( nodeConst21 * 2.0 ) ) / nodeConst22 ) ) );
		nodeVar9 = textureSample( nodeUniform26, nodeUniform26_sampler, vec3<f32>( nodeConst19.xy, ( ( nodeConst20 + ( nodeConst21 * 3.0 ) ) / nodeConst22 ) ) );
		nodeVar10 = textureSample( nodeUniform26, nodeUniform26_sampler, vec3<f32>( nodeConst19.xy, ( ( nodeConst20 + ( nodeConst21 * 4.0 ) ) / nodeConst22 ) ) );
		nodeVar11 = textureSample( nodeUniform26, nodeUniform26_sampler, vec3<f32>( nodeConst19.xy, ( ( nodeConst20 + ( nodeConst21 * 5.0 ) ) / nodeConst22 ) ) );
		nodeVar12 = textureSample( nodeUniform26, nodeUniform26_sampler, vec3<f32>( nodeConst19.xy, ( ( nodeConst20 + ( nodeConst21 * 6.0 ) ) / nodeConst22 ) ) );
		let nodeConst23 = array< vec3<f32>, 9 >( nodeVar6.xyz, vec3<f32>( nodeVar6.w, nodeVar7.xy ), vec3<f32>( nodeVar7.zw, nodeVar8.x ), nodeVar8.yzw, nodeVar9.xyz, vec3<f32>( nodeVar9.w, nodeVar10.xy ), vec3<f32>( nodeVar10.zw, nodeVar11.x ), nodeVar11.yzw, nodeVar12.xyz );
		let nodeConst24 = ( max( ( ( ( ( ( ( ( ( ( nodeConst23[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeConst23[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeConst23[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst23[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeConst23[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeConst23[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeConst23[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeConst23[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst23[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ) * vec3<f32>( render.nodeUniform27 ) );
		lightProbeIrradiance = nodeConst24;
		lightProbePriority = render.nodeUniform25;
		

	}

	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + lightProbeIrradiance );
	nodeVar13 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar13 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst25 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst25 );
	let nodeConst26 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst27 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst25 * nodeConst26 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst27 ) * nodeConst26 ) ) ) * vec3<f32>( nodeConst27 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst28 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar14 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst28 ) );
	nodeVar15 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst28 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar14 );
	indirectDiffuse = ( indirectDiffuse + nodeVar15 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst29 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst29;

	// result

	output.color = nodeConst29;

	return output;

}
