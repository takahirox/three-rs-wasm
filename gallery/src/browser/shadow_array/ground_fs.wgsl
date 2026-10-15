// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform16_sampler : sampler_comparison;
@binding( 2 ) @group( 1 ) var nodeUniform16 : texture_depth_2d_array;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform8 : mat3x3<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform17 : f32,
	nodeUniform18 : mat4x4<f32>,
	nodeUniform19 : f32,
	nodeUniform20 : f32,
	nodeUniform21 : f32,
	nodeUniform22 : mat4x4<f32>,
	nodeUniform23 : f32,
	nodeUniform24 : f32,
	nodeUniform25 : f32,
	nodeUniform26 : mat4x4<f32>,
	nodeUniform27 : f32,
	nodeUniform28 : f32,
	nodeUniform29 : f32,
	nodeUniform30 : vec3<f32>,
	nodeUniform31 : f32,
	nodeUniform32 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Shininess : f32;
var<private> SpecularColor : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar7 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> shadowValue : f32;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar42 : vec4<f32>;
var<private> nodeVar43 : vec4<f32>;
var<private> nodeVar44 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar45 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec4<f32>;

// codes
fn mx_rotl32 ( x : u32, k : i32 ) -> u32 {

	var nodeVar0 : i32;
	var nodeVar1 : u32;

	nodeVar0 = k;
	nodeVar1 = x;

	return ( ( nodeVar1 << u32( nodeVar0 ) ) | ( nodeVar1 >> u32( ( 32 - nodeVar0 ) ) ) );

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


fn mx_select ( b : bool, t : f32, f : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : bool;
	var nodeVar3 : f32;

	nodeVar0 = f;
	nodeVar1 = t;
	nodeVar2 = b;

	return select( nodeVar0, nodeVar1, nodeVar2 );

}


fn mx_negate_if ( val : f32, b : bool ) -> f32 {

	var nodeVar0 : bool;
	var nodeVar1 : f32;
	var nodeVar2 : f32;

	nodeVar0 = b;
	nodeVar1 = val;

	return select( nodeVar1, ( - nodeVar1 ), nodeVar0 );

}


fn mx_gradient_float_1 ( hash : u32, x : f32, y : f32, z : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : u32;
	var nodeVar4 : u32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;

	nodeVar0 = z;
	nodeVar1 = y;
	nodeVar2 = x;
	nodeVar3 = hash;
	nodeVar4 = ( nodeVar3 & 15u );
	nodeVar5 = mx_select( ( nodeVar4 < 8u ), nodeVar2, nodeVar1 );
	nodeVar6 = mx_select( ( nodeVar4 < 4u ), nodeVar1, mx_select( ( ( nodeVar4 == 12u ) || ( nodeVar4 == 14u ) ), nodeVar2, nodeVar0 ) );

	return ( mx_negate_if( nodeVar5, bool( ( nodeVar4 & 1u ) ) ) + mx_negate_if( nodeVar6, bool( ( nodeVar4 & 2u ) ) ) );

}


fn mx_floor ( x : f32 ) -> i32 {

	var nodeVar0 : f32;

	nodeVar0 = x;

	return i32( floor( nodeVar0 ) );

}


fn mx_fade ( t : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = t;

	return ( ( ( nodeVar0 * nodeVar0 ) * nodeVar0 ) * ( ( nodeVar0 * ( ( nodeVar0 * 6.0 ) - 15.0 ) ) + 10.0 ) );

}


fn mx_hash_vec3_1 ( x : i32, y : i32, z : i32 ) -> vec3<u32> {

	var nodeVar0 : i32;
	var nodeVar1 : i32;
	var nodeVar2 : i32;
	var nodeVar3 : u32;
	var nodeVar4 : vec3<u32>;

	nodeVar0 = z;
	nodeVar1 = y;
	nodeVar2 = x;
	nodeVar3 = mx_hash_int_2( nodeVar2, nodeVar1, nodeVar0 );
	nodeVar4 = vec3<u32>( 0u, 0u, 0u );
	nodeVar4.x = ( nodeVar3 & 255u );
	nodeVar4.y = ( ( nodeVar3 >> 8u ) & 255u );
	nodeVar4.z = ( ( nodeVar3 >> 16u ) & 255u );

	return nodeVar4;

}


fn mx_gradient_vec3_1 ( hash : vec3<u32>, x : f32, y : f32, z : f32 ) -> vec3<f32> {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : vec3<u32>;

	nodeVar0 = z;
	nodeVar1 = y;
	nodeVar2 = x;
	nodeVar3 = hash;

	return vec3<f32>( mx_gradient_float_1( nodeVar3.x, nodeVar2, nodeVar1, nodeVar0 ), mx_gradient_float_1( nodeVar3.y, nodeVar2, nodeVar1, nodeVar0 ), mx_gradient_float_1( nodeVar3.z, nodeVar2, nodeVar1, nodeVar0 ) );

}


fn mx_trilerp_1 ( v0 : vec3<f32>, v1 : vec3<f32>, v2 : vec3<f32>, v3 : vec3<f32>, v4 : vec3<f32>, v5 : vec3<f32>, v6 : vec3<f32>, v7 : vec3<f32>, s : f32, t : f32, r : f32 ) -> vec3<f32> {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : vec3<f32>;
	var nodeVar4 : vec3<f32>;
	var nodeVar5 : vec3<f32>;
	var nodeVar6 : vec3<f32>;
	var nodeVar7 : vec3<f32>;
	var nodeVar8 : vec3<f32>;
	var nodeVar9 : vec3<f32>;
	var nodeVar10 : vec3<f32>;
	var nodeVar11 : f32;
	var nodeVar12 : f32;
	var nodeVar13 : f32;

	nodeVar0 = r;
	nodeVar1 = t;
	nodeVar2 = s;
	nodeVar3 = v7;
	nodeVar4 = v6;
	nodeVar5 = v5;
	nodeVar6 = v4;
	nodeVar7 = v3;
	nodeVar8 = v2;
	nodeVar9 = v1;
	nodeVar10 = v0;
	nodeVar11 = ( 1.0 - nodeVar2 );
	nodeVar12 = ( 1.0 - nodeVar1 );
	nodeVar13 = ( 1.0 - nodeVar0 );

	return ( ( vec3<f32>( nodeVar13 ) * ( ( vec3<f32>( nodeVar12 ) * ( ( nodeVar10 * vec3<f32>( nodeVar11 ) ) + ( nodeVar9 * vec3<f32>( nodeVar2 ) ) ) ) + ( vec3<f32>( nodeVar1 ) * ( ( nodeVar8 * vec3<f32>( nodeVar11 ) ) + ( nodeVar7 * vec3<f32>( nodeVar2 ) ) ) ) ) ) + ( vec3<f32>( nodeVar0 ) * ( ( vec3<f32>( nodeVar12 ) * ( ( nodeVar6 * vec3<f32>( nodeVar11 ) ) + ( nodeVar5 * vec3<f32>( nodeVar2 ) ) ) ) + ( vec3<f32>( nodeVar1 ) * ( ( nodeVar4 * vec3<f32>( nodeVar11 ) ) + ( nodeVar3 * vec3<f32>( nodeVar2 ) ) ) ) ) ) );

}


fn mx_gradient_scale3d_1 ( v : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;

	nodeVar0 = v;

	return ( vec3<f32>( 0.982 ) * nodeVar0 );

}


fn mx_perlin_noise_vec3_1 ( p : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : i32;
	var nodeVar2 : i32;
	var nodeVar3 : i32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;
	var nodeVar7 : f32;
	var nodeVar8 : f32;
	var nodeVar9 : f32;
	var nodeVar10 : f32;
	var nodeVar11 : f32;
	var nodeVar12 : f32;
	var nodeVar13 : vec3<f32>;

	nodeVar0 = p;
	nodeVar1 = 0;
	nodeVar2 = 0;
	nodeVar3 = 0;
	nodeVar4 = nodeVar0.x;
	nodeVar1 = mx_floor( nodeVar4 );
	nodeVar5 = ( nodeVar4 - f32( nodeVar1 ) );
	nodeVar6 = nodeVar0.y;
	nodeVar2 = mx_floor( nodeVar6 );
	nodeVar7 = ( nodeVar6 - f32( nodeVar2 ) );
	nodeVar8 = nodeVar0.z;
	nodeVar3 = mx_floor( nodeVar8 );
	nodeVar9 = ( nodeVar8 - f32( nodeVar3 ) );
	nodeVar10 = mx_fade( nodeVar5 );
	nodeVar11 = mx_fade( nodeVar7 );
	nodeVar12 = mx_fade( nodeVar9 );
	nodeVar13 = mx_trilerp_1( mx_gradient_vec3_1( mx_hash_vec3_1( nodeVar1, nodeVar2, nodeVar3 ), nodeVar5, nodeVar7, nodeVar9 ), mx_gradient_vec3_1( mx_hash_vec3_1( ( nodeVar1 + 1 ), nodeVar2, nodeVar3 ), ( nodeVar5 - 1.0 ), nodeVar7, nodeVar9 ), mx_gradient_vec3_1( mx_hash_vec3_1( nodeVar1, ( nodeVar2 + 1 ), nodeVar3 ), nodeVar5, ( nodeVar7 - 1.0 ), nodeVar9 ), mx_gradient_vec3_1( mx_hash_vec3_1( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ), nodeVar3 ), ( nodeVar5 - 1.0 ), ( nodeVar7 - 1.0 ), nodeVar9 ), mx_gradient_vec3_1( mx_hash_vec3_1( nodeVar1, nodeVar2, ( nodeVar3 + 1 ) ), nodeVar5, nodeVar7, ( nodeVar9 - 1.0 ) ), mx_gradient_vec3_1( mx_hash_vec3_1( ( nodeVar1 + 1 ), nodeVar2, ( nodeVar3 + 1 ) ), ( nodeVar5 - 1.0 ), nodeVar7, ( nodeVar9 - 1.0 ) ), mx_gradient_vec3_1( mx_hash_vec3_1( nodeVar1, ( nodeVar2 + 1 ), ( nodeVar3 + 1 ) ), nodeVar5, ( nodeVar7 - 1.0 ), ( nodeVar9 - 1.0 ) ), mx_gradient_vec3_1( mx_hash_vec3_1( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ), ( nodeVar3 + 1 ) ), ( nodeVar5 - 1.0 ), ( nodeVar7 - 1.0 ), ( nodeVar9 - 1.0 ) ), nodeVar10, nodeVar11, nodeVar12 );

	return mx_gradient_scale3d_1( nodeVar13 );

}


