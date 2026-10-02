// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct NodeBuffer_5318Struct {
	value : array< vec4<f32> >
};
@binding( 1 ) @group( 0 )
var<storage, read> NodeBuffer_5318 : NodeBuffer_5318Struct;

struct NodeBuffer_5326Struct {
	value : array< atomic<u32> >
};
@binding( 2 ) @group( 0 )
var<storage, read_write> NodeBuffer_5326 : NodeBuffer_5326Struct;

struct NodeBuffer_5327Struct {
	value : array< u32 >
};
@binding( 3 ) @group( 0 )
var<storage, read_write> NodeBuffer_5327 : NodeBuffer_5327Struct;

struct objectStruct {
	nodeUniform0 : u32,
	nodeUniform2 : vec3<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : u32,
	nodeUniform5 : u32,
	nodeUniform8 : u32
};
@binding( 0 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : f32;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<i32>;
var<private> nodeVar12 : vec3<i32>;

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


	// flow -> VXGI.Voxelize
	if ( instanceIndex >= object.nodeUniform8 ) { return; }


	if ( ( instanceIndex < object.nodeUniform0 ) ) {

		let nodeConst0 = ( instanceIndex * 5u );
		nodeVar0 = ( 2.0 / object.nodeUniform3 );
		let nodeConst1 = ( ( NodeBuffer_5318.value[ nodeConst0 ].xyz - object.nodeUniform2 ) * vec3<f32>( nodeVar0 ) );
		let nodeConst2 = ( ( NodeBuffer_5318.value[ ( nodeConst0 + 1u ) ].xyz - object.nodeUniform2 ) * vec3<f32>( nodeVar0 ) );
		let nodeConst3 = ( ( NodeBuffer_5318.value[ ( nodeConst0 + 2u ) ].xyz - object.nodeUniform2 ) * vec3<f32>( nodeVar0 ) );
		let nodeConst4 = cross( ( nodeConst2 - nodeConst1 ), ( nodeConst3 - nodeConst1 ) );
		let nodeConst5 = abs( nodeConst4 );
		let nodeConst6 = ( ( nodeConst5.z >= nodeConst5.x ) && ( nodeConst5.z >= nodeConst5.y ) );
		let nodeConst7 = ( ( ! nodeConst6 ) && ( nodeConst5.y >= nodeConst5.x ) );

		if ( nodeConst6 ) {

			nodeVar1 = nodeConst1;

		} else {


			if ( nodeConst7 ) {

				nodeVar2 = nodeConst1.zxy;

			} else {

				nodeVar2 = nodeConst1.yzx;

			}

			nodeVar1 = nodeVar2;

		}

		let nodeConst8 = nodeVar1;

		if ( nodeConst6 ) {

			nodeVar3 = nodeConst2;

		} else {


			if ( nodeConst7 ) {

				nodeVar4 = nodeConst2.zxy;

			} else {

				nodeVar4 = nodeConst2.yzx;

			}

			nodeVar3 = nodeVar4;

		}

		let nodeConst9 = nodeVar3;

		if ( nodeConst6 ) {

			nodeVar5 = nodeConst3;

		} else {


			if ( nodeConst7 ) {

				nodeVar6 = nodeConst3.zxy;

			} else {

				nodeVar6 = nodeConst3.yzx;

			}

			nodeVar5 = nodeVar6;

		}

		let nodeConst10 = nodeVar5;

		if ( nodeConst6 ) {

			nodeVar7 = nodeConst4;

		} else {


			if ( nodeConst7 ) {

				nodeVar8 = nodeConst4.zxy;

			} else {

				nodeVar8 = nodeConst4.yzx;

			}

			nodeVar7 = nodeVar8;

		}

		let nodeConst11 = nodeVar7;

		if ( nodeConst6 ) {

			nodeVar9 = vec3<f32>( 144.0, 112.0, 144.0 );

		} else {


			if ( nodeConst7 ) {

				nodeVar10 = vec3<f32>( 144.0, 112.0, 144.0 ).zxy;

			} else {

				nodeVar10 = vec3<f32>( 144.0, 112.0, 144.0 ).yzx;

			}

			nodeVar9 = nodeVar10;

		}

		let nodeConst12 = nodeVar9;
		let nodeConst13 = min( nodeConst8, min( nodeConst9, nodeConst10 ) );
		let nodeConst14 = max( nodeConst8, max( nodeConst9, nodeConst10 ) );
		let nodeConst15 = max( f32( i32( floor( nodeConst13.x ) ) ), 0.0 );
		let nodeConst16 = min( i32( floor( nodeConst14.x ) ), ( i32( nodeConst12.x ) - 1 ) );
		let nodeConst17 = max( f32( i32( floor( nodeConst13.y ) ) ), 0.0 );
		let nodeConst18 = min( i32( floor( nodeConst14.y ) ), ( i32( nodeConst12.y ) - 1 ) );
		let nodeConst19 = sign( nodeConst11.z );
		let nodeConst20 = ( vec2<f32>( ( nodeConst8.xy.y - nodeConst9.xy.y ), ( nodeConst9.xy.x - nodeConst8.xy.x ) ) * vec2<f32>( nodeConst19 ) );
		let nodeConst21 = ( 0.5 * ( abs( nodeConst20.x ) + abs( nodeConst20.y ) ) );
		let nodeConst22 = ( vec2<f32>( ( nodeConst9.xy.y - nodeConst10.xy.y ), ( nodeConst10.xy.x - nodeConst9.xy.x ) ) * vec2<f32>( nodeConst19 ) );
		let nodeConst23 = ( 0.5 * ( abs( nodeConst22.x ) + abs( nodeConst22.y ) ) );
		let nodeConst24 = ( vec2<f32>( ( nodeConst10.xy.y - nodeConst8.xy.y ), ( nodeConst8.xy.x - nodeConst10.xy.x ) ) * vec2<f32>( nodeConst19 ) );
		let nodeConst25 = ( 0.5 * ( abs( nodeConst24.x ) + abs( nodeConst24.y ) ) );
		let nodeConst26 = ( ( 0.5 * ( abs( nodeConst11.x ) + abs( nodeConst11.y ) ) ) / abs( nodeConst11.z ) );

		for ( var i : i32 = i32( nodeConst15 ); i <= nodeConst16; i ++ ) {

			for ( var j : i32 = i32( nodeConst17 ); j <= nodeConst18; j ++ ) {

				let nodeConst27 = vec2<f32>( ( f32( i ) + 0.5 ), ( f32( j ) + 0.5 ) );

				if ( ( ( ( ( dot( nodeConst20, ( nodeConst27 - nodeConst8.xy ) ) + nodeConst21 ) >= 0.0 ) && ( ( dot( nodeConst22, ( nodeConst27 - nodeConst9.xy ) ) + nodeConst23 ) >= 0.0 ) ) && ( ( dot( nodeConst24, ( nodeConst27 - nodeConst10.xy ) ) + nodeConst25 ) >= 0.0 ) ) ) {

					let nodeConst28 = ( nodeConst8.z - ( ( ( nodeConst11.x * ( nodeConst27.x - nodeConst8.x ) ) + ( nodeConst11.y * ( nodeConst27.y - nodeConst8.y ) ) ) / nodeConst11.z ) );
					let nodeConst29 = max( f32( i32( floor( max( ( nodeConst28 - nodeConst26 ), nodeConst13.z ) ) ) ), 0.0 );
					let nodeConst30 = min( i32( floor( min( ( nodeConst28 + nodeConst26 ), nodeConst14.z ) ) ), ( i32( nodeConst12.z ) - 1 ) );

					for ( var k : i32 = i32( nodeConst29 ); k <= nodeConst30; k ++ ) {


						if ( nodeConst6 ) {

							nodeVar11 = vec3<i32>( i, j, k );

						} else {


							if ( nodeConst7 ) {

								nodeVar12 = vec3<i32>( j, k, i );

							} else {

								nodeVar12 = vec3<i32>( k, i, j );

							}

							nodeVar11 = nodeVar12;

						}

						let nodeConst31 = nodeVar11;
						let nodeConst32 = vec3<u32>( ( nodeConst31 / vec3<i32>( i32( 2.0 ) ) ) );
						let nodeConst33 = ( ( u32( ( nodeConst31.x & 1 ) ) | ( u32( ( nodeConst31.y & 1 ) ) << 1u ) ) | ( u32( ( nodeConst31.z & 1 ) ) << 2u ) );
						let nodeConst34 = ( nodeConst32.x + ( object.nodeUniform4 * ( nodeConst32.y + ( object.nodeUniform5 * nodeConst32.z ) ) ) );
						atomicOr( &NodeBuffer_5326.value[ nodeConst34 ], ( 1u << nodeConst33 ) );
						NodeBuffer_5327.value[ nodeConst34 ] = ( instanceIndex + 1u );

					}

					

				}


			}

		}

		

	}


	

}
