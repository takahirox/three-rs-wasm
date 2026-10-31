// Three.js r187 - Node System

// directives


// structs


// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform0 : texture_2d<u32>;
@binding( 2 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform2 : texture_2d<f32>;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform17 : vec3<f32>,
	nodeUniform18 : f32,
	nodeUniform19 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : vec3<f32>,
	nodeUniform8 : f32,
	nodeUniform11 : mat3x3<f32>,
	nodeUniform16 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) @interpolate(flat, either) vBatchIndirectId : u32,
	@location( 4 ) vBatchColor : vec4<f32>,
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
	varyings.v_normalViewGeometry = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform11 * normalLocal ), 0.0 ) ).xyz );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform16 );
	varyings.v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - varyings.v_positionView );
	let VERTEX_nodeConst20 = ( render.cameraProjectionMatrix * vec4<f32>( varyings.v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeConst20;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
