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
@binding( 1 ) @group( 1 ) var nodeUniform34_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform34 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform40_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform40 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform52_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform52 : texture_3d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform53_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform53 : texture_3d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform54_sampler : sampler;
@binding( 10 ) @group( 1 ) var nodeUniform54 : texture_3d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : i32,
	nodeUniform37 : mat3x3<f32>,
	nodeUniform51 : vec3<f32>,
	nodeUniform55 : vec3<f32>,
	nodeUniform56 : f32,
	nodeUniform57 : f32,
	nodeUniform58 : f32,
	nodeUniform59 : f32,
	nodeUniform60 : f32,
	nodeUniform61 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform5 : u32,
	nodeUniform18 : f32,
	nodeUniform21 : f32,
	nodeUniform7 : f32,
	nodeUniform13 : vec3<f32>,
	nodeUniform22 : f32,
	nodeUniform10 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform23 : f32,
	nodeUniform24 : f32,
	nodeUniform26 : f32,
	nodeUniform27 : f32,
	nodeUniform28 : f32,
	nodeUniform16 : f32,
	nodeUniform17 : f32,
	nodeUniform20 : f32,
	nodeUniform19 : f32,
	nodeUniform15 : f32,
	nodeUniform14 : vec3<f32>,
	nodeUniform25 : f32,
	nodeUniform8 : f32,
	nodeUniform11 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform31 : f32,
	nodeUniform32 : f32,
	nodeUniform44 : f32,
	nodeUniform45 : f32,
	nodeUniform49 : f32,
	nodeUniform50 : f32,
	nodeUniform33 : vec3<f32>,
	nodeUniform47 : vec3<f32>,
	nodeUniform48 : vec3<f32>,
	nodeUniform29 : vec3<f32>,
	nodeUniform46 : vec3<f32>,
	nodeUniform35 : mat4x4<f32>,
	nodeUniform38 : f32,
	nodeUniform39 : f32,
	nodeUniform41 : f32,
	nodeUniform42 : vec2<f32>,
	nodeUniform43 : f32,
	cameraPosition : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : bool;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> workingLuminanceCoefficients : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec4<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec4<f32>;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec4<f32>;
var<private> nodeVar30 : vec4<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : vec4<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : vec4<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn24 ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn fn32 ( color : vec3<f32>, adjustment : f32, luminanceCoefficients : vec3<f32> ) -> vec3<f32> {

	


	return max( mix( vec3<f32>( dot( color, luminanceCoefficients ) ), color, adjustment ), vec3<f32>( 0.0 ) );

}


fn fn34 ( color : vec3<f32>, adjustment : f32 ) -> vec3<f32> {

	

	let nodeConst0 = cos( adjustment );

	return max( ( ( color * vec3<f32>( nodeConst0 ) ) + ( ( cross( vec3<f32>( 0.57735, 0.57735, 0.57735 ), color ) * vec3<f32>( sin( adjustment ) ) ) + ( vec3<f32>( 0.57735, 0.57735, 0.57735 ) * vec3<f32>( ( dot( vec3<f32>( 0.57735, 0.57735, 0.57735 ), color ) * ( 1.0 - nodeConst0 ) ) ) ) ) ), vec3<f32>( 0.0 ) );

}


fn fn48 ( x : f32 ) -> i32 {

	var nodeVar0 : f32;

	nodeVar0 = x;

	return i32( floor( nodeVar0 ) );

}


fn fn65 ( t : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = t;

	return ( ( ( nodeVar0 * nodeVar0 ) * nodeVar0 ) * ( ( nodeVar0 * ( ( nodeVar0 * 6.0 ) - 15.0 ) ) + 10.0 ) );

}


fn fn61 ( x : u32, k : i32 ) -> u32 {

	var nodeVar0 : i32;
	var nodeVar1 : u32;

	nodeVar0 = k;
	nodeVar1 = x;

	return ( ( nodeVar1 << u32( nodeVar0 ) ) | ( nodeVar1 >> u32( ( 32 - nodeVar0 ) ) ) );

}


