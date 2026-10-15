// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct NodeBuffer_1015Struct {
	value : array< vec4<f32>, 6 >
};
@binding( 0 ) @group( 1 )
var<uniform> NodeBuffer_1015 : NodeBuffer_1015Struct;

struct NodeBuffer_1003Struct {
	value : array< vec4<f32> >
};
@binding( 1 ) @group( 1 )
var<storage, read_write> NodeBuffer_1003 : NodeBuffer_1003Struct;

struct NodeBuffer_991Struct {
	value : array< vec4<u32> >
};
@binding( 3 ) @group( 1 )
var<storage, read> NodeBuffer_991 : NodeBuffer_991Struct;

struct NodeBuffer_992Struct {
	value : array< vec4<f32> >
};
@binding( 4 ) @group( 1 )
var<storage, read> NodeBuffer_992 : NodeBuffer_992Struct;

struct NodeBuffer_1007Struct {
	value : array< atomic<u32> >
};
@binding( 5 ) @group( 1 )
var<storage, read_write> NodeBuffer_1007 : NodeBuffer_1007Struct;

struct NodeBuffer_1010Struct {
	value : array< vec4<u32> >
};
@binding( 6 ) @group( 1 )
var<storage, read_write> NodeBuffer_1010 : NodeBuffer_1010Struct;

struct NodeBuffer_1004Struct {
	value : array< mat4x4<f32> >
};
@binding( 7 ) @group( 1 )
var<storage, read_write> NodeBuffer_1004 : NodeBuffer_1004Struct;

struct NodeBuffer_1005Struct {
	value : array< mat4x4<f32> >
};
@binding( 8 ) @group( 1 )
var<storage, read_write> NodeBuffer_1005 : NodeBuffer_1005Struct;

struct objectStruct {
	nodeUniform2 : f32,
	nodeUniform3 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform8 : f32,
	nodeUniform14 : mat4x4<f32>,
	nodeUniform15 : u32
};
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform7 : f32,
	nodeUniform4 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> nodeVar0 : bool;
var<private> nodeVar1 : u32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : mat4x4<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : bool;

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


	// flow -> Compute Frustum
	if ( instanceIndex >= object.nodeUniform15 ) { return; }

	nodeVar0 = true;

	for ( var i : i32 = 0; i < 6; i ++ ) {


		if ( ( ( dot( NodeBuffer_1015.value[ i ].xyz, NodeBuffer_1003.value[ instanceIndex ].xyz ) + NodeBuffer_1015.value[ i ].w ) < ( - ( NodeBuffer_1003.value[ instanceIndex ].w * 2.0 ) ) ) ) {

			nodeVar0 = false;
			

		}


	}


	if ( nodeVar0 ) {

		nodeVar1 = 0u;
		nodeVar2 = ( ( ( object.nodeUniform2 / max( 0.01, distance( object.nodeUniform3, NodeBuffer_1003.value[ instanceIndex ].xyz ) ) ) * render.nodeUniform4.y ) / 2.0 );

		if ( ( ( ( 0.2 * NodeBuffer_1003.value[ instanceIndex ].w ) * nodeVar2 ) <= object.nodeUniform5 ) ) {

			nodeVar1 = 6u;
			

		} else {


			if ( ( ( ( 0.1 * NodeBuffer_1003.value[ instanceIndex ].w ) * nodeVar2 ) <= object.nodeUniform5 ) ) {

				nodeVar1 = 5u;
				

			} else {


				if ( ( ( ( 0.06 * NodeBuffer_1003.value[ instanceIndex ].w ) * nodeVar2 ) <= object.nodeUniform5 ) ) {

					nodeVar1 = 4u;
					

				} else {


					if ( ( ( ( 0.03 * NodeBuffer_1003.value[ instanceIndex ].w ) * nodeVar2 ) <= object.nodeUniform5 ) ) {

						nodeVar1 = 3u;
						

					} else {


						if ( ( ( ( 0.015 * NodeBuffer_1003.value[ instanceIndex ].w ) * nodeVar2 ) <= object.nodeUniform5 ) ) {

							nodeVar1 = 2u;
							

						} else {


							if ( ( ( ( 0.005 * NodeBuffer_1003.value[ instanceIndex ].w ) * nodeVar2 ) <= object.nodeUniform5 ) ) {

								nodeVar1 = 1u;
								

							}

							

						}

						

					}

					

				}

				

			}

			

		}


		for ( var cIdx : u32 = 0u; cIdx < ( ( NodeBuffer_991.value[ nodeVar1 ].y + 63u ) / 64u ); cIdx ++ ) {

			nodeVar3 = ( ( render.nodeUniform7 * object.nodeUniform8 ) + f32( instanceIndex ) );
			nodeVar4 = cos( nodeVar3 );
			nodeVar5 = sin( nodeVar3 );
			nodeVar6 = mat4x4<f32>( vec4<f32>( ( nodeVar4 * NodeBuffer_1003.value[ instanceIndex ].w ), 0.0, ( nodeVar5 * NodeBuffer_1003.value[ instanceIndex ].w ), 0.0 ), vec4<f32>( 0.0, NodeBuffer_1003.value[ instanceIndex ].w, 0.0, 0.0 ), vec4<f32>( ( ( - nodeVar5 ) * NodeBuffer_1003.value[ instanceIndex ].w ), 0.0, ( nodeVar4 * NodeBuffer_1003.value[ instanceIndex ].w ), 0.0 ), vec4<f32>( NodeBuffer_1003.value[ instanceIndex ].xyz, 1.0 ) );
			nodeVar7 = ( nodeVar6 * vec4<f32>( NodeBuffer_992.value[ ( NodeBuffer_991.value[ nodeVar1 ].z + cIdx ) ].xyz, 1.0 ) ).xyz;
			nodeVar8 = ( NodeBuffer_992.value[ ( NodeBuffer_991.value[ nodeVar1 ].z + cIdx ) ].w * NodeBuffer_1003.value[ instanceIndex ].w );
			nodeVar9 = true;

			for ( var pIdx : i32 = 0; pIdx < 6; pIdx ++ ) {


				if ( ( ( dot( NodeBuffer_1015.value[ pIdx ].xyz, nodeVar7 ) + NodeBuffer_1015.value[ pIdx ].w ) < ( - nodeVar8 ) ) ) {

					nodeVar9 = false;
					

				}


			}


			if ( nodeVar9 ) {

				let nodeConst0 = atomicAdd( &NodeBuffer_1007.value[ 0u ], 1u );
				NodeBuffer_1010.value[ nodeConst0 ] = vec4<u32>( instanceIndex, NodeBuffer_991.value[ nodeVar1 ].x, NodeBuffer_991.value[ nodeVar1 ].y, cIdx );
				

			}


		}

		NodeBuffer_1004.value[ instanceIndex ] = nodeVar6;
		NodeBuffer_1005.value[ instanceIndex ] = ( object.nodeUniform14 * nodeVar6 );
		

	}


	

}
