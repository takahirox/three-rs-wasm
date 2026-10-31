// Three.js r187 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct nodeUniform0Struct {
	value : array< f32 >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> nodeUniform0 : nodeUniform0Struct;

struct objectStruct {
	nodeUniform1 : u32
};
@binding( 1 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : f32;

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
	if ( instanceIndex >= object.nodeUniform1 ) { return; }


	if ( ( instanceIndex < 16384u ) ) {

		nodeVar0 = 0.0;
		let nodeConst0 = ( instanceIndex / 128u );
		let nodeConst1 = ( instanceIndex % 128u );
		nodeVar0 = max( nodeVar0, nodeUniform0.value[ ( ( ( min( ( ( nodeConst0 * 2u ) + 0u ), 255u ) * 256u ) + min( ( ( nodeConst1 * 2u ) + 0u ), 255u ) ) + 0u ) ] );
		nodeVar0 = max( nodeVar0, nodeUniform0.value[ ( ( ( min( ( ( nodeConst0 * 2u ) + 0u ), 255u ) * 256u ) + min( ( ( nodeConst1 * 2u ) + 1u ), 255u ) ) + 0u ) ] );
		nodeVar0 = max( nodeVar0, nodeUniform0.value[ ( ( ( min( ( ( nodeConst0 * 2u ) + 1u ), 255u ) * 256u ) + min( ( ( nodeConst1 * 2u ) + 0u ), 255u ) ) + 0u ) ] );
		nodeVar0 = max( nodeVar0, nodeUniform0.value[ ( ( ( min( ( ( nodeConst0 * 2u ) + 1u ), 255u ) * 256u ) + min( ( ( nodeConst1 * 2u ) + 1u ), 255u ) ) + 0u ) ] );
		nodeUniform0.value[ ( instanceIndex + 65536u ) ] = nodeVar0;
		

	}


	

}
