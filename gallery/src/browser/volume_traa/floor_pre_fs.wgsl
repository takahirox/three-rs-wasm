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
@binding( 1 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform8 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform23_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform23 : texture_depth_cube;
@binding( 5 ) @group( 1 ) var nodeUniform32_sampler : sampler_comparison;
@binding( 6 ) @group( 1 ) var nodeUniform32 : texture_depth_2d;
@binding( 7 ) @group( 1 ) var nodeUniform42_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform42 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform25 : f32,
	nodeUniform26 : f32,
	nodeUniform36 : f32,
	nodeUniform37 : f32,
	nodeUniform40 : f32,
	nodeUniform41 : f32,
	nodeUniform29 : vec3<f32>,
	nodeUniform38 : vec3<f32>,
	nodeUniform39 : vec3<f32>,
	nodeUniform28 : mat4x4<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform16 : f32,
	nodeUniform15 : f32,
	nodeUniform14 : f32,
	nodeUniform17 : f32,
	nodeUniform18 : f32,
	nodeUniform19 : vec2<f32>,
	nodeUniform20 : f32,
	nodeUniform21 : f32,
	nodeUniform22 : f32,
	nodeUniform24 : f32,
	nodeUniform30 : f32,
	nodeUniform31 : f32,
	nodeUniform33 : f32,
	nodeUniform34 : vec2<f32>,
	nodeUniform35 : f32
};
@binding( 0 ) @group( 0 )
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
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn24 ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn fn25 ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	

	let nodeConst0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeConst0 ), sin( nodeConst0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

}


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
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionWorld : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

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
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar0 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar0;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst1 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst1 );
	let nodeConst2 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst3 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst1 * nodeConst2 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst3 ) * nodeConst2 ) ) ) * vec3<f32>( nodeConst3 ) ) );
	let nodeConst4 = ( render.nodeUniform10 - v_positionView );
	let nodeConst5 = normalize( nodeConst4 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst6 = ( render.nodeUniform12 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform14 ) ) ), 1.0 ) ).xyz;
	let nodeConst7 = abs( nodeConst6 );
	nodeVar1 = 1.0;
	let nodeConst8 = max( max( nodeConst7.x, nodeConst7.y ), nodeConst7.z );

	if ( ( ( ( nodeConst8 - render.nodeUniform15 ) <= 0.0 ) && ( ( nodeConst8 - render.nodeUniform16 ) >= 0.0 ) ) ) {

		let nodeConst9 = ( render.nodeUniform15 - render.nodeUniform16 );
		let nodeConst10 = ( ( render.nodeUniform15 / nodeConst9 ) + render.nodeUniform17 );
		let nodeConst11 = length( nodeConst6 );
		let nodeConst12 = ( - ( ( render.nodeUniform16 * render.nodeUniform15 ) / ( nodeConst9 * nodeConst11 ) ) );
		let nodeConst13 = ( render.nodeUniform18 / render.nodeUniform19.x );
		let nodeConst14 = normalize( nodeConst6 );
		let nodeConst15 = abs( nodeConst14 );

		if ( ( nodeConst15.x > nodeConst15.z ) ) {

			nodeVar2 = vec3<f32>( 0.0, 1.0, 0.0 );

		} else {

			nodeVar2 = vec3<f32>( 1.0, 0.0, 0.0 );

		}

		let nodeConst16 = normalize( cross( nodeConst14, nodeVar2 ) );
		let nodeConst17 = cross( nodeConst14, nodeConst16 );
		let nodeConst18 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst19 = ( render.nodeUniform20 - render.nodeUniform21 );
		let nodeConst20 = ( ( render.nodeUniform20 / nodeConst19 ) + render.nodeUniform22 );
		let nodeConst21 = ( - ( ( render.nodeUniform21 * render.nodeUniform20 ) / ( nodeConst19 * nodeConst11 ) ) );
		nodeVar3 = 0.0;

		for ( var i : i32 = 0; i < 5; i ++ ) {

			let nodeConst22 = fn25( i, 5, nodeConst18 );
			let nodeConst23 = ( nodeConst14 + ( ( ( nodeConst16 * vec3<f32>( nodeConst22.x ) ) + ( nodeConst17 * vec3<f32>( nodeConst22.y ) ) ) * vec3<f32>( nodeConst13 ) ) );
			let nodeConst24 = abs( nodeConst23 );
			let nodeConst25 = max( max( nodeConst24.x, nodeConst24.y ), nodeConst24.z );
			nodeVar4 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, vec3<f32>( ( - nodeConst23.x ), nodeConst23.yz ), ( nodeConst20 + ( nodeConst21 / nodeConst25 ) ) );
			nodeVar3 = ( nodeVar3 + nodeVar4 );

		}

		nodeVar1 = ( nodeVar3 * 0.2 );
		

	}

	nodeVar5 = mix( 1.0, nodeVar1, render.nodeUniform24 );
	nodeVar6 = ( vec3<f32>( clamp( dot( normalView, nodeConst5 ), 0.0, 1.0 ) ) * ( ( render.nodeUniform11 * vec3<f32>( nodeVar5 ) ) * vec3<f32>( fn44( length( nodeConst4 ), render.nodeUniform25, render.nodeUniform26 ) ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst26 = clamp( dot( positionViewDirection, normalize( ( nodeConst5 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst27 = exp2( ( ( ( nodeConst26 * -5.55473 ) - 6.98316 ) * nodeConst26 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar6 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst27 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst27 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst28 = normalize( ( nodeConst5 + positionViewDirection ) );
	let nodeConst29 = clamp( dot( positionViewDirection, nodeConst28 ), 0.0, 1.0 );
	let nodeConst30 = exp2( ( ( ( nodeConst29 * -5.55473 ) - 6.98316 ) * nodeConst29 ) );
	let nodeConst31 = max( Roughness, 0.045 );
	let nodeConst32 = ( nodeConst31 * nodeConst31 );
	directSpecular = ( directSpecular + ( ( nodeVar6 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst30 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst30 ) ) ) * vec3<f32>( fn110( nodeConst32, clamp( dot( normalView, nodeConst5 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst32, clamp( dot( normalView, nodeConst28 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst33 = ( render.nodeUniform27 - v_positionView );
	let nodeConst34 = normalize( nodeConst33 );
	let nodeConst35 = ( render.nodeUniform28 * vec4<f32>( v_positionWorld, 1.0 ) );
	let nodeConst36 = ( nodeConst35.xyz / vec3<f32>( nodeConst35.w ) );

	if ( all( ( abs( ( ( nodeConst36 * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ) ) < vec3<f32>( 1.0 ) ) ) ) {

		shadowPositionWorld = v_positionWorld;
		let nodeConst37 = ( render.nodeUniform28 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform30 ) ) ), 1.0 ) );
		let nodeConst38 = ( nodeConst37.xyz / vec3<f32>( nodeConst37.w ) );
		let nodeConst39 = vec3<f32>( nodeConst38.x, ( 1.0 - nodeConst38.y ), ( nodeConst38.z + render.nodeUniform31 ) );

		if ( ( ( ( ( ( nodeConst39.x >= 0.0 ) && ( nodeConst39.x <= 1.0 ) ) && ( nodeConst39.y >= 0.0 ) ) && ( nodeConst39.y <= 1.0 ) ) && ( nodeConst39.z <= 1.0 ) ) ) {

			let nodeConst40 = ( fn24( fragCoord.xy ) * 6.28318530718 );
			let nodeConst41 = ( vec2<f32>( render.nodeUniform33 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform34 ) );
			let nodeConst42 = ( nodeConst39.xy + ( fn25( 0, 5, nodeConst40 ) * nodeConst41 ) );
			nodeVar9 = textureSampleCompare( nodeUniform32, nodeUniform32_sampler, nodeConst42, nodeConst39.z );
			let nodeConst43 = ( nodeConst39.xy + ( fn25( 1, 5, nodeConst40 ) * nodeConst41 ) );
			nodeVar10 = textureSampleCompare( nodeUniform32, nodeUniform32_sampler, nodeConst43, nodeConst39.z );
			let nodeConst44 = ( nodeConst39.xy + ( fn25( 2, 5, nodeConst40 ) * nodeConst41 ) );
			nodeVar11 = textureSampleCompare( nodeUniform32, nodeUniform32_sampler, nodeConst44, nodeConst39.z );
			let nodeConst45 = ( nodeConst39.xy + ( fn25( 3, 5, nodeConst40 ) * nodeConst41 ) );
			nodeVar12 = textureSampleCompare( nodeUniform32, nodeUniform32_sampler, nodeConst45, nodeConst39.z );
			let nodeConst46 = ( nodeConst39.xy + ( fn25( 4, 5, nodeConst40 ) * nodeConst41 ) );
			nodeVar13 = textureSampleCompare( nodeUniform32, nodeUniform32_sampler, nodeConst46, nodeConst39.z );
			nodeVar8 = ( ( ( ( ( nodeVar9 + nodeVar10 ) + nodeVar11 ) + nodeVar12 ) + nodeVar13 ) * 0.2 );

		} else {

			nodeVar8 = 1.0;

		}

		nodeVar14 = mix( 1.0, nodeVar8, render.nodeUniform35 );
		let nodeConst47 = ( ( ( render.nodeUniform29 * vec3<f32>( nodeVar14 ) ) * vec3<f32>( smoothstep( render.nodeUniform36, render.nodeUniform37, dot( nodeConst34, normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform38 - render.nodeUniform39 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( fn44( length( nodeConst33 ), render.nodeUniform40, render.nodeUniform41 ) ) );
		nodeVar15 = textureSample( nodeUniform42, nodeUniform42_sampler, nodeConst36.xy );
		nodeVar7 = ( vec4<f32>( nodeConst47, 1.0 ) * nodeVar15 );

	} else {

		shadowPositionWorld = v_positionWorld;
		let nodeConst37 = ( render.nodeUniform28 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform30 ) ) ), 1.0 ) );
		let nodeConst38 = ( nodeConst37.xyz / vec3<f32>( nodeConst37.w ) );
		let nodeConst39 = vec3<f32>( nodeConst38.x, ( 1.0 - nodeConst38.y ), ( nodeConst38.z + render.nodeUniform31 ) );

		if ( ( ( ( ( ( nodeConst39.x >= 0.0 ) && ( nodeConst39.x <= 1.0 ) ) && ( nodeConst39.y >= 0.0 ) ) && ( nodeConst39.y <= 1.0 ) ) && ( nodeConst39.z <= 1.0 ) ) ) {

			let nodeConst40 = ( fn24( fragCoord.xy ) * 6.28318530718 );
			let nodeConst41 = ( vec2<f32>( render.nodeUniform33 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform34 ) );
			let nodeConst42 = ( nodeConst39.xy + ( fn25( 0, 5, nodeConst40 ) * nodeConst41 ) );
			nodeVar9 = textureSampleCompare( nodeUniform32, nodeUniform32_sampler, nodeConst42, nodeConst39.z );
			let nodeConst43 = ( nodeConst39.xy + ( fn25( 1, 5, nodeConst40 ) * nodeConst41 ) );
			nodeVar10 = textureSampleCompare( nodeUniform32, nodeUniform32_sampler, nodeConst43, nodeConst39.z );
			let nodeConst44 = ( nodeConst39.xy + ( fn25( 2, 5, nodeConst40 ) * nodeConst41 ) );
			nodeVar11 = textureSampleCompare( nodeUniform32, nodeUniform32_sampler, nodeConst44, nodeConst39.z );
			let nodeConst45 = ( nodeConst39.xy + ( fn25( 3, 5, nodeConst40 ) * nodeConst41 ) );
			nodeVar12 = textureSampleCompare( nodeUniform32, nodeUniform32_sampler, nodeConst45, nodeConst39.z );
			let nodeConst46 = ( nodeConst39.xy + ( fn25( 4, 5, nodeConst40 ) * nodeConst41 ) );
			nodeVar13 = textureSampleCompare( nodeUniform32, nodeUniform32_sampler, nodeConst46, nodeConst39.z );
			nodeVar16 = ( ( ( ( ( nodeVar9 + nodeVar10 ) + nodeVar11 ) + nodeVar12 ) + nodeVar13 ) * 0.2 );

		} else {

			nodeVar16 = 1.0;

		}

		nodeVar14 = mix( 1.0, nodeVar16, render.nodeUniform35 );
		let nodeConst47 = ( ( ( render.nodeUniform29 * vec3<f32>( nodeVar14 ) ) * vec3<f32>( smoothstep( render.nodeUniform36, render.nodeUniform37, dot( nodeConst34, normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform38 - render.nodeUniform39 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( fn44( length( nodeConst33 ), render.nodeUniform40, render.nodeUniform41 ) ) );
		nodeVar7 = vec4<f32>( nodeConst47, 1.0 );

	}

	nodeVar17 = ( vec4<f32>( clamp( dot( normalView, nodeConst34 ), 0.0, 1.0 ) ) * nodeVar7 );
	let nodeConst48 = clamp( dot( positionViewDirection, normalize( ( nodeConst34 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst49 = exp2( ( ( ( nodeConst48 * -5.55473 ) - 6.98316 ) * nodeConst48 ) );
	directDiffuse = ( vec4<f32>( directDiffuse, 1.0 ) + ( ( nodeVar17 * vec4<f32>( ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ), 1.0 ) ) * vec4<f32>( ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst49 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst49 ) ) ) ), 1.0 ) ) ).xyz;
	let nodeConst50 = normalize( ( nodeConst34 + positionViewDirection ) );
	let nodeConst51 = clamp( dot( positionViewDirection, nodeConst50 ), 0.0, 1.0 );
	let nodeConst52 = exp2( ( ( ( nodeConst51 * -5.55473 ) - 6.98316 ) * nodeConst51 ) );
	let nodeConst53 = max( Roughness, 0.045 );
	let nodeConst54 = ( nodeConst53 * nodeConst53 );
	directSpecular = ( vec4<f32>( directSpecular, 1.0 ) + ( ( nodeVar17 * vec4<f32>( ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst52 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst52 ) ) ) * vec3<f32>( fn110( nodeConst54, clamp( dot( normalView, nodeConst34 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst54, clamp( dot( normalView, nodeConst50 ), 0.0, 1.0 ) ) ) ), 1.0 ) ) * vec4<f32>( multiScatteringCompensation, 1.0 ) ) ).xyz;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar18 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar18 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst55 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst55 );
	let nodeConst56 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst57 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst55 * nodeConst56 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst57 ) * nodeConst56 ) ) ) * vec3<f32>( nodeConst57 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst58 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar19 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst58 ) );
	nodeVar20 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst58 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar19 );
	indirectDiffuse = ( indirectDiffuse + nodeVar20 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst59 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst59;

	// result

	output.color = nodeConst59;

	return output;

}
