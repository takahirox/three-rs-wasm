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
@binding( 1 ) @group( 2 ) var nodeUniform9_sampler : sampler;
@binding( 2 ) @group( 2 ) var nodeUniform9 : texture_2d<f32>;
@binding( 3 ) @group( 2 ) var nodeUniform24_sampler : sampler;
@binding( 4 ) @group( 2 ) var nodeUniform24 : texture_3d<f32>;
@binding( 5 ) @group( 2 ) var nodeUniform30_sampler : sampler;
@binding( 6 ) @group( 2 ) var nodeUniform30 : texture_3d<f32>;

struct cameraViewMatricesStruct {
	value : array< mat4x4<f32>, 48 >
};
@binding( 0 ) @group( 0 )
var<uniform> cameraViewMatrices : cameraViewMatricesStruct;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform6 : mat3x3<f32>,
	nodeUniform7 : vec3<f32>,
	nodeUniform8 : f32,
	nodeUniform10 : mat4x4<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform28 : vec3<f32>
};
@binding( 0 ) @group( 2 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform13 : vec3<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform17 : vec3<f32>,
	nodeUniform18 : f32,
	nodeUniform19 : f32,
	nodeUniform12 : vec3<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform31 : f32,
	nodeUniform29 : f32,
	nodeUniform25 : f32,
	nodeUniform23 : f32
};
@binding( 1 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
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
var<private> nodeVar0 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
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
var<private> nodeVar10 : vec4<f32>;
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn44 ( lightDistance : f32, cutoffDistance : f32, decayExponent : f32 ) -> f32 {

	var nodeVar0 : f32;


	if ( ( cutoffDistance > 0.0 ) ) {

		let nodeConst0 = ( 1.0 / max( pow( lightDistance, decayExponent ), 0.01 ) );
		let nodeConst1 = ( lightDistance / cutoffDistance );
		let nodeConst2 = clamp( ( 1.0 - ( ( ( nodeConst1 * nodeConst1 ) * nodeConst1 ) * nodeConst1 ) ), 0.0, 1.0 );
		nodeVar0 = ( nodeConst0 * ( nodeConst2 * nodeConst2 ) );

	} else {

		let nodeConst0 = ( 1.0 / max( pow( lightDistance, decayExponent ), 0.01 ) );
		nodeVar0 = nodeConst0;

	}


	return nodeVar0;

}


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
fn main( @location( 0 ) @interpolate(flat, either) v_cameraIndex : u32,
	@location( 1 ) v_positionView : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) v_positionWorld : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform2;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform3, 0.045 ) + max( max( nodeConst0.x, nodeConst0.y ), nodeConst0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	EmissiveColor = ( object.nodeUniform7 * vec3<f32>( object.nodeUniform8 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar0 = textureSample( nodeUniform9, nodeUniform9_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar0;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst1 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst1 );
	let nodeConst2 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst3 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst1 * nodeConst2 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst3 ) * nodeConst2 ) ) ) * vec3<f32>( nodeConst3 ) ) );
	let nodeConst4 = ( ( cameraViewMatrices.value[ v_cameraIndex ] * vec4<f32>( render.nodeUniform12, 1.0 ) ).xyz - v_positionView );
	let nodeConst5 = normalize( nodeConst4 );
	nodeVar1 = ( vec3<f32>( clamp( dot( normalView, nodeConst5 ), 0.0, 1.0 ) ) * ( render.nodeUniform13 * vec3<f32>( fn44( length( nodeConst4 ), render.nodeUniform14, render.nodeUniform15 ) ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst6 = clamp( dot( positionViewDirection, normalize( ( nodeConst5 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst7 = exp2( ( ( ( nodeConst6 * -5.55473 ) - 6.98316 ) * nodeConst6 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar1 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst7 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst7 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst8 = normalize( ( nodeConst5 + positionViewDirection ) );
	let nodeConst9 = clamp( dot( positionViewDirection, nodeConst8 ), 0.0, 1.0 );
	let nodeConst10 = exp2( ( ( ( nodeConst9 * -5.55473 ) - 6.98316 ) * nodeConst9 ) );
	let nodeConst11 = max( Roughness, 0.045 );
	let nodeConst12 = ( nodeConst11 * nodeConst11 );
	directSpecular = ( directSpecular + ( ( nodeVar1 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst10 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst10 ) ) ) * vec3<f32>( fn110( nodeConst12, clamp( dot( normalView, nodeConst5 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst12, clamp( dot( normalView, nodeConst8 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst13 = ( ( cameraViewMatrices.value[ v_cameraIndex ] * vec4<f32>( render.nodeUniform16, 1.0 ) ).xyz - v_positionView );
	let nodeConst14 = normalize( nodeConst13 );
	nodeVar2 = ( vec3<f32>( clamp( dot( normalView, nodeConst14 ), 0.0, 1.0 ) ) * ( render.nodeUniform17 * vec3<f32>( fn44( length( nodeConst13 ), render.nodeUniform18, render.nodeUniform19 ) ) ) );
	let nodeConst15 = clamp( dot( positionViewDirection, normalize( ( nodeConst14 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst16 = exp2( ( ( ( nodeConst15 * -5.55473 ) - 6.98316 ) * nodeConst15 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar2 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst16 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst16 ) ) ) ) ) );
	let nodeConst17 = normalize( ( nodeConst14 + positionViewDirection ) );
	let nodeConst18 = clamp( dot( positionViewDirection, nodeConst17 ), 0.0, 1.0 );
	let nodeConst19 = exp2( ( ( ( nodeConst18 * -5.55473 ) - 6.98316 ) * nodeConst18 ) );
	let nodeConst20 = max( Roughness, 0.045 );
	let nodeConst21 = ( nodeConst20 * nodeConst20 );
	directSpecular = ( directSpecular + ( ( nodeVar2 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst19 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst19 ) ) ) * vec3<f32>( fn110( nodeConst21, clamp( dot( normalView, nodeConst14 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst21, clamp( dot( normalView, nodeConst17 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	lightProbeIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	lightProbePriority = ( - 1000000.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * cameraViewMatrices.value[ v_cameraIndex ] ).xyz );
	let nodeConst22 = ( ( object.nodeUniform20 - object.nodeUniform21 ) / object.nodeUniform22 );
	let nodeConst23 = ( v_positionWorld + ( ( normalWorld * nodeConst22 ) * vec3<f32>( 0.5 ) ) );
	let nodeConst24 = ( nodeConst22 * vec3<f32>( 0.01 ) );

	if ( ( ( all( ( nodeConst23 >= ( object.nodeUniform21 - nodeConst24 ) ) ) && all( ( nodeConst23 <= ( object.nodeUniform20 + nodeConst24 ) ) ) ) && ( render.nodeUniform23 >= lightProbePriority ) ) ) {

		let nodeConst25 = ( object.nodeUniform20 - object.nodeUniform21 );
		let nodeConst26 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform22 );
		let nodeConst27 = clamp( ( ( ( v_positionWorld + ( ( normalWorld * ( nodeConst25 / object.nodeUniform22 ) ) * vec3<f32>( 0.5 ) ) ) - object.nodeUniform21 ) / nodeConst25 ), nodeConst26, ( vec3<f32>( 1.0 ) - nodeConst26 ) );
		let nodeConst28 = ( ( nodeConst27.z * object.nodeUniform22.z ) + 1.0 );
		let nodeConst29 = ( object.nodeUniform22.z + 2.0 );
		let nodeConst30 = ( nodeConst29 * 7.0 );
		nodeVar3 = textureSample( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeConst27.xy, ( ( nodeConst28 + ( nodeConst29 * 0.0 ) ) / nodeConst30 ) ) );
		nodeVar4 = textureSample( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeConst27.xy, ( ( nodeConst28 + ( nodeConst29 * 1.0 ) ) / nodeConst30 ) ) );
		nodeVar5 = textureSample( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeConst27.xy, ( ( nodeConst28 + ( nodeConst29 * 2.0 ) ) / nodeConst30 ) ) );
		nodeVar6 = textureSample( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeConst27.xy, ( ( nodeConst28 + ( nodeConst29 * 3.0 ) ) / nodeConst30 ) ) );
		nodeVar7 = textureSample( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeConst27.xy, ( ( nodeConst28 + ( nodeConst29 * 4.0 ) ) / nodeConst30 ) ) );
		nodeVar8 = textureSample( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeConst27.xy, ( ( nodeConst28 + ( nodeConst29 * 5.0 ) ) / nodeConst30 ) ) );
		nodeVar9 = textureSample( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeConst27.xy, ( ( nodeConst28 + ( nodeConst29 * 6.0 ) ) / nodeConst30 ) ) );
		let nodeConst31 = array< vec3<f32>, 9 >( nodeVar3.xyz, vec3<f32>( nodeVar3.w, nodeVar4.xy ), vec3<f32>( nodeVar4.zw, nodeVar5.x ), nodeVar5.yzw, nodeVar6.xyz, vec3<f32>( nodeVar6.w, nodeVar7.xy ), vec3<f32>( nodeVar7.zw, nodeVar8.x ), nodeVar8.yzw, nodeVar9.xyz );
		let nodeConst32 = ( max( ( ( ( ( ( ( ( ( ( nodeConst31[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeConst31[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeConst31[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst31[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeConst31[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeConst31[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeConst31[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeConst31[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst31[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ) * vec3<f32>( render.nodeUniform25 ) );
		lightProbeIrradiance = nodeConst32;
		lightProbePriority = render.nodeUniform23;
		

	}

	let nodeConst33 = ( ( object.nodeUniform26 - object.nodeUniform27 ) / object.nodeUniform28 );
	let nodeConst34 = ( v_positionWorld + ( ( normalWorld * nodeConst33 ) * vec3<f32>( 0.5 ) ) );
	let nodeConst35 = ( nodeConst33 * vec3<f32>( 0.01 ) );

	if ( ( ( all( ( nodeConst34 >= ( object.nodeUniform27 - nodeConst35 ) ) ) && all( ( nodeConst34 <= ( object.nodeUniform26 + nodeConst35 ) ) ) ) && ( render.nodeUniform29 >= lightProbePriority ) ) ) {

		let nodeConst36 = ( object.nodeUniform26 - object.nodeUniform27 );
		let nodeConst37 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform28 );
		let nodeConst38 = clamp( ( ( ( v_positionWorld + ( ( normalWorld * ( nodeConst36 / object.nodeUniform28 ) ) * vec3<f32>( 0.5 ) ) ) - object.nodeUniform27 ) / nodeConst36 ), nodeConst37, ( vec3<f32>( 1.0 ) - nodeConst37 ) );
		let nodeConst39 = ( ( nodeConst38.z * object.nodeUniform28.z ) + 1.0 );
		let nodeConst40 = ( object.nodeUniform28.z + 2.0 );
		let nodeConst41 = ( nodeConst40 * 7.0 );
		nodeVar10 = textureSample( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeConst38.xy, ( ( nodeConst39 + ( nodeConst40 * 0.0 ) ) / nodeConst41 ) ) );
		nodeVar11 = textureSample( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeConst38.xy, ( ( nodeConst39 + ( nodeConst40 * 1.0 ) ) / nodeConst41 ) ) );
		nodeVar12 = textureSample( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeConst38.xy, ( ( nodeConst39 + ( nodeConst40 * 2.0 ) ) / nodeConst41 ) ) );
		nodeVar13 = textureSample( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeConst38.xy, ( ( nodeConst39 + ( nodeConst40 * 3.0 ) ) / nodeConst41 ) ) );
		nodeVar14 = textureSample( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeConst38.xy, ( ( nodeConst39 + ( nodeConst40 * 4.0 ) ) / nodeConst41 ) ) );
		nodeVar15 = textureSample( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeConst38.xy, ( ( nodeConst39 + ( nodeConst40 * 5.0 ) ) / nodeConst41 ) ) );
		nodeVar16 = textureSample( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeConst38.xy, ( ( nodeConst39 + ( nodeConst40 * 6.0 ) ) / nodeConst41 ) ) );
		let nodeConst42 = array< vec3<f32>, 9 >( nodeVar10.xyz, vec3<f32>( nodeVar10.w, nodeVar11.xy ), vec3<f32>( nodeVar11.zw, nodeVar12.x ), nodeVar12.yzw, nodeVar13.xyz, vec3<f32>( nodeVar13.w, nodeVar14.xy ), vec3<f32>( nodeVar14.zw, nodeVar15.x ), nodeVar15.yzw, nodeVar16.xyz );
		let nodeConst43 = ( max( ( ( ( ( ( ( ( ( ( nodeConst42[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeConst42[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeConst42[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst42[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeConst42[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeConst42[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeConst42[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeConst42[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst42[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ) * vec3<f32>( render.nodeUniform31 ) );
		lightProbeIrradiance = nodeConst43;
		lightProbePriority = render.nodeUniform29;
		

	}

	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + lightProbeIrradiance );
	nodeVar17 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar17 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst44 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst44 );
	let nodeConst45 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst46 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst44 * nodeConst45 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst46 ) * nodeConst45 ) ) ) * vec3<f32>( nodeConst46 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst47 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar18 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst47 ) );
	nodeVar19 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst47 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar18 );
	indirectDiffuse = ( indirectDiffuse + nodeVar19 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst48 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst48;

	// result

	output.color = nodeConst48;

	return output;

}
