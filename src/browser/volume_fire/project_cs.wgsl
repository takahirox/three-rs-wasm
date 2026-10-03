// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms
@binding( 0 ) @group( 0 ) var nodeUniform0 : texture_storage_3d<rgba16float, write>;
@binding( 1 ) @group( 0 ) var nodeUniform1_sampler : sampler;
@binding( 2 ) @group( 0 ) var nodeUniform1 : texture_3d<f32>;
@binding( 3 ) @group( 0 ) var nodeUniform2_sampler : sampler;
@binding( 4 ) @group( 0 ) var nodeUniform2 : texture_3d<f32>;

struct objectStruct {
	nodeUniform3 : u32
};
@binding( 5 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec3<u32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec4<f32>;

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


	// flow -> project
	if ( instanceIndex >= object.nodeUniform3 ) { return; }

	nodeVar0 = vec3<u32>( ( instanceIndex % 100u ), ( ( instanceIndex / 100u ) % 100u ), ( instanceIndex / 10000u ) );
	nodeVar1 = ( ( vec3<f32>( nodeVar0 ) + vec3<f32>( 0.5 ) ) / vec3<f32>( 100.0, 100.0, 200.0 ) );
	nodeVar2 = textureSampleLevel( nodeUniform1, nodeUniform1_sampler, nodeVar1, 0.0 );
	nodeVar3 = textureSampleLevel( nodeUniform2, nodeUniform2_sampler, ( nodeVar1 + vec3<f32>( 0.01, 0.0, 0.0 ) ), 0.0 );
	nodeVar4 = textureSampleLevel( nodeUniform2, nodeUniform2_sampler, ( nodeVar1 - vec3<f32>( 0.01, 0.0, 0.0 ) ), 0.0 );
	nodeVar5 = textureSampleLevel( nodeUniform2, nodeUniform2_sampler, ( nodeVar1 + vec3<f32>( 0.0, 0.01, 0.0 ) ), 0.0 );
	nodeVar6 = textureSampleLevel( nodeUniform2, nodeUniform2_sampler, ( nodeVar1 - vec3<f32>( 0.0, 0.01, 0.0 ) ), 0.0 );
	nodeVar7 = textureSampleLevel( nodeUniform2, nodeUniform2_sampler, ( nodeVar1 + vec3<f32>( 0.0, 0.0, 0.005 ) ), 0.0 );
	nodeVar8 = textureSampleLevel( nodeUniform2, nodeUniform2_sampler, ( nodeVar1 - vec3<f32>( 0.0, 0.0, 0.005 ) ), 0.0 );
	textureStore( nodeUniform0, nodeVar0, vec4<f32>( ( nodeVar2.xyz - ( vec3<f32>( ( nodeVar3.x - nodeVar4.x ), ( nodeVar5.x - nodeVar6.x ), ( nodeVar7.x - nodeVar8.x ) ) * vec3<f32>( 0.5 ) ) ), 0.0 ) );

	

}
