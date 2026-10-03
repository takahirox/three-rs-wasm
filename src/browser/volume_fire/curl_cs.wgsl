// Three.js r186 - Node System

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
var<private> nodeVar0 : vec3<u32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;

// codes
fn tsl_mod_vec4( x : vec4f, y : vec4f ) -> vec4f { return x - y * floor( x / y ); }
fn tsl_mod_vec3( x : vec3f, y : vec3f ) -> vec3f { return x - y * floor( x / y ); }
fn fn4 ( x : vec4<f32> ) -> vec4<f32> {

	


	return tsl_mod_vec4( ( ( ( x * x ) * vec4<f32>( 34.0 ) ) + x ), vec4<f32>( 289.0 ) );

}


fn fn5 ( v : vec3<f32> ) -> f32 {

	var nodeVar0 : vec2<f32>;
	var nodeVar1 : vec3<f32>;
	var nodeVar2 : vec3<f32>;
	var nodeVar3 : vec3<f32>;
	var nodeVar4 : vec3<f32>;
	var nodeVar5 : vec3<f32>;
	var nodeVar6 : vec4<f32>;
	var nodeVar7 : vec3<f32>;
	var nodeVar8 : vec4<f32>;
	var nodeVar9 : vec4<f32>;
	var nodeVar10 : vec4<f32>;
	var nodeVar11 : vec4<f32>;
	var nodeVar12 : vec4<f32>;
	var nodeVar13 : vec4<f32>;
	var nodeVar14 : vec4<f32>;
	var nodeVar15 : vec4<f32>;
	var nodeVar16 : vec3<f32>;
	var nodeVar17 : vec3<f32>;
	var nodeVar18 : vec4<f32>;
	var nodeVar19 : vec4<f32>;
	var nodeVar20 : vec3<f32>;
	var nodeVar21 : vec3<f32>;
	var nodeVar22 : vec4<f32>;
	var nodeVar23 : vec3<f32>;
	var nodeVar24 : vec3<f32>;
	var nodeVar25 : vec3<f32>;
	var nodeVar26 : vec4<f32>;

	nodeVar0 = ( vec2<f32>( 1.0 ) / vec2<f32>( 6.0, 3.0 ) );
	nodeVar1 = floor( ( v + vec3<f32>( dot( v, vec3<f32>( nodeVar0, 0.0 ).yyy ) ) ) );
	nodeVar1 = tsl_mod_vec3( nodeVar1, vec3<f32>( 289.0 ) );
	nodeVar2 = ( ( v - nodeVar1 ) + vec3<f32>( dot( nodeVar1, vec3<f32>( nodeVar0, 0.0 ).xxx ) ) );
	nodeVar3 = step( nodeVar2.yzx, nodeVar2 );
	nodeVar4 = min( nodeVar3, ( vec3<f32>( 1.0 ) - nodeVar3 ).zxy );
	nodeVar5 = max( nodeVar3, ( vec3<f32>( 1.0 ) - nodeVar3 ).zxy );
	nodeVar6 = fn4( ( ( fn4( ( ( fn4( ( vec4<f32>( nodeVar1.z ) + vec4<f32>( 0.0, nodeVar4.z, nodeVar5.z, 1.0 ) ) ) + vec4<f32>( nodeVar1.y ) ) + vec4<f32>( 0.0, nodeVar4.y, nodeVar5.y, 1.0 ) ) ) + vec4<f32>( nodeVar1.x ) ) + vec4<f32>( 0.0, nodeVar4.x, nodeVar5.x, 1.0 ) ) );
	nodeVar7 = ( ( vec3<f32>( 0.142857142857 ) * vec4<f32>( 0.0, 0.5, 1.0, 2.0 ).wyz ) - vec4<f32>( 0.0, 0.5, 1.0, 2.0 ).xzx );
	nodeVar8 = ( nodeVar6 - ( vec4<f32>( 49.0 ) * floor( ( ( nodeVar6 * vec4<f32>( nodeVar7.z ) ) * vec4<f32>( nodeVar7.z ) ) ) ) );
	nodeVar9 = floor( ( nodeVar8 * vec4<f32>( nodeVar7.z ) ) );
	nodeVar10 = ( ( nodeVar9 * vec4<f32>( nodeVar7.x ) ) + vec4<f32>( nodeVar7, 1.0 ).yyyy );
	nodeVar11 = ( ( floor( ( nodeVar8 - ( vec4<f32>( 7.0 ) * nodeVar9 ) ) ) * vec4<f32>( nodeVar7.x ) ) + vec4<f32>( nodeVar7, 1.0 ).yyyy );
	nodeVar12 = vec4<f32>( nodeVar10.xy, nodeVar11.xy );
	nodeVar13 = ( ( vec4<f32>( 1.0 ) - abs( nodeVar10 ) ) - abs( nodeVar11 ) );
	nodeVar14 = ( - step( nodeVar13, vec4<f32>( 0.0, 0.0, 0.0, 0.0 ) ) );
	nodeVar15 = ( nodeVar12.xzyw + ( ( ( floor( nodeVar12 ) * vec4<f32>( 2.0 ) ) + vec4<f32>( 1.0 ) ).xzyw * nodeVar14.xxyy ) );
	nodeVar16 = vec3<f32>( nodeVar15.xy, nodeVar13.x );
	nodeVar17 = vec3<f32>( nodeVar15.zw, nodeVar13.y );
	nodeVar18 = vec4<f32>( nodeVar10.zw, nodeVar11.zw );
	nodeVar19 = ( nodeVar18.xzyw + ( ( ( floor( nodeVar18 ) * vec4<f32>( 2.0 ) ) + vec4<f32>( 1.0 ) ).xzyw * nodeVar14.zzww ) );
	nodeVar20 = vec3<f32>( nodeVar19.xy, nodeVar13.z );
	nodeVar21 = vec3<f32>( nodeVar19.zw, nodeVar13.w );
	nodeVar22 = inverseSqrt( vec4<f32>( dot( nodeVar16, nodeVar16 ), dot( nodeVar17, nodeVar17 ), dot( nodeVar20, nodeVar20 ), dot( nodeVar21, nodeVar21 ) ) );
	nodeVar16 = ( nodeVar16 * vec3<f32>( nodeVar22.x ) );
	nodeVar17 = ( nodeVar17 * vec3<f32>( nodeVar22.y ) );
	nodeVar20 = ( nodeVar20 * vec3<f32>( nodeVar22.z ) );
	nodeVar21 = ( nodeVar21 * vec3<f32>( nodeVar22.w ) );
	nodeVar23 = ( ( nodeVar2 - nodeVar4 ) + vec3<f32>( nodeVar0.x ) );
	nodeVar24 = ( ( nodeVar2 - nodeVar5 ) + vec3<f32>( nodeVar0.y ) );
	nodeVar25 = ( nodeVar2 - vec4<f32>( 0.0, 0.5, 1.0, 2.0 ).yyy );
	nodeVar26 = max( ( vec4<f32>( 0.6 ) - vec4<f32>( dot( nodeVar2, nodeVar2 ), dot( nodeVar23, nodeVar23 ), dot( nodeVar24, nodeVar24 ), dot( nodeVar25, nodeVar25 ) ) ), vec4<f32>( 0.0 ) );

	return ( 0.5 + ( 12.0 * dot( ( ( nodeVar26 * nodeVar26 ) * nodeVar26 ), vec4<f32>( dot( nodeVar16, nodeVar2 ), dot( nodeVar17, nodeVar23 ), dot( nodeVar20, nodeVar24 ), dot( nodeVar21, nodeVar25 ) ) ) ) );

}


