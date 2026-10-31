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
@binding( 1 ) @group( 1 ) var nodeUniform7_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform7 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform15_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform15 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform19_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform19 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : vec3<f32>,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : f32,
	nodeUniform20 : mat4x4<f32>,
	nodeUniform22 : f32,
	nodeUniform23 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraPosition : vec3<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform13 : f32,
	nodeUniform14 : f32,
	nodeUniform16 : f32,
	nodeUniform17 : vec2<f32>,
	nodeUniform18 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : u32;
var<private> nodeVar3 : vec3<f32>;
var<private> normalLocal : vec3<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
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
var<private> nodeVar68 : f32;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : u32;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : f32;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : f32;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : f32;
var<private> nodeVar118 : vec4<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : f32;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> normalWorldGeometry : vec3<f32>;
var<private> nodeVar131 : f32;
var<private> nodeVar132 : f32;
var<private> nodeVar133 : u32;
var<private> nodeVar134 : f32;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : f32;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : f32;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar145 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar146 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar154 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar155 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn5 ( seed : u32 ) -> f32 {

	

	let nodeConst0 = ( ( seed * 747796405u ) + 2891336453u );
	let nodeConst1 = ( ( ( nodeConst0 >> ( ( nodeConst0 >> 28u ) + 4u ) ) ^ nodeConst0 ) * 277803737u );

	return ( f32( ( ( nodeConst1 >> 22u ) ^ nodeConst1 ) ) * 2.3283064365386963e-10 );

}


fn fn61 ( x : u32, k : i32 ) -> u32 {

	var nodeVar0 : i32;
	var nodeVar1 : u32;

	nodeVar0 = k;
	nodeVar1 = x;

	return ( ( nodeVar1 << u32( nodeVar0 ) ) | ( nodeVar1 >> u32( ( 32 - nodeVar0 ) ) ) );

}


fn fn63 ( a : u32, b : u32, c : u32 ) -> u32 {

	var nodeVar0 : u32;
	var nodeVar1 : u32;
	var nodeVar2 : u32;

	nodeVar0 = c;
	nodeVar1 = b;
	nodeVar2 = a;
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - fn61( nodeVar1, 14 ) );
	nodeVar2 = ( nodeVar2 ^ nodeVar0 );
	nodeVar2 = ( nodeVar2 - fn61( nodeVar0, 11 ) );
	nodeVar1 = ( nodeVar1 ^ nodeVar2 );
	nodeVar1 = ( nodeVar1 - fn61( nodeVar2, 25 ) );
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - fn61( nodeVar1, 16 ) );
	nodeVar2 = ( nodeVar2 ^ nodeVar0 );
	nodeVar2 = ( nodeVar2 - fn61( nodeVar0, 4 ) );
	nodeVar1 = ( nodeVar1 ^ nodeVar2 );
	nodeVar1 = ( nodeVar1 - fn61( nodeVar2, 14 ) );
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - fn61( nodeVar1, 24 ) );

	return nodeVar0;

}


fn fn46 ( b : bool, t : f32, f : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : bool;
	var nodeVar3 : f32;

	nodeVar0 = f;
	nodeVar1 = t;
	nodeVar2 = b;

	return select( nodeVar0, nodeVar1, nodeVar2 );

}


fn fn47 ( val : f32, b : bool ) -> f32 {

	var nodeVar0 : bool;
	var nodeVar1 : f32;
	var nodeVar2 : f32;

	nodeVar0 = b;
	nodeVar1 = val;

	return select( nodeVar1, ( - nodeVar1 ), nodeVar0 );

}


fn fn48 ( x : f32 ) -> i32 {

	var nodeVar0 : f32;

	nodeVar0 = x;

	return i32( floor( nodeVar0 ) );

}


fn fn65 ( t : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = t;

	return ( ( ( nodeVar0 * nodeVar0 ) * nodeVar0 ) * ( ( nodeVar0 * ( ( nodeVar0 * 6.0 ) - 15.0 ) ) + 10.0 ) );

}


fn fn68 ( x : i32, y : i32, z : i32 ) -> u32 {

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

	return fn63( nodeVar4, nodeVar5, nodeVar6 );

}


fn fn54 ( hash : u32, x : f32, y : f32, z : f32 ) -> f32 {

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
	nodeVar5 = fn46( ( nodeVar4 < 8u ), nodeVar2, nodeVar1 );
	nodeVar6 = fn46( ( nodeVar4 < 4u ), nodeVar1, fn46( ( ( nodeVar4 == 12u ) || ( nodeVar4 == 14u ) ), nodeVar2, nodeVar0 ) );

	return ( fn47( nodeVar5, bool( ( nodeVar4 & 1u ) ) ) + fn47( nodeVar6, bool( ( nodeVar4 & 2u ) ) ) );

}


fn fn51 ( v0 : f32, v1 : f32, v2 : f32, v3 : f32, v4 : f32, v5 : f32, v6 : f32, v7 : f32, s : f32, t : f32, r : f32 ) -> f32 {

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


fn fn58 ( v : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = v;

	return ( 0.982 * nodeVar0 );

}


fn fn74 ( p : vec3<f32> ) -> f32 {

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
	nodeVar1 = fn48( nodeVar4 );
	nodeVar5 = ( nodeVar4 - f32( nodeVar1 ) );
	nodeVar6 = nodeVar0.y;
	nodeVar2 = fn48( nodeVar6 );
	nodeVar7 = ( nodeVar6 - f32( nodeVar2 ) );
	nodeVar8 = nodeVar0.z;
	nodeVar3 = fn48( nodeVar8 );
	nodeVar9 = ( nodeVar8 - f32( nodeVar3 ) );
	nodeVar10 = fn65( nodeVar5 );
	nodeVar11 = fn65( nodeVar7 );
	nodeVar12 = fn65( nodeVar9 );
	nodeVar13 = fn51( fn54( fn68( nodeVar1, nodeVar2, nodeVar3 ), nodeVar5, nodeVar7, nodeVar9 ), fn54( fn68( ( nodeVar1 + 1 ), nodeVar2, nodeVar3 ), ( nodeVar5 - 1.0 ), nodeVar7, nodeVar9 ), fn54( fn68( nodeVar1, ( nodeVar2 + 1 ), nodeVar3 ), nodeVar5, ( nodeVar7 - 1.0 ), nodeVar9 ), fn54( fn68( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ), nodeVar3 ), ( nodeVar5 - 1.0 ), ( nodeVar7 - 1.0 ), nodeVar9 ), fn54( fn68( nodeVar1, nodeVar2, ( nodeVar3 + 1 ) ), nodeVar5, nodeVar7, ( nodeVar9 - 1.0 ) ), fn54( fn68( ( nodeVar1 + 1 ), nodeVar2, ( nodeVar3 + 1 ) ), ( nodeVar5 - 1.0 ), nodeVar7, ( nodeVar9 - 1.0 ) ), fn54( fn68( nodeVar1, ( nodeVar2 + 1 ), ( nodeVar3 + 1 ) ), nodeVar5, ( nodeVar7 - 1.0 ), ( nodeVar9 - 1.0 ) ), fn54( fn68( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ), ( nodeVar3 + 1 ) ), ( nodeVar5 - 1.0 ), ( nodeVar7 - 1.0 ), ( nodeVar9 - 1.0 ) ), nodeVar10, nodeVar11, nodeVar12 );

	return fn58( nodeVar13 );

}


