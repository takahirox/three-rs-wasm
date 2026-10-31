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
@binding( 1 ) @group( 1 ) var nodeUniform4_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform4 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform8 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform16_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform16 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform21_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform21 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform23_sampler : sampler;
@binding( 10 ) @group( 1 ) var nodeUniform23 : texture_2d<f32>;
@binding( 11 ) @group( 1 ) var nodeUniform33_sampler : sampler_comparison;
@binding( 12 ) @group( 1 ) var nodeUniform33 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : vec3<f32>,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform9 : mat3x3<f32>,
	nodeUniform10 : f32,
	nodeUniform11 : mat3x3<f32>,
	nodeUniform13 : mat3x3<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : vec3<f32>,
	nodeUniform17 : mat3x3<f32>,
	nodeUniform18 : f32,
	nodeUniform19 : vec3<f32>,
	nodeUniform20 : f32,
	nodeUniform22 : mat4x4<f32>,
	nodeUniform24 : mat3x3<f32>,
	nodeUniform25 : vec2<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform29 : vec3<f32>,
	nodeUniform38 : vec3<f32>,
	nodeUniform39 : vec3<f32>,
	nodeUniform37 : vec3<f32>,
	nodeUniform41 : vec3<f32>,
	nodeUniform42 : vec3<f32>,
	nodeUniform40 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform43 : vec3<f32>,
	nodeUniform44 : f32,
	nodeUniform45 : f32,
	nodeUniform30 : mat4x4<f32>,
	nodeUniform31 : f32,
	nodeUniform32 : f32,
	nodeUniform34 : f32,
	nodeUniform35 : vec2<f32>,
	nodeUniform36 : f32
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
var<private> IOR : f32;
var<private> SpecularColor : vec3<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> nodeVar4 : f32;
var<private> tangentViewFrame : vec3<f32>;
var<private> NORMAL_tangentView : vec3<f32>;
var<private> bitangentViewFrame : vec3<f32>;
var<private> NORMAL_bitangentView : vec3<f32>;
var<private> NORMAL_TBNViewMatrix : mat3x3<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar6 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar7 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar18 : vec4<f32>;

