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

	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), ( 1.0 * vec4<f32>( mix( mix( vec3<f32>( 0.8879231178794776, 0.8879231178794776, 0.8879231178794776 ), vec3<f32>( 0.023153366173251363, 0.3813260114221238, 0.4793201830913402 ), ( smoothstep( 0.48, 0.5, nodeVarying4.x ) - smoothstep( 0.6, 0.62, nodeVarying4.x ) ) ), vec3<f32>( 0.6307571363387763, 0.08865558627723595, 0.04231141061442144 ), ( smoothstep( 0.66, 0.68, nodeVarying4.x ) - smoothstep( 0.72, 0.74, nodeVarying4.x ) ) ), 1.0 ).w ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	nodeVar0 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar0;

	// result

	output.color = nodeVar0;

	return output;

}
