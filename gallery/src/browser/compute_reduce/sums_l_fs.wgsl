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

struct WorkgroupSums_LeftStruct {
	value : array< u32 >
};
@binding( 0 ) @group( 1 )
var<storage, read> WorkgroupSums_Left : WorkgroupSums_LeftStruct;

struct Current_LeftStruct {
	value : array< u32 >
};
@binding( 2 ) @group( 1 )
var<storage, read> Current_Left : Current_LeftStruct;

struct objectStruct {
	nodeUniform1 : f32,
	nodeUniform3 : f32,
	nodeUniform6 : mat4x4<f32>
};
@binding( 1 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : u32;
var<private> color : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> Output : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) nodeVarying4 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = 0u;
	color = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst0 = ( nodeVarying4 * vec2<f32>( f32( 8u ), f32( 16u ) ) );
	let nodeConst1 = vec2<u32>( u32( floor( nodeConst0.x ) ), u32( floor( nodeConst0.y ) ) );
	nodeVar0 = WorkgroupSums_Left.value[ ( ( 8u * nodeConst1.y ) + nodeConst1.x ) ];
	let nodeConst2 = ( f32( nodeVar0 ) / f32( ( 8u * 16u ) ) );
	nodeVar1 = vec3<f32>( ( 1.0 - nodeConst2 ) );

	if ( ( object.nodeUniform1 == 1.0 ) ) {


		if ( ( Current_Left.value[ 0u ] == 262144u ) ) {

			nodeVar1 = vec3<f32>( 0.0, ( 1.0 - nodeConst2 ), 0.0 );
			

		} else {

			nodeVar1 = vec3<f32>( ( 1.0 - nodeConst2 ), 0.0, 0.0 );
			

		}

		

	}

	color = nodeVar1;
	DiffuseColor = vec4<f32>( color, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	let nodeConst3 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst3;

	// result

	output.color = nodeConst3;

	return output;

}
