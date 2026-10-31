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
@binding( 1 ) @group( 0 ) var nodeUniform3 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform1 : i32,
	nodeUniform2 : vec3<f32>,
	nodeUniform4 : mat3x3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar0 : i32;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : vec4<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : vec4<f32>;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : vec4<f32>;
var<private> nodeVar24 : vec4<f32>;

// codes


@fragment
fn main( @builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	Output = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	let nodeConst0 = i32( object.nodeUniform2.x );
	nodeVar0 = ( ( i32( fragCoord.xy.x ) + ( object.nodeUniform1 * nodeConst0 ) ) + ( ( i32( fragCoord.xy.y ) * nodeConst0 ) * i32( object.nodeUniform2.z ) ) );
	nodeVar1 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 192.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar2 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 193.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar3 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 194.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar4 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 195.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar5 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 196.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar6 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 197.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar7 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 198.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar8 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 199.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar9 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 200.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar10 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 201.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar11 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 202.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar12 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 203.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar13 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 204.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar14 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 205.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar15 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 206.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar16 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 207.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar17 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 208.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar18 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 209.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar19 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 210.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar20 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 211.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar21 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 212.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar22 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 213.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar23 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 214.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar24 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 215.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );

	// result

	output.color = vec4<f32>( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( nodeVar1 + nodeVar2 ) + nodeVar3 ) + nodeVar4 ) + nodeVar5 ) + nodeVar6 ) + nodeVar7 ) + nodeVar8 ) + nodeVar9 ) + nodeVar10 ) + nodeVar11 ) + nodeVar12 ) + nodeVar13 ) + nodeVar14 ) + nodeVar15 ) + nodeVar16 ) + nodeVar17 ) + nodeVar18 ) + nodeVar19 ) + nodeVar20 ) + nodeVar21 ) + nodeVar22 ) + nodeVar23 ) + nodeVar24 ).xyz, 0.0 );

	return output;

}
