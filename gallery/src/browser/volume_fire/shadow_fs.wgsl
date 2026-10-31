// Three.js r187 - Node System

// global
diagnostic( off, derivative_uniformity );


// directives


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
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> Output : vec4<f32>;

// codes
fn tsl_mod_vec4( x : vec4f, y : vec4f ) -> vec4f { return x - y * floor( x / y ); }
fn fn130 ( x : vec4<f32> ) -> vec4<f32> {

	


	return tsl_mod_vec4( ( ( ( x * x ) * vec4<f32>( 34.0 ) ) + x ), vec4<f32>( 289.0 ) );

}


fn tsl_mod_vec3( x : vec3f, y : vec3f ) -> vec3f { return x - y * floor( x / y ); }
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




@fragment
fn main( @location( 0 ) v_positionWorld : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = ( 29.393876913398138 / f32( object.nodeUniform0 ) );
	var nodeVar1 : vec3<f32> = v_positionWorld;
	nodeVar2 = normalize( ( nodeVar1 - render.cameraPosition ) );
	nodeVar3 = 0.0;
	nodeVar4 = 1.0;

	for ( var i : i32 = 0; i < object.nodeUniform0; i ++ ) {

		let nodeConst0 = ( nodeVar1 + ( nodeVar2 * vec3<f32>( nodeVar3 ) ) );
		nodeVar5 = ( ( ( nodeConst0 - vec3<f32>( 0.0, 6.0, 0.0 ) ) / object.nodeUniform3 ) + vec3<f32>( 0.5 ) );
		nodeVar6 = textureSampleLevel( nodeUniform4, nodeUniform4_sampler, nodeVar5, 0.0 );
		nodeVar7 = clamp( ( nodeVar5 + ( ( nodeVar6.xyz / object.nodeUniform3 ) * vec3<f32>( 0.15 ) ) ), vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) );
		nodeVar8 = textureSampleLevel( nodeUniform5, nodeUniform5_sampler, nodeVar7, 0.0 );
		nodeVar8.x = ( nodeVar8.x * ( ( fn131( ( ( nodeConst0 * vec3<f32>( 5.5 ) ) + vec3<f32>( 0.0, ( - ( nodeVar8.z * 0.8 ) ), 0.0 ) ) ) * 0.35 ) + 0.85 ) );
		let nodeConst1 = min( nodeVar7, ( vec3<f32>( 1.0, 1.0, 1.0 ) - nodeVar7 ) );
		nodeVar8.x = ( nodeVar8.x * smoothstep( 0.0, 0.06, min( nodeConst1.x, min( nodeConst1.y, nodeConst1.z ) ) ) );
		nodeVar4 = ( nodeVar4 * exp( ( ( - ( ( nodeVar8.x * object.nodeUniform6 ) * 0.01 ) ) * nodeVar0 ) ) );
		nodeVar3 = ( nodeVar3 + nodeVar0 );

	}


	if ( ( nodeVar4 >= 0.99 ) ) {

		discard;
		

	}

	let nodeConst2 = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), ( ( 1.0 - nodeVar4 ) * 5.0 ) );
	DiffuseColor = vec4<f32>( nodeConst2.xyz, nodeConst2.w );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform7 );
	let nodeConst3 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst3;

	// result

	output.color = nodeConst3;

	return output;

}
