// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );

// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform17_sampler : sampler_comparison;
@binding( 2 ) @group( 1 ) var nodeUniform17 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : vec3<f32>,
	nodeUniform3 : f32,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform11 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform23 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform5 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform21 : f32,
	nodeUniform22 : f32,
	nodeUniform14 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform10 : mat4x4<f32>,
	nodeUniform15 : f32,
	nodeUniform16 : f32,
	nodeUniform18 : f32,
	nodeUniform19 : vec2<f32>,
	nodeUniform20 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : bool;
var<private> nodeVar8 : vec2<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<bool>;
var<private> nodeVar19 : bool;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : vec4<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec2<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec2<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec2<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : vec2<f32>;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec2<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : i32;
var<private> nodeVar44 : i32;
var<private> nodeVar45 : i32;
var<private> nodeVar46 : i32;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : i32;
var<private> nodeVar56 : i32;
var<private> nodeVar57 : i32;
var<private> nodeVar58 : u32;
var<private> nodeVar59 : u32;
var<private> nodeVar60 : u32;
var<private> nodeVar61 : u32;
var<private> nodeVar62 : vec3<u32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar73 : vec4<f32>;
var<private> nodeVar74 : vec4<f32>;
var<private> nodeVar75 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar76 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec4<f32>;

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

fn interleavedGradientNoise ( position : vec2<f32> ) -> f32 {

	

	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}

