// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms
@binding( 0 ) @group( 0 ) var nodeUniform0 : texture_storage_3d<rgba8unorm, read>;
@binding( 1 ) @group( 0 ) var nodeUniform1 : texture_storage_3d<rgba8unorm, write>;

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


	// flow -> VXGI.OpacityMip2
	if ( instanceIndex >= object.nodeUniform2 ) { return; }


	if ( ( instanceIndex < 4536u ) ) {

		let nodeConst0 = vec3<u32>( ( instanceIndex % 18u ), ( ( instanceIndex / 18u ) % 14u ), ( instanceIndex / 252u ) );
		let nodeConst1 = vec3<i32>( ( nodeConst0 * vec3<u32>( 2u ) ) );
		nodeVar0 = textureLoad( nodeUniform0, ( nodeConst1 + vec3<i32>( 0, 0, 0 ) ) );
		let nodeConst2 = nodeVar0;
		nodeVar1 = textureLoad( nodeUniform0, ( nodeConst1 + vec3<i32>( 1, 0, 0 ) ) );
		let nodeConst3 = nodeVar1;
		nodeVar2 = textureLoad( nodeUniform0, ( nodeConst1 + vec3<i32>( 0, 1, 0 ) ) );
		let nodeConst4 = nodeVar2;
		nodeVar3 = textureLoad( nodeUniform0, ( nodeConst1 + vec3<i32>( 1, 1, 0 ) ) );
		let nodeConst5 = nodeVar3;
		nodeVar4 = textureLoad( nodeUniform0, ( nodeConst1 + vec3<i32>( 0, 0, 1 ) ) );
		let nodeConst6 = nodeVar4;
		nodeVar5 = textureLoad( nodeUniform0, ( nodeConst1 + vec3<i32>( 1, 0, 1 ) ) );
		let nodeConst7 = nodeVar5;
		nodeVar6 = textureLoad( nodeUniform0, ( nodeConst1 + vec3<i32>( 0, 1, 1 ) ) );
		let nodeConst8 = nodeVar6;
		nodeVar7 = textureLoad( nodeUniform0, ( nodeConst1 + vec3<i32>( 1, 1, 1 ) ) );
		let nodeConst9 = nodeVar7;
		textureStore( nodeUniform1, nodeConst0, vec4<f32>( ( ( ( ( ( 0.0 + ( 1.0 - ( ( 1.0 - nodeConst2.x ) * ( 1.0 - nodeConst3.x ) ) ) ) + ( 1.0 - ( ( 1.0 - nodeConst6.x ) * ( 1.0 - nodeConst7.x ) ) ) ) + ( 1.0 - ( ( 1.0 - nodeConst4.x ) * ( 1.0 - nodeConst5.x ) ) ) ) + ( 1.0 - ( ( 1.0 - nodeConst8.x ) * ( 1.0 - nodeConst9.x ) ) ) ) * 0.25 ), ( ( ( ( ( 0.0 + ( 1.0 - ( ( 1.0 - nodeConst2.y ) * ( 1.0 - nodeConst4.y ) ) ) ) + ( 1.0 - ( ( 1.0 - nodeConst6.y ) * ( 1.0 - nodeConst8.y ) ) ) ) + ( 1.0 - ( ( 1.0 - nodeConst3.y ) * ( 1.0 - nodeConst5.y ) ) ) ) + ( 1.0 - ( ( 1.0 - nodeConst7.y ) * ( 1.0 - nodeConst9.y ) ) ) ) * 0.25 ), ( ( ( ( ( 0.0 + ( 1.0 - ( ( 1.0 - nodeConst2.z ) * ( 1.0 - nodeConst6.z ) ) ) ) + ( 1.0 - ( ( 1.0 - nodeConst4.z ) * ( 1.0 - nodeConst8.z ) ) ) ) + ( 1.0 - ( ( 1.0 - nodeConst3.z ) * ( 1.0 - nodeConst7.z ) ) ) ) + ( 1.0 - ( ( 1.0 - nodeConst5.z ) * ( 1.0 - nodeConst9.z ) ) ) ) * 0.25 ), ( ( ( ( ( ( ( ( ( 0.0 + nodeConst2.w ) + nodeConst3.w ) + nodeConst4.w ) + nodeConst5.w ) + nodeConst6.w ) + nodeConst7.w ) + nodeConst8.w ) + nodeConst9.w ) * 0.125 ) ) );
		

	}


	

}
