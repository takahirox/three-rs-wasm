// Three.js r187 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct nodeUniform1Struct {
	value : array< vec4<f32> >
};
@binding( 1 ) @group( 0 )
var<storage, read> nodeUniform1 : nodeUniform1Struct;

struct nodeUniform6Struct {
	value : array< atomic<u32> >
};
@binding( 2 ) @group( 0 )
var<storage, read_write> nodeUniform6 : nodeUniform6Struct;

struct nodeUniform7Struct {
	value : array< u32 >
};
@binding( 3 ) @group( 0 )
var<storage, read_write> nodeUniform7 : nodeUniform7Struct;

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
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<i32>;
var<private> nodeVar11 : vec3<i32>;

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
		let nodeConst1 = ( 2.0 / object.nodeUniform3 );
		let nodeConst2 = ( ( nodeUniform1.value[ nodeConst0 ].xyz - object.nodeUniform2 ) * vec3<f32>( nodeConst1 ) );
		let nodeConst3 = ( ( nodeUniform1.value[ ( nodeConst0 + 1u ) ].xyz - object.nodeUniform2 ) * vec3<f32>( nodeConst1 ) );
		let nodeConst4 = ( ( nodeUniform1.value[ ( nodeConst0 + 2u ) ].xyz - object.nodeUniform2 ) * vec3<f32>( nodeConst1 ) );
		let nodeConst5 = cross( ( nodeConst3 - nodeConst2 ), ( nodeConst4 - nodeConst2 ) );
		let nodeConst6 = abs( nodeConst5 );
		let nodeConst7 = ( ( nodeConst6.z >= nodeConst6.x ) && ( nodeConst6.z >= nodeConst6.y ) );
		let nodeConst8 = ( ( ! nodeConst7 ) && ( nodeConst6.y >= nodeConst6.x ) );

		if ( nodeConst7 ) {

			nodeVar0 = nodeConst2;

		} else {


			if ( nodeConst8 ) {

				nodeVar1 = nodeConst2.zxy;

			} else {

				nodeVar1 = nodeConst2.yzx;

			}

			nodeVar0 = nodeVar1;

		}

		let nodeConst9 = nodeVar0;

		if ( nodeConst7 ) {

			nodeVar2 = nodeConst3;

		} else {


			if ( nodeConst8 ) {

				nodeVar3 = nodeConst3.zxy;

			} else {

				nodeVar3 = nodeConst3.yzx;

			}

			nodeVar2 = nodeVar3;

		}

		let nodeConst10 = nodeVar2;

		if ( nodeConst7 ) {

			nodeVar4 = nodeConst4;

		} else {


			if ( nodeConst8 ) {

				nodeVar5 = nodeConst4.zxy;

			} else {

				nodeVar5 = nodeConst4.yzx;

			}

			nodeVar4 = nodeVar5;

		}

		let nodeConst11 = nodeVar4;

		if ( nodeConst7 ) {

			nodeVar6 = nodeConst5;

		} else {


			if ( nodeConst8 ) {

				nodeVar7 = nodeConst5.zxy;

			} else {

				nodeVar7 = nodeConst5.yzx;

			}

			nodeVar6 = nodeVar7;

		}

		let nodeConst12 = nodeVar6;

		if ( nodeConst7 ) {

			nodeVar8 = vec3<f32>( 144.0, 112.0, 144.0 );

		} else {


			if ( nodeConst8 ) {

				nodeVar9 = vec3<f32>( 144.0, 112.0, 144.0 ).zxy;

			} else {

				nodeVar9 = vec3<f32>( 144.0, 112.0, 144.0 ).yzx;

			}

			nodeVar8 = nodeVar9;

		}

		let nodeConst13 = nodeVar8;
		let nodeConst14 = min( nodeConst9, min( nodeConst10, nodeConst11 ) );
		let nodeConst15 = max( nodeConst9, max( nodeConst10, nodeConst11 ) );
		let nodeConst16 = max( i32( floor( nodeConst14.x ) ), 0 );
		let nodeConst17 = min( i32( floor( nodeConst15.x ) ), ( i32( nodeConst13.x ) - 1 ) );
		let nodeConst18 = max( i32( floor( nodeConst14.y ) ), 0 );
		let nodeConst19 = min( i32( floor( nodeConst15.y ) ), ( i32( nodeConst13.y ) - 1 ) );
		let nodeConst20 = sign( nodeConst12.z );
		let nodeConst21 = ( vec2<f32>( ( nodeConst9.xy.y - nodeConst10.xy.y ), ( nodeConst10.xy.x - nodeConst9.xy.x ) ) * vec2<f32>( nodeConst20 ) );
		let nodeConst22 = ( 0.5 * ( abs( nodeConst21.x ) + abs( nodeConst21.y ) ) );
		let nodeConst23 = ( vec2<f32>( ( nodeConst10.xy.y - nodeConst11.xy.y ), ( nodeConst11.xy.x - nodeConst10.xy.x ) ) * vec2<f32>( nodeConst20 ) );
		let nodeConst24 = ( 0.5 * ( abs( nodeConst23.x ) + abs( nodeConst23.y ) ) );
		let nodeConst25 = ( vec2<f32>( ( nodeConst11.xy.y - nodeConst9.xy.y ), ( nodeConst9.xy.x - nodeConst11.xy.x ) ) * vec2<f32>( nodeConst20 ) );
		let nodeConst26 = ( 0.5 * ( abs( nodeConst25.x ) + abs( nodeConst25.y ) ) );
		let nodeConst27 = ( ( 0.5 * ( abs( nodeConst12.x ) + abs( nodeConst12.y ) ) ) / abs( nodeConst12.z ) );

		for ( var i : i32 = nodeConst16; i <= nodeConst17; i ++ ) {

			for ( var j : i32 = nodeConst18; j <= nodeConst19; j ++ ) {

				let nodeConst28 = vec2<f32>( ( f32( i ) + 0.5 ), ( f32( j ) + 0.5 ) );

				if ( ( ( ( ( dot( nodeConst21, ( nodeConst28 - nodeConst9.xy ) ) + nodeConst22 ) >= 0.0 ) && ( ( dot( nodeConst23, ( nodeConst28 - nodeConst10.xy ) ) + nodeConst24 ) >= 0.0 ) ) && ( ( dot( nodeConst25, ( nodeConst28 - nodeConst11.xy ) ) + nodeConst26 ) >= 0.0 ) ) ) {

					let nodeConst29 = ( nodeConst9.z - ( ( ( nodeConst12.x * ( nodeConst28.x - nodeConst9.x ) ) + ( nodeConst12.y * ( nodeConst28.y - nodeConst9.y ) ) ) / nodeConst12.z ) );
					let nodeConst30 = max( i32( floor( max( ( nodeConst29 - nodeConst27 ), nodeConst14.z ) ) ), 0 );
					let nodeConst31 = min( i32( floor( min( ( nodeConst29 + nodeConst27 ), nodeConst15.z ) ) ), ( i32( nodeConst13.z ) - 1 ) );

					for ( var k : i32 = nodeConst30; k <= nodeConst31; k ++ ) {


						if ( nodeConst7 ) {

							nodeVar10 = vec3<i32>( i, j, k );

						} else {


							if ( nodeConst8 ) {

								nodeVar11 = vec3<i32>( j, k, i );

							} else {

								nodeVar11 = vec3<i32>( k, i, j );

							}

							nodeVar10 = nodeVar11;

						}

						let nodeConst32 = nodeVar10;
						let nodeConst33 = vec3<u32>( ( nodeConst32 / vec3<i32>( i32( 2.0 ) ) ) );
						let nodeConst34 = ( ( u32( ( nodeConst32.x & 1 ) ) | ( u32( ( nodeConst32.y & 1 ) ) << 1u ) ) | ( u32( ( nodeConst32.z & 1 ) ) << 2u ) );
						let nodeConst35 = ( nodeConst33.x + ( object.nodeUniform4 * ( nodeConst33.y + ( object.nodeUniform5 * nodeConst33.z ) ) ) );
						atomicOr( &nodeUniform6.value[ nodeConst35 ], ( 1u << nodeConst34 ) );
						nodeUniform7.value[ nodeConst35 ] = ( instanceIndex + 1u );

					}

					

				}


			}

		}

		

	}


	

}
