// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform11_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform11 : texture_2d<f32>;

struct renderStruct {
	nodeUniform7 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraPosition : vec3<f32>,
	nodeUniform12 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform8 : f32,
	nodeUniform9 : f32,
	nodeUniform10 : f32,
	nodeUniform13 : vec3<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform16 : f32
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
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec2<f32>;
var<private> nodeVar13 : vec2<f32>;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec2<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec2<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec2<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec2<f32>;
var<private> nodeVar25 : vec2<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec2<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec2<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : f32;
var<private> AmbientOcclusion : f32;
var<private> nodeVar40 : f32;
var<private> Output : vec4<f32>;
var<private> nodeVar41 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_positionWorld : vec3<f32>,
	@location( 1 ) nodeVarying5 : f32,
	@location( 2 ) nodeVarying6 : vec3<f32>,
	@location( 3 ) nodeVarying7 : vec3<f32>,
	@location( 4 ) nodeVarying8 : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

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
	nodeVar10 = ( ( min( ( vec3<f32>( nodeVarying5 ) * nodeVar7 ), vec3<f32>( 80.0 ) ) * vec3<f32>( 760.0 ) ) * vec3<f32>( ( clamp( ( ( nodeVar1 - 0.9999566769464484 ) * 50000.0 ), 0.0, 1.0 ) * object.nodeUniform3 ) ) );
	nodeVar11 = ( ( ( ( nodeVar8 + nodeVar9 ) * vec3<f32>( 0.04 ) ) + nodeVar10 ) + vec3<f32>( 0.0, 0.0003, 0.00075 ) );

