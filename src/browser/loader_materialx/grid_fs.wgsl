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
	nodeUniform0 : mat4x4<f32>,
	nodeUniform1 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec2<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> Output : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_positionWorld : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = fwidth( v_positionWorld.xz );
	nodeVar1 = max( nodeVar0.x, nodeVar0.y );
	nodeVar2 = fract( v_positionWorld.xz );
	nodeVar3 = abs( ( nodeVar2.x - 0.5 ) );
	nodeVar4 = abs( ( nodeVar2.y - 0.5 ) );
	DiffuseColor = ( mix( vec4<f32>( 1.0, 1.0, 1.0, 0.0 ), vec4<f32>( 0.5, 0.5, 0.5, 1.0 ), max( smoothstep( ( 0.03 + nodeVar1 ), ( 0.03 - nodeVar1 ), max( nodeVar3, nodeVar4 ) ), max( clamp( ( ( ( 0.007 - nodeVar3 ) / nodeVar0.x ) + 0.5 ), 0.0, 1.0 ), clamp( ( ( ( 0.007 - nodeVar4 ) / nodeVar0.y ) + 0.5 ), 0.0, 1.0 ) ) ) ) * vec4<f32>( smoothstep( 30.0, ( 30.0 - 20.0 ), length( v_positionWorld ) ) ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	nodeVar5 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar5;

	// result

	output.color = nodeVar5;

	return output;

}
