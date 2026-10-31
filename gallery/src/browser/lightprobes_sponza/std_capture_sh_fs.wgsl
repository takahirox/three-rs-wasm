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
@binding( 9 ) @group( 2 ) var nodeUniform26_sampler : sampler_comparison;
@binding( 10 ) @group( 2 ) var nodeUniform26 : texture_depth_2d;
@binding( 11 ) @group( 2 ) var nodeUniform34_sampler : sampler;
@binding( 12 ) @group( 2 ) var nodeUniform34 : texture_3d<f32>;

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
	nodeUniform30 : vec3<f32>,
	nodeUniform31 : vec3<f32>,
	nodeUniform32 : vec3<f32>
};
@binding( 0 ) @group( 2 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform22 : vec3<f32>,
	nodeUniform35 : f32,
	nodeUniform33 : f32,
	nodeUniform20 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform23 : mat4x4<f32>,
	nodeUniform24 : f32,
	nodeUniform25 : f32,
	nodeUniform27 : f32,
	nodeUniform28 : vec2<f32>,
	nodeUniform29 : f32
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
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> lightProbeIrradiance : vec3<f32>;
var<private> lightProbePriority : f32;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : vec4<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
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
	@location( 2 ) v_tangentView : vec3<f32>,
	@location( 3 ) v_bitangentView : vec3<f32>,
	@location( 4 ) v_positionViewDirection : vec3<f32>,
	@location( 5 ) v_positionWorld : vec3<f32>,
	@location( 6 ) NORMAL_v_bitangentView : vec3<f32>,
	@location( 7 ) nodeVarying11 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

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
	let nodeConst6 = normalize( ( cameraViewMatrices.value[ v_cameraIndex ] * vec4<f32>( ( render.nodeUniform20 - render.nodeUniform21 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * cameraViewMatrices.value[ v_cameraIndex ] ).xyz );
	let nodeConst7 = ( render.nodeUniform23 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform24 ) ) ), 1.0 ) );
	let nodeConst8 = ( nodeConst7.xyz / vec3<f32>( nodeConst7.w ) );
	let nodeConst9 = vec3<f32>( nodeConst8.x, ( 1.0 - nodeConst8.y ), ( nodeConst8.z + render.nodeUniform25 ) );

	if ( ( ( ( ( ( nodeConst9.x >= 0.0 ) && ( nodeConst9.x <= 1.0 ) ) && ( nodeConst9.y >= 0.0 ) ) && ( nodeConst9.y <= 1.0 ) ) && ( nodeConst9.z <= 1.0 ) ) ) {

		let nodeConst10 = ( fn45( fragCoord.xy ) * 6.28318530718 );
		let nodeConst11 = ( vec2<f32>( render.nodeUniform27 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform28 ) );
		let nodeConst12 = ( nodeConst9.xy + ( fn46( 0, 5, nodeConst10 ) * nodeConst11 ) );
		nodeVar6 = textureSampleCompare( nodeUniform26, nodeUniform26_sampler, nodeConst12, nodeConst9.z );
		let nodeConst13 = ( nodeConst9.xy + ( fn46( 1, 5, nodeConst10 ) * nodeConst11 ) );
		nodeVar7 = textureSampleCompare( nodeUniform26, nodeUniform26_sampler, nodeConst13, nodeConst9.z );
		let nodeConst14 = ( nodeConst9.xy + ( fn46( 2, 5, nodeConst10 ) * nodeConst11 ) );
		nodeVar8 = textureSampleCompare( nodeUniform26, nodeUniform26_sampler, nodeConst14, nodeConst9.z );
		let nodeConst15 = ( nodeConst9.xy + ( fn46( 3, 5, nodeConst10 ) * nodeConst11 ) );
		nodeVar9 = textureSampleCompare( nodeUniform26, nodeUniform26_sampler, nodeConst15, nodeConst9.z );
		let nodeConst16 = ( nodeConst9.xy + ( fn46( 4, 5, nodeConst10 ) * nodeConst11 ) );
		nodeVar10 = textureSampleCompare( nodeUniform26, nodeUniform26_sampler, nodeConst16, nodeConst9.z );
		nodeVar5 = ( ( ( ( ( nodeVar6 + nodeVar7 ) + nodeVar8 ) + nodeVar9 ) + nodeVar10 ) * 0.2 );

	} else {

		nodeVar5 = 1.0;

	}

	nodeVar11 = mix( 1.0, nodeVar5, render.nodeUniform29 );
	nodeVar12 = ( vec3<f32>( clamp( dot( normalView, nodeConst6 ), 0.0, 1.0 ) ) * ( render.nodeUniform22 * vec3<f32>( nodeVar11 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst17 = clamp( dot( positionViewDirection, normalize( ( nodeConst6 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst18 = exp2( ( ( ( nodeConst17 * -5.55473 ) - 6.98316 ) * nodeConst17 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar12 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst18 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst18 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst19 = normalize( ( nodeConst6 + positionViewDirection ) );
	let nodeConst20 = clamp( dot( positionViewDirection, nodeConst19 ), 0.0, 1.0 );
	let nodeConst21 = exp2( ( ( ( nodeConst20 * -5.55473 ) - 6.98316 ) * nodeConst20 ) );
	let nodeConst22 = max( Roughness, 0.045 );
	let nodeConst23 = ( nodeConst22 * nodeConst22 );
	directSpecular = ( directSpecular + ( ( nodeVar12 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst21 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst21 ) ) ) * vec3<f32>( fn12( nodeConst23, clamp( dot( normalView, nodeConst6 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn14( nodeConst23, clamp( dot( normalView, nodeConst19 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	lightProbeIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	lightProbePriority = ( - 1000000.0 );
	let nodeConst24 = ( ( object.nodeUniform30 - object.nodeUniform31 ) / object.nodeUniform32 );
	let nodeConst25 = ( v_positionWorld + ( ( normalWorld * nodeConst24 ) * vec3<f32>( 0.5 ) ) );
	let nodeConst26 = ( nodeConst24 * vec3<f32>( 0.01 ) );

	if ( ( ( all( ( nodeConst25 >= ( object.nodeUniform31 - nodeConst26 ) ) ) && all( ( nodeConst25 <= ( object.nodeUniform30 + nodeConst26 ) ) ) ) && ( render.nodeUniform33 >= lightProbePriority ) ) ) {

		let nodeConst27 = ( object.nodeUniform30 - object.nodeUniform31 );
		let nodeConst28 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform32 );
		let nodeConst29 = clamp( ( ( ( v_positionWorld + ( ( normalWorld * ( nodeConst27 / object.nodeUniform32 ) ) * vec3<f32>( 0.5 ) ) ) - object.nodeUniform31 ) / nodeConst27 ), nodeConst28, ( vec3<f32>( 1.0 ) - nodeConst28 ) );
		let nodeConst30 = ( ( nodeConst29.z * object.nodeUniform32.z ) + 1.0 );
		let nodeConst31 = ( object.nodeUniform32.z + 2.0 );
		let nodeConst32 = ( nodeConst31 * 7.0 );
		nodeVar13 = textureSample( nodeUniform34, nodeUniform34_sampler, vec3<f32>( nodeConst29.xy, ( ( nodeConst30 + ( nodeConst31 * 0.0 ) ) / nodeConst32 ) ) );
		nodeVar14 = textureSample( nodeUniform34, nodeUniform34_sampler, vec3<f32>( nodeConst29.xy, ( ( nodeConst30 + ( nodeConst31 * 1.0 ) ) / nodeConst32 ) ) );
		nodeVar15 = textureSample( nodeUniform34, nodeUniform34_sampler, vec3<f32>( nodeConst29.xy, ( ( nodeConst30 + ( nodeConst31 * 2.0 ) ) / nodeConst32 ) ) );
		nodeVar16 = textureSample( nodeUniform34, nodeUniform34_sampler, vec3<f32>( nodeConst29.xy, ( ( nodeConst30 + ( nodeConst31 * 3.0 ) ) / nodeConst32 ) ) );
		nodeVar17 = textureSample( nodeUniform34, nodeUniform34_sampler, vec3<f32>( nodeConst29.xy, ( ( nodeConst30 + ( nodeConst31 * 4.0 ) ) / nodeConst32 ) ) );
		nodeVar18 = textureSample( nodeUniform34, nodeUniform34_sampler, vec3<f32>( nodeConst29.xy, ( ( nodeConst30 + ( nodeConst31 * 5.0 ) ) / nodeConst32 ) ) );
		nodeVar19 = textureSample( nodeUniform34, nodeUniform34_sampler, vec3<f32>( nodeConst29.xy, ( ( nodeConst30 + ( nodeConst31 * 6.0 ) ) / nodeConst32 ) ) );
		let nodeConst33 = array< vec3<f32>, 9 >( nodeVar13.xyz, vec3<f32>( nodeVar13.w, nodeVar14.xy ), vec3<f32>( nodeVar14.zw, nodeVar15.x ), nodeVar15.yzw, nodeVar16.xyz, vec3<f32>( nodeVar16.w, nodeVar17.xy ), vec3<f32>( nodeVar17.zw, nodeVar18.x ), nodeVar18.yzw, nodeVar19.xyz );
		let nodeConst34 = ( max( ( ( ( ( ( ( ( ( ( nodeConst33[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeConst33[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeConst33[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst33[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeConst33[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeConst33[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeConst33[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeConst33[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst33[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ) * vec3<f32>( render.nodeUniform35 ) );
		lightProbeIrradiance = nodeConst34;
		lightProbePriority = render.nodeUniform33;
		

	}

	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + lightProbeIrradiance );
	nodeVar20 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar20 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst35 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst35 );
	let nodeConst36 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst37 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst35 * nodeConst36 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst37 ) * nodeConst36 ) ) ) * vec3<f32>( nodeConst37 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst38 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar21 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst38 ) );
	nodeVar22 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst38 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar21 );
	indirectDiffuse = ( indirectDiffuse + nodeVar22 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst39 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst39;

	// result

	output.color = nodeConst39;

	return output;

}
