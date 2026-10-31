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
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> workingLuminanceCoefficients : vec3<f32>;
var<private> Output : vec4<f32>;

// codes
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


fn fn67 ( x : i32, y : i32 ) -> u32 {

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

	return fn63( nodeVar3, nodeVar4, nodeVar5 );

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


fn fn53 ( hash : u32, x : f32, y : f32 ) -> f32 {

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
	nodeVar4 = fn46( ( nodeVar3 < 4u ), nodeVar1, nodeVar0 );
	nodeVar5 = ( 2.0 * fn46( ( nodeVar3 < 4u ), nodeVar0, nodeVar1 ) );

	return ( fn47( nodeVar4, bool( ( nodeVar3 & 1u ) ) ) + fn47( nodeVar5, bool( ( nodeVar3 & 2u ) ) ) );

}


fn fn49 ( v0 : f32, v1 : f32, v2 : f32, v3 : f32, s : f32, t : f32 ) -> f32 {

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


fn fn57 ( v : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = v;

	return ( 0.6616 * nodeVar0 );

}


fn fn73 ( p : vec2<f32> ) -> f32 {

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
	nodeVar1 = fn48( nodeVar3 );
	nodeVar4 = ( nodeVar3 - f32( nodeVar1 ) );
	nodeVar5 = nodeVar0.y;
	nodeVar2 = fn48( nodeVar5 );
	nodeVar6 = ( nodeVar5 - f32( nodeVar2 ) );
	nodeVar7 = fn65( nodeVar4 );
	nodeVar8 = fn65( nodeVar6 );
	nodeVar9 = fn49( fn53( fn67( nodeVar1, nodeVar2 ), nodeVar4, nodeVar6 ), fn53( fn67( ( nodeVar1 + 1 ), nodeVar2 ), ( nodeVar4 - 1.0 ), nodeVar6 ), fn53( fn67( nodeVar1, ( nodeVar2 + 1 ) ), nodeVar4, ( nodeVar6 - 1.0 ) ), fn53( fn67( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ) ), ( nodeVar4 - 1.0 ), ( nodeVar6 - 1.0 ) ), nodeVar7, nodeVar8 );

	return fn57( nodeVar9 );

}


fn fn32 ( color : vec3<f32>, adjustment : f32, luminanceCoefficients : vec3<f32> ) -> vec3<f32> {

	


	return max( mix( vec3<f32>( dot( color, luminanceCoefficients ) ), color, adjustment ), vec3<f32>( 0.0 ) );

}




@fragment
fn main( @location( 0 ) v_positionWorld : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	let nodeConst0 = ( ( fn73( ( v_positionWorld.xz * vec2<f32>( 0.012 ) ) ) * 1.0 ) + 0.0 );
	let nodeConst1 = clamp( ( ( v_positionWorld.y - object.nodeUniform1 ) / ( object.nodeUniform2 - object.nodeUniform1 ) ), 0.0, 1.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
	normalView = NORMAL_normalView;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst2 = clamp( normalWorld.y, 0.0, 1.0 );
	let nodeConst3 = ( ( fn73( ( v_positionWorld.xz * vec2<f32>( 0.18 ) ) ) * 1.0 ) + 0.0 );
	let nodeConst4 = ( 1.0 - nodeConst2 );
	let nodeConst5 = ( ( fn73( ( v_positionWorld.xz * vec2<f32>( 0.05 ) ) ) * 1.0 ) + 0.0 );
	let nodeConst6 = ( ( mix( vec3<f32>( 0.16513219449147767, 0.15592646369776456, 0.13843161502267545 ), vec3<f32>( 0.1499597898006365, 0.171441100722554, 0.09084171117479915 ), ( ( ( smoothstep( 0.45, 0.72, nodeConst3 ) * smoothstep( 0.62, 0.32, nodeConst4 ) ) * smoothstep( 0.66, 0.34, nodeConst1 ) ) * 0.45 ) ) * vec3<f32>( ( ( ( ( ( ( sin( ( ( ( ( ( v_positionWorld.y * 0.5 ) + ( v_positionWorld.x * 0.08 ) ) + ( v_positionWorld.z * 0.05 ) ) + ( nodeConst5 * 3.0 ) ) + ( nodeConst0 * 4.0 ) ) ) * 0.6 ) + ( sin( ( ( v_positionWorld.y * 1.4 ) + ( nodeConst3 * 2.0 ) ) ) * 0.4 ) ) * 0.5 ) + 0.5 ) * 0.36 ) + 0.8 ) ) ) * vec3<f32>( ( ( nodeConst3 * 0.18 ) + 1.0 ) ) );
	let nodeConst7 = smoothstep( 180.0, 820.0, distance( v_positionWorld, render.cameraPosition ) );
	workingLuminanceCoefficients = vec3<f32>( 0.2126, 0.7152, 0.0722 );
	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), ( 1.0 * vec4<f32>( mix( fn32( ( ( ( mix( mix( mix( mix( mix( mix( vec3<f32>( 0.15592646369776456, 0.16826940017946088, 0.08650046202808521 ), vec3<f32>( 0.2541520943200296, 0.23455058215026167, 0.08021982030622662 ), ( smoothstep( 0.15, 0.75, nodeConst0 ) * smoothstep( 0.22, 0.5, nodeConst1 ) ) ), vec3<f32>( 0.040915196900556984, 0.05126945836711539, 0.028426039499072558 ), ( ( smoothstep( 0.16, 0.34, nodeConst1 ) * smoothstep( 0.5, 0.72, nodeConst2 ) ) * 0.75 ) ), nodeConst6, smoothstep( 0.46, 0.64, ( nodeConst1 + ( nodeConst5 * 0.06 ) ) ) ), nodeConst6, smoothstep( 0.34, 0.62, nodeConst4 ) ), vec3<f32>( 0.22696587349938613, 0.19461783043107173, 0.158960835050774 ), ( ( ( smoothstep( 0.42, 0.7, nodeConst4 ) * smoothstep( 0.35, 0.7, nodeConst2 ) ) * ( ( nodeConst5 * 0.5 ) + 0.5 ) ) * 0.5 ) ), mix( vec3<f32>( 0.8148465722120952, 0.8387990117372213, 0.8713671191959567 ), vec3<f32>( 0.6038273388475408, 0.6724431569510133, 0.76052450467022 ), ( smoothstep( 0.2, 0.7, nodeConst3 ) * 0.6 ) ), ( smoothstep( 0.56, 0.78, ( ( nodeConst1 + ( nodeConst5 * 0.08 ) ) + ( nodeConst3 * 0.05 ) ) ) * smoothstep( 0.45, 0.75, nodeConst2 ) ) ) * vec3<f32>( ( 1.0 - ( ( smoothstep( 0.24, 0.06, nodeConst1 ) * nodeConst2 ) * 0.32 ) ) ) ) * vec3<f32>( ( ( ( ( nodeConst0 * 0.5 ) + 0.5 ) * 0.3 ) + 0.84 ) ) ) * vec3<f32>( ( ( ( ( nodeConst3 * 0.5 ) + 0.5 ) * 0.12 ) + 0.94 ) ) ), ( ( ( 1.0 - nodeConst7 ) * 0.5 ) + 0.5 ), workingLuminanceCoefficients ), vec3<f32>( 0.623960391667596, 0.5775804404214573, 0.4910208498384856 ), ( nodeConst7 * 0.62 ) ), 1.0 ).w ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform7 );
	let nodeConst8 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst8;

	// result

	output.color = nodeConst8;

	return output;

}
