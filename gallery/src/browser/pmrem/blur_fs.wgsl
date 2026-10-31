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
@binding( 1 ) @group( 1 ) var nodeUniform5_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform5 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform1 : f32,
	nodeUniform4 : mat3x3<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> positionWorldDirection : vec3<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : f32;
var<private> positionViewDirection : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> reflectVector : vec3<f32>;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_positionWorldDirection : vec3<f32>,
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	positionWorldDirection = normalize( v_positionWorldDirection );
	nodeVar0 = positionWorldDirection;

	if ( ( abs( nodeVar0.z ) < 0.999 ) ) {

		nodeVar1 = vec3<f32>( 0.0, 0.0, 1.0 );

	} else {

		nodeVar1 = vec3<f32>( 1.0, 0.0, 0.0 );

	}

	nodeVar2 = normalize( cross( nodeVar1, nodeVar0 ) );
	nodeVar3 = cross( nodeVar0, nodeVar2 );
	let nodeConst0 = min( ( object.nodeUniform1 * 3.0 ), 3.141592653589793 );
	nodeVar4 = ( 1.0 - exp( ( ( ( nodeConst0 * nodeConst0 ) * -0.5 ) / ( object.nodeUniform1 * object.nodeUniform1 ) ) ) );
	nodeVar5 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar6 = 0.0;
	positionViewDirection = normalize( v_positionViewDirection );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
	normalView = NORMAL_normalView;
	reflectVector = normalize( ( render.cameraWorldMatrix * vec4<f32>( reflect( ( - positionViewDirection ), normalView ), 0.0 ) ).xyz );

	for ( var i : i32 = 0; i < 20; i ++ ) {

		nodeVar7 = ( object.nodeUniform1 * sqrt( ( log( ( 1.0 - ( ( ( f32( i ) + 0.5 ) / 20.0 ) * nodeVar4 ) ) ) * -2.0 ) ) );
		nodeVar8 = ( f32( i ) * 2.399963229728653 );
		nodeVar9 = ( sin( nodeVar7 ) / nodeVar7 );
		let nodeConst1 = ( ( nodeVar0 * vec3<f32>( cos( nodeVar7 ) ) ) + ( ( ( nodeVar2 * vec3<f32>( cos( nodeVar8 ) ) ) + ( nodeVar3 * vec3<f32>( sin( nodeVar8 ) ) ) ) * vec3<f32>( sin( nodeVar7 ) ) ) );
		nodeVar10 = textureSampleLevel( nodeUniform5, nodeUniform5_sampler, vec3<f32>( ( - nodeConst1.x ), nodeConst1.yz ), 0.0 );
		nodeVar5 = ( nodeVar5 + ( nodeVar10.xyz * vec3<f32>( nodeVar9 ) ) );
		nodeVar6 = ( nodeVar6 + nodeVar9 );

	}


	// result

	output.color = vec4<f32>( ( nodeVar5 / vec3<f32>( nodeVar6 ) ), 1.0 );

	return output;

}
