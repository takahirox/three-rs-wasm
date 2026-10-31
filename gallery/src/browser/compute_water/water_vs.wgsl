// Three.js r187 - Node System

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
	nodeUniform18 : mat4x4<f32>,
	nodeUniform20 : f32,
	nodeUniform21 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : vec3<f32>
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
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> modelViewMatrix : mat4x4<f32>;
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

		let nodeConst1 = i32( ( vertexIndex / 128u ) );
		let nodeConst2 = i32( ( vertexIndex % 128u ) );
		let nodeConst3 = ( ( nodeConst1 * 128 ) + max( 0, ( nodeConst2 - 1 ) ) );
		nodeVar1 = HeightA.value[ nodeConst3 ];

	} else {

		let nodeConst1 = i32( ( vertexIndex / 128u ) );
		let nodeConst2 = i32( ( vertexIndex % 128u ) );
		let nodeConst3 = ( ( nodeConst1 * 128 ) + max( 0, ( nodeConst2 - 1 ) ) );
		nodeVar1 = HeightB.value[ nodeConst3 ];

	}


	if ( bool( object.nodeUniform0 ) ) {

		let nodeConst1 = i32( ( vertexIndex / 128u ) );
		let nodeConst2 = i32( ( vertexIndex % 128u ) );
		let nodeConst4 = ( ( nodeConst1 * 128 ) + min( ( nodeConst2 + 1 ), i32( ( 128u - 1u ) ) ) );
		nodeVar2 = HeightA.value[ nodeConst4 ];

	} else {

		let nodeConst1 = i32( ( vertexIndex / 128u ) );
		let nodeConst2 = i32( ( vertexIndex % 128u ) );
		let nodeConst4 = ( ( nodeConst1 * 128 ) + min( ( nodeConst2 + 1 ), i32( ( 128u - 1u ) ) ) );
		nodeVar2 = HeightB.value[ nodeConst4 ];

	}


	if ( bool( object.nodeUniform0 ) ) {

		let nodeConst1 = i32( ( vertexIndex / 128u ) );
		let nodeConst2 = i32( ( vertexIndex % 128u ) );
		let nodeConst5 = ( ( max( 0, ( nodeConst1 - 1 ) ) * 128 ) + nodeConst2 );
		nodeVar3 = HeightA.value[ nodeConst5 ];

	} else {

		let nodeConst1 = i32( ( vertexIndex / 128u ) );
		let nodeConst2 = i32( ( vertexIndex % 128u ) );
		let nodeConst5 = ( ( max( 0, ( nodeConst1 - 1 ) ) * 128 ) + nodeConst2 );
		nodeVar3 = HeightB.value[ nodeConst5 ];

	}


	if ( bool( object.nodeUniform0 ) ) {

		let nodeConst1 = i32( ( vertexIndex / 128u ) );
		let nodeConst2 = i32( ( vertexIndex % 128u ) );
		let nodeConst6 = ( ( min( ( nodeConst1 + 1 ), i32( ( 128u - 1u ) ) ) * 128 ) + nodeConst2 );
		nodeVar4 = HeightA.value[ nodeConst6 ];

	} else {

		let nodeConst1 = i32( ( vertexIndex / 128u ) );
		let nodeConst2 = i32( ( vertexIndex % 128u ) );
		let nodeConst6 = ( ( min( ( nodeConst1 + 1 ), i32( ( 128u - 1u ) ) ) * 128 ) + nodeConst2 );
		nodeVar4 = HeightB.value[ nodeConst6 ];

	}

	varyings.nodeVarying4 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( object.nodeUniform8 * vec3<f32>( ( ( nodeVar1 - nodeVar2 ) * 21.333333333333332 ), ( - ( ( nodeVar3 - nodeVar4 ) * 21.333333333333332 ) ), 1.0 ) ), 0.0 ) ).xyz );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform12 );
	v_positionView = ( modelViewMatrix * vec4<f32>( positionLocal, 1.0 ) ).xyz;
	varyings.v_positionViewDirection = ( - v_positionView );
	let VERTEX_nodeConst28 = ( render.cameraProjectionMatrix * vec4<f32>( v_positionView, 1.0 ) );
	VERTEX_v_modelViewProjection = VERTEX_nodeConst28;

	// result

	varyings.builtinClipSpace = VERTEX_v_modelViewProjection;

	return varyings;

}
