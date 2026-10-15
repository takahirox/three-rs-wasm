// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform5_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform5 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : f32,
	nodeUniform6 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform8 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform7 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : f32;
var<private> nodeVar123 : f32;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : f32;
var<private> nodeVar126 : f32;
var<private> nodeVar127 : f32;
var<private> nodeVar128 : f32;
var<private> nodeVar129 : f32;
var<private> nodeVar130 : f32;
var<private> nodeVar131 : f32;
var<private> nodeVar132 : f32;
var<private> nodeVar133 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : vec4<f32>;

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


fn mx_fractal_noise_float ( p : vec3<f32>, octaves : i32, lacunarity : f32, diminish : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : vec3<f32>;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : i32;

	nodeVar0 = diminish;
	nodeVar1 = lacunarity;
	nodeVar2 = p;
	nodeVar3 = 0.0;
	nodeVar4 = 1.0;
	nodeVar5 = octaves;

	for ( var i : i32 = 0; i < nodeVar5; i ++ ) {

		nodeVar3 = ( nodeVar3 + ( nodeVar4 * mx_perlin_noise_float_1( nodeVar2 ) ) );
		nodeVar4 = ( nodeVar4 * nodeVar0 );
		nodeVar2 = ( nodeVar2 * vec3<f32>( nodeVar1 ) );

	}


	return nodeVar3;

}


fn V_GGX_SmithCorrelated ( alpha : f32, dotNL : f32, dotNV : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = ( alpha * alpha );

	return ( 0.5 / max( ( ( dotNL * sqrt( ( nodeVar0 + ( ( 1.0 - nodeVar0 ) * ( dotNV * dotNV ) ) ) ) ) + ( dotNV * sqrt( ( nodeVar0 + ( ( 1.0 - nodeVar0 ) * ( dotNL * dotNL ) ) ) ) ) ), 0.000001 ) );

}


fn D_GGX ( alpha : f32, dotNH : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;

	nodeVar0 = ( alpha * alpha );
	nodeVar1 = ( 1.0 - ( ( dotNH * dotNH ) * ( 1.0 - nodeVar0 ) ) );

	return ( ( nodeVar0 / ( nodeVar1 * nodeVar1 ) ) * 0.3183098861837907 );

}




