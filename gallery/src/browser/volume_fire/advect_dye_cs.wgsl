// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms
@binding( 0 ) @group( 0 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 0 ) var nodeUniform0 : texture_3d<f32>;
@binding( 2 ) @group( 0 ) var nodeUniform1_sampler : sampler;
@binding( 3 ) @group( 0 ) var nodeUniform1 : texture_3d<f32>;
@binding( 5 ) @group( 0 ) var nodeUniform6 : texture_storage_3d<rgba16float, write>;

struct objectStruct {
	nodeUniform2 : vec3<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform7 : u32
};
@binding( 4 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec3<u32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : f32;

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


	// flow -> advectDye
	if ( instanceIndex >= object.nodeUniform7 ) { return; }

	nodeVar0 = vec3<u32>( ( instanceIndex % 100u ), ( ( instanceIndex / 100u ) % 100u ), ( instanceIndex / 10000u ) );
	nodeVar1 = ( ( vec3<f32>( nodeVar0 ) + vec3<f32>( 0.5 ) ) / vec3<f32>( 100.0, 100.0, 200.0 ) );
	nodeVar2 = textureSampleLevel( nodeUniform1, nodeUniform1_sampler, nodeVar1, 0.0 );
	nodeVar3 = ( nodeVar1 - ( ( nodeVar2.xyz / object.nodeUniform2 ) * vec3<f32>( object.nodeUniform3 ) ) );
	nodeVar4 = textureSampleLevel( nodeUniform0, nodeUniform0_sampler, nodeVar3, 0.0 );
	nodeVar5 = ( nodeVar4.x * max( ( 1.0 - ( object.nodeUniform4 * object.nodeUniform3 ) ), 0.0 ) );
	nodeVar6 = ( nodeVar4.y * max( ( 1.0 - ( object.nodeUniform5 * object.nodeUniform3 ) ), 0.0 ) );
	nodeVar7 = textureSampleLevel( nodeUniform0, nodeUniform0_sampler, ( ( floor( ( nodeVar3 * vec3<f32>( 100.0, 100.0, 200.0 ) ) ) + vec3<f32>( 0.5 ) ) / vec3<f32>( 100.0, 100.0, 200.0 ) ), 0.0 );
	nodeVar8 = ( nodeVar7.z + object.nodeUniform3 );
	nodeVar6 = clamp( nodeVar6, 0.0, 12.0 );

	if ( ( nodeVar5 <= 0.01 ) ) {

		nodeVar8 = 0.0;
		

	}

	textureStore( nodeUniform6, nodeVar0, vec4<f32>( nodeVar5, nodeVar6, nodeVar8, 1.0 ) );

	

}
