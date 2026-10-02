// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>,
	@builtin( frag_depth ) depth : f32
};
var<private> output : OutputStruct;

// uniforms

struct NodeBuffer_1005Struct {
	value : array< u32 >
};
@binding( 0 ) @group( 1 )
var<storage, read> NodeBuffer_1005 : NodeBuffer_1005Struct;

struct NodeBuffer_997Struct {
	value : array< u32 >
};
@binding( 1 ) @group( 1 )
var<storage, read> NodeBuffer_997 : NodeBuffer_997Struct;

struct NodeBuffer_1007Struct {
	value : array< u32 >
};
@binding( 2 ) @group( 1 )
var<storage, read> NodeBuffer_1007 : NodeBuffer_1007Struct;

struct renderStruct {
	nodeUniform1 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> nodeVar0 : u32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : u32;
var<private> nodeVar4 : u32;
var<private> nodeVar5 : u32;
var<private> nodeVar6 : u32;

// codes


@fragment
fn main( @builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = ( ( u32( ( render.nodeUniform1.y - fragCoord.xy.y ) ) * u32( render.nodeUniform1.x ) ) + u32( fragCoord.xy.x ) );
	nodeVar1 = ( f32( ( NodeBuffer_1005.value[ nodeVar0 ] >> 16u ) ) / 65535.0 );
	nodeVar2 = ( nodeVar1 * nodeVar1 );
	output.depth = ( 1.0 - ( nodeVar2 * nodeVar2 ) );

	if ( ( f32( ( NodeBuffer_1005.value[ nodeVar0 ] >> 16u ) ) == 0.0 ) ) {

		discard;
		

	}

	nodeVar3 = ( NodeBuffer_997.value[ ( NodeBuffer_1005.value[ nodeVar0 ] & 65535u ) ] + ( ( NodeBuffer_1007.value[ nodeVar0 ] & 131071u ) * 1000u ) );
	nodeVar4 = ( ( nodeVar3 * 747796405u ) + 289559509u );
	nodeVar5 = ( ( ( nodeVar4 >> 16u ) ^ nodeVar4 ) * 277803737u );
	nodeVar6 = ( ( nodeVar5 >> 16u ) ^ nodeVar5 );

	// result

	output.color = vec4<f32>( ( ( ( f32( ( nodeVar6 & 255u ) ) / 255.0 ) * 0.8 ) + 0.2 ), ( ( ( f32( ( ( nodeVar6 >> 8u ) & 255u ) ) / 255.0 ) * 0.8 ) + 0.2 ), ( ( ( f32( ( ( nodeVar6 >> 16u ) & 255u ) ) / 255.0 ) * 0.8 ) + 0.2 ), 1.0 );

	return output;

}
