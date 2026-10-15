// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform41_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform41 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform47_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform47 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : i32,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform8 : f32,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform11 : f32,
	nodeUniform12 : f32,
	nodeUniform13 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : f32,
	nodeUniform16 : f32,
	nodeUniform17 : f32,
	nodeUniform18 : f32,
	nodeUniform19 : f32,
	nodeUniform20 : f32,
	nodeUniform21 : f32,
	nodeUniform22 : f32,
	nodeUniform23 : f32,
	nodeUniform24 : f32,
	nodeUniform25 : f32,
	nodeUniform26 : f32,
	nodeUniform27 : f32,
	nodeUniform28 : f32,
	nodeUniform44 : mat3x3<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform5 : u32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform29 : f32,
	nodeUniform32 : f32,
	nodeUniform34 : f32,
	nodeUniform35 : f32,
	nodeUniform39 : f32,
	nodeUniform40 : f32,
	nodeUniform33 : vec3<f32>,
	nodeUniform30 : vec3<f32>,
	nodeUniform36 : vec3<f32>,
	nodeUniform37 : vec3<f32>,
	nodeUniform38 : vec3<f32>,
	nodeUniform42 : mat4x4<f32>,
	nodeUniform45 : f32,
	nodeUniform46 : f32,
	nodeUniform50 : f32,
	cameraPosition : vec3<f32>,
	nodeUniform48 : f32,
	nodeUniform49 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : bool;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : bool;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : vec2<f32>;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar46 : vec4<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec4<f32>;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : vec2<f32>;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : vec2<f32>;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : vec2<f32>;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : vec2<f32>;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : vec2<f32>;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : vec4<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec4<f32>;

// codes
fn interleavedGradientNoise ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

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


fn mx_trilerp_0 ( v0 : f32, v1 : f32, v2 : f32, v3 : f32, v4 : f32, v5 : f32, v6 : f32, v7 : f32, s : f32, t : f32, r : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;
	var nodeVar7 : f32;
	var nodeVar8 : f32;
	var nodeVar9 : f32;
	var nodeVar10 : f32;
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

	return ( ( nodeVar13 * ( ( nodeVar12 * ( ( nodeVar10 * nodeVar11 ) + ( nodeVar9 * nodeVar2 ) ) ) + ( nodeVar1 * ( ( nodeVar8 * nodeVar11 ) + ( nodeVar7 * nodeVar2 ) ) ) ) ) + ( nodeVar0 * ( ( nodeVar12 * ( ( nodeVar6 * nodeVar11 ) + ( nodeVar5 * nodeVar2 ) ) ) + ( nodeVar1 * ( ( nodeVar4 * nodeVar11 ) + ( nodeVar3 * nodeVar2 ) ) ) ) ) );

}


fn mx_gradient_scale3d_0 ( v : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = v;

	return ( 0.982 * nodeVar0 );

}


fn mx_perlin_noise_float_1 ( p : vec3<f32> ) -> f32 {

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
	var nodeVar13 : f32;

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
	nodeVar13 = mx_trilerp_0( mx_gradient_float_1( mx_hash_int_2( nodeVar1, nodeVar2, nodeVar3 ), nodeVar5, nodeVar7, nodeVar9 ), mx_gradient_float_1( mx_hash_int_2( ( nodeVar1 + 1 ), nodeVar2, nodeVar3 ), ( nodeVar5 - 1.0 ), nodeVar7, nodeVar9 ), mx_gradient_float_1( mx_hash_int_2( nodeVar1, ( nodeVar2 + 1 ), nodeVar3 ), nodeVar5, ( nodeVar7 - 1.0 ), nodeVar9 ), mx_gradient_float_1( mx_hash_int_2( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ), nodeVar3 ), ( nodeVar5 - 1.0 ), ( nodeVar7 - 1.0 ), nodeVar9 ), mx_gradient_float_1( mx_hash_int_2( nodeVar1, nodeVar2, ( nodeVar3 + 1 ) ), nodeVar5, nodeVar7, ( nodeVar9 - 1.0 ) ), mx_gradient_float_1( mx_hash_int_2( ( nodeVar1 + 1 ), nodeVar2, ( nodeVar3 + 1 ) ), ( nodeVar5 - 1.0 ), nodeVar7, ( nodeVar9 - 1.0 ) ), mx_gradient_float_1( mx_hash_int_2( nodeVar1, ( nodeVar2 + 1 ), ( nodeVar3 + 1 ) ), nodeVar5, ( nodeVar7 - 1.0 ), ( nodeVar9 - 1.0 ) ), mx_gradient_float_1( mx_hash_int_2( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ), ( nodeVar3 + 1 ) ), ( nodeVar5 - 1.0 ), ( nodeVar7 - 1.0 ), ( nodeVar9 - 1.0 ) ), nodeVar10, nodeVar11, nodeVar12 );

	return mx_gradient_scale3d_0( nodeVar13 );

}


