// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform4 : mat3x3<f32>,
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
var<private> nodeVar1 : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : f32;
var<private> Output : vec4<f32>;
var<private> nodeVar8 : vec4<f32>;

// codes
fn mx_floor ( x : f32 ) -> i32 {

	var nodeVar0 : f32;

	nodeVar0 = x;

	return i32( floor( nodeVar0 ) );

}


fn mx_fade ( t : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = t;

	return ( ( ( nodeVar0 * nodeVar0 ) * nodeVar0 ) * ( ( nodeVar0 * ( ( nodeVar0 * 6.0 ) - 15.0 ) ) + 10.0 ) );

}


fn mx_rotl32 ( x : u32, k : i32 ) -> u32 {

	var nodeVar0 : i32;
	var nodeVar1 : u32;

	nodeVar0 = k;
	nodeVar1 = x;

	return ( ( nodeVar1 << u32( nodeVar0 ) ) | ( nodeVar1 >> u32( ( 32 - nodeVar0 ) ) ) );

}


fn mx_bjfinal ( a : u32, b : u32, c : u32 ) -> u32 {

	var nodeVar0 : u32;
	var nodeVar1 : u32;
	var nodeVar2 : u32;

	nodeVar0 = c;
	nodeVar1 = b;
	nodeVar2 = a;
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - mx_rotl32( nodeVar1, 14 ) );
	nodeVar2 = ( nodeVar2 ^ nodeVar0 );
	nodeVar2 = ( nodeVar2 - mx_rotl32( nodeVar0, 11 ) );
	nodeVar1 = ( nodeVar1 ^ nodeVar2 );
	nodeVar1 = ( nodeVar1 - mx_rotl32( nodeVar2, 25 ) );
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - mx_rotl32( nodeVar1, 16 ) );
	nodeVar2 = ( nodeVar2 ^ nodeVar0 );
	nodeVar2 = ( nodeVar2 - mx_rotl32( nodeVar0, 4 ) );
	nodeVar1 = ( nodeVar1 ^ nodeVar2 );
	nodeVar1 = ( nodeVar1 - mx_rotl32( nodeVar2, 14 ) );
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - mx_rotl32( nodeVar1, 24 ) );

	return nodeVar0;

}


fn mx_hash_int_1 ( x : i32, y : i32 ) -> u32 {

	var nodeVar0 : i32;
	var nodeVar1 : i32;
	var nodeVar2 : u32;
	var nodeVar3 : u32;
	var nodeVar4 : u32;
	var nodeVar5 : u32;

	nodeVar0 = y;
	nodeVar1 = x;
	nodeVar2 = 2u;
	nodeVar3 = 0u;
	nodeVar4 = 0u;
	nodeVar5 = 0u;
	nodeVar5 = ( ( 3735928559u + ( nodeVar2 << 2u ) ) + 13u );
	nodeVar4 = nodeVar5;
	nodeVar3 = nodeVar4;
	nodeVar3 = ( nodeVar3 + u32( nodeVar1 ) );
	nodeVar4 = ( nodeVar4 + u32( nodeVar0 ) );

	return mx_bjfinal( nodeVar3, nodeVar4, nodeVar5 );

}


fn mx_select ( b : bool, t : f32, f : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : bool;
	var nodeVar3 : f32;

	nodeVar0 = f;
	nodeVar1 = t;
	nodeVar2 = b;

	return select( nodeVar0, nodeVar1, nodeVar2 );

}


fn mx_negate_if ( val : f32, b : bool ) -> f32 {

	var nodeVar0 : bool;
	var nodeVar1 : f32;
	var nodeVar2 : f32;

	nodeVar0 = b;
	nodeVar1 = val;

	return select( nodeVar1, ( - nodeVar1 ), nodeVar0 );

}


fn mx_gradient_float_0 ( hash : u32, x : f32, y : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : u32;
	var nodeVar3 : u32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;

	nodeVar0 = y;
	nodeVar1 = x;
	nodeVar2 = hash;
	nodeVar3 = ( nodeVar2 & 7u );
	nodeVar4 = mx_select( ( nodeVar3 < 4u ), nodeVar1, nodeVar0 );
	nodeVar5 = ( 2.0 * mx_select( ( nodeVar3 < 4u ), nodeVar0, nodeVar1 ) );

	return ( mx_negate_if( nodeVar4, bool( ( nodeVar3 & 1u ) ) ) + mx_negate_if( nodeVar5, bool( ( nodeVar3 & 2u ) ) ) );

}


