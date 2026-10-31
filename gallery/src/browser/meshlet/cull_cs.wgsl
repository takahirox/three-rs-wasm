// Three.js r187 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals
var<workgroup> WorkgroupArray_20834: array< atomic<u32>, 1 >;
var<workgroup> WorkgroupArray_20835: array< u32, 1 >;

// structs


// uniforms

struct nodeUniform0Struct {
	value : array< vec4<u32> >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> nodeUniform0 : nodeUniform0Struct;

struct nodeUniform2Struct {
	value : array< vec4<f32> >
};
@binding( 2 ) @group( 0 )
var<storage, read> nodeUniform2 : nodeUniform2Struct;

struct nodeUniform4Struct {
	value : array< mat4x4<f32> >
};
@binding( 3 ) @group( 0 )
var<storage, read> nodeUniform4 : nodeUniform4Struct;

struct nodeUniform6Struct {
	value : array< vec4<f32>, 6 >
};
@binding( 4 ) @group( 0 )
var<uniform> nodeUniform6 : nodeUniform6Struct;

struct nodeUniform7Struct {
	value : array< vec4<f32> >
};
@binding( 5 ) @group( 0 )
var<storage, read> nodeUniform7 : nodeUniform7Struct;

struct nodeUniform9Struct {
	value : array< vec4<f32> >
};
@binding( 6 ) @group( 0 )
var<storage, read> nodeUniform9 : nodeUniform9Struct;

struct nodeUniform17Struct {
	value : array< mat4x4<f32> >
};
@binding( 7 ) @group( 0 )
var<storage, read_write> nodeUniform17 : nodeUniform17Struct;

struct nodeUniform19Struct {
	value : array< atomic<u32> >
};
@binding( 8 ) @group( 0 )
var<storage, read_write> nodeUniform19 : nodeUniform19Struct;

struct nodeUniform21Struct {
	value : array< u32 >
};
@binding( 9 ) @group( 0 )
var<storage, read_write> nodeUniform21 : nodeUniform21Struct;

struct objectStruct {
	nodeUniform1 : u32,
	nodeUniform3 : u32,
	nodeUniform5 : u32,
	nodeUniform8 : vec3<f32>,
	nodeUniform10 : f32,
	nodeUniform11 : f32,
	nodeUniform12 : f32,
	nodeUniform13 : vec3<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform16 : f32,
	nodeUniform18 : mat4x4<f32>,
	nodeUniform20 : u32
};
@binding( 1 ) @group( 0 )
var<uniform> object : objectStruct;

// vars


// codes


@compute @workgroup_size( 64, 1, 1 )
fn main( @builtin( global_invocation_id ) globalId : vec3<u32>,
	@builtin( workgroup_id ) workgroupId : vec3<u32>,
	@builtin( local_invocation_id ) localId : vec3<u32>,
	@builtin( num_workgroups ) numWorkgroups : vec3<u32> ) {

	// local vars
	
	var nodeVar0 : u32;
	var nodeVar1 : u32;
	var nodeVar2 : u32;
	var nodeVar3 : u32;
	var nodeVar4 : u32;
	var nodeVar5 : u32;
	var nodeVar6 : u32;
	var nodeVar7 : mat4x4<f32>;
	var nodeVar8 : vec3<f32>;
	var nodeVar9 : bool;
	var nodeVar10 : f32;
	var nodeVar11 : f32;
	var nodeVar12 : f32;
	var nodeVar13 : f32;
	var nodeVar14 : u32;
	var nodeVar15 : u32;
	var nodeVar16 : u32;


	// system
	instanceIndex = globalId.x
		+ globalId.y * ( 64 * numWorkgroups.x )
		+ globalId.z * ( 64 * numWorkgroups.x ) * ( 1 * numWorkgroups.y );

	// flow
	// code


	// flow -> MeshletDrawingPipeline Cull
	nodeVar0 = nodeUniform0.value[ instanceIndex ].w;
	nodeVar1 = ( nodeVar0 & 65535u );
	nodeVar2 = ( nodeVar0 >> 16u );
	nodeVar3 = 255u;
	nodeVar4 = 0u;
	nodeVar5 = 0u;
	let nodeConst0 = bitcast<vec4<u32>>( nodeUniform2.value[ ( ( ( nodeVar1 * 3u ) + 0u ) + object.nodeUniform3 ) ] );
	let nodeConst1 = ( instanceIndex - nodeConst0.x );

	if ( ( ( instanceIndex < object.nodeUniform1 ) && ( nodeConst1 < nodeConst0.y ) ) ) {

		nodeVar6 = ( bitcast<vec4<u32>>( nodeUniform2.value[ ( ( ( nodeVar1 * 3u ) + 1u ) + object.nodeUniform3 ) ] ).x + nodeConst1 );
		nodeVar7 = nodeUniform4.value[ nodeVar6 ];
		nodeVar8 = ( nodeVar7 * vec4<f32>( nodeUniform2.value[ ( ( ( nodeVar2 * 8u ) + 0u ) + object.nodeUniform5 ) ].xyz, 1.0 ) ).xyz;
		nodeVar9 = true;

		for ( var i : i32 = 0; i < 6; i ++ ) {

			let nodeConst2 = abs( nodeUniform7.value[ nodeVar6 ].w );
			let nodeConst3 = vec4<f32>( nodeVar8, ( ( nodeConst2 * nodeUniform2.value[ ( ( ( nodeVar2 * 8u ) + 0u ) + object.nodeUniform5 ) ].w ) + nodeUniform7.value[ nodeVar6 ].x ) );

			if ( ( ( dot( nodeUniform6.value[ i ].xyz, nodeConst3.xyz ) + nodeUniform6.value[ i ].w ) < ( - nodeConst3.w ) ) ) {

				nodeVar9 = false;
				

			}


		}

		nodeVar10 = ( distance( object.nodeUniform8, nodeUniform9.value[ ( nodeVar6 * 2u ) ].xyz ) / object.nodeUniform10 );

		if ( ( ( nodeVar10 < nodeUniform9.value[ ( nodeVar6 * 2u ) ].w ) || ( nodeVar10 >= nodeUniform9.value[ ( ( nodeVar6 * 2u ) + 1u ) ].x ) ) ) {

			nodeVar9 = false;
			

		}


		if ( ( object.nodeUniform12 > 0.5 ) ) {

			nodeVar11 = max( 0.01, distance( object.nodeUniform13, nodeVar8 ) );

		} else {

			nodeVar11 = 1.0;

		}

		nodeVar12 = ( object.nodeUniform11 / nodeVar11 );
		let nodeConst2 = abs( nodeUniform7.value[ nodeVar6 ].w );
		nodeVar13 = ( ( nodeConst2 * nodeUniform2.value[ ( ( ( nodeVar2 * 8u ) + 0u ) + object.nodeUniform5 ) ].w ) * nodeVar12 );

		if ( ( nodeVar13 < object.nodeUniform14 ) ) {

			nodeVar9 = false;
			

		}


		if ( nodeVar9 ) {

			nodeVar14 = min( bitcast<vec4<u32>>( nodeUniform2.value[ ( ( ( nodeVar2 * 8u ) + 1u ) + object.nodeUniform5 ) ] ).x, u32( nodeUniform2.value[ ( ( ( nodeVar1 * 3u ) + 2u ) + object.nodeUniform3 ) ].z ) );
			nodeVar3 = 0u;

			for ( var lod : u32 = 1u; lod < nodeVar14; lod ++ ) {


				if ( ( ( ( nodeUniform2.value[ ( ( ( nodeVar2 * 8u ) + ( lod + 2u ) ) + object.nodeUniform5 ) ].w * nodeConst2 ) * nodeVar12 ) <= object.nodeUniform15 ) ) {

					nodeVar3 = lod;
					

				}


			}


			if ( ( nodeVar13 < object.nodeUniform16 ) ) {

				nodeVar3 = ( nodeVar14 - 1u );
				

			}

			nodeUniform17.value[ nodeVar6 ] = ( object.nodeUniform18 * nodeVar7 );
			nodeVar4 = ( u32( nodeUniform2.value[ ( ( ( nodeVar2 * 8u ) + ( nodeVar3 + 2u ) ) + object.nodeUniform5 ) ].y ) / 64u );

			if ( ( ( ( u32( nodeUniform2.value[ ( ( ( nodeVar1 * 3u ) + 2u ) + object.nodeUniform3 ) ].w ) & 2u ) != 0u ) && ( nodeVar10 >= nodeUniform9.value[ ( ( nodeVar6 * 2u ) + 1u ) ].y ) ) ) {

				nodeVar3 = ( nodeVar3 | 512u );
				

			}

			nodeVar5 = pack2x16float(( vec2<f32>( nodeUniform7.value[ nodeVar6 ].w, nodeUniform7.value[ nodeVar6 ].x ) * vec2<f32>( 1.001 ) ));
			

		}

		

	}

	let nodeConst4 = atomicAdd( &WorkgroupArray_20834[ 0u ], nodeVar4 );
	nodeVar15 = nodeConst4;
	workgroupBarrier();

	if ( ( localId.x == 0u ) ) {

		let nodeConst5 = atomicLoad( &WorkgroupArray_20834[ 0u ] );
		let nodeConst6 = atomicAdd( &nodeUniform19.value[ 0u ], nodeConst5 );
		WorkgroupArray_20835[ 0u ] = nodeConst6;
		

	}

	workgroupBarrier();
	nodeVar16 = ( WorkgroupArray_20835[ 0u ] + nodeVar15 );

	if ( ( instanceIndex < object.nodeUniform1 ) ) {

		nodeUniform0.value[ instanceIndex ] = vec4<u32>( nodeVar16, ( nodeVar3 | ( nodeVar1 << 16u ) ), nodeVar5, nodeVar0 );
		

	}


	for ( var cluster : u32 = 0u; cluster < nodeVar4; cluster ++ ) {

		let nodeConst7 = ( nodeVar16 + cluster );

		if ( ( nodeConst7 < object.nodeUniform20 ) ) {

			nodeUniform21.value[ nodeConst7 ] = instanceIndex;
			

		}


	}


	

}
