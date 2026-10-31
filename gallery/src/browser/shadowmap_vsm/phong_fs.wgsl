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
@binding( 1 ) @group( 1 ) var nodeUniform16_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform16 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform31_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform31 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform10 : mat4x4<f32>,
	nodeUniform17 : mat3x3<f32>,
	nodeUniform32 : mat3x3<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform19 : f32,
	nodeUniform20 : f32,
	nodeUniform23 : f32,
	nodeUniform24 : f32,
	nodeUniform11 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform18 : f32,
	nodeUniform28 : mat4x4<f32>,
	nodeUniform29 : f32,
	nodeUniform30 : f32,
	nodeUniform33 : f32,
	nodeUniform34 : vec3<f32>,
	nodeUniform35 : f32,
	nodeUniform36 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Shininess : f32;
var<private> SpecularColor : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar0 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : f32;
var<private> directSpecular : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : vec2<f32>;
var<private> nodeVar7 : f32;
var<private> indirectDiffuse : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar8 : vec4<f32>;

// codes
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




@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_normalViewGeometry : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Shininess = max( object.nodeUniform2, 0.0001 );
	SpecularColor = object.nodeUniform3;
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + render.nodeUniform6 );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	let nodeConst0 = ( render.nodeUniform9 - v_positionView );
	let nodeConst1 = normalize( nodeConst0 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst2 = ( render.nodeUniform12 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform14 ) ) ), 1.0 ) );
	let nodeConst3 = ( nodeConst2.xyz / vec3<f32>( nodeConst2.w ) );
	let nodeConst4 = vec3<f32>( nodeConst3.x, ( 1.0 - nodeConst3.y ), ( nodeConst3.z + render.nodeUniform15 ) );

	if ( ( ( ( ( ( nodeConst4.x >= 0.0 ) && ( nodeConst4.x <= 1.0 ) ) && ( nodeConst4.y >= 0.0 ) ) && ( nodeConst4.y <= 1.0 ) ) && ( nodeConst4.z <= 1.0 ) ) ) {

		nodeVar1 = 1.0;
		nodeVar2 = textureSample( nodeUniform16, nodeUniform16_sampler, ( object.nodeUniform17 * vec3<f32>( nodeConst4.xy, 1.0 ) ).xy ).xy;
		let nodeConst5 = step( nodeConst4.z, nodeVar2.x );

		if ( ( nodeConst5 != 1.0 ) ) {

			let nodeConst6 = max( 1e-7, ( nodeVar2.y * nodeVar2.y ) );
			let nodeConst7 = ( nodeConst4.z - nodeVar2.x );
			nodeVar1 = max( nodeConst5, clamp( ( ( ( nodeConst6 / ( nodeConst6 + ( nodeConst7 * nodeConst7 ) ) ) - 0.3 ) / 0.65 ), 0.0, 1.0 ) );
			

		}

		nodeVar0 = nodeVar1;

	} else {

		nodeVar0 = 1.0;

	}

	nodeVar3 = mix( 1.0, nodeVar0, render.nodeUniform18 );
	let nodeConst8 = ( vec3<f32>( clamp( dot( normalView, nodeConst1 ), 0.0, 1.0 ) ) * ( ( ( render.nodeUniform11 * vec3<f32>( nodeVar3 ) ) * vec3<f32>( smoothstep( render.nodeUniform19, render.nodeUniform20, dot( nodeConst1, normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform21 - render.nodeUniform22 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( fn44( length( nodeConst0 ), render.nodeUniform23, render.nodeUniform24 ) ) ) );
	directDiffuse = ( directDiffuse + ( nodeConst8 * ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	positionViewDirection = normalize( v_positionViewDirection );
	let nodeConst9 = normalize( ( nodeConst1 + positionViewDirection ) );
	let nodeConst10 = clamp( dot( positionViewDirection, nodeConst9 ), 0.0, 1.0 );
	let nodeConst11 = exp2( ( ( ( nodeConst10 * -5.55473 ) - 6.98316 ) * nodeConst10 ) );
	directSpecular = ( directSpecular + ( ( nodeConst8 * ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst11 ) ) ) + vec3<f32>( ( 1.0 * nodeConst11 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeConst9 ), 0.0, 1.0 ), Shininess ) ) ) ) ) * vec3<f32>( 1.0 ) ) );
	let nodeConst12 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform25 - render.nodeUniform26 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	let nodeConst13 = ( render.nodeUniform28 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform29 ) ) ), 1.0 ) );
	let nodeConst14 = ( nodeConst13.xyz / vec3<f32>( nodeConst13.w ) );
	let nodeConst15 = vec3<f32>( nodeConst14.x, ( 1.0 - nodeConst14.y ), ( nodeConst14.z + render.nodeUniform30 ) );

	if ( ( ( ( ( ( nodeConst15.x >= 0.0 ) && ( nodeConst15.x <= 1.0 ) ) && ( nodeConst15.y >= 0.0 ) ) && ( nodeConst15.y <= 1.0 ) ) && ( nodeConst15.z <= 1.0 ) ) ) {

		nodeVar5 = 1.0;
		nodeVar6 = textureSample( nodeUniform31, nodeUniform31_sampler, ( object.nodeUniform32 * vec3<f32>( nodeConst15.xy, 1.0 ) ).xy ).xy;
		let nodeConst16 = step( nodeConst15.z, nodeVar6.x );

		if ( ( nodeConst16 != 1.0 ) ) {

			let nodeConst17 = max( 1e-7, ( nodeVar6.y * nodeVar6.y ) );
			let nodeConst18 = ( nodeConst15.z - nodeVar6.x );
			nodeVar5 = max( nodeConst16, clamp( ( ( ( nodeConst17 / ( nodeConst17 + ( nodeConst18 * nodeConst18 ) ) ) - 0.3 ) / 0.65 ), 0.0, 1.0 ) );
			

		}

		nodeVar4 = nodeVar5;

	} else {

		nodeVar4 = 1.0;

	}

	nodeVar7 = mix( 1.0, nodeVar4, render.nodeUniform33 );
	let nodeConst19 = ( vec3<f32>( clamp( dot( normalView, nodeConst12 ), 0.0, 1.0 ) ) * ( render.nodeUniform27 * vec3<f32>( nodeVar7 ) ) );
	directDiffuse = ( directDiffuse + ( nodeConst19 * ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) ) ) );
	let nodeConst20 = normalize( ( nodeConst12 + positionViewDirection ) );
	let nodeConst21 = clamp( dot( positionViewDirection, nodeConst20 ), 0.0, 1.0 );
	let nodeConst22 = exp2( ( ( ( nodeConst21 * -5.55473 ) - 6.98316 ) * nodeConst21 ) );
	directSpecular = ( directSpecular + ( ( nodeConst19 * ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst22 ) ) ) + vec3<f32>( ( 1.0 * nodeConst22 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeConst20 ), 0.0, 1.0 ), Shininess ) ) ) ) ) * vec3<f32>( 1.0 ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( vec4<f32>( indirectDiffuse, 1.0 ) + ( vec4<f32>( irradiance, 1.0 ) * ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) ) ) ).xyz;
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar8 = vec4<f32>( mix( Output.xyz, render.nodeUniform34, smoothstep( render.nodeUniform35, render.nodeUniform36, ( - v_positionView.z ) ) ), Output.w );
	let nodeConst23 = nodeVar8;
	Output = nodeConst23;

	// result

	output.color = nodeConst23;

	return output;

}
