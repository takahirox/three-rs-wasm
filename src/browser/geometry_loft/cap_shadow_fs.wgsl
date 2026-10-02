// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform3 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : f32;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : i32;
var<private> nodeVar4 : i32;
var<private> nodeVar5 : i32;
var<private> nodeVar6 : i32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : i32;
var<private> nodeVar16 : i32;
var<private> nodeVar17 : i32;
var<private> nodeVar18 : u32;
var<private> nodeVar19 : u32;
var<private> nodeVar20 : u32;
var<private> nodeVar21 : u32;
var<private> nodeVar22 : vec3<u32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar25 : vec4<f32>;

// codes
fn mx_rotl32 ( x : u32, k : i32 ) -> u32 {

	var nodeVar0 : i32;
	var nodeVar1 : u32;

	nodeVar0 = k;
	nodeVar1 = x;

	return ( ( nodeVar1 << u32( nodeVar0 ) ) | ( nodeVar1 >> u32( ( 32 - nodeVar0 ) ) ) );

}


fn mx_bjmix ( a : u32, b : u32, c : u32 ) -> vec3<u32> {

	var nodeVar0 : u32;
	var nodeVar1 : u32;
	var nodeVar2 : u32;

	nodeVar0 = a;
	nodeVar1 = b;
	nodeVar2 = c;
	nodeVar0 = ( nodeVar0 - nodeVar2 );
	nodeVar0 = ( nodeVar0 ^ mx_rotl32( nodeVar2, 4 ) );
	nodeVar2 = ( nodeVar2 + nodeVar1 );
	nodeVar1 = ( nodeVar1 - nodeVar0 );
	nodeVar1 = ( nodeVar1 ^ mx_rotl32( nodeVar0, 6 ) );
	nodeVar0 = ( nodeVar0 + nodeVar2 );
	nodeVar2 = ( nodeVar2 - nodeVar1 );
	nodeVar2 = ( nodeVar2 ^ mx_rotl32( nodeVar1, 8 ) );
	nodeVar1 = ( nodeVar1 + nodeVar0 );
	nodeVar0 = ( nodeVar0 - nodeVar2 );
	nodeVar0 = ( nodeVar0 ^ mx_rotl32( nodeVar2, 16 ) );
	nodeVar2 = ( nodeVar2 + nodeVar1 );
	nodeVar1 = ( nodeVar1 - nodeVar0 );
	nodeVar1 = ( nodeVar1 ^ mx_rotl32( nodeVar0, 19 ) );
	nodeVar0 = ( nodeVar0 + nodeVar2 );
	nodeVar2 = ( nodeVar2 - nodeVar1 );
	nodeVar2 = ( nodeVar2 ^ mx_rotl32( nodeVar1, 4 ) );
	nodeVar1 = ( nodeVar1 + nodeVar0 );

	return vec3<u32>( nodeVar0, nodeVar1, nodeVar2 );

}


fn mx_bjfinal ( a : u32, b : u32, c : u32 ) -> u32 {

	var nodeVar0 : u32;
	var nodeVar1 : u32;
	var nodeVar2 : u32;

	nodeVar0 = c;
	nodeVar1 = b;
	nodeVar2 = a;
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - mx_rotl32( nodeVar1, 14 ) );
	nodeVar2 = ( nodeVar2 ^ nodeVar0 );
	nodeVar2 = ( nodeVar2 - mx_rotl32( nodeVar0, 11 ) );
	nodeVar1 = ( nodeVar1 ^ nodeVar2 );
	nodeVar1 = ( nodeVar1 - mx_rotl32( nodeVar2, 25 ) );
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - mx_rotl32( nodeVar1, 16 ) );
	nodeVar2 = ( nodeVar2 ^ nodeVar0 );
	nodeVar2 = ( nodeVar2 - mx_rotl32( nodeVar0, 4 ) );
	nodeVar1 = ( nodeVar1 ^ nodeVar2 );
	nodeVar1 = ( nodeVar1 - mx_rotl32( nodeVar2, 14 ) );
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - mx_rotl32( nodeVar1, 24 ) );

	return nodeVar0;

}


fn mx_bits_to_01 ( bits : u32 ) -> f32 {

	var nodeVar0 : u32;

	nodeVar0 = bits;

	return ( f32( nodeVar0 ) / 4294967295.0 );

}


fn mx_floor ( x : f32 ) -> i32 {

	var nodeVar0 : f32;

	nodeVar0 = x;

	return i32( floor( nodeVar0 ) );

}