fn mx_bilerp_0 ( v0 : f32, v1 : f32, v2 : f32, v3 : f32, s : f32, t : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;

	nodeVar0 = t;
	nodeVar1 = s;
	nodeVar2 = v3;
	nodeVar3 = v2;
	nodeVar4 = v1;
	nodeVar5 = v0;
	nodeVar6 = ( 1.0 - nodeVar1 );

	return ( ( ( 1.0 - nodeVar0 ) * ( ( nodeVar5 * nodeVar6 ) + ( nodeVar4 * nodeVar1 ) ) ) + ( nodeVar0 * ( ( nodeVar3 * nodeVar6 ) + ( nodeVar2 * nodeVar1 ) ) ) );

}


fn mx_gradient_scale2d_0 ( v : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = v;

	return ( 0.6616 * nodeVar0 );

}


fn mx_perlin_noise_float_0 ( p : vec2<f32> ) -> f32 {

	var nodeVar0 : vec2<f32>;
	var nodeVar1 : i32;
	var nodeVar2 : i32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;
	var nodeVar7 : f32;
	var nodeVar8 : f32;
	var nodeVar9 : f32;

	nodeVar0 = p;
	nodeVar1 = 0;
	nodeVar2 = 0;
	nodeVar3 = nodeVar0.x;
	nodeVar1 = mx_floor( nodeVar3 );
	nodeVar4 = ( nodeVar3 - f32( nodeVar1 ) );
	nodeVar5 = nodeVar0.y;
	nodeVar2 = mx_floor( nodeVar5 );
	nodeVar6 = ( nodeVar5 - f32( nodeVar2 ) );
	nodeVar7 = mx_fade( nodeVar4 );
	nodeVar8 = mx_fade( nodeVar6 );
	nodeVar9 = mx_bilerp_0( mx_gradient_float_0( mx_hash_int_1( nodeVar1, nodeVar2 ), nodeVar4, nodeVar6 ), mx_gradient_float_0( mx_hash_int_1( ( nodeVar1 + 1 ), nodeVar2 ), ( nodeVar4 - 1.0 ), nodeVar6 ), mx_gradient_float_0( mx_hash_int_1( nodeVar1, ( nodeVar2 + 1 ) ), nodeVar4, ( nodeVar6 - 1.0 ) ), mx_gradient_float_0( mx_hash_int_1( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ) ), ( nodeVar4 - 1.0 ), ( nodeVar6 - 1.0 ) ), nodeVar7, nodeVar8 );

	return mx_gradient_scale2d_0( nodeVar9 );

}




