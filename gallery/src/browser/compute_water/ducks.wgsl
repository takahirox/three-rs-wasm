// Three.js r187 - Node System

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
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;

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

		let nodeConst0 = ( ( u32( clamp( floor( ( ( ( nodeVar0.z / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) * 128u ) + u32( clamp( floor( ( ( ( nodeVar0.x / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) );
		nodeVar2 = HeightA.value[ nodeConst0 ];

	} else {

		let nodeConst0 = ( ( u32( clamp( floor( ( ( ( nodeVar0.z / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) * 128u ) + u32( clamp( floor( ( ( ( nodeVar0.x / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) );
		nodeVar2 = HeightB.value[ nodeConst0 ];

	}

	nodeVar0.y = ( nodeVar0.y + ( ( ( nodeVar2 + -0.04 ) - nodeVar0.y ) * 0.98 ) );
	nodeVar1.x = ( nodeVar1.x * 0.92 );
	nodeVar1.y = ( nodeVar1.y * 0.92 );

	if ( bool( object.nodeUniform1 ) ) {

		let nodeConst0 = ( ( u32( clamp( floor( ( ( ( nodeVar0.z / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) * 128u ) + u32( clamp( floor( ( ( ( nodeVar0.x / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) );
		let nodeConst1 = i32( ( nodeConst0 / 128u ) );
		let nodeConst2 = i32( ( nodeConst0 % 128u ) );
		let nodeConst3 = ( ( nodeConst1 * 128 ) + max( 0, ( nodeConst2 - 1 ) ) );
		nodeVar3 = HeightA.value[ nodeConst3 ];

	} else {

		let nodeConst0 = ( ( u32( clamp( floor( ( ( ( nodeVar0.z / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) * 128u ) + u32( clamp( floor( ( ( ( nodeVar0.x / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) );
		let nodeConst1 = i32( ( nodeConst0 / 128u ) );
		let nodeConst2 = i32( ( nodeConst0 % 128u ) );
		let nodeConst3 = ( ( nodeConst1 * 128 ) + max( 0, ( nodeConst2 - 1 ) ) );
		nodeVar3 = HeightB.value[ nodeConst3 ];

	}


	if ( bool( object.nodeUniform1 ) ) {

		let nodeConst0 = ( ( u32( clamp( floor( ( ( ( nodeVar0.z / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) * 128u ) + u32( clamp( floor( ( ( ( nodeVar0.x / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) );
		let nodeConst1 = i32( ( nodeConst0 / 128u ) );
		let nodeConst2 = i32( ( nodeConst0 % 128u ) );
		let nodeConst4 = ( ( nodeConst1 * 128 ) + min( ( nodeConst2 + 1 ), i32( ( 128u - 1u ) ) ) );
		nodeVar4 = HeightA.value[ nodeConst4 ];

	} else {

		let nodeConst0 = ( ( u32( clamp( floor( ( ( ( nodeVar0.z / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) * 128u ) + u32( clamp( floor( ( ( ( nodeVar0.x / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) );
		let nodeConst1 = i32( ( nodeConst0 / 128u ) );
		let nodeConst2 = i32( ( nodeConst0 % 128u ) );
		let nodeConst4 = ( ( nodeConst1 * 128 ) + min( ( nodeConst2 + 1 ), i32( ( 128u - 1u ) ) ) );
		nodeVar4 = HeightB.value[ nodeConst4 ];

	}

	nodeVar1.x = ( nodeVar1.x + ( ( ( nodeVar3 - nodeVar4 ) * 21.333333333333332 ) * 0.015 ) );

	if ( bool( object.nodeUniform1 ) ) {

		let nodeConst0 = ( ( u32( clamp( floor( ( ( ( nodeVar0.z / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) * 128u ) + u32( clamp( floor( ( ( ( nodeVar0.x / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) );
		let nodeConst1 = i32( ( nodeConst0 / 128u ) );
		let nodeConst2 = i32( ( nodeConst0 % 128u ) );
		let nodeConst5 = ( ( max( 0, ( nodeConst1 - 1 ) ) * 128 ) + nodeConst2 );
		nodeVar5 = HeightA.value[ nodeConst5 ];

	} else {

		let nodeConst0 = ( ( u32( clamp( floor( ( ( ( nodeVar0.z / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) * 128u ) + u32( clamp( floor( ( ( ( nodeVar0.x / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) );
		let nodeConst1 = i32( ( nodeConst0 / 128u ) );
		let nodeConst2 = i32( ( nodeConst0 % 128u ) );
		let nodeConst5 = ( ( max( 0, ( nodeConst1 - 1 ) ) * 128 ) + nodeConst2 );
		nodeVar5 = HeightB.value[ nodeConst5 ];

	}


	if ( bool( object.nodeUniform1 ) ) {

		let nodeConst0 = ( ( u32( clamp( floor( ( ( ( nodeVar0.z / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) * 128u ) + u32( clamp( floor( ( ( ( nodeVar0.x / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) );
		let nodeConst1 = i32( ( nodeConst0 / 128u ) );
		let nodeConst2 = i32( ( nodeConst0 % 128u ) );
		let nodeConst6 = ( ( min( ( nodeConst1 + 1 ), i32( ( 128u - 1u ) ) ) * 128 ) + nodeConst2 );
		nodeVar6 = HeightA.value[ nodeConst6 ];

	} else {

		let nodeConst0 = ( ( u32( clamp( floor( ( ( ( nodeVar0.z / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) * 128u ) + u32( clamp( floor( ( ( ( nodeVar0.x / 6.0 ) + 0.5 ) * 128.0 ) ), 0.0, 127.0 ) ) );
		let nodeConst1 = i32( ( nodeConst0 / 128u ) );
		let nodeConst2 = i32( ( nodeConst0 % 128u ) );
		let nodeConst6 = ( ( min( ( nodeConst1 + 1 ), i32( ( 128u - 1u ) ) ) * 128 ) + nodeConst2 );
		nodeVar6 = HeightB.value[ nodeConst6 ];

	}

	nodeVar1.y = ( nodeVar1.y + ( ( ( nodeVar5 - nodeVar6 ) * 21.333333333333332 ) * 0.015 ) );
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
