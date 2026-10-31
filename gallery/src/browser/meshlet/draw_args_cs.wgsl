// Three.js r187 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct nodeUniform0Struct {
	value : array< u32 >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> nodeUniform0 : nodeUniform0Struct;

struct nodeUniform1Struct {
	value : array< u32 >
};
@binding( 1 ) @group( 0 )
var<storage, read> nodeUniform1 : nodeUniform1Struct;

struct nodeUniform2Struct {
	value : array< vec4<f32> >
};
@binding( 2 ) @group( 0 )
var<storage, read> nodeUniform2 : nodeUniform2Struct;

struct objectStruct {
	nodeUniform3 : u32,
	nodeUniform4 : u32
};
@binding( 3 ) @group( 0 )
var<uniform> object : objectStruct;

// vars


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


	// flow -> MeshletDrawingPipeline Draw Args
	if ( instanceIndex >= object.nodeUniform4 ) { return; }

	let nodeConst0 = ( instanceIndex * 4u );
	nodeUniform0.value[ nodeConst0 ] = 192u;
	nodeUniform0.value[ ( nodeConst0 + 1u ) ] = min( nodeUniform1.value[ ( instanceIndex + 1u ) ], bitcast<vec4<u32>>( nodeUniform2.value[ ( ( ( ( instanceIndex / 2u ) * 3u ) + 0u ) + object.nodeUniform3 ) ] ).w );
	nodeUniform0.value[ ( nodeConst0 + 2u ) ] = 0u;
	nodeUniform0.value[ ( nodeConst0 + 3u ) ] = 0u;

	

}
