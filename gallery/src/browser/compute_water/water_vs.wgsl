// Three.js r186 - Node System

// directives


// structs


// uniforms

struct HeightAStruct {
	value : array< f32 >
};
@binding( 5 ) @group( 1 )
var<storage, read> HeightA : HeightAStruct;

struct HeightBStruct {
	value : array< f32 >
};
@binding( 6 ) @group( 1 )
var<storage, read> HeightB : HeightBStruct;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : f32,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform17 : f32,
	nodeUniform18 : mat4x4<f32>,
	nodeUniform20 : f32,
	nodeUniform21 : f32,
	nodeUniform23 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) nodeVarying4 : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> nodeVar0 : f32;
var<private> normalLocal : vec3<f32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> VERTEX_nodeVar175 : vec4<f32>;
var<private> positionLocal : vec3<f32>;
var<private> v_modelViewProjection : vec4<f32>;
var<private> v_positionView : vec3<f32>;
var<private> VERTEX_v_modelViewProjection : vec4<f32>;

// codes


@vertex
fn main( @builtin( vertex_index ) vertexIndex : u32,
	@location( 0 ) position : vec3<f32>,
	@location( 1 ) normal : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	positionLocal = position;

	if ( bool( object.nodeUniform0 ) ) {

		nodeVar0 = HeightA.value[ vertexIndex ];

	} else {

		nodeVar0 = HeightB.value[ vertexIndex ];

	}

	positionLocal = vec3<f32>( positionLocal.x, positionLocal.y, nodeVar0 );
	normalLocal = normal;
	varyings.v_normalViewGeometry = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform8 * normalLocal ), 0.0 ) ).xyz );

	if ( bool( object.nodeUniform0 ) ) {

		nodeVar2 = HeightA.value[ ( ( i32( ( vertexIndex / 128u ) ) * 128 ) + max( 0, ( i32( ( vertexIndex % 128u ) ) - 1 ) ) ) ];

	} else {

		nodeVar2 = HeightB.value[ ( ( i32( ( vertexIndex / 128u ) ) * 128 ) + max( 0, ( i32( ( vertexIndex % 128u ) ) - 1 ) ) ) ];

	}


	if ( bool( object.nodeUniform0 ) ) {

		nodeVar3 = HeightA.value[ ( ( i32( ( vertexIndex / 128u ) ) * 128 ) + i32( min( u32( ( i32( ( vertexIndex % 128u ) ) + 1 ) ), ( 128u - 1u ) ) ) ) ];

	} else {

		nodeVar3 = HeightB.value[ ( ( i32( ( vertexIndex / 128u ) ) * 128 ) + i32( min( u32( ( i32( ( vertexIndex % 128u ) ) + 1 ) ), ( 128u - 1u ) ) ) ) ];

	}


	if ( bool( object.nodeUniform0 ) ) {

		nodeVar4 = HeightA.value[ ( ( max( 0, ( i32( ( vertexIndex / 128u ) ) - 1 ) ) * 128 ) + i32( ( vertexIndex % 128u ) ) ) ];

	} else {

		nodeVar4 = HeightB.value[ ( ( max( 0, ( i32( ( vertexIndex / 128u ) ) - 1 ) ) * 128 ) + i32( ( vertexIndex % 128u ) ) ) ];

	}


	if ( bool( object.nodeUniform0 ) ) {

		nodeVar5 = HeightA.value[ ( ( min( u32( ( i32( ( vertexIndex / 128u ) ) + 1 ) ), ( 128u - 1u ) ) * 128u ) + u32( i32( ( vertexIndex % 128u ) ) ) ) ];

	} else {

		nodeVar5 = HeightB.value[ ( ( min( u32( ( i32( ( vertexIndex / 128u ) ) + 1 ) ), ( 128u - 1u ) ) * 128u ) + u32( i32( ( vertexIndex % 128u ) ) ) ) ];

	}

	varyings.nodeVarying4 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform8 * vec3<f32>( ( ( nodeVar2 - nodeVar3 ) * 21.333333333333332 ), ( - ( ( nodeVar4 - nodeVar5 ) * 21.333333333333332 ) ), 1.0 ) ), 0.0 ) ).xyz );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform12 );
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - v_positionView );
	VERTEX_nodeVar175 = ( render.cameraProjectionMatrix * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeVar175;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