fn tsl_mod_float( x : f32, y : f32 ) -> f32 { return x - y * floor( x / y ); }
fn fn130 ( p : vec3<f32> ) -> f32 {

	

	let nodeConst0 = floor( p );
	let nodeConst1 = ( nodeConst0 + vec3<f32>( 0.0, 0.0, 0.0 ) );
	let nodeConst2 = ( nodeConst0 + vec3<f32>( 1.0, 0.0, 0.0 ) );
	let nodeConst3 = fract( p );
	let nodeConst4 = ( ( nodeConst3 * nodeConst3 ) * ( ( nodeConst3 * vec3<f32>( -2.0 ) ) + vec3<f32>( 3.0 ) ) );
	let nodeConst5 = ( nodeConst0 + vec3<f32>( 0.0, 1.0, 0.0 ) );
	let nodeConst6 = ( nodeConst0 + vec3<f32>( 1.0, 1.0, 0.0 ) );
	let nodeConst7 = ( nodeConst0 + vec3<f32>( 0.0, 0.0, 1.0 ) );
	let nodeConst8 = ( nodeConst0 + vec3<f32>( 1.0, 0.0, 1.0 ) );
	let nodeConst9 = ( nodeConst0 + vec3<f32>( 0.0, 1.0, 1.0 ) );
	let nodeConst10 = ( nodeConst0 + vec3<f32>( 1.0, 1.0, 1.0 ) );

	return ( ( mix( mix( mix( fn5( ( ( ( u32( ( nodeConst1.x + 1048576.0 ) ) * 73856093u ) ^ ( u32( ( nodeConst1.y + 1048576.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeConst1.z + 1048576.0 ) ) * 83492791u ) ) ), fn5( ( ( ( u32( ( nodeConst2.x + 1048576.0 ) ) * 73856093u ) ^ ( u32( ( nodeConst2.y + 1048576.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeConst2.z + 1048576.0 ) ) * 83492791u ) ) ), nodeConst4.x ), mix( fn5( ( ( ( u32( ( nodeConst5.x + 1048576.0 ) ) * 73856093u ) ^ ( u32( ( nodeConst5.y + 1048576.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeConst5.z + 1048576.0 ) ) * 83492791u ) ) ), fn5( ( ( ( u32( ( nodeConst6.x + 1048576.0 ) ) * 73856093u ) ^ ( u32( ( nodeConst6.y + 1048576.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeConst6.z + 1048576.0 ) ) * 83492791u ) ) ), nodeConst4.x ), nodeConst4.y ), mix( mix( fn5( ( ( ( u32( ( nodeConst7.x + 1048576.0 ) ) * 73856093u ) ^ ( u32( ( nodeConst7.y + 1048576.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeConst7.z + 1048576.0 ) ) * 83492791u ) ) ), fn5( ( ( ( u32( ( nodeConst8.x + 1048576.0 ) ) * 73856093u ) ^ ( u32( ( nodeConst8.y + 1048576.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeConst8.z + 1048576.0 ) ) * 83492791u ) ) ), nodeConst4.x ), mix( fn5( ( ( ( u32( ( nodeConst9.x + 1048576.0 ) ) * 73856093u ) ^ ( u32( ( nodeConst9.y + 1048576.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeConst9.z + 1048576.0 ) ) * 83492791u ) ) ), fn5( ( ( ( u32( ( nodeConst10.x + 1048576.0 ) ) * 73856093u ) ^ ( u32( ( nodeConst10.y + 1048576.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeConst10.z + 1048576.0 ) ) * 83492791u ) ) ), nodeConst4.x ), nodeConst4.y ), nodeConst4.z ) * 2.0 ) - 1.0 );

}


fn fn87 ( p : vec3<f32>, octaves : i32, lacunarity : f32, diminish : f32 ) -> f32 {

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

		nodeVar3 = ( nodeVar3 + ( nodeVar4 * fn74( nodeVar2 ) ) );
		nodeVar4 = ( nodeVar4 * nodeVar0 );
		nodeVar2 = ( nodeVar2 * vec3<f32>( nodeVar1 ) );

	}


	return nodeVar3;

}


fn fn24 ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn fn25 ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	

	let nodeConst0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeConst0 ), sin( nodeConst0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

}


fn fn110 ( alpha : f32, dotNL : f32, dotNV : f32 ) -> f32 {

	

	let nodeConst0 = ( alpha * alpha );

	return ( 0.5 / max( ( ( dotNL * sqrt( ( nodeConst0 + ( ( 1.0 - nodeConst0 ) * ( dotNV * dotNV ) ) ) ) ) + ( dotNV * sqrt( ( nodeConst0 + ( ( 1.0 - nodeConst0 ) * ( dotNL * dotNL ) ) ) ) ) ), 0.000001 ) );

}


fn fn112 ( alpha : f32, dotNH : f32 ) -> f32 {

	

	let nodeConst0 = ( alpha * alpha );
	let nodeConst1 = ( 1.0 - ( ( dotNH * dotNH ) * ( 1.0 - nodeConst0 ) ) );

	return ( ( nodeConst0 / ( nodeConst1 * nodeConst1 ) ) * 0.3183098861837907 );

}




@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) positionLocal : vec3<f32>,
	@location( 2 ) @interpolate( flat, either ) nodeVarying3 : f32,
	@location( 3 ) @interpolate( flat, either ) nodeVarying4 : vec3<f32>,
	@location( 4 ) v_positionWorld : vec3<f32>,
	@location( 5 ) nodeVarying6 : f32,
	@location( 6 ) v_normalWorldGeometry : vec3<f32>,
	@location( 7 ) v_normalViewGeometry : vec3<f32>,
	@location( 8 ) v_positionViewDirection : vec3<f32>,
	@location( 9 ) nodeVarying11 : vec2<f32>,
	@location( 10 ) nodeVarying12 : vec3<f32>,
	@location( 11 ) nodeVarying13 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	let nodeConst0 = ( nodeVarying3 == 4.0 );
	let nodeConst1 = ( nodeVarying3 == 6.0 );
	let nodeConst2 = ( nodeConst0 || nodeConst1 );

	if ( nodeConst2 ) {


		if ( nodeConst1 ) {

			let nodeConst3 = floor( ( nodeVarying4 * vec3<f32>( 2.0 ) ) );
			nodeVar2 = ( ( ( u32( ( nodeConst3.x + 2097152.0 ) ) * 73856093u ) ^ ( u32( ( nodeConst3.y + 2097152.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeConst3.z + 2097152.0 ) ) * 83492791u ) );
			let nodeConst4 = ( nodeVarying11.y * 2.4 );
			let nodeConst5 = vec3<f32>( ( nodeVarying11.x * 0.5 ), ( nodeVarying11.y * 0.5 ), ( 0.1 + nodeConst4 ) );
			let nodeConst6 = vec3<f32>( ( - nodeConst5.x ), ( - nodeConst5.y ), 0.1 );
			let nodeConst7 = ( positionLocal - nodeVarying4 );
			normalLocal = nodeVarying12;
			let nodeConst8 = normalize( cross( vec3<f32>( 0.0, 1.0, 0.0 ), normalLocal ) );
			let nodeConst9 = vec3<f32>( dot( nodeConst7, nodeConst8 ), nodeConst7.y, 0.0 );
			let nodeConst10 = normalize( ( positionLocal - ( object.nodeUniform0 * vec4<f32>( render.cameraPosition, 1.0 ) ).xyz ) );
			let nodeConst11 = vec3<f32>( dot( nodeConst10, nodeConst8 ), nodeConst10.y, ( - dot( nodeConst10, normalLocal ) ) );
			let nodeConst12 = ( ( vec3<f32>( ( - nodeConst5.x ), nodeConst6.y, 0.1 ) - nodeConst9 ) / nodeConst11 );
			let nodeConst13 = ( ( vec3<f32>( nodeConst5.x, nodeConst5.y, ( 0.1 + 0.05 ) ) - nodeConst9 ) / nodeConst11 );
			let nodeConst14 = max( nodeConst12, nodeConst13 );
			let nodeConst15 = min( nodeConst12, nodeConst13 );
			let nodeConst16 = max( max( nodeConst15.x, nodeConst15.y ), nodeConst15.z );
			let nodeConst17 = step( 0.18, fn5( ( nodeVar2 + 79599u ) ) );
			let nodeConst18 = ( 0.1 + 1.2 );
			let nodeConst19 = vec3<f32>( -0.45, nodeConst6.y, nodeConst18 );
			let nodeConst20 = ( ( nodeConst19 - nodeConst9 ) / nodeConst11 );
			let nodeConst21 = ( nodeConst5.z - 1.2 );
			let nodeConst22 = vec3<f32>( 0.45, ( nodeConst6.y + 1.5 ), nodeConst21 );
			let nodeConst23 = ( ( nodeConst22 - nodeConst9 ) / nodeConst11 );
			let nodeConst24 = max( nodeConst20, nodeConst23 );
			let nodeConst25 = min( nodeConst20, nodeConst23 );
			let nodeConst26 = max( max( nodeConst25.x, nodeConst25.y ), nodeConst25.z );
			let nodeConst27 = step( 0.45, fn5( ( nodeVar2 + 34690u ) ) );
			let nodeConst28 = fn5( ( nodeVar2 + 105097u ) );
			let nodeConst29 = mix( ( ( - nodeConst5.x ) + 0.7 ), ( nodeConst5.x - 0.7 ), step( 0.5, nodeConst28 ) );
			let nodeConst30 = ( 0.1 + ( nodeConst4 * 0.5 ) );
			let nodeConst31 = vec3<f32>( ( nodeConst29 - 0.75 ), nodeConst6.y, nodeConst30 );
			let nodeConst32 = ( ( nodeConst31 - nodeConst9 ) / nodeConst11 );
			let nodeConst33 = vec3<f32>( ( nodeConst29 + 0.75 ), ( nodeConst6.y + 1.0 ), ( nodeConst30 + 0.65 ) );
			let nodeConst34 = ( ( nodeConst33 - nodeConst9 ) / nodeConst11 );
			let nodeConst35 = max( nodeConst32, nodeConst34 );
			let nodeConst36 = min( nodeConst32, nodeConst34 );
			let nodeConst37 = max( max( nodeConst36.x, nodeConst36.y ), nodeConst36.z );
			let nodeConst38 = ( nodeConst5.x * 0.62 );
			let nodeConst39 = fn5( ( nodeVar2 + 40310u ) );
			let nodeConst40 = mix( 0.16, 0.3, nodeConst39 );
			let nodeConst41 = vec3<f32>( ( nodeConst38 - nodeConst40 ), ( nodeConst6.y + 0.4 ), ( 0.1 + 0.25 ) );
			let nodeConst42 = ( ( nodeConst41 - nodeConst9 ) / nodeConst11 );
			let nodeConst43 = fn5( ( nodeVar2 + 66130u ) );
			let nodeConst44 = vec3<f32>( ( nodeConst38 + nodeConst40 ), ( ( nodeConst6.y + 0.4 ) + mix( 0.5, 1.2, nodeConst43 ) ), ( ( 0.1 + 0.25 ) + ( nodeConst40 * 2.0 ) ) );
			let nodeConst45 = ( ( nodeConst44 - nodeConst9 ) / nodeConst11 );
			let nodeConst46 = max( nodeConst42, nodeConst45 );
			let nodeConst47 = min( nodeConst42, nodeConst45 );
			let nodeConst48 = max( max( nodeConst47.x, nodeConst47.y ), nodeConst47.z );
			let nodeConst49 = ( nodeConst5.x * 0.0 );
			let nodeConst50 = fn5( ( nodeVar2 + 39610u ) );
			let nodeConst51 = mix( 0.16, 0.3, nodeConst50 );
			let nodeConst52 = vec3<f32>( ( nodeConst49 - nodeConst51 ), ( nodeConst6.y + 0.4 ), ( 0.1 + 0.25 ) );
			let nodeConst53 = ( ( nodeConst52 - nodeConst9 ) / nodeConst11 );
			let nodeConst54 = fn5( ( nodeVar2 + 66030u ) );
			let nodeConst55 = vec3<f32>( ( nodeConst49 + nodeConst51 ), ( ( nodeConst6.y + 0.4 ) + mix( 0.5, 1.2, nodeConst54 ) ), ( ( 0.1 + 0.25 ) + ( nodeConst51 * 2.0 ) ) );
			let nodeConst56 = ( ( nodeConst55 - nodeConst9 ) / nodeConst11 );
			let nodeConst57 = max( nodeConst53, nodeConst56 );
			let nodeConst58 = min( nodeConst53, nodeConst56 );
			let nodeConst59 = max( max( nodeConst58.x, nodeConst58.y ), nodeConst58.z );
			let nodeConst60 = ( nodeConst5.x * -0.62 );
			let nodeConst61 = fn5( ( nodeVar2 + 38910u ) );
			let nodeConst62 = mix( 0.16, 0.3, nodeConst61 );
			let nodeConst63 = vec3<f32>( ( nodeConst60 - nodeConst62 ), ( nodeConst6.y + 0.4 ), ( 0.1 + 0.25 ) );
			let nodeConst64 = ( ( nodeConst63 - nodeConst9 ) / nodeConst11 );
			let nodeConst65 = fn5( ( nodeVar2 + 65930u ) );
			let nodeConst66 = vec3<f32>( ( nodeConst60 + nodeConst62 ), ( ( nodeConst6.y + 0.4 ) + mix( 0.5, 1.2, nodeConst65 ) ), ( ( 0.1 + 0.25 ) + ( nodeConst62 * 2.0 ) ) );
			let nodeConst67 = ( ( nodeConst66 - nodeConst9 ) / nodeConst11 );
			let nodeConst68 = max( nodeConst64, nodeConst67 );
			let nodeConst69 = min( nodeConst64, nodeConst67 );
			let nodeConst70 = max( max( nodeConst69.x, nodeConst69.y ), nodeConst69.z );
			let nodeConst71 = vec3<f32>( ( ( - nodeConst5.x ) + 0.15 ), nodeConst6.y, ( 0.1 + 0.06 ) );
			let nodeConst72 = ( ( nodeConst71 - nodeConst9 ) / nodeConst11 );
			let nodeConst73 = vec3<f32>( ( nodeConst5.x - 0.15 ), ( nodeConst6.y + 0.4 ), ( 0.1 + 0.95 ) );
			let nodeConst74 = ( ( nodeConst73 - nodeConst9 ) / nodeConst11 );
			let nodeConst75 = max( nodeConst72, nodeConst74 );
			let nodeConst76 = min( nodeConst72, nodeConst74 );
			let nodeConst77 = max( max( nodeConst76.x, nodeConst76.y ), nodeConst76.z );
			let nodeConst78 = max( ( ( nodeConst6 - nodeConst9 ) / nodeConst11 ), ( ( nodeConst5 - nodeConst9 ) / nodeConst11 ) );
			let nodeConst79 = min( min( nodeConst78.x, nodeConst78.y ), nodeConst78.z );
			let nodeConst80 = ( ( ( min( min( nodeConst75.x, nodeConst75.y ), nodeConst75.z ) > nodeConst77 ) && ( nodeConst77 > 0.0 ) ) && ( nodeConst77 < nodeConst79 ) );

			if ( nodeConst80 ) {

				nodeVar9 = nodeConst77;

			} else {

				nodeVar9 = nodeConst79;

			}

			let nodeConst81 = ( ( ( ( min( min( nodeConst68.x, nodeConst68.y ), nodeConst68.z ) > nodeConst70 ) && ( nodeConst70 > 0.0 ) ) && ( nodeConst61 > 0.15 ) ) && ( nodeConst70 < nodeVar9 ) );

			if ( nodeConst81 ) {

				nodeVar8 = nodeConst70;

			} else {

				nodeVar8 = nodeVar9;

			}

			let nodeConst82 = ( ( ( ( min( min( nodeConst57.x, nodeConst57.y ), nodeConst57.z ) > nodeConst59 ) && ( nodeConst59 > 0.0 ) ) && ( nodeConst50 > 0.15 ) ) && ( nodeConst59 < nodeVar8 ) );

			if ( nodeConst82 ) {

				nodeVar7 = nodeConst59;

			} else {

				nodeVar7 = nodeVar8;

			}

			let nodeConst83 = ( ( ( ( min( min( nodeConst46.x, nodeConst46.y ), nodeConst46.z ) > nodeConst48 ) && ( nodeConst48 > 0.0 ) ) && ( nodeConst39 > 0.15 ) ) && ( nodeConst48 < nodeVar7 ) );

			if ( nodeConst83 ) {

				nodeVar6 = nodeConst48;

			} else {

				nodeVar6 = nodeVar7;

			}

			let nodeConst84 = ( ( ( min( min( nodeConst35.x, nodeConst35.y ), nodeConst35.z ) > nodeConst37 ) && ( nodeConst37 > 0.0 ) ) && ( nodeConst37 < nodeVar6 ) );

			if ( nodeConst84 ) {

				nodeVar5 = nodeConst37;

			} else {

				nodeVar5 = nodeVar6;

			}

			let nodeConst85 = ( ( ( ( min( min( nodeConst24.x, nodeConst24.y ), nodeConst24.z ) > nodeConst26 ) && ( nodeConst26 > 0.0 ) ) && ( nodeConst27 > 0.5 ) ) && ( nodeConst26 < nodeVar5 ) );

			if ( nodeConst85 ) {

				nodeVar4 = nodeConst26;

			} else {

				nodeVar4 = nodeVar5;

			}

			let nodeConst86 = ( ( ( ( min( min( nodeConst14.x, nodeConst14.y ), nodeConst14.z ) > nodeConst16 ) && ( nodeConst16 > 0.0 ) ) && ( ( ( 1.0 - nodeConst17 ) * step( 0.45, fn5( ( nodeVar2 + 39650u ) ) ) ) > 0.5 ) ) && ( nodeConst16 < nodeVar4 ) );

			if ( nodeConst86 ) {

				nodeVar3 = ( mix( vec3<f32>( 0.0722718506743852, 0.08228270712149792, 0.09758734713304495 ), vec3<f32>( 0.00972121731707524, 0.010960094003125918, 0.012983032338510335 ), step( 0.62, fract( ( ( nodeConst9 + ( nodeConst11 * vec3<f32>( nodeConst16 ) ) ).y * 7.0 ) ) ) ) * vec3<f32>( 0.8 ) );

			} else {


				if ( nodeConst85 ) {

					let nodeConst87 = ( nodeConst9 + ( nodeConst11 * vec3<f32>( nodeConst26 ) ) );
					let nodeConst88 = ( ( nodeConst87.z - nodeConst18 ) / ( nodeConst21 - nodeConst18 ) );
					let nodeConst89 = ( ( ( nodeVar2 + 91u ) + ( u32( floor( ( nodeConst88 * 8.0 ) ) ) * 197u ) ) + ( u32( floor( ( ( ( nodeConst87 - nodeConst19 ) / ( nodeConst22 - nodeConst19 ) ).y * 4.0 ) ) ) * 4099u ) );
					let nodeConst90 = ( fn5( nodeConst89 ) * 6.0 );

					if ( ( nodeConst90 > 5.0 ) ) {

						nodeVar11 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

					} else {


						if ( ( nodeConst90 > 4.0 ) ) {

							nodeVar12 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

						} else {


							if ( ( nodeConst90 > 3.0 ) ) {

								nodeVar13 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

							} else {


								if ( ( nodeConst90 > 2.0 ) ) {

									nodeVar14 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

								} else {


									if ( ( nodeConst90 > 1.0 ) ) {

										nodeVar15 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

									} else {

										nodeVar15 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

									}

									nodeVar14 = nodeVar15;

								}

								nodeVar13 = nodeVar14;

							}

							nodeVar12 = nodeVar13;

						}

						nodeVar11 = nodeVar12;

					}

					let nodeConst91 = fract( ( ( ( nodeConst87 - nodeConst19 ) / ( nodeConst22 - nodeConst19 ) ).y * 4.0 ) );
					let nodeConst92 = fn5( ( nodeConst89 + 1u ) );
					let nodeConst93 = fn5( ( nodeVar2 + 119831u ) );
					let nodeConst94 = mix( vec3<f32>( 0.14702726648767014, 0.10224173307914941, 0.05612849004241121 ), vec3<f32>( 0.623960391667596, 0.6038273388475408, 0.55201140150344 ), step( 0.5, nodeConst93 ) );
					nodeVar10 = ( mix( mix( vec3<f32>( 0.015208514418949472, 0.012983032338510335, 0.010960094003125918 ), nodeVar11, ( ( ( step( 0.08, nodeConst91 ) * step( nodeConst91, mix( 0.4, 0.85, fract( ( nodeConst92 * 5.0 ) ) ) ) ) * step( abs( ( fract( ( nodeConst88 * 8.0 ) ) - 0.5 ) ), mix( 0.26, 0.46, fract( ( nodeConst92 * 13.0 ) ) ) ) ) * step( 0.15, nodeConst92 ) ) ), nodeConst94, step( nodeConst91, 0.08 ) ) * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeConst17 ), clamp( ( ( nodeConst87.z - 0.1 ) / nodeConst4 ), 0.0, 1.0 ) ) ) );

				} else {


					if ( nodeConst84 ) {

						let nodeConst93 = fn5( ( nodeVar2 + 119831u ) );
						let nodeConst95 = ( nodeConst9 + ( nodeConst11 * vec3<f32>( nodeConst37 ) ) );

						if ( ( ( ( nodeConst95 - nodeConst31 ) / ( nodeConst33 - nodeConst31 ) ).y > 0.92 ) ) {

							nodeVar17 = 1.3;

						} else {

							nodeVar17 = 0.85;

						}

						nodeVar16 = ( ( mix( vec3<f32>( 0.06847816983662762, 0.05126945836711539, 0.036889450395083165 ), vec3<f32>( 0.025186859622305935, 0.033104766565152086, 0.04231141061442144 ), nodeConst93 ) * vec3<f32>( nodeVar17 ) ) * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeConst17 ), clamp( ( ( nodeConst95.z - 0.1 ) / nodeConst4 ), 0.0, 1.0 ) ) ) );

					} else {


						if ( nodeConst83 ) {

							let nodeConst96 = ( nodeConst9 + ( nodeConst11 * vec3<f32>( nodeConst48 ) ) );

							if ( ( ( ( nodeConst96 - nodeConst41 ) / ( nodeConst44 - nodeConst41 ) ).y > 0.6 ) ) {

								let nodeConst97 = ( nodeConst43 * 6.0 );

								if ( ( nodeConst97 > 5.0 ) ) {

									nodeVar20 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

								} else {


									if ( ( nodeConst97 > 4.0 ) ) {

										nodeVar21 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

									} else {


										if ( ( nodeConst97 > 3.0 ) ) {

											nodeVar22 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

										} else {


											if ( ( nodeConst97 > 2.0 ) ) {

												nodeVar23 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

											} else {


												if ( ( nodeConst97 > 1.0 ) ) {

													nodeVar24 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

												} else {

													nodeVar24 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

												}

												nodeVar23 = nodeVar24;

											}

											nodeVar22 = nodeVar23;

										}

										nodeVar21 = nodeVar22;

									}

									nodeVar20 = nodeVar21;

								}

								nodeVar19 = nodeVar20;

							} else {

								nodeVar19 = vec3<f32>( 0.7011018919268015, 0.6653872982754769, 0.5972017883558645 );

							}

							nodeVar18 = ( nodeVar19 * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeConst17 ), clamp( ( ( nodeConst96.z - 0.1 ) / nodeConst4 ), 0.0, 1.0 ) ) ) );

						} else {


							if ( nodeConst82 ) {

								let nodeConst98 = ( nodeConst9 + ( nodeConst11 * vec3<f32>( nodeConst59 ) ) );

								if ( ( ( ( nodeConst98 - nodeConst52 ) / ( nodeConst55 - nodeConst52 ) ).y > 0.6 ) ) {

									let nodeConst99 = ( nodeConst54 * 6.0 );

									if ( ( nodeConst99 > 5.0 ) ) {

										nodeVar27 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

									} else {


										if ( ( nodeConst99 > 4.0 ) ) {

											nodeVar28 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

										} else {


											if ( ( nodeConst99 > 3.0 ) ) {

												nodeVar29 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

											} else {


												if ( ( nodeConst99 > 2.0 ) ) {

													nodeVar30 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

												} else {


													if ( ( nodeConst99 > 1.0 ) ) {

														nodeVar31 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

													} else {

														nodeVar31 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

													}

													nodeVar30 = nodeVar31;

												}

												nodeVar29 = nodeVar30;

											}

											nodeVar28 = nodeVar29;

										}

										nodeVar27 = nodeVar28;

									}

									nodeVar26 = nodeVar27;

								} else {

									nodeVar26 = vec3<f32>( 0.7011018919268015, 0.6653872982754769, 0.5972017883558645 );

								}

								nodeVar25 = ( nodeVar26 * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeConst17 ), clamp( ( ( nodeConst98.z - 0.1 ) / nodeConst4 ), 0.0, 1.0 ) ) ) );

							} else {


								if ( nodeConst81 ) {

									let nodeConst100 = ( nodeConst9 + ( nodeConst11 * vec3<f32>( nodeConst70 ) ) );

									if ( ( ( ( nodeConst100 - nodeConst63 ) / ( nodeConst66 - nodeConst63 ) ).y > 0.6 ) ) {

										let nodeConst101 = ( nodeConst65 * 6.0 );

										if ( ( nodeConst101 > 5.0 ) ) {

											nodeVar34 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

										} else {


											if ( ( nodeConst101 > 4.0 ) ) {

												nodeVar35 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

											} else {


												if ( ( nodeConst101 > 3.0 ) ) {

													nodeVar36 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

												} else {


													if ( ( nodeConst101 > 2.0 ) ) {

														nodeVar37 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

													} else {


														if ( ( nodeConst101 > 1.0 ) ) {

															nodeVar38 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

														} else {

															nodeVar38 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

														}

														nodeVar37 = nodeVar38;

													}

													nodeVar36 = nodeVar37;

												}

												nodeVar35 = nodeVar36;

											}

											nodeVar34 = nodeVar35;

										}

										nodeVar33 = nodeVar34;

									} else {

										nodeVar33 = vec3<f32>( 0.7011018919268015, 0.6653872982754769, 0.5972017883558645 );

									}

									nodeVar32 = ( nodeVar33 * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeConst17 ), clamp( ( ( nodeConst100.z - 0.1 ) / nodeConst4 ), 0.0, 1.0 ) ) ) );

								} else {


									if ( nodeConst80 ) {

										let nodeConst102 = ( nodeConst9 + ( nodeConst11 * vec3<f32>( nodeConst77 ) ) );

										if ( ( ( ( nodeConst102 - nodeConst71 ) / ( nodeConst73 - nodeConst71 ) ).y > 0.93 ) ) {

											nodeVar40 = vec3<f32>( 0.6938717612856897, 0.6583748172725346, 0.5775804404214573 );

										} else {

											nodeVar40 = vec3<f32>( 0.035601314869097636, 0.02955683443236377, 0.023153366173251363 );

										}

										nodeVar39 = ( nodeVar40 * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeConst17 ), clamp( ( ( nodeConst102.z - 0.1 ) / nodeConst4 ), 0.0, 1.0 ) ) ) );

									} else {

										let nodeConst103 = ( nodeConst9 + ( nodeConst11 * vec3<f32>( nodeConst79 ) ) );
										let nodeConst104 = ( ( nodeConst103 - nodeConst6 ) / ( nodeConst5 - nodeConst6 ) );
										let nodeConst105 = ( nodeConst104.z > 0.998 );

										if ( nodeConst105 ) {


											if ( bool( nodeConst27 ) ) {

												let nodeConst106 = ( nodeVarying11.x * 2.5 );
												let nodeConst107 = ( ( ( nodeVar2 + 31u ) + ( u32( floor( ( nodeConst104.x * nodeConst106 ) ) ) * 197u ) ) + ( u32( floor( ( nodeConst104.y * 4.0 ) ) ) * 4099u ) );
												let nodeConst108 = ( fn5( nodeConst107 ) * 6.0 );

												if ( ( nodeConst108 > 5.0 ) ) {

													nodeVar43 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

												} else {


													if ( ( nodeConst108 > 4.0 ) ) {

														nodeVar44 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

													} else {


														if ( ( nodeConst108 > 3.0 ) ) {

															nodeVar45 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

														} else {


															if ( ( nodeConst108 > 2.0 ) ) {

																nodeVar46 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

															} else {


																if ( ( nodeConst108 > 1.0 ) ) {

																	nodeVar47 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

																} else {

																	nodeVar47 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

																}

																nodeVar46 = nodeVar47;

															}

															nodeVar45 = nodeVar46;

														}

														nodeVar44 = nodeVar45;

													}

													nodeVar43 = nodeVar44;

												}

												let nodeConst109 = fract( ( nodeConst104.y * 4.0 ) );
												let nodeConst110 = fn5( ( nodeConst107 + 1u ) );
												let nodeConst93 = fn5( ( nodeVar2 + 119831u ) );
												let nodeConst94 = mix( vec3<f32>( 0.14702726648767014, 0.10224173307914941, 0.05612849004241121 ), vec3<f32>( 0.623960391667596, 0.6038273388475408, 0.55201140150344 ), step( 0.5, nodeConst93 ) );
												nodeVar42 = mix( mix( vec3<f32>( 0.015208514418949472, 0.012983032338510335, 0.010960094003125918 ), nodeVar43, ( ( ( step( 0.08, nodeConst109 ) * step( nodeConst109, mix( 0.4, 0.85, fract( ( nodeConst110 * 5.0 ) ) ) ) ) * step( abs( ( fract( ( nodeConst104.x * nodeConst106 ) ) - 0.5 ) ), mix( 0.26, 0.46, fract( ( nodeConst110 * 13.0 ) ) ) ) ) * step( 0.15, nodeConst110 ) ) ), nodeConst94, step( nodeConst109, 0.08 ) );

											} else {

												let nodeConst93 = fn5( ( nodeVar2 + 119831u ) );
												let nodeConst111 = ( nodeConst93 * 6.0 );

												if ( ( nodeConst111 > 5.0 ) ) {

													nodeVar48 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

												} else {


													if ( ( nodeConst111 > 4.0 ) ) {

														nodeVar49 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

													} else {


														if ( ( nodeConst111 > 3.0 ) ) {

															nodeVar50 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

														} else {


															if ( ( nodeConst111 > 2.0 ) ) {

																nodeVar51 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

															} else {


																if ( ( nodeConst111 > 1.0 ) ) {

																	nodeVar52 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

																} else {

																	nodeVar52 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

																}

																nodeVar51 = nodeVar52;

															}

															nodeVar50 = nodeVar51;

														}

														nodeVar49 = nodeVar50;

													}

													nodeVar48 = nodeVar49;

												}

												let nodeConst112 = mix( vec3<f32>( 0.6866853124288864, 0.6444796819634361, 0.5647115056965487 ), vec3<f32>( 0.5209955731953768, 0.48514994004665124, 0.4232676699760063 ), nodeConst28 );
												let nodeConst113 = mix( 0.3, 0.7, nodeConst28 );
												let nodeConst114 = ( fn5( ( nodeVar2 + 41260u ) ) * 6.0 );

												if ( ( nodeConst114 > 5.0 ) ) {

													nodeVar53 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

												} else {


													if ( ( nodeConst114 > 4.0 ) ) {

														nodeVar54 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

													} else {


														if ( ( nodeConst114 > 3.0 ) ) {

															nodeVar55 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

														} else {


															if ( ( nodeConst114 > 2.0 ) ) {

																nodeVar56 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

															} else {


																if ( ( nodeConst114 > 1.0 ) ) {

																	nodeVar57 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

																} else {

																	nodeVar57 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

																}

																nodeVar56 = nodeVar57;

															}

															nodeVar55 = nodeVar56;

														}

														nodeVar54 = nodeVar55;

													}

													nodeVar53 = nodeVar54;

												}

												nodeVar42 = mix( mix( mix( nodeVar48, nodeConst112, 0.45 ), vec3<f32>( 0.8069522576650873, 0.775822218312646, 0.7011018919268015 ), ( smoothstep( 0.10600000000000001, 0.094, abs( ( nodeConst104.x - nodeConst113 ) ) ) * smoothstep( 0.14600000000000002, 0.134, abs( ( nodeConst104.y - 0.62 ) ) ) ) ), nodeVar53, ( ( smoothstep( 0.081, 0.06899999999999999, abs( ( nodeConst104.x - nodeConst113 ) ) ) * smoothstep( 0.10600000000000001, 0.094, abs( ( nodeConst104.y - 0.62 ) ) ) ) * 0.85 ) );

											}

											nodeVar41 = nodeVar42;

										} else {

											let nodeConst115 = ( nodeConst104.y > 0.998 );

											if ( nodeConst115 ) {


												if ( ( fn5( ( nodeVar2 + 25910u ) ) > 0.65 ) ) {

													nodeVar59 = mix( vec3<f32>( 1.0, 0.6938717612856897, 0.3515325994898463 ), vec3<f32>( 1.0, 0.8227857543924378, 0.5972017883558645 ), fn5( ( nodeVar2 + 86350u ) ) );

												} else {

													nodeVar59 = mix( vec3<f32>( 0.85499260812105, 0.8879231178794776, 0.9215818562755338 ), vec3<f32>( 0.7379104087672317, 0.8069522576650873, 1.0 ), fn5( ( nodeVar2 + 59550u ) ) );

												}

												nodeVar58 = mix( vec3<f32>( 0.8148465722120952, 0.7912979403281551, 0.7379104087672317 ), ( nodeVar59 * vec3<f32>( mix( 0.5, 6.0, nodeConst17 ) ) ), ( step( abs( ( fract( ( nodeConst104.x * 3.0 ) ) - 0.5 ) ), 0.3 ) * step( abs( ( fract( ( nodeConst104.z * 4.0 ) ) - 0.5 ) ), 0.15 ) ) );

											} else {

												let nodeConst116 = ( nodeConst104.y < 0.002 );

												if ( nodeConst116 ) {


													if ( ( fn5( ( nodeVar2 + 27510u ) ) > 0.75 ) ) {

														nodeVar61 = mix( vec3<f32>( 0.01764195448412081, 0.019382360952473074, 0.023153366173251363 ), vec3<f32>( 0.623960391667596, 0.6038273388475408, 0.5394794890033748 ), tsl_mod_float( ( floor( ( nodeConst104.x * 8.0 ) ) + floor( ( nodeConst104.z * 10.0 ) ) ), 2.0 ) );

													} else {

														let nodeConst93 = fn5( ( nodeVar2 + 119831u ) );
														nodeVar61 = ( mix( vec3<f32>( 0.46207699964472876, 0.4286904966038916, 0.36130677977297226 ), vec3<f32>( 0.3005437944049895, 0.2746773120495699, 0.22696587349938613 ), nodeConst93 ) * vec3<f32>( ( 1.0 - ( max( step( 0.93, fract( ( nodeConst104.x * 8.0 ) ) ), step( 0.93, fract( ( nodeConst104.z * 10.0 ) ) ) ) * 0.35 ) ) ) );

													}

													nodeVar60 = nodeVar61;

												} else {


													if ( bool( nodeConst27 ) ) {

														let nodeConst117 = ( nodeConst4 * 2.5 );
														let nodeConst118 = ( ( ( nodeVar2 + 57u ) + ( u32( floor( ( nodeConst104.z * nodeConst117 ) ) ) * 197u ) ) + ( u32( floor( ( nodeConst104.y * 4.0 ) ) ) * 4099u ) );
														let nodeConst119 = ( fn5( nodeConst118 ) * 6.0 );

														if ( ( nodeConst119 > 5.0 ) ) {

															nodeVar63 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

														} else {


															if ( ( nodeConst119 > 4.0 ) ) {

																nodeVar64 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

															} else {


																if ( ( nodeConst119 > 3.0 ) ) {

																	nodeVar65 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

																} else {


																	if ( ( nodeConst119 > 2.0 ) ) {

																		nodeVar66 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

																	} else {


																		if ( ( nodeConst119 > 1.0 ) ) {

																			nodeVar67 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

																		} else {

																			nodeVar67 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

																		}

																		nodeVar66 = nodeVar67;

																	}

																	nodeVar65 = nodeVar66;

																}

																nodeVar64 = nodeVar65;

															}

															nodeVar63 = nodeVar64;

														}

														let nodeConst120 = fract( ( nodeConst104.y * 4.0 ) );
														let nodeConst121 = fn5( ( nodeConst118 + 1u ) );
														let nodeConst93 = fn5( ( nodeVar2 + 119831u ) );
														let nodeConst94 = mix( vec3<f32>( 0.14702726648767014, 0.10224173307914941, 0.05612849004241121 ), vec3<f32>( 0.623960391667596, 0.6038273388475408, 0.55201140150344 ), step( 0.5, nodeConst93 ) );
														nodeVar62 = mix( mix( vec3<f32>( 0.015208514418949472, 0.012983032338510335, 0.010960094003125918 ), nodeVar63, ( ( ( step( 0.08, nodeConst120 ) * step( nodeConst120, mix( 0.4, 0.85, fract( ( nodeConst121 * 5.0 ) ) ) ) ) * step( abs( ( fract( ( nodeConst104.z * nodeConst117 ) ) - 0.5 ) ), mix( 0.26, 0.46, fract( ( nodeConst121 * 13.0 ) ) ) ) ) * step( 0.15, nodeConst121 ) ) ), nodeConst94, step( nodeConst120, 0.08 ) );

													} else {

														let nodeConst112 = mix( vec3<f32>( 0.6866853124288864, 0.6444796819634361, 0.5647115056965487 ), vec3<f32>( 0.5209955731953768, 0.48514994004665124, 0.4232676699760063 ), nodeConst28 );
														nodeVar62 = nodeConst112;

													}

													nodeVar60 = nodeVar62;

												}

												nodeVar58 = nodeVar60;

											}

											nodeVar41 = nodeVar58;

										}


										if ( nodeConst105 ) {

											nodeVar68 = ( ( smoothstep( 0.0, 0.15, nodeConst104.x ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst104.x ) ) ) * ( smoothstep( 0.0, 0.15, nodeConst104.y ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst104.y ) ) ) );

										} else {

											let nodeConst116 = ( nodeConst104.y < 0.002 );
											let nodeConst115 = ( nodeConst104.y > 0.998 );

											if ( ( nodeConst116 || nodeConst115 ) ) {

												nodeVar69 = ( ( smoothstep( 0.0, 0.15, nodeConst104.x ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst104.x ) ) ) * ( smoothstep( 0.0, 0.15, nodeConst104.z ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst104.z ) ) ) );

											} else {

												nodeVar69 = ( ( smoothstep( 0.0, 0.15, nodeConst104.y ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst104.y ) ) ) * ( smoothstep( 0.0, 0.15, nodeConst104.z ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst104.z ) ) ) );

											}

											nodeVar68 = nodeVar69;

										}

										nodeVar39 = ( ( nodeVar41 * vec3<f32>( mix( 0.72, 1.0, nodeVar68 ) ) ) * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeConst17 ), clamp( ( ( nodeConst103.z - 0.1 ) / nodeConst4 ), 0.0, 1.0 ) ) ) );

									}

									nodeVar32 = nodeVar39;

								}

								nodeVar25 = nodeVar32;

							}

							nodeVar18 = nodeVar25;

						}

						nodeVar16 = nodeVar18;

					}

					nodeVar10 = nodeVar16;

				}

				nodeVar3 = nodeVar10;

			}


			if ( ( fn5( ( nodeVar2 + 25910u ) ) > 0.65 ) ) {

				nodeVar70 = mix( vec3<f32>( 1.0, 0.6938717612856897, 0.3515325994898463 ), vec3<f32>( 1.0, 0.8227857543924378, 0.5972017883558645 ), fn5( ( nodeVar2 + 86350u ) ) );

			} else {

				nodeVar70 = mix( vec3<f32>( 0.85499260812105, 0.8879231178794776, 0.9215818562755338 ), vec3<f32>( 0.7379104087672317, 0.8069522576650873, 1.0 ), fn5( ( nodeVar2 + 59550u ) ) );

			}


			if ( nodeConst86 ) {

				nodeVar71 = 0.0;

			} else {


				if ( nodeConst85 ) {

					nodeVar72 = 1.0;

				} else {


					if ( nodeConst84 ) {

						nodeVar73 = 1.0;

					} else {


						if ( nodeConst83 ) {

							nodeVar74 = 1.0;

						} else {


							if ( nodeConst82 ) {

								nodeVar75 = 1.0;

							} else {


								if ( nodeConst81 ) {

									nodeVar76 = 1.0;

								} else {


									if ( nodeConst80 ) {

										nodeVar77 = 1.0;

									} else {

										nodeVar77 = 1.0;

									}

									nodeVar76 = nodeVar77;

								}

								nodeVar75 = nodeVar76;

							}

							nodeVar74 = nodeVar75;

						}

						nodeVar73 = nodeVar74;

					}

					nodeVar72 = nodeVar73;

				}

				nodeVar71 = nodeVar72;

			}

			nodeVar1 = vec4<f32>( ( ( nodeVar3 * mix( vec3<f32>( 1.0, 1.0, 1.0 ), nodeVar70, ( nodeConst17 * 0.6 ) ) ) * vec3<f32>( mix( 0.45, 1.5, nodeConst17 ) ) ), ( nodeConst17 * nodeVar71 ) );

		} else {

			let nodeConst122 = floor( ( nodeVarying4 * vec3<f32>( 2.0 ) ) );
			nodeVar78 = ( ( ( u32( ( nodeConst122.x + 2097152.0 ) ) * 73856093u ) ^ ( u32( ( nodeConst122.y + 2097152.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeConst122.z + 2097152.0 ) ) * 83492791u ) );
			let nodeConst123 = vec3<f32>( ( nodeVarying11.x * 0.5 ), ( nodeVarying11.y * 0.5 ), ( 0.1 + ( nodeVarying11.y * 1.55 ) ) );
			let nodeConst124 = fn5( ( nodeVar78 + 119831u ) );
			let nodeConst125 = smoothstep( 0.3, 1.0, nodeConst124 );
			let nodeConst126 = ( nodeConst123.x * ( nodeConst125 * nodeConst125 ) );
			let nodeConst127 = vec3<f32>( ( - nodeConst123.x ), ( - nodeConst123.y ), 0.1 );
			let nodeConst128 = ( positionLocal - nodeVarying4 );
			normalLocal = nodeVarying12;
			let nodeConst129 = normalize( cross( vec3<f32>( 0.0, 1.0, 0.0 ), normalLocal ) );
			let nodeConst130 = vec3<f32>( dot( nodeConst128, nodeConst129 ), nodeConst128.y, 0.0 );
			let nodeConst131 = normalize( ( positionLocal - ( object.nodeUniform0 * vec4<f32>( render.cameraPosition, 1.0 ) ).xyz ) );
			let nodeConst132 = vec3<f32>( dot( nodeConst131, nodeConst129 ), nodeConst131.y, ( - dot( nodeConst131, normalLocal ) ) );
			let nodeConst133 = ( ( vec3<f32>( ( nodeConst123.x - nodeConst126 ), nodeConst127.y, 0.1 ) - nodeConst130 ) / nodeConst132 );
			let nodeConst134 = ( 0.1 + 0.12 );
			let nodeConst135 = ( ( vec3<f32>( nodeConst123.x, nodeConst123.y, nodeConst134 ) - nodeConst130 ) / nodeConst132 );
			let nodeConst136 = max( nodeConst133, nodeConst135 );
			let nodeConst137 = min( nodeConst133, nodeConst135 );
			let nodeConst138 = max( max( nodeConst137.x, nodeConst137.y ), nodeConst137.z );
			let nodeConst139 = ( ( vec3<f32>( ( - nodeConst123.x ), nodeConst127.y, 0.1 ) - nodeConst130 ) / nodeConst132 );
			let nodeConst140 = fn5( ( nodeVar78 + 105097u ) );
			let nodeConst141 = smoothstep( 0.3, 1.0, nodeConst140 );
			let nodeConst142 = ( nodeConst123.x * ( nodeConst141 * nodeConst141 ) );
			let nodeConst143 = ( ( vec3<f32>( ( ( - nodeConst123.x ) + nodeConst142 ), nodeConst123.y, nodeConst134 ) - nodeConst130 ) / nodeConst132 );
			let nodeConst144 = max( nodeConst139, nodeConst143 );
			let nodeConst145 = min( nodeConst139, nodeConst143 );
			let nodeConst146 = max( max( nodeConst145.x, nodeConst145.y ), nodeConst145.z );
			let nodeConst147 = ( nodeConst123.x * 0.82 );
			let nodeConst148 = vec3<f32>( ( nodeConst147 - 0.5 ), nodeConst127.y, ( nodeConst123.z - 0.7 ) );
			let nodeConst149 = ( ( nodeConst148 - nodeConst130 ) / nodeConst132 );
			let nodeConst150 = vec3<f32>( ( nodeConst147 + 0.5 ), ( nodeConst127.y + mix( 1.7, 2.3, nodeConst124 ) ), ( nodeConst123.z - 0.1 ) );
			let nodeConst151 = ( ( nodeConst150 - nodeConst130 ) / nodeConst132 );
			let nodeConst152 = max( nodeConst149, nodeConst151 );
			let nodeConst153 = min( nodeConst149, nodeConst151 );
			let nodeConst154 = max( max( nodeConst153.x, nodeConst153.y ), nodeConst153.z );
			let nodeConst155 = ( nodeConst123.x * -0.82 );
			let nodeConst156 = vec3<f32>( ( nodeConst155 - 0.5 ), nodeConst127.y, ( nodeConst123.z - 0.7 ) );
			let nodeConst157 = ( ( nodeConst156 - nodeConst130 ) / nodeConst132 );
			let nodeConst158 = vec3<f32>( ( nodeConst155 + 0.5 ), ( nodeConst127.y + mix( 1.7, 2.3, nodeConst140 ) ), ( nodeConst123.z - 0.1 ) );
			let nodeConst159 = ( ( nodeConst158 - nodeConst130 ) / nodeConst132 );
			let nodeConst160 = max( nodeConst157, nodeConst159 );
			let nodeConst161 = min( nodeConst157, nodeConst159 );
			let nodeConst162 = max( max( nodeConst161.x, nodeConst161.y ), nodeConst161.z );
			let nodeConst163 = mix( ( nodeConst123.x * -0.3 ), ( nodeConst123.x * 0.3 ), nodeConst124 );
			let nodeConst164 = vec3<f32>( ( nodeConst163 - 1.1 ), nodeConst127.y, ( nodeConst123.z - 0.95 ) );
			let nodeConst165 = ( ( nodeConst164 - nodeConst130 ) / nodeConst132 );
			let nodeConst166 = vec3<f32>( ( nodeConst163 + 1.1 ), ( nodeConst127.y + mix( 0.8, 0.9, nodeConst140 ) ), ( nodeConst123.z - 0.1 ) );
			let nodeConst167 = ( ( nodeConst166 - nodeConst130 ) / nodeConst132 );
			let nodeConst168 = max( nodeConst165, nodeConst167 );
			let nodeConst169 = min( nodeConst165, nodeConst167 );
			let nodeConst170 = max( max( nodeConst169.x, nodeConst169.y ), nodeConst169.z );
			let nodeConst171 = mix( -0.6, 0.6, nodeConst140 );
			let nodeConst172 = ( nodeVarying11.y * 1.55 );
			let nodeConst173 = ( ( 0.1 + ( nodeConst172 * 0.5 ) ) + mix( -0.4, 0.5, nodeConst124 ) );
			let nodeConst174 = vec3<f32>( ( nodeConst171 - 0.6 ), nodeConst127.y, ( nodeConst173 - 0.35 ) );
			let nodeConst175 = ( ( nodeConst174 - nodeConst130 ) / nodeConst132 );
			let nodeConst176 = vec3<f32>( ( nodeConst171 + 0.6 ), ( nodeConst127.y + 0.42 ), ( nodeConst173 + 0.35 ) );
			let nodeConst177 = ( ( nodeConst176 - nodeConst130 ) / nodeConst132 );
			let nodeConst178 = max( nodeConst175, nodeConst177 );
			let nodeConst179 = min( nodeConst175, nodeConst177 );
			let nodeConst180 = max( max( nodeConst179.x, nodeConst179.y ), nodeConst179.z );
			let nodeConst181 = max( ( ( nodeConst127 - nodeConst130 ) / nodeConst132 ), ( ( nodeConst123 - nodeConst130 ) / nodeConst132 ) );
			let nodeConst182 = min( min( nodeConst181.x, nodeConst181.y ), nodeConst181.z );
			let nodeConst183 = ( ( ( min( min( nodeConst178.x, nodeConst178.y ), nodeConst178.z ) > nodeConst180 ) && ( nodeConst180 > 0.0 ) ) && ( nodeConst180 < nodeConst182 ) );

			if ( nodeConst183 ) {

				nodeVar84 = nodeConst180;

			} else {

				nodeVar84 = nodeConst182;

			}

			let nodeConst184 = ( ( ( min( min( nodeConst168.x, nodeConst168.y ), nodeConst168.z ) > nodeConst170 ) && ( nodeConst170 > 0.0 ) ) && ( nodeConst170 < nodeVar84 ) );

			if ( nodeConst184 ) {

				nodeVar83 = nodeConst170;

			} else {

				nodeVar83 = nodeVar84;

			}

			let nodeConst185 = ( ( ( ( min( min( nodeConst160.x, nodeConst160.y ), nodeConst160.z ) > nodeConst162 ) && ( nodeConst162 > 0.0 ) ) && ( fn5( ( nodeVar78 + 15070u ) ) > 0.4 ) ) && ( nodeConst162 < nodeVar83 ) );

			if ( nodeConst185 ) {

				nodeVar82 = nodeConst162;

			} else {

				nodeVar82 = nodeVar83;

			}

			let nodeConst186 = ( ( ( ( min( min( nodeConst152.x, nodeConst152.y ), nodeConst152.z ) > nodeConst154 ) && ( nodeConst154 > 0.0 ) ) && ( fn5( ( nodeVar78 + 8200u ) ) > 0.4 ) ) && ( nodeConst154 < nodeVar82 ) );

			if ( nodeConst186 ) {

				nodeVar81 = nodeConst154;

			} else {

				nodeVar81 = nodeVar82;

			}

			let nodeConst187 = ( ( ( ( min( min( nodeConst144.x, nodeConst144.y ), nodeConst144.z ) > nodeConst146 ) && ( nodeConst146 > 0.0 ) ) && ( nodeConst142 > 0.05 ) ) && ( nodeConst146 < nodeVar81 ) );

			if ( nodeConst187 ) {

				nodeVar80 = nodeConst146;

			} else {

				nodeVar80 = nodeVar81;

			}

			let nodeConst188 = ( ( ( ( min( min( nodeConst136.x, nodeConst136.y ), nodeConst136.z ) > nodeConst138 ) && ( nodeConst138 > 0.0 ) ) && ( nodeConst126 > 0.05 ) ) && ( nodeConst138 < nodeVar80 ) );

			if ( nodeConst188 ) {

				let nodeConst189 = ( fn5( ( nodeVar78 + 125490u ) ) * 6.0 );

				if ( ( nodeConst189 > 5.0 ) ) {

					nodeVar85 = mix( vec3<f32>( 0.26225065751888765, 0.10224173307914941, 0.057805430183792694 ), vec3<f32>( 0.3231432091022285, 0.14412847084818123, 0.08437621153575764 ), nodeConst124 );

				} else {


					if ( ( nodeConst189 > 4.0 ) ) {

						nodeVar86 = mix( vec3<f32>( 0.1499597898006365, 0.17788841597328695, 0.09758734713304495 ), vec3<f32>( 0.1912016827303171, 0.22696587349938613, 0.11443537381770343 ), nodeConst124 );

					} else {


						if ( ( nodeConst189 > 3.0 ) ) {

							nodeVar87 = mix( vec3<f32>( 0.11443537381770343, 0.16202937562896222, 0.1912016827303171 ), vec3<f32>( 0.158960835050774, 0.21952619971859377, 0.25818285291079235 ), nodeConst124 );

						} else {


							if ( ( nodeConst189 > 2.0 ) ) {

								nodeVar88 = mix( vec3<f32>( 0.16202937562896222, 0.14412847084818123, 0.12743768042608497 ), vec3<f32>( 0.22696587349938613, 0.20507873637973145, 0.181164244239483 ), nodeConst124 );

							} else {


								if ( ( nodeConst189 > 1.0 ) ) {

									nodeVar89 = mix( vec3<f32>( 0.2541520943200296, 0.19461783043107173, 0.12743768042608497 ), vec3<f32>( 0.3277780980458375, 0.26225065751888765, 0.16826940017946088 ), nodeConst124 );

								} else {

									nodeVar89 = mix( vec3<f32>( 0.5906188409113381, 0.5209955731953768, 0.3813260114221238 ), vec3<f32>( 0.6866853124288864, 0.6104955708001716, 0.4793201830913402 ), nodeConst124 );

								}

								nodeVar88 = nodeVar89;

							}

							nodeVar87 = nodeVar88;

						}

						nodeVar86 = nodeVar87;

					}

					nodeVar85 = nodeVar86;

				}

				let nodeConst190 = ( nodeConst130 + ( nodeConst132 * vec3<f32>( nodeConst138 ) ) );
				nodeVar79 = ( ( nodeVar85 * vec3<f32>( mix( 0.78, 1.12, fract( ( nodeConst190.x * 2.5 ) ) ) ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeConst190.z - 0.1 ) / nodeConst172 ), 0.0, 1.0 ) ) ) );

			} else {


				if ( nodeConst187 ) {

					let nodeConst189 = ( fn5( ( nodeVar78 + 125490u ) ) * 6.0 );

					if ( ( nodeConst189 > 5.0 ) ) {

						nodeVar91 = mix( vec3<f32>( 0.26225065751888765, 0.10224173307914941, 0.057805430183792694 ), vec3<f32>( 0.3231432091022285, 0.14412847084818123, 0.08437621153575764 ), nodeConst124 );

					} else {


						if ( ( nodeConst189 > 4.0 ) ) {

							nodeVar92 = mix( vec3<f32>( 0.1499597898006365, 0.17788841597328695, 0.09758734713304495 ), vec3<f32>( 0.1912016827303171, 0.22696587349938613, 0.11443537381770343 ), nodeConst124 );

						} else {


							if ( ( nodeConst189 > 3.0 ) ) {

								nodeVar93 = mix( vec3<f32>( 0.11443537381770343, 0.16202937562896222, 0.1912016827303171 ), vec3<f32>( 0.158960835050774, 0.21952619971859377, 0.25818285291079235 ), nodeConst124 );

							} else {


								if ( ( nodeConst189 > 2.0 ) ) {

									nodeVar94 = mix( vec3<f32>( 0.16202937562896222, 0.14412847084818123, 0.12743768042608497 ), vec3<f32>( 0.22696587349938613, 0.20507873637973145, 0.181164244239483 ), nodeConst124 );

								} else {


									if ( ( nodeConst189 > 1.0 ) ) {

										nodeVar95 = mix( vec3<f32>( 0.2541520943200296, 0.19461783043107173, 0.12743768042608497 ), vec3<f32>( 0.3277780980458375, 0.26225065751888765, 0.16826940017946088 ), nodeConst124 );

									} else {

										nodeVar95 = mix( vec3<f32>( 0.5906188409113381, 0.5209955731953768, 0.3813260114221238 ), vec3<f32>( 0.6866853124288864, 0.6104955708001716, 0.4793201830913402 ), nodeConst124 );

									}

									nodeVar94 = nodeVar95;

								}

								nodeVar93 = nodeVar94;

							}

							nodeVar92 = nodeVar93;

						}

						nodeVar91 = nodeVar92;

					}

					let nodeConst191 = ( nodeConst130 + ( nodeConst132 * vec3<f32>( nodeConst146 ) ) );
					nodeVar90 = ( ( nodeVar91 * vec3<f32>( mix( 0.78, 1.12, fract( ( nodeConst191.x * 2.5 ) ) ) ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeConst191.z - 0.1 ) / nodeConst172 ), 0.0, 1.0 ) ) ) );

				} else {


					if ( nodeConst186 ) {

						let nodeConst192 = ( nodeConst130 + ( nodeConst132 * vec3<f32>( nodeConst154 ) ) );

						if ( ( ( ( nodeConst192 - nodeConst148 ) / ( nodeConst150 - nodeConst148 ) ).y > 0.94 ) ) {

							nodeVar97 = 1.2;

						} else {

							nodeVar97 = 0.82;

						}

						nodeVar96 = ( ( mix( vec3<f32>( 0.04231141061442144, 0.025186859622305935, 0.015996293361446288 ), vec3<f32>( 0.09084171117479915, 0.06301001764564068, 0.04231141061442144 ), nodeConst140 ) * vec3<f32>( nodeVar97 ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeConst192.z - 0.1 ) / nodeConst172 ), 0.0, 1.0 ) ) ) );

					} else {


						if ( nodeConst185 ) {

							let nodeConst193 = ( nodeConst130 + ( nodeConst132 * vec3<f32>( nodeConst162 ) ) );

							if ( ( ( ( nodeConst193 - nodeConst156 ) / ( nodeConst158 - nodeConst156 ) ).y > 0.94 ) ) {

								nodeVar99 = 1.2;

							} else {

								nodeVar99 = 0.82;

							}

							nodeVar98 = ( ( mix( vec3<f32>( 0.04231141061442144, 0.025186859622305935, 0.015996293361446288 ), vec3<f32>( 0.09084171117479915, 0.06301001764564068, 0.04231141061442144 ), nodeConst140 ) * vec3<f32>( nodeVar99 ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeConst193.z - 0.1 ) / nodeConst172 ), 0.0, 1.0 ) ) ) );

						} else {


							if ( nodeConst184 ) {

								let nodeConst194 = ( nodeConst130 + ( nodeConst132 * vec3<f32>( nodeConst170 ) ) );

								if ( ( ( ( nodeConst194 - nodeConst164 ) / ( nodeConst166 - nodeConst164 ) ).y > 0.9 ) ) {

									nodeVar101 = 1.12;

								} else {

									nodeVar101 = 0.85;

								}

								nodeVar100 = ( ( mix( vec3<f32>( 0.10224173307914941, 0.06847816983662762, 0.04231141061442144 ), vec3<f32>( 0.054480276435339814, 0.09305896283800832, 0.14412847084818123 ), nodeConst140 ) * vec3<f32>( nodeVar101 ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeConst194.z - 0.1 ) / nodeConst172 ), 0.0, 1.0 ) ) ) );

							} else {


								if ( nodeConst183 ) {

									let nodeConst195 = ( nodeConst130 + ( nodeConst132 * vec3<f32>( nodeConst180 ) ) );

									if ( ( ( ( nodeConst195 - nodeConst174 ) / ( nodeConst176 - nodeConst174 ) ).y > 0.94 ) ) {

										nodeVar103 = 1.25;

									} else {

										nodeVar103 = 0.8;

									}

									nodeVar102 = ( ( mix( vec3<f32>( 0.06847816983662762, 0.035601314869097636, 0.019382360952473074 ), vec3<f32>( 0.14702726648767014, 0.06847816983662762, 0.02955683443236377 ), nodeConst124 ) * vec3<f32>( nodeVar103 ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeConst195.z - 0.1 ) / nodeConst172 ), 0.0, 1.0 ) ) ) );

								} else {

									let nodeConst196 = ( nodeConst130 + ( nodeConst132 * vec3<f32>( nodeConst182 ) ) );
									let nodeConst197 = ( ( nodeConst196 - nodeConst127 ) / ( nodeConst123 - nodeConst127 ) );
									let nodeConst198 = ( nodeConst197.z > 0.998 );

									if ( nodeConst198 ) {

										let nodeConst199 = mix( mix( vec3<f32>( 0.3231432091022285, 0.25818285291079235, 0.171441100722554 ), vec3<f32>( 0.158960835050774, 0.19461783043107173, 0.22322795730611386 ), nodeConst140 ), vec3<f32>( 0.48514994004665124, 0.4178850708380236, 0.3094689228067428 ), ( nodeConst124 * 0.6 ) );
										let nodeConst200 = mix( nodeConst199, ( nodeConst199 * vec3<f32>( 0.5 ) ), smoothstep( 0.05, 0.04, nodeConst197.y ) );
										let nodeConst201 = mix( 0.22, 0.78, nodeConst140 );

										if ( ( nodeConst201 < 0.5 ) ) {

											nodeVar105 = mix( 0.68, 0.82, nodeConst124 );

										} else {

											nodeVar105 = mix( 0.18, 0.32, nodeConst124 );

										}

										nodeVar104 = mix( mix( mix( nodeConst200, mix( vec3<f32>( 0.10224173307914941, 0.061246054224174035, 0.030713443727452196 ), vec3<f32>( 0.040915196900556984, 0.03954623527052923, 0.04518620437910499 ), step( 0.5, nodeConst124 ) ), ( smoothstep( 0.09100000000000001, 0.079, abs( ( nodeConst197.x - nodeConst201 ) ) ) * smoothstep( 0.356, 0.344, abs( ( nodeConst197.y - 0.33 ) ) ) ) ), vec3<f32>( 0.0069954101845983935, 0.006048833020386069, 0.005181516700061659 ), ( smoothstep( 0.081, 0.06899999999999999, abs( ( nodeConst197.x - nodeVar105 ) ) ) * smoothstep( 0.09100000000000001, 0.079, abs( ( nodeConst197.y - 0.56 ) ) ) ) ), mix( vec3<f32>( 0.025186859622305935, 0.04231141061442144, 0.06847816983662762 ), vec3<f32>( 0.19461783043107173, 0.10224173307914941, 0.04231141061442144 ), fn5( ( nodeVar78 + 11240u ) ) ), ( smoothstep( 0.061, 0.049, abs( ( nodeConst197.x - nodeVar105 ) ) ) * smoothstep( 0.07100000000000001, 0.059000000000000004, abs( ( nodeConst197.y - 0.56 ) ) ) ) );

									} else {

										let nodeConst202 = ( nodeConst197.y > 0.998 );

										if ( nodeConst202 ) {

											let nodeConst199 = mix( mix( vec3<f32>( 0.3231432091022285, 0.25818285291079235, 0.171441100722554 ), vec3<f32>( 0.158960835050774, 0.19461783043107173, 0.22322795730611386 ), nodeConst140 ), vec3<f32>( 0.48514994004665124, 0.4178850708380236, 0.3094689228067428 ), ( nodeConst124 * 0.6 ) );

											if ( ( fn5( ( nodeVar78 + 25910u ) ) > 0.88 ) ) {

												nodeVar107 = mix( vec3<f32>( 0.7379104087672317, 0.8069522576650873, 1.0 ), vec3<f32>( 0.34670405634441115, 0.46778379610254284, 1.0 ), fn5( ( nodeVar78 + 59550u ) ) );

											} else {

												nodeVar107 = mix( vec3<f32>( 1.0, 0.4793201830913402, 0.059511238155621766 ), vec3<f32>( 1.0, 0.775822218312646, 0.33245153633549385 ), fn5( ( nodeVar78 + 86350u ) ) );

											}

											let nodeConst203 = step( 0.8, fn5( ( nodeVar78 + 79599u ) ) );
											nodeVar106 = mix( mix( nodeConst199, vec3<f32>( 1.0, 1.0, 1.0 ), 0.5 ), ( nodeVar107 * vec3<f32>( mix( 1.0, 4.5, nodeConst203 ) ) ), smoothstep( 0.16, 0.13, length( vec2<f32>( ( nodeConst197.x - 0.5 ), ( nodeConst197.z - 0.5 ) ) ) ) );

										} else {

											let nodeConst204 = ( nodeConst197.y < 0.002 );

											if ( nodeConst204 ) {

												nodeVar108 = mix( ( mix( vec3<f32>( 0.06847816983662762, 0.033104766565152086, 0.014443843592229466 ), vec3<f32>( 0.14412847084818123, 0.0722718506743852, 0.02955683443236377 ), nodeConst140 ) * vec3<f32>( ( 1.0 - ( step( 0.94, fract( ( nodeConst197.x * 6.0 ) ) ) * 0.3 ) ) ) ), mix( vec3<f32>( 0.19461783043107173, 0.04373502925049377, 0.031896033067374104 ), vec3<f32>( 0.04231141061442144, 0.09530746662221588, 0.11697066774917994 ), nodeConst124 ), ( ( smoothstep( 0.306, 0.294, abs( ( nodeConst197.x - 0.5 ) ) ) * smoothstep( 0.266, 0.254, abs( ( nodeConst197.z - 0.62 ) ) ) ) * 0.9 ) );

											} else {

												let nodeConst199 = mix( mix( vec3<f32>( 0.3231432091022285, 0.25818285291079235, 0.171441100722554 ), vec3<f32>( 0.158960835050774, 0.19461783043107173, 0.22322795730611386 ), nodeConst140 ), vec3<f32>( 0.48514994004665124, 0.4178850708380236, 0.3094689228067428 ), ( nodeConst124 * 0.6 ) );
												let nodeConst200 = mix( nodeConst199, ( nodeConst199 * vec3<f32>( 0.5 ) ), smoothstep( 0.05, 0.04, nodeConst197.y ) );
												nodeVar108 = nodeConst200;

											}

											nodeVar106 = nodeVar108;

										}

										nodeVar104 = nodeVar106;

									}


									if ( nodeConst198 ) {

										nodeVar109 = ( ( smoothstep( 0.0, 0.15, nodeConst197.x ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst197.x ) ) ) * ( smoothstep( 0.0, 0.15, nodeConst197.y ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst197.y ) ) ) );

									} else {

										let nodeConst204 = ( nodeConst197.y < 0.002 );
										let nodeConst202 = ( nodeConst197.y > 0.998 );

										if ( ( nodeConst204 || nodeConst202 ) ) {

											nodeVar110 = ( ( smoothstep( 0.0, 0.15, nodeConst197.x ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst197.x ) ) ) * ( smoothstep( 0.0, 0.15, nodeConst197.z ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst197.z ) ) ) );

										} else {

											nodeVar110 = ( ( smoothstep( 0.0, 0.15, nodeConst197.y ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst197.y ) ) ) * ( smoothstep( 0.0, 0.15, nodeConst197.z ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst197.z ) ) ) );

										}

										nodeVar109 = nodeVar110;

									}

									nodeVar102 = ( ( nodeVar104 * vec3<f32>( mix( 0.72, 1.0, nodeVar109 ) ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeConst196.z - 0.1 ) / nodeConst172 ), 0.0, 1.0 ) ) ) );

								}

								nodeVar100 = nodeVar102;

							}

							nodeVar98 = nodeVar100;

						}

						nodeVar96 = nodeVar98;

					}

					nodeVar90 = nodeVar96;

				}

				nodeVar79 = nodeVar90;

			}


			if ( ( fn5( ( nodeVar78 + 25910u ) ) > 0.88 ) ) {

				nodeVar111 = mix( vec3<f32>( 0.7379104087672317, 0.8069522576650873, 1.0 ), vec3<f32>( 0.34670405634441115, 0.46778379610254284, 1.0 ), fn5( ( nodeVar78 + 59550u ) ) );

			} else {

				nodeVar111 = mix( vec3<f32>( 1.0, 0.4793201830913402, 0.059511238155621766 ), vec3<f32>( 1.0, 0.775822218312646, 0.33245153633549385 ), fn5( ( nodeVar78 + 86350u ) ) );

			}

			let nodeConst203 = step( 0.8, fn5( ( nodeVar78 + 79599u ) ) );

			if ( nodeConst188 ) {

				nodeVar112 = 0.2;

			} else {


				if ( nodeConst187 ) {

					nodeVar113 = 0.2;

				} else {


					if ( nodeConst186 ) {

						nodeVar114 = 1.0;

					} else {


						if ( nodeConst185 ) {

							nodeVar115 = 1.0;

						} else {


							if ( nodeConst184 ) {

								nodeVar116 = 1.0;

							} else {


								if ( nodeConst183 ) {

									nodeVar117 = 1.0;

								} else {

									nodeVar117 = 1.0;

								}

								nodeVar116 = nodeVar117;

							}

							nodeVar115 = nodeVar116;

						}

						nodeVar114 = nodeVar115;

					}

					nodeVar113 = nodeVar114;

				}

				nodeVar112 = nodeVar113;

			}

			nodeVar1 = vec4<f32>( ( ( nodeVar79 * mix( vec3<f32>( 1.0, 1.0, 1.0 ), nodeVar111, ( nodeConst203 * 0.85 ) ) ) * vec3<f32>( mix( 1.0, 1.3, nodeConst203 ) ) ), ( nodeConst203 * nodeVar112 ) );

		}

		nodeVar118 = nodeVar1;

		if ( nodeConst1 ) {

			nodeVar119 = vec3<f32>( 0.6038273388475408, 0.6583748172725346, 0.623960391667596 );

		} else {

			nodeVar119 = vec3<f32>( 0.46778379610254284, 0.5647115056965487, 0.5209955731953768 );

		}


		if ( nodeConst1 ) {

			nodeVar120 = vec3<f32>( 0.008023192982520563, 0.010329823026364548, 0.012983032338510335 );

		} else {

			let nodeConst205 = ( v_positionWorld * vec3<f32>( 0.3 ) );
			nodeVar120 = mix( vec3<f32>( 0.006512090790025684, 0.008023192982520563, 0.010329823026364548 ), vec3<f32>( 0.01680737574872402, 0.024157632443547246, 0.030713443727452196 ), ( ( ( fn130( nodeConst205 ) + ( fn130( ( nodeConst205 * vec3<f32>( 2.0 ) ) ) * 0.5 ) ) * 0.5 ) + 0.5 ) );

		}


		if ( nodeConst1 ) {

			let nodeConst206 = ( smoothstep( -0.15, 0.5, ( fn87( vec3<f32>( ( v_positionWorld.x * 1.3 ), ( v_positionWorld.y * 0.06 ), ( v_positionWorld.z * 1.3 ) ), 2, 2.0, 0.5 ) * 1.0 ) ) * 0.45 );
			nodeVar121 = ( 0.14 + ( nodeConst206 * 0.22 ) );

		} else {

			let nodeConst206 = ( smoothstep( -0.15, 0.5, ( fn87( vec3<f32>( ( v_positionWorld.x * 1.3 ), ( v_positionWorld.y * 0.06 ), ( v_positionWorld.z * 1.3 ) ), 2, 2.0, 0.5 ) * 1.0 ) ) * 0.45 );
			let nodeConst207 = ( smoothstep( 0.32, 0.0, nodeVarying13.y ) * 0.4 );
			let nodeConst208 = clamp( ( ( 0.64 + nodeConst206 ) + nodeConst207 ), 0.0, 0.95 );
			nodeVar121 = nodeConst208;

		}

		nodeVar0 = mix( ( nodeVar118.xyz * nodeVar119 ), nodeVar120, nodeVar121 );

	} else {

		let nodeConst209 = ( nodeVarying3 == 7.0 );

		if ( nodeConst209 ) {

			nodeVar122 = mix( ( object.nodeUniform3 * vec3<f32>( 0.3 ) ), vec3<f32>( 0.01764195448412081, 0.014443843592229466, 0.011612245176281512 ), 0.5 );

		} else {

			let nodeConst210 = ( nodeVarying3 == 8.0 );

			if ( nodeConst210 ) {

				let nodeConst211 = floor( ( vec2<f32>( v_positionWorld.x, v_positionWorld.z ) * vec2<f32>( 0.2 ) ) );
				let nodeConst212 = ( fn5( ( ( u32( ( nodeConst211.x + 65536.0 ) ) * 73856093u ) ^ ( u32( ( nodeConst211.y + 65536.0 ) ) * 19349663u ) ) ) * 5.0 );

				if ( ( nodeConst212 > 4.0 ) ) {

					nodeVar124 = vec3<f32>( 0.014443843592229466, 0.013702083043526807, 0.015996293361446288 );

				} else {


					if ( ( nodeConst212 > 3.0 ) ) {

						nodeVar125 = vec3<f32>( 0.10224173307914941, 0.11193242782769693, 0.09758734713304495 );

					} else {


						if ( ( nodeConst212 > 2.0 ) ) {

							nodeVar126 = vec3<f32>( 0.015996293361446288, 0.03954623527052923, 0.07818742179702069 );

						} else {


							if ( ( nodeConst212 > 1.0 ) ) {

								nodeVar127 = vec3<f32>( 0.01764195448412081, 0.06847816983662762, 0.031896033067374104 );

							} else {

								nodeVar127 = vec3<f32>( 0.14412847084818123, 0.028426039499072558, 0.028426039499072558 );

							}

							nodeVar126 = nodeVar127;

						}

						nodeVar125 = nodeVar126;

					}

					nodeVar124 = nodeVar125;

				}

				nodeVar123 = nodeVar124;

			} else {

				let nodeConst213 = ( nodeVarying3 == 2.0 );

				if ( nodeConst213 ) {

					nodeVar128 = ( object.nodeUniform3 * vec3<f32>( 0.55 ) );

				} else {

					let nodeConst214 = ( nodeVarying3 == 3.0 );

					if ( nodeConst214 ) {

						let nodeConst215 = ( nodeVarying6 * 0.18 );
						nodeVar129 = ( mix( object.nodeUniform3, vec3<f32>( 1.0, 1.0, 1.0 ), 0.22 ) * vec3<f32>( ( 1.0 + nodeConst215 ) ) );

					} else {

						let nodeConst216 = ( nodeVarying3 == 5.0 );

						if ( nodeConst216 ) {

							let nodeConst217 = ( ( fn130( ( v_positionWorld * vec3<f32>( 0.4 ) ) ) * 0.5 ) + 0.5 );
							let nodeConst218 = fract( ( nodeVarying13.y * 6.0 ) );
							normalWorldGeometry = normalize( v_normalWorldGeometry );
							let nodeConst220 = length( fwidth( v_positionWorld ) );
							let nodeConst221 = ( ( ( ( smoothstep( 0.06, 0.14, nodeVarying13.x ) * smoothstep( 0.94, 0.86, nodeVarying13.x ) ) * smoothstep( 0.12, 0.2, nodeVarying13.y ) ) * smoothstep( 0.96, 0.88, nodeVarying13.y ) ) * ( smoothstep( 0.65, 0.4, abs( normalWorldGeometry.y ) ) * smoothstep( 0.08, 0.015, nodeConst220 ) ) );
							let nodeConst222 = vec3<f32>( ( v_positionWorld.x * 6.0 ), ( v_positionWorld.y * 0.5 ), ( v_positionWorld.z * 6.0 ) );
							nodeVar130 = mix( ( ( mix( vec3<f32>( 0.8879231178794776, 0.8796223968851662, 0.8387990117372213 ), vec3<f32>( 0.623960391667596, 0.6038273388475408, 0.5394794890033748 ), nodeConst217 ) + vec3<f32>( ( fn130( ( v_positionWorld * vec3<f32>( 5.0 ) ) ) * 0.04 ) ) ) * vec3<f32>( mix( 1.0, ( mix( 0.82, 1.04, nodeConst218 ) * 0.42 ), nodeConst221 ) ) ), vec3<f32>( 0.158960835050774, 0.13843161502267545, 0.10224173307914941 ), ( ( ( smoothstep( 0.4, 0.0, nodeVarying13.y ) * ( ( ( ( fn130( nodeConst222 ) + ( fn130( ( nodeConst222 * vec3<f32>( 2.0 ) ) ) * 0.5 ) ) + ( fn130( ( nodeConst222 * vec3<f32>( 4.0 ) ) ) * 0.25 ) ) * 0.5 ) + 0.5 ) ) * ( nodeConst217 + 0.3 ) ) * 0.5 ) );

						} else {


							if ( ( nodeVarying3 == 1.0 ) ) {

								nodeVar131 = 0.12;

							} else {


								if ( nodeConst214 ) {

									nodeVar132 = 0.2;

								} else {

									nodeVar132 = 0.0;

								}

								nodeVar131 = nodeVar132;

							}

							let nodeConst215 = ( nodeVarying6 * 0.18 );
							let nodeConst223 = ( positionLocal.y / 0.3 );
							let nodeConst224 = floor( nodeConst223 );
							normalWorldGeometry = normalize( v_normalWorldGeometry );
							let nodeConst225 = ( ( ( ( positionLocal.x * normalWorldGeometry.z ) - ( positionLocal.z * normalWorldGeometry.x ) ) / 0.6 ) + ( tsl_mod_float( nodeConst224, 2.0 ) * 0.5 ) );
							nodeVar133 = ( ( u32( ( nodeConst224 + 65536.0 ) ) * 73856093u ) ^ ( u32( ( floor( nodeConst225 ) + 65536.0 ) ) * 19349663u ) );
							let nodeConst226 = ( ( fn5( ( nodeVar133 + 1u ) ) - 0.5 ) * 0.14 );
							let nodeConst227 = ( ( mix( object.nodeUniform3, vec3<f32>( 1.0, 1.0, 1.0 ), nodeVar131 ) * vec3<f32>( ( ( ( 1.0 + nodeConst215 ) + ( fn130( ( v_positionWorld * vec3<f32>( 0.7 ) ) ) * 0.06 ) ) + ( ( fn5( nodeVar133 ) - 0.5 ) * 0.14 ) ) ) ) * vec3<f32>( ( 1.0 + nodeConst226 ), 1.0, ( 1.0 - nodeConst226 ) ) );
							let nodeConst228 = abs( normalWorldGeometry );
							let nodeConst229 = clamp( ( ( ( nodeConst228.z * fwidth( v_positionWorld.x ) ) + ( nodeConst228.x * fwidth( v_positionWorld.z ) ) ) / 0.6 ), 0.000001, 0.5 );
							let nodeConst230 = max( nodeConst229, 0.008333333333333333 );
							let nodeConst231 = ( 0.5 - abs( ( fract( nodeConst225 ) - 0.5 ) ) );
							let nodeConst232 = clamp( fwidth( nodeConst223 ), 0.000001, 0.5 );
							let nodeConst233 = max( nodeConst232, 0.016666666666666666 );
							let nodeConst234 = ( 0.5 - abs( ( fract( nodeConst223 ) - 0.5 ) ) );
							let nodeConst235 = smoothstep( 0.7, 0.45, nodeConst228.y );
							let nodeConst236 = ( max( ( smoothstep( ( nodeConst230 + nodeConst229 ), ( nodeConst230 - nodeConst229 ), nodeConst231 ) * min( ( 0.008333333333333333 / nodeConst230 ), 1.0 ) ), ( smoothstep( ( nodeConst233 + nodeConst232 ), ( nodeConst233 - nodeConst232 ), nodeConst234 ) * min( ( 0.016666666666666666 / nodeConst233 ), 1.0 ) ) ) * nodeConst235 );
							let nodeConst237 = ( 1.0 - nodeConst235 );

							if ( ( nodeConst237 > 0.0 ) ) {

								let nodeConst238 = ( v_positionWorld * vec3<f32>( 0.025 ) );
								nodeVar134 = ( smoothstep( 0.0, 0.55, ( ( fn130( nodeConst238 ) + ( fn130( ( nodeConst238 * vec3<f32>( 2.0 ) ) ) * 0.5 ) ) + ( fn130( ( nodeConst238 * vec3<f32>( 4.0 ) ) ) * 0.25 ) ) ) * 0.22 );

							} else {

								nodeVar134 = 0.0;

							}

							nodeVar130 = mix( mix( nodeConst227, ( nodeConst227 * vec3<f32>( 0.6 ) ), nodeConst236 ), vec3<f32>( 0.06847816983662762, 0.054480276435339814, 0.036889450395083165 ), mix( ( ( smoothstep( -0.1, 0.45, ( fn87( vec3<f32>( ( v_positionWorld.x * 1.5 ), ( v_positionWorld.y * 0.04 ), ( v_positionWorld.z * 1.5 ) ), 2, 2.0, 0.5 ) * 1.0 ) ) * smoothstep( 210.0, 0.0, v_positionWorld.y ) ) * 0.6 ), nodeVar134, nodeConst237 ) );

						}

						nodeVar129 = nodeVar130;

					}

					nodeVar128 = nodeVar129;

				}

				nodeVar123 = nodeVar128;

			}

			nodeVar122 = nodeVar123;

		}

		nodeVar0 = nodeVar122;

	}

	DiffuseColor = vec4<f32>( nodeVar0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform6 );
	DiffuseColor.w = 1.0;
	let nodeConst239 = 0.0;
	Metalness = nodeConst239;

	if ( nodeConst1 ) {

		nodeVar135 = 0.14;

	} else {

		let nodeConst210 = ( nodeVarying3 == 8.0 );

		if ( nodeConst210 ) {

			nodeVar136 = 0.85;

		} else {

			let nodeConst209 = ( nodeVarying3 == 7.0 );

			if ( nodeConst209 ) {

				nodeVar137 = 0.6;

			} else {


				if ( nodeConst0 ) {

					let nodeConst207 = ( smoothstep( 0.32, 0.0, nodeVarying13.y ) * 0.4 );
					let nodeConst206 = ( smoothstep( -0.15, 0.5, ( fn87( vec3<f32>( ( v_positionWorld.x * 1.3 ), ( v_positionWorld.y * 0.06 ), ( v_positionWorld.z * 1.3 ) ), 2, 2.0, 0.5 ) * 1.0 ) ) * 0.45 );
					nodeVar138 = ( ( 0.16 + ( nodeConst207 * 0.45 ) ) + ( nodeConst206 * 0.2 ) );

				} else {

					let nodeConst214 = ( nodeVarying3 == 3.0 );

					if ( nodeConst214 ) {

						nodeVar139 = 0.8;

					} else {

						let nodeConst216 = ( nodeVarying3 == 5.0 );

						if ( nodeConst216 ) {

							normalWorldGeometry = normalize( v_normalWorldGeometry );
							let nodeConst220 = length( fwidth( v_positionWorld ) );
							let nodeConst221 = ( ( ( ( smoothstep( 0.06, 0.14, nodeVarying13.x ) * smoothstep( 0.94, 0.86, nodeVarying13.x ) ) * smoothstep( 0.12, 0.2, nodeVarying13.y ) ) * smoothstep( 0.96, 0.88, nodeVarying13.y ) ) * ( smoothstep( 0.65, 0.4, abs( normalWorldGeometry.y ) ) * smoothstep( 0.08, 0.015, nodeConst220 ) ) );
							nodeVar140 = ( 0.52 + ( nodeConst221 * 0.08 ) );

						} else {

							normalWorldGeometry = normalize( v_normalWorldGeometry );
							let nodeConst228 = abs( normalWorldGeometry );
							let nodeConst229 = clamp( ( ( ( nodeConst228.z * fwidth( v_positionWorld.x ) ) + ( nodeConst228.x * fwidth( v_positionWorld.z ) ) ) / 0.6 ), 0.000001, 0.5 );
							let nodeConst230 = max( nodeConst229, 0.008333333333333333 );
							let nodeConst223 = ( positionLocal.y / 0.3 );
							let nodeConst224 = floor( nodeConst223 );
							let nodeConst225 = ( ( ( ( positionLocal.x * normalWorldGeometry.z ) - ( positionLocal.z * normalWorldGeometry.x ) ) / 0.6 ) + ( tsl_mod_float( nodeConst224, 2.0 ) * 0.5 ) );
							let nodeConst231 = ( 0.5 - abs( ( fract( nodeConst225 ) - 0.5 ) ) );
							let nodeConst232 = clamp( fwidth( nodeConst223 ), 0.000001, 0.5 );
							let nodeConst233 = max( nodeConst232, 0.016666666666666666 );
							let nodeConst234 = ( 0.5 - abs( ( fract( nodeConst223 ) - 0.5 ) ) );
							let nodeConst235 = smoothstep( 0.7, 0.45, nodeConst228.y );
							let nodeConst236 = ( max( ( smoothstep( ( nodeConst230 + nodeConst229 ), ( nodeConst230 - nodeConst229 ), nodeConst231 ) * min( ( 0.008333333333333333 / nodeConst230 ), 1.0 ) ), ( smoothstep( ( nodeConst233 + nodeConst232 ), ( nodeConst233 - nodeConst232 ), nodeConst234 ) * min( ( 0.016666666666666666 / nodeConst233 ), 1.0 ) ) ) * nodeConst235 );
							nodeVar140 = ( ( ( fn130( ( v_positionWorld * vec3<f32>( 0.5 ) ) ) * 0.08 ) + 0.82 ) + ( nodeConst236 * 0.12 ) );

						}

						nodeVar139 = nodeVar140;

					}

					nodeVar138 = nodeVar139;

				}

				nodeVar137 = nodeVar138;

			}

			nodeVar136 = nodeVar137;

		}

		nodeVar135 = nodeVar136;

	}

	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst240 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( nodeVar135, 0.045 ) + max( max( nodeConst240.x, nodeConst240.y ), nodeConst240.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - nodeConst239 ) ) );

	if ( nodeConst2 ) {


		if ( nodeConst1 ) {

			nodeVar142 = 2.0;

		} else {

			let nodeConst206 = ( smoothstep( -0.15, 0.5, ( fn87( vec3<f32>( ( v_positionWorld.x * 1.3 ), ( v_positionWorld.y * 0.06 ), ( v_positionWorld.z * 1.3 ) ), 2, 2.0, 0.5 ) * 1.0 ) ) * 0.45 );
			let nodeConst207 = ( smoothstep( 0.32, 0.0, nodeVarying13.y ) * 0.4 );
			let nodeConst208 = clamp( ( ( 0.64 + nodeConst206 ) + nodeConst207 ), 0.0, 0.95 );
			nodeVar142 = ( 2.2 * ( 1.0 - ( nodeConst208 * 0.6 ) ) );

		}

		nodeVar141 = ( ( nodeVar118.xyz * vec3<f32>( nodeVar118.w ) ) * vec3<f32>( nodeVar142 ) );

	} else {

		nodeVar141 = vec3<f32>( 0.0, 0.0, 0.0 );

	}

	EmissiveColor = nodeVar141;
	let nodeConst241 = dpdx( v_positionView );
	let nodeConst242 = - dpdy( v_positionView );
	NORMAL_normalView = normalViewGeometry;
	let NORMAL_nodeConst243 = cross( nodeConst242, NORMAL_normalView );
	let NORMAL_nodeConst244 = dot( nodeConst241, NORMAL_nodeConst243 );
	let nodeConst213 = ( nodeVarying3 == 2.0 );
	let nodeConst214 = ( nodeVarying3 == 3.0 );
	let nodeConst209 = ( nodeVarying3 == 7.0 );
	let nodeConst210 = ( nodeVarying3 == 8.0 );

	if ( ( ( ( ( ( nodeConst0 || nodeConst213 ) || nodeConst214 ) || nodeConst1 ) || nodeConst209 ) || nodeConst210 ) ) {

		nodeVar143 = 0.0;

	} else {

		let nodeConst216 = ( nodeVarying3 == 5.0 );

		if ( nodeConst216 ) {

			normalWorldGeometry = normalize( v_normalWorldGeometry );
			let nodeConst220 = length( fwidth( v_positionWorld ) );
			let nodeConst221 = ( ( ( ( smoothstep( 0.06, 0.14, nodeVarying13.x ) * smoothstep( 0.94, 0.86, nodeVarying13.x ) ) * smoothstep( 0.12, 0.2, nodeVarying13.y ) ) * smoothstep( 0.96, 0.88, nodeVarying13.y ) ) * ( smoothstep( 0.65, 0.4, abs( normalWorldGeometry.y ) ) * smoothstep( 0.08, 0.015, nodeConst220 ) ) );
			let nodeConst218 = fract( ( nodeVarying13.y * 6.0 ) );
			nodeVar144 = ( nodeConst221 * ( ( nodeConst218 * 0.012 ) - 0.01 ) );

		} else {

			let nodeConst220 = length( fwidth( v_positionWorld ) );
			let nodeConst245 = max( ( nodeConst220 * 1.5 ), 0.008 );
			normalWorldGeometry = normalize( v_normalWorldGeometry );
			let nodeConst223 = ( positionLocal.y / 0.3 );
			let nodeConst224 = floor( nodeConst223 );
			let nodeConst225 = ( ( ( ( positionLocal.x * normalWorldGeometry.z ) - ( positionLocal.z * normalWorldGeometry.x ) ) / 0.6 ) + ( tsl_mod_float( nodeConst224, 2.0 ) * 0.5 ) );
			let nodeConst231 = ( 0.5 - abs( ( fract( nodeConst225 ) - 0.5 ) ) );
			let nodeConst234 = ( 0.5 - abs( ( fract( nodeConst223 ) - 0.5 ) ) );
			let nodeConst228 = abs( normalWorldGeometry );
			let nodeConst235 = smoothstep( 0.7, 0.45, nodeConst228.y );
			nodeVar144 = ( ( ( smoothstep( 0.0, nodeConst245, ( nodeConst231 * 0.6 ) ) * smoothstep( 0.0, nodeConst245, ( nodeConst234 * 0.3 ) ) ) * nodeConst235 ) * 0.003 );

		}

		nodeVar143 = nodeVar144;

	}

	let nodeConst246 = ( vec3<f32>( sign( NORMAL_nodeConst244 ) ) * ( ( vec3<f32>( dpdx( nodeVar143 ) ) * NORMAL_nodeConst243 ) + ( vec3<f32>( - dpdy( nodeVar143 ) ) * cross( NORMAL_normalView, nodeConst241 ) ) ) );
	normalView = normalize( ( ( vec3<f32>( abs( NORMAL_nodeConst244 ) ) * NORMAL_normalView ) - nodeConst246 ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar145 = textureSample( nodeUniform7, nodeUniform7_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar145;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst247 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst247 );
	let nodeConst248 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst249 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst247 * nodeConst248 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst249 ) * nodeConst248 ) ) ) * vec3<f32>( nodeConst249 ) ) );
	let nodeConst250 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform9 - render.nodeUniform10 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst251 = ( render.nodeUniform12 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform13 ) ) ), 1.0 ) );
	let nodeConst252 = ( nodeConst251.xyz / vec3<f32>( nodeConst251.w ) );
	let nodeConst253 = vec3<f32>( nodeConst252.x, ( 1.0 - nodeConst252.y ), ( nodeConst252.z + render.nodeUniform14 ) );

	if ( ( ( ( ( ( nodeConst253.x >= 0.0 ) && ( nodeConst253.x <= 1.0 ) ) && ( nodeConst253.y >= 0.0 ) ) && ( nodeConst253.y <= 1.0 ) ) && ( nodeConst253.z <= 1.0 ) ) ) {

		let nodeConst254 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst255 = ( vec2<f32>( render.nodeUniform16 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform17 ) );
		let nodeConst256 = ( nodeConst253.xy + ( fn25( 0, 5, nodeConst254 ) * nodeConst255 ) );
		nodeVar147 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeConst256, nodeConst253.z );
		let nodeConst257 = ( nodeConst253.xy + ( fn25( 1, 5, nodeConst254 ) * nodeConst255 ) );
		nodeVar148 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeConst257, nodeConst253.z );
		let nodeConst258 = ( nodeConst253.xy + ( fn25( 2, 5, nodeConst254 ) * nodeConst255 ) );
		nodeVar149 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeConst258, nodeConst253.z );
		let nodeConst259 = ( nodeConst253.xy + ( fn25( 3, 5, nodeConst254 ) * nodeConst255 ) );
		nodeVar150 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeConst259, nodeConst253.z );
		let nodeConst260 = ( nodeConst253.xy + ( fn25( 4, 5, nodeConst254 ) * nodeConst255 ) );
		nodeVar151 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeConst260, nodeConst253.z );
		nodeVar146 = ( ( ( ( ( nodeVar147 + nodeVar148 ) + nodeVar149 ) + nodeVar150 ) + nodeVar151 ) * 0.2 );

	} else {

		nodeVar146 = 1.0;

	}

	nodeVar152 = mix( 1.0, nodeVar146, render.nodeUniform18 );
	nodeVar153 = ( vec3<f32>( clamp( dot( normalView, nodeConst250 ), 0.0, 1.0 ) ) * ( render.nodeUniform11 * vec3<f32>( nodeVar152 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst261 = clamp( dot( positionViewDirection, normalize( ( nodeConst250 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst262 = exp2( ( ( ( nodeConst261 * -5.55473 ) - 6.98316 ) * nodeConst261 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar153 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst262 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst262 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst263 = normalize( ( nodeConst250 + positionViewDirection ) );
	let nodeConst264 = clamp( dot( positionViewDirection, nodeConst263 ), 0.0, 1.0 );
	let nodeConst265 = exp2( ( ( ( nodeConst264 * -5.55473 ) - 6.98316 ) * nodeConst264 ) );
	let nodeConst266 = max( Roughness, 0.045 );
	let nodeConst267 = ( nodeConst266 * nodeConst266 );
	directSpecular = ( directSpecular + ( ( nodeVar153 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst265 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst265 ) ) ) * vec3<f32>( fn110( nodeConst267, clamp( dot( normalView, nodeConst250 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst267, clamp( dot( normalView, nodeConst263 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst268 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst269 = ( object.nodeUniform20 * vec4<f32>( nodeConst268, 1.0 ) );
	let nodeConst270 = clamp( Roughness, 0.0, 1.0 );
	nodeVar154 = textureSampleLevel( nodeUniform19, nodeUniform19_sampler, vec3<f32>( ( - nodeConst269.x ), nodeConst269.yz ), ( ( object.nodeUniform22 * nodeConst270 ) * ( 2.0 - nodeConst270 ) ) );
	radiance = ( radiance + ( nodeVar154.xyz * vec3<f32>( object.nodeUniform23 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst271 = ( object.nodeUniform20 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst272 = clamp( 1.0, 0.0, 1.0 );
	nodeVar155 = textureSampleLevel( nodeUniform19, nodeUniform19_sampler, vec3<f32>( ( - nodeConst271.x ), nodeConst271.yz ), ( ( object.nodeUniform22 * nodeConst272 ) * ( 2.0 - nodeConst272 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar155.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform23 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar156 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar156 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst273 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst273 );
	let nodeConst274 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst275 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst273 * nodeConst274 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst275 ) * nodeConst274 ) ) ) * vec3<f32>( nodeConst275 ) ) );
	let nodeConst276 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar157 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst276 ) );
	nodeVar158 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst276 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar157 );
	indirectDiffuse = ( indirectDiffuse + nodeVar158 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst277 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst277;

	// result

	output.color = nodeConst277;

	return output;

}
