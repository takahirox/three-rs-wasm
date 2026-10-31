// Three.js r187 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct nodeUniform0Struct {
	value : array< vec4<f32> >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> nodeUniform0 : nodeUniform0Struct;

struct objectStruct {
	nodeUniform1 : u32
};
@binding( 1 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec3<f32>;

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

	if ( instanceIndex >= object.nodeUniform1 ) { return; }

	nodeVar0 = vec3<f32>( 10000.0, 10000.0, 10000.0 );
	nodeUniform0.value[ instanceIndex ].x = nodeVar0[ 0 ];
	nodeUniform0.value[ instanceIndex ].y = nodeVar0[ 1 ];
	nodeUniform0.value[ instanceIndex ].z = nodeVar0[ 2 ];
	nodeUniform0.value[ instanceIndex ].w = vec3<f32>( -1.0, -1.0, -1.0 ).x;

	

}
