// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms
@binding( 2 ) @group( 0 ) var nodeUniform2 : texture_storage_3d<rgba8unorm, write>;

struct NodeBuffer_5339Struct {
	value : array< u32 >
};
@binding( 1 ) @group( 0 )
var<storage, read> NodeBuffer_5339 : NodeBuffer_5339Struct;

struct objectStruct {
	nodeUniform0 : u32,
	nodeUniform3 : u32
};
@binding( 0 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;

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


	// flow -> VXGI.Resolve
	if ( instanceIndex >= object.nodeUniform3 ) { return; }


	if ( ( instanceIndex < object.nodeUniform0 ) ) {

		let nodeConst0 = vec3<u32>( ( instanceIndex % 72u ), ( ( instanceIndex / 72u ) % 56u ), ( instanceIndex / 4032u ) );
		let nodeConst1 = NodeBuffer_5339.value[ instanceIndex ];

		if ( ( ( nodeConst1 & 3u ) != 0u ) ) {

			nodeVar0 = 0.25;

		} else {

			nodeVar0 = 0.0;

		}


		if ( ( ( nodeConst1 & 12u ) != 0u ) ) {

			nodeVar1 = 0.25;

		} else {

			nodeVar1 = 0.0;

		}


		if ( ( ( nodeConst1 & 48u ) != 0u ) ) {

			nodeVar2 = 0.25;

		} else {

			nodeVar2 = 0.0;

		}


		if ( ( ( nodeConst1 & 192u ) != 0u ) ) {

			nodeVar3 = 0.25;

		} else {

			nodeVar3 = 0.0;

		}


		if ( ( ( nodeConst1 & 5u ) != 0u ) ) {

			nodeVar4 = 0.25;

		} else {

			nodeVar4 = 0.0;

		}


		if ( ( ( nodeConst1 & 10u ) != 0u ) ) {

			nodeVar5 = 0.25;

		} else {

			nodeVar5 = 0.0;

		}


		if ( ( ( nodeConst1 & 80u ) != 0u ) ) {

			nodeVar6 = 0.25;

		} else {

			nodeVar6 = 0.0;

		}


		if ( ( ( nodeConst1 & 160u ) != 0u ) ) {

			nodeVar7 = 0.25;

		} else {

			nodeVar7 = 0.0;

		}


		if ( ( ( nodeConst1 & 17u ) != 0u ) ) {

			nodeVar8 = 0.25;

		} else {

			nodeVar8 = 0.0;

		}


		if ( ( ( nodeConst1 & 34u ) != 0u ) ) {

			nodeVar9 = 0.25;

		} else {

			nodeVar9 = 0.0;

		}


		if ( ( ( nodeConst1 & 68u ) != 0u ) ) {

			nodeVar10 = 0.25;

		} else {

			nodeVar10 = 0.0;

		}


		if ( ( ( nodeConst1 & 136u ) != 0u ) ) {

			nodeVar11 = 0.25;

		} else {

			nodeVar11 = 0.0;

		}

		textureStore( nodeUniform2, nodeConst0, vec4<f32>( ( ( ( nodeVar0 + nodeVar1 ) + nodeVar2 ) + nodeVar3 ), ( ( ( nodeVar4 + nodeVar5 ) + nodeVar6 ) + nodeVar7 ), ( ( ( nodeVar8 + nodeVar9 ) + nodeVar10 ) + nodeVar11 ), ( f32( countOneBits( nodeConst1 ) ) / 8.0 ) ) );
		

	}


	

}
