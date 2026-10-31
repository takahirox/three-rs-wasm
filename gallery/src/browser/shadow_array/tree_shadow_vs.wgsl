// Three.js r187 - Node System

// directives


// structs


// uniforms
@binding( 1 ) @group( 2 ) var nodeUniform0 : texture_2d<u32>;
@binding( 2 ) @group( 2 ) var nodeUniform1 : texture_2d<f32>;
@binding( 3 ) @group( 2 ) var nodeUniform2 : texture_2d<f32>;

struct cameraProjectionMatricesStruct {
	value : array< mat4x4<f32>, 4 >
};
@binding( 0 ) @group( 0 )
var<uniform> cameraProjectionMatrices : cameraProjectionMatricesStruct;

struct cameraViewMatricesStruct {
	value : array< mat4x4<f32>, 4 >
};
@binding( 1 ) @group( 0 )
var<uniform> cameraViewMatrices : cameraViewMatricesStruct;

struct cameraIndexStruct {
	u_cameraIndex : u32
};
@binding( 0 ) @group( 1 )
var<uniform> cameraIndex : cameraIndexStruct;

struct objectStruct {
	nodeUniform3 : f32,
	nodeUniform7 : mat4x4<f32>
};
@binding( 0 ) @group( 2 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) @interpolate(flat, either) vBatchIndirectId : u32,
	@location( 1 ) vBatchColor : vec4<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> nodeVar0 : u32;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> normalLocal : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> positionLocal : vec3<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_cameraIndex : u32;
var<private> v_positionView : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes


@vertex
fn main( @builtin( instance_index ) instanceIndex : u32,
	@location( 0 ) position : vec3<f32>,
	@location( 1 ) normal : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	let nodeConst0 = i32( textureDimensions( nodeUniform0, 0 ).x );
	let nodeConst1 = i32( instanceIndex );
	let nodeConst2 = ( nodeConst1 % nodeConst0 );
	let nodeConst3 = ( nodeConst1 / nodeConst0 );
	nodeVar0 = textureLoad( nodeUniform0, vec2<i32>( nodeConst2, nodeConst3 ), u32( 0u ) ).x;
	varyings.vBatchIndirectId = nodeVar0;
	let nodeConst4 = i32( textureDimensions( nodeUniform1, 0 ).x );
	let nodeConst5 = i32( ( f32( nodeVar0 ) * 4.0 ) );
	let nodeConst6 = ( nodeConst5 % nodeConst4 );
	let nodeConst7 = ( nodeConst5 / nodeConst4 );
	let nodeConst8 = i32( textureDimensions( nodeUniform2, 0 ).x );
	let nodeConst9 = i32( nodeVar0 );
	let nodeConst10 = ( nodeConst9 % nodeConst8 );
	let nodeConst11 = ( nodeConst9 / nodeConst8 );
	nodeVar1 = textureLoad( nodeUniform2, vec2<i32>( nodeConst10, nodeConst11 ), u32( 0u ) );
	varyings.vBatchColor = nodeVar1;
	positionLocal = position;
	nodeVar2 = textureLoad( nodeUniform1, vec2<i32>( nodeConst6, nodeConst7 ), u32( 0u ) );
	nodeVar3 = textureLoad( nodeUniform1, vec2<i32>( ( nodeConst6 + 1 ), nodeConst7 ), u32( 0u ) );
	nodeVar4 = textureLoad( nodeUniform1, vec2<i32>( ( nodeConst6 + 2 ), nodeConst7 ), u32( 0u ) );
	nodeVar5 = textureLoad( nodeUniform1, vec2<i32>( ( nodeConst6 + 3 ), nodeConst7 ), u32( 0u ) );
	let nodeConst12 = mat4x4<f32>( nodeVar2, nodeVar3, nodeVar4, nodeVar5 );
	positionLocal = ( nodeConst12 * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	normalLocal = normal;
	let nodeConst13 = mat3x3<f32>( nodeConst12[ 0 ].xyz, nodeConst12[ 1 ].xyz, nodeConst12[ 2 ].xyz );
	normalLocal = ( nodeConst13 * ( normalLocal / vec3<f32>( dot( nodeConst13[ 0u ], nodeConst13[ 0u ] ), dot( nodeConst13[ 1u ], nodeConst13[ 1u ] ), dot( nodeConst13[ 2u ], nodeConst13[ 2u ] ) ) ) );
	v_cameraIndex = cameraIndex.u_cameraIndex;
	modelViewMatrix = ( cameraViewMatrices.value[ v_cameraIndex ] * object.nodeUniform7 );
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	let VERTEX_nodeConst15 = ( cameraProjectionMatrices.value[ v_cameraIndex ] * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeConst15;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
