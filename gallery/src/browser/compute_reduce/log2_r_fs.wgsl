// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms

struct Current_RightStruct {
	value : array< u32 >
};
@binding( 0 ) @group( 1 )
var<storage, read> Current_Right : Current_RightStruct;

struct objectStruct {
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform7 : mat4x4<f32>
};
@binding( 1 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : u32;
var<private> color : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) nodeVarying4 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = 0u;
	color = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar0 = Current_Right.value[ ( 1u << u32( ( ( nodeVarying4 * vec2<f32>( 18.0, 18.0 ) ) * vec2<f32>( 1.0 ) ).x ) ) ];
	nodeVar1 = ( f32( nodeVar0 ) / ( object.nodeUniform1 * object.nodeUniform2 ) );
	nodeVar2 = vec3<f32>( ( 1.0 - nodeVar1 ) );

	if ( ( object.nodeUniform3 == 1.0 ) ) {


		if ( ( f32( Current_Right.value[ 0u ] ) == 262144.0 ) ) {

			nodeVar2 = vec3<f32>( 0.0, ( 1.0 - nodeVar1 ), 0.0 );
			

		} else {

			nodeVar2 = vec3<f32>( ( 1.0 - nodeVar1 ), 0.0, 0.0 );
			

		}

		

	}

	color = nodeVar2;
	DiffuseColor = vec4<f32>( color, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform4 );
	DiffuseColor.w = 1.0;
	nodeVar3 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar3;

	// result

	output.color = nodeVar3;

	return output;

}
