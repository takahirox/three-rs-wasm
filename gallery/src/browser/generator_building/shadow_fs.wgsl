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

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : vec3<f32>,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraPosition : vec3<f32>
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
var<private> Output : vec4<f32>;

// codes
fn fn5 ( seed : u32 ) -> f32 {

	

	let nodeConst0 = ( ( seed * 747796405u ) + 2891336453u );
	let nodeConst1 = ( ( ( nodeConst0 >> ( ( nodeConst0 >> 28u ) + 4u ) ) ^ nodeConst0 ) * 277803737u );

	return ( f32( ( ( nodeConst1 >> 22u ) ^ nodeConst1 ) ) * 2.3283064365386963e-10 );

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




@fragment
fn main( @location( 0 ) positionLocal : vec3<f32>,
	@location( 1 ) @interpolate( flat, either ) nodeVarying3 : f32,
	@location( 2 ) @interpolate( flat, either ) nodeVarying4 : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32>,
	@location( 4 ) nodeVarying6 : f32,
	@location( 5 ) v_normalWorldGeometry : vec3<f32>,
	@location( 6 ) nodeVarying10 : vec2<f32>,
	@location( 7 ) nodeVarying11 : vec3<f32>,
	@location( 8 ) nodeVarying12 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	let nodeConst0 = ( nodeVarying3 == 6.0 );

	if ( ( ( nodeVarying3 == 4.0 ) || nodeConst0 ) ) {


		if ( nodeConst0 ) {

			let nodeConst1 = floor( ( nodeVarying4 * vec3<f32>( 2.0 ) ) );
			nodeVar2 = ( ( ( u32( ( nodeConst1.x + 2097152.0 ) ) * 73856093u ) ^ ( u32( ( nodeConst1.y + 2097152.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeConst1.z + 2097152.0 ) ) * 83492791u ) );
			let nodeConst2 = ( nodeVarying10.y * 2.4 );
			let nodeConst3 = vec3<f32>( ( nodeVarying10.x * 0.5 ), ( nodeVarying10.y * 0.5 ), ( 0.1 + nodeConst2 ) );
			let nodeConst4 = vec3<f32>( ( - nodeConst3.x ), ( - nodeConst3.y ), 0.1 );
			let nodeConst5 = ( positionLocal - nodeVarying4 );
			normalLocal = nodeVarying11;
			let nodeConst6 = normalize( cross( vec3<f32>( 0.0, 1.0, 0.0 ), normalLocal ) );
			let nodeConst7 = vec3<f32>( dot( nodeConst5, nodeConst6 ), nodeConst5.y, 0.0 );
			let nodeConst8 = normalize( ( positionLocal - ( object.nodeUniform0 * vec4<f32>( render.cameraPosition, 1.0 ) ).xyz ) );
			let nodeConst9 = vec3<f32>( dot( nodeConst8, nodeConst6 ), nodeConst8.y, ( - dot( nodeConst8, normalLocal ) ) );
			let nodeConst10 = ( ( vec3<f32>( ( - nodeConst3.x ), nodeConst4.y, 0.1 ) - nodeConst7 ) / nodeConst9 );
			let nodeConst11 = ( ( vec3<f32>( nodeConst3.x, nodeConst3.y, ( 0.1 + 0.05 ) ) - nodeConst7 ) / nodeConst9 );
			let nodeConst12 = max( nodeConst10, nodeConst11 );
			let nodeConst13 = min( nodeConst10, nodeConst11 );
			let nodeConst14 = max( max( nodeConst13.x, nodeConst13.y ), nodeConst13.z );
			let nodeConst15 = step( 0.18, fn5( ( nodeVar2 + 79599u ) ) );
			let nodeConst16 = ( 0.1 + 1.2 );
			let nodeConst17 = vec3<f32>( -0.45, nodeConst4.y, nodeConst16 );
			let nodeConst18 = ( ( nodeConst17 - nodeConst7 ) / nodeConst9 );
			let nodeConst19 = ( nodeConst3.z - 1.2 );
			let nodeConst20 = vec3<f32>( 0.45, ( nodeConst4.y + 1.5 ), nodeConst19 );
			let nodeConst21 = ( ( nodeConst20 - nodeConst7 ) / nodeConst9 );
			let nodeConst22 = max( nodeConst18, nodeConst21 );
			let nodeConst23 = min( nodeConst18, nodeConst21 );
			let nodeConst24 = max( max( nodeConst23.x, nodeConst23.y ), nodeConst23.z );
			let nodeConst25 = step( 0.45, fn5( ( nodeVar2 + 34690u ) ) );
			let nodeConst26 = fn5( ( nodeVar2 + 105097u ) );
			let nodeConst27 = mix( ( ( - nodeConst3.x ) + 0.7 ), ( nodeConst3.x - 0.7 ), step( 0.5, nodeConst26 ) );
			let nodeConst28 = ( 0.1 + ( nodeConst2 * 0.5 ) );
			let nodeConst29 = vec3<f32>( ( nodeConst27 - 0.75 ), nodeConst4.y, nodeConst28 );
			let nodeConst30 = ( ( nodeConst29 - nodeConst7 ) / nodeConst9 );
			let nodeConst31 = vec3<f32>( ( nodeConst27 + 0.75 ), ( nodeConst4.y + 1.0 ), ( nodeConst28 + 0.65 ) );
			let nodeConst32 = ( ( nodeConst31 - nodeConst7 ) / nodeConst9 );
			let nodeConst33 = max( nodeConst30, nodeConst32 );
			let nodeConst34 = min( nodeConst30, nodeConst32 );
			let nodeConst35 = max( max( nodeConst34.x, nodeConst34.y ), nodeConst34.z );
			let nodeConst36 = ( nodeConst3.x * 0.62 );
			let nodeConst37 = fn5( ( nodeVar2 + 40310u ) );
			let nodeConst38 = mix( 0.16, 0.3, nodeConst37 );
			let nodeConst39 = vec3<f32>( ( nodeConst36 - nodeConst38 ), ( nodeConst4.y + 0.4 ), ( 0.1 + 0.25 ) );
			let nodeConst40 = ( ( nodeConst39 - nodeConst7 ) / nodeConst9 );
			let nodeConst41 = fn5( ( nodeVar2 + 66130u ) );
			let nodeConst42 = vec3<f32>( ( nodeConst36 + nodeConst38 ), ( ( nodeConst4.y + 0.4 ) + mix( 0.5, 1.2, nodeConst41 ) ), ( ( 0.1 + 0.25 ) + ( nodeConst38 * 2.0 ) ) );
			let nodeConst43 = ( ( nodeConst42 - nodeConst7 ) / nodeConst9 );
			let nodeConst44 = max( nodeConst40, nodeConst43 );
			let nodeConst45 = min( nodeConst40, nodeConst43 );
			let nodeConst46 = max( max( nodeConst45.x, nodeConst45.y ), nodeConst45.z );
			let nodeConst47 = ( nodeConst3.x * 0.0 );
			let nodeConst48 = fn5( ( nodeVar2 + 39610u ) );
			let nodeConst49 = mix( 0.16, 0.3, nodeConst48 );
			let nodeConst50 = vec3<f32>( ( nodeConst47 - nodeConst49 ), ( nodeConst4.y + 0.4 ), ( 0.1 + 0.25 ) );
			let nodeConst51 = ( ( nodeConst50 - nodeConst7 ) / nodeConst9 );
			let nodeConst52 = fn5( ( nodeVar2 + 66030u ) );
			let nodeConst53 = vec3<f32>( ( nodeConst47 + nodeConst49 ), ( ( nodeConst4.y + 0.4 ) + mix( 0.5, 1.2, nodeConst52 ) ), ( ( 0.1 + 0.25 ) + ( nodeConst49 * 2.0 ) ) );
			let nodeConst54 = ( ( nodeConst53 - nodeConst7 ) / nodeConst9 );
			let nodeConst55 = max( nodeConst51, nodeConst54 );
			let nodeConst56 = min( nodeConst51, nodeConst54 );
			let nodeConst57 = max( max( nodeConst56.x, nodeConst56.y ), nodeConst56.z );
			let nodeConst58 = ( nodeConst3.x * -0.62 );
			let nodeConst59 = fn5( ( nodeVar2 + 38910u ) );
			let nodeConst60 = mix( 0.16, 0.3, nodeConst59 );
			let nodeConst61 = vec3<f32>( ( nodeConst58 - nodeConst60 ), ( nodeConst4.y + 0.4 ), ( 0.1 + 0.25 ) );
			let nodeConst62 = ( ( nodeConst61 - nodeConst7 ) / nodeConst9 );
			let nodeConst63 = fn5( ( nodeVar2 + 65930u ) );
			let nodeConst64 = vec3<f32>( ( nodeConst58 + nodeConst60 ), ( ( nodeConst4.y + 0.4 ) + mix( 0.5, 1.2, nodeConst63 ) ), ( ( 0.1 + 0.25 ) + ( nodeConst60 * 2.0 ) ) );
			let nodeConst65 = ( ( nodeConst64 - nodeConst7 ) / nodeConst9 );
			let nodeConst66 = max( nodeConst62, nodeConst65 );
			let nodeConst67 = min( nodeConst62, nodeConst65 );
			let nodeConst68 = max( max( nodeConst67.x, nodeConst67.y ), nodeConst67.z );
			let nodeConst69 = vec3<f32>( ( ( - nodeConst3.x ) + 0.15 ), nodeConst4.y, ( 0.1 + 0.06 ) );
			let nodeConst70 = ( ( nodeConst69 - nodeConst7 ) / nodeConst9 );
			let nodeConst71 = vec3<f32>( ( nodeConst3.x - 0.15 ), ( nodeConst4.y + 0.4 ), ( 0.1 + 0.95 ) );
			let nodeConst72 = ( ( nodeConst71 - nodeConst7 ) / nodeConst9 );
			let nodeConst73 = max( nodeConst70, nodeConst72 );
			let nodeConst74 = min( nodeConst70, nodeConst72 );
			let nodeConst75 = max( max( nodeConst74.x, nodeConst74.y ), nodeConst74.z );
			let nodeConst76 = max( ( ( nodeConst4 - nodeConst7 ) / nodeConst9 ), ( ( nodeConst3 - nodeConst7 ) / nodeConst9 ) );
			let nodeConst77 = min( min( nodeConst76.x, nodeConst76.y ), nodeConst76.z );
			let nodeConst78 = ( ( ( min( min( nodeConst73.x, nodeConst73.y ), nodeConst73.z ) > nodeConst75 ) && ( nodeConst75 > 0.0 ) ) && ( nodeConst75 < nodeConst77 ) );

			if ( nodeConst78 ) {

				nodeVar9 = nodeConst75;

			} else {

				nodeVar9 = nodeConst77;

			}

			let nodeConst79 = ( ( ( ( min( min( nodeConst66.x, nodeConst66.y ), nodeConst66.z ) > nodeConst68 ) && ( nodeConst68 > 0.0 ) ) && ( nodeConst59 > 0.15 ) ) && ( nodeConst68 < nodeVar9 ) );

			if ( nodeConst79 ) {

				nodeVar8 = nodeConst68;

			} else {

				nodeVar8 = nodeVar9;

			}

			let nodeConst80 = ( ( ( ( min( min( nodeConst55.x, nodeConst55.y ), nodeConst55.z ) > nodeConst57 ) && ( nodeConst57 > 0.0 ) ) && ( nodeConst48 > 0.15 ) ) && ( nodeConst57 < nodeVar8 ) );

			if ( nodeConst80 ) {

				nodeVar7 = nodeConst57;

			} else {

				nodeVar7 = nodeVar8;

			}

			let nodeConst81 = ( ( ( ( min( min( nodeConst44.x, nodeConst44.y ), nodeConst44.z ) > nodeConst46 ) && ( nodeConst46 > 0.0 ) ) && ( nodeConst37 > 0.15 ) ) && ( nodeConst46 < nodeVar7 ) );

			if ( nodeConst81 ) {

				nodeVar6 = nodeConst46;

			} else {

				nodeVar6 = nodeVar7;

			}

			let nodeConst82 = ( ( ( min( min( nodeConst33.x, nodeConst33.y ), nodeConst33.z ) > nodeConst35 ) && ( nodeConst35 > 0.0 ) ) && ( nodeConst35 < nodeVar6 ) );

			if ( nodeConst82 ) {

				nodeVar5 = nodeConst35;

			} else {

				nodeVar5 = nodeVar6;

			}

			let nodeConst83 = ( ( ( ( min( min( nodeConst22.x, nodeConst22.y ), nodeConst22.z ) > nodeConst24 ) && ( nodeConst24 > 0.0 ) ) && ( nodeConst25 > 0.5 ) ) && ( nodeConst24 < nodeVar5 ) );

			if ( nodeConst83 ) {

				nodeVar4 = nodeConst24;

			} else {

				nodeVar4 = nodeVar5;

			}

			let nodeConst84 = ( ( ( ( min( min( nodeConst12.x, nodeConst12.y ), nodeConst12.z ) > nodeConst14 ) && ( nodeConst14 > 0.0 ) ) && ( ( ( 1.0 - nodeConst15 ) * step( 0.45, fn5( ( nodeVar2 + 39650u ) ) ) ) > 0.5 ) ) && ( nodeConst14 < nodeVar4 ) );

			if ( nodeConst84 ) {

				nodeVar3 = ( mix( vec3<f32>( 0.0722718506743852, 0.08228270712149792, 0.09758734713304495 ), vec3<f32>( 0.00972121731707524, 0.010960094003125918, 0.012983032338510335 ), step( 0.62, fract( ( ( nodeConst7 + ( nodeConst9 * vec3<f32>( nodeConst14 ) ) ).y * 7.0 ) ) ) ) * vec3<f32>( 0.8 ) );

			} else {


				if ( nodeConst83 ) {

					let nodeConst85 = ( nodeConst7 + ( nodeConst9 * vec3<f32>( nodeConst24 ) ) );
					let nodeConst86 = ( ( nodeConst85.z - nodeConst16 ) / ( nodeConst19 - nodeConst16 ) );
					let nodeConst87 = ( ( ( nodeVar2 + 91u ) + ( u32( floor( ( nodeConst86 * 8.0 ) ) ) * 197u ) ) + ( u32( floor( ( ( ( nodeConst85 - nodeConst17 ) / ( nodeConst20 - nodeConst17 ) ).y * 4.0 ) ) ) * 4099u ) );
					let nodeConst88 = ( fn5( nodeConst87 ) * 6.0 );

					if ( ( nodeConst88 > 5.0 ) ) {

						nodeVar11 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

					} else {


						if ( ( nodeConst88 > 4.0 ) ) {

							nodeVar12 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

						} else {


							if ( ( nodeConst88 > 3.0 ) ) {

								nodeVar13 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

							} else {


								if ( ( nodeConst88 > 2.0 ) ) {

									nodeVar14 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

								} else {


									if ( ( nodeConst88 > 1.0 ) ) {

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

					let nodeConst89 = fract( ( ( ( nodeConst85 - nodeConst17 ) / ( nodeConst20 - nodeConst17 ) ).y * 4.0 ) );
					let nodeConst90 = fn5( ( nodeConst87 + 1u ) );
					let nodeConst91 = fn5( ( nodeVar2 + 119831u ) );
					let nodeConst92 = mix( vec3<f32>( 0.14702726648767014, 0.10224173307914941, 0.05612849004241121 ), vec3<f32>( 0.623960391667596, 0.6038273388475408, 0.55201140150344 ), step( 0.5, nodeConst91 ) );
					nodeVar10 = ( mix( mix( vec3<f32>( 0.015208514418949472, 0.012983032338510335, 0.010960094003125918 ), nodeVar11, ( ( ( step( 0.08, nodeConst89 ) * step( nodeConst89, mix( 0.4, 0.85, fract( ( nodeConst90 * 5.0 ) ) ) ) ) * step( abs( ( fract( ( nodeConst86 * 8.0 ) ) - 0.5 ) ), mix( 0.26, 0.46, fract( ( nodeConst90 * 13.0 ) ) ) ) ) * step( 0.15, nodeConst90 ) ) ), nodeConst92, step( nodeConst89, 0.08 ) ) * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeConst15 ), clamp( ( ( nodeConst85.z - 0.1 ) / nodeConst2 ), 0.0, 1.0 ) ) ) );

				} else {


					if ( nodeConst82 ) {

						let nodeConst91 = fn5( ( nodeVar2 + 119831u ) );
						let nodeConst93 = ( nodeConst7 + ( nodeConst9 * vec3<f32>( nodeConst35 ) ) );

						if ( ( ( ( nodeConst93 - nodeConst29 ) / ( nodeConst31 - nodeConst29 ) ).y > 0.92 ) ) {

							nodeVar17 = 1.3;

						} else {

							nodeVar17 = 0.85;

						}

						nodeVar16 = ( ( mix( vec3<f32>( 0.06847816983662762, 0.05126945836711539, 0.036889450395083165 ), vec3<f32>( 0.025186859622305935, 0.033104766565152086, 0.04231141061442144 ), nodeConst91 ) * vec3<f32>( nodeVar17 ) ) * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeConst15 ), clamp( ( ( nodeConst93.z - 0.1 ) / nodeConst2 ), 0.0, 1.0 ) ) ) );

					} else {


						if ( nodeConst81 ) {

							let nodeConst94 = ( nodeConst7 + ( nodeConst9 * vec3<f32>( nodeConst46 ) ) );

							if ( ( ( ( nodeConst94 - nodeConst39 ) / ( nodeConst42 - nodeConst39 ) ).y > 0.6 ) ) {

								let nodeConst95 = ( nodeConst41 * 6.0 );

								if ( ( nodeConst95 > 5.0 ) ) {

									nodeVar20 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

								} else {


									if ( ( nodeConst95 > 4.0 ) ) {

										nodeVar21 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

									} else {


										if ( ( nodeConst95 > 3.0 ) ) {

											nodeVar22 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

										} else {


											if ( ( nodeConst95 > 2.0 ) ) {

												nodeVar23 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

											} else {


												if ( ( nodeConst95 > 1.0 ) ) {

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

							nodeVar18 = ( nodeVar19 * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeConst15 ), clamp( ( ( nodeConst94.z - 0.1 ) / nodeConst2 ), 0.0, 1.0 ) ) ) );

						} else {


							if ( nodeConst80 ) {

								let nodeConst96 = ( nodeConst7 + ( nodeConst9 * vec3<f32>( nodeConst57 ) ) );

								if ( ( ( ( nodeConst96 - nodeConst50 ) / ( nodeConst53 - nodeConst50 ) ).y > 0.6 ) ) {

									let nodeConst97 = ( nodeConst52 * 6.0 );

									if ( ( nodeConst97 > 5.0 ) ) {

										nodeVar27 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

									} else {


										if ( ( nodeConst97 > 4.0 ) ) {

											nodeVar28 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

										} else {


											if ( ( nodeConst97 > 3.0 ) ) {

												nodeVar29 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

											} else {


												if ( ( nodeConst97 > 2.0 ) ) {

													nodeVar30 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

												} else {


													if ( ( nodeConst97 > 1.0 ) ) {

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

								nodeVar25 = ( nodeVar26 * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeConst15 ), clamp( ( ( nodeConst96.z - 0.1 ) / nodeConst2 ), 0.0, 1.0 ) ) ) );

							} else {


								if ( nodeConst79 ) {

									let nodeConst98 = ( nodeConst7 + ( nodeConst9 * vec3<f32>( nodeConst68 ) ) );

									if ( ( ( ( nodeConst98 - nodeConst61 ) / ( nodeConst64 - nodeConst61 ) ).y > 0.6 ) ) {

										let nodeConst99 = ( nodeConst63 * 6.0 );

										if ( ( nodeConst99 > 5.0 ) ) {

											nodeVar34 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

										} else {


											if ( ( nodeConst99 > 4.0 ) ) {

												nodeVar35 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

											} else {


												if ( ( nodeConst99 > 3.0 ) ) {

													nodeVar36 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

												} else {


													if ( ( nodeConst99 > 2.0 ) ) {

														nodeVar37 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

													} else {


														if ( ( nodeConst99 > 1.0 ) ) {

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

									nodeVar32 = ( nodeVar33 * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeConst15 ), clamp( ( ( nodeConst98.z - 0.1 ) / nodeConst2 ), 0.0, 1.0 ) ) ) );

								} else {


									if ( nodeConst78 ) {

										let nodeConst100 = ( nodeConst7 + ( nodeConst9 * vec3<f32>( nodeConst75 ) ) );

										if ( ( ( ( nodeConst100 - nodeConst69 ) / ( nodeConst71 - nodeConst69 ) ).y > 0.93 ) ) {

											nodeVar40 = vec3<f32>( 0.6938717612856897, 0.6583748172725346, 0.5775804404214573 );

										} else {

											nodeVar40 = vec3<f32>( 0.035601314869097636, 0.02955683443236377, 0.023153366173251363 );

										}

										nodeVar39 = ( nodeVar40 * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeConst15 ), clamp( ( ( nodeConst100.z - 0.1 ) / nodeConst2 ), 0.0, 1.0 ) ) ) );

									} else {

										let nodeConst101 = ( nodeConst7 + ( nodeConst9 * vec3<f32>( nodeConst77 ) ) );
										let nodeConst102 = ( ( nodeConst101 - nodeConst4 ) / ( nodeConst3 - nodeConst4 ) );
										let nodeConst103 = ( nodeConst102.z > 0.998 );

										if ( nodeConst103 ) {


											if ( bool( nodeConst25 ) ) {

												let nodeConst104 = ( nodeVarying10.x * 2.5 );
												let nodeConst105 = ( ( ( nodeVar2 + 31u ) + ( u32( floor( ( nodeConst102.x * nodeConst104 ) ) ) * 197u ) ) + ( u32( floor( ( nodeConst102.y * 4.0 ) ) ) * 4099u ) );
												let nodeConst106 = ( fn5( nodeConst105 ) * 6.0 );

												if ( ( nodeConst106 > 5.0 ) ) {

													nodeVar43 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

												} else {


													if ( ( nodeConst106 > 4.0 ) ) {

														nodeVar44 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

													} else {


														if ( ( nodeConst106 > 3.0 ) ) {

															nodeVar45 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

														} else {


															if ( ( nodeConst106 > 2.0 ) ) {

																nodeVar46 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

															} else {


																if ( ( nodeConst106 > 1.0 ) ) {

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

												let nodeConst107 = fract( ( nodeConst102.y * 4.0 ) );
												let nodeConst108 = fn5( ( nodeConst105 + 1u ) );
												let nodeConst91 = fn5( ( nodeVar2 + 119831u ) );
												let nodeConst92 = mix( vec3<f32>( 0.14702726648767014, 0.10224173307914941, 0.05612849004241121 ), vec3<f32>( 0.623960391667596, 0.6038273388475408, 0.55201140150344 ), step( 0.5, nodeConst91 ) );
												nodeVar42 = mix( mix( vec3<f32>( 0.015208514418949472, 0.012983032338510335, 0.010960094003125918 ), nodeVar43, ( ( ( step( 0.08, nodeConst107 ) * step( nodeConst107, mix( 0.4, 0.85, fract( ( nodeConst108 * 5.0 ) ) ) ) ) * step( abs( ( fract( ( nodeConst102.x * nodeConst104 ) ) - 0.5 ) ), mix( 0.26, 0.46, fract( ( nodeConst108 * 13.0 ) ) ) ) ) * step( 0.15, nodeConst108 ) ) ), nodeConst92, step( nodeConst107, 0.08 ) );

											} else {

												let nodeConst91 = fn5( ( nodeVar2 + 119831u ) );
												let nodeConst109 = ( nodeConst91 * 6.0 );

												if ( ( nodeConst109 > 5.0 ) ) {

													nodeVar48 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

												} else {


													if ( ( nodeConst109 > 4.0 ) ) {

														nodeVar49 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

													} else {


														if ( ( nodeConst109 > 3.0 ) ) {

															nodeVar50 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

														} else {


															if ( ( nodeConst109 > 2.0 ) ) {

																nodeVar51 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

															} else {


																if ( ( nodeConst109 > 1.0 ) ) {

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

												let nodeConst110 = mix( vec3<f32>( 0.6866853124288864, 0.6444796819634361, 0.5647115056965487 ), vec3<f32>( 0.5209955731953768, 0.48514994004665124, 0.4232676699760063 ), nodeConst26 );
												let nodeConst111 = mix( 0.3, 0.7, nodeConst26 );
												let nodeConst112 = ( fn5( ( nodeVar2 + 41260u ) ) * 6.0 );

												if ( ( nodeConst112 > 5.0 ) ) {

													nodeVar53 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

												} else {


													if ( ( nodeConst112 > 4.0 ) ) {

														nodeVar54 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

													} else {


														if ( ( nodeConst112 > 3.0 ) ) {

															nodeVar55 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

														} else {


															if ( ( nodeConst112 > 2.0 ) ) {

																nodeVar56 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

															} else {


																if ( ( nodeConst112 > 1.0 ) ) {

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

												nodeVar42 = mix( mix( mix( nodeVar48, nodeConst110, 0.45 ), vec3<f32>( 0.8069522576650873, 0.775822218312646, 0.7011018919268015 ), ( smoothstep( 0.10600000000000001, 0.094, abs( ( nodeConst102.x - nodeConst111 ) ) ) * smoothstep( 0.14600000000000002, 0.134, abs( ( nodeConst102.y - 0.62 ) ) ) ) ), nodeVar53, ( ( smoothstep( 0.081, 0.06899999999999999, abs( ( nodeConst102.x - nodeConst111 ) ) ) * smoothstep( 0.10600000000000001, 0.094, abs( ( nodeConst102.y - 0.62 ) ) ) ) * 0.85 ) );

											}

											nodeVar41 = nodeVar42;

										} else {

											let nodeConst113 = ( nodeConst102.y > 0.998 );

											if ( nodeConst113 ) {


												if ( ( fn5( ( nodeVar2 + 25910u ) ) > 0.65 ) ) {

													nodeVar59 = mix( vec3<f32>( 1.0, 0.6938717612856897, 0.3515325994898463 ), vec3<f32>( 1.0, 0.8227857543924378, 0.5972017883558645 ), fn5( ( nodeVar2 + 86350u ) ) );

												} else {

													nodeVar59 = mix( vec3<f32>( 0.85499260812105, 0.8879231178794776, 0.9215818562755338 ), vec3<f32>( 0.7379104087672317, 0.8069522576650873, 1.0 ), fn5( ( nodeVar2 + 59550u ) ) );

												}

												nodeVar58 = mix( vec3<f32>( 0.8148465722120952, 0.7912979403281551, 0.7379104087672317 ), ( nodeVar59 * vec3<f32>( mix( 0.5, 6.0, nodeConst15 ) ) ), ( step( abs( ( fract( ( nodeConst102.x * 3.0 ) ) - 0.5 ) ), 0.3 ) * step( abs( ( fract( ( nodeConst102.z * 4.0 ) ) - 0.5 ) ), 0.15 ) ) );

											} else {

												let nodeConst114 = ( nodeConst102.y < 0.002 );

												if ( nodeConst114 ) {


													if ( ( fn5( ( nodeVar2 + 27510u ) ) > 0.75 ) ) {

														nodeVar61 = mix( vec3<f32>( 0.01764195448412081, 0.019382360952473074, 0.023153366173251363 ), vec3<f32>( 0.623960391667596, 0.6038273388475408, 0.5394794890033748 ), tsl_mod_float( ( floor( ( nodeConst102.x * 8.0 ) ) + floor( ( nodeConst102.z * 10.0 ) ) ), 2.0 ) );

													} else {

														let nodeConst91 = fn5( ( nodeVar2 + 119831u ) );
														nodeVar61 = ( mix( vec3<f32>( 0.46207699964472876, 0.4286904966038916, 0.36130677977297226 ), vec3<f32>( 0.3005437944049895, 0.2746773120495699, 0.22696587349938613 ), nodeConst91 ) * vec3<f32>( ( 1.0 - ( max( step( 0.93, fract( ( nodeConst102.x * 8.0 ) ) ), step( 0.93, fract( ( nodeConst102.z * 10.0 ) ) ) ) * 0.35 ) ) ) );

													}

													nodeVar60 = nodeVar61;

												} else {


													if ( bool( nodeConst25 ) ) {

														let nodeConst115 = ( nodeConst2 * 2.5 );
														let nodeConst116 = ( ( ( nodeVar2 + 57u ) + ( u32( floor( ( nodeConst102.z * nodeConst115 ) ) ) * 197u ) ) + ( u32( floor( ( nodeConst102.y * 4.0 ) ) ) * 4099u ) );
														let nodeConst117 = ( fn5( nodeConst116 ) * 6.0 );

														if ( ( nodeConst117 > 5.0 ) ) {

															nodeVar63 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

														} else {


															if ( ( nodeConst117 > 4.0 ) ) {

																nodeVar64 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

															} else {


																if ( ( nodeConst117 > 3.0 ) ) {

																	nodeVar65 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

																} else {


																	if ( ( nodeConst117 > 2.0 ) ) {

																		nodeVar66 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

																	} else {


																		if ( ( nodeConst117 > 1.0 ) ) {

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

														let nodeConst118 = fract( ( nodeConst102.y * 4.0 ) );
														let nodeConst119 = fn5( ( nodeConst116 + 1u ) );
														let nodeConst91 = fn5( ( nodeVar2 + 119831u ) );
														let nodeConst92 = mix( vec3<f32>( 0.14702726648767014, 0.10224173307914941, 0.05612849004241121 ), vec3<f32>( 0.623960391667596, 0.6038273388475408, 0.55201140150344 ), step( 0.5, nodeConst91 ) );
														nodeVar62 = mix( mix( vec3<f32>( 0.015208514418949472, 0.012983032338510335, 0.010960094003125918 ), nodeVar63, ( ( ( step( 0.08, nodeConst118 ) * step( nodeConst118, mix( 0.4, 0.85, fract( ( nodeConst119 * 5.0 ) ) ) ) ) * step( abs( ( fract( ( nodeConst102.z * nodeConst115 ) ) - 0.5 ) ), mix( 0.26, 0.46, fract( ( nodeConst119 * 13.0 ) ) ) ) ) * step( 0.15, nodeConst119 ) ) ), nodeConst92, step( nodeConst118, 0.08 ) );

													} else {

														let nodeConst110 = mix( vec3<f32>( 0.6866853124288864, 0.6444796819634361, 0.5647115056965487 ), vec3<f32>( 0.5209955731953768, 0.48514994004665124, 0.4232676699760063 ), nodeConst26 );
														nodeVar62 = nodeConst110;

													}

													nodeVar60 = nodeVar62;

												}

												nodeVar58 = nodeVar60;

											}

											nodeVar41 = nodeVar58;

										}


										if ( nodeConst103 ) {

											nodeVar68 = ( ( smoothstep( 0.0, 0.15, nodeConst102.x ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst102.x ) ) ) * ( smoothstep( 0.0, 0.15, nodeConst102.y ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst102.y ) ) ) );

										} else {

											let nodeConst114 = ( nodeConst102.y < 0.002 );
											let nodeConst113 = ( nodeConst102.y > 0.998 );

											if ( ( nodeConst114 || nodeConst113 ) ) {

												nodeVar69 = ( ( smoothstep( 0.0, 0.15, nodeConst102.x ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst102.x ) ) ) * ( smoothstep( 0.0, 0.15, nodeConst102.z ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst102.z ) ) ) );

											} else {

												nodeVar69 = ( ( smoothstep( 0.0, 0.15, nodeConst102.y ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst102.y ) ) ) * ( smoothstep( 0.0, 0.15, nodeConst102.z ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst102.z ) ) ) );

											}

											nodeVar68 = nodeVar69;

										}

										nodeVar39 = ( ( nodeVar41 * vec3<f32>( mix( 0.72, 1.0, nodeVar68 ) ) ) * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeConst15 ), clamp( ( ( nodeConst101.z - 0.1 ) / nodeConst2 ), 0.0, 1.0 ) ) ) );

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


			if ( nodeConst84 ) {

				nodeVar71 = 0.0;

			} else {


				if ( nodeConst83 ) {

					nodeVar72 = 1.0;

				} else {


					if ( nodeConst82 ) {

						nodeVar73 = 1.0;

					} else {


						if ( nodeConst81 ) {

							nodeVar74 = 1.0;

						} else {


							if ( nodeConst80 ) {

								nodeVar75 = 1.0;

							} else {


								if ( nodeConst79 ) {

									nodeVar76 = 1.0;

								} else {


									if ( nodeConst78 ) {

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

			nodeVar1 = vec4<f32>( ( ( nodeVar3 * mix( vec3<f32>( 1.0, 1.0, 1.0 ), nodeVar70, ( nodeConst15 * 0.6 ) ) ) * vec3<f32>( mix( 0.45, 1.5, nodeConst15 ) ) ), ( nodeConst15 * nodeVar71 ) );

		} else {

			let nodeConst120 = floor( ( nodeVarying4 * vec3<f32>( 2.0 ) ) );
			nodeVar78 = ( ( ( u32( ( nodeConst120.x + 2097152.0 ) ) * 73856093u ) ^ ( u32( ( nodeConst120.y + 2097152.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeConst120.z + 2097152.0 ) ) * 83492791u ) );
			let nodeConst121 = vec3<f32>( ( nodeVarying10.x * 0.5 ), ( nodeVarying10.y * 0.5 ), ( 0.1 + ( nodeVarying10.y * 1.55 ) ) );
			let nodeConst122 = fn5( ( nodeVar78 + 119831u ) );
			let nodeConst123 = smoothstep( 0.3, 1.0, nodeConst122 );
			let nodeConst124 = ( nodeConst121.x * ( nodeConst123 * nodeConst123 ) );
			let nodeConst125 = vec3<f32>( ( - nodeConst121.x ), ( - nodeConst121.y ), 0.1 );
			let nodeConst126 = ( positionLocal - nodeVarying4 );
			normalLocal = nodeVarying11;
			let nodeConst127 = normalize( cross( vec3<f32>( 0.0, 1.0, 0.0 ), normalLocal ) );
			let nodeConst128 = vec3<f32>( dot( nodeConst126, nodeConst127 ), nodeConst126.y, 0.0 );
			let nodeConst129 = normalize( ( positionLocal - ( object.nodeUniform0 * vec4<f32>( render.cameraPosition, 1.0 ) ).xyz ) );
			let nodeConst130 = vec3<f32>( dot( nodeConst129, nodeConst127 ), nodeConst129.y, ( - dot( nodeConst129, normalLocal ) ) );
			let nodeConst131 = ( ( vec3<f32>( ( nodeConst121.x - nodeConst124 ), nodeConst125.y, 0.1 ) - nodeConst128 ) / nodeConst130 );
			let nodeConst132 = ( 0.1 + 0.12 );
			let nodeConst133 = ( ( vec3<f32>( nodeConst121.x, nodeConst121.y, nodeConst132 ) - nodeConst128 ) / nodeConst130 );
			let nodeConst134 = max( nodeConst131, nodeConst133 );
			let nodeConst135 = min( nodeConst131, nodeConst133 );
			let nodeConst136 = max( max( nodeConst135.x, nodeConst135.y ), nodeConst135.z );
			let nodeConst137 = ( ( vec3<f32>( ( - nodeConst121.x ), nodeConst125.y, 0.1 ) - nodeConst128 ) / nodeConst130 );
			let nodeConst138 = fn5( ( nodeVar78 + 105097u ) );
			let nodeConst139 = smoothstep( 0.3, 1.0, nodeConst138 );
			let nodeConst140 = ( nodeConst121.x * ( nodeConst139 * nodeConst139 ) );
			let nodeConst141 = ( ( vec3<f32>( ( ( - nodeConst121.x ) + nodeConst140 ), nodeConst121.y, nodeConst132 ) - nodeConst128 ) / nodeConst130 );
			let nodeConst142 = max( nodeConst137, nodeConst141 );
			let nodeConst143 = min( nodeConst137, nodeConst141 );
			let nodeConst144 = max( max( nodeConst143.x, nodeConst143.y ), nodeConst143.z );
			let nodeConst145 = ( nodeConst121.x * 0.82 );
			let nodeConst146 = vec3<f32>( ( nodeConst145 - 0.5 ), nodeConst125.y, ( nodeConst121.z - 0.7 ) );
			let nodeConst147 = ( ( nodeConst146 - nodeConst128 ) / nodeConst130 );
			let nodeConst148 = vec3<f32>( ( nodeConst145 + 0.5 ), ( nodeConst125.y + mix( 1.7, 2.3, nodeConst122 ) ), ( nodeConst121.z - 0.1 ) );
			let nodeConst149 = ( ( nodeConst148 - nodeConst128 ) / nodeConst130 );
			let nodeConst150 = max( nodeConst147, nodeConst149 );
			let nodeConst151 = min( nodeConst147, nodeConst149 );
			let nodeConst152 = max( max( nodeConst151.x, nodeConst151.y ), nodeConst151.z );
			let nodeConst153 = ( nodeConst121.x * -0.82 );
			let nodeConst154 = vec3<f32>( ( nodeConst153 - 0.5 ), nodeConst125.y, ( nodeConst121.z - 0.7 ) );
			let nodeConst155 = ( ( nodeConst154 - nodeConst128 ) / nodeConst130 );
			let nodeConst156 = vec3<f32>( ( nodeConst153 + 0.5 ), ( nodeConst125.y + mix( 1.7, 2.3, nodeConst138 ) ), ( nodeConst121.z - 0.1 ) );
			let nodeConst157 = ( ( nodeConst156 - nodeConst128 ) / nodeConst130 );
			let nodeConst158 = max( nodeConst155, nodeConst157 );
			let nodeConst159 = min( nodeConst155, nodeConst157 );
			let nodeConst160 = max( max( nodeConst159.x, nodeConst159.y ), nodeConst159.z );
			let nodeConst161 = mix( ( nodeConst121.x * -0.3 ), ( nodeConst121.x * 0.3 ), nodeConst122 );
			let nodeConst162 = vec3<f32>( ( nodeConst161 - 1.1 ), nodeConst125.y, ( nodeConst121.z - 0.95 ) );
			let nodeConst163 = ( ( nodeConst162 - nodeConst128 ) / nodeConst130 );
			let nodeConst164 = vec3<f32>( ( nodeConst161 + 1.1 ), ( nodeConst125.y + mix( 0.8, 0.9, nodeConst138 ) ), ( nodeConst121.z - 0.1 ) );
			let nodeConst165 = ( ( nodeConst164 - nodeConst128 ) / nodeConst130 );
			let nodeConst166 = max( nodeConst163, nodeConst165 );
			let nodeConst167 = min( nodeConst163, nodeConst165 );
			let nodeConst168 = max( max( nodeConst167.x, nodeConst167.y ), nodeConst167.z );
			let nodeConst169 = mix( -0.6, 0.6, nodeConst138 );
			let nodeConst170 = ( nodeVarying10.y * 1.55 );
			let nodeConst171 = ( ( 0.1 + ( nodeConst170 * 0.5 ) ) + mix( -0.4, 0.5, nodeConst122 ) );
			let nodeConst172 = vec3<f32>( ( nodeConst169 - 0.6 ), nodeConst125.y, ( nodeConst171 - 0.35 ) );
			let nodeConst173 = ( ( nodeConst172 - nodeConst128 ) / nodeConst130 );
			let nodeConst174 = vec3<f32>( ( nodeConst169 + 0.6 ), ( nodeConst125.y + 0.42 ), ( nodeConst171 + 0.35 ) );
			let nodeConst175 = ( ( nodeConst174 - nodeConst128 ) / nodeConst130 );
			let nodeConst176 = max( nodeConst173, nodeConst175 );
			let nodeConst177 = min( nodeConst173, nodeConst175 );
			let nodeConst178 = max( max( nodeConst177.x, nodeConst177.y ), nodeConst177.z );
			let nodeConst179 = max( ( ( nodeConst125 - nodeConst128 ) / nodeConst130 ), ( ( nodeConst121 - nodeConst128 ) / nodeConst130 ) );
			let nodeConst180 = min( min( nodeConst179.x, nodeConst179.y ), nodeConst179.z );
			let nodeConst181 = ( ( ( min( min( nodeConst176.x, nodeConst176.y ), nodeConst176.z ) > nodeConst178 ) && ( nodeConst178 > 0.0 ) ) && ( nodeConst178 < nodeConst180 ) );

			if ( nodeConst181 ) {

				nodeVar84 = nodeConst178;

			} else {

				nodeVar84 = nodeConst180;

			}

			let nodeConst182 = ( ( ( min( min( nodeConst166.x, nodeConst166.y ), nodeConst166.z ) > nodeConst168 ) && ( nodeConst168 > 0.0 ) ) && ( nodeConst168 < nodeVar84 ) );

			if ( nodeConst182 ) {

				nodeVar83 = nodeConst168;

			} else {

				nodeVar83 = nodeVar84;

			}

			let nodeConst183 = ( ( ( ( min( min( nodeConst158.x, nodeConst158.y ), nodeConst158.z ) > nodeConst160 ) && ( nodeConst160 > 0.0 ) ) && ( fn5( ( nodeVar78 + 15070u ) ) > 0.4 ) ) && ( nodeConst160 < nodeVar83 ) );

			if ( nodeConst183 ) {

				nodeVar82 = nodeConst160;

			} else {

				nodeVar82 = nodeVar83;

			}

			let nodeConst184 = ( ( ( ( min( min( nodeConst150.x, nodeConst150.y ), nodeConst150.z ) > nodeConst152 ) && ( nodeConst152 > 0.0 ) ) && ( fn5( ( nodeVar78 + 8200u ) ) > 0.4 ) ) && ( nodeConst152 < nodeVar82 ) );

			if ( nodeConst184 ) {

				nodeVar81 = nodeConst152;

			} else {

				nodeVar81 = nodeVar82;

			}

			let nodeConst185 = ( ( ( ( min( min( nodeConst142.x, nodeConst142.y ), nodeConst142.z ) > nodeConst144 ) && ( nodeConst144 > 0.0 ) ) && ( nodeConst140 > 0.05 ) ) && ( nodeConst144 < nodeVar81 ) );

			if ( nodeConst185 ) {

				nodeVar80 = nodeConst144;

			} else {

				nodeVar80 = nodeVar81;

			}

			let nodeConst186 = ( ( ( ( min( min( nodeConst134.x, nodeConst134.y ), nodeConst134.z ) > nodeConst136 ) && ( nodeConst136 > 0.0 ) ) && ( nodeConst124 > 0.05 ) ) && ( nodeConst136 < nodeVar80 ) );

			if ( nodeConst186 ) {

				let nodeConst187 = ( fn5( ( nodeVar78 + 125490u ) ) * 6.0 );

				if ( ( nodeConst187 > 5.0 ) ) {

					nodeVar85 = mix( vec3<f32>( 0.26225065751888765, 0.10224173307914941, 0.057805430183792694 ), vec3<f32>( 0.3231432091022285, 0.14412847084818123, 0.08437621153575764 ), nodeConst122 );

				} else {


					if ( ( nodeConst187 > 4.0 ) ) {

						nodeVar86 = mix( vec3<f32>( 0.1499597898006365, 0.17788841597328695, 0.09758734713304495 ), vec3<f32>( 0.1912016827303171, 0.22696587349938613, 0.11443537381770343 ), nodeConst122 );

					} else {


						if ( ( nodeConst187 > 3.0 ) ) {

							nodeVar87 = mix( vec3<f32>( 0.11443537381770343, 0.16202937562896222, 0.1912016827303171 ), vec3<f32>( 0.158960835050774, 0.21952619971859377, 0.25818285291079235 ), nodeConst122 );

						} else {


							if ( ( nodeConst187 > 2.0 ) ) {

								nodeVar88 = mix( vec3<f32>( 0.16202937562896222, 0.14412847084818123, 0.12743768042608497 ), vec3<f32>( 0.22696587349938613, 0.20507873637973145, 0.181164244239483 ), nodeConst122 );

							} else {


								if ( ( nodeConst187 > 1.0 ) ) {

									nodeVar89 = mix( vec3<f32>( 0.2541520943200296, 0.19461783043107173, 0.12743768042608497 ), vec3<f32>( 0.3277780980458375, 0.26225065751888765, 0.16826940017946088 ), nodeConst122 );

								} else {

									nodeVar89 = mix( vec3<f32>( 0.5906188409113381, 0.5209955731953768, 0.3813260114221238 ), vec3<f32>( 0.6866853124288864, 0.6104955708001716, 0.4793201830913402 ), nodeConst122 );

								}

								nodeVar88 = nodeVar89;

							}

							nodeVar87 = nodeVar88;

						}

						nodeVar86 = nodeVar87;

					}

					nodeVar85 = nodeVar86;

				}

				let nodeConst188 = ( nodeConst128 + ( nodeConst130 * vec3<f32>( nodeConst136 ) ) );
				nodeVar79 = ( ( nodeVar85 * vec3<f32>( mix( 0.78, 1.12, fract( ( nodeConst188.x * 2.5 ) ) ) ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeConst188.z - 0.1 ) / nodeConst170 ), 0.0, 1.0 ) ) ) );

			} else {


				if ( nodeConst185 ) {

					let nodeConst187 = ( fn5( ( nodeVar78 + 125490u ) ) * 6.0 );

					if ( ( nodeConst187 > 5.0 ) ) {

						nodeVar91 = mix( vec3<f32>( 0.26225065751888765, 0.10224173307914941, 0.057805430183792694 ), vec3<f32>( 0.3231432091022285, 0.14412847084818123, 0.08437621153575764 ), nodeConst122 );

					} else {


						if ( ( nodeConst187 > 4.0 ) ) {

							nodeVar92 = mix( vec3<f32>( 0.1499597898006365, 0.17788841597328695, 0.09758734713304495 ), vec3<f32>( 0.1912016827303171, 0.22696587349938613, 0.11443537381770343 ), nodeConst122 );

						} else {


							if ( ( nodeConst187 > 3.0 ) ) {

								nodeVar93 = mix( vec3<f32>( 0.11443537381770343, 0.16202937562896222, 0.1912016827303171 ), vec3<f32>( 0.158960835050774, 0.21952619971859377, 0.25818285291079235 ), nodeConst122 );

							} else {


								if ( ( nodeConst187 > 2.0 ) ) {

									nodeVar94 = mix( vec3<f32>( 0.16202937562896222, 0.14412847084818123, 0.12743768042608497 ), vec3<f32>( 0.22696587349938613, 0.20507873637973145, 0.181164244239483 ), nodeConst122 );

								} else {


									if ( ( nodeConst187 > 1.0 ) ) {

										nodeVar95 = mix( vec3<f32>( 0.2541520943200296, 0.19461783043107173, 0.12743768042608497 ), vec3<f32>( 0.3277780980458375, 0.26225065751888765, 0.16826940017946088 ), nodeConst122 );

									} else {

										nodeVar95 = mix( vec3<f32>( 0.5906188409113381, 0.5209955731953768, 0.3813260114221238 ), vec3<f32>( 0.6866853124288864, 0.6104955708001716, 0.4793201830913402 ), nodeConst122 );

									}

									nodeVar94 = nodeVar95;

								}

								nodeVar93 = nodeVar94;

							}

							nodeVar92 = nodeVar93;

						}

						nodeVar91 = nodeVar92;

					}

					let nodeConst189 = ( nodeConst128 + ( nodeConst130 * vec3<f32>( nodeConst144 ) ) );
					nodeVar90 = ( ( nodeVar91 * vec3<f32>( mix( 0.78, 1.12, fract( ( nodeConst189.x * 2.5 ) ) ) ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeConst189.z - 0.1 ) / nodeConst170 ), 0.0, 1.0 ) ) ) );

				} else {


					if ( nodeConst184 ) {

						let nodeConst190 = ( nodeConst128 + ( nodeConst130 * vec3<f32>( nodeConst152 ) ) );

						if ( ( ( ( nodeConst190 - nodeConst146 ) / ( nodeConst148 - nodeConst146 ) ).y > 0.94 ) ) {

							nodeVar97 = 1.2;

						} else {

							nodeVar97 = 0.82;

						}

						nodeVar96 = ( ( mix( vec3<f32>( 0.04231141061442144, 0.025186859622305935, 0.015996293361446288 ), vec3<f32>( 0.09084171117479915, 0.06301001764564068, 0.04231141061442144 ), nodeConst138 ) * vec3<f32>( nodeVar97 ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeConst190.z - 0.1 ) / nodeConst170 ), 0.0, 1.0 ) ) ) );

					} else {


						if ( nodeConst183 ) {

							let nodeConst191 = ( nodeConst128 + ( nodeConst130 * vec3<f32>( nodeConst160 ) ) );

							if ( ( ( ( nodeConst191 - nodeConst154 ) / ( nodeConst156 - nodeConst154 ) ).y > 0.94 ) ) {

								nodeVar99 = 1.2;

							} else {

								nodeVar99 = 0.82;

							}

							nodeVar98 = ( ( mix( vec3<f32>( 0.04231141061442144, 0.025186859622305935, 0.015996293361446288 ), vec3<f32>( 0.09084171117479915, 0.06301001764564068, 0.04231141061442144 ), nodeConst138 ) * vec3<f32>( nodeVar99 ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeConst191.z - 0.1 ) / nodeConst170 ), 0.0, 1.0 ) ) ) );

						} else {


							if ( nodeConst182 ) {

								let nodeConst192 = ( nodeConst128 + ( nodeConst130 * vec3<f32>( nodeConst168 ) ) );

								if ( ( ( ( nodeConst192 - nodeConst162 ) / ( nodeConst164 - nodeConst162 ) ).y > 0.9 ) ) {

									nodeVar101 = 1.12;

								} else {

									nodeVar101 = 0.85;

								}

								nodeVar100 = ( ( mix( vec3<f32>( 0.10224173307914941, 0.06847816983662762, 0.04231141061442144 ), vec3<f32>( 0.054480276435339814, 0.09305896283800832, 0.14412847084818123 ), nodeConst138 ) * vec3<f32>( nodeVar101 ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeConst192.z - 0.1 ) / nodeConst170 ), 0.0, 1.0 ) ) ) );

							} else {


								if ( nodeConst181 ) {

									let nodeConst193 = ( nodeConst128 + ( nodeConst130 * vec3<f32>( nodeConst178 ) ) );

									if ( ( ( ( nodeConst193 - nodeConst172 ) / ( nodeConst174 - nodeConst172 ) ).y > 0.94 ) ) {

										nodeVar103 = 1.25;

									} else {

										nodeVar103 = 0.8;

									}

									nodeVar102 = ( ( mix( vec3<f32>( 0.06847816983662762, 0.035601314869097636, 0.019382360952473074 ), vec3<f32>( 0.14702726648767014, 0.06847816983662762, 0.02955683443236377 ), nodeConst122 ) * vec3<f32>( nodeVar103 ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeConst193.z - 0.1 ) / nodeConst170 ), 0.0, 1.0 ) ) ) );

								} else {

									let nodeConst194 = ( nodeConst128 + ( nodeConst130 * vec3<f32>( nodeConst180 ) ) );
									let nodeConst195 = ( ( nodeConst194 - nodeConst125 ) / ( nodeConst121 - nodeConst125 ) );
									let nodeConst196 = ( nodeConst195.z > 0.998 );

									if ( nodeConst196 ) {

										let nodeConst197 = mix( mix( vec3<f32>( 0.3231432091022285, 0.25818285291079235, 0.171441100722554 ), vec3<f32>( 0.158960835050774, 0.19461783043107173, 0.22322795730611386 ), nodeConst138 ), vec3<f32>( 0.48514994004665124, 0.4178850708380236, 0.3094689228067428 ), ( nodeConst122 * 0.6 ) );
										let nodeConst198 = mix( nodeConst197, ( nodeConst197 * vec3<f32>( 0.5 ) ), smoothstep( 0.05, 0.04, nodeConst195.y ) );
										let nodeConst199 = mix( 0.22, 0.78, nodeConst138 );

										if ( ( nodeConst199 < 0.5 ) ) {

											nodeVar105 = mix( 0.68, 0.82, nodeConst122 );

										} else {

											nodeVar105 = mix( 0.18, 0.32, nodeConst122 );

										}

										nodeVar104 = mix( mix( mix( nodeConst198, mix( vec3<f32>( 0.10224173307914941, 0.061246054224174035, 0.030713443727452196 ), vec3<f32>( 0.040915196900556984, 0.03954623527052923, 0.04518620437910499 ), step( 0.5, nodeConst122 ) ), ( smoothstep( 0.09100000000000001, 0.079, abs( ( nodeConst195.x - nodeConst199 ) ) ) * smoothstep( 0.356, 0.344, abs( ( nodeConst195.y - 0.33 ) ) ) ) ), vec3<f32>( 0.0069954101845983935, 0.006048833020386069, 0.005181516700061659 ), ( smoothstep( 0.081, 0.06899999999999999, abs( ( nodeConst195.x - nodeVar105 ) ) ) * smoothstep( 0.09100000000000001, 0.079, abs( ( nodeConst195.y - 0.56 ) ) ) ) ), mix( vec3<f32>( 0.025186859622305935, 0.04231141061442144, 0.06847816983662762 ), vec3<f32>( 0.19461783043107173, 0.10224173307914941, 0.04231141061442144 ), fn5( ( nodeVar78 + 11240u ) ) ), ( smoothstep( 0.061, 0.049, abs( ( nodeConst195.x - nodeVar105 ) ) ) * smoothstep( 0.07100000000000001, 0.059000000000000004, abs( ( nodeConst195.y - 0.56 ) ) ) ) );

									} else {

										let nodeConst200 = ( nodeConst195.y > 0.998 );

										if ( nodeConst200 ) {

											let nodeConst197 = mix( mix( vec3<f32>( 0.3231432091022285, 0.25818285291079235, 0.171441100722554 ), vec3<f32>( 0.158960835050774, 0.19461783043107173, 0.22322795730611386 ), nodeConst138 ), vec3<f32>( 0.48514994004665124, 0.4178850708380236, 0.3094689228067428 ), ( nodeConst122 * 0.6 ) );

											if ( ( fn5( ( nodeVar78 + 25910u ) ) > 0.88 ) ) {

												nodeVar107 = mix( vec3<f32>( 0.7379104087672317, 0.8069522576650873, 1.0 ), vec3<f32>( 0.34670405634441115, 0.46778379610254284, 1.0 ), fn5( ( nodeVar78 + 59550u ) ) );

											} else {

												nodeVar107 = mix( vec3<f32>( 1.0, 0.4793201830913402, 0.059511238155621766 ), vec3<f32>( 1.0, 0.775822218312646, 0.33245153633549385 ), fn5( ( nodeVar78 + 86350u ) ) );

											}

											let nodeConst201 = step( 0.8, fn5( ( nodeVar78 + 79599u ) ) );
											nodeVar106 = mix( mix( nodeConst197, vec3<f32>( 1.0, 1.0, 1.0 ), 0.5 ), ( nodeVar107 * vec3<f32>( mix( 1.0, 4.5, nodeConst201 ) ) ), smoothstep( 0.16, 0.13, length( vec2<f32>( ( nodeConst195.x - 0.5 ), ( nodeConst195.z - 0.5 ) ) ) ) );

										} else {

											let nodeConst202 = ( nodeConst195.y < 0.002 );

											if ( nodeConst202 ) {

												nodeVar108 = mix( ( mix( vec3<f32>( 0.06847816983662762, 0.033104766565152086, 0.014443843592229466 ), vec3<f32>( 0.14412847084818123, 0.0722718506743852, 0.02955683443236377 ), nodeConst138 ) * vec3<f32>( ( 1.0 - ( step( 0.94, fract( ( nodeConst195.x * 6.0 ) ) ) * 0.3 ) ) ) ), mix( vec3<f32>( 0.19461783043107173, 0.04373502925049377, 0.031896033067374104 ), vec3<f32>( 0.04231141061442144, 0.09530746662221588, 0.11697066774917994 ), nodeConst122 ), ( ( smoothstep( 0.306, 0.294, abs( ( nodeConst195.x - 0.5 ) ) ) * smoothstep( 0.266, 0.254, abs( ( nodeConst195.z - 0.62 ) ) ) ) * 0.9 ) );

											} else {

												let nodeConst197 = mix( mix( vec3<f32>( 0.3231432091022285, 0.25818285291079235, 0.171441100722554 ), vec3<f32>( 0.158960835050774, 0.19461783043107173, 0.22322795730611386 ), nodeConst138 ), vec3<f32>( 0.48514994004665124, 0.4178850708380236, 0.3094689228067428 ), ( nodeConst122 * 0.6 ) );
												let nodeConst198 = mix( nodeConst197, ( nodeConst197 * vec3<f32>( 0.5 ) ), smoothstep( 0.05, 0.04, nodeConst195.y ) );
												nodeVar108 = nodeConst198;

											}

											nodeVar106 = nodeVar108;

										}

										nodeVar104 = nodeVar106;

									}


									if ( nodeConst196 ) {

										nodeVar109 = ( ( smoothstep( 0.0, 0.15, nodeConst195.x ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst195.x ) ) ) * ( smoothstep( 0.0, 0.15, nodeConst195.y ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst195.y ) ) ) );

									} else {

										let nodeConst202 = ( nodeConst195.y < 0.002 );
										let nodeConst200 = ( nodeConst195.y > 0.998 );

										if ( ( nodeConst202 || nodeConst200 ) ) {

											nodeVar110 = ( ( smoothstep( 0.0, 0.15, nodeConst195.x ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst195.x ) ) ) * ( smoothstep( 0.0, 0.15, nodeConst195.z ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst195.z ) ) ) );

										} else {

											nodeVar110 = ( ( smoothstep( 0.0, 0.15, nodeConst195.y ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst195.y ) ) ) * ( smoothstep( 0.0, 0.15, nodeConst195.z ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeConst195.z ) ) ) );

										}

										nodeVar109 = nodeVar110;

									}

									nodeVar102 = ( ( nodeVar104 * vec3<f32>( mix( 0.72, 1.0, nodeVar109 ) ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeConst194.z - 0.1 ) / nodeConst170 ), 0.0, 1.0 ) ) ) );

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

			let nodeConst201 = step( 0.8, fn5( ( nodeVar78 + 79599u ) ) );

			if ( nodeConst186 ) {

				nodeVar112 = 0.2;

			} else {


				if ( nodeConst185 ) {

					nodeVar113 = 0.2;

				} else {


					if ( nodeConst184 ) {

						nodeVar114 = 1.0;

					} else {


						if ( nodeConst183 ) {

							nodeVar115 = 1.0;

						} else {


							if ( nodeConst182 ) {

								nodeVar116 = 1.0;

							} else {


								if ( nodeConst181 ) {

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

			nodeVar1 = vec4<f32>( ( ( nodeVar79 * mix( vec3<f32>( 1.0, 1.0, 1.0 ), nodeVar111, ( nodeConst201 * 0.85 ) ) ) * vec3<f32>( mix( 1.0, 1.3, nodeConst201 ) ) ), ( nodeConst201 * nodeVar112 ) );

		}

		nodeVar118 = nodeVar1;

		if ( nodeConst0 ) {

			nodeVar119 = vec3<f32>( 0.6038273388475408, 0.6583748172725346, 0.623960391667596 );

		} else {

			nodeVar119 = vec3<f32>( 0.46778379610254284, 0.5647115056965487, 0.5209955731953768 );

		}


		if ( nodeConst0 ) {

			nodeVar120 = vec3<f32>( 0.008023192982520563, 0.010329823026364548, 0.012983032338510335 );

		} else {

			let nodeConst203 = ( v_positionWorld * vec3<f32>( 0.3 ) );
			nodeVar120 = mix( vec3<f32>( 0.006512090790025684, 0.008023192982520563, 0.010329823026364548 ), vec3<f32>( 0.01680737574872402, 0.024157632443547246, 0.030713443727452196 ), ( ( ( fn130( nodeConst203 ) + ( fn130( ( nodeConst203 * vec3<f32>( 2.0 ) ) ) * 0.5 ) ) * 0.5 ) + 0.5 ) );

		}


		if ( nodeConst0 ) {

			let nodeConst204 = ( smoothstep( -0.15, 0.5, ( fn87( vec3<f32>( ( v_positionWorld.x * 1.3 ), ( v_positionWorld.y * 0.06 ), ( v_positionWorld.z * 1.3 ) ), 2, 2.0, 0.5 ) * 1.0 ) ) * 0.45 );
			nodeVar121 = ( 0.14 + ( nodeConst204 * 0.22 ) );

		} else {

			let nodeConst204 = ( smoothstep( -0.15, 0.5, ( fn87( vec3<f32>( ( v_positionWorld.x * 1.3 ), ( v_positionWorld.y * 0.06 ), ( v_positionWorld.z * 1.3 ) ), 2, 2.0, 0.5 ) * 1.0 ) ) * 0.45 );
			nodeVar121 = clamp( ( ( 0.64 + nodeConst204 ) + ( smoothstep( 0.32, 0.0, nodeVarying12.y ) * 0.4 ) ), 0.0, 0.95 );

		}

		nodeVar0 = mix( ( nodeVar118.xyz * nodeVar119 ), nodeVar120, nodeVar121 );

	} else {


		if ( ( nodeVarying3 == 7.0 ) ) {

			nodeVar122 = mix( ( object.nodeUniform3 * vec3<f32>( 0.3 ) ), vec3<f32>( 0.01764195448412081, 0.014443843592229466, 0.011612245176281512 ), 0.5 );

		} else {


			if ( ( nodeVarying3 == 8.0 ) ) {

				let nodeConst205 = floor( ( vec2<f32>( v_positionWorld.x, v_positionWorld.z ) * vec2<f32>( 0.2 ) ) );
				let nodeConst206 = ( fn5( ( ( u32( ( nodeConst205.x + 65536.0 ) ) * 73856093u ) ^ ( u32( ( nodeConst205.y + 65536.0 ) ) * 19349663u ) ) ) * 5.0 );

				if ( ( nodeConst206 > 4.0 ) ) {

					nodeVar124 = vec3<f32>( 0.014443843592229466, 0.013702083043526807, 0.015996293361446288 );

				} else {


					if ( ( nodeConst206 > 3.0 ) ) {

						nodeVar125 = vec3<f32>( 0.10224173307914941, 0.11193242782769693, 0.09758734713304495 );

					} else {


						if ( ( nodeConst206 > 2.0 ) ) {

							nodeVar126 = vec3<f32>( 0.015996293361446288, 0.03954623527052923, 0.07818742179702069 );

						} else {


							if ( ( nodeConst206 > 1.0 ) ) {

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


				if ( ( nodeVarying3 == 2.0 ) ) {

					nodeVar128 = ( object.nodeUniform3 * vec3<f32>( 0.55 ) );

				} else {

					let nodeConst207 = ( nodeVarying3 == 3.0 );

					if ( nodeConst207 ) {

						let nodeConst208 = ( nodeVarying6 * 0.18 );
						nodeVar129 = ( mix( object.nodeUniform3, vec3<f32>( 1.0, 1.0, 1.0 ), 0.22 ) * vec3<f32>( ( 1.0 + nodeConst208 ) ) );

					} else {


						if ( ( nodeVarying3 == 5.0 ) ) {

							let nodeConst209 = ( ( fn130( ( v_positionWorld * vec3<f32>( 0.4 ) ) ) * 0.5 ) + 0.5 );
							normalWorldGeometry = normalize( v_normalWorldGeometry );
							let nodeConst210 = vec3<f32>( ( v_positionWorld.x * 6.0 ), ( v_positionWorld.y * 0.5 ), ( v_positionWorld.z * 6.0 ) );
							nodeVar130 = mix( ( ( mix( vec3<f32>( 0.8879231178794776, 0.8796223968851662, 0.8387990117372213 ), vec3<f32>( 0.623960391667596, 0.6038273388475408, 0.5394794890033748 ), nodeConst209 ) + vec3<f32>( ( fn130( ( v_positionWorld * vec3<f32>( 5.0 ) ) ) * 0.04 ) ) ) * vec3<f32>( mix( 1.0, ( mix( 0.82, 1.04, fract( ( nodeVarying12.y * 6.0 ) ) ) * 0.42 ), ( ( ( ( smoothstep( 0.06, 0.14, nodeVarying12.x ) * smoothstep( 0.94, 0.86, nodeVarying12.x ) ) * smoothstep( 0.12, 0.2, nodeVarying12.y ) ) * smoothstep( 0.96, 0.88, nodeVarying12.y ) ) * ( smoothstep( 0.65, 0.4, abs( normalWorldGeometry.y ) ) * smoothstep( 0.08, 0.015, length( fwidth( v_positionWorld ) ) ) ) ) ) ) ), vec3<f32>( 0.158960835050774, 0.13843161502267545, 0.10224173307914941 ), ( ( ( smoothstep( 0.4, 0.0, nodeVarying12.y ) * ( ( ( ( fn130( nodeConst210 ) + ( fn130( ( nodeConst210 * vec3<f32>( 2.0 ) ) ) * 0.5 ) ) + ( fn130( ( nodeConst210 * vec3<f32>( 4.0 ) ) ) * 0.25 ) ) * 0.5 ) + 0.5 ) ) * ( nodeConst209 + 0.3 ) ) * 0.5 ) );

						} else {


							if ( ( nodeVarying3 == 1.0 ) ) {

								nodeVar131 = 0.12;

							} else {


								if ( nodeConst207 ) {

									nodeVar132 = 0.2;

								} else {

									nodeVar132 = 0.0;

								}

								nodeVar131 = nodeVar132;

							}

							let nodeConst208 = ( nodeVarying6 * 0.18 );
							let nodeConst211 = ( positionLocal.y / 0.3 );
							let nodeConst212 = floor( nodeConst211 );
							normalWorldGeometry = normalize( v_normalWorldGeometry );
							let nodeConst213 = ( ( ( ( positionLocal.x * normalWorldGeometry.z ) - ( positionLocal.z * normalWorldGeometry.x ) ) / 0.6 ) + ( tsl_mod_float( nodeConst212, 2.0 ) * 0.5 ) );
							nodeVar133 = ( ( u32( ( nodeConst212 + 65536.0 ) ) * 73856093u ) ^ ( u32( ( floor( nodeConst213 ) + 65536.0 ) ) * 19349663u ) );
							let nodeConst214 = ( ( fn5( ( nodeVar133 + 1u ) ) - 0.5 ) * 0.14 );
							let nodeConst215 = ( ( mix( object.nodeUniform3, vec3<f32>( 1.0, 1.0, 1.0 ), nodeVar131 ) * vec3<f32>( ( ( ( 1.0 + nodeConst208 ) + ( fn130( ( v_positionWorld * vec3<f32>( 0.7 ) ) ) * 0.06 ) ) + ( ( fn5( nodeVar133 ) - 0.5 ) * 0.14 ) ) ) ) * vec3<f32>( ( 1.0 + nodeConst214 ), 1.0, ( 1.0 - nodeConst214 ) ) );
							let nodeConst216 = abs( normalWorldGeometry );
							let nodeConst217 = clamp( ( ( ( nodeConst216.z * fwidth( v_positionWorld.x ) ) + ( nodeConst216.x * fwidth( v_positionWorld.z ) ) ) / 0.6 ), 0.000001, 0.5 );
							let nodeConst218 = max( nodeConst217, 0.008333333333333333 );
							let nodeConst219 = clamp( fwidth( nodeConst211 ), 0.000001, 0.5 );
							let nodeConst220 = max( nodeConst219, 0.016666666666666666 );
							let nodeConst221 = smoothstep( 0.7, 0.45, nodeConst216.y );
							let nodeConst222 = ( 1.0 - nodeConst221 );

							if ( ( nodeConst222 > 0.0 ) ) {

								let nodeConst223 = ( v_positionWorld * vec3<f32>( 0.025 ) );
								nodeVar134 = ( smoothstep( 0.0, 0.55, ( ( fn130( nodeConst223 ) + ( fn130( ( nodeConst223 * vec3<f32>( 2.0 ) ) ) * 0.5 ) ) + ( fn130( ( nodeConst223 * vec3<f32>( 4.0 ) ) ) * 0.25 ) ) ) * 0.22 );

							} else {

								nodeVar134 = 0.0;

							}

							nodeVar130 = mix( mix( nodeConst215, ( nodeConst215 * vec3<f32>( 0.6 ) ), ( max( ( smoothstep( ( nodeConst218 + nodeConst217 ), ( nodeConst218 - nodeConst217 ), ( 0.5 - abs( ( fract( nodeConst213 ) - 0.5 ) ) ) ) * min( ( 0.008333333333333333 / nodeConst218 ), 1.0 ) ), ( smoothstep( ( nodeConst220 + nodeConst219 ), ( nodeConst220 - nodeConst219 ), ( 0.5 - abs( ( fract( nodeConst211 ) - 0.5 ) ) ) ) * min( ( 0.016666666666666666 / nodeConst220 ), 1.0 ) ) ) * nodeConst221 ) ), vec3<f32>( 0.06847816983662762, 0.054480276435339814, 0.036889450395083165 ), mix( ( ( smoothstep( -0.1, 0.45, ( fn87( vec3<f32>( ( v_positionWorld.x * 1.5 ), ( v_positionWorld.y * 0.04 ), ( v_positionWorld.z * 1.5 ) ), 2, 2.0, 0.5 ) * 1.0 ) ) * smoothstep( 210.0, 0.0, v_positionWorld.y ) ) * 0.6 ), nodeVar134, nodeConst222 ) );

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

	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), ( 1.0 * vec4<f32>( nodeVar0, 1.0 ).w ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform6 );
	let nodeConst224 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst224;

	// result

	output.color = nodeConst224;

	return output;

}
