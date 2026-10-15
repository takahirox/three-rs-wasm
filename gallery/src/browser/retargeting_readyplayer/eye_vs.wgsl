// Three.js r186 - Node System

// directives


// structs


// uniforms
@binding( 6 ) @group( 1 ) var nodeUniform2 : texture_2d_array<f32>;

struct NodeBuffer_2401Struct {
	value : array< vec4<f32>, 2 >
};
@binding( 5 ) @group( 1 )
var<uniform> NodeBuffer_2401 : NodeBuffer_2401Struct;

struct NodeBuffer_2589Struct {
	value : array< mat4x4<f32>, 67 >
};
@binding( 7 ) @group( 1 )
var<uniform> NodeBuffer_2589 : NodeBuffer_2589Struct;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform3 : mat4x4<f32>,
	nodeUniform5 : mat4x4<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform9 : f32,
	nodeUniform10 : f32,
	nodeUniform11 : f32,
	nodeUniform13 : mat3x3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : f32,
	nodeUniform17 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform19 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform18 : vec3<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform26 : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@location( 2 ) nodeVarying6 : vec2<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : i32;
var<private> nodeVar3 : i32;
var<private> nodeVar4 : vec2<i32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> normalLocal : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> VERTEX_nodeVar174 : vec4<f32>;
var<private> positionLocal : vec3<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes


@vertex
fn main( @builtin( vertex_index ) vertexIndex : u32,
	@location( 0 ) position : vec3<f32>,
	@location( 1 ) skinIndex : vec4<u32>,
	@location( 2 ) skinWeight : vec4<f32>,
	@location( 3 ) normal : vec3<f32>,
	@location( 4 ) uv : vec2<f32> ) -> VaryingsStruct {

	// flow
	// code

	positionLocal = position;
	positionLocal = ( positionLocal * vec3<f32>( object.nodeUniform0 ) );

	for ( var i : i32 = 0; i < 2; i ++ ) {

		nodeVar0 = 0.0;
		nodeVar1 = NodeBuffer_2401.value[ i ].x;
		nodeVar0 = nodeVar1;

		if ( ( nodeVar0 != 0.0 ) ) {

			nodeVar2 = ( ( i32( vertexIndex ) * 1 ) + 0 );
			nodeVar3 = ( nodeVar2 / 120 );
			nodeVar4 = vec2<i32>( ( nodeVar2 - ( nodeVar3 * 120 ) ), nodeVar3 );
			nodeVar5 = textureLoad( nodeUniform2, nodeVar4, i, u32( 0u ) );
			positionLocal = ( positionLocal + ( nodeVar5.xyz * vec3<f32>( nodeVar0 ) ) );
			

		}


	}

	nodeVar6 = ( object.nodeUniform5 * vec4<f32>( positionLocal, 1.0 ) );
	positionLocal = ( object.nodeUniform3 * ( ( ( ( ( skinWeight.x * NodeBuffer_2589.value[ skinIndex.x ] ) * nodeVar6 ) + ( ( skinWeight.y * NodeBuffer_2589.value[ skinIndex.y ] ) * nodeVar6 ) ) + ( ( skinWeight.z * NodeBuffer_2589.value[ skinIndex.z ] ) * nodeVar6 ) ) + ( ( skinWeight.w * NodeBuffer_2589.value[ skinIndex.w ] ) * nodeVar6 ) ) ).xyz;
	normalLocal = normal;
	normalLocal = ( mat3x3<f32>( ( ( object.nodeUniform3 * ( ( ( skinWeight.x * NodeBuffer_2589.value[ skinIndex.x ] + skinWeight.y * NodeBuffer_2589.value[ skinIndex.y ] ) + skinWeight.z * NodeBuffer_2589.value[ skinIndex.z ] ) + skinWeight.w * NodeBuffer_2589.value[ skinIndex.w ] ) ) * object.nodeUniform5 )[ 0 ].xyz, ( ( object.nodeUniform3 * ( ( ( skinWeight.x * NodeBuffer_2589.value[ skinIndex.x ] + skinWeight.y * NodeBuffer_2589.value[ skinIndex.y ] ) + skinWeight.z * NodeBuffer_2589.value[ skinIndex.z ] ) + skinWeight.w * NodeBuffer_2589.value[ skinIndex.w ] ) ) * object.nodeUniform5 )[ 1 ].xyz, ( ( object.nodeUniform3 * ( ( ( skinWeight.x * NodeBuffer_2589.value[ skinIndex.x ] + skinWeight.y * NodeBuffer_2589.value[ skinIndex.y ] ) + skinWeight.z * NodeBuffer_2589.value[ skinIndex.z ] ) + skinWeight.w * NodeBuffer_2589.value[ skinIndex.w ] ) ) * object.nodeUniform5 )[ 2 ].xyz ) * normalLocal );
	varyings.nodeVarying6 = uv;
	varyings.v_normalViewGeometry = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform13 * normalLocal ), 0.0 ) ).xyz );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform17 );
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - v_positionView );
	VERTEX_nodeVar174 = ( render.cameraProjectionMatrix * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar174;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
