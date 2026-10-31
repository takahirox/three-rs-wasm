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
@binding( 1 ) @group( 1 ) var nodeUniform12_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform12 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform14_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform14 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform15_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform15 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform1 : f32,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : vec3<f32>,
	nodeUniform6 : f32,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform11 : mat4x4<f32>,
	nodeUniform16 : mat4x4<f32>,
	nodeUniform18 : f32,
	nodeUniform19 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	cameraPosition : vec3<f32>,
	nodeUniform13 : vec2<f32>
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
var<private> nodeVar0 : f32;
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
var<private> radiance : vec3<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
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




@fragment
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( ( vec3<f32>( 1.0 ) * vec3<f32>( 0.8, 0.8, 0.8 ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	Metalness = object.nodeUniform1;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( 0.0, 0.045 ) + max( max( nodeConst0.x, nodeConst0.y ), nodeConst0.z ) ), 1.0 );
	IOR = 1.504;
	let nodeConst1 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeConst1 * nodeConst1 ) ) * vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform4 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform4, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform1 ) ) );
	Transmission = 1.0;

	if ( false ) {

		nodeVar0 = 0.0;

	} else {

		let nodeConst2 = 2.0;
		nodeVar0 = nodeConst2;

	}

	Thickness = nodeVar0;
	let nodeConst2 = 2.0;
	AttenuationDistance = nodeConst2;
	AttenuationColor = vec3<f32>( 0.83, 0.4, 0.04 );
	EmissiveColor = ( object.nodeUniform5 * vec3<f32>( object.nodeUniform6 ) );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
	normalView = NORMAL_normalView;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst3 = normalize( ( render.cameraPosition - v_positionWorld ) );
	let nodeConst4 = fn124( normalWorld, nodeConst3, Thickness, IOR, object.nodeUniform11 );
	let nodeConst5 = ( render.cameraProjectionMatrix * ( render.cameraViewMatrix * vec4<f32>( ( v_positionWorld + nodeConst4 ), 1.0 ) ) );
	nodeVar1 = ( nodeConst5.xy / vec2<f32>( nodeConst5.w ) );
	nodeVar1 = ( nodeVar1 + vec2<f32>( 1.0 ) );
	nodeVar1 = ( nodeVar1 / vec2<f32>( 2.0 ) );
	nodeVar1 = vec2<f32>( nodeVar1.x, ( 1.0 - nodeVar1.y ) );
	nodeVar2 = textureSample( nodeUniform12, nodeUniform12_sampler, vec2<f32>( Roughness, clamp( dot( normalWorld, nodeConst3 ), 0.0, 1.0 ) ) ).xy;
	let nodeConst6 = ( DiffuseContribution * fn126( length( nodeConst4 ), AttenuationColor, AttenuationDistance ) );
	let cameraViewport = vec4<f32>( 0.0, 0.0, render.nodeUniform13.x, render.nodeUniform13.y );
	let nodeConst7 = ( ( ( nodeVar1 * cameraViewport.zw ) + cameraViewport.xy ) / render.nodeUniform13 );
	let nodeConst8 = ( log2( cameraViewport.z ) * fn125( Roughness, IOR ) );
	let nodeConst9 = vec4<f32>( vec2<f32>( textureDimensions( nodeUniform14, i32( nodeConst8 ) ) ), vec2<f32>( textureDimensions( nodeUniform14, i32( ( nodeConst8 + 1.0 ) ) ) ) );
	let nodeConst10 = ( ( vec4<f32>( vec3<f32>( nodeConst7, 0.0 ), 1.0 ).xyxy * nodeConst9 ) + vec4<f32>( 0.5 ) );
	let nodeConst11 = fract( nodeConst10 );
	let nodeConst12 = ( vec4<f32>( 0.16666666666666666 ) * ( ( nodeConst11 * ( nodeConst11 * ( ( vec4<f32>( 3.0 ) * nodeConst11 ) - vec4<f32>( 6.0 ) ) ) ) + vec4<f32>( 4.0 ) ) );
	let nodeConst13 = ( ( vec4<f32>( 0.16666666666666666 ) * ( ( nodeConst11 * ( ( nodeConst11 * ( ( - nodeConst11 ) + vec4<f32>( 3.0 ) ) ) - vec4<f32>( 3.0 ) ) ) + vec4<f32>( 1.0 ) ) ) + nodeConst12 );
	let nodeConst14 = floor( nodeConst10 );
	let nodeConst15 = ( vec4<f32>( 1.0 ) / nodeConst9 );
	let nodeConst16 = ( ( ( nodeConst14 + ( vec4<f32>( -1.0 ) + ( nodeConst12 / nodeConst13 ) ) ) - vec4<f32>( 0.5 ) ) * nodeConst15 );
	let nodeConst17 = floor( nodeConst8 );
	nodeVar3 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, nodeConst16.xy, nodeConst17 );
	let nodeConst18 = ( vec4<f32>( 0.16666666666666666 ) * pow( nodeConst11, vec4<f32>( 3.0 ) ) );
	let nodeConst19 = ( ( vec4<f32>( 0.16666666666666666 ) * ( ( nodeConst11 * ( ( nodeConst11 * ( ( vec4<f32>( -3.0 ) * nodeConst11 ) + vec4<f32>( 3.0 ) ) ) + vec4<f32>( 3.0 ) ) ) + vec4<f32>( 1.0 ) ) ) + nodeConst18 );
	let nodeConst20 = ( ( ( nodeConst14 + ( vec4<f32>( 1.0 ) + ( nodeConst18 / nodeConst19 ) ) ) - vec4<f32>( 0.5 ) ) * nodeConst15 );
	nodeVar4 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, vec2<f32>( nodeConst20.xy.x, nodeConst16.xy.y ), nodeConst17 );
	nodeVar5 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, vec2<f32>( nodeConst16.xy.x, nodeConst20.xy.y ), nodeConst17 );
	nodeVar6 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, nodeConst20.xy, nodeConst17 );
	let nodeConst21 = ceil( nodeConst8 );
	nodeVar7 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, nodeConst16.zw, nodeConst21 );
	nodeVar8 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, vec2<f32>( nodeConst20.zw.x, nodeConst16.zw.y ), nodeConst21 );
	nodeVar9 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, vec2<f32>( nodeConst16.zw.x, nodeConst20.zw.y ), nodeConst21 );
	nodeVar10 = textureSampleLevel( nodeUniform14, nodeUniform14_sampler, nodeConst20.zw, nodeConst21 );
	let nodeConst22 = mix( ( ( vec4<f32>( nodeConst13.xy.y ) * ( ( vec4<f32>( nodeConst13.xy.x ) * nodeVar3 ) + ( vec4<f32>( nodeConst19.xy.x ) * nodeVar4 ) ) ) + ( vec4<f32>( nodeConst19.xy.y ) * ( ( vec4<f32>( nodeConst13.xy.x ) * nodeVar5 ) + ( vec4<f32>( nodeConst19.xy.x ) * nodeVar6 ) ) ) ), ( ( vec4<f32>( nodeConst13.zw.y ) * ( ( vec4<f32>( nodeConst13.zw.x ) * nodeVar7 ) + ( vec4<f32>( nodeConst19.zw.x ) * nodeVar8 ) ) ) + ( vec4<f32>( nodeConst19.zw.y ) * ( ( vec4<f32>( nodeConst13.zw.x ) * nodeVar9 ) + ( vec4<f32>( nodeConst19.zw.x ) * nodeVar10 ) ) ) ), fract( nodeConst8 ) );
	let nodeConst23 = vec4<f32>( ( ( vec3<f32>( 1.0 ) - ( ( SpecularColorBlended * vec3<f32>( nodeVar2.x ) ) + vec3<f32>( ( SpecularF90 * nodeVar2.y ) ) ) ) * ( nodeConst6 * nodeConst22.xyz ) ), ( 1.0 - ( ( 1.0 - nodeConst22.w ) * ( ( ( nodeConst6.x + nodeConst6.y ) + nodeConst6.z ) / 3.0 ) ) ) );
	DiffuseColor.w = ( DiffuseColor.w * mix( 1.0, nodeConst23.w, Transmission ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar11 = textureSample( nodeUniform12, nodeUniform12_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar11;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst24 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst24 );
	let nodeConst25 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst26 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst24 * nodeConst25 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst26 ) * nodeConst25 ) ) ) * vec3<f32>( nodeConst26 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst27 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst28 = ( object.nodeUniform16 * vec4<f32>( nodeConst27, 1.0 ) );
	let nodeConst29 = clamp( Roughness, 0.0, 1.0 );
	nodeVar12 = textureSampleLevel( nodeUniform15, nodeUniform15_sampler, vec3<f32>( ( - nodeConst28.x ), nodeConst28.yz ), ( ( object.nodeUniform18 * nodeConst29 ) * ( 2.0 - nodeConst29 ) ) );
	radiance = ( radiance + ( nodeVar12.xyz * vec3<f32>( object.nodeUniform19 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst30 = ( object.nodeUniform16 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst31 = clamp( 1.0, 0.0, 1.0 );
	nodeVar13 = textureSampleLevel( nodeUniform15, nodeUniform15_sampler, vec3<f32>( ( - nodeConst30.x ), nodeConst30.yz ), ( ( object.nodeUniform18 * nodeConst31 ) * ( 2.0 - nodeConst31 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar13.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform19 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar14 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar14 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst32 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst32 );
	let nodeConst33 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst34 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst32 * nodeConst33 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst34 ) * nodeConst33 ) ) ) * vec3<f32>( nodeConst34 ) ) );
	let nodeConst35 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar15 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst35 ) );
	nodeVar16 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst35 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar15 );
	indirectDiffuse = ( indirectDiffuse + nodeVar16 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	totalDiffuse = mix( vec4<f32>( ( directDiffuse + indirectDiffuse ), 1.0 ), nodeConst23, Transmission ).xyz;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst36 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst36;

	// result

	output.color = nodeConst36;

	return output;

}