fn vogelDiskSample ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	var nodeVar0 : f32;

	nodeVar0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeVar0 ), sin( nodeVar0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

}




@fragment
fn main( @location( 0 ) v_positionWorld : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	nodeVar0 = ( render.cameraPosition - v_positionWorld );
	nodeVar1 = length( nodeVar0 );
	nodeVar2 = ( object.nodeUniform3 * 2.0 );
	nodeVar3 = ( nodeVar1 > nodeVar2 );

	if ( nodeVar3 ) {

		nodeVar4 = render.cameraPosition;
		nodeVar5 = v_positionWorld;
		nodeVar6 = true;
		

	} else {

		nodeVar4 = v_positionWorld;
		nodeVar5 = render.cameraPosition;
		nodeVar6 = false;
		

	}

	nodeVar7 = ( nodeVar5 - nodeVar4 );
	nodeVar8 = length( nodeVar7 );
	nodeVar9 = ( nodeVar8 / f32( object.nodeUniform4 ) );
	nodeVar10 = nodeVar9;
	nodeVar11 = normalize( nodeVar7 );
	nodeVar12 = nodeVar11;
	nodeVar13 = 0.0;
	nodeVar14 = vec3<f32>( 1.0, 1.0, 1.0 );
	nodeVar15 = ( fract( ( interleavedGradientNoise( fragCoord.xy ) + ( f32( render.nodeUniform5 ) * 0.618033988749895 ) ) ) * nodeVar10 );
	nodeVar16 = ( nodeVar13 + nodeVar15 );
	nodeVar13 = nodeVar16;

	for ( var i : i32 = 0; i < object.nodeUniform4; i ++ ) {

		nodeVar17 = vec3<f32>( 0.0 );

		if ( ( 1.0 == 1.0 ) ) {

			nodeVar19 = vec3<f32>( 0.0, 0.0, 0.0 );
			nodeVar20 = clamp( ( ( ( ( object.nodeUniform7 / 8.34 ) * 0.5 ) + 0.2 ) + object.nodeUniform8 ), 0.0, 1.0 );
			nodeVar19 = mix( vec3<f32>( 0.0, 0.0, 0.0 ), object.nodeUniform6, smoothstep( 0.05, 0.35, nodeVar20 ) );
			nodeVar19 = mix( nodeVar19, object.nodeUniform9, smoothstep( 0.35, 0.65, nodeVar20 ) );
			nodeVar19 = mix( nodeVar19, object.nodeUniform10, smoothstep( 0.65, 1.0, nodeVar20 ) );
			nodeVar21 = cos( object.nodeUniform12 );
			nodeVar18 = max( ( ( max( mix( vec3<f32>( dot( nodeVar19, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), nodeVar19, object.nodeUniform11 ), vec3<f32>( 0.0 ) ) * vec3<f32>( nodeVar21 ) ) + ( ( cross( vec3<f32>( 0.57735, 0.57735, 0.57735 ), max( mix( vec3<f32>( dot( nodeVar19, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), nodeVar19, object.nodeUniform11 ), vec3<f32>( 0.0 ) ) ) * vec3<f32>( sin( object.nodeUniform12 ) ) ) + ( vec3<f32>( 0.57735, 0.57735, 0.57735 ) * vec3<f32>( ( dot( vec3<f32>( 0.57735, 0.57735, 0.57735 ), max( mix( vec3<f32>( dot( nodeVar19, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), nodeVar19, object.nodeUniform11 ), vec3<f32>( 0.0 ) ) ) * ( 1.0 - nodeVar21 ) ) ) ) ) ), vec3<f32>( 0.0 ) );

		} else {

			nodeVar22 = vec3<f32>( 0.0, 0.0, 0.0 );
			nodeVar23 = vec3<f32>( 0.0, object.nodeUniform15, 0.0 );
			nodeVar24 = length( ( v_positionWorld - ( ( object.nodeUniform13 + object.nodeUniform14 ) + ( nodeVar23 * vec3<f32>( clamp( ( dot( ( v_positionWorld - object.nodeUniform13 ), nodeVar23 ) / dot( nodeVar23, nodeVar23 ) ), 0.0, 1.0 ) ) ) ) ) );
			nodeVar25 = ( v_positionWorld.xz - object.nodeUniform13.xz );
			nodeVar26 = atan2( nodeVar25.y, nodeVar25.x );
			nodeVar27 = clamp( ( clamp( ( 1.0 - ( nodeVar24 / object.nodeUniform16 ) ), 0.0, 1.0 ) * mix( ( ( ( ( ( ( ( mx_perlin_noise_float_1( vec3<f32>( ( v_positionWorld.x * ( 0.6 * object.nodeUniform17 ) ), ( object.nodeUniform18 * 1.2 ), ( v_positionWorld.z * ( 0.6 * object.nodeUniform17 ) ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) * 0.65 ) + ( ( ( ( ( mx_perlin_noise_float_1( vec3<f32>( ( v_positionWorld.x * ( 1.5 * object.nodeUniform17 ) ), ( object.nodeUniform18 * 2.5 ), ( v_positionWorld.z * ( 1.5 * object.nodeUniform17 ) ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) * 0.35 ) ) * ( ( mix( 1.0, ( ( ( ( mx_perlin_noise_float_1( vec3<f32>( ( cos( nodeVar26 ) * ( 1.5 * object.nodeUniform17 ) ), ( sin( nodeVar26 ) * ( 1.5 * object.nodeUniform17 ) ), ( object.nodeUniform18 * 0.6 ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ), smoothstep( 0.0, object.nodeUniform19, length( nodeVar25 ) ) ) * 0.5 ) + 0.5 ) ), 1.0, clamp( ( nodeVar24 / object.nodeUniform20 ), 0.0, 1.0 ) ) ), 0.0, 1.0 );
			nodeVar22 = mix( vec3<f32>( 0.0, 0.0, 0.0 ), object.nodeUniform6, smoothstep( 0.05, 0.35, nodeVar27 ) );
			nodeVar22 = mix( nodeVar22, object.nodeUniform9, smoothstep( 0.35, 0.65, nodeVar27 ) );
			nodeVar22 = mix( nodeVar22, object.nodeUniform10, smoothstep( 0.65, 1.0, nodeVar27 ) );
			nodeVar28 = cos( object.nodeUniform12 );
			nodeVar18 = max( ( ( max( mix( vec3<f32>( dot( nodeVar22, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), nodeVar22, object.nodeUniform11 ), vec3<f32>( 0.0 ) ) * vec3<f32>( nodeVar28 ) ) + ( ( cross( vec3<f32>( 0.57735, 0.57735, 0.57735 ), max( mix( vec3<f32>( dot( nodeVar22, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), nodeVar22, object.nodeUniform11 ), vec3<f32>( 0.0 ) ) ) * vec3<f32>( sin( object.nodeUniform12 ) ) ) + ( vec3<f32>( 0.57735, 0.57735, 0.57735 ) * vec3<f32>( ( dot( vec3<f32>( 0.57735, 0.57735, 0.57735 ), max( mix( vec3<f32>( dot( nodeVar22, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ), nodeVar22, object.nodeUniform11 ), vec3<f32>( 0.0 ) ) ) * ( 1.0 - nodeVar28 ) ) ) ) ) ), vec3<f32>( 0.0 ) );

		}


		if ( ( 1.0 == 1.0 ) ) {

			nodeVar29 = object.nodeUniform23;

		} else {

			nodeVar29 = object.nodeUniform24;

		}


		if ( ( 1.0 == 1.0 ) ) {

			nodeVar31 = vec3<f32>( 0.0, object.nodeUniform15, 0.0 );
			nodeVar32 = length( ( v_positionWorld - ( ( object.nodeUniform13 + object.nodeUniform14 ) + ( nodeVar31 * vec3<f32>( clamp( ( dot( ( v_positionWorld - object.nodeUniform13 ), nodeVar31 ) / dot( nodeVar31, nodeVar31 ) ), 0.0, 1.0 ) ) ) ) ) );
			nodeVar30 = ( ( 1.0 / ( pow( nodeVar32, 2.0 ) + pow( 1.2, 2.0 ) ) ) / ( 1.0 / max( pow( length( ( v_positionWorld - object.nodeUniform13 ) ), 2.0 ), 0.01 ) ) );

		} else {

			nodeVar30 = 1.0;

		}


		if ( ( 1.0 == 1.0 ) ) {

			nodeVar33 = mix( object.nodeUniform26, object.nodeUniform27, smoothstep( 0.0, 1.0, clamp( ( nodeVar32 / object.nodeUniform28 ), 0.0, 1.0 ) ) );

		} else {

			nodeVar33 = 1.0;

		}


		if ( ( render.nodeUniform29 > 0.0 ) ) {

			nodeVar35 = length( ( render.nodeUniform30 - ( render.cameraViewMatrix * vec4<f32>( ( nodeVar4 + ( nodeVar12 * vec3<f32>( nodeVar13 ) ) ), 1.0 ) ).xyz ) );
			nodeVar36 = ( nodeVar35 / render.nodeUniform29 );
			nodeVar37 = clamp( ( 1.0 - ( ( ( nodeVar36 * nodeVar36 ) * nodeVar36 ) * nodeVar36 ) ), 0.0, 1.0 );
			nodeVar34 = ( ( 1.0 / max( pow( nodeVar35, render.nodeUniform32 ), 0.01 ) ) * ( nodeVar37 * nodeVar37 ) );

		} else {

			nodeVar34 = ( 1.0 / max( pow( length( ( render.nodeUniform30 - ( render.cameraViewMatrix * vec4<f32>( ( nodeVar4 + ( nodeVar12 * vec3<f32>( nodeVar13 ) ) ), 1.0 ) ).xyz ) ), render.nodeUniform32 ), 0.01 ) );

		}

		nodeVar38 = ( ( ( ( ( ( ( ( ( nodeVar18 * vec3<f32>( pow( max( ( object.nodeUniform7 / 8.34 ), 0.0 ), 4.0 ) ) ) * vec3<f32>( max( ( object.nodeUniform21 / 11.02 ), 0.0 ) ) ) * vec3<f32>( object.nodeUniform22 ) ) * vec3<f32>( nodeVar29 ) ) * vec3<f32>( object.nodeUniform25 ) ) * vec3<f32>( smoothstep( 0.0, 3.0, object.nodeUniform18 ) ) ) * vec3<f32>( nodeVar30 ) ) * vec3<f32>( nodeVar33 ) ) * vec3<f32>( nodeVar34 ) );
		nodeVar17 = ( nodeVar17 + nodeVar38 );
		nodeVar39 = ( nodeVar4 + ( nodeVar12 * vec3<f32>( nodeVar13 ) ) );
		nodeVar40 = ( render.nodeUniform36 - ( render.cameraViewMatrix * vec4<f32>( nodeVar39, 1.0 ) ).xyz );

		if ( ( render.nodeUniform39 > 0.0 ) ) {

			nodeVar42 = length( nodeVar40 );
			nodeVar43 = ( nodeVar42 / render.nodeUniform39 );
			nodeVar44 = clamp( ( 1.0 - ( ( ( nodeVar43 * nodeVar43 ) * nodeVar43 ) * nodeVar43 ) ), 0.0, 1.0 );
			nodeVar41 = ( ( 1.0 / max( pow( nodeVar42, render.nodeUniform40 ), 0.01 ) ) * ( nodeVar44 * nodeVar44 ) );

		} else {

			nodeVar41 = ( 1.0 / max( pow( length( nodeVar40 ), render.nodeUniform40 ), 0.01 ) );

		}

		nodeVar45 = ( ( render.nodeUniform33 * vec3<f32>( smoothstep( render.nodeUniform34, render.nodeUniform35, dot( normalize( nodeVar40 ), normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform37 - render.nodeUniform38 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( nodeVar41 ) );
		shadowPositionWorld = nodeVar39;
		normalViewGeometry = normalize( v_normalViewGeometry );
		NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
		normalView = NORMAL_normalView;
		normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
		nodeVar46 = ( render.nodeUniform42 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform45 ) ) ), 1.0 ) );
		nodeVar47 = ( nodeVar46.xyz / vec3<f32>( nodeVar46.w ) );
		nodeVar48 = vec3<f32>( nodeVar47.x, ( 1.0 - nodeVar47.y ), ( nodeVar47.z + render.nodeUniform46 ) );
		nodeVar49 = textureSample( nodeUniform41, nodeUniform41_sampler, nodeVar48.xy );

		if ( ( ( ( ( ( nodeVar48.x >= 0.0 ) && ( nodeVar48.x <= 1.0 ) ) && ( nodeVar48.y >= 0.0 ) ) && ( nodeVar48.y <= 1.0 ) ) && ( nodeVar48.z <= 1.0 ) ) ) {

			nodeVar51 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar52 = ( render.nodeUniform48 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform49 ).x );
			nodeVar53 = ( nodeVar48.xy + ( vogelDiskSample( 0, 5, nodeVar51 ) * vec2<f32>( nodeVar52 ) ) );
			nodeVar54 = textureSampleCompare( nodeUniform47, nodeUniform47_sampler, nodeVar53, nodeVar48.z );
			nodeVar55 = ( nodeVar48.xy + ( vogelDiskSample( 1, 5, nodeVar51 ) * vec2<f32>( nodeVar52 ) ) );
			nodeVar56 = textureSampleCompare( nodeUniform47, nodeUniform47_sampler, nodeVar55, nodeVar48.z );
			nodeVar57 = ( nodeVar48.xy + ( vogelDiskSample( 2, 5, nodeVar51 ) * vec2<f32>( nodeVar52 ) ) );
			nodeVar58 = textureSampleCompare( nodeUniform47, nodeUniform47_sampler, nodeVar57, nodeVar48.z );
			nodeVar59 = ( nodeVar48.xy + ( vogelDiskSample( 3, 5, nodeVar51 ) * vec2<f32>( nodeVar52 ) ) );
			nodeVar60 = textureSampleCompare( nodeUniform47, nodeUniform47_sampler, nodeVar59, nodeVar48.z );
			nodeVar61 = ( nodeVar48.xy + ( vogelDiskSample( 4, 5, nodeVar51 ) * vec2<f32>( nodeVar52 ) ) );
			nodeVar62 = textureSampleCompare( nodeUniform47, nodeUniform47_sampler, nodeVar61, nodeVar48.z );
			nodeVar50 = ( ( ( ( ( nodeVar54 + nodeVar56 ) + nodeVar58 ) + nodeVar60 ) + nodeVar62 ) * 0.2 );

		} else {

			nodeVar50 = 1.0;

		}

		nodeVar63 = mix( vec4<f32>( 1.0 ), mix( nodeVar49, vec4<f32>( 1.0 ), vec4<f32>( nodeVar50 ) ), ( render.nodeUniform50 * nodeVar49.w ) );
		nodeVar45 = ( vec4<f32>( nodeVar45, 1.0 ) * nodeVar63 ).xyz;
		nodeVar17 = ( nodeVar17 + nodeVar45 );
		nodeVar64 = ( nodeVar17 * vec3<f32>( 0.01 ) );

		if ( nodeVar6 ) {

			nodeVar65 = ( nodeVar65 + ( ( nodeVar64 * nodeVar14 ) * vec3<f32>( nodeVar10 ) ) );
			

		} else {

			nodeVar65 = ( ( nodeVar65 * exp( ( ( - ( nodeVar17 * vec3<f32>( 0.01 ) ) ) * vec3<f32>( nodeVar10 ) ) ) ) + ( nodeVar64 * vec3<f32>( nodeVar10 ) ) );
			

		}

		nodeVar14 = ( nodeVar14 * exp( ( ( - ( nodeVar17 * vec3<f32>( 0.01 ) ) ) * vec3<f32>( nodeVar10 ) ) ) );
		nodeVar13 = ( nodeVar13 + nodeVar10 );

	}

	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar66 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar66;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar67 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar67;
	nodeVar68 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar68;
	outgoingLight = nodeVar65;
	nodeVar69 = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar69;

	// result

	output.color = nodeVar69;

	return output;

}
