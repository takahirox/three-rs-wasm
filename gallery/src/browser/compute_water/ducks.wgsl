// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs

struct StructType0 {
	position : vec3<f32>,
	velocity : vec2<f32>
};


// uniforms

struct DuckInstanceDataStruct {
	value : array< StructType0 >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> DuckInstanceData : DuckInstanceDataStruct;

struct HeightAStruct {
	value : array< f32 >
};
@binding( 2 ) @group( 0 )
var<storage, read_write> HeightA : HeightAStruct;

struct HeightBStruct {
	value : array< f32 >
};
@binding( 3 ) @group( 0 )
var<storage, read_write> HeightB : HeightBStruct;

struct objectStruct {
	nodeUniform1 : f32,
	nodeUniform4 : u32
};
@binding( 1 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : vec2<f32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : u32;
var<private> nodeVar5 : u32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : u32;
var<private> nodeVar8 : u32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : u32;
var<private> nodeVar11 : u32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : u32;
var<private> nodeVar14 : u32;

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


	// flow -> Update Ducks
	if ( instanceIndex >= object.nodeUniform4 ) { return; }

	nodeVar0 = DuckInstanceData.value[ instanceIndex ].position;
	nodeVar1 = DuckInstanceData.value[ instanceIndex ].velocity;

	if ( bool( object.nodeUniform1 ) ) {

		nodeVar2 = HeightA.value[ ( ( u32( clamp( floor( ( ( ( nodeVar0.z / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) * 128u ) + u32( clamp( floor( ( ( ( nodeVar0.x / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) ) ];

	} else {

		nodeVar2 = HeightB.value[ ( ( u32( clamp( floor( ( ( ( nodeVar0.z / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) * 128u ) + u32( clamp( floor( ( ( ( nodeVar0.x / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) ) ];

	}

	nodeVar0.y = ( nodeVar0.y + ( ( ( nodeVar2 + -0.04 ) - nodeVar0.y ) * 0.98 ) );
	nodeVar1.x = ( nodeVar1.x * 0.92 );
	nodeVar1.y = ( nodeVar1.y * 0.92 );

	if ( bool( object.nodeUniform1 ) ) {

		nodeVar4 = ( ( u32( clamp( floor( ( ( ( nodeVar0.z / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) * 128u ) + u32( clamp( floor( ( ( ( nodeVar0.x / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) );
		nodeVar3 = HeightA.value[ ( ( i32( ( nodeVar4 / 128u ) ) * 128 ) + max( 0, ( i32( ( nodeVar4 % 128u ) ) - 1 ) ) ) ];

	} else {

		nodeVar5 = ( ( u32( clamp( floor( ( ( ( nodeVar0.z / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) * 128u ) + u32( clamp( floor( ( ( ( nodeVar0.x / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) );
		nodeVar3 = HeightB.value[ ( ( i32( ( nodeVar5 / 128u ) ) * 128 ) + max( 0, ( i32( ( nodeVar5 % 128u ) ) - 1 ) ) ) ];

	}


	if ( bool( object.nodeUniform1 ) ) {

		nodeVar7 = ( ( u32( clamp( floor( ( ( ( nodeVar0.z / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) * 128u ) + u32( clamp( floor( ( ( ( nodeVar0.x / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) );
		nodeVar6 = HeightA.value[ ( ( i32( ( nodeVar7 / 128u ) ) * 128 ) + i32( min( u32( ( i32( ( nodeVar7 % 128u ) ) + 1 ) ), ( 128u - 1u ) ) ) ) ];

	} else {

		nodeVar8 = ( ( u32( clamp( floor( ( ( ( nodeVar0.z / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) * 128u ) + u32( clamp( floor( ( ( ( nodeVar0.x / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) );
		nodeVar6 = HeightB.value[ ( ( i32( ( nodeVar8 / 128u ) ) * 128 ) + i32( min( u32( ( i32( ( nodeVar8 % 128u ) ) + 1 ) ), ( 128u - 1u ) ) ) ) ];

	}

	nodeVar1.x = ( nodeVar1.x + ( ( ( nodeVar3 - nodeVar6 ) * 21.333333333333332 ) * 0.015 ) );

	if ( bool( object.nodeUniform1 ) ) {

		nodeVar10 = ( ( u32( clamp( floor( ( ( ( nodeVar0.z / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) * 128u ) + u32( clamp( floor( ( ( ( nodeVar0.x / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) );
		nodeVar9 = HeightA.value[ ( ( max( 0, ( i32( ( nodeVar10 / 128u ) ) - 1 ) ) * 128 ) + i32( ( nodeVar10 % 128u ) ) ) ];

	} else {

		nodeVar11 = ( ( u32( clamp( floor( ( ( ( nodeVar0.z / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) * 128u ) + u32( clamp( floor( ( ( ( nodeVar0.x / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) );
		nodeVar9 = HeightB.value[ ( ( max( 0, ( i32( ( nodeVar11 / 128u ) ) - 1 ) ) * 128 ) + i32( ( nodeVar11 % 128u ) ) ) ];

	}


	if ( bool( object.nodeUniform1 ) ) {

		nodeVar13 = ( ( u32( clamp( floor( ( ( ( nodeVar0.z / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) * 128u ) + u32( clamp( floor( ( ( ( nodeVar0.x / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) );
		nodeVar12 = HeightA.value[ ( ( min( u32( ( i32( ( nodeVar13 / 128u ) ) + 1 ) ), ( 128u - 1u ) ) * 128u ) + u32( i32( ( nodeVar13 % 128u ) ) ) ) ];

	} else {

		nodeVar14 = ( ( u32( clamp( floor( ( ( ( nodeVar0.z / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) * 128u ) + u32( clamp( floor( ( ( ( nodeVar0.x / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) );
		nodeVar12 = HeightB.value[ ( ( min( u32( ( i32( ( nodeVar14 / 128u ) ) + 1 ) ), ( 128u - 1u ) ) * 128u ) + u32( i32( ( nodeVar14 % 128u ) ) ) ) ];

	}

	nodeVar1.y = ( nodeVar1.y + ( ( ( nodeVar9 - nodeVar12 ) * 21.333333333333332 ) * 0.015 ) );
	nodeVar0.x = ( nodeVar0.x + nodeVar1.x );
	nodeVar0.z = ( nodeVar0.z + nodeVar1.y );

	if ( ( nodeVar0.x < -2.8 ) ) {

		nodeVar0.x = -2.8;
		nodeVar1.x = ( nodeVar1.x * -0.4 );
		

	} else {


		if ( ( nodeVar0.x > 2.8 ) ) {

			nodeVar0.x = 2.8;
			nodeVar1.x = ( nodeVar1.x * -0.4 );
			

		}

		

	}


	if ( ( nodeVar0.z < -2.8 ) ) {

		nodeVar0.z = -2.8;
		nodeVar1.y = ( nodeVar1.y * -0.4 );
		

	} else {


		if ( ( nodeVar0.z > 2.8 ) ) {

			nodeVar0.z = 2.8;
			nodeVar1.y = ( nodeVar1.y * -0.4 );
			

		}

		

	}

	DuckInstanceData.value[ instanceIndex ].position = nodeVar0;
	DuckInstanceData.value[ instanceIndex ].velocity = nodeVar1;

	

}
