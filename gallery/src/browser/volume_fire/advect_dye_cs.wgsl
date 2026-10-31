// Three.js r187 - Node System

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
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : f32;

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

	let nodeConst0 = vec3<u32>( ( instanceIndex % 100u ), ( ( instanceIndex / 100u ) % 100u ), ( instanceIndex / 10000u ) );
	let nodeConst1 = ( ( vec3<f32>( nodeConst0 ) + vec3<f32>( 0.5 ) ) / vec3<f32>( 100.0, 100.0, 200.0 ) );
	nodeVar0 = textureSampleLevel( nodeUniform1, nodeUniform1_sampler, nodeConst1, 0.0 );
	let nodeConst2 = ( nodeConst1 - ( ( nodeVar0.xyz / object.nodeUniform2 ) * vec3<f32>( object.nodeUniform3 ) ) );
	nodeVar1 = textureSampleLevel( nodeUniform0, nodeUniform0_sampler, nodeConst2, 0.0 );
	nodeVar2 = ( nodeVar1.x * max( ( 1.0 - ( object.nodeUniform4 * object.nodeUniform3 ) ), 0.0 ) );
	nodeVar3 = ( nodeVar1.y * max( ( 1.0 - ( object.nodeUniform5 * object.nodeUniform3 ) ), 0.0 ) );
	nodeVar4 = textureSampleLevel( nodeUniform0, nodeUniform0_sampler, ( ( floor( ( nodeConst2 * vec3<f32>( 100.0, 100.0, 200.0 ) ) ) + vec3<f32>( 0.5 ) ) / vec3<f32>( 100.0, 100.0, 200.0 ) ), 0.0 );
	nodeVar5 = ( nodeVar4.z + object.nodeUniform3 );
	nodeVar3 = clamp( nodeVar3, 0.0, 12.0 );

	if ( ( nodeVar2 <= 0.01 ) ) {

		nodeVar5 = 0.0;
		

	}

	textureStore( nodeUniform6, nodeConst0, vec4<f32>( nodeVar2, nodeVar3, nodeVar5, 1.0 ) );

	

}
