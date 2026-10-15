// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct NodeBuffer_1008Struct {
	value : array< u32 >
};
@binding( 0 ) @group( 1 )
var<storage, read> NodeBuffer_1008 : NodeBuffer_1008Struct;

struct NodeBuffer_1010Struct {
	value : array< vec4<u32> >
};
@binding( 1 ) @group( 1 )
var<storage, read_write> NodeBuffer_1010 : NodeBuffer_1010Struct;

struct NodeBuffer_1005Struct {
	value : array< mat4x4<f32> >
};
@binding( 2 ) @group( 1 )
var<storage, read_write> NodeBuffer_1005 : NodeBuffer_1005Struct;

struct NodeBuffer_993Struct {
	value : array< vec4<f32> >
};
@binding( 3 ) @group( 1 )
var<storage, read> NodeBuffer_993 : NodeBuffer_993Struct;

struct NodeBuffer_995Struct {
	value : array< u32 >
};
@binding( 4 ) @group( 1 )
var<storage, read> NodeBuffer_995 : NodeBuffer_995Struct;

struct NodeBuffer_999Struct {
	value : array< atomic<u32> >
};
@binding( 6 ) @group( 1 )
var<storage, read_write> NodeBuffer_999 : NodeBuffer_999Struct;

struct NodeBuffer_1001Struct {
	value : array< atomic<u32> >
};
@binding( 7 ) @group( 1 )
var<storage, read_write> NodeBuffer_1001 : NodeBuffer_1001Struct;

struct NodeBuffer_1011Struct {
	value : array< atomic<u32> >
};
@binding( 8 ) @group( 1 )
var<storage, read_write> NodeBuffer_1011 : NodeBuffer_1011Struct;

