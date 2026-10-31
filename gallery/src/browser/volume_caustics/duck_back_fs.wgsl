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
@binding( 1 ) @group( 1 ) var nodeUniform13_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform13 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform22_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform22 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform24_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform24 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : f32,
	nodeUniform7 : vec3<f32>,
	nodeUniform8 : f32,
	nodeUniform9 : f32,
	nodeUniform10 : f32,
	nodeUniform11 : f32,
	nodeUniform12 : vec3<f32>,
	nodeUniform14 : mat4x4<f32>,
	nodeUniform15 : f32,
	nodeUniform16 : vec3<f32>,
	nodeUniform21 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform26 : f32,
	nodeUniform27 : f32,
	nodeUniform30 : f32,
	nodeUniform31 : f32,
	nodeUniform25 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform29 : vec3<f32>,
	nodeUniform17 : vec3<f32>,
	cameraPosition : vec3<f32>,
	nodeUniform23 : vec2<f32>
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
var<private> Transmission : f32;
var<private> Thickness : f32;
var<private> AttenuationDistance : f32;
var<private> AttenuationColor : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar4 : vec2<f32>;
var<private> nodeVar5 : vec2<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn124 ( n : vec3<f32>, v : vec3<f32>, thickness : f32, ior : f32, modelMatrix : mat4x4<f32> ) -> vec3<f32> {

	


	return ( normalize( refract( ( - v ), normalize( n ), ( 1.0 / ior ) ) ) * ( vec3<f32>( thickness ) * vec3<f32>( length( modelMatrix[ 0u ].xyz ), length( modelMatrix[ 1u ].xyz ), length( modelMatrix[ 2u ].xyz ) ) ) );

}


fn fn126 ( transmissionDistance : f32, attenuationColor : vec3<f32>, attenuationDistance : f32 ) -> vec3<f32> {

	


	if ( ( attenuationDistance != 0.0 ) ) {

		return exp( ( ( - ( ( - log( attenuationColor ) ) / vec3<f32>( attenuationDistance ) ) ) * vec3<f32>( transmissionDistance ) ) );

	}


	return vec3<f32>( 1.0, 1.0, 1.0 );

}


