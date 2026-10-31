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

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform1 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Output : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_positionWorld : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	let nodeConst0 = fwidth( v_positionWorld.xz );
	let nodeConst1 = max( nodeConst0.x, nodeConst0.y );
	let nodeConst2 = fract( v_positionWorld.xz );
	let nodeConst3 = abs( ( nodeConst2.x - 0.5 ) );
	let nodeConst4 = abs( ( nodeConst2.y - 0.5 ) );
	DiffuseColor = ( mix( vec4<f32>( 1.0, 1.0, 1.0, 0.0 ), vec4<f32>( 0.5, 0.5, 0.5, 1.0 ), max( smoothstep( ( 0.03 + nodeConst1 ), ( 0.03 - nodeConst1 ), max( nodeConst3, nodeConst4 ) ), max( clamp( ( ( ( 0.007 - nodeConst3 ) / nodeConst0.x ) + 0.5 ), 0.0, 1.0 ), clamp( ( ( ( 0.007 - nodeConst4 ) / nodeConst0.y ) + 0.5 ), 0.0, 1.0 ) ) ) ) * vec4<f32>( smoothstep( 30.0, ( 30.0 - 20.0 ), length( v_positionWorld ) ) ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	let nodeConst5 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst5;

	// result

	output.color = nodeConst5;

	return output;

}
