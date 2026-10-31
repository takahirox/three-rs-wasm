// Three.js r187 - Node System

// global
diagnostic( off, derivative_uniformity );


// directives


// structs

struct OutputType {
	@location( 0 ) m0 : vec4<f32>,
	@location( 1 ) m1 : vec4<f32>,
	@location( 2 ) m2 : vec4<f32>,
	@location( 3 ) m3 : vec4<f32>,
	
};
var<private> output : OutputType;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform20_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform20 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform35_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform35 : texture_depth_cube;

struct objectStruct {
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : f32,
	nodeUniform11 : f32,
	nodeUniform12 : f32,
	nodeUniform14 : mat3x3<f32>,
	nodeUniform15 : f32,
	nodeUniform16 : vec3<f32>,
	nodeUniform17 : f32,
	nodeUniform18 : vec3<f32>,
	nodeUniform19 : f32,
	nodeUniform21 : mat4x4<f32>,
	nodeUniform40 : mat4x4<f32>,
	nodeUniform43 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform39 : mat4x4<f32>,
	nodeUniform41 : mat4x4<f32>,
	nodeUniform42 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform37 : f32,
	nodeUniform38 : f32,
	nodeUniform22 : vec3<f32>,
	nodeUniform24 : mat4x4<f32>,
	nodeUniform26 : f32,
	nodeUniform36 : f32,
	nodeUniform28 : f32,
	nodeUniform27 : f32,
	nodeUniform29 : f32,
	nodeUniform30 : f32,
	nodeUniform31 : vec2<f32>,
	nodeUniform32 : f32,
	nodeUniform33 : f32,
	nodeUniform34 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> IOR : f32;
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
var<private> irradiance : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;

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
fn main( @location( 0 ) positionLocal : vec3<f32>,
	@location( 1 ) positionPrevious : vec3<f32>,
	@location( 2 ) v_positionView : vec3<f32>,
	@location( 3 ) v_normalViewGeometry : vec3<f32>,
	@location( 4 ) v_positionViewDirection : vec3<f32>,
	@location( 5 ) v_positionWorld : vec3<f32>,
	@location( 6 ) vInstanceColor : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputType {

	// flow
	// code

	DiffuseColor = vec4<f32>( ( object.nodeUniform9 * vInstanceColor ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform10 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform11;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst1 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform12, 0.045 ) + max( max( nodeConst1.x, nodeConst1.y ), nodeConst1.z ) ), 1.0 );
	IOR = object.nodeUniform15;
	let nodeConst2 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeConst2 * nodeConst2 ) ) * object.nodeUniform16 ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform17 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform17, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform11 ) ) );
	EmissiveColor = ( object.nodeUniform18 * vec3<f32>( object.nodeUniform19 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar0 = textureSample( nodeUniform20, nodeUniform20_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar0;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst3 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst3 );
	let nodeConst4 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst5 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst3 * nodeConst4 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst5 ) * nodeConst4 ) ) ) * vec3<f32>( nodeConst5 ) ) );
	let nodeConst6 = ( render.nodeUniform22 - v_positionView );
	let nodeConst7 = normalize( nodeConst6 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst8 = ( render.nodeUniform24 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform26 ) ) ), 1.0 ) ).xyz;
	let nodeConst9 = abs( nodeConst8 );
	nodeVar1 = 1.0;
	let nodeConst10 = max( max( nodeConst9.x, nodeConst9.y ), nodeConst9.z );

	if ( ( ( ( nodeConst10 - render.nodeUniform27 ) <= 0.0 ) && ( ( nodeConst10 - render.nodeUniform28 ) >= 0.0 ) ) ) {

		let nodeConst11 = ( render.nodeUniform27 - render.nodeUniform28 );
		let nodeConst12 = ( ( render.nodeUniform27 / nodeConst11 ) + render.nodeUniform29 );
		let nodeConst13 = length( nodeConst8 );
		let nodeConst14 = ( - ( ( render.nodeUniform28 * render.nodeUniform27 ) / ( nodeConst11 * nodeConst13 ) ) );
		let nodeConst15 = ( render.nodeUniform30 / render.nodeUniform31.x );
		let nodeConst16 = normalize( nodeConst8 );
		let nodeConst17 = abs( nodeConst16 );

		if ( ( nodeConst17.x > nodeConst17.z ) ) {

			nodeVar2 = vec3<f32>( 0.0, 1.0, 0.0 );

		} else {

			nodeVar2 = vec3<f32>( 1.0, 0.0, 0.0 );

		}

		let nodeConst18 = normalize( cross( nodeConst16, nodeVar2 ) );
		let nodeConst19 = cross( nodeConst16, nodeConst18 );
		let nodeConst20 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst21 = ( render.nodeUniform32 - render.nodeUniform33 );
		let nodeConst22 = ( ( render.nodeUniform32 / nodeConst21 ) + render.nodeUniform34 );
		let nodeConst23 = ( - ( ( render.nodeUniform33 * render.nodeUniform32 ) / ( nodeConst21 * nodeConst13 ) ) );
		nodeVar3 = 0.0;

		for ( var i : i32 = 0; i < 5; i ++ ) {

			let nodeConst24 = fn25( i, 5, nodeConst20 );
			let nodeConst25 = ( nodeConst16 + ( ( ( nodeConst18 * vec3<f32>( nodeConst24.x ) ) + ( nodeConst19 * vec3<f32>( nodeConst24.y ) ) ) * vec3<f32>( nodeConst15 ) ) );
			let nodeConst26 = abs( nodeConst25 );
			let nodeConst27 = max( max( nodeConst26.x, nodeConst26.y ), nodeConst26.z );
			nodeVar4 = textureSampleCompare( nodeUniform35, nodeUniform35_sampler, vec3<f32>( ( - nodeConst25.x ), nodeConst25.yz ), ( nodeConst22 + ( nodeConst23 / nodeConst27 ) ) );
			nodeVar3 = ( nodeVar3 + nodeVar4 );

		}

		nodeVar1 = ( nodeVar3 * 0.2 );
		

	}

	nodeVar5 = mix( 1.0, nodeVar1, render.nodeUniform36 );
	nodeVar6 = ( vec3<f32>( clamp( dot( normalView, nodeConst7 ), 0.0, 1.0 ) ) * ( ( render.nodeUniform23 * vec3<f32>( nodeVar5 ) ) * vec3<f32>( fn44( length( nodeConst6 ), render.nodeUniform37, render.nodeUniform38 ) ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst28 = clamp( dot( positionViewDirection, normalize( ( nodeConst7 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst29 = exp2( ( ( ( nodeConst28 * -5.55473 ) - 6.98316 ) * nodeConst28 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar6 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst29 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst29 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst30 = normalize( ( nodeConst7 + positionViewDirection ) );
	let nodeConst31 = clamp( dot( positionViewDirection, nodeConst30 ), 0.0, 1.0 );
	let nodeConst32 = exp2( ( ( ( nodeConst31 * -5.55473 ) - 6.98316 ) * nodeConst31 ) );
	let nodeConst33 = max( Roughness, 0.045 );
	let nodeConst34 = ( nodeConst33 * nodeConst33 );
	directSpecular = ( directSpecular + ( ( nodeVar6 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst32 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst32 ) ) ) * vec3<f32>( fn110( nodeConst34, clamp( dot( normalView, nodeConst7 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst34, clamp( dot( normalView, nodeConst30 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar7 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar7 );
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
	nodeVar8 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst38 ) );
	nodeVar9 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst38 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar8 );
	indirectDiffuse = ( indirectDiffuse + nodeVar9 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	output.m0 = Output;
	output.m1 = DiffuseColor;
	let nodeConst39 = ( ( normalView * vec3<f32>( 0.5 ) ) + vec3<f32>( 0.5 ) );
	output.m2 = vec4<f32>( nodeConst39, 1.0 );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform40 );
	let nodeConst40 = ( ( render.nodeUniform39 * modelViewMatrix ) * vec4<f32>( positionLocal, 1.0 ) );
	let nodeConst41 = ( ( render.nodeUniform41 * ( render.nodeUniform42 * object.nodeUniform43 ) ) * vec4<f32>( positionPrevious, 1.0 ) );
	let nodeConst42 = ( ( nodeConst40.xy / vec2<f32>( nodeConst40.w ) ) - ( nodeConst41.xy / vec2<f32>( nodeConst41.w ) ) );
	output.m3 = vec4<f32>( vec3<f32>( nodeConst42, 0.0 ), 1.0 );

	// result

	return output;

}
