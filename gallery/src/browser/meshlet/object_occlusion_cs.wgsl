// Three.js r187 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct nodeUniform0Struct {
	value : array< vec4<u32> >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> nodeUniform0 : nodeUniform0Struct;

struct nodeUniform1Struct {
	value : array< vec4<f32> >
};
@binding( 1 ) @group( 0 )
var<storage, read> nodeUniform1 : nodeUniform1Struct;

struct nodeUniform4Struct {
	value : array< mat4x4<f32> >
};
@binding( 3 ) @group( 0 )
var<storage, read> nodeUniform4 : nodeUniform4Struct;

struct nodeUniform6Struct {
	value : array< vec4<f32> >
};
@binding( 4 ) @group( 0 )
var<storage, read> nodeUniform6 : nodeUniform6Struct;

struct nodeUniform7Struct {
	value : array< f32 >
};
@binding( 5 ) @group( 0 )
var<storage, read> nodeUniform7 : nodeUniform7Struct;

struct nodeUniform8Struct {
	value : array< vec4<f32>, 16 >
};
@binding( 6 ) @group( 0 )
var<uniform> nodeUniform8 : nodeUniform8Struct;

struct objectStruct {
	nodeUniform2 : u32,
	nodeUniform3 : mat4x4<f32>,
	nodeUniform5 : u32,
	nodeUniform9 : vec2<f32>,
	nodeUniform10 : u32,
	nodeUniform11 : u32
};
@binding( 2 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec4<u32>;
var<private> nodeVar1 : u32;
var<private> nodeVar2 : u32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : bool;
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : bool;

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


	// flow -> MeshletDrawingPipeline Object Occlusion
	if ( instanceIndex >= object.nodeUniform11 ) { return; }

	nodeVar0 = nodeUniform0.value[ instanceIndex ];

	if ( ( ( nodeVar0.y & 255u ) != 255u ) ) {

		nodeVar1 = ( nodeVar0.y >> 16u );
		nodeVar2 = ( bitcast<vec4<u32>>( nodeUniform1.value[ ( ( ( nodeVar1 * 3u ) + 1u ) + object.nodeUniform2 ) ] ).x + ( instanceIndex - bitcast<vec4<u32>>( nodeUniform1.value[ ( ( ( nodeVar1 * 3u ) + 0u ) + object.nodeUniform2 ) ] ).x ) );
		nodeVar3 = 1000000000.0;
		nodeVar4 = 1000000000.0;
		nodeVar5 = 1000000000.0;
		nodeVar6 = -1000000000.0;
		nodeVar7 = -1000000000.0;
		nodeVar8 = false;
		let nodeConst0 = vec4<f32>( ( nodeUniform4.value[ nodeVar2 ] * vec4<f32>( nodeUniform1.value[ ( ( ( ( nodeVar0.w >> 16u ) * 8u ) + 0u ) + object.nodeUniform5 ) ].xyz, 1.0 ) ).xyz, ( ( abs( nodeUniform6.value[ nodeVar2 ].w ) * nodeUniform1.value[ ( ( ( ( nodeVar0.w >> 16u ) * 8u ) + 0u ) + object.nodeUniform5 ) ].w ) + nodeUniform6.value[ nodeVar2 ].x ) );
		nodeVar9 = ( object.nodeUniform3 * vec4<f32>( ( ( vec3<f32>( -1.0, -1.0, -1.0 ) * vec3<f32>( nodeConst0.w ) ) + nodeConst0.xyz ), 1.0 ) );

		if ( ( nodeVar9.w <= 0.0 ) ) {

			nodeVar8 = true;
			

		} else {

			let nodeConst1 = ( nodeVar9.xyz / vec3<f32>( nodeVar9.w ) );
			nodeVar3 = min( nodeVar3, nodeConst1.x );
			nodeVar4 = min( nodeVar4, nodeConst1.y );
			nodeVar5 = min( nodeVar5, nodeConst1.z );
			nodeVar6 = max( nodeVar6, nodeConst1.x );
			nodeVar7 = max( nodeVar7, nodeConst1.y );
			

		}

		nodeVar10 = ( object.nodeUniform3 * vec4<f32>( ( ( vec3<f32>( 1.0, -1.0, -1.0 ) * vec3<f32>( nodeConst0.w ) ) + nodeConst0.xyz ), 1.0 ) );

		if ( ( nodeVar10.w <= 0.0 ) ) {

			nodeVar8 = true;
			

		} else {

			let nodeConst2 = ( nodeVar10.xyz / vec3<f32>( nodeVar10.w ) );
			nodeVar3 = min( nodeVar3, nodeConst2.x );
			nodeVar4 = min( nodeVar4, nodeConst2.y );
			nodeVar5 = min( nodeVar5, nodeConst2.z );
			nodeVar6 = max( nodeVar6, nodeConst2.x );
			nodeVar7 = max( nodeVar7, nodeConst2.y );
			

		}

		nodeVar11 = ( object.nodeUniform3 * vec4<f32>( ( ( vec3<f32>( -1.0, 1.0, -1.0 ) * vec3<f32>( nodeConst0.w ) ) + nodeConst0.xyz ), 1.0 ) );

		if ( ( nodeVar11.w <= 0.0 ) ) {

			nodeVar8 = true;
			

		} else {

			let nodeConst3 = ( nodeVar11.xyz / vec3<f32>( nodeVar11.w ) );
			nodeVar3 = min( nodeVar3, nodeConst3.x );
			nodeVar4 = min( nodeVar4, nodeConst3.y );
			nodeVar5 = min( nodeVar5, nodeConst3.z );
			nodeVar6 = max( nodeVar6, nodeConst3.x );
			nodeVar7 = max( nodeVar7, nodeConst3.y );
			

		}

		nodeVar12 = ( object.nodeUniform3 * vec4<f32>( ( ( vec3<f32>( 1.0, 1.0, -1.0 ) * vec3<f32>( nodeConst0.w ) ) + nodeConst0.xyz ), 1.0 ) );

		if ( ( nodeVar12.w <= 0.0 ) ) {

			nodeVar8 = true;
			

		} else {

			let nodeConst4 = ( nodeVar12.xyz / vec3<f32>( nodeVar12.w ) );
			nodeVar3 = min( nodeVar3, nodeConst4.x );
			nodeVar4 = min( nodeVar4, nodeConst4.y );
			nodeVar5 = min( nodeVar5, nodeConst4.z );
			nodeVar6 = max( nodeVar6, nodeConst4.x );
			nodeVar7 = max( nodeVar7, nodeConst4.y );
			

		}

		nodeVar13 = ( object.nodeUniform3 * vec4<f32>( ( ( vec3<f32>( -1.0, -1.0, 1.0 ) * vec3<f32>( nodeConst0.w ) ) + nodeConst0.xyz ), 1.0 ) );

		if ( ( nodeVar13.w <= 0.0 ) ) {

			nodeVar8 = true;
			

		} else {

			let nodeConst5 = ( nodeVar13.xyz / vec3<f32>( nodeVar13.w ) );
			nodeVar3 = min( nodeVar3, nodeConst5.x );
			nodeVar4 = min( nodeVar4, nodeConst5.y );
			nodeVar5 = min( nodeVar5, nodeConst5.z );
			nodeVar6 = max( nodeVar6, nodeConst5.x );
			nodeVar7 = max( nodeVar7, nodeConst5.y );
			

		}

		nodeVar14 = ( object.nodeUniform3 * vec4<f32>( ( ( vec3<f32>( 1.0, -1.0, 1.0 ) * vec3<f32>( nodeConst0.w ) ) + nodeConst0.xyz ), 1.0 ) );

		if ( ( nodeVar14.w <= 0.0 ) ) {

			nodeVar8 = true;
			

		} else {

			let nodeConst6 = ( nodeVar14.xyz / vec3<f32>( nodeVar14.w ) );
			nodeVar3 = min( nodeVar3, nodeConst6.x );
			nodeVar4 = min( nodeVar4, nodeConst6.y );
			nodeVar5 = min( nodeVar5, nodeConst6.z );
			nodeVar6 = max( nodeVar6, nodeConst6.x );
			nodeVar7 = max( nodeVar7, nodeConst6.y );
			

		}

		nodeVar15 = ( object.nodeUniform3 * vec4<f32>( ( ( vec3<f32>( -1.0, 1.0, 1.0 ) * vec3<f32>( nodeConst0.w ) ) + nodeConst0.xyz ), 1.0 ) );

		if ( ( nodeVar15.w <= 0.0 ) ) {

			nodeVar8 = true;
			

		} else {

			let nodeConst7 = ( nodeVar15.xyz / vec3<f32>( nodeVar15.w ) );
			nodeVar3 = min( nodeVar3, nodeConst7.x );
			nodeVar4 = min( nodeVar4, nodeConst7.y );
			nodeVar5 = min( nodeVar5, nodeConst7.z );
			nodeVar6 = max( nodeVar6, nodeConst7.x );
			nodeVar7 = max( nodeVar7, nodeConst7.y );
			

		}

		nodeVar16 = ( object.nodeUniform3 * vec4<f32>( ( ( vec3<f32>( 1.0, 1.0, 1.0 ) * vec3<f32>( nodeConst0.w ) ) + nodeConst0.xyz ), 1.0 ) );

		if ( ( nodeVar16.w <= 0.0 ) ) {

			nodeVar8 = true;
			

		} else {

			let nodeConst8 = ( nodeVar16.xyz / vec3<f32>( nodeVar16.w ) );
			nodeVar3 = min( nodeVar3, nodeConst8.x );
			nodeVar4 = min( nodeVar4, nodeConst8.y );
			nodeVar5 = min( nodeVar5, nodeConst8.z );
			nodeVar6 = max( nodeVar6, nodeConst8.x );
			nodeVar7 = max( nodeVar7, nodeConst8.y );
			

		}

		nodeVar17 = false;

		if ( ( ! nodeVar8 ) ) {

			let nodeConst9 = i32( clamp( ( ( ( nodeVar6 * 0.5 ) + 0.5 ) * object.nodeUniform9.x ), 0.0, ( object.nodeUniform9.x - 1.0 ) ) );
			let nodeConst10 = i32( clamp( ( ( ( nodeVar3 * 0.5 ) + 0.5 ) * object.nodeUniform9.x ), 0.0, ( object.nodeUniform9.x - 1.0 ) ) );
			let nodeConst11 = i32( clamp( ( ( 0.5 - ( nodeVar4 * 0.5 ) ) * object.nodeUniform9.y ), 0.0, ( object.nodeUniform9.y - 1.0 ) ) );
			let nodeConst12 = i32( clamp( ( ( 0.5 - ( nodeVar7 * 0.5 ) ) * object.nodeUniform9.y ), 0.0, ( object.nodeUniform9.y - 1.0 ) ) );
			let nodeConst13 = clamp( ( i32( ceil( log2( f32( ( max( ( nodeConst9 - nodeConst10 ), ( nodeConst11 - nodeConst12 ) ) + 1 ) ) ) ) ) - 1 ), 0, ( i32( object.nodeUniform10 ) - 1 ) );
			let nodeConst14 = u32( nodeUniform8.value[ nodeConst13 ].x );
			let nodeConst15 = u32( ( nodeConst13 + 1 ) );
			let nodeConst16 = ( u32( nodeConst12 ) >> nodeConst15 );
			let nodeConst17 = u32( nodeUniform8.value[ nodeConst13 ].y );
			let nodeConst18 = ( u32( nodeConst10 ) >> nodeConst15 );
			let nodeConst19 = ( u32( nodeConst9 ) >> nodeConst15 );
			let nodeConst20 = ( u32( nodeConst11 ) >> nodeConst15 );
			nodeVar17 = ( nodeVar5 > max( max( nodeUniform7.value[ ( ( nodeConst14 + ( nodeConst16 * nodeConst17 ) ) + nodeConst18 ) ], nodeUniform7.value[ ( ( nodeConst14 + ( nodeConst16 * nodeConst17 ) ) + nodeConst19 ) ] ), max( nodeUniform7.value[ ( ( nodeConst14 + ( nodeConst20 * nodeConst17 ) ) + nodeConst18 ) ], nodeUniform7.value[ ( ( nodeConst14 + ( nodeConst20 * nodeConst17 ) ) + nodeConst19 ) ] ) ) );
			

		}


		if ( nodeVar17 ) {

			nodeUniform0.value[ instanceIndex ] = vec4<u32>( nodeVar0.x, ( nodeVar0.y | 256u ), nodeVar0.z, nodeVar0.w );
			

		}

		

	}


	

}