fn fn63 ( a : u32, b : u32, c : u32 ) -> u32 {

	var nodeVar0 : u32;
	var nodeVar1 : u32;
	var nodeVar2 : u32;

	nodeVar0 = c;
	nodeVar1 = b;
	nodeVar2 = a;
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - fn61( nodeVar1, 14 ) );
	nodeVar2 = ( nodeVar2 ^ nodeVar0 );
	nodeVar2 = ( nodeVar2 - fn61( nodeVar0, 11 ) );
	nodeVar1 = ( nodeVar1 ^ nodeVar2 );
	nodeVar1 = ( nodeVar1 - fn61( nodeVar2, 25 ) );
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - fn61( nodeVar1, 16 ) );
	nodeVar2 = ( nodeVar2 ^ nodeVar0 );
	nodeVar2 = ( nodeVar2 - fn61( nodeVar0, 4 ) );
	nodeVar1 = ( nodeVar1 ^ nodeVar2 );
	nodeVar1 = ( nodeVar1 - fn61( nodeVar2, 14 ) );
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - fn61( nodeVar1, 24 ) );

	return nodeVar0;

}


fn fn68 ( x : i32, y : i32, z : i32 ) -> u32 {

	var nodeVar0 : i32;
	var nodeVar1 : i32;
	var nodeVar2 : i32;
	var nodeVar3 : u32;
	var nodeVar4 : u32;
	var nodeVar5 : u32;
	var nodeVar6 : u32;

	nodeVar0 = z;
	nodeVar1 = y;
	nodeVar2 = x;
	nodeVar3 = 3u;
	nodeVar4 = 0u;
	nodeVar5 = 0u;
	nodeVar6 = 0u;
	nodeVar6 = ( ( 3735928559u + ( nodeVar3 << 2u ) ) + 13u );
	nodeVar5 = nodeVar6;
	nodeVar4 = nodeVar5;
	nodeVar4 = ( nodeVar4 + u32( nodeVar2 ) );
	nodeVar5 = ( nodeVar5 + u32( nodeVar1 ) );
	nodeVar6 = ( nodeVar6 + u32( nodeVar0 ) );

	return fn63( nodeVar4, nodeVar5, nodeVar6 );

}


fn fn46 ( b : bool, t : f32, f : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : bool;
	var nodeVar3 : f32;

	nodeVar0 = f;
	nodeVar1 = t;
	nodeVar2 = b;

	return select( nodeVar0, nodeVar1, nodeVar2 );

}


fn fn47 ( val : f32, b : bool ) -> f32 {

	var nodeVar0 : bool;
	var nodeVar1 : f32;
	var nodeVar2 : f32;

	nodeVar0 = b;
	nodeVar1 = val;

	return select( nodeVar1, ( - nodeVar1 ), nodeVar0 );

}


fn fn54 ( hash : u32, x : f32, y : f32, z : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : u32;
	var nodeVar4 : u32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;

	nodeVar0 = z;
	nodeVar1 = y;
	nodeVar2 = x;
	nodeVar3 = hash;
	nodeVar4 = ( nodeVar3 & 15u );
	nodeVar5 = fn46( ( nodeVar4 < 8u ), nodeVar2, nodeVar1 );
	nodeVar6 = fn46( ( nodeVar4 < 4u ), nodeVar1, fn46( ( ( nodeVar4 == 12u ) || ( nodeVar4 == 14u ) ), nodeVar2, nodeVar0 ) );

	return ( fn47( nodeVar5, bool( ( nodeVar4 & 1u ) ) ) + fn47( nodeVar6, bool( ( nodeVar4 & 2u ) ) ) );

}


