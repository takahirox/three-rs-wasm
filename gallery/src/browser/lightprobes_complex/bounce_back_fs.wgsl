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
@binding( 3 ) @group( 2 ) var nodeUniform24_sampler : sampler_comparison;
@binding( 4 ) @group( 2 ) var nodeUniform24 : texture_depth_cube;
@binding( 5 ) @group( 2 ) var nodeUniform40_sampler : sampler_comparison;
@binding( 6 ) @group( 2 ) var nodeUniform40 : texture_depth_cube;
@binding( 7 ) @group( 2 ) var nodeUniform48_sampler : sampler;
@binding( 8 ) @group( 2 ) var nodeUniform48 : texture_3d<f32>;
@binding( 9 ) @group( 2 ) var nodeUniform54_sampler : sampler;
@binding( 10 ) @group( 2 ) var nodeUniform54 : texture_3d<f32>;

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
	nodeUniform44 : vec3<f32>,
	nodeUniform45 : vec3<f32>,
	nodeUniform46 : vec3<f32>,
	nodeUniform50 : vec3<f32>,
	nodeUniform51 : vec3<f32>,
	nodeUniform52 : vec3<f32>
};
@binding( 0 ) @group( 2 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform13 : vec3<f32>,
	nodeUniform26 : f32,
	nodeUniform27 : f32,
	nodeUniform29 : vec3<f32>,
	nodeUniform42 : f32,
	nodeUniform43 : f32,
	nodeUniform12 : vec3<f32>,
	nodeUniform14 : mat4x4<f32>,
	nodeUniform15 : f32,
	nodeUniform25 : f32,
	nodeUniform28 : vec3<f32>,
	nodeUniform30 : mat4x4<f32>,
	nodeUniform31 : f32,
	nodeUniform41 : f32,
	nodeUniform55 : f32,
	nodeUniform53 : f32,
	nodeUniform49 : f32,
	nodeUniform47 : f32,
	nodeUniform17 : f32,
	nodeUniform16 : f32,
	nodeUniform18 : f32,
	nodeUniform19 : f32,
	nodeUniform20 : vec2<f32>,
	nodeUniform21 : f32,
	nodeUniform22 : f32,
	nodeUniform23 : f32,
	nodeUniform33 : f32,
	nodeUniform32 : f32,
	nodeUniform34 : f32,
	nodeUniform35 : f32,
	nodeUniform36 : vec2<f32>,
	nodeUniform37 : f32,
	nodeUniform38 : f32,
	nodeUniform39 : f32
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
var<private> nodeVar7 : f32;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec3<f32>;
var<private> lightProbeIrradiance : vec3<f32>;
var<private> lightProbePriority : f32;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : vec4<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : vec4<f32>;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : vec4<f32>;
var<private> nodeVar24 : vec4<f32>;
var<private> nodeVar25 : vec4<f32>;
var<private> nodeVar26 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
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
fn main( @location( 0 ) @interpolate(flat, either) v_cameraIndex : u32,
	@location( 1 ) v_positionView : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@location( 4 ) v_positionWorld : vec3<f32>,
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
	EmissiveColor = ( object.nodeUniform7 * vec3<f32>( object.nodeUniform8 ) );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
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
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * cameraViewMatrices.value[ v_cameraIndex ] ).xyz );
	let nodeConst6 = ( render.nodeUniform14 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform15 ) ) ), 1.0 ) ).xyz;
	let nodeConst7 = abs( nodeConst6 );
	nodeVar1 = 1.0;
	let nodeConst8 = max( max( nodeConst7.x, nodeConst7.y ), nodeConst7.z );

	if ( ( ( ( nodeConst8 - render.nodeUniform16 ) <= 0.0 ) && ( ( nodeConst8 - render.nodeUniform17 ) >= 0.0 ) ) ) {

		let nodeConst9 = ( render.nodeUniform16 - render.nodeUniform17 );
		let nodeConst10 = ( ( render.nodeUniform16 / nodeConst9 ) + render.nodeUniform18 );
		let nodeConst11 = length( nodeConst6 );
		let nodeConst12 = ( - ( ( render.nodeUniform17 * render.nodeUniform16 ) / ( nodeConst9 * nodeConst11 ) ) );
		let nodeConst13 = ( render.nodeUniform19 / render.nodeUniform20.x );
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
		let nodeConst19 = ( render.nodeUniform21 - render.nodeUniform22 );
		let nodeConst20 = ( ( render.nodeUniform21 / nodeConst19 ) + render.nodeUniform23 );
		let nodeConst21 = ( - ( ( render.nodeUniform22 * render.nodeUniform21 ) / ( nodeConst19 * nodeConst11 ) ) );
		nodeVar3 = 0.0;

		for ( var i : i32 = 0; i < 5; i ++ ) {

			let nodeConst22 = fn25( i, 5, nodeConst18 );
			let nodeConst23 = ( nodeConst14 + ( ( ( nodeConst16 * vec3<f32>( nodeConst22.x ) ) + ( nodeConst17 * vec3<f32>( nodeConst22.y ) ) ) * vec3<f32>( nodeConst13 ) ) );
			let nodeConst24 = abs( nodeConst23 );
			let nodeConst25 = max( max( nodeConst24.x, nodeConst24.y ), nodeConst24.z );
			nodeVar4 = textureSampleCompare( nodeUniform24, nodeUniform24_sampler, vec3<f32>( ( - nodeConst23.x ), nodeConst23.yz ), ( nodeConst20 + ( nodeConst21 / nodeConst25 ) ) );
			nodeVar3 = ( nodeVar3 + nodeVar4 );

		}

		nodeVar1 = ( nodeVar3 * 0.2 );
		

	}

	nodeVar5 = mix( 1.0, nodeVar1, render.nodeUniform25 );
	nodeVar6 = ( vec3<f32>( clamp( dot( normalView, nodeConst5 ), 0.0, 1.0 ) ) * ( ( render.nodeUniform13 * vec3<f32>( nodeVar5 ) ) * vec3<f32>( fn44( length( nodeConst4 ), render.nodeUniform26, render.nodeUniform27 ) ) ) );
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
	let nodeConst33 = ( ( cameraViewMatrices.value[ v_cameraIndex ] * vec4<f32>( render.nodeUniform28, 1.0 ) ).xyz - v_positionView );
	let nodeConst34 = normalize( nodeConst33 );
	shadowPositionWorld = v_positionWorld;
	let nodeConst35 = ( render.nodeUniform30 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform31 ) ) ), 1.0 ) ).xyz;
	let nodeConst36 = abs( nodeConst35 );
	nodeVar7 = 1.0;
	let nodeConst37 = max( max( nodeConst36.x, nodeConst36.y ), nodeConst36.z );

	if ( ( ( ( nodeConst37 - render.nodeUniform32 ) <= 0.0 ) && ( ( nodeConst37 - render.nodeUniform33 ) >= 0.0 ) ) ) {

		let nodeConst38 = ( render.nodeUniform32 - render.nodeUniform33 );
		let nodeConst39 = ( ( render.nodeUniform32 / nodeConst38 ) + render.nodeUniform34 );
		let nodeConst40 = length( nodeConst35 );
		let nodeConst41 = ( - ( ( render.nodeUniform33 * render.nodeUniform32 ) / ( nodeConst38 * nodeConst40 ) ) );
		let nodeConst42 = ( render.nodeUniform35 / render.nodeUniform36.x );
		let nodeConst43 = normalize( nodeConst35 );
		let nodeConst44 = abs( nodeConst43 );

		if ( ( nodeConst44.x > nodeConst44.z ) ) {

			nodeVar8 = vec3<f32>( 0.0, 1.0, 0.0 );

		} else {

			nodeVar8 = vec3<f32>( 1.0, 0.0, 0.0 );

		}

		let nodeConst45 = normalize( cross( nodeConst43, nodeVar8 ) );
		let nodeConst46 = cross( nodeConst43, nodeConst45 );
		let nodeConst47 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst48 = ( render.nodeUniform37 - render.nodeUniform38 );
		let nodeConst49 = ( ( render.nodeUniform37 / nodeConst48 ) + render.nodeUniform39 );
		let nodeConst50 = ( - ( ( render.nodeUniform38 * render.nodeUniform37 ) / ( nodeConst48 * nodeConst40 ) ) );
		nodeVar9 = 0.0;

		for ( var i : i32 = 0; i < 5; i ++ ) {

			let nodeConst51 = fn25( i, 5, nodeConst47 );
			let nodeConst52 = ( nodeConst43 + ( ( ( nodeConst45 * vec3<f32>( nodeConst51.x ) ) + ( nodeConst46 * vec3<f32>( nodeConst51.y ) ) ) * vec3<f32>( nodeConst42 ) ) );
			let nodeConst53 = abs( nodeConst52 );
			let nodeConst54 = max( max( nodeConst53.x, nodeConst53.y ), nodeConst53.z );
			nodeVar10 = textureSampleCompare( nodeUniform40, nodeUniform40_sampler, vec3<f32>( ( - nodeConst52.x ), nodeConst52.yz ), ( nodeConst49 + ( nodeConst50 / nodeConst54 ) ) );
			nodeVar9 = ( nodeVar9 + nodeVar10 );

		}

		nodeVar7 = ( nodeVar9 * 0.2 );
		

	}

	nodeVar11 = mix( 1.0, nodeVar7, render.nodeUniform41 );
	nodeVar12 = ( vec3<f32>( clamp( dot( normalView, nodeConst34 ), 0.0, 1.0 ) ) * ( ( render.nodeUniform29 * vec3<f32>( nodeVar11 ) ) * vec3<f32>( fn44( length( nodeConst33 ), render.nodeUniform42, render.nodeUniform43 ) ) ) );
	let nodeConst55 = clamp( dot( positionViewDirection, normalize( ( nodeConst34 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst56 = exp2( ( ( ( nodeConst55 * -5.55473 ) - 6.98316 ) * nodeConst55 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar12 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst56 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst56 ) ) ) ) ) );
	let nodeConst57 = normalize( ( nodeConst34 + positionViewDirection ) );
	let nodeConst58 = clamp( dot( positionViewDirection, nodeConst57 ), 0.0, 1.0 );
	let nodeConst59 = exp2( ( ( ( nodeConst58 * -5.55473 ) - 6.98316 ) * nodeConst58 ) );
	let nodeConst60 = max( Roughness, 0.045 );
	let nodeConst61 = ( nodeConst60 * nodeConst60 );
	directSpecular = ( directSpecular + ( ( nodeVar12 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst59 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst59 ) ) ) * vec3<f32>( fn110( nodeConst61, clamp( dot( normalView, nodeConst34 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst61, clamp( dot( normalView, nodeConst57 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	lightProbeIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	lightProbePriority = ( - 1000000.0 );
	let nodeConst62 = ( ( object.nodeUniform44 - object.nodeUniform45 ) / object.nodeUniform46 );
	let nodeConst63 = ( v_positionWorld + ( ( normalWorld * nodeConst62 ) * vec3<f32>( 0.5 ) ) );
	let nodeConst64 = ( nodeConst62 * vec3<f32>( 0.01 ) );

	if ( ( ( all( ( nodeConst63 >= ( object.nodeUniform45 - nodeConst64 ) ) ) && all( ( nodeConst63 <= ( object.nodeUniform44 + nodeConst64 ) ) ) ) && ( render.nodeUniform47 >= lightProbePriority ) ) ) {

		let nodeConst65 = ( object.nodeUniform44 - object.nodeUniform45 );
		let nodeConst66 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform46 );
		let nodeConst67 = clamp( ( ( ( v_positionWorld + ( ( normalWorld * ( nodeConst65 / object.nodeUniform46 ) ) * vec3<f32>( 0.5 ) ) ) - object.nodeUniform45 ) / nodeConst65 ), nodeConst66, ( vec3<f32>( 1.0 ) - nodeConst66 ) );
		let nodeConst68 = ( ( nodeConst67.z * object.nodeUniform46.z ) + 1.0 );
		let nodeConst69 = ( object.nodeUniform46.z + 2.0 );
		let nodeConst70 = ( nodeConst69 * 7.0 );
		nodeVar13 = textureSample( nodeUniform48, nodeUniform48_sampler, vec3<f32>( nodeConst67.xy, ( ( nodeConst68 + ( nodeConst69 * 0.0 ) ) / nodeConst70 ) ) );
		nodeVar14 = textureSample( nodeUniform48, nodeUniform48_sampler, vec3<f32>( nodeConst67.xy, ( ( nodeConst68 + ( nodeConst69 * 1.0 ) ) / nodeConst70 ) ) );
		nodeVar15 = textureSample( nodeUniform48, nodeUniform48_sampler, vec3<f32>( nodeConst67.xy, ( ( nodeConst68 + ( nodeConst69 * 2.0 ) ) / nodeConst70 ) ) );
		nodeVar16 = textureSample( nodeUniform48, nodeUniform48_sampler, vec3<f32>( nodeConst67.xy, ( ( nodeConst68 + ( nodeConst69 * 3.0 ) ) / nodeConst70 ) ) );
		nodeVar17 = textureSample( nodeUniform48, nodeUniform48_sampler, vec3<f32>( nodeConst67.xy, ( ( nodeConst68 + ( nodeConst69 * 4.0 ) ) / nodeConst70 ) ) );
		nodeVar18 = textureSample( nodeUniform48, nodeUniform48_sampler, vec3<f32>( nodeConst67.xy, ( ( nodeConst68 + ( nodeConst69 * 5.0 ) ) / nodeConst70 ) ) );
		nodeVar19 = textureSample( nodeUniform48, nodeUniform48_sampler, vec3<f32>( nodeConst67.xy, ( ( nodeConst68 + ( nodeConst69 * 6.0 ) ) / nodeConst70 ) ) );
		let nodeConst71 = array< vec3<f32>, 9 >( nodeVar13.xyz, vec3<f32>( nodeVar13.w, nodeVar14.xy ), vec3<f32>( nodeVar14.zw, nodeVar15.x ), nodeVar15.yzw, nodeVar16.xyz, vec3<f32>( nodeVar16.w, nodeVar17.xy ), vec3<f32>( nodeVar17.zw, nodeVar18.x ), nodeVar18.yzw, nodeVar19.xyz );
		let nodeConst72 = ( max( ( ( ( ( ( ( ( ( ( nodeConst71[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeConst71[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeConst71[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst71[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeConst71[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeConst71[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeConst71[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeConst71[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst71[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ) * vec3<f32>( render.nodeUniform49 ) );
		lightProbeIrradiance = nodeConst72;
		lightProbePriority = render.nodeUniform47;
		

	}

	let nodeConst73 = ( ( object.nodeUniform50 - object.nodeUniform51 ) / object.nodeUniform52 );
	let nodeConst74 = ( v_positionWorld + ( ( normalWorld * nodeConst73 ) * vec3<f32>( 0.5 ) ) );
	let nodeConst75 = ( nodeConst73 * vec3<f32>( 0.01 ) );

	if ( ( ( all( ( nodeConst74 >= ( object.nodeUniform51 - nodeConst75 ) ) ) && all( ( nodeConst74 <= ( object.nodeUniform50 + nodeConst75 ) ) ) ) && ( render.nodeUniform53 >= lightProbePriority ) ) ) {

		let nodeConst76 = ( object.nodeUniform50 - object.nodeUniform51 );
		let nodeConst77 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform52 );
		let nodeConst78 = clamp( ( ( ( v_positionWorld + ( ( normalWorld * ( nodeConst76 / object.nodeUniform52 ) ) * vec3<f32>( 0.5 ) ) ) - object.nodeUniform51 ) / nodeConst76 ), nodeConst77, ( vec3<f32>( 1.0 ) - nodeConst77 ) );
		let nodeConst79 = ( ( nodeConst78.z * object.nodeUniform52.z ) + 1.0 );
		let nodeConst80 = ( object.nodeUniform52.z + 2.0 );
		let nodeConst81 = ( nodeConst80 * 7.0 );
		nodeVar20 = textureSample( nodeUniform54, nodeUniform54_sampler, vec3<f32>( nodeConst78.xy, ( ( nodeConst79 + ( nodeConst80 * 0.0 ) ) / nodeConst81 ) ) );
		nodeVar21 = textureSample( nodeUniform54, nodeUniform54_sampler, vec3<f32>( nodeConst78.xy, ( ( nodeConst79 + ( nodeConst80 * 1.0 ) ) / nodeConst81 ) ) );
		nodeVar22 = textureSample( nodeUniform54, nodeUniform54_sampler, vec3<f32>( nodeConst78.xy, ( ( nodeConst79 + ( nodeConst80 * 2.0 ) ) / nodeConst81 ) ) );
		nodeVar23 = textureSample( nodeUniform54, nodeUniform54_sampler, vec3<f32>( nodeConst78.xy, ( ( nodeConst79 + ( nodeConst80 * 3.0 ) ) / nodeConst81 ) ) );
		nodeVar24 = textureSample( nodeUniform54, nodeUniform54_sampler, vec3<f32>( nodeConst78.xy, ( ( nodeConst79 + ( nodeConst80 * 4.0 ) ) / nodeConst81 ) ) );
		nodeVar25 = textureSample( nodeUniform54, nodeUniform54_sampler, vec3<f32>( nodeConst78.xy, ( ( nodeConst79 + ( nodeConst80 * 5.0 ) ) / nodeConst81 ) ) );
		nodeVar26 = textureSample( nodeUniform54, nodeUniform54_sampler, vec3<f32>( nodeConst78.xy, ( ( nodeConst79 + ( nodeConst80 * 6.0 ) ) / nodeConst81 ) ) );
		let nodeConst82 = array< vec3<f32>, 9 >( nodeVar20.xyz, vec3<f32>( nodeVar20.w, nodeVar21.xy ), vec3<f32>( nodeVar21.zw, nodeVar22.x ), nodeVar22.yzw, nodeVar23.xyz, vec3<f32>( nodeVar23.w, nodeVar24.xy ), vec3<f32>( nodeVar24.zw, nodeVar25.x ), nodeVar25.yzw, nodeVar26.xyz );
		let nodeConst83 = ( max( ( ( ( ( ( ( ( ( ( nodeConst82[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeConst82[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeConst82[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst82[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeConst82[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeConst82[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeConst82[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeConst82[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst82[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ) * vec3<f32>( render.nodeUniform55 ) );
		lightProbeIrradiance = nodeConst83;
		lightProbePriority = render.nodeUniform53;
		

	}

	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + lightProbeIrradiance );
	nodeVar27 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar27 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst84 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst84 );
	let nodeConst85 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst86 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst84 * nodeConst85 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst86 ) * nodeConst85 ) ) ) * vec3<f32>( nodeConst86 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst87 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar28 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst87 ) );
	nodeVar29 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst87 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar28 );
	indirectDiffuse = ( indirectDiffuse + nodeVar29 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst88 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst88;

	// result

	output.color = nodeConst88;

	return output;

}
