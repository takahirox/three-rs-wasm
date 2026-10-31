// Three.js r187 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct nodeUniform0Struct {
	value : array< atomic<u32> >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> nodeUniform0 : nodeUniform0Struct;

struct nodeUniform2Struct {
	value : array< u32 >
};
@binding( 2 ) @group( 0 )
var<storage, read_write> nodeUniform2 : nodeUniform2Struct;

struct nodeUniform3Struct {
	value : array< vec4<u32> >
};
@binding( 3 ) @group( 0 )
var<storage, read_write> nodeUniform3 : nodeUniform3Struct;

struct nodeUniform4Struct {
	value : array< vec4<f32> >
};
@binding( 4 ) @group( 0 )
var<storage, read> nodeUniform4 : nodeUniform4Struct;

struct nodeUniform6Struct {
	value : array< u32 >
};
@binding( 5 ) @group( 0 )
var<storage, read_write> nodeUniform6 : nodeUniform6Struct;

struct nodeUniform9Struct {
	value : array< mat4x4<f32> >
};
@binding( 6 ) @group( 0 )
var<storage, read> nodeUniform9 : nodeUniform9Struct;

struct nodeUniform11Struct {
	value : array< vec4<f32>, 6 >
};
@binding( 7 ) @group( 0 )
var<uniform> nodeUniform11 : nodeUniform11Struct;

struct nodeUniform13Struct {
	value : array< f32 >
};
@binding( 8 ) @group( 0 )
var<storage, read> nodeUniform13 : nodeUniform13Struct;

struct nodeUniform14Struct {
	value : array< vec4<f32>, 16 >
};
@binding( 9 ) @group( 0 )
var<uniform> nodeUniform14 : nodeUniform14Struct;

struct nodeUniform17Struct {
	value : array< vec4<u32> >
};
@binding( 10 ) @group( 0 )
var<storage, read_write> nodeUniform17 : nodeUniform17Struct;