fn fn125 ( roughness : f32, ior : f32 ) -> f32 {

	


	return ( roughness * clamp( ( ( ior * 2.0 ) - 2.0 ), 0.0, 1.0 ) );

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
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	Metalness = object.nodeUniform2;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform3, 0.045 ) + max( max( nodeConst0.x, nodeConst0.y ), nodeConst0.z ) ), 1.0 );
	IOR = object.nodeUniform6;
	let nodeConst1 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeConst1 * nodeConst1 ) ) * object.nodeUniform7 ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform8 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform8, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	Transmission = object.nodeUniform9;
	Thickness = object.nodeUniform10;
	AttenuationDistance = object.nodeUniform11;
	AttenuationColor = object.nodeUniform12;
	positionViewDirection = normalize( v_positionViewDirection );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
	normalView = NORMAL_normalView;
	let nodeConst2 = ( normalize( refract( ( - positionViewDirection ), normalView, ( 1.0 / 1.5 ) ) ).xy * vec2<f32>( 0.6 ) );
	let nodeConst3 = ( pow( normalView.z, -0.9 ) * 0.004 );
	nodeVar0 = textureSample( nodeUniform13, nodeUniform13_sampler, ( nodeConst2 + vec2<f32>( ( - nodeConst3 ), 0.0 ) ) );
	nodeVar1 = textureSample( nodeUniform13, nodeUniform13_sampler, ( nodeConst2 + vec2<f32>( 0.0, ( - nodeConst3 ) ) ) );
	nodeVar2 = textureSample( nodeUniform13, nodeUniform13_sampler, ( nodeConst2 + vec2<f32>( nodeConst3, nodeConst3 ) ) );
	let nodeConst4 = pow( normalView.z, object.nodeUniform15 );
	nodeVar3 = ( ( ( vec3<f32>( nodeVar0.x, nodeVar1.y, nodeVar2.z ) * vec3<f32>( ( nodeConst4 * 60.0 ) ) ) + vec3<f32>( nodeConst4 ) ) * object.nodeUniform16 );
	EmissiveColor = ( ( nodeVar3 * vec3<f32>( ( pow( clamp( dot( positionViewDirection, ( - normalize( ( render.nodeUniform17 - v_positionView ) ) ) ), 0.0, 1.0 ), 3.0 ) + 0.1 ) ) ) * vec3<f32>( 0.02 ) );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst5 = normalize( ( render.cameraPosition - v_positionWorld ) );
	let nodeConst6 = fn124( normalWorld, nodeConst5, Thickness, IOR, object.nodeUniform21 );
	let nodeConst7 = ( render.cameraProjectionMatrix * ( render.cameraViewMatrix * vec4<f32>( ( v_positionWorld + nodeConst6 ), 1.0 ) ) );
	nodeVar4 = ( nodeConst7.xy / vec2<f32>( nodeConst7.w ) );
	nodeVar4 = ( nodeVar4 + vec2<f32>( 1.0 ) );
	nodeVar4 = ( nodeVar4 / vec2<f32>( 2.0 ) );
	nodeVar4 = vec2<f32>( nodeVar4.x, ( 1.0 - nodeVar4.y ) );
	nodeVar5 = textureSample( nodeUniform22, nodeUniform22_sampler, vec2<f32>( Roughness, clamp( dot( normalWorld, nodeConst5 ), 0.0, 1.0 ) ) ).xy;
	let nodeConst8 = ( DiffuseContribution * fn126( length( nodeConst6 ), AttenuationColor, AttenuationDistance ) );
	let cameraViewport = vec4<f32>( 0.0, 0.0, render.nodeUniform23.x, render.nodeUniform23.y );
	let nodeConst9 = ( ( ( nodeVar4 * cameraViewport.zw ) + cameraViewport.xy ) / render.nodeUniform23 );
	let nodeConst10 = ( log2( cameraViewport.z ) * fn125( Roughness, IOR ) );
	let nodeConst11 = vec4<f32>( vec2<f32>( textureDimensions( nodeUniform24, i32( nodeConst10 ) ) ), vec2<f32>( textureDimensions( nodeUniform24, i32( ( nodeConst10 + 1.0 ) ) ) ) );
	let nodeConst12 = ( ( vec4<f32>( vec3<f32>( nodeConst9, 0.0 ), 1.0 ).xyxy * nodeConst11 ) + vec4<f32>( 0.5 ) );
	let nodeConst13 = fract( nodeConst12 );
	let nodeConst14 = ( vec4<f32>( 0.16666666666666666 ) * ( ( nodeConst13 * ( nodeConst13 * ( ( vec4<f32>( 3.0 ) * nodeConst13 ) - vec4<f32>( 6.0 ) ) ) ) + vec4<f32>( 4.0 ) ) );
	let nodeConst15 = ( ( vec4<f32>( 0.16666666666666666 ) * ( ( nodeConst13 * ( ( nodeConst13 * ( ( - nodeConst13 ) + vec4<f32>( 3.0 ) ) ) - vec4<f32>( 3.0 ) ) ) + vec4<f32>( 1.0 ) ) ) + nodeConst14 );
	let nodeConst16 = floor( nodeConst12 );
	let nodeConst17 = ( vec4<f32>( 1.0 ) / nodeConst11 );
	let nodeConst18 = ( ( ( nodeConst16 + ( vec4<f32>( -1.0 ) + ( nodeConst14 / nodeConst15 ) ) ) - vec4<f32>( 0.5 ) ) * nodeConst17 );
	let nodeConst19 = floor( nodeConst10 );
	nodeVar6 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, nodeConst18.xy, nodeConst19 );
	let nodeConst20 = ( vec4<f32>( 0.16666666666666666 ) * pow( nodeConst13, vec4<f32>( 3.0 ) ) );
	let nodeConst21 = ( ( vec4<f32>( 0.16666666666666666 ) * ( ( nodeConst13 * ( ( nodeConst13 * ( ( vec4<f32>( -3.0 ) * nodeConst13 ) + vec4<f32>( 3.0 ) ) ) + vec4<f32>( 3.0 ) ) ) + vec4<f32>( 1.0 ) ) ) + nodeConst20 );
	let nodeConst22 = ( ( ( nodeConst16 + ( vec4<f32>( 1.0 ) + ( nodeConst20 / nodeConst21 ) ) ) - vec4<f32>( 0.5 ) ) * nodeConst17 );
	nodeVar7 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, vec2<f32>( nodeConst22.xy.x, nodeConst18.xy.y ), nodeConst19 );
	nodeVar8 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, vec2<f32>( nodeConst18.xy.x, nodeConst22.xy.y ), nodeConst19 );
	nodeVar9 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, nodeConst22.xy, nodeConst19 );
	let nodeConst23 = ceil( nodeConst10 );
	nodeVar10 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, nodeConst18.zw, nodeConst23 );
	nodeVar11 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, vec2<f32>( nodeConst22.zw.x, nodeConst18.zw.y ), nodeConst23 );
	nodeVar12 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, vec2<f32>( nodeConst18.zw.x, nodeConst22.zw.y ), nodeConst23 );
	nodeVar13 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, nodeConst22.zw, nodeConst23 );
	let nodeConst24 = mix( ( ( vec4<f32>( nodeConst15.xy.y ) * ( ( vec4<f32>( nodeConst15.xy.x ) * nodeVar6 ) + ( vec4<f32>( nodeConst21.xy.x ) * nodeVar7 ) ) ) + ( vec4<f32>( nodeConst21.xy.y ) * ( ( vec4<f32>( nodeConst15.xy.x ) * nodeVar8 ) + ( vec4<f32>( nodeConst21.xy.x ) * nodeVar9 ) ) ) ), ( ( vec4<f32>( nodeConst15.zw.y ) * ( ( vec4<f32>( nodeConst15.zw.x ) * nodeVar10 ) + ( vec4<f32>( nodeConst21.zw.x ) * nodeVar11 ) ) ) + ( vec4<f32>( nodeConst21.zw.y ) * ( ( vec4<f32>( nodeConst15.zw.x ) * nodeVar12 ) + ( vec4<f32>( nodeConst21.zw.x ) * nodeVar13 ) ) ) ), fract( nodeConst10 ) );
	let nodeConst25 = vec4<f32>( ( ( vec3<f32>( 1.0 ) - ( ( SpecularColorBlended * vec3<f32>( nodeVar5.x ) ) + vec3<f32>( ( SpecularF90 * nodeVar5.y ) ) ) ) * ( nodeConst8 * nodeConst24.xyz ) ), ( 1.0 - ( ( 1.0 - nodeConst24.w ) * ( ( ( nodeConst8.x + nodeConst8.y ) + nodeConst8.z ) / 3.0 ) ) ) );
	DiffuseColor.w = ( DiffuseColor.w * mix( 1.0, nodeConst25.w, Transmission ) );
	nodeVar14 = textureSample( nodeUniform22, nodeUniform22_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar14;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst26 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst26 );
	let nodeConst27 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst28 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst26 * nodeConst27 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst28 ) * nodeConst27 ) ) ) * vec3<f32>( nodeConst28 ) ) );
	let nodeConst29 = ( render.nodeUniform17 - v_positionView );
	let nodeConst30 = normalize( nodeConst29 );
	nodeVar15 = ( vec3<f32>( clamp( dot( normalView, nodeConst30 ), 0.0, 1.0 ) ) * ( ( render.nodeUniform25 * vec3<f32>( smoothstep( render.nodeUniform26, render.nodeUniform27, dot( nodeConst30, normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform28 - render.nodeUniform29 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( fn44( length( nodeConst29 ), render.nodeUniform30, render.nodeUniform31 ) ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst31 = clamp( dot( positionViewDirection, normalize( ( nodeConst30 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst32 = exp2( ( ( ( nodeConst31 * -5.55473 ) - 6.98316 ) * nodeConst31 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar15 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst32 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst32 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst33 = normalize( ( nodeConst30 + positionViewDirection ) );
	let nodeConst34 = clamp( dot( positionViewDirection, nodeConst33 ), 0.0, 1.0 );
	let nodeConst35 = exp2( ( ( ( nodeConst34 * -5.55473 ) - 6.98316 ) * nodeConst34 ) );
	let nodeConst36 = max( Roughness, 0.045 );
	let nodeConst37 = ( nodeConst36 * nodeConst36 );
	directSpecular = ( directSpecular + ( ( nodeVar15 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst35 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst35 ) ) ) * vec3<f32>( fn110( nodeConst37, clamp( dot( normalView, nodeConst30 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst37, clamp( dot( normalView, nodeConst33 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar16 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar16 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst38 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst38 );
	let nodeConst39 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst40 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst38 * nodeConst39 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst40 ) * nodeConst39 ) ) ) * vec3<f32>( nodeConst40 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst41 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar17 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst41 ) );
	nodeVar18 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst41 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar17 );
	indirectDiffuse = ( indirectDiffuse + nodeVar18 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = mix( vec4<f32>( ( directDiffuse + indirectDiffuse ), 1.0 ), nodeConst25, Transmission ).xyz;
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst42 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst42;

	// result

	output.color = nodeConst42;

	return output;

}
