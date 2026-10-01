// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms

struct objectStruct {
	intensity : f32,
	radius : f32,
	hardness : f32,
	nodeUniform5 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : f32;
var<private> Output : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) nodeVarying4 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = pow( ( 1.0 - clamp( ( length( ( nodeVarying4 - vec2<f32>( 0.5 ) ) ) / object.radius ), 0.0, 1.0 ) ), ( ( object.hardness * 8.0 ) + 1.0 ) );
	DiffuseColor = vec4<f32>( ( object.intensity * nodeVar0 ) );
	DiffuseColor.w = ( DiffuseColor.w * nodeVar0 );
	nodeVar1 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar1;

	// result

	output.color = nodeVar1;

	return output;

}