fn mx_hash_int_2 ( x : i32, y : i32, z : i32 ) -> u32 {

	var nodeVar0 : i32;
	var nodeVar1 : i32;
	var nodeVar2 : i32;
	var nodeVar3 : u32;
	var nodeVar4 : u32;
	var nodeVar5 : u32;
	var nodeVar6 : u32;

	nodeVar0 = z;
	nodeVar1 = y;
	nodeVar2 = x;
	nodeVar3 = 3u;
	nodeVar4 = 0u;
	nodeVar5 = 0u;
	nodeVar6 = 0u;
	nodeVar6 = ( ( 3735928559u + ( nodeVar3 << 2u ) ) + 13u );
	nodeVar5 = nodeVar6;
	nodeVar4 = nodeVar5;
	nodeVar4 = ( nodeVar4 + u32( nodeVar2 ) );
	nodeVar5 = ( nodeVar5 + u32( nodeVar1 ) );
	nodeVar6 = ( nodeVar6 + u32( nodeVar0 ) );

	return mx_bjfinal( nodeVar4, nodeVar5, nodeVar6 );

}


fn mx_worley_distance_1 ( p : vec3<f32>, x : i32, y : i32, z : i32, xoff : i32, yoff : i32, zoff : i32, jitter : f32, metric : i32 ) -> f32 {

	var nodeVar0 : i32;
	var nodeVar1 : f32;
	var nodeVar2 : i32;
	var nodeVar3 : i32;
	var nodeVar4 : i32;
	var nodeVar5 : i32;
	var nodeVar6 : i32;
	var nodeVar7 : i32;
	var nodeVar8 : vec3<f32>;
	var nodeVar9 : vec3<f32>;
	var nodeVar10 : i32;
	var nodeVar11 : i32;
	var nodeVar12 : i32;
	var nodeVar13 : u32;
	var nodeVar14 : u32;
	var nodeVar15 : u32;
	var nodeVar16 : u32;
	var nodeVar17 : vec3<u32>;
	var nodeVar18 : vec3<f32>;
	var nodeVar19 : vec3<f32>;
	var nodeVar20 : vec3<f32>;

	nodeVar0 = metric;
	nodeVar1 = jitter;
	nodeVar2 = zoff;
	nodeVar3 = yoff;
	nodeVar4 = xoff;
	nodeVar5 = z;
	nodeVar6 = y;
	nodeVar7 = x;
	nodeVar8 = p;
	nodeVar9 = vec3<f32>( f32( ( nodeVar7 + nodeVar4 ) ), f32( ( nodeVar6 + nodeVar3 ) ), f32( ( nodeVar5 + nodeVar2 ) ) );
	nodeVar10 = i32( floor( nodeVar9.x ) );
	nodeVar11 = i32( floor( nodeVar9.y ) );
	nodeVar12 = i32( floor( nodeVar9.z ) );
	nodeVar13 = 3735928588u;
	nodeVar14 = 0u;
	nodeVar15 = 0u;
	nodeVar16 = 0u;
	nodeVar16 = nodeVar13;
	nodeVar15 = nodeVar16;
	nodeVar14 = nodeVar15;
	nodeVar14 = ( nodeVar14 + u32( nodeVar10 ) );
	nodeVar15 = ( nodeVar15 + u32( nodeVar11 ) );
	nodeVar16 = ( nodeVar16 + u32( nodeVar12 ) );
	nodeVar17 = mx_bjmix( nodeVar14, nodeVar15, nodeVar16 );
	nodeVar18 = vec3<f32>( mx_bits_to_01( mx_bjfinal( nodeVar17.x, nodeVar17.y, nodeVar17.z ) ), mx_bits_to_01( mx_bjfinal( ( nodeVar17.x + 1u ), nodeVar17.y, nodeVar17.z ) ), mx_bits_to_01( mx_bjfinal( ( nodeVar17.x + 2u ), nodeVar17.y, nodeVar17.z ) ) );
	nodeVar18 = ( nodeVar18 - vec3<f32>( 0.5 ) );
	nodeVar18 = ( nodeVar18 * vec3<f32>( nodeVar1 ) );
	nodeVar18 = ( nodeVar18 + vec3<f32>( 0.5 ) );
	nodeVar19 = ( vec3<f32>( f32( nodeVar7 ), f32( nodeVar6 ), f32( nodeVar5 ) ) + nodeVar18 );
	nodeVar20 = ( nodeVar19 - nodeVar8 );

	if ( ( nodeVar0 == 2 ) ) {

		return ( ( abs( nodeVar20.x ) + abs( nodeVar20.y ) ) + abs( nodeVar20.z ) );

	}


	if ( ( nodeVar0 == 3 ) ) {

		return max( max( abs( nodeVar20.x ), abs( nodeVar20.y ) ), abs( nodeVar20.z ) );

	}


	return dot( nodeVar20, nodeVar20 );

}


fn mx_cell_noise_float_2 ( p : vec3<f32> ) -> f32 {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : i32;
	var nodeVar2 : i32;
	var nodeVar3 : i32;

	nodeVar0 = p;
	nodeVar1 = mx_floor( nodeVar0.x );
	nodeVar2 = mx_floor( nodeVar0.y );
	nodeVar3 = mx_floor( nodeVar0.z );

	return mx_bits_to_01( mx_hash_int_2( nodeVar1, nodeVar2, nodeVar3 ) );

}