@fragment
fn main( @location( 0 ) v_positionWorld : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = ( ( mx_perlin_noise_float_0( ( v_positionWorld.xz * vec2<f32>( 0.012 ) ) ) * 1.0 ) + 0.0 );
	nodeVar1 = clamp( ( ( v_positionWorld.y - object.nodeUniform1 ) / ( object.nodeUniform2 - object.nodeUniform1 ) ), 0.0, 1.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
	normalView = NORMAL_normalView;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar2 = clamp( normalWorld.y, 0.0, 1.0 );
	nodeVar3 = ( ( mx_perlin_noise_float_0( ( v_positionWorld.xz * vec2<f32>( 0.18 ) ) ) * 1.0 ) + 0.0 );
	nodeVar4 = ( 1.0 - nodeVar2 );
	nodeVar5 = ( ( mx_perlin_noise_float_0( ( v_positionWorld.xz * vec2<f32>( 0.05 ) ) ) * 1.0 ) + 0.0 );
	nodeVar6 = ( ( mix( vec3<f32>( 0.16513219449147767, 0.15592646369776456, 0.13843161502267545 ), vec3<f32>( 0.1499597898006365, 0.171441100722554, 0.09084171117479915 ), ( ( ( smoothstep( 0.45, 0.72, nodeVar3 ) * smoothstep( 0.62, 0.32, nodeVar4 ) ) * smoothstep( 0.66, 0.34, nodeVar1 ) ) * 0.45 ) ) * vec3<f32>( ( ( ( ( ( ( sin( ( ( ( ( ( v_positionWorld.y * 0.5 ) + ( v_positionWorld.x * 0.08 ) ) + ( v_positionWorld.z * 0.05 ) ) + ( nodeVar5 * 3.0 ) ) + ( nodeVar0 * 4.0 ) ) ) * 0.6 ) + ( sin( ( ( v_positionWorld.y * 1.4 ) + ( nodeVar3 * 2.0 ) ) ) * 0.4 ) ) * 0.5 ) + 0.5 ) * 0.36 ) + 0.8 ) ) ) * vec3<f32>( ( ( nodeVar3 * 0.18 ) + 1.0 ) ) );
	nodeVar7 = smoothstep( 180.0, 820.0, distance( v_positionWorld, render.cameraPosition ) );
	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), ( 1.0 * vec4<f32>( mix( max( mix( vec3<f32>( dot( ( ( ( mix( mix( mix( mix( mix( mix( vec3<f32>( 0.15592646369776456, 0.16826940017946088, 0.08650046202808521 ), vec3<f32>( 0.2541520943200296, 0.23455058215026167, 0.08021982030622662 ), ( smoothstep( 0.15, 0.75, nodeVar0 ) * smoothstep( 0.22, 0.5, nodeVar1 ) ) ), vec3<f32>( 0.040915196900556984, 0.05126945836711539, 0.028426039499072558 ), ( ( smoothstep( 0.16, 0.34, nodeVar1 ) * smoothstep( 0.5, 0.72, nodeVar2 ) ) * 0.75 ) ), nodeVar6, smoothstep( 0.46, 0.64, ( nodeVar1 + ( nodeVar5 * 0.06 ) ) ) ), nodeVar6, smoothstep( 0.34, 0.62, nodeVar4 ) ), vec3<f32>( 0.22696587349938613, 0.19461783043107173, 0.158960835050774 ), ( ( ( smoothstep( 0.42, 0.7, nodeVar4 ) * smoothstep( 0.35, 0.7, nodeVar2 ) ) * ( ( nodeVar5 * 0.5 ) + 0.5 ) ) * 0.5 ) ), mix( vec3<f32>( 0.8148465722120952, 0.8387990117372213, 0.8713671191959567 ), vec3<f32>( 0.6038273388475408, 0.6724431569510133, 0.76052450467022 ), ( smoothstep( 0.2, 0.7, nodeVar3 ) * 0.6 ) ), ( smoothstep( 0.56, 0.78, ( ( nodeVar1 + ( nodeVar5 * 0.08 ) ) + ( nodeVar3 * 0.05 ) ) ) * smoothstep( 0.45, 0.75, nodeVar2 ) ) ) * vec3<f32>( ( 1.0 - ( ( smoothstep( 0.24, 0.06, nodeVar1 ) * nodeVar2 ) * 0.32 ) ) ) ) * vec3<f32>( ( ( ( ( nodeVar0 * 0.5 ) + 0.5 ) * 0.3 ) + 0.84 ) ) ) * vec3<f32>( ( ( ( ( nodeVar3 * 0.5 ) + 0.5 ) * 0.12 ) + 0.94 ) ) ), vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), ( ( ( mix( mix( mix( mix( mix( mix( vec3<f32>( 0.15592646369776456, 0.16826940017946088, 0.08650046202808521 ), vec3<f32>( 0.2541520943200296, 0.23455058215026167, 0.08021982030622662 ), ( smoothstep( 0.15, 0.75, nodeVar0 ) * smoothstep( 0.22, 0.5, nodeVar1 ) ) ), vec3<f32>( 0.040915196900556984, 0.05126945836711539, 0.028426039499072558 ), ( ( smoothstep( 0.16, 0.34, nodeVar1 ) * smoothstep( 0.5, 0.72, nodeVar2 ) ) * 0.75 ) ), nodeVar6, smoothstep( 0.46, 0.64, ( nodeVar1 + ( nodeVar5 * 0.06 ) ) ) ), nodeVar6, smoothstep( 0.34, 0.62, nodeVar4 ) ), vec3<f32>( 0.22696587349938613, 0.19461783043107173, 0.158960835050774 ), ( ( ( smoothstep( 0.42, 0.7, nodeVar4 ) * smoothstep( 0.35, 0.7, nodeVar2 ) ) * ( ( nodeVar5 * 0.5 ) + 0.5 ) ) * 0.5 ) ), mix( vec3<f32>( 0.8148465722120952, 0.8387990117372213, 0.8713671191959567 ), vec3<f32>( 0.6038273388475408, 0.6724431569510133, 0.76052450467022 ), ( smoothstep( 0.2, 0.7, nodeVar3 ) * 0.6 ) ), ( smoothstep( 0.56, 0.78, ( ( nodeVar1 + ( nodeVar5 * 0.08 ) ) + ( nodeVar3 * 0.05 ) ) ) * smoothstep( 0.45, 0.75, nodeVar2 ) ) ) * vec3<f32>( ( 1.0 - ( ( smoothstep( 0.24, 0.06, nodeVar1 ) * nodeVar2 ) * 0.32 ) ) ) ) * vec3<f32>( ( ( ( ( nodeVar0 * 0.5 ) + 0.5 ) * 0.3 ) + 0.84 ) ) ) * vec3<f32>( ( ( ( ( nodeVar3 * 0.5 ) + 0.5 ) * 0.12 ) + 0.94 ) ) ), ( ( ( 1.0 - nodeVar7 ) * 0.5 ) + 0.5 ) ), vec3<f32>( 0.0 ) ), vec3<f32>( 0.623960391667596, 0.5775804404214573, 0.4910208498384856 ), ( nodeVar7 * 0.62 ) ), 1.0 ).w ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform7 );
	nodeVar8 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar8;

	// result

	output.color = nodeVar8;

	return output;

}
