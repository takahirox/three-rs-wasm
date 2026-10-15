// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms
@binding( 0 ) @group( 0 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 0 ) var nodeUniform0 : texture_3d<f32>;
@binding( 3 ) @group( 0 ) var nodeUniform3_sampler : sampler;
@binding( 4 ) @group( 0 ) var nodeUniform3 : texture_3d<f32>;
@binding( 5 ) @group( 0 ) var nodeUniform6_sampler : sampler;
@binding( 6 ) @group( 0 ) var nodeUniform6 : texture_3d<f32>;
@binding( 7 ) @group( 0 ) var nodeUniform16 : texture_storage_3d<rgba16float, write>;

struct objectStruct {
	nodeUniform1 : vec3<f32>,
	nodeUniform2 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform7 : f32,
	nodeUniform8 : f32,
	nodeUniform9 : f32,
	nodeUniform10 : f32,
	nodeUniform11 : f32,
	nodeUniform12 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform17 : u32
};
@binding( 2 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec3<u32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : vec3<f32>;

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


	// flow -> advectVelocity
	if ( instanceIndex >= object.nodeUniform17 ) { return; }

	nodeVar0 = vec3<u32>( ( instanceIndex % 100u ), ( ( instanceIndex / 100u ) % 100u ), ( instanceIndex / 10000u ) );
	nodeVar1 = ( ( vec3<f32>( nodeVar0 ) + vec3<f32>( 0.5 ) ) / vec3<f32>( 100.0, 100.0, 200.0 ) );
	nodeVar2 = textureSampleLevel( nodeUniform0, nodeUniform0_sampler, nodeVar1, 0.0 );
	nodeVar3 = textureSampleLevel( nodeUniform0, nodeUniform0_sampler, ( nodeVar1 - ( ( nodeVar2.xyz / object.nodeUniform1 ) * vec3<f32>( object.nodeUniform2 ) ) ), 0.0 );
	nodeVar4 = nodeVar3.xyz;
	nodeVar5 = textureSampleLevel( nodeUniform3, nodeUniform3_sampler, nodeVar1, 0.0 );
	nodeVar4 = ( nodeVar4 + ( vec3<f32>( 0.0, ( ( ( nodeVar5.y * object.nodeUniform4 ) - ( nodeVar5.x * object.nodeUniform5 ) ) * 12.0 ), 0.0 ) * vec3<f32>( object.nodeUniform2 ) ) );
	nodeVar6 = textureSampleLevel( nodeUniform6, nodeUniform6_sampler, ( nodeVar1 + ( vec3<f32>( 0.0, ( ( - nodeVar5.z ) * 0.6 ), ( nodeVar5.z * 0.13 ) ) / vec3<f32>( object.nodeUniform7 ) ) ), 0.0 );
	nodeVar7 = textureSampleLevel( nodeUniform6, nodeUniform6_sampler, ( ( nodeVar1 * vec3<f32>( 0.5 ) ) + ( vec3<f32>( 0.0, ( object.nodeUniform10 * 0.25 ), ( object.nodeUniform10 * 0.06 ) ) / vec3<f32>( object.nodeUniform7 ) ) ), 0.0 );
	nodeVar4 = ( nodeVar4 + ( ( ( ( ( ( nodeVar6.xyz * vec3<f32>( object.nodeUniform8 ) ) * vec3<f32>( nodeVar5.y ) ) * vec3<f32>( exp( ( nodeVar5.z * ( - object.nodeUniform9 ) ) ) ) ) + ( ( nodeVar7.xyz * vec3<f32>( ( object.nodeUniform8 * 0.2 ) ) ) * vec3<f32>( nodeVar5.x ) ) ) * vec3<f32>( 12.0 ) ) * vec3<f32>( object.nodeUniform2 ) ) );
	nodeVar4 = ( nodeVar4 * vec3<f32>( max( ( 1.0 - ( object.nodeUniform11 * object.nodeUniform2 ) ), 0.0 ) ) );
	nodeVar8 = distance( ( ( ( nodeVar1 - vec3<f32>( 0.5 ) ) * object.nodeUniform1 ) + vec3<f32>( 0.0, 6.0, 0.0 ) ), object.nodeUniform12 );

	if ( ( nodeVar8 < 1.0 ) ) {

		nodeVar9 = textureSampleLevel( nodeUniform6, nodeUniform6_sampler, ( nodeVar1 + ( vec3<f32>( 0.0, ( object.nodeUniform10 * 0.5 ), 0.0 ) / vec3<f32>( object.nodeUniform7 ) ) ), 0.0 );
		nodeVar4 = ( nodeVar4 + ( ( ( ( object.nodeUniform13 * vec3<f32>( object.nodeUniform14 ) ) + ( ( nodeVar9.xyz * vec3<f32>( object.nodeUniform8 ) ) * vec3<f32>( object.nodeUniform15 ) ) ) * vec3<f32>( object.nodeUniform2 ) ) * vec3<f32>( smoothstep( 0.0, 1.0, ( 1.0 - ( nodeVar8 / 1.0 ) ) ) ) ) );
		

	}

	nodeVar10 = min( nodeVar1, ( vec3<f32>( 1.0, 1.0, 1.0 ) - nodeVar1 ) );
	nodeVar4 = ( nodeVar4 * vec3<f32>( smoothstep( 0.0, 0.08, min( nodeVar10.x, min( nodeVar10.y, nodeVar10.z ) ) ) ) );
	textureStore( nodeUniform16, nodeVar0, vec4<f32>( nodeVar4, 0.0 ) );

	

}
