// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform3_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform3 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform4 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraWorldMatrix : mat4x4<f32>,
	cameraProjectionMatrixInverse : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> positionView : vec3<f32>;
var<private> normalFlat : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> reflectVector : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar2 : i32;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_clipSpace : vec4<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	positionViewDirection = vec3<f32>( 0.0, 0.0, 1.0 );
	nodeVar1 = ( render.cameraProjectionMatrixInverse * v_clipSpace );
	positionView = ( nodeVar1.xyz / vec3<f32>( nodeVar1.w ) );
	normalFlat = normalize( cross( dpdx( positionView ), - dpdy( positionView ) ) );
	normalViewGeometry = normalFlat;
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	reflectVector = normalize( ( render.cameraWorldMatrix * vec4<f32>( reflect( ( - positionViewDirection ), normalView ), 0.0 ) ).xyz );
	Output = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar2 = i32( fragCoord.xy.x );
	nodeVar3 = vec3<f32>( 0.0, 0.0, 0.0 );

	for ( var i : i32 = 0; i < 512; i ++ ) {

		nodeVar4 = ( 1.0 - ( ( ( f32( i ) * 2.0 ) + 1.0 ) / 512.0 ) );
		nodeVar5 = sqrt( max( ( 1.0 - ( nodeVar4 * nodeVar4 ) ), 0.0 ) );
		nodeVar6 = ( f32( i ) * 2.399963229728653 );
		nodeVar7 = vec3<f32>( ( nodeVar5 * cos( nodeVar6 ) ), nodeVar4, ( nodeVar5 * sin( nodeVar6 ) ) );
		nodeVar8 = ( object.nodeUniform4 * vec4<f32>( nodeVar7, 1.0 ) );
		nodeVar9 = textureSampleLevel( nodeUniform3, nodeUniform3_sampler, vec3<f32>( ( - nodeVar8.x ), nodeVar8.yz ), 0.0 );
		nodeVar3 = ( nodeVar3 + ( nodeVar9.xyz * vec3<f32>( array< f32, 9 >( 0.282095, ( nodeVar7.y * 0.488603 ), ( nodeVar7.z * 0.488603 ), ( nodeVar7.x * 0.488603 ), ( ( nodeVar7.x * nodeVar7.y ) * 1.092548 ), ( ( nodeVar7.y * nodeVar7.z ) * 1.092548 ), ( ( ( ( nodeVar7.z * nodeVar7.z ) * 3.0 ) - 1.0 ) * 0.315392 ), ( ( nodeVar7.x * nodeVar7.z ) * 1.092548 ), ( ( ( nodeVar7.x * nodeVar7.x ) - ( nodeVar7.y * nodeVar7.y ) ) * 0.546274 ) )[ nodeVar2 ] ) ) );

	}


	// result

	output.color = vec4<f32>( ( nodeVar3 * vec3<f32>( 0.02454369260617026 ) ), 1.0 );

	return output;

}