fn fn51 ( v0 : f32, v1 : f32, v2 : f32, v3 : f32, v4 : f32, v5 : f32, v6 : f32, v7 : f32, s : f32, t : f32, r : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;
	var nodeVar7 : f32;
	var nodeVar8 : f32;
	var nodeVar9 : f32;
	var nodeVar10 : f32;
	var nodeVar11 : f32;
	var nodeVar12 : f32;
	var nodeVar13 : f32;

	nodeVar0 = r;
	nodeVar1 = t;
	nodeVar2 = s;
	nodeVar3 = v7;
	nodeVar4 = v6;
	nodeVar5 = v5;
	nodeVar6 = v4;
	nodeVar7 = v3;
	nodeVar8 = v2;
	nodeVar9 = v1;
	nodeVar10 = v0;
	nodeVar11 = ( 1.0 - nodeVar2 );
	nodeVar12 = ( 1.0 - nodeVar1 );
	nodeVar13 = ( 1.0 - nodeVar0 );

	return ( ( nodeVar13 * ( ( nodeVar12 * ( ( nodeVar10 * nodeVar11 ) + ( nodeVar9 * nodeVar2 ) ) ) + ( nodeVar1 * ( ( nodeVar8 * nodeVar11 ) + ( nodeVar7 * nodeVar2 ) ) ) ) ) + ( nodeVar0 * ( ( nodeVar12 * ( ( nodeVar6 * nodeVar11 ) + ( nodeVar5 * nodeVar2 ) ) ) + ( nodeVar1 * ( ( nodeVar4 * nodeVar11 ) + ( nodeVar3 * nodeVar2 ) ) ) ) ) );

}


fn fn58 ( v : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = v;

	return ( 0.982 * nodeVar0 );

}


fn fn74 ( p : vec3<f32> ) -> f32 {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : i32;
	var nodeVar2 : i32;
	var nodeVar3 : i32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;
	var nodeVar7 : f32;
	var nodeVar8 : f32;
	var nodeVar9 : f32;
	var nodeVar10 : f32;
	var nodeVar11 : f32;
	var nodeVar12 : f32;
	var nodeVar13 : f32;

	nodeVar0 = p;
	nodeVar1 = 0;
	nodeVar2 = 0;
	nodeVar3 = 0;
	nodeVar4 = nodeVar0.x;
	nodeVar1 = fn48( nodeVar4 );
	nodeVar5 = ( nodeVar4 - f32( nodeVar1 ) );
	nodeVar6 = nodeVar0.y;
	nodeVar2 = fn48( nodeVar6 );
	nodeVar7 = ( nodeVar6 - f32( nodeVar2 ) );
	nodeVar8 = nodeVar0.z;
	nodeVar3 = fn48( nodeVar8 );
	nodeVar9 = ( nodeVar8 - f32( nodeVar3 ) );
	nodeVar10 = fn65( nodeVar5 );
	nodeVar11 = fn65( nodeVar7 );
	nodeVar12 = fn65( nodeVar9 );
	nodeVar13 = fn51( fn54( fn68( nodeVar1, nodeVar2, nodeVar3 ), nodeVar5, nodeVar7, nodeVar9 ), fn54( fn68( ( nodeVar1 + 1 ), nodeVar2, nodeVar3 ), ( nodeVar5 - 1.0 ), nodeVar7, nodeVar9 ), fn54( fn68( nodeVar1, ( nodeVar2 + 1 ), nodeVar3 ), nodeVar5, ( nodeVar7 - 1.0 ), nodeVar9 ), fn54( fn68( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ), nodeVar3 ), ( nodeVar5 - 1.0 ), ( nodeVar7 - 1.0 ), nodeVar9 ), fn54( fn68( nodeVar1, nodeVar2, ( nodeVar3 + 1 ) ), nodeVar5, nodeVar7, ( nodeVar9 - 1.0 ) ), fn54( fn68( ( nodeVar1 + 1 ), nodeVar2, ( nodeVar3 + 1 ) ), ( nodeVar5 - 1.0 ), nodeVar7, ( nodeVar9 - 1.0 ) ), fn54( fn68( nodeVar1, ( nodeVar2 + 1 ), ( nodeVar3 + 1 ) ), nodeVar5, ( nodeVar7 - 1.0 ), ( nodeVar9 - 1.0 ) ), fn54( fn68( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ), ( nodeVar3 + 1 ) ), ( nodeVar5 - 1.0 ), ( nodeVar7 - 1.0 ), ( nodeVar9 - 1.0 ) ), nodeVar10, nodeVar11, nodeVar12 );

	return fn58( nodeVar13 );

}


