// Three.js r186 - Node System

// directives


// structs


// uniforms

struct NodeBuffer_1019Struct {
	value : array< u32 >
};
@binding( 1 ) @group( 1 )
var<storage, read> NodeBuffer_1019 : NodeBuffer_1019Struct;

struct NodeBuffer_995Struct {
	value : array< vec2<f32> >
};
@binding( 2 ) @group( 1 )
var<storage, read> NodeBuffer_995 : NodeBuffer_995Struct;

struct NodeBuffer_996Struct {
	value : array< u32 >
};
@binding( 3 ) @group( 1 )
var<storage, read> NodeBuffer_996 : NodeBuffer_996Struct;

struct NodeBuffer_1012Struct {
	value : array< mat4x4<f32> >
};
@binding( 4 ) @group( 1 )
var<storage, read> NodeBuffer_1012 : NodeBuffer_1012Struct;

struct NodeBuffer_994Struct {
	value : array< vec4<f32> >
};
@binding( 5 ) @group( 1 )
var<storage, read> NodeBuffer_994 : NodeBuffer_994Struct;

struct NodeBuffer_993Struct {
	value : array< vec4<f32> >
};
@binding( 6 ) @group( 1 )
var<storage, read> NodeBuffer_993 : NodeBuffer_993Struct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform9 : mat4x4<f32>
};
@binding( 7 ) @group( 1 )
var<uniform> object : objectStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) @interpolate(flat, either) vInstId : u32,
	@location( 1 ) @interpolate(flat, either) vMegaTriIdx : u32,
	@location( 2 ) vUv : vec2<f32>,
	@location( 3 ) vNormal : vec3<f32>,
	@location( 4 ) vTangent : vec3<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> nodeVar0 : u32;
var<private> nodeVar1 : vec2<f32>;
var<private> nodeVar2 : u32;
var<private> nodeVar3 : vec2<f32>;
var<private> nodeVar4 : u32;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec2<f32>;
var<private> nodeVar7 : vec2<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> VERTEX_nodeVar15 : vec4<f32>;
var<private> positionLocal : vec3<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes


@vertex
fn main( @builtin( vertex_index ) vertexIndex : u32,
	@location( 0 ) position : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	positionLocal = position;
	nodeVar0 = ( ( ( vertexIndex / 3u ) * 2u ) + 1u );
	varyings.vInstId = NodeBuffer_1019.value[ nodeVar0 ];
	varyings.vMegaTriIdx = NodeBuffer_1019.value[ ( nodeVar0 + 1u ) ];
	nodeVar2 = ( vertexIndex % 3u );

	if ( ( f32( nodeVar2 ) == 1.0 ) ) {

		nodeVar1 = NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( NodeBuffer_1019.value[ ( nodeVar0 + 1u ) ] * 3u ) + 1u ) ] ];

	} else {


		if ( ( f32( nodeVar2 ) == 2.0 ) ) {

			nodeVar3 = NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( NodeBuffer_1019.value[ ( nodeVar0 + 1u ) ] * 3u ) + 2u ) ] ];

		} else {

			nodeVar3 = NodeBuffer_995.value[ NodeBuffer_996.value[ ( NodeBuffer_1019.value[ ( nodeVar0 + 1u ) ] * 3u ) ] ];

		}

		nodeVar1 = nodeVar3;

	}

	varyings.vUv = nodeVar1;
	nodeVar4 = ( NodeBuffer_1019.value[ ( nodeVar0 + 1u ) ] * 3u );
	nodeVar5 = normalize( ( NodeBuffer_1012.value[ NodeBuffer_1019.value[ nodeVar0 ] ] * vec4<f32>( NodeBuffer_994.value[ NodeBuffer_996.value[ ( nodeVar4 + nodeVar2 ) ] ].xyz, 0.0 ) ).xyz );
	varyings.vNormal = nodeVar5;
	nodeVar6 = ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( nodeVar4 + 2u ) ] ] - NodeBuffer_995.value[ NodeBuffer_996.value[ nodeVar4 ] ] );
	nodeVar7 = ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( nodeVar4 + 1u ) ] ] - NodeBuffer_995.value[ NodeBuffer_996.value[ nodeVar4 ] ] );
	nodeVar8 = ( ( ( ( ( NodeBuffer_1012.value[ NodeBuffer_1019.value[ nodeVar0 ] ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( nodeVar4 + 1u ) ] ] ).xyz - ( NodeBuffer_1012.value[ NodeBuffer_1019.value[ nodeVar0 ] ] * NodeBuffer_993.value[ NodeBuffer_996.value[ nodeVar4 ] ] ).xyz ) * vec3<f32>( nodeVar6.y ) ) - ( ( ( NodeBuffer_1012.value[ NodeBuffer_1019.value[ nodeVar0 ] ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( nodeVar4 + 2u ) ] ] ).xyz - ( NodeBuffer_1012.value[ NodeBuffer_1019.value[ nodeVar0 ] ] * NodeBuffer_993.value[ NodeBuffer_996.value[ nodeVar4 ] ] ).xyz ) * vec3<f32>( nodeVar7.y ) ) ) * vec3<f32>( sign( ( ( nodeVar7.x * nodeVar6.y ) - ( nodeVar7.y * nodeVar6.x ) ) ) ) );
	varyings.vTangent = normalize( ( nodeVar8 - ( nodeVar5 * vec3<f32>( dot( nodeVar5, nodeVar8 ) ) ) ) );

	if ( ( f32( nodeVar2 ) == 1.0 ) ) {

		nodeVar9 = ( NodeBuffer_1012.value[ NodeBuffer_1019.value[ nodeVar0 ] ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( nodeVar4 + 1u ) ] ] ).xyz;

	} else {


		if ( ( f32( nodeVar2 ) == 2.0 ) ) {

			nodeVar10 = ( NodeBuffer_1012.value[ NodeBuffer_1019.value[ nodeVar0 ] ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( nodeVar4 + 2u ) ] ] ).xyz;

		} else {

			nodeVar10 = ( NodeBuffer_1012.value[ NodeBuffer_1019.value[ nodeVar0 ] ] * NodeBuffer_993.value[ NodeBuffer_996.value[ nodeVar4 ] ] ).xyz;

		}

		nodeVar9 = nodeVar10;

	}

	positionLocal = nodeVar9;
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform9 );
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	VERTEX_nodeVar15 = ( render.cameraProjectionMatrix * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar15;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
