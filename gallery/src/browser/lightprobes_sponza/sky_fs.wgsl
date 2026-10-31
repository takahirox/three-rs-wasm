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

struct renderStruct {
	nodeUniform7 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraPosition : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform2 : f32,
	nodeUniform3 : u32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform8 : f32,
	nodeUniform9 : f32,
	nodeUniform10 : f32,
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform13 : f32,
	nodeUniform14 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar1 : bool;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : vec2<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> Output : vec4<f32>;

// codes
fn fn133 ( i : vec2<f32> ) -> vec2<f32> {

	var nodeVar0 : vec3<f32>;

	nodeVar0 = fract( ( vec3<f32>( i, 0.0 ).xyx * vec3<f32>( 0.1031, 0.103, 0.0973 ) ) );
	nodeVar0 = ( nodeVar0 + vec3<f32>( dot( nodeVar0, ( nodeVar0.yzx + vec3<f32>( 33.33 ) ) ) ) );

	return ( ( fract( ( ( nodeVar0.xx + nodeVar0.yz ) * nodeVar0.zy ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) );

}


fn fn134 ( p : vec2<f32> ) -> f32 {

	

	let nodeConst0 = floor( p );
	let nodeConst1 = fract( p );
	let nodeConst2 = ( ( ( nodeConst1 * nodeConst1 ) * nodeConst1 ) * ( ( nodeConst1 * ( ( nodeConst1 * vec2<f32>( 6.0 ) ) - vec2<f32>( 15.0 ) ) ) + vec2<f32>( 10.0 ) ) );

	return ( mix( mix( dot( fn133( nodeConst0 ), nodeConst1 ), dot( fn133( ( nodeConst0 + vec2<f32>( 1.0, 0.0 ) ) ), ( nodeConst1 - vec2<f32>( 1.0, 0.0 ) ) ), nodeConst2.x ), mix( dot( fn133( ( nodeConst0 + vec2<f32>( 0.0, 1.0 ) ) ), ( nodeConst1 - vec2<f32>( 0.0, 1.0 ) ) ), dot( fn133( ( nodeConst0 + vec2<f32>( 1.0, 1.0 ) ) ), ( nodeConst1 - vec2<f32>( 1.0, 1.0 ) ) ), nodeConst2.x ), nodeConst2.y ) * 1.6 );

}


fn fn135 ( position : vec2<f32>, drift : vec2<f32> ) -> f32 {

	var nodeVar0 : vec2<f32>;
	var nodeVar1 : f32;
	var nodeVar2 : f32;

	nodeVar0 = position;
	nodeVar1 = 0.0;
	nodeVar2 = 1.0;

	for ( var i : i32 = 0; i < 4; i ++ ) {

		nodeVar1 = ( nodeVar1 + ( nodeVar2 * fn134( nodeVar0 ) ) );
		nodeVar2 = ( nodeVar2 * 0.5 );
		nodeVar0 = ( nodeVar0 * vec2<f32>( 2.0 ) );
		nodeVar0 = ( nodeVar0 + drift );

	}


	return nodeVar1;

}




@fragment
fn main( @location( 0 ) v_positionWorld : vec3<f32>,
	@location( 1 ) nodeVarying5 : f32,
	@location( 2 ) nodeVarying6 : vec3<f32>,
	@location( 3 ) nodeVarying7 : vec3<f32>,
	@location( 4 ) nodeVarying8 : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	let nodeConst0 = normalize( ( v_positionWorld - render.cameraPosition ) );
	let nodeConst1 = dot( nodeConst0, nodeVarying7 );
	let nodeConst2 = ( nodeVarying6 * vec3<f32>( ( 0.05968310365946075 * ( 1.0 + pow( ( ( nodeConst1 * 0.5 ) + 0.5 ), 2.0 ) ) ) ) );
	let nodeConst3 = pow( object.nodeUniform2, 2.0 );
	let nodeConst4 = ( nodeVarying8 * vec3<f32>( ( ( 0.07957747154594767 * ( 1.0 - nodeConst3 ) ) * ( 1.0 / pow( ( ( 1.0 - ( ( 2.0 * object.nodeUniform2 ) * nodeConst1 ) ) + nodeConst3 ), 1.5 ) ) ) ) );
	let nodeConst5 = acos( max( 0.0, nodeConst0.y ) );
	let nodeConst6 = ( 1.0 / ( cos( nodeConst5 ) + ( 0.15 * pow( ( 93.885 - ( ( nodeConst5 * 180.0 ) / 3.141592653589793 ) ), -1.253 ) ) ) );
	let nodeConst7 = exp( ( - ( ( nodeVarying6 * vec3<f32>( ( 8400.0 * nodeConst6 ) ) ) + ( nodeVarying8 * vec3<f32>( ( 1250.0 * nodeConst6 ) ) ) ) ) );
	var nodeVar0 : vec3<f32> = pow( ( ( vec3<f32>( nodeVarying5 ) * ( ( nodeConst2 + nodeConst4 ) / ( nodeVarying6 + nodeVarying8 ) ) ) * ( vec3<f32>( 1.0 ) - nodeConst7 ) ), vec3<f32>( 1.5, 1.5, 1.5 ) );
	nodeVar0 = ( nodeVar0 * mix( vec3<f32>( 1.0, 1.0, 1.0 ), pow( ( ( vec3<f32>( nodeVarying5 ) * ( ( nodeConst2 + nodeConst4 ) / ( nodeVarying6 + nodeVarying8 ) ) ) * nodeConst7 ), vec3<f32>( 0.5, 0.5, 0.5 ) ), clamp( pow( ( 1.0 - nodeVarying7.y ), 5.0 ), 0.0, 1.0 ) ) );
	let nodeConst8 = ( vec3<f32>( 0.1, 0.1, 0.1 ) * nodeConst7 );
	nodeVar1 = bool( object.nodeUniform3 );
	let nodeConst9 = ( ( min( ( vec3<f32>( nodeVarying5 ) * nodeConst7 ), vec3<f32>( 80.0 ) ) * vec3<f32>( 760.0 ) ) * vec3<f32>( ( clamp( ( ( nodeConst1 - 0.9999566769464484 ) * 50000.0 ), 0.0, 1.0 ) * f32( nodeVar1 ) ) ) );
	nodeVar2 = ( ( ( ( nodeVar0 + nodeConst8 ) * vec3<f32>( 0.04 ) ) + nodeConst9 ) + vec3<f32>( 0.0, 0.0003, 0.00075 ) );

	if ( ( ( nodeConst0.y > 0.0 ) && ( object.nodeUniform4 > 0.0 ) ) ) {

		nodeVar3 = ( nodeConst0.xz / vec2<f32>( ( nodeConst0.y * mix( 1.0, 0.1, object.nodeUniform5 ) ) ) );
		nodeVar3 = ( nodeVar3 * vec2<f32>( object.nodeUniform6 ) );
		nodeVar3 = ( nodeVar3 + vec2<f32>( ( render.nodeUniform7 * object.nodeUniform8 ) ) );
		nodeVar4 = clamp( ( ( fn135( ( nodeVar3 * vec2<f32>( 1000.0 ) ), vec2<f32>( ( ( render.nodeUniform7 * object.nodeUniform8 ) * 300.0 ) ) ) * 0.7 ) + 0.5 ), 0.0, 1.0 );
		nodeVar5 = ( 1.0 - clamp( ( object.nodeUniform4 + ( ( ( ( fn134( ( nodeVar3 * vec2<f32>( 300.0 ) ) ) * 0.37 ) + 0.5 ) - 0.5 ) * 0.6 ) ), 0.0, 1.0 ) );
		nodeVar6 = smoothstep( nodeVar5, ( nodeVar5 + 0.3 ), nodeVar4 );
		let nodeConst10 = smoothstep( 0.0, ( 0.03 + ( 0.06 * object.nodeUniform5 ) ), nodeConst0.y );
		nodeVar6 = ( nodeVar6 * nodeConst10 );
		nodeVar7 = ( ( ( vec3<f32>( nodeVarying5 ) * nodeConst7 ) * vec3<f32>( 0.22 ) ) * vec3<f32>( 0.04 ) );
		nodeVar8 = max( 0.0, ( nodeVar4 - nodeVar5 ) );
		nodeVar9 = exp( ( nodeVar8 * -4.0 ) );
		nodeVar10 = ( ( ( nodeVar0 * vec3<f32>( 0.04 ) ) + vec3<f32>( 0.0, 0.0003, 0.00075 ) ) + ( nodeVar7 * vec3<f32>( mix( 0.45, 1.0, clamp( ( ( nodeVar9 * ( 1.0 - ( nodeVar9 * nodeVar9 ) ) ) * 2.6 ), 0.0, 1.0 ) ) ) ) );
		nodeVar10 = ( nodeVar10 + ( ( ( nodeVar7 * vec3<f32>( clamp( ( 0.51 / pow( ( 1.49 - ( nodeConst1 * 1.4 ) ), 1.5 ) ), 0.0, 3.0 ) ) ) * vec3<f32>( ( ( nodeVar6 * ( 1.0 - nodeVar6 ) ) * 4.0 ) ) ) * vec3<f32>( 0.6 ) ) );
		nodeVar10 = ( nodeVar10 * vec3<f32>( max( smoothstep( -0.08, 0.3, nodeVarying7.y ), 0.03 ) ) );
		nodeVar11 = ( ( 1.0 - exp( ( ( nodeVar8 * object.nodeUniform9 ) * -12.0 ) ) ) * nodeConst10 );
		nodeVar2 = ( nodeVar2 - ( ( ( nodeConst8 * vec3<f32>( 0.04 ) ) + nodeConst9 ) * vec3<f32>( nodeVar11 ) ) );
		nodeVar2 = mix( nodeVar2, mix( nodeVar2, nodeVar10, nodeConst7 ), nodeVar11 );
		

	}

	DiffuseColor = vec4<f32>( nodeVar2, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform10 );
	DiffuseColor.w = 1.0;
	let nodeConst11 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst11;

	// result

	output.color = nodeConst11;

	return output;

}
