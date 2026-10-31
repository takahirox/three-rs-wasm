// Three.js r187 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms
@binding( 0 ) @group( 0 ) var nodeUniform0 : texture_depth_2d;

struct nodeUniform1Struct {
	value : array< f32 >
};
@binding( 1 ) @group( 0 )
var<storage, read_write> nodeUniform1 : nodeUniform1Struct;

struct objectStruct {
	nodeUniform2 : u32
};
@binding( 2 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;

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


	// flow -> MeshletDrawingPipeline HZB
	if ( instanceIndex >= object.nodeUniform2 ) { return; }


	if ( ( instanceIndex < 65536u ) ) {

		nodeVar0 = 0.0;
		let nodeConst0 = ( instanceIndex % 256u );
		let nodeConst1 = ( instanceIndex / 256u );
		nodeVar1 = textureLoad( nodeUniform0, vec2<i32>( i32( min( ( ( nodeConst0 * 2u ) + 0u ), 511u ) ), i32( min( ( ( nodeConst1 * 2u ) + 0u ), 511u ) ) ), u32( 0u ) );
		nodeVar0 = max( nodeVar0, nodeVar1 );
		nodeVar2 = textureLoad( nodeUniform0, vec2<i32>( i32( min( ( ( nodeConst0 * 2u ) + 1u ), 511u ) ), i32( min( ( ( nodeConst1 * 2u ) + 0u ), 511u ) ) ), u32( 0u ) );
		nodeVar0 = max( nodeVar0, nodeVar2 );
		nodeVar3 = textureLoad( nodeUniform0, vec2<i32>( i32( min( ( ( nodeConst0 * 2u ) + 0u ), 511u ) ), i32( min( ( ( nodeConst1 * 2u ) + 1u ), 511u ) ) ), u32( 0u ) );
		nodeVar0 = max( nodeVar0, nodeVar3 );
		nodeVar4 = textureLoad( nodeUniform0, vec2<i32>( i32( min( ( ( nodeConst0 * 2u ) + 1u ), 511u ) ), i32( min( ( ( nodeConst1 * 2u ) + 1u ), 511u ) ) ), u32( 0u ) );
		nodeVar0 = max( nodeVar0, nodeVar4 );
		nodeUniform1.value[ ( instanceIndex + 0u ) ] = nodeVar0;
		

	}


	

}