struct objectStruct {
	nodeUniform1 : u32,
	nodeUniform5 : u32,
	nodeUniform7 : u32,
	nodeUniform8 : u32,
	nodeUniform10 : vec3<f32>,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform15 : vec2<f32>,
	nodeUniform16 : u32
};
@binding( 1 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : u32;
var<private> nodeVar1 : u32;
var<private> nodeVar2 : vec4<u32>;
var<private> nodeVar3 : u32;
var<private> nodeVar4 : u32;
var<private> nodeVar5 : u32;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : bool;
var<private> nodeVar9 : bool;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : bool;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : vec4<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : vec4<f32>;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : vec4<f32>;
var<private> nodeVar24 : bool;
var<private> nodeVar25 : u32;
var<private> nodeVar26 : u32;

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


	// flow -> MeshletDrawingPipeline Cluster Cull
	nodeVar0 = ( ( ( ( workgroupId.y * 65535u ) + workgroupId.x ) * 64u ) + localId.x );
	let nodeConst0 = atomicLoad( &nodeUniform0.value[ 0u ] );

	if ( ( nodeVar0 >= min( nodeConst0, object.nodeUniform1 ) ) ) {

		return;
		

	}

	nodeVar1 = nodeUniform2.value[ nodeVar0 ];
	nodeVar2 = nodeUniform3.value[ nodeVar1 ];
	nodeVar3 = ( nodeVar0 - nodeVar2.x );
	nodeVar4 = ( nodeVar2.y >> 16u );
	nodeVar5 = ( nodeVar2.w >> 16u );
	let nodeConst1 = ( nodeVar2.y & 255u );
	let nodeConst2 = u32( nodeUniform4.value[ ( ( ( nodeVar5 * 8u ) + ( nodeConst1 + 2u ) ) + object.nodeUniform5 ) ].y );

	if ( ( nodeVar3 < ( nodeConst2 / 64u ) ) ) {


		if ( ( ( nodeVar2.y & 256u ) != 0u ) ) {

			let nodeConst3 = bitcast<vec4<u32>>( nodeUniform4.value[ ( ( ( nodeVar4 * 3u ) + 1u ) + object.nodeUniform7 ) ] );
			let nodeConst4 = bitcast<vec4<u32>>( nodeUniform4.value[ ( ( ( nodeVar4 * 3u ) + 0u ) + object.nodeUniform7 ) ] );
			let nodeConst5 = ( nodeVar1 - nodeConst4.x );
			let nodeConst6 = ( ( nodeConst3.z + ( nodeConst5 * nodeConst3.w ) ) + nodeVar3 );

			if ( ( nodeUniform6.value[ nodeConst6 ] != ( nodeConst1 + 1u ) ) ) {

				atomicAdd( &nodeUniform0.value[ object.nodeUniform8 ], 1u );
				

			}

			nodeUniform6.value[ nodeConst6 ] = 0u;
			return;
			

		}

		let nodeConst3 = bitcast<vec4<u32>>( nodeUniform4.value[ ( ( ( nodeVar4 * 3u ) + 1u ) + object.nodeUniform7 ) ] );
		let nodeConst4 = bitcast<vec4<u32>>( nodeUniform4.value[ ( ( ( nodeVar4 * 3u ) + 0u ) + object.nodeUniform7 ) ] );
		let nodeConst5 = ( nodeVar1 - nodeConst4.x );
		let nodeConst7 = bitcast<vec4<u32>>( nodeUniform4.value[ ( ( ( nodeVar5 * 8u ) + 1u ) + object.nodeUniform5 ) ] );
		let nodeConst8 = ( ( nodeConst7.z + u32( nodeUniform4.value[ ( ( ( nodeVar5 * 8u ) + ( nodeConst1 + 2u ) ) + object.nodeUniform5 ) ].z ) ) + nodeVar3 );
		nodeVar6 = ( nodeUniform9.value[ ( nodeConst3.x + nodeConst5 ) ] * vec4<f32>( nodeUniform4.value[ ( nodeConst8 * 2u ) ].xyz, 1.0 ) ).xyz;
		let nodeConst9 = unpack2x16float(nodeVar2.z);
		nodeVar7 = ( ( nodeUniform4.value[ ( nodeConst8 * 2u ) ].w * abs( nodeConst9.x ) ) + nodeConst9.y );
		nodeVar8 = true;

		if ( ( ( nodeConst9.x > 0.0 ) && ( ( u32( nodeUniform4.value[ ( ( ( nodeVar4 * 3u ) + 2u ) + object.nodeUniform7 ) ].w ) & 1u ) != 0u ) ) ) {

			let nodeConst10 = ( nodeVar6 - object.nodeUniform10 );

			if ( ( dot( nodeConst10, normalize( ( nodeUniform9.value[ ( nodeConst3.x + nodeConst5 ) ] * vec4<f32>( nodeUniform4.value[ ( ( nodeConst8 * 2u ) + 1u ) ].xyz, 0.0 ) ).xyz ) ) >= ( ( nodeUniform4.value[ ( ( nodeConst8 * 2u ) + 1u ) ].w * length( nodeConst10 ) ) + nodeVar7 ) ) ) {

				nodeVar8 = false;
				

			}

			

		}


		for ( var plane : i32 = 0; plane < 6; plane ++ ) {

			let nodeConst11 = vec4<f32>( nodeVar6, nodeVar7 );

			if ( ( ( dot( nodeUniform11.value[ plane ].xyz, nodeConst11.xyz ) + nodeUniform11.value[ plane ].w ) < ( - nodeConst11.w ) ) ) {

				nodeVar8 = false;
				

			}


		}


		if ( nodeVar8 ) {

			let nodeConst6 = ( ( nodeConst3.z + ( nodeConst5 * nodeConst3.w ) ) + nodeVar3 );
			let nodeConst12 = ( nodeConst1 + 1u );
			nodeVar9 = ( nodeUniform6.value[ nodeConst6 ] == nodeConst12 );
			nodeVar10 = 1000000000.0;
			nodeVar11 = 1000000000.0;
			nodeVar12 = 1000000000.0;
			nodeVar13 = -1000000000.0;
			nodeVar14 = -1000000000.0;
			nodeVar15 = false;
			let nodeConst11 = vec4<f32>( nodeVar6, nodeVar7 );
			nodeVar16 = ( object.nodeUniform12 * vec4<f32>( ( ( vec3<f32>( -1.0, -1.0, -1.0 ) * vec3<f32>( nodeConst11.w ) ) + nodeConst11.xyz ), 1.0 ) );

			if ( ( nodeVar16.w <= 0.0 ) ) {

				nodeVar15 = true;
				

			} else {

				let nodeConst13 = ( nodeVar16.xyz / vec3<f32>( nodeVar16.w ) );
				nodeVar10 = min( nodeVar10, nodeConst13.x );
				nodeVar11 = min( nodeVar11, nodeConst13.y );
				nodeVar12 = min( nodeVar12, nodeConst13.z );
				nodeVar13 = max( nodeVar13, nodeConst13.x );
				nodeVar14 = max( nodeVar14, nodeConst13.y );
				

			}

			nodeVar17 = ( object.nodeUniform12 * vec4<f32>( ( ( vec3<f32>( 1.0, -1.0, -1.0 ) * vec3<f32>( nodeConst11.w ) ) + nodeConst11.xyz ), 1.0 ) );

			if ( ( nodeVar17.w <= 0.0 ) ) {

				nodeVar15 = true;
				

			} else {

				let nodeConst14 = ( nodeVar17.xyz / vec3<f32>( nodeVar17.w ) );
				nodeVar10 = min( nodeVar10, nodeConst14.x );
				nodeVar11 = min( nodeVar11, nodeConst14.y );
				nodeVar12 = min( nodeVar12, nodeConst14.z );
				nodeVar13 = max( nodeVar13, nodeConst14.x );
				nodeVar14 = max( nodeVar14, nodeConst14.y );
				

			}

			nodeVar18 = ( object.nodeUniform12 * vec4<f32>( ( ( vec3<f32>( -1.0, 1.0, -1.0 ) * vec3<f32>( nodeConst11.w ) ) + nodeConst11.xyz ), 1.0 ) );

			if ( ( nodeVar18.w <= 0.0 ) ) {

				nodeVar15 = true;
				

			} else {

				let nodeConst15 = ( nodeVar18.xyz / vec3<f32>( nodeVar18.w ) );
				nodeVar10 = min( nodeVar10, nodeConst15.x );
				nodeVar11 = min( nodeVar11, nodeConst15.y );
				nodeVar12 = min( nodeVar12, nodeConst15.z );
				nodeVar13 = max( nodeVar13, nodeConst15.x );
				nodeVar14 = max( nodeVar14, nodeConst15.y );
				

			}

			nodeVar19 = ( object.nodeUniform12 * vec4<f32>( ( ( vec3<f32>( 1.0, 1.0, -1.0 ) * vec3<f32>( nodeConst11.w ) ) + nodeConst11.xyz ), 1.0 ) );

			if ( ( nodeVar19.w <= 0.0 ) ) {

				nodeVar15 = true;
				

			} else {

				let nodeConst16 = ( nodeVar19.xyz / vec3<f32>( nodeVar19.w ) );
				nodeVar10 = min( nodeVar10, nodeConst16.x );
				nodeVar11 = min( nodeVar11, nodeConst16.y );
				nodeVar12 = min( nodeVar12, nodeConst16.z );
				nodeVar13 = max( nodeVar13, nodeConst16.x );
				nodeVar14 = max( nodeVar14, nodeConst16.y );
				

			}

			nodeVar20 = ( object.nodeUniform12 * vec4<f32>( ( ( vec3<f32>( -1.0, -1.0, 1.0 ) * vec3<f32>( nodeConst11.w ) ) + nodeConst11.xyz ), 1.0 ) );

			if ( ( nodeVar20.w <= 0.0 ) ) {

				nodeVar15 = true;
				

			} else {

				let nodeConst17 = ( nodeVar20.xyz / vec3<f32>( nodeVar20.w ) );
				nodeVar10 = min( nodeVar10, nodeConst17.x );
				nodeVar11 = min( nodeVar11, nodeConst17.y );
				nodeVar12 = min( nodeVar12, nodeConst17.z );
				nodeVar13 = max( nodeVar13, nodeConst17.x );
				nodeVar14 = max( nodeVar14, nodeConst17.y );
				

			}

			nodeVar21 = ( object.nodeUniform12 * vec4<f32>( ( ( vec3<f32>( 1.0, -1.0, 1.0 ) * vec3<f32>( nodeConst11.w ) ) + nodeConst11.xyz ), 1.0 ) );

			if ( ( nodeVar21.w <= 0.0 ) ) {

				nodeVar15 = true;
				

			} else {

				let nodeConst18 = ( nodeVar21.xyz / vec3<f32>( nodeVar21.w ) );
				nodeVar10 = min( nodeVar10, nodeConst18.x );
				nodeVar11 = min( nodeVar11, nodeConst18.y );
				nodeVar12 = min( nodeVar12, nodeConst18.z );
				nodeVar13 = max( nodeVar13, nodeConst18.x );
				nodeVar14 = max( nodeVar14, nodeConst18.y );
				

			}

			nodeVar22 = ( object.nodeUniform12 * vec4<f32>( ( ( vec3<f32>( -1.0, 1.0, 1.0 ) * vec3<f32>( nodeConst11.w ) ) + nodeConst11.xyz ), 1.0 ) );

			if ( ( nodeVar22.w <= 0.0 ) ) {

				nodeVar15 = true;
				

			} else {

				let nodeConst19 = ( nodeVar22.xyz / vec3<f32>( nodeVar22.w ) );
				nodeVar10 = min( nodeVar10, nodeConst19.x );
				nodeVar11 = min( nodeVar11, nodeConst19.y );
				nodeVar12 = min( nodeVar12, nodeConst19.z );
				nodeVar13 = max( nodeVar13, nodeConst19.x );
				nodeVar14 = max( nodeVar14, nodeConst19.y );
				

			}

			nodeVar23 = ( object.nodeUniform12 * vec4<f32>( ( ( vec3<f32>( 1.0, 1.0, 1.0 ) * vec3<f32>( nodeConst11.w ) ) + nodeConst11.xyz ), 1.0 ) );

			if ( ( nodeVar23.w <= 0.0 ) ) {

				nodeVar15 = true;
				

			} else {

				let nodeConst20 = ( nodeVar23.xyz / vec3<f32>( nodeVar23.w ) );
				nodeVar10 = min( nodeVar10, nodeConst20.x );
				nodeVar11 = min( nodeVar11, nodeConst20.y );
				nodeVar12 = min( nodeVar12, nodeConst20.z );
				nodeVar13 = max( nodeVar13, nodeConst20.x );
				nodeVar14 = max( nodeVar14, nodeConst20.y );
				

			}

			nodeVar24 = false;

			if ( ( ! nodeVar15 ) ) {

				let nodeConst21 = i32( clamp( ( ( ( nodeVar13 * 0.5 ) + 0.5 ) * object.nodeUniform15.x ), 0.0, ( object.nodeUniform15.x - 1.0 ) ) );
				let nodeConst22 = i32( clamp( ( ( ( nodeVar10 * 0.5 ) + 0.5 ) * object.nodeUniform15.x ), 0.0, ( object.nodeUniform15.x - 1.0 ) ) );
				let nodeConst23 = i32( clamp( ( ( 0.5 - ( nodeVar11 * 0.5 ) ) * object.nodeUniform15.y ), 0.0, ( object.nodeUniform15.y - 1.0 ) ) );
				let nodeConst24 = i32( clamp( ( ( 0.5 - ( nodeVar14 * 0.5 ) ) * object.nodeUniform15.y ), 0.0, ( object.nodeUniform15.y - 1.0 ) ) );
				let nodeConst25 = clamp( ( i32( ceil( log2( f32( ( max( ( nodeConst21 - nodeConst22 ), ( nodeConst23 - nodeConst24 ) ) + 1 ) ) ) ) ) - 1 ), 0, ( i32( object.nodeUniform16 ) - 1 ) );
				let nodeConst26 = u32( nodeUniform14.value[ nodeConst25 ].x );
				let nodeConst27 = u32( ( nodeConst25 + 1 ) );
				let nodeConst28 = ( u32( nodeConst24 ) >> nodeConst27 );
				let nodeConst29 = u32( nodeUniform14.value[ nodeConst25 ].y );
				let nodeConst30 = ( u32( nodeConst22 ) >> nodeConst27 );
				let nodeConst31 = ( u32( nodeConst21 ) >> nodeConst27 );
				let nodeConst32 = ( u32( nodeConst23 ) >> nodeConst27 );
				nodeVar24 = ( nodeVar12 > max( max( nodeUniform13.value[ ( ( nodeConst26 + ( nodeConst28 * nodeConst29 ) ) + nodeConst30 ) ], nodeUniform13.value[ ( ( nodeConst26 + ( nodeConst28 * nodeConst29 ) ) + nodeConst31 ) ] ), max( nodeUniform13.value[ ( ( nodeConst26 + ( nodeConst32 * nodeConst29 ) ) + nodeConst30 ) ], nodeUniform13.value[ ( ( nodeConst26 + ( nodeConst32 * nodeConst29 ) ) + nodeConst31 ) ] ) ) );
				

			}


			if ( ( ( ! nodeVar24 ) && ( ! nodeVar9 ) ) ) {


				if ( ( ( nodeVar2.y & 512u ) != 0u ) ) {

					nodeVar25 = 1u;

				} else {

					nodeVar25 = 0u;

				}

				let nodeConst33 = atomicAdd( &nodeUniform0.value[ ( ( ( nodeVar4 * 2u ) + 1u ) + nodeVar25 ) ], 1u );
				let nodeConst34 = nodeConst4.w;

				if ( ( nodeConst33 < nodeConst34 ) ) {

					nodeUniform17.value[ ( ( nodeConst4.z + ( nodeVar25 * nodeConst34 ) ) + nodeConst33 ) ] = vec4<u32>( nodeConst5, ( nodeConst7.y + u32( nodeUniform4.value[ ( ( ( nodeVar5 * 8u ) + ( nodeConst1 + 2u ) ) + object.nodeUniform5 ) ].x ) ), nodeConst2, nodeVar3 );
					

				}

				

			}


			if ( ( nodeVar24 && ( ! nodeVar9 ) ) ) {

				atomicAdd( &nodeUniform0.value[ object.nodeUniform8 ], 1u );
				

			}


			if ( nodeVar24 ) {

				nodeVar26 = 0u;

			} else {

				nodeVar26 = nodeConst12;

			}

			nodeUniform6.value[ nodeConst6 ] = nodeVar26;
			

		}

		

	}


	

}