fn fn6 ( x : vec3<f32> ) -> vec3<f32> {

	


	return vec3<f32>( fn5( ( ( x * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ) ), ( ( fn5( vec3<f32>( ( x.y - 19.1 ), ( x.z + 33.4 ), ( x.x + 47.2 ) ) ) * 2.0 ) - 1.0 ), fn5( ( ( vec3<f32>( ( x.z + 74.2 ), ( x.x - 124.5 ), ( x.y + 99.4 ) ) * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ) ) );

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

	nodeVar0 = vec3<u32>( ( instanceIndex % 100u ), ( ( instanceIndex / 100u ) % 100u ), ( instanceIndex / 10000u ) );
	nodeVar1 = ( ( ( vec3<f32>( nodeVar0 ) + vec3<f32>( 0.5 ) ) / vec3<f32>( 100.0, 100.0, 200.0 ) ) * vec3<f32>( 1.0, 1.0, 2.0 ) );
	nodeVar2 = ( 0.1 / object.nodeUniform1 );
	nodeVar3 = vec3<f32>( 0.0, nodeVar2, 0.0 );
	nodeVar4 = fn6( ( ( nodeVar1 + nodeVar3 ) * vec3<f32>( object.nodeUniform1 ) ) );
	nodeVar5 = fn6( ( ( nodeVar1 - nodeVar3 ) * vec3<f32>( object.nodeUniform1 ) ) );
	nodeVar6 = vec3<f32>( 0.0, 0.0, nodeVar2 );
	nodeVar7 = fn6( ( ( nodeVar1 + nodeVar6 ) * vec3<f32>( object.nodeUniform1 ) ) );
	nodeVar8 = fn6( ( ( nodeVar1 - nodeVar6 ) * vec3<f32>( object.nodeUniform1 ) ) );
	nodeVar9 = vec3<f32>( nodeVar2, 0.0, 0.0 );
	nodeVar10 = fn6( ( ( nodeVar1 + nodeVar9 ) * vec3<f32>( object.nodeUniform1 ) ) );
	nodeVar11 = fn6( ( ( nodeVar1 - nodeVar9 ) * vec3<f32>( object.nodeUniform1 ) ) );
	textureStore( nodeUniform0, nodeVar0, vec4<f32>( ( vec3<f32>( ( ( ( nodeVar4.z - nodeVar5.z ) - nodeVar7.y ) + nodeVar8.y ), ( ( ( nodeVar7.x - nodeVar8.x ) - nodeVar10.z ) + nodeVar11.z ), ( ( ( nodeVar10.y - nodeVar11.y ) - nodeVar4.x ) + nodeVar5.x ) ) * vec3<f32>( 5.0 ) ), 0.0 ) );

	

}
