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
@binding( 3 ) @group( 1 ) var nodeUniform5_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform5 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform13_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform13 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform15_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform15 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform25_sampler : sampler_comparison;
@binding( 10 ) @group( 1 ) var nodeUniform25 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform6 : mat3x3<f32>,
	nodeUniform7 : f32,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform10 : mat3x3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform14 : mat4x4<f32>,
	nodeUniform16 : mat3x3<f32>,
	nodeUniform17 : vec2<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform19 : vec3<f32>,
	nodeUniform26 : f32,
	nodeUniform27 : vec2<f32>,
	nodeUniform23 : mat4x4<f32>,
	nodeUniform22 : vec4<f32>,
	nodeUniform29 : mat4x4<f32>,
	nodeUniform28 : vec4<f32>,
	nodeUniform21 : f32,
	nodeUniform24 : f32,
	nodeUniform30 : f32,
	nodeUniform31 : f32
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
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_tangentView : vec3<f32>,
	@location( 3 ) v_bitangentView : vec3<f32>,
	@location( 4 ) v_positionViewDirection : vec3<f32>,
	@location( 5 ) v_positionWorld : vec3<f32>,
	@location( 6 ) NORMAL_v_bitangentView : vec3<f32>,
	@location( 7 ) nodeVarying10 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying10, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	nodeVar1 = textureSample( nodeUniform5, nodeUniform5_sampler, ( object.nodeUniform6 * vec3<f32>( nodeVarying10, 1.0 ) ).xy );
	Metalness = ( object.nodeUniform4 * nodeVar1.z );
	nodeVar2 = textureSample( nodeUniform5, nodeUniform5_sampler, ( object.nodeUniform8 * vec3<f32>( nodeVarying10, 1.0 ) ).xy );
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst1 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( ( object.nodeUniform7 * nodeVar2.y ), 0.045 ) + max( max( nodeConst1.x, nodeConst1.y ), nodeConst1.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - ( object.nodeUniform4 * nodeVar1.z ) ) ) );
	EmissiveColor = ( object.nodeUniform11 * vec3<f32>( object.nodeUniform12 ) );
	NORMAL_tangentView = normalize( v_tangentView );
	NORMAL_bitangentView = normalize( NORMAL_v_bitangentView );
	NORMAL_normalView = normalViewGeometry;
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar3 = textureSample( nodeUniform15, nodeUniform15_sampler, ( object.nodeUniform16 * vec3<f32>( nodeVarying10, 1.0 ) ).xy );
	let nodeConst2 = ( ( nodeVar3 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeConst2.xy * object.nodeUniform17 ), nodeConst2.z ) ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar4 = textureSample( nodeUniform13, nodeUniform13_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar4;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst3 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst3 );
	let nodeConst4 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst5 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst3 * nodeConst4 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst5 ) * nodeConst4 ) ) ) * vec3<f32>( nodeConst5 ) ) );
	let nodeConst6 = normalize( ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform19, 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar5 = vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform21 ) ) ), 1.0 );
	nodeVar6 = ( - v_positionView.z );
	shadowValue = 1.0;

	if ( ( ( nodeVar6 >= render.nodeUniform22.x ) && ( nodeVar6 < render.nodeUniform22.y ) ) ) {

		let nodeConst7 = ( render.nodeUniform23 * nodeVar5 );
		let nodeConst8 = ( nodeConst7.xyz / vec3<f32>( nodeConst7.w ) );
		let nodeConst9 = vec3<f32>( nodeConst8.x, ( 1.0 - nodeConst8.y ), ( nodeConst8.z + render.nodeUniform24 ) );

		if ( ( ( ( ( ( nodeConst9.x >= 0.0 ) && ( nodeConst9.x <= 1.0 ) ) && ( nodeConst9.y >= 0.0 ) ) && ( nodeConst9.y <= 1.0 ) ) && ( nodeConst9.z <= 1.0 ) ) ) {

			let nodeConst10 = ( fn45( fragCoord.xy ) * 6.28318530718 );
			let nodeConst11 = ( vec2<f32>( render.nodeUniform26 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform27 ) );
			let nodeConst12 = ( nodeConst9.xy + ( fn46( 0, 5, nodeConst10 ) * nodeConst11 ) );
			nodeVar8 = textureSampleCompare( nodeUniform25, nodeUniform25_sampler, nodeConst12, nodeConst9.z );
			let nodeConst13 = ( nodeConst9.xy + ( fn46( 1, 5, nodeConst10 ) * nodeConst11 ) );
			nodeVar9 = textureSampleCompare( nodeUniform25, nodeUniform25_sampler, nodeConst13, nodeConst9.z );
			let nodeConst14 = ( nodeConst9.xy + ( fn46( 2, 5, nodeConst10 ) * nodeConst11 ) );
			nodeVar10 = textureSampleCompare( nodeUniform25, nodeUniform25_sampler, nodeConst14, nodeConst9.z );
			let nodeConst15 = ( nodeConst9.xy + ( fn46( 3, 5, nodeConst10 ) * nodeConst11 ) );
			nodeVar11 = textureSampleCompare( nodeUniform25, nodeUniform25_sampler, nodeConst15, nodeConst9.z );
			let nodeConst16 = ( nodeConst9.xy + ( fn46( 4, 5, nodeConst10 ) * nodeConst11 ) );
			nodeVar12 = textureSampleCompare( nodeUniform25, nodeUniform25_sampler, nodeConst16, nodeConst9.z );
			nodeVar7 = ( ( ( ( ( nodeVar8 + nodeVar9 ) + nodeVar10 ) + nodeVar11 ) + nodeVar12 ) * 0.2 );

		} else {

			nodeVar7 = 1.0;

		}

		shadowValue = mix( nodeVar7, shadowValue, smoothstep( render.nodeUniform22.z, render.nodeUniform22.y, nodeVar6 ) );
		

	}


	if ( ( ( nodeVar6 >= render.nodeUniform28.x ) && ( nodeVar6 < render.nodeUniform28.y ) ) ) {

		let nodeConst17 = ( render.nodeUniform29 * nodeVar5 );
		let nodeConst18 = ( nodeConst17.xyz / vec3<f32>( nodeConst17.w ) );
		let nodeConst19 = vec3<f32>( nodeConst18.x, ( 1.0 - nodeConst18.y ), ( nodeConst18.z + render.nodeUniform30 ) );

		if ( ( ( ( ( ( nodeConst19.x >= 0.0 ) && ( nodeConst19.x <= 1.0 ) ) && ( nodeConst19.y >= 0.0 ) ) && ( nodeConst19.y <= 1.0 ) ) && ( nodeConst19.z <= 1.0 ) ) ) {

			let nodeConst20 = ( fn45( fragCoord.xy ) * 6.28318530718 );
			let nodeConst21 = ( vec2<f32>( render.nodeUniform26 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform27 ) );
			let nodeConst22 = ( nodeConst19.xy + ( fn46( 0, 5, nodeConst20 ) * nodeConst21 ) );
			nodeVar14 = textureSampleCompare( nodeUniform25, nodeUniform25_sampler, nodeConst22, nodeConst19.z );
			let nodeConst23 = ( nodeConst19.xy + ( fn46( 1, 5, nodeConst20 ) * nodeConst21 ) );
			nodeVar15 = textureSampleCompare( nodeUniform25, nodeUniform25_sampler, nodeConst23, nodeConst19.z );
			let nodeConst24 = ( nodeConst19.xy + ( fn46( 2, 5, nodeConst20 ) * nodeConst21 ) );
			nodeVar16 = textureSampleCompare( nodeUniform25, nodeUniform25_sampler, nodeConst24, nodeConst19.z );
			let nodeConst25 = ( nodeConst19.xy + ( fn46( 3, 5, nodeConst20 ) * nodeConst21 ) );
			nodeVar17 = textureSampleCompare( nodeUniform25, nodeUniform25_sampler, nodeConst25, nodeConst19.z );
			let nodeConst26 = ( nodeConst19.xy + ( fn46( 4, 5, nodeConst20 ) * nodeConst21 ) );
			nodeVar18 = textureSampleCompare( nodeUniform25, nodeUniform25_sampler, nodeConst26, nodeConst19.z );
			nodeVar13 = ( ( ( ( ( nodeVar14 + nodeVar15 ) + nodeVar16 ) + nodeVar17 ) + nodeVar18 ) * 0.2 );

		} else {

			nodeVar13 = 1.0;

		}

		shadowValue = mix( nodeVar13, shadowValue, smoothstep( render.nodeUniform28.z, render.nodeUniform28.y, nodeVar6 ) );
		

	}

	nodeVar19 = mix( 1.0, shadowValue, render.nodeUniform31 );
	nodeVar20 = ( vec3<f32>( clamp( dot( normalView, nodeConst6 ), 0.0, 1.0 ) ) * ( render.nodeUniform20 * vec3<f32>( nodeVar19 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst27 = clamp( dot( positionViewDirection, normalize( ( nodeConst6 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst28 = exp2( ( ( ( nodeConst27 * -5.55473 ) - 6.98316 ) * nodeConst27 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar20 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst28 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst28 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst29 = normalize( ( nodeConst6 + positionViewDirection ) );
	let nodeConst30 = clamp( dot( positionViewDirection, nodeConst29 ), 0.0, 1.0 );
	let nodeConst31 = exp2( ( ( ( nodeConst30 * -5.55473 ) - 6.98316 ) * nodeConst30 ) );
	let nodeConst32 = max( Roughness, 0.045 );
	let nodeConst33 = ( nodeConst32 * nodeConst32 );
	directSpecular = ( directSpecular + ( ( nodeVar20 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst31 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst31 ) ) ) * vec3<f32>( fn12( nodeConst33, clamp( dot( normalView, nodeConst6 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn14( nodeConst33, clamp( dot( normalView, nodeConst29 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar21 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar21 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst34 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst34 );
	let nodeConst35 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst36 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst34 * nodeConst35 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst36 ) * nodeConst35 ) ) ) * vec3<f32>( nodeConst36 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst37 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar22 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst37 ) );
	nodeVar23 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst37 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar22 );
	indirectDiffuse = ( indirectDiffuse + nodeVar23 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst38 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst38;

	// result

	output.color = nodeConst38;

	return output;

}
