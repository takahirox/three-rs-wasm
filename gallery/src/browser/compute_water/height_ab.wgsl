// Three.js r187 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct HeightAStruct {
	value : array< f32 >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> HeightA : HeightAStruct;

struct PrevHeightStruct {
	value : array< f32 >
};
@binding( 1 ) @group( 0 )
var<storage, read_write> PrevHeight : PrevHeightStruct;

struct HeightBStruct {
	value : array< f32 >
};
@binding( 3 ) @group( 0 )
var<storage, read_write> HeightB : HeightBStruct;

struct objectStruct {
	viscosity : f32,
	mousePos : vec2<f32>,
	mouseSize : f32,
	mouseDeep : f32,
	mouseSpeed : vec2<f32>
};
@binding( 2 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;

// codes


@compute @workgroup_size( 16, 16, 1 )
fn main( @builtin( global_invocation_id ) globalId : vec3<u32>,
	@builtin( workgroup_id ) workgroupId : vec3<u32>,
	@builtin( local_invocation_id ) localId : vec3<u32>,
	@builtin( num_workgroups ) numWorkgroups : vec3<u32> ) {

	// local vars
	

	// system
	instanceIndex = globalId.x
		+ globalId.y * ( 16 * numWorkgroups.x )
		+ globalId.z * ( 16 * numWorkgroups.x ) * ( 16 * numWorkgroups.y );

	// flow
	// code


	// flow -> Update Height A→B
	nodeVar0 = HeightA.value[ instanceIndex ];
	nodeVar1 = PrevHeight.value[ instanceIndex ];
	let nodeConst0 = i32( ( instanceIndex / 128u ) );
	let nodeConst1 = i32( ( instanceIndex % 128u ) );
	var nodeVar2 : f32 = ( ( ( HeightA.value[ ( ( min( ( nodeConst0 + 1 ), i32( ( 128u - 1u ) ) ) * 128 ) + nodeConst1 ) ] + HeightA.value[ ( ( max( 0, ( nodeConst0 - 1 ) ) * 128 ) + nodeConst1 ) ] ) + HeightA.value[ ( ( nodeConst0 * 128 ) + min( ( nodeConst1 + 1 ), i32( ( 128u - 1u ) ) ) ) ] ) + HeightA.value[ ( ( nodeConst0 * 128 ) + max( 0, ( nodeConst1 - 1 ) ) ) ] );
	nodeVar2 = ( nodeVar2 * 0.5 );
	nodeVar2 = ( nodeVar2 - nodeVar1 );
	var nodeVar3 : f32 = ( nodeVar2 * object.viscosity );
	nodeVar3 = ( nodeVar3 + ( ( ( cos( clamp( ( ( length( ( ( ( vec2<f32>( ( f32( globalId.x ) * 0.0078125 ), ( f32( globalId.y ) * 0.0078125 ) ) - vec2<f32>( 0.5, 0.5 ) ) * vec2<f32>( 6.0 ) ) - object.mousePos ) ) * 3.141592653589793 ) / object.mouseSize ), 0.0, 3.141592653589793 ) ) + 1.0 ) * object.mouseDeep ) * length( object.mouseSpeed ) ) );
	PrevHeight.value[ instanceIndex ] = nodeVar0;
	HeightB.value[ instanceIndex ] = nodeVar3;

	

}