	if ( ( ( nodeVar0.y > 0.0 ) && ( object.nodeUniform4 > 0.0 ) ) ) {

		nodeVar12 = ( nodeVar0.xz / vec2<f32>( ( nodeVar0.y * mix( 1.0, 0.1, object.nodeUniform5 ) ) ) );
		nodeVar12 = ( nodeVar12 * vec2<f32>( object.nodeUniform6 ) );
		nodeVar12 = ( nodeVar12 + vec2<f32>( ( render.nodeUniform7 * object.nodeUniform8 ) ) );
		nodeVar13 = ( nodeVar12 * vec2<f32>( 1000.0 ) );
		nodeVar14 = 0.0;
		nodeVar15 = 1.0;

		for ( var i : i32 = 0; i < 4; i ++ ) {

			nodeVar16 = floor( nodeVar13 );
			nodeVar17 = fract( ( vec3<f32>( nodeVar16, 0.0 ).xyx * vec3<f32>( 0.1031, 0.103, 0.0973 ) ) );
			nodeVar17 = ( nodeVar17 + vec3<f32>( dot( nodeVar17, ( nodeVar17.yzx + vec3<f32>( 33.33 ) ) ) ) );
			nodeVar18 = fract( nodeVar13 );
			nodeVar19 = fract( ( vec3<f32>( ( nodeVar16 + vec2<f32>( 1.0, 0.0 ) ), 0.0 ).xyx * vec3<f32>( 0.1031, 0.103, 0.0973 ) ) );
			nodeVar19 = ( nodeVar19 + vec3<f32>( dot( nodeVar19, ( nodeVar19.yzx + vec3<f32>( 33.33 ) ) ) ) );
			nodeVar20 = ( ( ( nodeVar18 * nodeVar18 ) * nodeVar18 ) * ( ( nodeVar18 * ( ( nodeVar18 * vec2<f32>( 6.0 ) ) - vec2<f32>( 15.0 ) ) ) + vec2<f32>( 10.0 ) ) );
			nodeVar21 = fract( ( vec3<f32>( ( nodeVar16 + vec2<f32>( 0.0, 1.0 ) ), 0.0 ).xyx * vec3<f32>( 0.1031, 0.103, 0.0973 ) ) );
			nodeVar21 = ( nodeVar21 + vec3<f32>( dot( nodeVar21, ( nodeVar21.yzx + vec3<f32>( 33.33 ) ) ) ) );
			nodeVar22 = fract( ( vec3<f32>( ( nodeVar16 + vec2<f32>( 1.0, 1.0 ) ), 0.0 ).xyx * vec3<f32>( 0.1031, 0.103, 0.0973 ) ) );
			nodeVar22 = ( nodeVar22 + vec3<f32>( dot( nodeVar22, ( nodeVar22.yzx + vec3<f32>( 33.33 ) ) ) ) );
			nodeVar14 = ( nodeVar14 + ( nodeVar15 * ( mix( mix( dot( ( ( fract( ( ( nodeVar17.xx + nodeVar17.yz ) * nodeVar17.zy ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar18 ), dot( ( ( fract( ( ( nodeVar19.xx + nodeVar19.yz ) * nodeVar19.zy ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), ( nodeVar18 - vec2<f32>( 1.0, 0.0 ) ) ), nodeVar20.x ), mix( dot( ( ( fract( ( ( nodeVar21.xx + nodeVar21.yz ) * nodeVar21.zy ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), ( nodeVar18 - vec2<f32>( 0.0, 1.0 ) ) ), dot( ( ( fract( ( ( nodeVar22.xx + nodeVar22.yz ) * nodeVar22.zy ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), ( nodeVar18 - vec2<f32>( 1.0, 1.0 ) ) ), nodeVar20.x ), nodeVar20.y ) * 1.6 ) ) );
			nodeVar15 = ( nodeVar15 * 0.5 );
			nodeVar13 = ( nodeVar13 * vec2<f32>( 2.0 ) );
			nodeVar13 = ( nodeVar13 + vec2<f32>( ( ( render.nodeUniform7 * object.nodeUniform8 ) * 300.0 ) ) );

		}

		nodeVar23 = clamp( ( ( nodeVar14 * 0.7 ) + 0.5 ), 0.0, 1.0 );
		nodeVar24 = ( nodeVar12 * vec2<f32>( 300.0 ) );
		nodeVar25 = floor( nodeVar24 );
		nodeVar26 = fract( ( vec3<f32>( nodeVar25, 0.0 ).xyx * vec3<f32>( 0.1031, 0.103, 0.0973 ) ) );
		nodeVar26 = ( nodeVar26 + vec3<f32>( dot( nodeVar26, ( nodeVar26.yzx + vec3<f32>( 33.33 ) ) ) ) );
		nodeVar27 = fract( nodeVar24 );
		nodeVar28 = fract( ( vec3<f32>( ( nodeVar25 + vec2<f32>( 1.0, 0.0 ) ), 0.0 ).xyx * vec3<f32>( 0.1031, 0.103, 0.0973 ) ) );
		nodeVar28 = ( nodeVar28 + vec3<f32>( dot( nodeVar28, ( nodeVar28.yzx + vec3<f32>( 33.33 ) ) ) ) );
		nodeVar29 = ( ( ( nodeVar27 * nodeVar27 ) * nodeVar27 ) * ( ( nodeVar27 * ( ( nodeVar27 * vec2<f32>( 6.0 ) ) - vec2<f32>( 15.0 ) ) ) + vec2<f32>( 10.0 ) ) );
		nodeVar30 = fract( ( vec3<f32>( ( nodeVar25 + vec2<f32>( 0.0, 1.0 ) ), 0.0 ).xyx * vec3<f32>( 0.1031, 0.103, 0.0973 ) ) );
		nodeVar30 = ( nodeVar30 + vec3<f32>( dot( nodeVar30, ( nodeVar30.yzx + vec3<f32>( 33.33 ) ) ) ) );
		nodeVar31 = fract( ( vec3<f32>( ( nodeVar25 + vec2<f32>( 1.0, 1.0 ) ), 0.0 ).xyx * vec3<f32>( 0.1031, 0.103, 0.0973 ) ) );
		nodeVar31 = ( nodeVar31 + vec3<f32>( dot( nodeVar31, ( nodeVar31.yzx + vec3<f32>( 33.33 ) ) ) ) );
		nodeVar32 = ( 1.0 - clamp( ( object.nodeUniform4 + ( ( ( ( ( mix( mix( dot( ( ( fract( ( ( nodeVar26.xx + nodeVar26.yz ) * nodeVar26.zy ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar27 ), dot( ( ( fract( ( ( nodeVar28.xx + nodeVar28.yz ) * nodeVar28.zy ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), ( nodeVar27 - vec2<f32>( 1.0, 0.0 ) ) ), nodeVar29.x ), mix( dot( ( ( fract( ( ( nodeVar30.xx + nodeVar30.yz ) * nodeVar30.zy ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), ( nodeVar27 - vec2<f32>( 0.0, 1.0 ) ) ), dot( ( ( fract( ( ( nodeVar31.xx + nodeVar31.yz ) * nodeVar31.zy ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), ( nodeVar27 - vec2<f32>( 1.0, 1.0 ) ) ), nodeVar29.x ), nodeVar29.y ) * 1.6 ) * 0.37 ) + 0.5 ) - 0.5 ) * 0.6 ) ), 0.0, 1.0 ) );
		nodeVar33 = smoothstep( nodeVar32, ( nodeVar32 + 0.3 ), nodeVar23 );
		nodeVar34 = smoothstep( 0.0, ( 0.03 + ( 0.06 * object.nodeUniform5 ) ), nodeVar0.y );
		nodeVar33 = ( nodeVar33 * nodeVar34 );
		nodeVar35 = ( ( ( vec3<f32>( nodeVarying5 ) * nodeVar7 ) * vec3<f32>( 0.22 ) ) * vec3<f32>( 0.04 ) );
		nodeVar36 = max( 0.0, ( nodeVar23 - nodeVar32 ) );
		nodeVar37 = exp( ( nodeVar36 * -4.0 ) );
		nodeVar38 = ( ( ( nodeVar8 * vec3<f32>( 0.04 ) ) + vec3<f32>( 0.0, 0.0003, 0.00075 ) ) + ( nodeVar35 * vec3<f32>( mix( 0.45, 1.0, clamp( ( ( nodeVar37 * ( 1.0 - ( nodeVar37 * nodeVar37 ) ) ) * 2.6 ), 0.0, 1.0 ) ) ) ) );
		nodeVar38 = ( nodeVar38 + ( ( ( nodeVar35 * vec3<f32>( clamp( ( 0.51 / pow( ( 1.49 - ( nodeVar1 * 1.4 ) ), 1.5 ) ), 0.0, 3.0 ) ) ) * vec3<f32>( ( ( nodeVar33 * ( 1.0 - nodeVar33 ) ) * 4.0 ) ) ) * vec3<f32>( 0.6 ) ) );
		nodeVar38 = ( nodeVar38 * vec3<f32>( max( smoothstep( -0.08, 0.3, nodeVarying7.y ), 0.03 ) ) );
		nodeVar39 = ( ( 1.0 - exp( ( ( nodeVar36 * object.nodeUniform9 ) * -12.0 ) ) ) * nodeVar34 );
		nodeVar11 = ( nodeVar11 - ( ( ( nodeVar9 * vec3<f32>( 0.04 ) ) + nodeVar10 ) * vec3<f32>( nodeVar39 ) ) );
		nodeVar11 = mix( nodeVar11, mix( nodeVar11, nodeVar38, nodeVar7 ), nodeVar39 );
		

	}

	DiffuseColor = vec4<f32>( nodeVar11, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform10 );
	DiffuseColor.w = 1.0;
	nodeVar40 = textureSample( nodeUniform11, nodeUniform11_sampler, ( fragCoord.xy / render.nodeUniform12 ) ).x;
	AmbientOcclusion = max( nodeVar40, 0.001 );
	nodeVar41 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar41;

	// result

	output.color = nodeVar41;

	return output;

}
