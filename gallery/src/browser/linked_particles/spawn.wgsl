// Three.js r187 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct nodeUniform0Struct {
	value : array< vec4<f32> >
};
@binding( 0 ) @group( 0 )
var<storage, read_write> nodeUniform0 : nodeUniform0Struct;

struct nodeUniform5Struct {
	value : array< vec4<f32> >
};
@binding( 2 ) @group( 0 )
var<storage, read_write> nodeUniform5 : nodeUniform5Struct;

struct objectStruct {
	nodeUniform1 : f32,
	nodeUniform2 : vec3<f32>,
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : f32,
	nodeUniform6 : u32
};
@binding( 1 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;

// codes
fn tsl_mod_float( x : f32, y : f32 ) -> f32 { return x - y * floor( x / y ); }
fn fn5 ( seed : u32 ) -> f32 {

	

	let nodeConst0 = ( ( seed * 747796405u ) + 2891336453u );
	let nodeConst1 = ( ( ( nodeConst0 >> ( ( nodeConst0 >> 28u ) + 4u ) ) ^ nodeConst0 ) * 277803737u );

	return ( f32( ( ( nodeConst1 >> 22u ) ^ nodeConst1 ) ) * 2.3283064365386963e-10 );

}




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


	// flow -> Spawn Particles
	if ( instanceIndex >= object.nodeUniform6 ) { return; }

	let nodeConst0 = i32( tsl_mod_float( ( object.nodeUniform1 + f32( instanceIndex ) ), 8192.0 ) );
	nodeUniform0.value[ nodeConst0 ].w = 1.0;
	let nodeConst1 = ( fn5( u32( nodeConst0 ) ) * 6.283185307179586 );
	let nodeConst2 = ( fn5( u32( ( nodeConst0 + 1 ) ) ) * 3.141592653589793 );
	let nodeConst3 = vec3<f32>( ( sin( nodeConst1 ) * cos( nodeConst2 ) ), ( sin( nodeConst1 ) * sin( nodeConst2 ) ), cos( nodeConst1 ) );
	nodeVar0 = ( mix( object.nodeUniform2, object.nodeUniform3, clamp( ( f32( instanceIndex ) / ( object.nodeUniform4 - 1.0 ) ), 0.0, 1.0 ) ) + ( nodeConst3 * vec3<f32>( 0.01 ) ) );
	nodeUniform0.value[ nodeConst0 ].x = nodeVar0[ 0 ];
	nodeUniform0.value[ nodeConst0 ].y = nodeVar0[ 1 ];
	nodeUniform0.value[ nodeConst0 ].z = nodeVar0[ 2 ];
	nodeVar1 = ( nodeConst3 * vec3<f32>( 5.0 ) );
	nodeUniform5.value[ nodeConst0 ].x = nodeVar1[ 0 ];
	nodeUniform5.value[ nodeConst0 ].y = nodeVar1[ 1 ];
	nodeUniform5.value[ nodeConst0 ].z = nodeVar1[ 2 ];

	

}