fn fn44 ( lightDistance : f32, cutoffDistance : f32, decayExponent : f32 ) -> f32 {

	var nodeVar0 : f32;


	if ( ( cutoffDistance > 0.0 ) ) {

		let nodeConst0 = ( 1.0 / max( pow( lightDistance, decayExponent ), 0.01 ) );
		let nodeConst1 = ( lightDistance / cutoffDistance );
		let nodeConst2 = clamp( ( 1.0 - ( ( ( nodeConst1 * nodeConst1 ) * nodeConst1 ) * nodeConst1 ) ), 0.0, 1.0 );
		nodeVar0 = ( nodeConst0 * ( nodeConst2 * nodeConst2 ) );

	} else {

		let nodeConst0 = ( 1.0 / max( pow( lightDistance, decayExponent ), 0.01 ) );
		nodeVar0 = nodeConst0;

	}


	return nodeVar0;

}


fn fn25 ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	

	let nodeConst0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeConst0 ), sin( nodeConst0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

}


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
fn main( @location( 0 ) v_positionWorld : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );

	if ( ( length( ( render.cameraPosition - v_positionWorld ) ) > ( object.nodeUniform3 * 2.0 ) ) ) {

		nodeVar0 = render.cameraPosition;
		nodeVar1 = v_positionWorld;
		nodeVar2 = true;
		

	} else {

		nodeVar0 = v_positionWorld;
		nodeVar1 = render.cameraPosition;
		nodeVar2 = false;
		

	}

	let nodeConst0 = ( nodeVar1 - nodeVar0 );
	nodeVar3 = ( length( nodeConst0 ) / f32( object.nodeUniform4 ) );
	nodeVar4 = normalize( nodeConst0 );
	nodeVar5 = 0.0;
	nodeVar6 = vec3<f32>( 1.0, 1.0, 1.0 );
	nodeVar5 = ( nodeVar5 + ( fract( ( fn24( fragCoord.xy ) + ( f32( render.nodeUniform5 ) * 0.618033988749895 ) ) ) * nodeVar3 ) );

	for ( var i : i32 = 0; i < object.nodeUniform4; i ++ ) {

		nodeVar7 = vec3<f32>( 0.0 );

		if ( ( 1.0 == 1.0 ) ) {

			nodeVar9 = vec3<f32>( 0.0, 0.0, 0.0 );
			let nodeConst1 = clamp( ( ( ( ( render.nodeUniform7 / 8.34 ) * 0.5 ) + 0.2 ) + render.nodeUniform8 ), 0.0, 1.0 );
			nodeVar9 = mix( vec3<f32>( 0.0, 0.0, 0.0 ), render.nodeUniform6, smoothstep( 0.05, 0.35, nodeConst1 ) );
			nodeVar9 = mix( nodeVar9, render.nodeUniform9, smoothstep( 0.35, 0.65, nodeConst1 ) );
			nodeVar9 = mix( nodeVar9, render.nodeUniform10, smoothstep( 0.65, 1.0, nodeConst1 ) );
			workingLuminanceCoefficients = vec3<f32>( 0.2126, 0.7152, 0.0722 );
			nodeVar8 = fn34( fn32( nodeVar9, render.nodeUniform11, workingLuminanceCoefficients ), render.nodeUniform12 );

		} else {

			nodeVar10 = vec3<f32>( 0.0, 0.0, 0.0 );
			let nodeConst2 = vec3<f32>( 0.0, render.nodeUniform15, 0.0 );
			let nodeConst3 = length( ( v_positionWorld - ( ( render.nodeUniform13 + render.nodeUniform14 ) + ( nodeConst2 * vec3<f32>( clamp( ( dot( ( v_positionWorld - render.nodeUniform13 ), nodeConst2 ) / dot( nodeConst2, nodeConst2 ) ), 0.0, 1.0 ) ) ) ) ) );
			let nodeConst4 = ( v_positionWorld.xz - render.nodeUniform13.xz );
			let nodeConst5 = atan2( nodeConst4.y, nodeConst4.x );
			let nodeConst6 = clamp( ( clamp( ( 1.0 - ( nodeConst3 / render.nodeUniform16 ) ), 0.0, 1.0 ) * mix( ( ( ( ( ( ( ( fn74( vec3<f32>( ( v_positionWorld.x * ( 0.6 * render.nodeUniform17 ) ), ( render.nodeUniform18 * 1.2 ), ( v_positionWorld.z * ( 0.6 * render.nodeUniform17 ) ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) * 0.65 ) + ( ( ( ( ( fn74( vec3<f32>( ( v_positionWorld.x * ( 1.5 * render.nodeUniform17 ) ), ( render.nodeUniform18 * 2.5 ), ( v_positionWorld.z * ( 1.5 * render.nodeUniform17 ) ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) * 0.35 ) ) * ( ( mix( 1.0, ( ( ( ( fn74( vec3<f32>( ( cos( nodeConst5 ) * ( 1.5 * render.nodeUniform17 ) ), ( sin( nodeConst5 ) * ( 1.5 * render.nodeUniform17 ) ), ( render.nodeUniform18 * 0.6 ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ), smoothstep( 0.0, render.nodeUniform19, length( nodeConst4 ) ) ) * 0.5 ) + 0.5 ) ), 1.0, clamp( ( nodeConst3 / render.nodeUniform20 ), 0.0, 1.0 ) ) ), 0.0, 1.0 );
			nodeVar10 = mix( vec3<f32>( 0.0, 0.0, 0.0 ), render.nodeUniform6, smoothstep( 0.05, 0.35, nodeConst6 ) );
			nodeVar10 = mix( nodeVar10, render.nodeUniform9, smoothstep( 0.35, 0.65, nodeConst6 ) );
			nodeVar10 = mix( nodeVar10, render.nodeUniform10, smoothstep( 0.65, 1.0, nodeConst6 ) );
			workingLuminanceCoefficients = vec3<f32>( 0.2126, 0.7152, 0.0722 );
			nodeVar8 = fn34( fn32( nodeVar10, render.nodeUniform11, workingLuminanceCoefficients ), render.nodeUniform12 );

		}


		if ( ( 1.0 == 1.0 ) ) {

			nodeVar11 = render.nodeUniform23;

		} else {

			nodeVar11 = render.nodeUniform24;

		}


		if ( ( 1.0 == 1.0 ) ) {

			let nodeConst2 = vec3<f32>( 0.0, render.nodeUniform15, 0.0 );
			let nodeConst3 = length( ( v_positionWorld - ( ( render.nodeUniform13 + render.nodeUniform14 ) + ( nodeConst2 * vec3<f32>( clamp( ( dot( ( v_positionWorld - render.nodeUniform13 ), nodeConst2 ) / dot( nodeConst2, nodeConst2 ) ), 0.0, 1.0 ) ) ) ) ) );
			nodeVar12 = ( ( 1.0 / ( pow( nodeConst3, 2.0 ) + pow( 1.2, 2.0 ) ) ) / ( 1.0 / max( pow( length( ( v_positionWorld - render.nodeUniform13 ) ), 2.0 ), 0.01 ) ) );

		} else {

			nodeVar12 = 1.0;

		}


		if ( ( 1.0 == 1.0 ) ) {

			let nodeConst2 = vec3<f32>( 0.0, render.nodeUniform15, 0.0 );
			let nodeConst3 = length( ( v_positionWorld - ( ( render.nodeUniform13 + render.nodeUniform14 ) + ( nodeConst2 * vec3<f32>( clamp( ( dot( ( v_positionWorld - render.nodeUniform13 ), nodeConst2 ) / dot( nodeConst2, nodeConst2 ) ), 0.0, 1.0 ) ) ) ) ) );
			nodeVar13 = mix( render.nodeUniform26, render.nodeUniform27, smoothstep( 0.0, 1.0, clamp( ( nodeConst3 / render.nodeUniform28 ), 0.0, 1.0 ) ) );

		} else {

			nodeVar13 = 1.0;

		}

		let nodeConst7 = ( nodeVar0 + ( nodeVar4 * vec3<f32>( nodeVar5 ) ) );
		nodeVar14 = ( ( ( ( ( ( ( ( ( nodeVar8 * vec3<f32>( pow( max( ( render.nodeUniform7 / 8.34 ), 0.0 ), 4.0 ) ) ) * vec3<f32>( max( ( render.nodeUniform21 / 11.02 ), 0.0 ) ) ) * vec3<f32>( render.nodeUniform22 ) ) * vec3<f32>( nodeVar11 ) ) * vec3<f32>( render.nodeUniform25 ) ) * vec3<f32>( smoothstep( 0.0, 3.0, render.nodeUniform18 ) ) ) * vec3<f32>( nodeVar12 ) ) * vec3<f32>( nodeVar13 ) ) * vec3<f32>( fn44( length( ( render.nodeUniform29 - ( render.cameraViewMatrix * vec4<f32>( nodeConst7, 1.0 ) ).xyz ) ), render.nodeUniform31, render.nodeUniform32 ) ) );
		nodeVar7 = ( nodeVar7 + nodeVar14 );
		shadowPositionWorld = nodeConst7;
		normalViewGeometry = normalize( v_normalViewGeometry );
		NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
		normalView = NORMAL_normalView;
		normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
		let nodeConst8 = ( render.nodeUniform35 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform38 ) ) ), 1.0 ) );
		let nodeConst9 = ( nodeConst8.xyz / vec3<f32>( nodeConst8.w ) );
		let nodeConst10 = vec3<f32>( nodeConst9.x, ( 1.0 - nodeConst9.y ), ( nodeConst9.z + render.nodeUniform39 ) );
		nodeVar15 = textureSample( nodeUniform34, nodeUniform34_sampler, nodeConst10.xy );

		if ( ( ( ( ( ( nodeConst10.x >= 0.0 ) && ( nodeConst10.x <= 1.0 ) ) && ( nodeConst10.y >= 0.0 ) ) && ( nodeConst10.y <= 1.0 ) ) && ( nodeConst10.z <= 1.0 ) ) ) {

			let nodeConst11 = ( fn24( fragCoord.xy ) * 6.28318530718 );
			let nodeConst12 = ( vec2<f32>( render.nodeUniform41 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform42 ) );
			let nodeConst13 = ( nodeConst10.xy + ( fn25( 0, 5, nodeConst11 ) * nodeConst12 ) );
			nodeVar17 = textureSampleCompare( nodeUniform40, nodeUniform40_sampler, nodeConst13, nodeConst10.z );
			let nodeConst14 = ( nodeConst10.xy + ( fn25( 1, 5, nodeConst11 ) * nodeConst12 ) );
			nodeVar18 = textureSampleCompare( nodeUniform40, nodeUniform40_sampler, nodeConst14, nodeConst10.z );
			let nodeConst15 = ( nodeConst10.xy + ( fn25( 2, 5, nodeConst11 ) * nodeConst12 ) );
			nodeVar19 = textureSampleCompare( nodeUniform40, nodeUniform40_sampler, nodeConst15, nodeConst10.z );
			let nodeConst16 = ( nodeConst10.xy + ( fn25( 3, 5, nodeConst11 ) * nodeConst12 ) );
			nodeVar20 = textureSampleCompare( nodeUniform40, nodeUniform40_sampler, nodeConst16, nodeConst10.z );
			let nodeConst17 = ( nodeConst10.xy + ( fn25( 4, 5, nodeConst11 ) * nodeConst12 ) );
			nodeVar21 = textureSampleCompare( nodeUniform40, nodeUniform40_sampler, nodeConst17, nodeConst10.z );
			nodeVar16 = ( ( ( ( ( nodeVar17 + nodeVar18 ) + nodeVar19 ) + nodeVar20 ) + nodeVar21 ) * 0.2 );

		} else {

			nodeVar16 = 1.0;

		}

		nodeVar22 = mix( vec4<f32>( 1.0 ), mix( nodeVar15, vec4<f32>( 1.0 ), vec4<f32>( nodeVar16 ) ), ( render.nodeUniform43 * nodeVar15.w ) );
		let nodeConst18 = ( render.nodeUniform46 - ( render.cameraViewMatrix * vec4<f32>( nodeConst7, 1.0 ) ).xyz );
		nodeVar23 = ( ( ( vec4<f32>( render.nodeUniform33, 1.0 ) * nodeVar22 ) * vec4<f32>( smoothstep( render.nodeUniform44, render.nodeUniform45, dot( normalize( nodeConst18 ), normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform47 - render.nodeUniform48 ), 0.0 ) ).xyz ) ) ) ) ) * vec4<f32>( fn44( length( nodeConst18 ), render.nodeUniform49, render.nodeUniform50 ) ) ).xyz;
		nodeVar23 = ( vec4<f32>( nodeVar23, 1.0 ) * nodeVar22 ).xyz;
		nodeVar7 = ( nodeVar7 + nodeVar23 );
		nodeVar24 = ( ( ( nodeConst7 - vec3<f32>( 0.0, 6.0, 0.0 ) ) / object.nodeUniform51 ) + vec3<f32>( 0.5 ) );
		nodeVar25 = textureSampleLevel( nodeUniform52, nodeUniform52_sampler, nodeVar24, 0.0 );
		nodeVar26 = clamp( ( nodeVar24 + ( ( nodeVar25.xyz / object.nodeUniform51 ) * vec3<f32>( 0.15 ) ) ), vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) );
		nodeVar27 = textureSampleLevel( nodeUniform53, nodeUniform53_sampler, nodeVar26, 0.0 );
		nodeVar27.x = ( nodeVar27.x * ( ( fn131( ( ( nodeConst7 * vec3<f32>( 5.5 ) ) + vec3<f32>( 0.0, ( - ( nodeVar27.z * 0.8 ) ), 0.0 ) ) ) * 0.35 ) + 0.85 ) );
		let nodeConst19 = min( nodeVar26, ( vec3<f32>( 1.0, 1.0, 1.0 ) - nodeVar26 ) );
		nodeVar27.x = ( nodeVar27.x * smoothstep( 0.0, 0.06, min( nodeConst19.x, min( nodeConst19.y, nodeConst19.z ) ) ) );
		nodeVar28 = 0.0;
		let nodeConst20 = normalize( ( object.nodeUniform55 - nodeConst7 ) );
		let nodeConst21 = ( ( ( ( nodeConst7 + ( nodeConst20 * vec3<f32>( 0.175 ) ) ) - vec3<f32>( 0.0, 6.0, 0.0 ) ) / object.nodeUniform51 ) + vec3<f32>( 0.5 ) );
		nodeVar29 = textureSampleLevel( nodeUniform54, nodeUniform54_sampler, nodeConst21, 0.0 );
		let nodeConst22 = min( nodeConst21, ( vec3<f32>( 1.0, 1.0, 1.0 ) - nodeConst21 ) );
		nodeVar28 = ( nodeVar28 + ( nodeVar29.x * smoothstep( 0.0, 0.06, min( nodeConst22.x, min( nodeConst22.y, nodeConst22.z ) ) ) ) );
		let nodeConst23 = ( ( ( ( nodeConst7 + ( nodeConst20 * vec3<f32>( 0.5249999999999999 ) ) ) - vec3<f32>( 0.0, 6.0, 0.0 ) ) / object.nodeUniform51 ) + vec3<f32>( 0.5 ) );
		nodeVar30 = textureSampleLevel( nodeUniform54, nodeUniform54_sampler, nodeConst23, 0.0 );
		let nodeConst24 = min( nodeConst23, ( vec3<f32>( 1.0, 1.0, 1.0 ) - nodeConst23 ) );
		nodeVar28 = ( nodeVar28 + ( nodeVar30.x * smoothstep( 0.0, 0.06, min( nodeConst24.x, min( nodeConst24.y, nodeConst24.z ) ) ) ) );
		let nodeConst25 = ( ( nodeVar28 * 0.35 ) * object.nodeUniform56 );
		let nodeConst26 = exp( ( - nodeConst25 ) );
		let nodeConst27 = mix( nodeConst26, ( nodeConst26 + ( exp( ( - ( nodeConst25 * 0.25 ) ) ) * 0.5 ) ), object.nodeUniform57 );
		let nodeConst28 = ( object.nodeUniform60 * object.nodeUniform60 );
		nodeVar7 = ( nodeVar7 * ( ( vec3<f32>( nodeVar27.x ) * vec3<f32>( clamp( ( mix( nodeConst27, ( nodeConst27 * ( 1.0 - exp( ( - ( nodeConst25 * 2.0 ) ) ) ) ), object.nodeUniform58 ) + object.nodeUniform59 ), 0.0, 1.0 ) ) ) * vec3<f32>( ( ( ( ( 1.0 - nodeConst28 ) / pow( ( ( 1.0 + nodeConst28 ) - ( ( 2.0 * object.nodeUniform60 ) * clamp( dot( normalize( ( render.cameraPosition - nodeConst7 ) ), nodeConst20 ), -1.0, 1.0 ) ) ), 1.5 ) ) * 0.079577 ) * 12.56637 ) ) ) );
		nodeVar31 = ( nodeVar7 * vec3<f32>( 0.01 ) );
		nodeVar32 = ( ( ( nodeConst7 - vec3<f32>( 0.0, 6.0, 0.0 ) ) / object.nodeUniform51 ) + vec3<f32>( 0.5 ) );
		nodeVar33 = textureSampleLevel( nodeUniform52, nodeUniform52_sampler, nodeVar32, 0.0 );
		nodeVar34 = clamp( ( nodeVar32 + ( ( nodeVar33.xyz / object.nodeUniform51 ) * vec3<f32>( 0.15 ) ) ), vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) );
		nodeVar35 = textureSampleLevel( nodeUniform53, nodeUniform53_sampler, nodeVar34, 0.0 );
		nodeVar35.x = ( nodeVar35.x * ( ( fn131( ( ( nodeConst7 * vec3<f32>( 5.5 ) ) + vec3<f32>( 0.0, ( - ( nodeVar35.z * 0.8 ) ), 0.0 ) ) ) * 0.35 ) + 0.85 ) );
		let nodeConst29 = min( nodeVar34, ( vec3<f32>( 1.0, 1.0, 1.0 ) - nodeVar34 ) );
		nodeVar35.x = ( nodeVar35.x * smoothstep( 0.0, 0.06, min( nodeConst29.x, min( nodeConst29.y, nodeConst29.z ) ) ) );
		nodeVar36 = vec3<f32>( 0.0, 0.0, 0.0 );
		let nodeConst30 = clamp( nodeVar35.y, 0.0, 1.0 );
		nodeVar36 = mix( vec3<f32>( 0.0, 0.0, 0.0 ), render.nodeUniform6, smoothstep( 0.05, 0.35, nodeConst30 ) );
		nodeVar36 = mix( nodeVar36, render.nodeUniform9, smoothstep( 0.35, 0.65, nodeConst30 ) );
		nodeVar36 = mix( nodeVar36, render.nodeUniform10, smoothstep( 0.65, 1.0, nodeConst30 ) );
		nodeVar31 = ( nodeVar31 + ( ( ( fn34( ( ( nodeVar36 * vec3<f32>( pow( nodeVar35.y, ( 6.0 - object.nodeUniform61 ) ) ) ) * vec3<f32>( render.nodeUniform22 ) ), render.nodeUniform12 ) * vec3<f32>( ( nodeVar35.x + 0.15 ) ) ) * vec3<f32>( ( 400.0 / pow( length( ( nodeConst7 - object.nodeUniform55 ) ), 2.0 ) ) ) ) * vec3<f32>( 0.01 ) ) );

		if ( nodeVar2 ) {

			nodeVar37 = ( nodeVar37 + ( ( nodeVar31 * nodeVar6 ) * vec3<f32>( nodeVar3 ) ) );
			

		} else {

			let nodeConst31 = exp( ( ( - ( nodeVar7 * vec3<f32>( 0.01 ) ) ) * vec3<f32>( nodeVar3 ) ) );
			nodeVar37 = ( ( nodeVar37 * nodeConst31 ) + ( nodeVar31 * vec3<f32>( nodeVar3 ) ) );
			

		}

		let nodeConst31 = exp( ( ( - ( nodeVar7 * vec3<f32>( 0.01 ) ) ) * vec3<f32>( nodeVar3 ) ) );
		nodeVar6 = ( nodeVar6 * nodeConst31 );
		nodeVar5 = ( nodeVar5 + nodeVar3 );

	}

	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar37;
	let nodeConst32 = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst32;

	// result

	output.color = nodeConst32;

	return output;

}
