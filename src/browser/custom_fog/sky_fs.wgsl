// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


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
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : bool;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec2<f32>;
var<private> nodeVar14 : vec2<f32>;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec2<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec2<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec2<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : vec2<f32>;
var<private> nodeVar26 : vec2<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec2<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec2<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : f32;
var<private> Output : vec4<f32>;
var<private> nodeVar41 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_positionWorld : vec3<f32>,
	@location( 1 ) nodeVarying5 : f32,
	@location( 2 ) nodeVarying6 : vec3<f32>,
	@location( 3 ) nodeVarying7 : vec3<f32>,
	@location( 4 ) nodeVarying8 : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = normalize( ( v_positionWorld - render.cameraPosition ) );
	nodeVar1 = dot( nodeVar0, nodeVarying7 );
	nodeVar2 = ( nodeVarying6 * vec3<f32>( ( 0.05968310365946075 * ( 1.0 + pow( ( ( nodeVar1 * 0.5 ) + 0.5 ), 2.0 ) ) ) ) );
	nodeVar3 = pow( object.nodeUniform2, 2.0 );
	nodeVar4 = ( nodeVarying8 * vec3<f32>( ( ( 0.07957747154594767 * ( 1.0 - nodeVar3 ) ) * ( 1.0 / pow( ( ( 1.0 - ( ( 2.0 * object.nodeUniform2 ) * nodeVar1 ) ) + nodeVar3 ), 1.5 ) ) ) ) );
	nodeVar5 = acos( max( 0.0, nodeVar0.y ) );
	nodeVar6 = ( 1.0 / ( cos( nodeVar5 ) + ( 0.15 * pow( ( 93.885 - ( ( nodeVar5 * 180.0 ) / 3.141592653589793 ) ), -1.253 ) ) ) );
	nodeVar7 = exp( ( - ( ( nodeVarying6 * vec3<f32>( ( 8400.0 * nodeVar6 ) ) ) + ( nodeVarying8 * vec3<f32>( ( 1250.0 * nodeVar6 ) ) ) ) ) );
	nodeVar8 = pow( ( ( vec3<f32>( nodeVarying5 ) * ( ( nodeVar2 + nodeVar4 ) / ( nodeVarying6 + nodeVarying8 ) ) ) * ( vec3<f32>( 1.0 ) - nodeVar7 ) ), vec3<f32>( 1.5, 1.5, 1.5 ) );
	nodeVar8 = ( nodeVar8 * mix( vec3<f32>( 1.0, 1.0, 1.0 ), pow( ( ( vec3<f32>( nodeVarying5 ) * ( ( nodeVar2 + nodeVar4 ) / ( nodeVarying6 + nodeVarying8 ) ) ) * nodeVar7 ), vec3<f32>( 0.5, 0.5, 0.5 ) ), clamp( pow( ( 1.0 - nodeVarying7.y ), 5.0 ), 0.0, 1.0 ) ) );
	nodeVar9 = ( vec3<f32>( 0.1, 0.1, 0.1 ) * nodeVar7 );
	nodeVar10 = bool( object.nodeUniform3 );
	nodeVar11 = ( ( min( ( vec3<f32>( nodeVarying5 ) * nodeVar7 ), vec3<f32>( 80.0 ) ) * vec3<f32>( 760.0 ) ) * vec3<f32>( ( clamp( ( ( nodeVar1 - 0.9999566769464484 ) * 50000.0 ), 0.0, 1.0 ) * f32( nodeVar10 ) ) ) );
	nodeVar12 = ( ( ( ( nodeVar8 + nodeVar9 ) * vec3<f32>( 0.04 ) ) + nodeVar11 ) + vec3<f32>( 0.0, 0.0003, 0.00075 ) );

	if ( ( ( nodeVar0.y > 0.0 ) && ( object.nodeUniform4 > 0.0 ) ) ) {

		nodeVar13 = ( nodeVar0.xz / vec2<f32>( ( nodeVar0.y * mix( 1.0, 0.1, object.nodeUniform5 ) ) ) );
		nodeVar13 = ( nodeVar13 * vec2<f32>( object.nodeUniform6 ) );
		nodeVar13 = ( nodeVar13 + vec2<f32>( ( render.nodeUniform7 * object.nodeUniform8 ) ) );
		nodeVar14 = ( nodeVar13 * vec2<f32>( 1000.0 ) );
		nodeVar15 = 0.0;
		nodeVar16 = 1.0;

		for ( var i : i32 = 0; i < 4; i ++ ) {

			nodeVar17 = floor( nodeVar14 );
			nodeVar18 = fract( ( vec3<f32>( nodeVar17, 0.0 ).xyx * vec3<f32>( 0.1031, 0.103, 0.0973 ) ) );
			nodeVar18 = ( nodeVar18 + vec3<f32>( dot( nodeVar18, ( nodeVar18.yzx + vec3<f32>( 33.33 ) ) ) ) );
			nodeVar19 = fract( nodeVar14 );
			nodeVar20 = fract( ( vec3<f32>( ( nodeVar17 + vec2<f32>( 1.0, 0.0 ) ), 0.0 ).xyx * vec3<f32>( 0.1031, 0.103, 0.0973 ) ) );
			nodeVar20 = ( nodeVar20 + vec3<f32>( dot( nodeVar20, ( nodeVar20.yzx + vec3<f32>( 33.33 ) ) ) ) );
			nodeVar21 = ( ( ( nodeVar19 * nodeVar19 ) * nodeVar19 ) * ( ( nodeVar19 * ( ( nodeVar19 * vec2<f32>( 6.0 ) ) - vec2<f32>( 15.0 ) ) ) + vec2<f32>( 10.0 ) ) );
			nodeVar22 = fract( ( vec3<f32>( ( nodeVar17 + vec2<f32>( 0.0, 1.0 ) ), 0.0 ).xyx * vec3<f32>( 0.1031, 0.103, 0.0973 ) ) );
			nodeVar22 = ( nodeVar22 + vec3<f32>( dot( nodeVar22, ( nodeVar22.yzx + vec3<f32>( 33.33 ) ) ) ) );
			nodeVar23 = fract( ( vec3<f32>( ( nodeVar17 + vec2<f32>( 1.0, 1.0 ) ), 0.0 ).xyx * vec3<f32>( 0.1031, 0.103, 0.0973 ) ) );
			nodeVar23 = ( nodeVar23 + vec3<f32>( dot( nodeVar23, ( nodeVar23.yzx + vec3<f32>( 33.33 ) ) ) ) );
			nodeVar15 = ( nodeVar15 + ( nodeVar16 * ( mix( mix( dot( ( ( fract( ( ( nodeVar18.xx + nodeVar18.yz ) * nodeVar18.zy ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar19 ), dot( ( ( fract( ( ( nodeVar20.xx + nodeVar20.yz ) * nodeVar20.zy ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), ( nodeVar19 - vec2<f32>( 1.0, 0.0 ) ) ), nodeVar21.x ), mix( dot( ( ( fract( ( ( nodeVar22.xx + nodeVar22.yz ) * nodeVar22.zy ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), ( nodeVar19 - vec2<f32>( 0.0, 1.0 ) ) ), dot( ( ( fract( ( ( nodeVar23.xx + nodeVar23.yz ) * nodeVar23.zy ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), ( nodeVar19 - vec2<f32>( 1.0, 1.0 ) ) ), nodeVar21.x ), nodeVar21.y ) * 1.6 ) ) );
			nodeVar16 = ( nodeVar16 * 0.5 );
			nodeVar14 = ( nodeVar14 * vec2<f32>( 2.0 ) );
			nodeVar14 = ( nodeVar14 + vec2<f32>( ( ( render.nodeUniform7 * object.nodeUniform8 ) * 300.0 ) ) );

		}

		nodeVar24 = clamp( ( ( nodeVar15 * 0.7 ) + 0.5 ), 0.0, 1.0 );
		nodeVar25 = ( nodeVar13 * vec2<f32>( 300.0 ) );
		nodeVar26 = floor( nodeVar25 );
		nodeVar27 = fract( ( vec3<f32>( nodeVar26, 0.0 ).xyx * vec3<f32>( 0.1031, 0.103, 0.0973 ) ) );
		nodeVar27 = ( nodeVar27 + vec3<f32>( dot( nodeVar27, ( nodeVar27.yzx + vec3<f32>( 33.33 ) ) ) ) );
		nodeVar28 = fract( nodeVar25 );
		nodeVar29 = fract( ( vec3<f32>( ( nodeVar26 + vec2<f32>( 1.0, 0.0 ) ), 0.0 ).xyx * vec3<f32>( 0.1031, 0.103, 0.0973 ) ) );
		nodeVar29 = ( nodeVar29 + vec3<f32>( dot( nodeVar29, ( nodeVar29.yzx + vec3<f32>( 33.33 ) ) ) ) );
		nodeVar30 = ( ( ( nodeVar28 * nodeVar28 ) * nodeVar28 ) * ( ( nodeVar28 * ( ( nodeVar28 * vec2<f32>( 6.0 ) ) - vec2<f32>( 15.0 ) ) ) + vec2<f32>( 10.0 ) ) );
		nodeVar31 = fract( ( vec3<f32>( ( nodeVar26 + vec2<f32>( 0.0, 1.0 ) ), 0.0 ).xyx * vec3<f32>( 0.1031, 0.103, 0.0973 ) ) );
		nodeVar31 = ( nodeVar31 + vec3<f32>( dot( nodeVar31, ( nodeVar31.yzx + vec3<f32>( 33.33 ) ) ) ) );
		nodeVar32 = fract( ( vec3<f32>( ( nodeVar26 + vec2<f32>( 1.0, 1.0 ) ), 0.0 ).xyx * vec3<f32>( 0.1031, 0.103, 0.0973 ) ) );
		nodeVar32 = ( nodeVar32 + vec3<f32>( dot( nodeVar32, ( nodeVar32.yzx + vec3<f32>( 33.33 ) ) ) ) );
		nodeVar33 = ( 1.0 - clamp( ( object.nodeUniform4 + ( ( ( ( ( mix( mix( dot( ( ( fract( ( ( nodeVar27.xx + nodeVar27.yz ) * nodeVar27.zy ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar28 ), dot( ( ( fract( ( ( nodeVar29.xx + nodeVar29.yz ) * nodeVar29.zy ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), ( nodeVar28 - vec2<f32>( 1.0, 0.0 ) ) ), nodeVar30.x ), mix( dot( ( ( fract( ( ( nodeVar31.xx + nodeVar31.yz ) * nodeVar31.zy ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), ( nodeVar28 - vec2<f32>( 0.0, 1.0 ) ) ), dot( ( ( fract( ( ( nodeVar32.xx + nodeVar32.yz ) * nodeVar32.zy ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), ( nodeVar28 - vec2<f32>( 1.0, 1.0 ) ) ), nodeVar30.x ), nodeVar30.y ) * 1.6 ) * 0.37 ) + 0.5 ) - 0.5 ) * 0.6 ) ), 0.0, 1.0 ) );
		nodeVar34 = smoothstep( nodeVar33, ( nodeVar33 + 0.3 ), nodeVar24 );
		nodeVar35 = smoothstep( 0.0, ( 0.03 + ( 0.06 * object.nodeUniform5 ) ), nodeVar0.y );
		nodeVar34 = ( nodeVar34 * nodeVar35 );
		nodeVar36 = ( ( ( vec3<f32>( nodeVarying5 ) * nodeVar7 ) * vec3<f32>( 0.22 ) ) * vec3<f32>( 0.04 ) );
		nodeVar37 = max( 0.0, ( nodeVar24 - nodeVar33 ) );
		nodeVar38 = exp( ( nodeVar37 * -4.0 ) );
		nodeVar39 = ( ( ( nodeVar8 * vec3<f32>( 0.04 ) ) + vec3<f32>( 0.0, 0.0003, 0.00075 ) ) + ( nodeVar36 * vec3<f32>( mix( 0.45, 1.0, clamp( ( ( nodeVar38 * ( 1.0 - ( nodeVar38 * nodeVar38 ) ) ) * 2.6 ), 0.0, 1.0 ) ) ) ) );
		nodeVar39 = ( nodeVar39 + ( ( ( nodeVar36 * vec3<f32>( clamp( ( 0.51 / pow( ( 1.49 - ( nodeVar1 * 1.4 ) ), 1.5 ) ), 0.0, 3.0 ) ) ) * vec3<f32>( ( ( nodeVar34 * ( 1.0 - nodeVar34 ) ) * 4.0 ) ) ) * vec3<f32>( 0.6 ) ) );
		nodeVar39 = ( nodeVar39 * vec3<f32>( max( smoothstep( -0.08, 0.3, nodeVarying7.y ), 0.03 ) ) );
		nodeVar40 = ( ( 1.0 - exp( ( ( nodeVar37 * object.nodeUniform9 ) * -12.0 ) ) ) * nodeVar35 );
		nodeVar12 = ( nodeVar12 - ( ( ( nodeVar9 * vec3<f32>( 0.04 ) ) + nodeVar11 ) * vec3<f32>( nodeVar40 ) ) );
		nodeVar12 = mix( nodeVar12, mix( nodeVar12, nodeVar39, nodeVar7 ), nodeVar40 );
		

	}

	DiffuseColor = vec4<f32>( nodeVar12, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform10 );
	DiffuseColor.w = 1.0;
	nodeVar41 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar41;

	// result

	output.color = nodeVar41;

	return output;

}
