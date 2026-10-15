// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms

struct WorkgroupSums_RightStruct {
	value : array< u32 >
};
@binding( 0 ) @group( 1 )
var<storage, read> WorkgroupSums_Right : WorkgroupSums_RightStruct;

struct Current_RightStruct {
	value : array< u32 >
};
@binding( 2 ) @group( 1 )
var<storage, read> Current_Right : Current_RightStruct;

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
var<private> nodeVar1 : vec2<f32>;
var<private> nodeVar2 : vec2<u32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) nodeVarying4 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = 0u;
	color = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar1 = ( nodeVarying4 * vec2<f32>( f32( 8u ), f32( 16u ) ) );
	nodeVar2 = vec2<u32>( u32( floor( nodeVar1.x ) ), u32( floor( nodeVar1.y ) ) );
	nodeVar0 = WorkgroupSums_Right.value[ ( ( 8u * nodeVar2.y ) + nodeVar2.x ) ];
	nodeVar3 = ( f32( nodeVar0 ) / f32( ( 8u * 16u ) ) );
	nodeVar4 = vec3<f32>( ( 1.0 - nodeVar3 ) );

	if ( ( object.nodeUniform1 == 1.0 ) ) {


		if ( ( f32( Current_Right.value[ 0u ] ) == 262144.0 ) ) {

			nodeVar4 = vec3<f32>( 0.0, ( 1.0 - nodeVar3 ), 0.0 );
			

		} else {

			nodeVar4 = vec3<f32>( ( 1.0 - nodeVar3 ), 0.0, 0.0 );
			

		}

		

	}

	color = nodeVar4;
	DiffuseColor = vec4<f32>( color, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	nodeVar5 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar5;

	// result

	output.color = nodeVar5;

	return output;

}
