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
var<private> nodeVar25 : vec4<f32>;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : vec4<f32>;
var<private> nodeVar28 : vec4<f32>;
var<private> nodeVar29 : vec4<f32>;
var<private> nodeVar30 : vec4<f32>;
var<private> nodeVar31 : vec4<f32>;
var<private> nodeVar32 : vec4<f32>;
var<private> nodeVar33 : vec4<f32>;
var<private> nodeVar34 : vec4<f32>;
var<private> nodeVar35 : vec4<f32>;
var<private> nodeVar36 : vec4<f32>;
var<private> nodeVar37 : vec4<f32>;
var<private> nodeVar38 : vec4<f32>;
var<private> nodeVar39 : vec4<f32>;
var<private> nodeVar40 : vec4<f32>;
var<private> nodeVar41 : vec4<f32>;
var<private> nodeVar42 : vec4<f32>;
var<private> nodeVar43 : vec4<f32>;
var<private> nodeVar44 : vec4<f32>;
var<private> nodeVar45 : vec4<f32>;
var<private> nodeVar46 : vec4<f32>;
var<private> nodeVar47 : vec4<f32>;
var<private> nodeVar48 : vec4<f32>;

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
	nodeVar1 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 96.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar2 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 97.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar3 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 98.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar4 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 99.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar5 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 100.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar6 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 101.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar7 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 102.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar8 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 103.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar9 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 104.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar10 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 105.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar11 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 106.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar12 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 107.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar13 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 108.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar14 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 109.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar15 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 110.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar16 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 111.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar17 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 112.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar18 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 113.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar19 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 114.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar20 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 115.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar21 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 116.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar22 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 117.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar23 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 118.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar24 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 119.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar25 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 120.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar26 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 121.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar27 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 122.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar28 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 123.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar29 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 124.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar30 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 125.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar31 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 126.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar32 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 127.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar33 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 128.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar34 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 129.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar35 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 130.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar36 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 131.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar37 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 132.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar38 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 133.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar39 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 134.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar40 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 135.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar41 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 136.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar42 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 137.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar43 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 138.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar44 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 139.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar45 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 140.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar46 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 141.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar47 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 142.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );
	nodeVar48 = textureLoad( nodeUniform3, vec2<i32>( ( object.nodeUniform4 * vec3<f32>( vec2<f32>( vec2<i32>( i32( 143.0 ), nodeVar0 ) ), 1.0 ) ).xy ), u32( 0u ) );

	// result

	output.color = vec4<f32>( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( nodeVar1 + nodeVar2 ) + nodeVar3 ) + nodeVar4 ) + nodeVar5 ) + nodeVar6 ) + nodeVar7 ) + nodeVar8 ) + nodeVar9 ) + nodeVar10 ) + nodeVar11 ) + nodeVar12 ) + nodeVar13 ) + nodeVar14 ) + nodeVar15 ) + nodeVar16 ) + nodeVar17 ) + nodeVar18 ) + nodeVar19 ) + nodeVar20 ) + nodeVar21 ) + nodeVar22 ) + nodeVar23 ) + nodeVar24 ).xyz, ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( ( nodeVar25 + nodeVar26 ) + nodeVar27 ) + nodeVar28 ) + nodeVar29 ) + nodeVar30 ) + nodeVar31 ) + nodeVar32 ) + nodeVar33 ) + nodeVar34 ) + nodeVar35 ) + nodeVar36 ) + nodeVar37 ) + nodeVar38 ) + nodeVar39 ) + nodeVar40 ) + nodeVar41 ) + nodeVar42 ) + nodeVar43 ) + nodeVar44 ) + nodeVar45 ) + nodeVar46 ) + nodeVar47 ) + nodeVar48 ).x );

	return output;

}