fn mx_fractal_noise_vec3 ( p : vec3<f32>, octaves : i32, lacunarity : f32, diminish : f32 ) -> vec3<f32> {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : vec3<f32>;
	var nodeVar3 : vec3<f32>;
	var nodeVar4 : f32;
	var nodeVar5 : i32;

	nodeVar0 = diminish;
	nodeVar1 = lacunarity;
	nodeVar2 = p;
	nodeVar3 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar4 = 1.0;
	nodeVar5 = octaves;

	for ( var i : i32 = 0; i < nodeVar5; i ++ ) {

		nodeVar3 = ( nodeVar3 + ( vec3<f32>( nodeVar4 ) * mx_perlin_noise_vec3_1( nodeVar2 ) ) );
		nodeVar4 = ( nodeVar4 * nodeVar0 );
		nodeVar2 = ( nodeVar2 * vec3<f32>( nodeVar1 ) );

	}


	return nodeVar3;

}




@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_normalViewGeometry : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( mix( vec3<f32>( 0.4, 0.7, 0.3 ), vec3<f32>( 0.6, 0.5, 0.3 ), clamp( ( mx_fractal_noise_vec3( ( v_positionWorld * vec3<f32>( 0.05 ) ), 3, 2.0, 0.5 ) * vec3<f32>( 1.0 ) ), vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) ).x ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Shininess = max( object.nodeUniform2, 0.0001 );
	SpecularColor = object.nodeUniform3;
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar0 = ( irradiance + render.nodeUniform6 );
	irradiance = nodeVar0;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	nodeVar1 = ( render.nodeUniform10 - render.nodeUniform11 );
	nodeVar2 = vec4<f32>( nodeVar1, 0.0 );
	nodeVar3 = ( render.cameraViewMatrix * nodeVar2 );
	nodeVar4 = normalize( nodeVar3.xyz );
	nodeVar5 = nodeVar4;
	nodeVar6 = dot( normalView, nodeVar5 );
	shadowPositionWorld = v_positionWorld;
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar8 = ( render.nodeUniform13 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform14 ) ) ), 1.0 ) );
	nodeVar9 = ( nodeVar8.xyz / vec3<f32>( nodeVar8.w ) );
	nodeVar10 = vec3<f32>( nodeVar9.x, ( 1.0 - nodeVar9.y ), ( nodeVar9.z + render.nodeUniform15 ) );

	if ( ( ( ( ( ( nodeVar10.x >= 0.0 ) && ( nodeVar10.x <= 1.0 ) ) && ( nodeVar10.y >= 0.0 ) ) && ( nodeVar10.y <= 1.0 ) ) && ( nodeVar10.z <= 1.0 ) ) ) {

		nodeVar11 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar10.xy, 0, nodeVar10.z );
		nodeVar7 = nodeVar11;

	} else {

		nodeVar7 = 1.0;

	}

	nodeVar12 = mix( 1.0, nodeVar7, render.nodeUniform17 );
	shadowPositionWorld = v_positionWorld;
	nodeVar14 = ( render.nodeUniform18 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform19 ) ) ), 1.0 ) );
	nodeVar15 = ( nodeVar14.xyz / vec3<f32>( nodeVar14.w ) );
	nodeVar16 = vec3<f32>( nodeVar15.x, ( 1.0 - nodeVar15.y ), ( nodeVar15.z + render.nodeUniform20 ) );

	if ( ( ( ( ( ( nodeVar16.x >= 0.0 ) && ( nodeVar16.x <= 1.0 ) ) && ( nodeVar16.y >= 0.0 ) ) && ( nodeVar16.y <= 1.0 ) ) && ( nodeVar16.z <= 1.0 ) ) ) {

		nodeVar17 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar16.xy, 1, nodeVar16.z );
		nodeVar13 = nodeVar17;

	} else {

		nodeVar13 = 1.0;

	}

	nodeVar18 = mix( 1.0, nodeVar13, render.nodeUniform21 );
	shadowPositionWorld = v_positionWorld;
	nodeVar20 = ( render.nodeUniform22 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform23 ) ) ), 1.0 ) );
	nodeVar21 = ( nodeVar20.xyz / vec3<f32>( nodeVar20.w ) );
	nodeVar22 = vec3<f32>( nodeVar21.x, ( 1.0 - nodeVar21.y ), ( nodeVar21.z + render.nodeUniform24 ) );

	if ( ( ( ( ( ( nodeVar22.x >= 0.0 ) && ( nodeVar22.x <= 1.0 ) ) && ( nodeVar22.y >= 0.0 ) ) && ( nodeVar22.y <= 1.0 ) ) && ( nodeVar22.z <= 1.0 ) ) ) {

		nodeVar23 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar22.xy, 2, nodeVar22.z );
		nodeVar19 = nodeVar23;

	} else {

		nodeVar19 = 1.0;

	}

	nodeVar24 = mix( 1.0, nodeVar19, render.nodeUniform25 );
	shadowPositionWorld = v_positionWorld;
	nodeVar26 = ( render.nodeUniform26 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform27 ) ) ), 1.0 ) );
	nodeVar27 = ( nodeVar26.xyz / vec3<f32>( nodeVar26.w ) );
	nodeVar28 = vec3<f32>( nodeVar27.x, ( 1.0 - nodeVar27.y ), ( nodeVar27.z + render.nodeUniform28 ) );

	if ( ( ( ( ( ( nodeVar28.x >= 0.0 ) && ( nodeVar28.x <= 1.0 ) ) && ( nodeVar28.y >= 0.0 ) ) && ( nodeVar28.y <= 1.0 ) ) && ( nodeVar28.z <= 1.0 ) ) ) {

		nodeVar29 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar28.xy, 3, nodeVar28.z );
		nodeVar25 = nodeVar29;

	} else {

		nodeVar25 = 1.0;

	}

	nodeVar30 = mix( 1.0, nodeVar25, render.nodeUniform29 );
	shadowValue = min( min( min( nodeVar12, nodeVar18 ), nodeVar24 ), nodeVar30 );
	nodeVar31 = ( vec3<f32>( clamp( nodeVar6, 0.0, 1.0 ) ) * ( render.nodeUniform12 * vec3<f32>( shadowValue ) ) );
	nodeVar32 = ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) );
	nodeVar33 = ( nodeVar31 * nodeVar32 );
	nodeVar34 = ( directDiffuse + nodeVar33 );
	directDiffuse = nodeVar34;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar35 = normalize( ( nodeVar5 + positionViewDirection ) );
	nodeVar36 = clamp( dot( positionViewDirection, nodeVar35 ), 0.0, 1.0 );
	nodeVar37 = exp2( ( ( ( nodeVar36 * -5.55473 ) - 6.98316 ) * nodeVar36 ) );
	nodeVar38 = ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar37 ) ) ) + vec3<f32>( ( 1.0 * nodeVar37 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeVar35 ), 0.0, 1.0 ), Shininess ) ) ) );
	nodeVar39 = ( nodeVar31 * nodeVar38 );
	nodeVar40 = ( nodeVar39 * vec3<f32>( 1.0 ) );
	nodeVar41 = ( directSpecular + nodeVar40 );
	directSpecular = nodeVar41;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar42 = ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) );
	nodeVar43 = ( vec4<f32>( irradiance, 1.0 ) * nodeVar42 );
	nodeVar44 = ( vec4<f32>( indirectDiffuse, 1.0 ) + nodeVar43 );
	indirectDiffuse = nodeVar44.xyz;
	ambientOcclusion = 1.0;
	nodeVar45 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar45;
	nodeVar46 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar46;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar47 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar47;
	nodeVar48 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar48;
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar49 = vec4<f32>( mix( Output.xyz, render.nodeUniform30, smoothstep( render.nodeUniform31, render.nodeUniform32, ( - v_positionView.z ) ) ), Output.w );
	Output = nodeVar49;

	// result

	output.color = nodeVar49;

	return output;

}