@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) positionLocal : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( ( ( vec3<f32>( 0.02955683443236377, 0.025186859622305935, 0.02121901037134225 ) * vec3<f32>( ( ( ( mx_fractal_noise_float( ( positionLocal * vec3<f32>( 35.0, 2.0, 35.0 ) ), 3, 2.0, 0.5 ) * 1.0 ) * 0.18 ) + 0.9 ) ) ) * vec3<f32>( smoothstep( 0.0, 0.5, positionLocal.y ) ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	Metalness = 0.0;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( 0.95, 0.0525 ) + max( max( nodeVar0.x, nodeVar0.y ), nodeVar0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - 0.0 ) ) );
	EmissiveColor = ( object.nodeUniform3 * vec3<f32>( object.nodeUniform4 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar1 = dot( normalView, positionViewDirection );
	nodeVar2 = textureSample( nodeUniform5, nodeUniform5_sampler, vec2<f32>( Roughness, clamp( nodeVar1, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar2;
	nodeVar3 = ( dfg.x + dfg.y );
	nodeVar4 = ( 1.0 / nodeVar3 );
	nodeVar5 = nodeVar4;
	nodeVar6 = ( nodeVar5 - 1.0 );
	nodeVar7 = ( SpecularColorBlended * vec3<f32>( nodeVar6 ) );
	nodeVar8 = ( nodeVar7 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar8;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar9 = dot( normalWorld, normalize( render.nodeUniform10 ) );
	nodeVar10 = ( nodeVar9 * 0.5 );
	nodeVar11 = ( nodeVar10 + 0.5 );
	nodeVar12 = mix( render.nodeUniform7, render.nodeUniform8, nodeVar11 );
	nodeVar13 = ( irradiance + nodeVar12 );
	irradiance = nodeVar13;
	nodeVar14 = ( render.nodeUniform11 - render.nodeUniform12 );
	nodeVar15 = vec4<f32>( nodeVar14, 0.0 );
	nodeVar16 = ( render.cameraViewMatrix * nodeVar15 );
	nodeVar17 = normalize( nodeVar16.xyz );
	nodeVar18 = nodeVar17;
	nodeVar19 = dot( normalView, nodeVar18 );
	nodeVar20 = ( vec3<f32>( clamp( nodeVar19, 0.0, 1.0 ) ) * render.nodeUniform13 );
	nodeVar21 = nodeVar20;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar22 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar23 = ( nodeVar21 * nodeVar22 );
	nodeVar24 = ( nodeVar18 + positionViewDirection );
	nodeVar25 = normalize( nodeVar24 );
	nodeVar26 = dot( positionViewDirection, nodeVar25 );
	nodeVar27 = clamp( nodeVar26, 0.0, 1.0 );
	nodeVar28 = exp2( ( ( ( nodeVar27 * -5.55473 ) - 6.98316 ) * nodeVar27 ) );
	nodeVar29 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar28 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar28 ) ) );
	nodeVar30 = ( vec3<f32>( 1.0 ) - nodeVar29 );
	nodeVar31 = nodeVar30;
	nodeVar32 = ( nodeVar23 * nodeVar31 );
	nodeVar33 = ( directDiffuse + nodeVar32 );
	directDiffuse = nodeVar33;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar34 = normalize( ( nodeVar18 + positionViewDirection ) );
	nodeVar35 = clamp( dot( positionViewDirection, nodeVar34 ), 0.0, 1.0 );
	nodeVar36 = exp2( ( ( ( nodeVar35 * -5.55473 ) - 6.98316 ) * nodeVar35 ) );
	nodeVar37 = ( Roughness * Roughness );
	nodeVar38 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar36 ) ) ) + vec3<f32>( ( 1.0 * nodeVar36 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar37, clamp( dot( normalView, nodeVar18 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar37, clamp( dot( normalView, nodeVar34 ), 0.0, 1.0 ) ) ) );
	nodeVar39 = ( nodeVar21 * nodeVar38 );
	nodeVar40 = ( nodeVar39 * multiScatteringCompensation );
	nodeVar41 = ( directSpecular + nodeVar40 );
	directSpecular = nodeVar41;
	nodeVar42 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar43 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar44 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar45 = ( SpecularF90 * dfg.y );
	nodeVar46 = ( nodeVar44 + vec3<f32>( nodeVar45 ) );
	nodeVar47 = ( nodeVar42 + nodeVar46 );
	nodeVar42 = nodeVar47;
	nodeVar48 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar49 = nodeVar48;
	nodeVar50 = ( nodeVar49 * vec3<f32>( 0.047619 ) );
	nodeVar51 = ( SpecularColor + nodeVar50 );
	nodeVar52 = ( nodeVar46 * nodeVar51 );
	nodeVar53 = ( dfg.x + dfg.y );
	nodeVar54 = ( 1.0 - nodeVar53 );
	nodeVar55 = nodeVar54;
	nodeVar56 = ( vec3<f32>( nodeVar55 ) * nodeVar51 );
	nodeVar57 = ( vec3<f32>( 1.0 ) - nodeVar56 );
	nodeVar58 = nodeVar57;
	nodeVar59 = ( nodeVar52 / nodeVar58 );
	nodeVar60 = ( nodeVar59 * vec3<f32>( nodeVar55 ) );
	nodeVar61 = ( nodeVar43 + nodeVar60 );
	nodeVar43 = nodeVar61;
	nodeVar62 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar63 = ( irradiance * nodeVar62 );
	nodeVar64 = ( nodeVar42 + nodeVar43 );
	nodeVar65 = ( vec3<f32>( 1.0 ) - nodeVar64 );
	nodeVar66 = nodeVar65;
	nodeVar67 = ( nodeVar63 * nodeVar66 );
	nodeVar68 = nodeVar67;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar69 = ( indirectDiffuse + nodeVar68 );
	indirectDiffuse = nodeVar69;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar70 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar71 = ( SpecularF90 * dfg.y );
	nodeVar72 = ( nodeVar70 + vec3<f32>( nodeVar71 ) );
	nodeVar73 = ( singleScatteringDielectric + nodeVar72 );
	singleScatteringDielectric = nodeVar73;
	nodeVar74 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar75 = nodeVar74;
	nodeVar76 = ( nodeVar75 * vec3<f32>( 0.047619 ) );
	nodeVar77 = ( SpecularColor + nodeVar76 );
	nodeVar78 = ( nodeVar72 * nodeVar77 );
	nodeVar79 = ( dfg.x + dfg.y );
	nodeVar80 = ( 1.0 - nodeVar79 );
	nodeVar81 = nodeVar80;
	nodeVar82 = ( vec3<f32>( nodeVar81 ) * nodeVar77 );
	nodeVar83 = ( vec3<f32>( 1.0 ) - nodeVar82 );
	nodeVar84 = nodeVar83;
	nodeVar85 = ( nodeVar78 / nodeVar84 );
	nodeVar86 = ( nodeVar85 * vec3<f32>( nodeVar81 ) );
	nodeVar87 = ( multiScatteringDielectric + nodeVar86 );
	multiScatteringDielectric = nodeVar87;
	nodeVar88 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar89 = ( SpecularF90 * dfg.y );
	nodeVar90 = ( nodeVar88 + vec3<f32>( nodeVar89 ) );
	nodeVar91 = ( singleScatteringMetallic + nodeVar90 );
	singleScatteringMetallic = nodeVar91;
	nodeVar92 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar93 = nodeVar92;
	nodeVar94 = ( nodeVar93 * vec3<f32>( 0.047619 ) );
	nodeVar95 = ( DiffuseColor.xyz + nodeVar94 );
	nodeVar96 = ( nodeVar90 * nodeVar95 );
	nodeVar97 = ( dfg.x + dfg.y );
	nodeVar98 = ( 1.0 - nodeVar97 );
	nodeVar99 = nodeVar98;
	nodeVar100 = ( vec3<f32>( nodeVar99 ) * nodeVar95 );
	nodeVar101 = ( vec3<f32>( 1.0 ) - nodeVar100 );
	nodeVar102 = nodeVar101;
	nodeVar103 = ( nodeVar96 / nodeVar102 );
	nodeVar104 = ( nodeVar103 * vec3<f32>( nodeVar99 ) );
	nodeVar105 = ( multiScatteringMetallic + nodeVar104 );
	multiScatteringMetallic = nodeVar105;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar106 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar107 = ( radiance * nodeVar106 );
	nodeVar108 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar109 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar110 = ( nodeVar108 * nodeVar109 );
	nodeVar111 = ( nodeVar107 + nodeVar110 );
	nodeVar112 = nodeVar111;
	nodeVar113 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar114 = ( vec3<f32>( 1.0 ) - nodeVar113 );
	nodeVar115 = nodeVar114;
	nodeVar116 = ( DiffuseContribution * nodeVar115 );
	nodeVar117 = ( nodeVar116 * nodeVar109 );
	nodeVar118 = nodeVar117;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar119 = ( indirectSpecular + nodeVar112 );
	indirectSpecular = nodeVar119;
	nodeVar120 = ( indirectDiffuse + nodeVar118 );
	indirectDiffuse = nodeVar120;
	ambientOcclusion = 1.0;
	nodeVar121 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar121;
	nodeVar122 = dot( normalView, positionViewDirection );
	nodeVar123 = ( clamp( nodeVar122, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar124 = ( Roughness * -16.0 );
	nodeVar125 = ( 1.0 - nodeVar124 );
	nodeVar126 = nodeVar125;
	nodeVar127 = ( - nodeVar126 );
	nodeVar128 = exp2( nodeVar127 );
	nodeVar129 = pow( nodeVar123, nodeVar128 );
	nodeVar130 = ( 1.0 - nodeVar129 );
	nodeVar131 = nodeVar130;
	nodeVar132 = ( ambientOcclusion - nodeVar131 );
	nodeVar133 = ( indirectSpecular * vec3<f32>( clamp( nodeVar132, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar133;
	nodeVar134 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar134;
	nodeVar135 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar135;
	nodeVar136 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar136;
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar137 = ( - v_positionView.z );
	nodeVar138 = vec4<f32>( mix( Output.xyz, render.nodeUniform14, ( 1.0 - exp( ( - ( ( ( render.nodeUniform15 * render.nodeUniform15 ) * nodeVar137 ) * nodeVar137 ) ) ) ) ), Output.w );
	Output = nodeVar138;

	// result

	output.color = nodeVar138;

	return output;

}
