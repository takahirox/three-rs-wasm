// Three.js r186 - Node System

// directives


// structs


// uniforms

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : u32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : u32,
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

// varyings

struct VaryingsStruct {
	@location( 0 ) positionLocal : vec3<f32>,
	@location( 1 ) @interpolate( flat, either ) nodeVarying3 : f32,
	@location( 2 ) @interpolate( flat, either ) nodeVarying4 : vec3<f32>,
	@location( 3 ) @interpolate( flat, either ) nodeVarying5 : vec3<f32>,
	@location( 4 ) v_positionWorld : vec3<f32>,
	@location( 5 ) nodeVarying7 : vec3<f32>,
	@location( 6 ) nodeVarying8 : f32,
	@location( 7 ) v_normalWorldGeometry : vec3<f32>,
	@location( 8 ) nodeVarying12 : vec2<f32>,
	@location( 9 ) nodeVarying13 : vec3<f32>,
	@location( 10 ) nodeVarying14 : vec2<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> nodeVar411 : f32;
var<private> nodeVar412 : f32;
var<private> nodeVar413 : f32;
var<private> nodeVar414 : f32;
var<private> nodeVar415 : u32;
var<private> nodeVar416 : u32;
var<private> nodeVar417 : u32;
var<private> nodeVar418 : u32;
var<private> nodeVar419 : f32;
var<private> nodeVar420 : u32;
var<private> nodeVar421 : u32;
var<private> nodeVar422 : vec3<f32>;
var<private> nodeVar435 : f32;
var<private> normalLocal : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> VERTEX_nodeVar467 : vec4<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec3<f32>;
var<private> v_normalViewGeometry : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

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


fn mx_hash_int_2 ( x : i32, y : i32, z : i32 ) -> u32 {

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

	return mx_bjfinal( nodeVar4, nodeVar5, nodeVar6 );

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


fn mx_gradient_float_1 ( hash : u32, x : f32, y : f32, z : f32 ) -> f32 {

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
	nodeVar5 = mx_select( ( nodeVar4 < 8u ), nodeVar2, nodeVar1 );
	nodeVar6 = mx_select( ( nodeVar4 < 4u ), nodeVar1, mx_select( ( ( nodeVar4 == 12u ) || ( nodeVar4 == 14u ) ), nodeVar2, nodeVar0 ) );

	return ( mx_negate_if( nodeVar5, bool( ( nodeVar4 & 1u ) ) ) + mx_negate_if( nodeVar6, bool( ( nodeVar4 & 2u ) ) ) );

}


fn mx_trilerp_0 ( v0 : f32, v1 : f32, v2 : f32, v3 : f32, v4 : f32, v5 : f32, v6 : f32, v7 : f32, s : f32, t : f32, r : f32 ) -> f32 {

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


fn mx_gradient_scale3d_0 ( v : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = v;

	return ( 0.982 * nodeVar0 );

}


fn mx_perlin_noise_float_1 ( p : vec3<f32> ) -> f32 {

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
	nodeVar1 = mx_floor( nodeVar4 );
	nodeVar5 = ( nodeVar4 - f32( nodeVar1 ) );
	nodeVar6 = nodeVar0.y;
	nodeVar2 = mx_floor( nodeVar6 );
	nodeVar7 = ( nodeVar6 - f32( nodeVar2 ) );
	nodeVar8 = nodeVar0.z;
	nodeVar3 = mx_floor( nodeVar8 );
	nodeVar9 = ( nodeVar8 - f32( nodeVar3 ) );
	nodeVar10 = mx_fade( nodeVar5 );
	nodeVar11 = mx_fade( nodeVar7 );
	nodeVar12 = mx_fade( nodeVar9 );
	nodeVar13 = mx_trilerp_0( mx_gradient_float_1( mx_hash_int_2( nodeVar1, nodeVar2, nodeVar3 ), nodeVar5, nodeVar7, nodeVar9 ), mx_gradient_float_1( mx_hash_int_2( ( nodeVar1 + 1 ), nodeVar2, nodeVar3 ), ( nodeVar5 - 1.0 ), nodeVar7, nodeVar9 ), mx_gradient_float_1( mx_hash_int_2( nodeVar1, ( nodeVar2 + 1 ), nodeVar3 ), nodeVar5, ( nodeVar7 - 1.0 ), nodeVar9 ), mx_gradient_float_1( mx_hash_int_2( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ), nodeVar3 ), ( nodeVar5 - 1.0 ), ( nodeVar7 - 1.0 ), nodeVar9 ), mx_gradient_float_1( mx_hash_int_2( nodeVar1, nodeVar2, ( nodeVar3 + 1 ) ), nodeVar5, nodeVar7, ( nodeVar9 - 1.0 ) ), mx_gradient_float_1( mx_hash_int_2( ( nodeVar1 + 1 ), nodeVar2, ( nodeVar3 + 1 ) ), ( nodeVar5 - 1.0 ), nodeVar7, ( nodeVar9 - 1.0 ) ), mx_gradient_float_1( mx_hash_int_2( nodeVar1, ( nodeVar2 + 1 ), ( nodeVar3 + 1 ) ), nodeVar5, ( nodeVar7 - 1.0 ), ( nodeVar9 - 1.0 ) ), mx_gradient_float_1( mx_hash_int_2( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ), ( nodeVar3 + 1 ) ), ( nodeVar5 - 1.0 ), ( nodeVar7 - 1.0 ), ( nodeVar9 - 1.0 ) ), nodeVar10, nodeVar11, nodeVar12 );

	return mx_gradient_scale3d_0( nodeVar13 );

}


fn mx_fractal_noise_float ( p : vec3<f32>, octaves : i32, lacunarity : f32, diminish : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : vec3<f32>;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : i32;

	nodeVar0 = diminish;
	nodeVar1 = lacunarity;
	nodeVar2 = p;
	nodeVar3 = 0.0;
	nodeVar4 = 1.0;
	nodeVar5 = octaves;

	for ( var i : i32 = 0; i < nodeVar5; i ++ ) {

		nodeVar3 = ( nodeVar3 + ( nodeVar4 * mx_perlin_noise_float_1( nodeVar2 ) ) );
		nodeVar4 = ( nodeVar4 * nodeVar0 );
		nodeVar2 = ( nodeVar2 * vec3<f32>( nodeVar1 ) );

	}


	return nodeVar3;

}




@vertex
fn main( @location( 0 ) partId : f32,
	@location( 1 ) roomCenter : vec3<f32>,
	@location( 2 ) roomSize : vec2<f32>,
	@location( 3 ) position : vec3<f32>,
	@location( 4 ) normal : vec3<f32>,
	@location( 5 ) uv : vec2<f32> ) -> VaryingsStruct {

	// flow
	// code

	varyings.nodeVarying3 = partId;
	varyings.nodeVarying4 = roomCenter;
	varyings.nodeVarying12 = roomSize;
	varyings.positionLocal = position;
	varyings.nodeVarying13 = normal;
	varyings.nodeVarying5 = roomCenter;
	varyings.v_positionWorld = ( object.nodeUniform2 * vec4<f32>( varyings.positionLocal, 1.0 ) ).xyz;
	varyings.nodeVarying14 = uv;
	nodeVar411 = ( varyings.v_positionWorld.x + 101.0 );
	nodeVar412 = floor( ( nodeVar411 / 112.0 ) );
	nodeVar413 = ( varyings.v_positionWorld.z + 71.0 );
	nodeVar414 = floor( ( nodeVar413 / 82.0 ) );
	nodeVar415 = object.nodeUniform3;
	nodeVar416 = ( ( ( u32( ( ( ( nodeVar412 * 3.0 ) + clamp( floor( ( ( ( nodeVar411 - ( nodeVar412 * 112.0 ) ) - 5.0 ) / 26.666666666666668 ) ), 0.0, 2.0 ) ) + 4096.0 ) ) * 73856093u ) ^ ( u32( ( ( ( nodeVar414 * 2.0 ) + clamp( floor( ( ( ( nodeVar413 - ( nodeVar414 * 82.0 ) ) - 5.0 ) / 25.0 ) ), 0.0, 1.0 ) ) + 4096.0 ) ) * 19349663u ) ) ^ ( nodeVar415 * 2654435761u ) );
	nodeVar417 = ( ( ( nodeVar416 + 230900u ) * 747796405u ) + 2891336453u );
	nodeVar418 = ( ( ( nodeVar417 >> ( ( nodeVar417 >> 28u ) + 4u ) ) ^ nodeVar417 ) * 277803737u );
	nodeVar419 = ( f32( ( ( nodeVar418 >> 22u ) ^ nodeVar418 ) ) * 2.3283064365386963e-10 );
	nodeVar420 = ( ( ( nodeVar416 + 155260u ) * 747796405u ) + 2891336453u );
	nodeVar421 = ( ( ( nodeVar420 >> ( ( nodeVar420 >> 28u ) + 4u ) ) ^ nodeVar420 ) * 277803737u );
	nodeVar422 = ( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( vec3<f32>( 0.3915724777393922, 0.09084171117479915, 0.04518620437910499 ), vec3<f32>( 0.33245153633549385, 0.06847816983662762, 0.0343398068028541 ), step( 0.058823529411764705, nodeVar419 ) ), vec3<f32>( 0.2541520943200296, 0.14412847084818123, 0.08437621153575764 ), step( 0.11764705882352941, nodeVar419 ) ), vec3<f32>( 0.20507873637973145, 0.12743768042608497, 0.08021982030622662 ), step( 0.17647058823529413, nodeVar419 ) ), vec3<f32>( 0.55201140150344, 0.36625259558833256, 0.16202937562896222 ), step( 0.23529411764705882, nodeVar419 ) ), vec3<f32>( 0.4793201830913402, 0.3231432091022285, 0.158960835050774 ), step( 0.29411764705882354, nodeVar419 ) ), vec3<f32>( 0.5394794890033748, 0.4396571738310091, 0.22696587349938613 ), step( 0.35294117647058826, nodeVar419 ) ), vec3<f32>( 0.5647115056965487, 0.5271151256969157, 0.4452011945063733 ), step( 0.4117647058823529, nodeVar419 ) ), vec3<f32>( 0.5647115056965487, 0.5271151256969157, 0.4452011945063733 ), step( 0.47058823529411764, nodeVar419 ) ), vec3<f32>( 0.508881320845802, 0.4735314961384573, 0.3915724777393922 ), step( 0.5294117647058824, nodeVar419 ) ), vec3<f32>( 0.6375968739867731, 0.6038273388475408, 0.514917665367466 ), step( 0.5882352941176471, nodeVar419 ) ), vec3<f32>( 0.45641102317066595, 0.4286904966038916, 0.35640014413537763 ), step( 0.6470588235294118, nodeVar419 ) ), vec3<f32>( 0.3231432091022285, 0.3139887133649649, 0.2746773120495699 ), step( 0.7058823529411765, nodeVar419 ) ), vec3<f32>( 0.25818285291079235, 0.2501582847191642, 0.22696587349938613 ), step( 0.7647058823529411, nodeVar419 ) ), vec3<f32>( 0.37626212298046485, 0.36625259558833256, 0.3231432091022285 ), step( 0.8235294117647058, nodeVar419 ) ), vec3<f32>( 0.7083757798856457, 0.6724431569510133, 0.5972017883558645 ), step( 0.8823529411764706, nodeVar419 ) ), vec3<f32>( 0.20155625378383743, 0.23839757380151394, 0.2663556047920505 ), step( 0.9411764705882353, nodeVar419 ) ) * vec3<f32>( ( ( ( f32( ( ( nodeVar421 >> 22u ) ^ nodeVar421 ) ) * 2.3283064365386963e-10 ) * 0.12 ) + 0.94 ) ) );
	varyings.nodeVarying7 = nodeVar422;
	nodeVar435 = ( mx_fractal_noise_float( ( varyings.v_positionWorld * vec3<f32>( 0.03 ) ), 2, 2.0, 0.5 ) * 1.0 );
	varyings.nodeVarying8 = nodeVar435;
	normalLocal = normal;
	v_normalViewGeometry = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform5 * normalLocal ), 0.0 ) ).xyz );
	normalViewGeometry = normalize( v_normalViewGeometry );
	varyings.v_normalWorldGeometry = normalize( ( vec4<f32>( normalViewGeometry, 0.0 ) * render.cameraViewMatrix ).xyz );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform2 );
	v_positionView = ( modelViewMatrix * vec4<f32>( varyings.positionLocal, 1.0 ) ).xyz;
	VERTEX_nodeVar467 = ( render.cameraProjectionMatrix * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar467;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