fn vogelDiskSample ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	var nodeVar0 : f32;

	nodeVar0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeVar0 ), sin( nodeVar0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

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
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	EmissiveColor = ( object.nodeUniform2 * vec3<f32>( object.nodeUniform3 ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar0 = dot( normalWorld, normalize( render.nodeUniform9 ) );
	nodeVar1 = ( nodeVar0 * 0.5 );
	nodeVar2 = ( nodeVar1 + 0.5 );
	nodeVar3 = mix( render.nodeUniform4, render.nodeUniform5, nodeVar2 );
	nodeVar4 = ( irradiance + nodeVar3 );
	irradiance = nodeVar4;
	nodeVar5 = 0.0;
	nodeVar6 = ( render.nodeUniform10 * vec4<f32>( v_positionWorld, 1.0 ) );
	nodeVar7 = ( nodeVar6.w > 0.0 );

	if ( nodeVar7 ) {

		nodeVar8 = ( abs( ( ( nodeVar6.xyz / vec3<f32>( nodeVar6.w ) ).xy - vec2<f32>( 0.5, 0.5 ) ) ) - vec2<f32>( 0.5, 0.5 ) );
		nodeVar5 = clamp( ( ( ( length( max( nodeVar8, vec2<f32>( 0.0 ) ) ) + min( max( nodeVar8.x, nodeVar8.y ), 0.0 ) ) * -2.0 ) * ( -1.0 / ( ( 1.0 - acos( render.nodeUniform12 ) ) - 1.0 ) ) ), 0.0, 1.0 );
		

	}

	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar9 = ( render.nodeUniform13 - v_positionView );
	nodeVar10 = normalize( nodeVar9 );
	nodeVar11 = dot( normalView, nodeVar10 );
	nodeVar13 = ( render.nodeUniform10 * vec4<f32>( v_positionWorld, 1.0 ) );
	nodeVar14 = ( nodeVar13.xyz / vec3<f32>( nodeVar13.w ) );
	nodeVar15 = ( nodeVar14 * vec3<f32>( 2.0 ) );
	nodeVar16 = ( nodeVar15 - vec3<f32>( 1.0 ) );
	nodeVar17 = abs( nodeVar16 );
	nodeVar18 = ( nodeVar17 < vec3<f32>( 1.0 ) );
	nodeVar19 = all( nodeVar18 );

	if ( nodeVar19 ) {

		shadowPositionWorld = v_positionWorld;
		nodeVar21 = ( render.nodeUniform10 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform15 ) ) ), 1.0 ) );
		nodeVar22 = ( nodeVar21.xyz / vec3<f32>( nodeVar21.w ) );
		nodeVar23 = vec3<f32>( nodeVar22.x, ( 1.0 - nodeVar22.y ), ( nodeVar22.z + render.nodeUniform16 ) );

		if ( ( ( ( ( ( nodeVar23.x >= 0.0 ) && ( nodeVar23.x <= 1.0 ) ) && ( nodeVar23.y >= 0.0 ) ) && ( nodeVar23.y <= 1.0 ) ) && ( nodeVar23.z <= 1.0 ) ) ) {

			nodeVar24 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar25 = ( render.nodeUniform18 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform19 ).x );
			nodeVar26 = ( nodeVar23.xy + ( vogelDiskSample( 0, 5, nodeVar24 ) * vec2<f32>( nodeVar25 ) ) );
			nodeVar27 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar26, nodeVar23.z );
			nodeVar28 = ( nodeVar23.xy + ( vogelDiskSample( 1, 5, nodeVar24 ) * vec2<f32>( nodeVar25 ) ) );
			nodeVar29 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar28, nodeVar23.z );
			nodeVar30 = ( nodeVar23.xy + ( vogelDiskSample( 2, 5, nodeVar24 ) * vec2<f32>( nodeVar25 ) ) );
			nodeVar31 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar30, nodeVar23.z );
			nodeVar32 = ( nodeVar23.xy + ( vogelDiskSample( 3, 5, nodeVar24 ) * vec2<f32>( nodeVar25 ) ) );
			nodeVar33 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar32, nodeVar23.z );
			nodeVar34 = ( nodeVar23.xy + ( vogelDiskSample( 4, 5, nodeVar24 ) * vec2<f32>( nodeVar25 ) ) );
			nodeVar35 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar34, nodeVar23.z );
			nodeVar20 = ( ( ( ( ( nodeVar27 + nodeVar29 ) + nodeVar31 ) + nodeVar33 ) + nodeVar35 ) * 0.2 );

		} else {

			nodeVar20 = 1.0;

		}

		nodeVar36 = mix( 1.0, nodeVar20, render.nodeUniform20 );

		if ( ( render.nodeUniform21 > 0.0 ) ) {

			nodeVar38 = length( nodeVar9 );
			nodeVar39 = ( nodeVar38 / render.nodeUniform21 );
			nodeVar40 = clamp( ( 1.0 - ( ( ( nodeVar39 * nodeVar39 ) * nodeVar39 ) * nodeVar39 ) ), 0.0, 1.0 );
			nodeVar37 = ( ( 1.0 / max( pow( nodeVar38, render.nodeUniform22 ), 0.01 ) ) * ( nodeVar40 * nodeVar40 ) );

		} else {

			nodeVar37 = ( 1.0 / max( pow( length( nodeVar9 ), render.nodeUniform22 ), 0.01 ) );

		}

		nodeVar41 = ( ( nodeVar14 * vec3<f32>( 10.0 ) ) + vec3<f32>( render.nodeUniform23 ) );
		nodeVar42 = 1.0;
		nodeVar43 = 0;
		nodeVar44 = 0;
		nodeVar45 = 0;
		nodeVar46 = 0;
		nodeVar47 = nodeVar41.x;
		nodeVar44 = mx_floor( nodeVar47 );
		nodeVar48 = nodeVar41.y;
		nodeVar45 = mx_floor( nodeVar48 );
		nodeVar49 = nodeVar41.z;
		nodeVar46 = mx_floor( nodeVar49 );
		nodeVar50 = vec3<f32>( ( nodeVar47 - f32( nodeVar44 ) ), ( nodeVar48 - f32( nodeVar45 ) ), ( nodeVar49 - f32( nodeVar46 ) ) );
		nodeVar51 = 1000000.0;
		nodeVar52 = vec3<f32>( 0.0, 0.0, 0.0 );

		for ( var x : i32 = -1; x <= 1; x ++ ) {

			for ( var y : i32 = -1; y <= 1; y ++ ) {

				for ( var z : i32 = -1; z <= 1; z ++ ) {

					nodeVar53 = mx_worley_distance_1( nodeVar50, x, y, z, nodeVar44, nodeVar45, nodeVar46, nodeVar42, 0 );
					nodeVar54 = vec3<f32>( f32( ( nodeVar44 + x ) ), f32( ( nodeVar45 + y ) ), f32( ( nodeVar46 + z ) ) );
					nodeVar55 = i32( floor( nodeVar54.x ) );
					nodeVar56 = i32( floor( nodeVar54.y ) );
					nodeVar57 = i32( floor( nodeVar54.z ) );
					nodeVar58 = 3735928588u;
					nodeVar59 = 0u;
					nodeVar60 = 0u;
					nodeVar61 = 0u;
					nodeVar61 = nodeVar58;
					nodeVar60 = nodeVar61;
					nodeVar59 = nodeVar60;
					nodeVar59 = ( nodeVar59 + u32( nodeVar55 ) );
					nodeVar60 = ( nodeVar60 + u32( nodeVar56 ) );
					nodeVar61 = ( nodeVar61 + u32( nodeVar57 ) );
					nodeVar62 = mx_bjmix( nodeVar59, nodeVar60, nodeVar61 );
					nodeVar63 = vec3<f32>( mx_bits_to_01( mx_bjfinal( nodeVar62.x, nodeVar62.y, nodeVar62.z ) ), mx_bits_to_01( mx_bjfinal( ( nodeVar62.x + 1u ), nodeVar62.y, nodeVar62.z ) ), mx_bits_to_01( mx_bjfinal( ( nodeVar62.x + 2u ), nodeVar62.y, nodeVar62.z ) ) );
					nodeVar63 = ( nodeVar63 - vec3<f32>( 0.5 ) );
					nodeVar63 = ( nodeVar63 * vec3<f32>( nodeVar42 ) );
					nodeVar63 = ( nodeVar63 + vec3<f32>( 0.5 ) );
					nodeVar64 = ( ( vec3<f32>( f32( x ), f32( y ), f32( z ) ) + nodeVar63 ) - nodeVar50 );

					if ( ( nodeVar53 < nodeVar51 ) ) {

						nodeVar51 = nodeVar53;
						nodeVar52 = nodeVar64;
						

					}

				}

			}

		}

		if ( ( nodeVar43 == 1 ) ) {

			nodeVar51 = mx_cell_noise_float_2( ( nodeVar52 + nodeVar41 ) );
			

		} else {

			nodeVar51 = sqrt( nodeVar51 );
			

		}

		nodeVar12 = ( ( ( ( render.nodeUniform14 * vec3<f32>( nodeVar36 ) ) * vec3<f32>( nodeVar5 ) ) * vec3<f32>( nodeVar37 ) ) * ( ( vec3<f32>( pow( nodeVar51, 2.0 ) ) * vec3<f32>( 0.10224173307914941, 0.5028864580233624, 0.6866853124288864 ) ) * vec3<f32>( 2.0 ) ) );

	} else {

		shadowPositionWorld = v_positionWorld;
		nodeVar36 = mix( 1.0, nodeVar20, render.nodeUniform20 );
		nodeVar36 = mix( 1.0, nodeVar20, render.nodeUniform20 );

		if ( ( render.nodeUniform21 > 0.0 ) ) {

			nodeVar66 = length( nodeVar9 );
			nodeVar67 = ( nodeVar66 / render.nodeUniform21 );
			nodeVar68 = clamp( ( 1.0 - ( ( ( nodeVar67 * nodeVar67 ) * nodeVar67 ) * nodeVar67 ) ), 0.0, 1.0 );
			nodeVar65 = ( ( 1.0 / max( pow( nodeVar66, render.nodeUniform22 ), 0.01 ) ) * ( nodeVar68 * nodeVar68 ) );

		} else {

			nodeVar65 = ( 1.0 / max( pow( length( nodeVar9 ), render.nodeUniform22 ), 0.01 ) );

		}

		nodeVar12 = ( ( ( render.nodeUniform14 * vec3<f32>( nodeVar36 ) ) * vec3<f32>( nodeVar5 ) ) * vec3<f32>( nodeVar65 ) );

	}

	nodeVar69 = ( vec3<f32>( clamp( nodeVar11, 0.0, 1.0 ) ) * nodeVar12 );
	nodeVar70 = ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) );
	nodeVar71 = ( nodeVar69 * nodeVar70 );
	nodeVar72 = ( directDiffuse + nodeVar71 );
	directDiffuse = nodeVar72;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar73 = ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) );
	nodeVar74 = ( vec4<f32>( irradiance, 1.0 ) * nodeVar73 );
	nodeVar75 = ( vec4<f32>( indirectDiffuse, 1.0 ) + nodeVar74 );
	indirectDiffuse = nodeVar75.xyz;
	ambientOcclusion = 1.0;
	nodeVar76 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar76;
	nodeVar77 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar77;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar78 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar78;
	nodeVar79 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar79;
	nodeVar80 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar80;

	// result

	output.color = nodeVar80;

	return output;

}
