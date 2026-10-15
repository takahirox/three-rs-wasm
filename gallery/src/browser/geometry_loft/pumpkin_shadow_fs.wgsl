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
	nodeUniform0 : f32,
	nodeUniform3 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) nodeVarying4 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), ( 1.0 * vec4<f32>( mix( mix( vec3<f32>( 0.33245153633549385, 0.07818742179702069, 0.008023192982520563 ), vec3<f32>( 0.7912979403281551, 0.28314874041918053, 0.04666508633021928 ), pow( abs( cos( ( nodeVarying4.y * 21.991148575128552 ) ) ), 0.35 ) ), vec3<f32>( 0.2541520943200296, 0.19461783043107173, 0.02732089163382382 ), ( smoothstep( 0.88, 1.0, nodeVarying4.x ) * 0.6 ) ), 1.0 ).w ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	nodeVar0 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar0;

	// result

	output.color = nodeVar0;

	return output;

}
