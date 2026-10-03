// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform4_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform4 : texture_3d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform5_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform5 : texture_3d<f32>;

struct objectStruct {
	nodeUniform0 : i32,
	nodeUniform1 : mat4x4<f32>,
	nodeUniform3 : vec3<f32>,
	nodeUniform6 : f32,
	nodeUniform7 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraPosition : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : f32;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar11 : vec4<f32>;

// codes
fn tsl_mod_vec4( x : vec4f, y : vec4f ) -> vec4f { return x - y * floor( x / y ); }
fn fn4 ( x : vec4<f32> ) -> vec4<f32> {

	


	return tsl_mod_vec4( ( ( ( x * x ) * vec4<f32>( 34.0 ) ) + x ), vec4<f32>( 289.0 ) );

}


fn tsl_mod_vec3( x : vec3f, y : vec3f ) -> vec3f { return x - y * floor( x / y ); }
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




@fragment
fn main( @location( 0 ) v_positionWorld : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = ( 29.393876913398138 / f32( object.nodeUniform0 ) );
	nodeVar1 = normalize( ( v_positionWorld - render.cameraPosition ) );
	nodeVar2 = 0.0;
	nodeVar3 = 1.0;

	for ( var i : i32 = 0; i < object.nodeUniform0; i ++ ) {

		nodeVar4 = ( v_positionWorld + ( nodeVar1 * vec3<f32>( nodeVar2 ) ) );
		nodeVar5 = ( ( ( nodeVar4 - vec3<f32>( 0.0, 6.0, 0.0 ) ) / object.nodeUniform3 ) + vec3<f32>( 0.5 ) );
		nodeVar6 = textureSampleLevel( nodeUniform4, nodeUniform4_sampler, nodeVar5, 0.0 );
		nodeVar7 = clamp( ( nodeVar5 + ( ( nodeVar6.xyz / object.nodeUniform3 ) * vec3<f32>( 0.15 ) ) ), vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) );
		nodeVar8 = textureSampleLevel( nodeUniform5, nodeUniform5_sampler, nodeVar7, 0.0 );
		nodeVar8.x = ( nodeVar8.x * ( ( fn5( ( ( nodeVar4 * vec3<f32>( 5.5 ) ) + vec3<f32>( 0.0, ( - ( nodeVar8.z * 0.8 ) ), 0.0 ) ) ) * 0.35 ) + 0.85 ) );
		nodeVar9 = min( nodeVar7, ( vec3<f32>( 1.0, 1.0, 1.0 ) - nodeVar7 ) );
		nodeVar8.x = ( nodeVar8.x * smoothstep( 0.0, 0.06, min( nodeVar9.x, min( nodeVar9.y, nodeVar9.z ) ) ) );
		nodeVar3 = ( nodeVar3 * exp( ( ( - ( ( nodeVar8.x * object.nodeUniform6 ) * 0.01 ) ) * nodeVar0 ) ) );
		nodeVar2 = ( nodeVar2 + nodeVar0 );

	}


	if ( ( nodeVar3 >= 0.99 ) ) {

		discard;
		

	}

	nodeVar10 = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), ( ( 1.0 - nodeVar3 ) * 5.0 ) );
	DiffuseColor = vec4<f32>( nodeVar10.xyz, nodeVar10.w );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform7 );
	nodeVar11 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar11;

	// result

	output.color = nodeVar11;

	return output;

}