// codes
fn fn24 ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn fn25 ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	

	let nodeConst0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeConst0 ), sin( nodeConst0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

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
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32>,
	@location( 4 ) nodeVarying7 : vec2<f32>,
	@builtin( front_facing ) isFront : bool,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform4, nodeUniform4_sampler, ( object.nodeUniform5 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform3, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform6 );
	DiffuseColor.w = 1.0;
	nodeVar1 = textureSample( nodeUniform8, nodeUniform8_sampler, ( object.nodeUniform9 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	Metalness = ( object.nodeUniform7 * nodeVar1.z );
	nodeVar2 = textureSample( nodeUniform8, nodeUniform8_sampler, ( object.nodeUniform11 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst1 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( object.nodeUniform10 * nodeVar2.y ), 0.045 ) + max( max( nodeConst1.x, nodeConst1.y ), nodeConst1.z ) ), 1.0 );
	IOR = object.nodeUniform14;
	let nodeConst2 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	nodeVar3 = textureSample( nodeUniform16, nodeUniform16_sampler, ( object.nodeUniform17 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	SpecularColor = ( min( ( vec3<f32>( ( nodeConst2 * nodeConst2 ) ) * ( object.nodeUniform15 * nodeVar3.xyz ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform18 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform18, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - ( object.nodeUniform7 * nodeVar1.z ) ) ) );
	EmissiveColor = ( object.nodeUniform19 * vec3<f32>( object.nodeUniform20 ) );
	let nodeConst3 = ( ( f32( isFront ) * 2.0 ) - 1.0 );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( nodeConst3 ) );
	let nodeConst4 = cross( - dpdy( v_positionView ), NORMAL_normalView );
	let nodeConst5 = dpdx( nodeVarying7 );
	let nodeConst6 = cross( NORMAL_normalView, dpdx( v_positionView ) );
	let nodeConst7 = - dpdy( nodeVarying7 );
	let nodeConst8 = ( ( nodeConst4 * vec3<f32>( nodeConst5.x ) ) + ( nodeConst6 * vec3<f32>( nodeConst7.x ) ) );
	let nodeConst9 = ( ( nodeConst4 * vec3<f32>( nodeConst5.y ) ) + ( nodeConst6 * vec3<f32>( nodeConst7.y ) ) );
	let nodeConst10 = max( dot( nodeConst8, nodeConst8 ), dot( nodeConst9, nodeConst9 ) );

	if ( ( nodeConst10 == 0.0 ) ) {

		nodeVar4 = 0.0;

	} else {

		nodeVar4 = inverseSqrt( nodeConst10 );

	}

	tangentViewFrame = ( nodeConst8 * vec3<f32>( nodeVar4 ) );
	NORMAL_tangentView = ( tangentViewFrame * vec3<f32>( nodeConst3 ) );
	bitangentViewFrame = ( nodeConst9 * vec3<f32>( nodeVar4 ) );
	NORMAL_bitangentView = ( bitangentViewFrame * vec3<f32>( nodeConst3 ) );
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar5 = textureSample( nodeUniform23, nodeUniform23_sampler, ( object.nodeUniform24 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	let nodeConst11 = ( ( nodeVar5 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeConst11.xy * object.nodeUniform25 ), nodeConst11.z ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar6 = textureSample( nodeUniform21, nodeUniform21_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar6;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst12 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst12 );
	let nodeConst13 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst14 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst12 * nodeConst13 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst14 ) * nodeConst13 ) ) ) * vec3<f32>( nodeConst14 ) ) );
	let nodeConst15 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform27 - render.nodeUniform28 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst16 = ( render.nodeUniform30 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform31 ) ) ), 1.0 ) );
	let nodeConst17 = ( nodeConst16.xyz / vec3<f32>( nodeConst16.w ) );
	let nodeConst18 = vec3<f32>( nodeConst17.x, ( 1.0 - nodeConst17.y ), ( nodeConst17.z + render.nodeUniform32 ) );

	if ( ( ( ( ( ( nodeConst18.x >= 0.0 ) && ( nodeConst18.x <= 1.0 ) ) && ( nodeConst18.y >= 0.0 ) ) && ( nodeConst18.y <= 1.0 ) ) && ( nodeConst18.z <= 1.0 ) ) ) {

		let nodeConst19 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst20 = ( vec2<f32>( render.nodeUniform34 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform35 ) );
		let nodeConst21 = ( nodeConst18.xy + ( fn25( 0, 5, nodeConst19 ) * nodeConst20 ) );
		nodeVar8 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst21, nodeConst18.z );
		let nodeConst22 = ( nodeConst18.xy + ( fn25( 1, 5, nodeConst19 ) * nodeConst20 ) );
		nodeVar9 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst22, nodeConst18.z );
		let nodeConst23 = ( nodeConst18.xy + ( fn25( 2, 5, nodeConst19 ) * nodeConst20 ) );
		nodeVar10 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst23, nodeConst18.z );
		let nodeConst24 = ( nodeConst18.xy + ( fn25( 3, 5, nodeConst19 ) * nodeConst20 ) );
		nodeVar11 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst24, nodeConst18.z );
		let nodeConst25 = ( nodeConst18.xy + ( fn25( 4, 5, nodeConst19 ) * nodeConst20 ) );
		nodeVar12 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst25, nodeConst18.z );
		nodeVar7 = ( ( ( ( ( nodeVar8 + nodeVar9 ) + nodeVar10 ) + nodeVar11 ) + nodeVar12 ) * 0.2 );

	} else {

		nodeVar7 = 1.0;

	}

	nodeVar13 = mix( 1.0, nodeVar7, render.nodeUniform36 );
	nodeVar14 = ( vec3<f32>( clamp( dot( normalView, nodeConst15 ), 0.0, 1.0 ) ) * ( render.nodeUniform29 * vec3<f32>( nodeVar13 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst26 = clamp( dot( positionViewDirection, normalize( ( nodeConst15 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst27 = exp2( ( ( ( nodeConst26 * -5.55473 ) - 6.98316 ) * nodeConst26 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar14 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst27 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst27 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst28 = normalize( ( nodeConst15 + positionViewDirection ) );
	let nodeConst29 = clamp( dot( positionViewDirection, nodeConst28 ), 0.0, 1.0 );
	let nodeConst30 = exp2( ( ( ( nodeConst29 * -5.55473 ) - 6.98316 ) * nodeConst29 ) );
	let nodeConst31 = max( Roughness, 0.045 );
	let nodeConst32 = ( nodeConst31 * nodeConst31 );
	directSpecular = ( directSpecular + ( ( nodeVar14 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst30 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst30 ) ) ) * vec3<f32>( fn110( nodeConst32, clamp( dot( normalView, nodeConst15 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst32, clamp( dot( normalView, nodeConst28 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + mix( render.nodeUniform37, render.nodeUniform38, ( ( dot( normalWorld, normalize( render.nodeUniform39 ) ) * 0.5 ) + 0.5 ) ) );
	irradiance = ( irradiance + mix( render.nodeUniform40, render.nodeUniform41, ( ( dot( normalWorld, normalize( render.nodeUniform42 ) ) * 0.5 ) + 0.5 ) ) );
	nodeVar15 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar15 );
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
	nodeVar16 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst36 ) );
	nodeVar17 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst36 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar16 );
	indirectDiffuse = ( indirectDiffuse + nodeVar17 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar18 = vec4<f32>( mix( Output.xyz, render.nodeUniform43, smoothstep( render.nodeUniform44, render.nodeUniform45, ( - v_positionView.z ) ) ), Output.w );
	let nodeConst37 = nodeVar18;
	Output = nodeConst37;

	// result

	output.color = nodeConst37;

	return output;

}
