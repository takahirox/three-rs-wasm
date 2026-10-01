// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms

struct renderStruct {
	nodeUniform1 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform2 : f32,
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform7 : mat3x3<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : i32;
var<private> nodeVar3 : i32;
var<private> nodeVar4 : i32;
var<private> nodeVar5 : i32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : i32;
var<private> nodeVar15 : i32;
var<private> nodeVar16 : i32;
var<private> nodeVar17 : u32;
var<private> nodeVar18 : u32;
var<private> nodeVar19 : u32;
var<private> nodeVar20 : u32;
var<private> nodeVar21 : vec3<u32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> Shininess : f32;
var<private> SpecularColor : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec4<f32>;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar41 : vec4<f32>;
var<private> nodeVar42 : vec4<f32>;
var<private> nodeVar43 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar44 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec4<f32>;

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
fn main( @location( 0 ) v_positionWorld : vec3<f32>,
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = ( ( v_positionWorld * vec3<f32>( 3.7 ) ) + vec3<f32>( render.nodeUniform1 ) );
	nodeVar1 = 1.0;
	nodeVar2 = 0;
	nodeVar3 = 0;
	nodeVar4 = 0;
	nodeVar5 = 0;
	nodeVar6 = nodeVar0.x;
	nodeVar3 = mx_floor( nodeVar6 );
	nodeVar7 = nodeVar0.y;
	nodeVar4 = mx_floor( nodeVar7 );
	nodeVar8 = nodeVar0.z;
	nodeVar5 = mx_floor( nodeVar8 );
	nodeVar9 = vec3<f32>( ( nodeVar6 - f32( nodeVar3 ) ), ( nodeVar7 - f32( nodeVar4 ) ), ( nodeVar8 - f32( nodeVar5 ) ) );
	nodeVar10 = 1000000.0;
	nodeVar11 = vec3<f32>( 0.0, 0.0, 0.0 );

	for ( var x : i32 = -1; x <= 1; x ++ ) {


		for ( var y : i32 = -1; y <= 1; y ++ ) {


			for ( var z : i32 = -1; z <= 1; z ++ ) {

				nodeVar12 = mx_worley_distance_1( nodeVar9, x, y, z, nodeVar3, nodeVar4, nodeVar5, nodeVar1, 0 );
				nodeVar13 = vec3<f32>( f32( ( nodeVar3 + x ) ), f32( ( nodeVar4 + y ) ), f32( ( nodeVar5 + z ) ) );
				nodeVar14 = i32( floor( nodeVar13.x ) );
				nodeVar15 = i32( floor( nodeVar13.y ) );
				nodeVar16 = i32( floor( nodeVar13.z ) );
				nodeVar17 = 3735928588u;
				nodeVar18 = 0u;
				nodeVar19 = 0u;
				nodeVar20 = 0u;
				nodeVar20 = nodeVar17;
				nodeVar19 = nodeVar20;
				nodeVar18 = nodeVar19;
				nodeVar18 = ( nodeVar18 + u32( nodeVar14 ) );
				nodeVar19 = ( nodeVar19 + u32( nodeVar15 ) );
				nodeVar20 = ( nodeVar20 + u32( nodeVar16 ) );
				nodeVar21 = mx_bjmix( nodeVar18, nodeVar19, nodeVar20 );
				nodeVar22 = vec3<f32>( mx_bits_to_01( mx_bjfinal( nodeVar21.x, nodeVar21.y, nodeVar21.z ) ), mx_bits_to_01( mx_bjfinal( ( nodeVar21.x + 1u ), nodeVar21.y, nodeVar21.z ) ), mx_bits_to_01( mx_bjfinal( ( nodeVar21.x + 2u ), nodeVar21.y, nodeVar21.z ) ) );
				nodeVar22 = ( nodeVar22 - vec3<f32>( 0.5 ) );
				nodeVar22 = ( nodeVar22 * vec3<f32>( nodeVar1 ) );
				nodeVar22 = ( nodeVar22 + vec3<f32>( 0.5 ) );
				nodeVar23 = ( ( vec3<f32>( f32( x ), f32( y ), f32( z ) ) + nodeVar22 ) - nodeVar9 );

				if ( ( nodeVar12 < nodeVar10 ) ) {

					nodeVar10 = nodeVar12;
					nodeVar11 = nodeVar23;
					

				}


			}


		}


	}


	if ( ( nodeVar2 == 1 ) ) {

		nodeVar10 = mx_cell_noise_float_2( ( nodeVar11 + nodeVar0 ) );
		

	} else {

		nodeVar10 = sqrt( nodeVar10 );
		

	}

	DiffuseColor = vec4<f32>( ( vec3<f32>( nodeVar10 ) + vec3<f32>( 0.02217388478862708, 0.9734452903978066, 1.0 ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * 1.0 );
	DiffuseColor.w = 1.0;
	Shininess = max( object.nodeUniform2, 0.0001 );
	SpecularColor = object.nodeUniform3;
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	nodeVar24 = ( render.nodeUniform9 - render.nodeUniform10 );
	nodeVar25 = vec4<f32>( nodeVar24, 0.0 );
	nodeVar26 = ( render.cameraViewMatrix * nodeVar25 );
	nodeVar27 = normalize( nodeVar26.xyz );
	nodeVar28 = nodeVar27;
	nodeVar29 = dot( normalView, nodeVar28 );
	nodeVar30 = ( vec3<f32>( clamp( nodeVar29, 0.0, 1.0 ) ) * render.nodeUniform11 );
	nodeVar31 = ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) );
	nodeVar32 = ( nodeVar30 * nodeVar31 );
	nodeVar33 = ( directDiffuse + nodeVar32 );
	directDiffuse = nodeVar33;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar34 = normalize( ( nodeVar28 + positionViewDirection ) );
	nodeVar35 = clamp( dot( positionViewDirection, nodeVar34 ), 0.0, 1.0 );
	nodeVar36 = exp2( ( ( ( nodeVar35 * -5.55473 ) - 6.98316 ) * nodeVar35 ) );
	nodeVar37 = ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar36 ) ) ) + vec3<f32>( ( 1.0 * nodeVar36 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeVar34 ), 0.0, 1.0 ), Shininess ) ) ) );
	nodeVar38 = ( nodeVar30 * nodeVar37 );
	nodeVar39 = ( nodeVar38 * vec3<f32>( 1.0 ) );
	nodeVar40 = ( directSpecular + nodeVar39 );
	directSpecular = nodeVar40;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar41 = ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) );
	nodeVar42 = ( vec4<f32>( irradiance, 1.0 ) * nodeVar41 );
	nodeVar43 = ( vec4<f32>( indirectDiffuse, 1.0 ) + nodeVar42 );
	indirectDiffuse = nodeVar43.xyz;
	ambientOcclusion = 1.0;
	nodeVar44 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar44;
	nodeVar45 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar45;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar46 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar46;
	nodeVar47 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar47;
	nodeVar48 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar48;

	// result

	output.color = nodeVar48;

	return output;

}
