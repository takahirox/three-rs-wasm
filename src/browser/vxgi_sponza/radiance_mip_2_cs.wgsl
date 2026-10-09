// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms
@binding( 0 ) @group( 0 ) var nodeUniform0 : texture_storage_3d<rgba16float, write>;
@binding( 1 ) @group( 0 ) var nodeUniform1 : texture_storage_3d<rgba16float, read>;

struct objectStruct {
	nodeUniform2 : u32
};
@binding( 2 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec4<f32>;

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


	// flow -> VXGI.RadianceMip2
	if ( instanceIndex >= object.nodeUniform2 ) { return; }


	if ( ( instanceIndex < 13824u ) ) {

		let nodeConst0 = vec3<u32>( ( instanceIndex % 36u ), ( ( instanceIndex / 36u ) % 16u ), ( instanceIndex / 576u ) );
		let nodeConst1 = vec3<i32>( ( nodeConst0 * vec3<u32>( 2u ) ) );
		nodeVar0 = textureLoad( nodeUniform1, ( nodeConst1 + vec3<i32>( 0, 0, 0 ) ) );
		nodeVar1 = textureLoad( nodeUniform1, ( nodeConst1 + vec3<i32>( 1, 0, 0 ) ) );
		nodeVar2 = textureLoad( nodeUniform1, ( nodeConst1 + vec3<i32>( 0, 1, 0 ) ) );
		nodeVar3 = textureLoad( nodeUniform1, ( nodeConst1 + vec3<i32>( 1, 1, 0 ) ) );
		nodeVar4 = textureLoad( nodeUniform1, ( nodeConst1 + vec3<i32>( 0, 0, 1 ) ) );
		nodeVar5 = textureLoad( nodeUniform1, ( nodeConst1 + vec3<i32>( 1, 0, 1 ) ) );
		nodeVar6 = textureLoad( nodeUniform1, ( nodeConst1 + vec3<i32>( 0, 1, 1 ) ) );
		nodeVar7 = textureLoad( nodeUniform1, ( nodeConst1 + vec3<i32>( 1, 1, 1 ) ) );
		textureStore( nodeUniform0, nodeConst0, ( ( ( ( ( ( ( ( ( vec4<f32>( 0.0, 0.0, 0.0, 0.0 ) + nodeVar0 ) + nodeVar1 ) + nodeVar2 ) + nodeVar3 ) + nodeVar4 ) + nodeVar5 ) + nodeVar6 ) + nodeVar7 ) * vec4<f32>( 0.125 ) ) );
		

	}


	

}
