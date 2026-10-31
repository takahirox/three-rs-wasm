// Three.js r187 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms
@binding( 0 ) @group( 0 ) var nodeUniform0 : texture_storage_3d<rgba16float, write>;

struct objectStruct {
	nodeUniform1 : f32,
	nodeUniform2 : u32
};
@binding( 1 ) @group( 0 )
var<uniform> object : objectStruct;

// vars


// codes
fn tsl_mod_vec4( x : vec4f, y : vec4f ) -> vec4f { return x - y * floor( x / y ); }
fn tsl_mod_vec3( x : vec3f, y : vec3f ) -> vec3f { return x - y * floor( x / y ); }
fn fn130 ( x : vec4<f32> ) -> vec4<f32> {

	


	return tsl_mod_vec4( ( ( ( x * x ) * vec4<f32>( 34.0 ) ) + x ), vec4<f32>( 289.0 ) );

}


fn fn131 ( v : vec3<f32> ) -> f32 {

	

	let nodeConst0 = ( vec2<f32>( 1.0 ) / vec2<f32>( 6.0, 3.0 ) );
	var nodeVar0 : vec3<f32> = floor( ( v + vec3<f32>( dot( v, vec3<f32>( nodeConst0, 0.0 ).yyy ) ) ) );
	nodeVar0 = tsl_mod_vec3( nodeVar0, vec3<f32>( 289.0 ) );
	let nodeConst1 = ( ( v - nodeVar0 ) + vec3<f32>( dot( nodeVar0, vec3<f32>( nodeConst0, 0.0 ).xxx ) ) );
	let nodeConst2 = step( nodeConst1.yzx, nodeConst1 );
	let nodeConst3 = min( nodeConst2, ( vec3<f32>( 1.0 ) - nodeConst2 ).zxy );
	let nodeConst4 = max( nodeConst2, ( vec3<f32>( 1.0 ) - nodeConst2 ).zxy );
	let nodeConst5 = fn130( ( ( fn130( ( ( fn130( ( vec4<f32>( nodeVar0.z ) + vec4<f32>( 0.0, nodeConst3.z, nodeConst4.z, 1.0 ) ) ) + vec4<f32>( nodeVar0.y ) ) + vec4<f32>( 0.0, nodeConst3.y, nodeConst4.y, 1.0 ) ) ) + vec4<f32>( nodeVar0.x ) ) + vec4<f32>( 0.0, nodeConst3.x, nodeConst4.x, 1.0 ) ) );
	let nodeConst6 = ( ( vec3<f32>( 0.142857142857 ) * vec4<f32>( 0.0, 0.5, 1.0, 2.0 ).wyz ) - vec4<f32>( 0.0, 0.5, 1.0, 2.0 ).xzx );
	let nodeConst7 = ( nodeConst5 - ( vec4<f32>( 49.0 ) * floor( ( ( nodeConst5 * vec4<f32>( nodeConst6.z ) ) * vec4<f32>( nodeConst6.z ) ) ) ) );
	let nodeConst8 = floor( ( nodeConst7 * vec4<f32>( nodeConst6.z ) ) );
	let nodeConst9 = ( ( nodeConst8 * vec4<f32>( nodeConst6.x ) ) + vec4<f32>( nodeConst6, 1.0 ).yyyy );
	let nodeConst10 = ( ( floor( ( nodeConst7 - ( vec4<f32>( 7.0 ) * nodeConst8 ) ) ) * vec4<f32>( nodeConst6.x ) ) + vec4<f32>( nodeConst6, 1.0 ).yyyy );
	let nodeConst11 = vec4<f32>( nodeConst9.xy, nodeConst10.xy );
	let nodeConst12 = ( ( vec4<f32>( 1.0 ) - abs( nodeConst9 ) ) - abs( nodeConst10 ) );
	let nodeConst13 = ( - step( nodeConst12, vec4<f32>( 0.0, 0.0, 0.0, 0.0 ) ) );
	let nodeConst14 = ( nodeConst11.xzyw + ( ( ( floor( nodeConst11 ) * vec4<f32>( 2.0 ) ) + vec4<f32>( 1.0 ) ).xzyw * nodeConst13.xxyy ) );
	var nodeVar1 : vec3<f32> = vec3<f32>( nodeConst14.xy, nodeConst12.x );
	var nodeVar2 : vec3<f32> = vec3<f32>( nodeConst14.zw, nodeConst12.y );
	let nodeConst15 = vec4<f32>( nodeConst9.zw, nodeConst10.zw );
	let nodeConst16 = ( nodeConst15.xzyw + ( ( ( floor( nodeConst15 ) * vec4<f32>( 2.0 ) ) + vec4<f32>( 1.0 ) ).xzyw * nodeConst13.zzww ) );
	var nodeVar3 : vec3<f32> = vec3<f32>( nodeConst16.xy, nodeConst12.z );
	var nodeVar4 : vec3<f32> = vec3<f32>( nodeConst16.zw, nodeConst12.w );
	let nodeConst17 = inverseSqrt( vec4<f32>( dot( nodeVar1, nodeVar1 ), dot( nodeVar2, nodeVar2 ), dot( nodeVar3, nodeVar3 ), dot( nodeVar4, nodeVar4 ) ) );
	nodeVar1 = ( nodeVar1 * vec3<f32>( nodeConst17.x ) );
	nodeVar2 = ( nodeVar2 * vec3<f32>( nodeConst17.y ) );
	nodeVar3 = ( nodeVar3 * vec3<f32>( nodeConst17.z ) );
	nodeVar4 = ( nodeVar4 * vec3<f32>( nodeConst17.w ) );
	let nodeConst18 = ( ( nodeConst1 - nodeConst3 ) + vec3<f32>( nodeConst0.x ) );
	let nodeConst19 = ( ( nodeConst1 - nodeConst4 ) + vec3<f32>( nodeConst0.y ) );
	let nodeConst20 = ( nodeConst1 - vec4<f32>( 0.0, 0.5, 1.0, 2.0 ).yyy );
	let nodeConst21 = max( ( vec4<f32>( 0.6 ) - vec4<f32>( dot( nodeConst1, nodeConst1 ), dot( nodeConst18, nodeConst18 ), dot( nodeConst19, nodeConst19 ), dot( nodeConst20, nodeConst20 ) ) ), vec4<f32>( 0.0 ) );

	return ( 0.5 + ( 12.0 * dot( ( ( nodeConst21 * nodeConst21 ) * nodeConst21 ), vec4<f32>( dot( nodeVar1, nodeConst1 ), dot( nodeVar2, nodeConst18 ), dot( nodeVar3, nodeConst19 ), dot( nodeVar4, nodeConst20 ) ) ) ) );

}


