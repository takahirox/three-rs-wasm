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
@binding( 1 ) @group( 1 ) var nodeUniform6_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform6 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : i32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform7 : f32
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
var<private> nodeVar2 : f32;
var<private> positionViewDirection : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> reflectVector : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_positionWorldDirection : vec3<f32>,
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	positionWorldDirection = normalize( v_positionWorldDirection );
	nodeVar0 = positionWorldDirection;
	let nodeConst0 = ( object.nodeUniform1 * object.nodeUniform1 );
	let nodeConst1 = ( nodeConst0 * nodeConst0 );
	let nodeConst2 = ( 2.0 / f32( object.nodeUniform2 ) );
	nodeVar1 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar2 = 0.0;
	positionViewDirection = normalize( v_positionViewDirection );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
	normalView = NORMAL_normalView;
	reflectVector = normalize( ( render.cameraWorldMatrix * vec4<f32>( reflect( ( - positionViewDirection ), normalView ), 0.0 ) ).xyz );

	for ( var face : i32 = 0; face < 3; face ++ ) {


		for ( var y : i32 = 0; y < object.nodeUniform2; y ++ ) {


			for ( var x : i32 = 0; x < object.nodeUniform2; x ++ ) {

				let nodeConst3 = ( ( ( vec2<f32>( f32( x ), f32( y ) ) + vec2<f32>( 0.5 ) ) * vec2<f32>( nodeConst2 ) ) - vec2<f32>( 1.0 ) );

				if ( ( face == 0 ) ) {

					nodeVar3 = vec3<f32>( 1.0, nodeConst3 );

				} else {


					if ( ( face == 1 ) ) {

						nodeVar4 = vec3<f32>( nodeConst3.x, 1.0, nodeConst3.y );

					} else {

						nodeVar4 = vec3<f32>( nodeConst3, 1.0 );

					}

					nodeVar3 = nodeVar4;

				}

				nodeVar5 = nodeVar3;
				let nodeConst4 = inverseSqrt( ( dot( nodeConst3, nodeConst3 ) + 1.0 ) );
				nodeVar6 = dot( nodeVar0, nodeVar5 );

				if ( ( nodeVar6 < 0.0 ) ) {

					nodeVar7 = -1.0;

				} else {

					nodeVar7 = 1.0;

				}

				nodeVar5 = ( nodeVar5 * vec3<f32>( nodeVar7 ) );
				nodeVar6 = ( abs( nodeVar6 ) * nodeConst4 );
				let nodeConst5 = ( ( nodeConst1 + 1.0 ) + ( ( nodeConst1 - 1.0 ) * nodeVar6 ) );
				let nodeConst6 = ( ( ( ( nodeVar6 * nodeConst4 ) * nodeConst4 ) * nodeConst4 ) / ( nodeConst5 * nodeConst5 ) );
				nodeVar8 = textureSampleLevel( nodeUniform6, nodeUniform6_sampler, vec3<f32>( ( - nodeVar5.x ), nodeVar5.yz ), object.nodeUniform7 );
				nodeVar1 = ( nodeVar1 + ( nodeVar8.xyz * vec3<f32>( nodeConst6 ) ) );
				nodeVar2 = ( nodeVar2 + nodeConst6 );

			}


		}


	}


	// result

	output.color = vec4<f32>( ( nodeVar1 / vec3<f32>( nodeVar2 ) ), 1.0 );

	return output;

}