struct renderStruct {
	nodeUniform5 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform6 : i32
};
@binding( 5 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : u32;
var<private> nodeVar1 : u32;
var<private> nodeVar2 : u32;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec2<f32>;
var<private> nodeVar10 : vec2<f32>;
var<private> nodeVar11 : vec2<f32>;
var<private> nodeVar12 : vec2<f32>;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : u32;
var<private> nodeVar32 : f32;

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


	// flow -> Compute Rasterize

	if ( ( instanceIndex < ( NodeBuffer_1008.value[ 0u ] * 64u ) ) ) {

		nodeVar0 = ( ( NodeBuffer_1010.value[ ( instanceIndex / 64u ) ].w * 64u ) + ( instanceIndex % 64u ) );

		if ( ( nodeVar0 < NodeBuffer_1010.value[ ( instanceIndex / 64u ) ].z ) ) {

			nodeVar1 = ( NodeBuffer_1010.value[ ( instanceIndex / 64u ) ].y + nodeVar0 );
			nodeVar2 = ( nodeVar1 * 3u );
			nodeVar3 = ( NodeBuffer_1005.value[ NodeBuffer_1010.value[ ( instanceIndex / 64u ) ].x ] * NodeBuffer_993.value[ NodeBuffer_995.value[ nodeVar2 ] ] );
			nodeVar4 = ( NodeBuffer_1005.value[ NodeBuffer_1010.value[ ( instanceIndex / 64u ) ].x ] * NodeBuffer_993.value[ NodeBuffer_995.value[ ( nodeVar2 + 1u ) ] ] );
			nodeVar5 = ( NodeBuffer_1005.value[ NodeBuffer_1010.value[ ( instanceIndex / 64u ) ].x ] * NodeBuffer_993.value[ NodeBuffer_995.value[ ( nodeVar2 + 2u ) ] ] );

			if ( ( ( ( nodeVar3.w > 0.0 ) && ( nodeVar4.w > 0.0 ) ) && ( nodeVar5.w > 0.0 ) ) ) {

				nodeVar6 = ( nodeVar5.xyz / vec3<f32>( nodeVar5.w ) );
				nodeVar7 = ( nodeVar3.xyz / vec3<f32>( nodeVar3.w ) );
				nodeVar8 = ( nodeVar4.xyz / vec3<f32>( nodeVar4.w ) );

				if ( ( ( ( ( nodeVar6.y - nodeVar7.y ) * ( nodeVar8.x - nodeVar7.x ) ) - ( ( nodeVar6.x - nodeVar7.x ) * ( nodeVar8.y - nodeVar7.y ) ) ) > 0.0 ) ) {


					if ( ( ( ( ( max( nodeVar7.x, max( nodeVar8.x, nodeVar6.x ) ) > -1.0 ) && ( min( nodeVar7.x, min( nodeVar8.x, nodeVar6.x ) ) < 1.0 ) ) && ( max( nodeVar7.y, max( nodeVar8.y, nodeVar6.y ) ) > -1.0 ) ) && ( min( nodeVar7.y, min( nodeVar8.y, nodeVar6.y ) ) < 1.0 ) ) ) {

						nodeVar9 = ( ( ( nodeVar7.xy + vec2<f32>( 1.0 ) ) * vec2<f32>( 0.5 ) ) * vec2<f32>( render.nodeUniform5.x, render.nodeUniform5.y ) );
						nodeVar10 = ( ( ( nodeVar8.xy + vec2<f32>( 1.0 ) ) * vec2<f32>( 0.5 ) ) * vec2<f32>( render.nodeUniform5.x, render.nodeUniform5.y ) );
						nodeVar11 = ( ( ( nodeVar6.xy + vec2<f32>( 1.0 ) ) * vec2<f32>( 0.5 ) ) * vec2<f32>( render.nodeUniform5.x, render.nodeUniform5.y ) );

						if ( ( ( ( ( i32( floor( max( 0.0, min( nodeVar9.x, min( nodeVar10.x, nodeVar11.x ) ) ) ) ) <= i32( floor( min( ( render.nodeUniform5.x - 1.0 ), max( nodeVar9.x, max( nodeVar10.x, nodeVar11.x ) ) ) ) ) ) && ( i32( floor( max( 0.0, min( nodeVar9.y, min( nodeVar10.y, nodeVar11.y ) ) ) ) ) <= i32( floor( min( ( render.nodeUniform5.y - 1.0 ), max( nodeVar9.y, max( nodeVar10.y, nodeVar11.y ) ) ) ) ) ) ) && ( ( i32( floor( min( ( render.nodeUniform5.x - 1.0 ), max( nodeVar9.x, max( nodeVar10.x, nodeVar11.x ) ) ) ) ) - i32( floor( max( 0.0, min( nodeVar9.x, min( nodeVar10.x, nodeVar11.x ) ) ) ) ) ) <= object.nodeUniform6 ) ) && ( ( i32( floor( min( ( render.nodeUniform5.y - 1.0 ), max( nodeVar9.y, max( nodeVar10.y, nodeVar11.y ) ) ) ) ) - i32( floor( max( 0.0, min( nodeVar9.y, min( nodeVar10.y, nodeVar11.y ) ) ) ) ) ) <= object.nodeUniform6 ) ) ) {

							nodeVar12 = vec2<f32>( ( f32( i32( floor( max( 0.0, min( nodeVar9.x, min( nodeVar10.x, nodeVar11.x ) ) ) ) ) ) + 0.5 ), ( f32( i32( floor( max( 0.0, min( nodeVar9.y, min( nodeVar10.y, nodeVar11.y ) ) ) ) ) ) + 0.5 ) );
							nodeVar13 = ( ( ( nodeVar12.y - nodeVar10.y ) * ( nodeVar11.x - nodeVar10.x ) ) - ( ( nodeVar12.x - nodeVar10.x ) * ( nodeVar11.y - nodeVar10.y ) ) );
							nodeVar14 = ( ( ( nodeVar12.y - nodeVar11.y ) * ( nodeVar9.x - nodeVar11.x ) ) - ( ( nodeVar12.x - nodeVar11.x ) * ( nodeVar9.y - nodeVar11.y ) ) );
							nodeVar15 = ( ( ( nodeVar12.y - nodeVar9.y ) * ( nodeVar10.x - nodeVar9.x ) ) - ( ( nodeVar12.x - nodeVar9.x ) * ( nodeVar10.y - nodeVar9.y ) ) );
							nodeVar17 = ( nodeVar10.y - nodeVar11.y );
							nodeVar18 = ( nodeVar11.x - nodeVar10.x );

							if ( ( ( nodeVar17 < 0.0 ) || ( ( nodeVar17 == 0.0 ) && ( nodeVar18 > 0.0 ) ) ) ) {

								nodeVar16 = 0.0;

							} else {

								nodeVar16 = -0.00001;

							}

							nodeVar13 = ( nodeVar13 + nodeVar16 );
							nodeVar20 = ( nodeVar11.y - nodeVar9.y );
							nodeVar21 = ( nodeVar9.x - nodeVar11.x );

							if ( ( ( nodeVar20 < 0.0 ) || ( ( nodeVar20 == 0.0 ) && ( nodeVar21 > 0.0 ) ) ) ) {

								nodeVar19 = 0.0;

							} else {

								nodeVar19 = -0.00001;

							}

							nodeVar14 = ( nodeVar14 + nodeVar19 );
							nodeVar23 = ( nodeVar9.y - nodeVar10.y );
							nodeVar24 = ( nodeVar10.x - nodeVar9.x );

							if ( ( ( nodeVar23 < 0.0 ) || ( ( nodeVar23 == 0.0 ) && ( nodeVar24 > 0.0 ) ) ) ) {

								nodeVar22 = 0.0;

							} else {

								nodeVar22 = -0.00001;

							}

							nodeVar15 = ( nodeVar15 + nodeVar22 );
							nodeVar25 = ( ( ( nodeVar11.y - nodeVar9.y ) * ( nodeVar10.x - nodeVar9.x ) ) - ( ( nodeVar11.x - nodeVar9.x ) * ( nodeVar10.y - nodeVar9.y ) ) );
							nodeVar26 = ( ( ( ( nodeVar13 / nodeVar25 ) * nodeVar7.z ) + ( ( nodeVar14 / nodeVar25 ) * nodeVar8.z ) ) + ( ( nodeVar15 / nodeVar25 ) * nodeVar6.z ) );

							for ( var y : i32 = i32( floor( max( 0.0, min( nodeVar9.y, min( nodeVar10.y, nodeVar11.y ) ) ) ) ); y <= i32( floor( min( ( render.nodeUniform5.y - 1.0 ), max( nodeVar9.y, max( nodeVar10.y, nodeVar11.y ) ) ) ) ); y ++ ) {

								nodeVar27 = nodeVar13;
								nodeVar28 = nodeVar14;
								nodeVar29 = nodeVar15;
								nodeVar30 = nodeVar26;

								for ( var x : i32 = i32( floor( max( 0.0, min( nodeVar9.x, min( nodeVar10.x, nodeVar11.x ) ) ) ) ); x <= i32( floor( min( ( render.nodeUniform5.x - 1.0 ), max( nodeVar9.x, max( nodeVar10.x, nodeVar11.x ) ) ) ) ); x ++ ) {


									if ( ( ( ( nodeVar27 >= 0.0 ) && ( nodeVar28 >= 0.0 ) ) && ( nodeVar29 >= 0.0 ) ) ) {


										if ( ( ( nodeVar30 >= 0.0 ) && ( nodeVar30 <= 1.0 ) ) ) {

											nodeVar31 = ( ( u32( y ) * u32( render.nodeUniform5.x ) ) + u32( x ) );
											let nodeConst0 = atomicLoad( &NodeBuffer_999.value[ nodeVar31 ] );
											nodeVar32 = sqrt( sqrt( ( 1.0 - nodeVar30 ) ) );

											if ( ( u32( ( nodeVar32 * 262143.0 ) ) >= ( nodeConst0 >> 14u ) ) ) {

												atomicMax( &NodeBuffer_999.value[ nodeVar31 ], ( ( u32( ( nodeVar32 * 262143.0 ) ) << 14u ) | ( nodeVar1 & 16383u ) ) );
												atomicMax( &NodeBuffer_1001.value[ nodeVar31 ], ( ( u32( ( nodeVar32 * 16383.0 ) ) << 18u ) | NodeBuffer_1010.value[ ( instanceIndex / 64u ) ].x ) );
												

											}

											

										}

										

									}

									nodeVar27 = ( nodeVar27 + nodeVar17 );
									nodeVar28 = ( nodeVar28 + nodeVar20 );
									nodeVar29 = ( nodeVar29 + nodeVar23 );
									nodeVar30 = ( nodeVar30 + ( ( ( ( nodeVar17 / nodeVar25 ) * nodeVar7.z ) + ( ( nodeVar20 / nodeVar25 ) * nodeVar8.z ) ) + ( ( nodeVar23 / nodeVar25 ) * nodeVar6.z ) ) );

								}

								nodeVar13 = ( nodeVar13 + nodeVar18 );
								nodeVar14 = ( nodeVar14 + nodeVar21 );
								nodeVar15 = ( nodeVar15 + nodeVar24 );
								nodeVar26 = ( nodeVar26 + ( ( ( ( nodeVar18 / nodeVar25 ) * nodeVar7.z ) + ( ( nodeVar21 / nodeVar25 ) * nodeVar8.z ) ) + ( ( nodeVar24 / nodeVar25 ) * nodeVar6.z ) ) );

							}

							

						} else {


							if ( ( ( i32( floor( max( 0.0, min( nodeVar9.x, min( nodeVar10.x, nodeVar11.x ) ) ) ) ) <= i32( floor( min( ( render.nodeUniform5.x - 1.0 ), max( nodeVar9.x, max( nodeVar10.x, nodeVar11.x ) ) ) ) ) ) && ( i32( floor( max( 0.0, min( nodeVar9.y, min( nodeVar10.y, nodeVar11.y ) ) ) ) ) <= i32( floor( min( ( render.nodeUniform5.y - 1.0 ), max( nodeVar9.y, max( nodeVar10.y, nodeVar11.y ) ) ) ) ) ) ) ) {

								let nodeConst1 = atomicAdd( &NodeBuffer_1011.value[ 0u ], 1u );
								atomicStore( &NodeBuffer_1011.value[ ( nodeConst1 + 1u ) ], ( ( NodeBuffer_1010.value[ ( instanceIndex / 64u ) ].x << 14u ) | ( nodeVar1 & 16383u ) ) );
								

							}

							

						}

						

					}

					

				}

				

			}

			

		}

		

	}


	

}
