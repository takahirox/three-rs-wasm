// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms

struct renderStruct {
	nodeUniform1 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform0 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform2 : f32,
	nodeUniform5 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;

// codes


@fragment
fn main( @builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = ( vec4<f32>( ( mix( vec3<f32>( 0.006512090790025684, 0.008568125615105716, 0.024157632443547246 ), vec3<f32>( 0.030713443727452196, 0.008023192982520563, 0.06662593863608139 ), ( fragCoord.xy / render.nodeUniform0 ).x ) + ( vec3<f32>( ( 1.0 - distance( ( fragCoord.xy / render.nodeUniform0 ), vec2<f32>( 0.5, 1.0 ) ) ) ) * vec3<f32>( 0.0036765073221525194, 0.10946171076915331, 0.13843161502267545 ) ) ), 1.0 ) * vec4<f32>( render.nodeUniform1 ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform2 );
	DiffuseColor.w = 1.0;
	nodeVar0 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar0;

	// result

	output.color = nodeVar0;

	return output;

}
