// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform1 : texture_depth_2d;

struct NodeBuffer_1002Struct {
	value : array< vec4<f32>, 16 >
};
@binding( 0 ) @group( 1 )
var<uniform> NodeBuffer_1002 : NodeBuffer_1002Struct;

struct NodeBuffer_1008Struct {
	value : array< f32 >
};
@binding( 3 ) @group( 1 )
var<storage, read_write> NodeBuffer_1008 : NodeBuffer_1008Struct;

struct objectStruct {
	nodeUniform2 : mat3x3<f32>,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : mat3x3<f32>,
	nodeUniform8 : u32
};
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform3 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> nodeVar0 : f32;
var<private> nodeVar1 : u32;
var<private> nodeVar2 : u32;
var<private> nodeVar3 : u32;
var<private> nodeVar4 : u32;
var<private> nodeVar5 : u32;
var<private> nodeVar6 : u32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;

// codes


@compute @workgroup_size( 64, 1, 1 )
fn main( @builtin( global_invocation_id ) globalId : vec3<u32>,
	@builtin( workgroup_id ) workgroupId : vec3<u32>,
	@builtin( local_invocation_id ) localId : vec3<u32>,
	@builtin( num_workgroups ) numWorkgroups : vec3<u32> ) {

	// local vars
	

	// system
	instanceIndex = globalId.x
		+ globalId.y * ( 64 * numWorkgroups.x )
		+ globalId.z * ( 64 * numWorkgroups.x ) * ( 1 * numWorkgroups.y );

	// flow
	// code


	// flow -> HZB Level 0
	if ( instanceIndex >= object.nodeUniform8 ) { return; }


	if ( ( instanceIndex < ( u32( NodeBuffer_1002.value[ 0u ].y ) * u32( NodeBuffer_1002.value[ 0u ].z ) ) ) ) {

		nodeVar0 = 0.0;
		nodeVar1 = ( instanceIndex % u32( NodeBuffer_1002.value[ 0u ].y ) );
		nodeVar2 = ( nodeVar1 * 2u );
		nodeVar3 = ( u32( render.nodeUniform3.x ) - 1u );
		nodeVar4 = ( instanceIndex / u32( NodeBuffer_1002.value[ 0u ].y ) );
		nodeVar5 = ( nodeVar4 * 2u );
		nodeVar6 = ( u32( render.nodeUniform3.y ) - 1u );
		nodeVar7 = textureLoad( nodeUniform1, vec2<i32>( ( object.nodeUniform2 * vec3<f32>( vec2<f32>( vec2<u32>( min( ( nodeVar2 + 0u ), nodeVar3 ), min( ( nodeVar5 + 0u ), nodeVar6 ) ) ), 1.0 ) ).xy ), u32( 0u ) );
		nodeVar0 = max( nodeVar0, nodeVar7 );
		nodeVar8 = textureLoad( nodeUniform1, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<u32>( min( ( nodeVar2 + 1u ), nodeVar3 ), min( ( nodeVar5 + 0u ), nodeVar6 ) ) ), 1.0 ) ).xy ), u32( 0u ) );
		nodeVar0 = max( nodeVar0, nodeVar8 );
		nodeVar9 = textureLoad( nodeUniform1, vec2<i32>( ( object.nodeUniform5 * vec3<f32>( vec2<f32>( vec2<u32>( min( ( nodeVar2 + 0u ), nodeVar3 ), min( ( nodeVar5 + 1u ), nodeVar6 ) ) ), 1.0 ) ).xy ), u32( 0u ) );
		nodeVar0 = max( nodeVar0, nodeVar9 );
		nodeVar10 = textureLoad( nodeUniform1, vec2<i32>( ( object.nodeUniform6 * vec3<f32>( vec2<f32>( vec2<u32>( min( ( nodeVar2 + 1u ), nodeVar3 ), min( ( nodeVar5 + 1u ), nodeVar6 ) ) ), 1.0 ) ).xy ), u32( 0u ) );
		nodeVar0 = max( nodeVar0, nodeVar10 );
		NodeBuffer_1008.value[ ( ( u32( NodeBuffer_1002.value[ 0u ].x ) + ( nodeVar4 * u32( NodeBuffer_1002.value[ 0u ].y ) ) ) + nodeVar1 ) ] = nodeVar0;
		

	}


	

}
