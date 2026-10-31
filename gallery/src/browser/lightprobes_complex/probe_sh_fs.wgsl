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
@binding( 1 ) @group( 0 ) var nodeUniform2 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform1 : i32,
	nodeUniform3 : mat3x3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar0 : i32;
var<private> nodeVar1 : i32;
var<private> nodeVar2 : i32;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec2<i32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec4<f32>;

// codes


@fragment
fn main( @builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	Output = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	let nodeConst0 = i32( fragCoord.xy.x );
	nodeVar0 = ( nodeConst0 / 24 );
	nodeVar1 = ( ( nodeConst0 / 4 ) % 6 );
	nodeVar2 = ( ( nodeConst0 % 4 ) * 8 );
	nodeVar3 = array< vec3<f32>, 6 >( vec3<f32>( 1.0, 0.0, 0.0 ), vec3<f32>( -1.0, 0.0, 0.0 ), vec3<f32>( 0.0, 1.0, 0.0 ), vec3<f32>( 0.0, -1.0, 0.0 ), vec3<f32>( 0.0, 0.0, 1.0 ), vec3<f32>( 0.0, 0.0, -1.0 ) )[ nodeVar1 ];
	nodeVar4 = array< vec3<f32>, 6 >( vec3<f32>( 0.0, 0.0, 1.0 ), vec3<f32>( 0.0, 0.0, -1.0 ), vec3<f32>( -1.0, 0.0, 0.0 ), vec3<f32>( -1.0, 0.0, 0.0 ), vec3<f32>( -1.0, 0.0, 0.0 ), vec3<f32>( 1.0, 0.0, 0.0 ) )[ nodeVar1 ];
	nodeVar5 = array< vec3<f32>, 6 >( vec3<f32>( 0.0, 1.0, 0.0 ), vec3<f32>( 0.0, 1.0, 0.0 ), vec3<f32>( 0.0, 0.0, -1.0 ), vec3<f32>( 0.0, 0.0, 1.0 ), vec3<f32>( 0.0, 1.0, 0.0 ), vec3<f32>( 0.0, 1.0, 0.0 ) )[ nodeVar1 ];
	nodeVar6 = vec2<i32>( ( nodeVar1 * 32 ), ( ( ( i32( fragCoord.xy.y ) - object.nodeUniform1 ) * 32 ) + nodeVar2 ) );
	nodeVar7 = vec3<f32>( 0.0, 0.0, 0.0 );

	for ( var i : i32 = 0; i < 32; i ++ ) {

		for ( var j : i32 = 0; j < 8; j ++ ) {

			let nodeConst1 = ( ( f32( i ) * 0.0625 ) - 1.0 );
			let nodeConst2 = ( 1.0 - ( f32( ( j + nodeVar2 ) ) * 0.0625 ) );
			nodeVar8 = normalize( ( ( nodeVar3 + ( nodeVar4 * vec3<f32>( ( nodeConst1 + 0.03125 ) ) ) ) + ( nodeVar5 * vec3<f32>( ( nodeConst2 - 0.03125 ) ) ) ) );
			nodeVar9 = textureLoad( nodeUniform2, vec2<i32>( ( object.nodeUniform3 * vec3<f32>( vec2<f32>( ( nodeVar6 + vec2<i32>( i, j ) ) ), 1.0 ) ).xy ), u32( 0u ) );
			let nodeConst3 = ( nodeConst2 - 0.0625 );
			let nodeConst4 = ( nodeConst1 + 0.0625 );
			nodeVar7 = ( nodeVar7 + ( nodeVar9.xyz * vec3<f32>( ( array< f32, 9 >( 0.282095, ( nodeVar8.y * 0.488603 ), ( nodeVar8.z * 0.488603 ), ( nodeVar8.x * 0.488603 ), ( ( nodeVar8.x * nodeVar8.y ) * 1.092548 ), ( ( nodeVar8.y * nodeVar8.z ) * 1.092548 ), ( ( ( ( nodeVar8.z * nodeVar8.z ) * 3.0 ) - 1.0 ) * 0.315392 ), ( ( nodeVar8.x * nodeVar8.z ) * 1.092548 ), ( ( ( nodeVar8.x * nodeVar8.x ) - ( nodeVar8.y * nodeVar8.y ) ) * 0.546274 ) )[ nodeVar0 ] * abs( ( ( ( atan( ( ( nodeConst1 * nodeConst2 ) / sqrt( ( ( ( nodeConst1 * nodeConst1 ) + ( nodeConst2 * nodeConst2 ) ) + 1.0 ) ) ) ) - atan( ( ( nodeConst1 * nodeConst3 ) / sqrt( ( ( ( nodeConst1 * nodeConst1 ) + ( nodeConst3 * nodeConst3 ) ) + 1.0 ) ) ) ) ) - atan( ( ( nodeConst4 * nodeConst2 ) / sqrt( ( ( ( nodeConst4 * nodeConst4 ) + ( nodeConst2 * nodeConst2 ) ) + 1.0 ) ) ) ) ) + atan( ( ( nodeConst4 * nodeConst3 ) / sqrt( ( ( ( nodeConst4 * nodeConst4 ) + ( nodeConst3 * nodeConst3 ) ) + 1.0 ) ) ) ) ) ) ) ) ) );

		}

	}


	// result

	output.color = vec4<f32>( nodeVar7, 1.0 );

	return output;

}
