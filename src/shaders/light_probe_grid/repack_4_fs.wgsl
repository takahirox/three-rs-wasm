// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 0 ) var nodeUniform1 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : i32,
	nodeUniform5 : mat3x3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar0 : i32;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;

// codes


@fragment
fn main( @builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	Output = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar0 = ( ( i32( fragCoord.xy.x ) + ( i32( fragCoord.xy.y ) * i32( object.nodeUniform3.x ) ) ) + ( ( object.nodeUniform4 * i32( object.nodeUniform3.x ) ) * i32( object.nodeUniform3.y ) ) );
	nodeVar1 = textureLoad( nodeUniform1, vec2<i32>( ( object.nodeUniform2 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 5.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar2 = textureLoad( nodeUniform1, vec2<i32>( ( object.nodeUniform5 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 6.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );

	// result

	output.color = vec4<f32>( nodeVar1.yz, nodeVar2.xy );

	return output;

}
