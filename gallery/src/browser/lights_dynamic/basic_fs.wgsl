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

struct nodeUniform6Struct {
	value : array< vec4<f32>, MAX_LIGHTS >
};
@binding( 1 ) @group( 0 )
var<uniform> nodeUniform6 : nodeUniform6Struct;

struct nodeUniform7Struct {
	value : array< vec4<f32>, MAX_LIGHTS >
};
@binding( 2 ) @group( 0 )
var<uniform> nodeUniform7 : nodeUniform7Struct;

struct nodeUniform8Struct {
	value : array< vec4<f32>, MAX_LIGHTS >
};
@binding( 3 ) @group( 0 )
var<uniform> nodeUniform8 : nodeUniform8Struct;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform4 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform2 : vec3<f32>,
	nodeUniform5 : i32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> dynPointDiffuse : vec3<f32>;
var<private> dynPointSpecular : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

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
fn main( @location( 0 ) v_positionView : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + render.nodeUniform2 );
	dynPointDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	dynPointSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	var nodeVar0 : vec3<f32> = v_positionView;

	for ( var i : i32 = 0; i < render.nodeUniform5; i ++ ) {

		nodeVar1 = ( nodeUniform6.value[ i ].xyz - nodeVar0 );
		nodeVar2 = normalize( nodeVar1 );
		nodeVar3 = ( nodeUniform7.value[ i ].xyz * vec3<f32>( fn44( length( nodeVar1 ), nodeUniform6.value[ i ].w, nodeUniform8.value[ i ].x ) ) );

	}

	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	directDiffuse = ( directDiffuse + dynPointDiffuse );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	directSpecular = ( directSpecular + dynPointSpecular );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = vec4<f32>( 0.0, 0.0, 0.0, 0.0 ).xyz;
	indirectDiffuse = ( vec4<f32>( indirectDiffuse, 1.0 ) + vec4<f32>( 1.0, 1.0, 1.0, 0.0 ) ).xyz;
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = ( indirectDiffuse * DiffuseColor.xyz );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst0 = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst0;

	// result

	output.color = nodeConst0;

	return output;

}
