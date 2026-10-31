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
@binding( 5 ) @group( 2 ) var nodeUniform20_sampler : sampler_comparison;
@binding( 6 ) @group( 2 ) var nodeUniform20 : texture_depth_2d;
@binding( 7 ) @group( 2 ) var nodeUniform28_sampler : sampler;
@binding( 8 ) @group( 2 ) var nodeUniform28 : texture_3d<f32>;

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
	nodeUniform24 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform26 : vec3<f32>
};
@binding( 0 ) @group( 2 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform16 : vec3<f32>,
	nodeUniform29 : f32,
	nodeUniform27 : f32,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform17 : mat4x4<f32>,
	nodeUniform18 : f32,
	nodeUniform19 : f32,
	nodeUniform21 : f32,
	nodeUniform22 : vec2<f32>,
	nodeUniform23 : f32
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
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar2 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> lightProbeIrradiance : vec3<f32>;
var<private> lightProbePriority : f32;
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
fn fn45 ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn fn46 ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	

	let nodeConst0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeConst0 ), sin( nodeConst0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

}


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
	@location( 4 ) nodeVarying8 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

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
	let nodeConst4 = normalize( ( cameraViewMatrices.value[ v_cameraIndex ] * vec4<f32>( ( render.nodeUniform14 - render.nodeUniform15 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * cameraViewMatrices.value[ v_cameraIndex ] ).xyz );
	let nodeConst5 = ( render.nodeUniform17 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform18 ) ) ), 1.0 ) );
	let nodeConst6 = ( nodeConst5.xyz / vec3<f32>( nodeConst5.w ) );
	let nodeConst7 = vec3<f32>( nodeConst6.x, ( 1.0 - nodeConst6.y ), ( nodeConst6.z + render.nodeUniform19 ) );

	if ( ( ( ( ( ( nodeConst7.x >= 0.0 ) && ( nodeConst7.x <= 1.0 ) ) && ( nodeConst7.y >= 0.0 ) ) && ( nodeConst7.y <= 1.0 ) ) && ( nodeConst7.z <= 1.0 ) ) ) {

		let nodeConst8 = ( fn45( fragCoord.xy ) * 6.28318530718 );
		let nodeConst9 = ( vec2<f32>( render.nodeUniform21 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform22 ) );
		let nodeConst10 = ( nodeConst7.xy + ( fn46( 0, 5, nodeConst8 ) * nodeConst9 ) );
		nodeVar3 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, nodeConst10, nodeConst7.z );
		let nodeConst11 = ( nodeConst7.xy + ( fn46( 1, 5, nodeConst8 ) * nodeConst9 ) );
		nodeVar4 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, nodeConst11, nodeConst7.z );
		let nodeConst12 = ( nodeConst7.xy + ( fn46( 2, 5, nodeConst8 ) * nodeConst9 ) );
		nodeVar5 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, nodeConst12, nodeConst7.z );
		let nodeConst13 = ( nodeConst7.xy + ( fn46( 3, 5, nodeConst8 ) * nodeConst9 ) );
		nodeVar6 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, nodeConst13, nodeConst7.z );
		let nodeConst14 = ( nodeConst7.xy + ( fn46( 4, 5, nodeConst8 ) * nodeConst9 ) );
		nodeVar7 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, nodeConst14, nodeConst7.z );
		nodeVar2 = ( ( ( ( ( nodeVar3 + nodeVar4 ) + nodeVar5 ) + nodeVar6 ) + nodeVar7 ) * 0.2 );

	} else {

		nodeVar2 = 1.0;

	}

	nodeVar8 = mix( 1.0, nodeVar2, render.nodeUniform23 );
	nodeVar9 = ( vec3<f32>( clamp( dot( normalView, nodeConst4 ), 0.0, 1.0 ) ) * ( render.nodeUniform16 * vec3<f32>( nodeVar8 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst15 = clamp( dot( positionViewDirection, normalize( ( nodeConst4 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst16 = exp2( ( ( ( nodeConst15 * -5.55473 ) - 6.98316 ) * nodeConst15 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar9 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst16 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst16 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst17 = normalize( ( nodeConst4 + positionViewDirection ) );
	let nodeConst18 = clamp( dot( positionViewDirection, nodeConst17 ), 0.0, 1.0 );
	let nodeConst19 = exp2( ( ( ( nodeConst18 * -5.55473 ) - 6.98316 ) * nodeConst18 ) );
	let nodeConst20 = max( Roughness, 0.045 );
	let nodeConst21 = ( nodeConst20 * nodeConst20 );
	directSpecular = ( directSpecular + ( ( nodeVar9 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst19 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst19 ) ) ) * vec3<f32>( fn12( nodeConst21, clamp( dot( normalView, nodeConst4 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn14( nodeConst21, clamp( dot( normalView, nodeConst17 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	lightProbeIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	lightProbePriority = ( - 1000000.0 );
	let nodeConst22 = ( ( object.nodeUniform24 - object.nodeUniform25 ) / object.nodeUniform26 );
	let nodeConst23 = ( v_positionWorld + ( ( normalWorld * nodeConst22 ) * vec3<f32>( 0.5 ) ) );
	let nodeConst24 = ( nodeConst22 * vec3<f32>( 0.01 ) );

	if ( ( ( all( ( nodeConst23 >= ( object.nodeUniform25 - nodeConst24 ) ) ) && all( ( nodeConst23 <= ( object.nodeUniform24 + nodeConst24 ) ) ) ) && ( render.nodeUniform27 >= lightProbePriority ) ) ) {

		let nodeConst25 = ( object.nodeUniform24 - object.nodeUniform25 );
		let nodeConst26 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform26 );
		let nodeConst27 = clamp( ( ( ( v_positionWorld + ( ( normalWorld * ( nodeConst25 / object.nodeUniform26 ) ) * vec3<f32>( 0.5 ) ) ) - object.nodeUniform25 ) / nodeConst25 ), nodeConst26, ( vec3<f32>( 1.0 ) - nodeConst26 ) );
		let nodeConst28 = ( ( nodeConst27.z * object.nodeUniform26.z ) + 1.0 );
		let nodeConst29 = ( object.nodeUniform26.z + 2.0 );
		let nodeConst30 = ( nodeConst29 * 7.0 );
		nodeVar10 = textureSample( nodeUniform28, nodeUniform28_sampler, vec3<f32>( nodeConst27.xy, ( ( nodeConst28 + ( nodeConst29 * 0.0 ) ) / nodeConst30 ) ) );
		nodeVar11 = textureSample( nodeUniform28, nodeUniform28_sampler, vec3<f32>( nodeConst27.xy, ( ( nodeConst28 + ( nodeConst29 * 1.0 ) ) / nodeConst30 ) ) );
		nodeVar12 = textureSample( nodeUniform28, nodeUniform28_sampler, vec3<f32>( nodeConst27.xy, ( ( nodeConst28 + ( nodeConst29 * 2.0 ) ) / nodeConst30 ) ) );
		nodeVar13 = textureSample( nodeUniform28, nodeUniform28_sampler, vec3<f32>( nodeConst27.xy, ( ( nodeConst28 + ( nodeConst29 * 3.0 ) ) / nodeConst30 ) ) );
		nodeVar14 = textureSample( nodeUniform28, nodeUniform28_sampler, vec3<f32>( nodeConst27.xy, ( ( nodeConst28 + ( nodeConst29 * 4.0 ) ) / nodeConst30 ) ) );
		nodeVar15 = textureSample( nodeUniform28, nodeUniform28_sampler, vec3<f32>( nodeConst27.xy, ( ( nodeConst28 + ( nodeConst29 * 5.0 ) ) / nodeConst30 ) ) );
		nodeVar16 = textureSample( nodeUniform28, nodeUniform28_sampler, vec3<f32>( nodeConst27.xy, ( ( nodeConst28 + ( nodeConst29 * 6.0 ) ) / nodeConst30 ) ) );
		let nodeConst31 = array< vec3<f32>, 9 >( nodeVar10.xyz, vec3<f32>( nodeVar10.w, nodeVar11.xy ), vec3<f32>( nodeVar11.zw, nodeVar12.x ), nodeVar12.yzw, nodeVar13.xyz, vec3<f32>( nodeVar13.w, nodeVar14.xy ), vec3<f32>( nodeVar14.zw, nodeVar15.x ), nodeVar15.yzw, nodeVar16.xyz );
		let nodeConst32 = ( max( ( ( ( ( ( ( ( ( ( nodeConst31[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeConst31[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeConst31[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst31[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeConst31[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeConst31[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeConst31[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeConst31[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst31[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ) * vec3<f32>( render.nodeUniform29 ) );
		lightProbeIrradiance = nodeConst32;
		lightProbePriority = render.nodeUniform27;
		

	}

	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + lightProbeIrradiance );
	nodeVar17 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar17 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst33 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst33 );
	let nodeConst34 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst35 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst33 * nodeConst34 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst35 ) * nodeConst34 ) ) ) * vec3<f32>( nodeConst35 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst36 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar18 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst36 ) );
	nodeVar19 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst36 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar18 );
	indirectDiffuse = ( indirectDiffuse + nodeVar19 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst37 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst37;

	// result

	output.color = nodeConst37;

	return output;

}