fn fn132 ( x : vec3<f32> ) -> vec3<f32> {

	


	return vec3<f32>( fn131( ( ( x * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ) ), ( ( fn131( vec3<f32>( ( x.y - 19.1 ), ( x.z + 33.4 ), ( x.x + 47.2 ) ) ) * 2.0 ) - 1.0 ), fn131( ( ( vec3<f32>( ( x.z + 74.2 ), ( x.x - 124.5 ), ( x.y + 99.4 ) ) * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ) ) );

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


	// flow -> computeCurlNoise
	if ( instanceIndex >= object.nodeUniform2 ) { return; }

	let nodeConst0 = vec3<u32>( ( instanceIndex % 100u ), ( ( instanceIndex / 100u ) % 100u ), ( instanceIndex / 10000u ) );
	let nodeConst1 = ( ( ( vec3<f32>( nodeConst0 ) + vec3<f32>( 0.5 ) ) / vec3<f32>( 100.0, 100.0, 200.0 ) ) * vec3<f32>( 1.0, 1.0, 2.0 ) );
	let nodeConst2 = ( 0.1 / object.nodeUniform1 );
	let nodeConst3 = vec3<f32>( 0.0, nodeConst2, 0.0 );
	let nodeConst4 = fn132( ( ( nodeConst1 + nodeConst3 ) * vec3<f32>( object.nodeUniform1 ) ) );
	let nodeConst5 = fn132( ( ( nodeConst1 - nodeConst3 ) * vec3<f32>( object.nodeUniform1 ) ) );
	let nodeConst6 = vec3<f32>( 0.0, 0.0, nodeConst2 );
	let nodeConst7 = fn132( ( ( nodeConst1 + nodeConst6 ) * vec3<f32>( object.nodeUniform1 ) ) );
	let nodeConst8 = fn132( ( ( nodeConst1 - nodeConst6 ) * vec3<f32>( object.nodeUniform1 ) ) );
	let nodeConst9 = vec3<f32>( nodeConst2, 0.0, 0.0 );
	let nodeConst10 = fn132( ( ( nodeConst1 + nodeConst9 ) * vec3<f32>( object.nodeUniform1 ) ) );
	let nodeConst11 = fn132( ( ( nodeConst1 - nodeConst9 ) * vec3<f32>( object.nodeUniform1 ) ) );
	textureStore( nodeUniform0, nodeConst0, vec4<f32>( ( vec3<f32>( ( ( ( nodeConst4.z - nodeConst5.z ) - nodeConst7.y ) + nodeConst8.y ), ( ( ( nodeConst7.x - nodeConst8.x ) - nodeConst10.z ) + nodeConst11.z ), ( ( ( nodeConst10.y - nodeConst11.y ) - nodeConst4.x ) + nodeConst5.x ) ) * vec3<f32>( 5.0 ) ), 0.0 ) );

	

}
