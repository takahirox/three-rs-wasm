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
@binding( 1 ) @group( 1 ) var nodeUniform11_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform11 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform22_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform22 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform24_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform24 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform29_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform29 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform33_sampler : sampler_comparison;
@binding( 10 ) @group( 1 ) var nodeUniform33 : texture_depth_2d;

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
	nodeUniform12 : mat3x3<f32>,
	nodeUniform13 : f32,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform16 : f32,
	nodeUniform19 : mat4x4<f32>,
	nodeUniform21 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform30 : mat4x4<f32>,
	nodeUniform31 : f32,
	nodeUniform32 : f32,
	nodeUniform34 : f32,
	nodeUniform35 : vec2<f32>,
	nodeUniform36 : f32,
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
var<private> nodeVar0 : vec4<f32>;
var<private> AttenuationDistance : f32;
var<private> AttenuationColor : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar1 : vec2<f32>;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar11 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : vec4<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
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
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) nodeVarying7 : vec2<f32>,
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
	IOR = object.nodeUniform6;
	let nodeConst1 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeConst1 * nodeConst1 ) ) * object.nodeUniform7 ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform8 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform8, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	Transmission = object.nodeUniform9;
	nodeVar0 = textureSample( nodeUniform11, nodeUniform11_sampler, ( object.nodeUniform12 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	Thickness = ( object.nodeUniform10 * nodeVar0.y );
	AttenuationDistance = object.nodeUniform13;
	AttenuationColor = object.nodeUniform14;
	EmissiveColor = ( object.nodeUniform15 * vec3<f32>( object.nodeUniform16 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst2 = normalize( ( render.cameraPosition - v_positionWorld ) );
	let nodeConst3 = fn124( normalWorld, nodeConst2, Thickness, IOR, object.nodeUniform21 );
	let nodeConst4 = ( render.cameraProjectionMatrix * ( render.cameraViewMatrix * vec4<f32>( ( v_positionWorld + nodeConst3 ), 1.0 ) ) );
	nodeVar1 = ( nodeConst4.xy / vec2<f32>( nodeConst4.w ) );
	nodeVar1 = ( nodeVar1 + vec2<f32>( 1.0 ) );
	nodeVar1 = ( nodeVar1 / vec2<f32>( 2.0 ) );
	nodeVar1 = vec2<f32>( nodeVar1.x, ( 1.0 - nodeVar1.y ) );
	nodeVar2 = textureSample( nodeUniform22, nodeUniform22_sampler, vec2<f32>( Roughness, clamp( dot( normalWorld, nodeConst2 ), 0.0, 1.0 ) ) ).xy;
	let nodeConst5 = ( DiffuseContribution * fn126( length( nodeConst3 ), AttenuationColor, AttenuationDistance ) );
	let cameraViewport = vec4<f32>( 0.0, 0.0, render.nodeUniform23.x, render.nodeUniform23.y );
	let nodeConst6 = ( ( ( nodeVar1 * cameraViewport.zw ) + cameraViewport.xy ) / render.nodeUniform23 );
	let nodeConst7 = ( log2( cameraViewport.z ) * fn125( Roughness, IOR ) );
	let nodeConst8 = vec4<f32>( vec2<f32>( textureDimensions( nodeUniform24, i32( nodeConst7 ) ) ), vec2<f32>( textureDimensions( nodeUniform24, i32( ( nodeConst7 + 1.0 ) ) ) ) );
	let nodeConst9 = ( ( vec4<f32>( vec3<f32>( nodeConst6, 0.0 ), 1.0 ).xyxy * nodeConst8 ) + vec4<f32>( 0.5 ) );
	let nodeConst10 = fract( nodeConst9 );
	let nodeConst11 = ( vec4<f32>( 0.16666666666666666 ) * ( ( nodeConst10 * ( nodeConst10 * ( ( vec4<f32>( 3.0 ) * nodeConst10 ) - vec4<f32>( 6.0 ) ) ) ) + vec4<f32>( 4.0 ) ) );
	let nodeConst12 = ( ( vec4<f32>( 0.16666666666666666 ) * ( ( nodeConst10 * ( ( nodeConst10 * ( ( - nodeConst10 ) + vec4<f32>( 3.0 ) ) ) - vec4<f32>( 3.0 ) ) ) + vec4<f32>( 1.0 ) ) ) + nodeConst11 );
	let nodeConst13 = floor( nodeConst9 );
	let nodeConst14 = ( vec4<f32>( 1.0 ) / nodeConst8 );
	let nodeConst15 = ( ( ( nodeConst13 + ( vec4<f32>( -1.0 ) + ( nodeConst11 / nodeConst12 ) ) ) - vec4<f32>( 0.5 ) ) * nodeConst14 );
	let nodeConst16 = floor( nodeConst7 );
	nodeVar3 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, nodeConst15.xy, nodeConst16 );
	let nodeConst17 = ( vec4<f32>( 0.16666666666666666 ) * pow( nodeConst10, vec4<f32>( 3.0 ) ) );
	let nodeConst18 = ( ( vec4<f32>( 0.16666666666666666 ) * ( ( nodeConst10 * ( ( nodeConst10 * ( ( vec4<f32>( -3.0 ) * nodeConst10 ) + vec4<f32>( 3.0 ) ) ) + vec4<f32>( 3.0 ) ) ) + vec4<f32>( 1.0 ) ) ) + nodeConst17 );
	let nodeConst19 = ( ( ( nodeConst13 + ( vec4<f32>( 1.0 ) + ( nodeConst17 / nodeConst18 ) ) ) - vec4<f32>( 0.5 ) ) * nodeConst14 );
	nodeVar4 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, vec2<f32>( nodeConst19.xy.x, nodeConst15.xy.y ), nodeConst16 );
	nodeVar5 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, vec2<f32>( nodeConst15.xy.x, nodeConst19.xy.y ), nodeConst16 );
	nodeVar6 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, nodeConst19.xy, nodeConst16 );
	let nodeConst20 = ceil( nodeConst7 );
	nodeVar7 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, nodeConst15.zw, nodeConst20 );
	nodeVar8 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, vec2<f32>( nodeConst19.zw.x, nodeConst15.zw.y ), nodeConst20 );
	nodeVar9 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, vec2<f32>( nodeConst15.zw.x, nodeConst19.zw.y ), nodeConst20 );
	nodeVar10 = textureSampleLevel( nodeUniform24, nodeUniform24_sampler, nodeConst19.zw, nodeConst20 );
	let nodeConst21 = mix( ( ( vec4<f32>( nodeConst12.xy.y ) * ( ( vec4<f32>( nodeConst12.xy.x ) * nodeVar3 ) + ( vec4<f32>( nodeConst18.xy.x ) * nodeVar4 ) ) ) + ( vec4<f32>( nodeConst18.xy.y ) * ( ( vec4<f32>( nodeConst12.xy.x ) * nodeVar5 ) + ( vec4<f32>( nodeConst18.xy.x ) * nodeVar6 ) ) ) ), ( ( vec4<f32>( nodeConst12.zw.y ) * ( ( vec4<f32>( nodeConst12.zw.x ) * nodeVar7 ) + ( vec4<f32>( nodeConst18.zw.x ) * nodeVar8 ) ) ) + ( vec4<f32>( nodeConst18.zw.y ) * ( ( vec4<f32>( nodeConst12.zw.x ) * nodeVar9 ) + ( vec4<f32>( nodeConst18.zw.x ) * nodeVar10 ) ) ) ), fract( nodeConst7 ) );
	let nodeConst22 = vec4<f32>( ( ( vec3<f32>( 1.0 ) - ( ( SpecularColorBlended * vec3<f32>( nodeVar2.x ) ) + vec3<f32>( ( SpecularF90 * nodeVar2.y ) ) ) ) * ( nodeConst5 * nodeConst21.xyz ) ), ( 1.0 - ( ( 1.0 - nodeConst21.w ) * ( ( ( nodeConst5.x + nodeConst5.y ) + nodeConst5.z ) / 3.0 ) ) ) );
	DiffuseColor.w = ( DiffuseColor.w * mix( 1.0, nodeConst22.w, Transmission ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar11 = textureSample( nodeUniform22, nodeUniform22_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar11;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst23 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst23 );
	let nodeConst24 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst25 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst23 * nodeConst24 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst25 ) * nodeConst24 ) ) ) * vec3<f32>( nodeConst25 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + render.nodeUniform25 );
	let nodeConst26 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform26 - render.nodeUniform27 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	let nodeConst27 = ( render.nodeUniform30 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform31 ) ) ), 1.0 ) );
	let nodeConst28 = ( nodeConst27.xyz / vec3<f32>( nodeConst27.w ) );
	let nodeConst29 = vec3<f32>( nodeConst28.x, ( 1.0 - nodeConst28.y ), ( nodeConst28.z + render.nodeUniform32 ) );
	nodeVar12 = textureSample( nodeUniform29, nodeUniform29_sampler, nodeConst29.xy );

	if ( ( ( ( ( ( nodeConst29.x >= 0.0 ) && ( nodeConst29.x <= 1.0 ) ) && ( nodeConst29.y >= 0.0 ) ) && ( nodeConst29.y <= 1.0 ) ) && ( nodeConst29.z <= 1.0 ) ) ) {

		let nodeConst30 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst31 = ( vec2<f32>( render.nodeUniform34 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform35 ) );
		let nodeConst32 = ( nodeConst29.xy + ( fn25( 0, 5, nodeConst30 ) * nodeConst31 ) );
		nodeVar14 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst32, nodeConst29.z );
		let nodeConst33 = ( nodeConst29.xy + ( fn25( 1, 5, nodeConst30 ) * nodeConst31 ) );
		nodeVar15 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst33, nodeConst29.z );
		let nodeConst34 = ( nodeConst29.xy + ( fn25( 2, 5, nodeConst30 ) * nodeConst31 ) );
		nodeVar16 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst34, nodeConst29.z );
		let nodeConst35 = ( nodeConst29.xy + ( fn25( 3, 5, nodeConst30 ) * nodeConst31 ) );
		nodeVar17 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst35, nodeConst29.z );
		let nodeConst36 = ( nodeConst29.xy + ( fn25( 4, 5, nodeConst30 ) * nodeConst31 ) );
		nodeVar18 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst36, nodeConst29.z );
		nodeVar13 = ( ( ( ( ( nodeVar14 + nodeVar15 ) + nodeVar16 ) + nodeVar17 ) + nodeVar18 ) * 0.2 );

	} else {

		nodeVar13 = 1.0;

	}

	nodeVar19 = mix( vec4<f32>( 1.0 ), mix( nodeVar12, vec4<f32>( 1.0 ), vec4<f32>( nodeVar13 ) ), ( render.nodeUniform36 * nodeVar12.w ) );
	nodeVar20 = ( vec4<f32>( clamp( dot( normalView, nodeConst26 ), 0.0, 1.0 ) ) * ( vec4<f32>( render.nodeUniform28, 1.0 ) * nodeVar19 ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst37 = clamp( dot( positionViewDirection, normalize( ( nodeConst26 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst38 = exp2( ( ( ( nodeConst37 * -5.55473 ) - 6.98316 ) * nodeConst37 ) );
	directDiffuse = ( vec4<f32>( directDiffuse, 1.0 ) + ( ( nodeVar20 * vec4<f32>( ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ), 1.0 ) ) * vec4<f32>( ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst38 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst38 ) ) ) ), 1.0 ) ) ).xyz;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst39 = normalize( ( nodeConst26 + positionViewDirection ) );
	let nodeConst40 = clamp( dot( positionViewDirection, nodeConst39 ), 0.0, 1.0 );
	let nodeConst41 = exp2( ( ( ( nodeConst40 * -5.55473 ) - 6.98316 ) * nodeConst40 ) );
	let nodeConst42 = max( Roughness, 0.045 );
	let nodeConst43 = ( nodeConst42 * nodeConst42 );
	directSpecular = ( vec4<f32>( directSpecular, 1.0 ) + ( ( nodeVar20 * vec4<f32>( ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst41 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst41 ) ) ) * vec3<f32>( fn110( nodeConst43, clamp( dot( normalView, nodeConst26 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst43, clamp( dot( normalView, nodeConst39 ), 0.0, 1.0 ) ) ) ), 1.0 ) ) * vec4<f32>( multiScatteringCompensation, 1.0 ) ) ).xyz;
	nodeVar21 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar21 );
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
	nodeVar22 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst47 ) );
	nodeVar23 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst47 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar22 );
	indirectDiffuse = ( indirectDiffuse + nodeVar23 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = mix( vec4<f32>( ( directDiffuse + indirectDiffuse ), 1.0 ), nodeConst22, Transmission ).xyz;
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst48 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst48;

	// result

	output.color = nodeConst48;

	return output;

}