@fragment
fn main( @location( 0 ) positionLocal : vec3<f32>,
	@location( 1 ) nodeVarying4 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = smoothstep( 0.42, 0.58, nodeVarying4.x );
	nodeVar1 = ( positionLocal * vec3<f32>( 2.4 ) );
	nodeVar2 = 1.0;
	nodeVar3 = 0;
	nodeVar4 = 0;
	nodeVar5 = 0;
	nodeVar6 = 0;
	nodeVar7 = nodeVar1.x;
	nodeVar4 = mx_floor( nodeVar7 );
	nodeVar8 = nodeVar1.y;
	nodeVar5 = mx_floor( nodeVar8 );
	nodeVar9 = nodeVar1.z;
	nodeVar6 = mx_floor( nodeVar9 );
	nodeVar10 = vec3<f32>( ( nodeVar7 - f32( nodeVar4 ) ), ( nodeVar8 - f32( nodeVar5 ) ), ( nodeVar9 - f32( nodeVar6 ) ) );
	nodeVar11 = 1000000.0;
	nodeVar12 = vec3<f32>( 0.0, 0.0, 0.0 );

	for ( var x : i32 = -1; x <= 1; x ++ ) {


		for ( var y : i32 = -1; y <= 1; y ++ ) {


			for ( var z : i32 = -1; z <= 1; z ++ ) {

				nodeVar13 = mx_worley_distance_1( nodeVar10, x, y, z, nodeVar4, nodeVar5, nodeVar6, nodeVar2, 0 );
				nodeVar14 = vec3<f32>( f32( ( nodeVar4 + x ) ), f32( ( nodeVar5 + y ) ), f32( ( nodeVar6 + z ) ) );
				nodeVar15 = i32( floor( nodeVar14.x ) );
				nodeVar16 = i32( floor( nodeVar14.y ) );
				nodeVar17 = i32( floor( nodeVar14.z ) );
				nodeVar18 = 3735928588u;
				nodeVar19 = 0u;
				nodeVar20 = 0u;
				nodeVar21 = 0u;
				nodeVar21 = nodeVar18;
				nodeVar20 = nodeVar21;
				nodeVar19 = nodeVar20;
				nodeVar19 = ( nodeVar19 + u32( nodeVar15 ) );
				nodeVar20 = ( nodeVar20 + u32( nodeVar16 ) );
				nodeVar21 = ( nodeVar21 + u32( nodeVar17 ) );
				nodeVar22 = mx_bjmix( nodeVar19, nodeVar20, nodeVar21 );
				nodeVar23 = vec3<f32>( mx_bits_to_01( mx_bjfinal( nodeVar22.x, nodeVar22.y, nodeVar22.z ) ), mx_bits_to_01( mx_bjfinal( ( nodeVar22.x + 1u ), nodeVar22.y, nodeVar22.z ) ), mx_bits_to_01( mx_bjfinal( ( nodeVar22.x + 2u ), nodeVar22.y, nodeVar22.z ) ) );
				nodeVar23 = ( nodeVar23 - vec3<f32>( 0.5 ) );
				nodeVar23 = ( nodeVar23 * vec3<f32>( nodeVar2 ) );
				nodeVar23 = ( nodeVar23 + vec3<f32>( 0.5 ) );
				nodeVar24 = ( ( vec3<f32>( f32( x ), f32( y ), f32( z ) ) + nodeVar23 ) - nodeVar10 );

				if ( ( nodeVar13 < nodeVar11 ) ) {

					nodeVar11 = nodeVar13;
					nodeVar12 = nodeVar24;
					

				}


			}


		}


	}


	if ( ( nodeVar3 == 1 ) ) {

		nodeVar11 = mx_cell_noise_float_2( ( nodeVar12 + nodeVar1 ) );
		

	} else {

		nodeVar11 = sqrt( nodeVar11 );
		

	}

	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), ( 1.0 * vec4<f32>( mix( mix( vec3<f32>( 0.8069522576650873, 0.7156935005005721, 0.55201140150344 ), vec3<f32>( 0.5209955731953768, 0.4232676699760063, 0.2704977910022518 ), ( ( ( sin( ( nodeVarying4.y * 376.99111843077515 ) ) * 0.5 ) + 0.5 ) * ( 1.0 - nodeVar0 ) ) ), mix( vec3<f32>( 0.36625259558833256, 0.026241221889696346, 0.014443843592229466 ), vec3<f32>( 0.8879231178794776, 0.8148465722120952, 0.6866853124288864 ), ( ( 1.0 - smoothstep( 0.18, 0.38, nodeVar11 ) ) * nodeVar0 ) ), nodeVar0 ), 1.0 ).w ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	nodeVar25 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar25;

	// result

	output.color = nodeVar25;

	return output;

}
