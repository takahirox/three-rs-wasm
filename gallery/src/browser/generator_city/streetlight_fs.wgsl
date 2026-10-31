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
@binding( 1 ) @group( 1 ) var nodeUniform7_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform7 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform17_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform17 : texture_3d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform19_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform19 : texture_cube<f32>;

struct objectStruct {
	nodeUniform4 : f32,
	nodeUniform6 : mat3x3<f32>,
	nodeUniform8 : mat4x4<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform20 : mat4x4<f32>,
	nodeUniform22 : f32,
	nodeUniform23 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform18 : f32,
	nodeUniform16 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> Metalness : f32;
var<private> nodeVar1 : f32;
var<private> Roughness : f32;
var<private> nodeVar2 : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
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
var<private> radiance : vec3<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn110 ( alpha : f32, dotNL : f32, dotNV : f32 ) -> f32 {

	

	let nodeConst0 = ( alpha * alpha );

	return ( 0.5 / max( ( ( dotNL * sqrt( ( nodeConst0 + ( ( 1.0 - nodeConst0 ) * ( dotNV * dotNV ) ) ) ) ) + ( dotNV * sqrt( ( nodeConst0 + ( ( 1.0 - nodeConst0 ) * ( dotNL * dotNL ) ) ) ) ) ), 0.000001 ) );

}


fn fn112 ( alpha : f32, dotNH : f32 ) -> f32 {

	

	let nodeConst0 = ( alpha * alpha );
	let nodeConst1 = ( 1.0 - ( ( dotNH * dotNH ) * ( 1.0 - nodeConst0 ) ) );

	return ( ( nodeConst0 / ( nodeConst1 * nodeConst1 ) ) * 0.3183098861837907 );

}




@fragment
fn main( @location( 0 ) @interpolate( flat, either ) nodeVarying3 : f32,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	let nodeConst1 = ( nodeVarying3 == 1.0 );

	if ( nodeConst1 ) {

		nodeVar0 = vec3<f32>( 1.0, 0.8713671191959567, 0.6038273388475408 );

	} else {

		nodeVar0 = vec3<f32>( 0.05126945836711539, 0.05612849004241121, 0.04666508633021928 );

	}

	DiffuseColor = vec4<f32>( nodeVar0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform4 );
	DiffuseColor.w = 1.0;

	if ( nodeConst1 ) {

		nodeVar1 = 0.0;

	} else {

		nodeVar1 = 0.7;

	}

	let nodeConst2 = nodeVar1;
	Metalness = nodeConst2;

	if ( nodeConst1 ) {

		nodeVar2 = 0.3;

	} else {

		nodeVar2 = 0.5;

	}

	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst3 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( nodeVar2, 0.045 ) + max( max( nodeConst3.x, nodeConst3.y ), nodeConst3.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - nodeConst2 ) ) );

	if ( nodeConst1 ) {

		nodeVar3 = ( vec3<f32>( 1.0, 0.76052450467022, 0.3813260114221238 ) * vec3<f32>( 60.0 ) );

	} else {

		nodeVar3 = vec3<f32>( 0.0, 0.0, 0.0 );

	}

	EmissiveColor = nodeVar3;
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar4 = textureSample( nodeUniform7, nodeUniform7_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar4;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst4 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst4 );
	let nodeConst5 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst6 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst4 * nodeConst5 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst6 ) * nodeConst5 ) ) ) * vec3<f32>( nodeConst6 ) ) );
	let nodeConst7 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform10 - render.nodeUniform11 ), 0.0 ) ).xyz );
	nodeVar5 = ( vec3<f32>( clamp( dot( normalView, nodeConst7 ), 0.0, 1.0 ) ) * render.nodeUniform12 );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst8 = clamp( dot( positionViewDirection, normalize( ( nodeConst7 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst9 = exp2( ( ( ( nodeConst8 * -5.55473 ) - 6.98316 ) * nodeConst8 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar5 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst9 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst9 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst10 = normalize( ( nodeConst7 + positionViewDirection ) );
	let nodeConst11 = clamp( dot( positionViewDirection, nodeConst10 ), 0.0, 1.0 );
	let nodeConst12 = exp2( ( ( ( nodeConst11 * -5.55473 ) - 6.98316 ) * nodeConst11 ) );
	let nodeConst13 = max( Roughness, 0.045 );
	let nodeConst14 = ( nodeConst13 * nodeConst13 );
	directSpecular = ( directSpecular + ( ( nodeVar5 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst12 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst12 ) ) ) * vec3<f32>( fn110( nodeConst14, clamp( dot( normalView, nodeConst7 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst14, clamp( dot( normalView, nodeConst10 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	lightProbeIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	lightProbePriority = ( - 1000000.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst15 = ( ( object.nodeUniform13 - object.nodeUniform14 ) / object.nodeUniform15 );
	let nodeConst16 = ( v_positionWorld + ( ( normalWorld * nodeConst15 ) * vec3<f32>( 0.5 ) ) );
	let nodeConst17 = ( nodeConst15 * vec3<f32>( 0.01 ) );

	if ( ( ( all( ( nodeConst16 >= ( object.nodeUniform14 - nodeConst17 ) ) ) && all( ( nodeConst16 <= ( object.nodeUniform13 + nodeConst17 ) ) ) ) && ( render.nodeUniform16 >= lightProbePriority ) ) ) {

		let nodeConst18 = ( object.nodeUniform13 - object.nodeUniform14 );
		let nodeConst19 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform15 );
		let nodeConst20 = clamp( ( ( ( v_positionWorld + ( ( normalWorld * ( nodeConst18 / object.nodeUniform15 ) ) * vec3<f32>( 0.5 ) ) ) - object.nodeUniform14 ) / nodeConst18 ), nodeConst19, ( vec3<f32>( 1.0 ) - nodeConst19 ) );
		let nodeConst21 = ( ( nodeConst20.z * object.nodeUniform15.z ) + 1.0 );
		let nodeConst22 = ( object.nodeUniform15.z + 2.0 );
		let nodeConst23 = ( nodeConst22 * 7.0 );
		nodeVar6 = textureSample( nodeUniform17, nodeUniform17_sampler, vec3<f32>( nodeConst20.xy, ( ( nodeConst21 + ( nodeConst22 * 0.0 ) ) / nodeConst23 ) ) );
		nodeVar7 = textureSample( nodeUniform17, nodeUniform17_sampler, vec3<f32>( nodeConst20.xy, ( ( nodeConst21 + ( nodeConst22 * 1.0 ) ) / nodeConst23 ) ) );
		nodeVar8 = textureSample( nodeUniform17, nodeUniform17_sampler, vec3<f32>( nodeConst20.xy, ( ( nodeConst21 + ( nodeConst22 * 2.0 ) ) / nodeConst23 ) ) );
		nodeVar9 = textureSample( nodeUniform17, nodeUniform17_sampler, vec3<f32>( nodeConst20.xy, ( ( nodeConst21 + ( nodeConst22 * 3.0 ) ) / nodeConst23 ) ) );
		nodeVar10 = textureSample( nodeUniform17, nodeUniform17_sampler, vec3<f32>( nodeConst20.xy, ( ( nodeConst21 + ( nodeConst22 * 4.0 ) ) / nodeConst23 ) ) );
		nodeVar11 = textureSample( nodeUniform17, nodeUniform17_sampler, vec3<f32>( nodeConst20.xy, ( ( nodeConst21 + ( nodeConst22 * 5.0 ) ) / nodeConst23 ) ) );
		nodeVar12 = textureSample( nodeUniform17, nodeUniform17_sampler, vec3<f32>( nodeConst20.xy, ( ( nodeConst21 + ( nodeConst22 * 6.0 ) ) / nodeConst23 ) ) );
		let nodeConst24 = array< vec3<f32>, 9 >( nodeVar6.xyz, vec3<f32>( nodeVar6.w, nodeVar7.xy ), vec3<f32>( nodeVar7.zw, nodeVar8.x ), nodeVar8.yzw, nodeVar9.xyz, vec3<f32>( nodeVar9.w, nodeVar10.xy ), vec3<f32>( nodeVar10.zw, nodeVar11.x ), nodeVar11.yzw, nodeVar12.xyz );
		let nodeConst25 = ( max( ( ( ( ( ( ( ( ( ( nodeConst24[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeConst24[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeConst24[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst24[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeConst24[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeConst24[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeConst24[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeConst24[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst24[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ) * vec3<f32>( render.nodeUniform18 ) );
		lightProbeIrradiance = nodeConst25;
		lightProbePriority = render.nodeUniform16;
		

	}

	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst26 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst27 = ( object.nodeUniform20 * vec4<f32>( nodeConst26, 1.0 ) );
	let nodeConst28 = clamp( Roughness, 0.0, 1.0 );
	nodeVar13 = textureSampleLevel( nodeUniform19, nodeUniform19_sampler, vec3<f32>( ( - nodeConst27.x ), nodeConst27.yz ), ( ( object.nodeUniform22 * nodeConst28 ) * ( 2.0 - nodeConst28 ) ) );
	radiance = ( radiance + ( nodeVar13.xyz * vec3<f32>( object.nodeUniform23 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst29 = ( object.nodeUniform20 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst30 = clamp( 1.0, 0.0, 1.0 );
	nodeVar14 = textureSampleLevel( nodeUniform19, nodeUniform19_sampler, vec3<f32>( ( - nodeConst29.x ), nodeConst29.yz ), ( ( object.nodeUniform22 * nodeConst30 ) * ( 2.0 - nodeConst30 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar14.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform23 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + lightProbeIrradiance );
	nodeVar15 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar15 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst31 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst31 );
	let nodeConst32 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst33 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst31 * nodeConst32 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst33 ) * nodeConst32 ) ) ) * vec3<f32>( nodeConst33 ) ) );
	let nodeConst34 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar16 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst34 ) );
	nodeVar17 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst34 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar16 );
	indirectDiffuse = ( indirectDiffuse + nodeVar17 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst35 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst35;

	// result

	output.color = nodeConst35;

	return output;

}
