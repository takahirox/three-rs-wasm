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
@binding( 3 ) @group( 1 ) var nodeUniform9_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform9 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform18_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform18 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform19_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform19 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform29_sampler : sampler_comparison;
@binding( 10 ) @group( 1 ) var nodeUniform29 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform6 : u32,
	nodeUniform7 : f32,
	nodeUniform8 : f32,
	nodeUniform10 : mat3x3<f32>,
	nodeUniform11 : f32,
	nodeUniform12 : mat3x3<f32>,
	nodeUniform15 : u32,
	nodeUniform16 : vec3<f32>,
	nodeUniform17 : f32,
	nodeUniform20 : mat3x3<f32>,
	nodeUniform21 : vec2<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform30 : f32,
	nodeUniform31 : vec2<f32>,
	nodeUniform27 : mat4x4<f32>,
	nodeUniform26 : vec4<f32>,
	nodeUniform33 : mat4x4<f32>,
	nodeUniform32 : vec4<f32>,
	nodeUniform25 : f32,
	nodeUniform28 : f32,
	nodeUniform34 : f32,
	nodeUniform35 : f32
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
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : f32;
var<private> shadowValue : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
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
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) nodeVarying5 : vec2<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_tangentView : vec3<f32>,
	@location( 4 ) v_bitangentView : vec3<f32>,
	@location( 5 ) v_positionViewDirection : vec3<f32>,
	@location( 6 ) v_positionWorld : vec3<f32>,
	@location( 7 ) NORMAL_v_bitangentView : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying5, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform7 );
	DiffuseColor.w = 1.0;
	nodeVar1 = textureSample( nodeUniform9, nodeUniform9_sampler, ( object.nodeUniform10 * vec3<f32>( nodeVarying5, 1.0 ) ).xy );
	Metalness = ( object.nodeUniform8 * nodeVar1.z );
	nodeVar2 = textureSample( nodeUniform9, nodeUniform9_sampler, ( object.nodeUniform12 * vec3<f32>( nodeVarying5, 1.0 ) ).xy );
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst2 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( object.nodeUniform11 * nodeVar2.y ), 0.045 ) + max( max( nodeConst2.x, nodeConst2.y ), nodeConst2.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - ( object.nodeUniform8 * nodeVar1.z ) ) ) );
	EmissiveColor = ( object.nodeUniform16 * vec3<f32>( object.nodeUniform17 ) );
	NORMAL_tangentView = normalize( v_tangentView );
	NORMAL_bitangentView = normalize( NORMAL_v_bitangentView );
	NORMAL_normalView = normalViewGeometry;
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar3 = textureSample( nodeUniform19, nodeUniform19_sampler, ( object.nodeUniform20 * vec3<f32>( nodeVarying5, 1.0 ) ).xy );
	let nodeConst3 = ( ( nodeVar3 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeConst3.xy * object.nodeUniform21 ), nodeConst3.z ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar4 = textureSample( nodeUniform18, nodeUniform18_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar4;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst4 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst4 );
	let nodeConst5 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst6 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst4 * nodeConst5 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst6 ) * nodeConst5 ) ) ) * vec3<f32>( nodeConst6 ) ) );
	let nodeConst7 = normalize( ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform23, 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar5 = vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform25 ) ) ), 1.0 );
	nodeVar6 = ( - v_positionView.z );
	shadowValue = 1.0;

	if ( ( ( nodeVar6 >= render.nodeUniform26.x ) && ( nodeVar6 < render.nodeUniform26.y ) ) ) {

		let nodeConst8 = ( render.nodeUniform27 * nodeVar5 );
		let nodeConst9 = ( nodeConst8.xyz / vec3<f32>( nodeConst8.w ) );
		let nodeConst10 = vec3<f32>( nodeConst9.x, ( 1.0 - nodeConst9.y ), ( nodeConst9.z + render.nodeUniform28 ) );

		if ( ( ( ( ( ( nodeConst10.x >= 0.0 ) && ( nodeConst10.x <= 1.0 ) ) && ( nodeConst10.y >= 0.0 ) ) && ( nodeConst10.y <= 1.0 ) ) && ( nodeConst10.z <= 1.0 ) ) ) {

			let nodeConst11 = ( fn45( fragCoord.xy ) * 6.28318530718 );
			let nodeConst12 = ( vec2<f32>( render.nodeUniform30 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform31 ) );
			let nodeConst13 = ( nodeConst10.xy + ( fn46( 0, 5, nodeConst11 ) * nodeConst12 ) );
			nodeVar8 = textureSampleCompare( nodeUniform29, nodeUniform29_sampler, nodeConst13, nodeConst10.z );
			let nodeConst14 = ( nodeConst10.xy + ( fn46( 1, 5, nodeConst11 ) * nodeConst12 ) );
			nodeVar9 = textureSampleCompare( nodeUniform29, nodeUniform29_sampler, nodeConst14, nodeConst10.z );
			let nodeConst15 = ( nodeConst10.xy + ( fn46( 2, 5, nodeConst11 ) * nodeConst12 ) );
			nodeVar10 = textureSampleCompare( nodeUniform29, nodeUniform29_sampler, nodeConst15, nodeConst10.z );
			let nodeConst16 = ( nodeConst10.xy + ( fn46( 3, 5, nodeConst11 ) * nodeConst12 ) );
			nodeVar11 = textureSampleCompare( nodeUniform29, nodeUniform29_sampler, nodeConst16, nodeConst10.z );
			let nodeConst17 = ( nodeConst10.xy + ( fn46( 4, 5, nodeConst11 ) * nodeConst12 ) );
			nodeVar12 = textureSampleCompare( nodeUniform29, nodeUniform29_sampler, nodeConst17, nodeConst10.z );
			nodeVar7 = ( ( ( ( ( nodeVar8 + nodeVar9 ) + nodeVar10 ) + nodeVar11 ) + nodeVar12 ) * 0.2 );

		} else {

			nodeVar7 = 1.0;

		}

		shadowValue = mix( nodeVar7, shadowValue, smoothstep( render.nodeUniform26.z, render.nodeUniform26.y, nodeVar6 ) );
		

	}


	if ( ( ( nodeVar6 >= render.nodeUniform32.x ) && ( nodeVar6 < render.nodeUniform32.y ) ) ) {

		let nodeConst18 = ( render.nodeUniform33 * nodeVar5 );
		let nodeConst19 = ( nodeConst18.xyz / vec3<f32>( nodeConst18.w ) );
		let nodeConst20 = vec3<f32>( nodeConst19.x, ( 1.0 - nodeConst19.y ), ( nodeConst19.z + render.nodeUniform34 ) );

		if ( ( ( ( ( ( nodeConst20.x >= 0.0 ) && ( nodeConst20.x <= 1.0 ) ) && ( nodeConst20.y >= 0.0 ) ) && ( nodeConst20.y <= 1.0 ) ) && ( nodeConst20.z <= 1.0 ) ) ) {

			let nodeConst21 = ( fn45( fragCoord.xy ) * 6.28318530718 );
			let nodeConst22 = ( vec2<f32>( render.nodeUniform30 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform31 ) );
			let nodeConst23 = ( nodeConst20.xy + ( fn46( 0, 5, nodeConst21 ) * nodeConst22 ) );
			nodeVar14 = textureSampleCompare( nodeUniform29, nodeUniform29_sampler, nodeConst23, nodeConst20.z );
			let nodeConst24 = ( nodeConst20.xy + ( fn46( 1, 5, nodeConst21 ) * nodeConst22 ) );
			nodeVar15 = textureSampleCompare( nodeUniform29, nodeUniform29_sampler, nodeConst24, nodeConst20.z );
			let nodeConst25 = ( nodeConst20.xy + ( fn46( 2, 5, nodeConst21 ) * nodeConst22 ) );
			nodeVar16 = textureSampleCompare( nodeUniform29, nodeUniform29_sampler, nodeConst25, nodeConst20.z );
			let nodeConst26 = ( nodeConst20.xy + ( fn46( 3, 5, nodeConst21 ) * nodeConst22 ) );
			nodeVar17 = textureSampleCompare( nodeUniform29, nodeUniform29_sampler, nodeConst26, nodeConst20.z );
			let nodeConst27 = ( nodeConst20.xy + ( fn46( 4, 5, nodeConst21 ) * nodeConst22 ) );
			nodeVar18 = textureSampleCompare( nodeUniform29, nodeUniform29_sampler, nodeConst27, nodeConst20.z );
			nodeVar13 = ( ( ( ( ( nodeVar14 + nodeVar15 ) + nodeVar16 ) + nodeVar17 ) + nodeVar18 ) * 0.2 );

		} else {

			nodeVar13 = 1.0;

		}

		shadowValue = mix( nodeVar13, shadowValue, smoothstep( render.nodeUniform32.z, render.nodeUniform32.y, nodeVar6 ) );
		

	}

	nodeVar19 = mix( 1.0, shadowValue, render.nodeUniform35 );
	nodeVar20 = ( vec3<f32>( clamp( dot( normalView, nodeConst7 ), 0.0, 1.0 ) ) * ( render.nodeUniform24 * vec3<f32>( nodeVar19 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst28 = clamp( dot( positionViewDirection, normalize( ( nodeConst7 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst29 = exp2( ( ( ( nodeConst28 * -5.55473 ) - 6.98316 ) * nodeConst28 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar20 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst29 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst29 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst30 = normalize( ( nodeConst7 + positionViewDirection ) );
	let nodeConst31 = clamp( dot( positionViewDirection, nodeConst30 ), 0.0, 1.0 );
	let nodeConst32 = exp2( ( ( ( nodeConst31 * -5.55473 ) - 6.98316 ) * nodeConst31 ) );
	let nodeConst33 = max( Roughness, 0.045 );
	let nodeConst34 = ( nodeConst33 * nodeConst33 );
	directSpecular = ( directSpecular + ( ( nodeVar20 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst32 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst32 ) ) ) * vec3<f32>( fn12( nodeConst34, clamp( dot( normalView, nodeConst7 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn14( nodeConst34, clamp( dot( normalView, nodeConst30 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar21 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar21 );
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
	nodeVar22 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst38 ) );
	nodeVar23 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst38 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar22 );
	indirectDiffuse = ( indirectDiffuse + nodeVar23 );
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
