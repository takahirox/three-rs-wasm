// Three.js r187 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct nodeUniform0Struct {
	value : array< vec4<f32> >
};
@binding( 0 ) @group( 1 )
var<storage, read_write> nodeUniform0 : nodeUniform0Struct;

struct nodeUniform1Struct {
	value : array< vec4<f32> >
};
@binding( 1 ) @group( 1 )
var<storage, read_write> nodeUniform1 : nodeUniform1Struct;

struct nodeUniform11Struct {
	value : array< vec4<f32> >
};
@binding( 3 ) @group( 1 )
var<storage, read_write> nodeUniform11 : nodeUniform11Struct;

struct nodeUniform15Struct {
	value : array< vec4<f32> >
};
@binding( 4 ) @group( 1 )
var<storage, read_write> nodeUniform15 : nodeUniform15Struct;

struct objectStruct {
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform9 : f32,
	nodeUniform10 : f32,
	nodeUniform12 : f32,
	nodeUniform13 : f32,
	nodeUniform14 : f32,
	nodeUniform16 : u32
};
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform8 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;

// codes
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


fn fn72 ( x : i32, y : i32, z : i32 ) -> vec3<u32> {

	var nodeVar0 : i32;
	var nodeVar1 : i32;
	var nodeVar2 : i32;
	var nodeVar3 : u32;
	var nodeVar4 : vec3<u32>;

	nodeVar0 = z;
	nodeVar1 = y;
	nodeVar2 = x;
	nodeVar3 = fn68( nodeVar2, nodeVar1, nodeVar0 );
	nodeVar4 = vec3<u32>( 0u, 0u, 0u );
	nodeVar4.x = ( nodeVar3 & 255u );
	nodeVar4.y = ( ( nodeVar3 >> 8u ) & 255u );
	nodeVar4.z = ( ( nodeVar3 >> 16u ) & 255u );

	return nodeVar4;

}


fn fn56 ( hash : vec3<u32>, x : f32, y : f32, z : f32 ) -> vec3<f32> {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : vec3<u32>;

	nodeVar0 = z;
	nodeVar1 = y;
	nodeVar2 = x;
	nodeVar3 = hash;

	return vec3<f32>( fn54( nodeVar3.x, nodeVar2, nodeVar1, nodeVar0 ), fn54( nodeVar3.y, nodeVar2, nodeVar1, nodeVar0 ), fn54( nodeVar3.z, nodeVar2, nodeVar1, nodeVar0 ) );

}


fn fn52 ( v0 : vec3<f32>, v1 : vec3<f32>, v2 : vec3<f32>, v3 : vec3<f32>, v4 : vec3<f32>, v5 : vec3<f32>, v6 : vec3<f32>, v7 : vec3<f32>, s : f32, t : f32, r : f32 ) -> vec3<f32> {

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


fn fn60 ( v : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;

	nodeVar0 = v;

	return ( vec3<f32>( 0.982 ) * nodeVar0 );

}


fn fn76 ( p : vec3<f32> ) -> vec3<f32> {

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
	nodeVar13 = fn52( fn56( fn72( nodeVar1, nodeVar2, nodeVar3 ), nodeVar5, nodeVar7, nodeVar9 ), fn56( fn72( ( nodeVar1 + 1 ), nodeVar2, nodeVar3 ), ( nodeVar5 - 1.0 ), nodeVar7, nodeVar9 ), fn56( fn72( nodeVar1, ( nodeVar2 + 1 ), nodeVar3 ), nodeVar5, ( nodeVar7 - 1.0 ), nodeVar9 ), fn56( fn72( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ), nodeVar3 ), ( nodeVar5 - 1.0 ), ( nodeVar7 - 1.0 ), nodeVar9 ), fn56( fn72( nodeVar1, nodeVar2, ( nodeVar3 + 1 ) ), nodeVar5, nodeVar7, ( nodeVar9 - 1.0 ) ), fn56( fn72( ( nodeVar1 + 1 ), nodeVar2, ( nodeVar3 + 1 ) ), ( nodeVar5 - 1.0 ), nodeVar7, ( nodeVar9 - 1.0 ) ), fn56( fn72( nodeVar1, ( nodeVar2 + 1 ), ( nodeVar3 + 1 ) ), nodeVar5, ( nodeVar7 - 1.0 ), ( nodeVar9 - 1.0 ) ), fn56( fn72( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ), ( nodeVar3 + 1 ) ), ( nodeVar5 - 1.0 ), ( nodeVar7 - 1.0 ), ( nodeVar9 - 1.0 ) ), nodeVar10, nodeVar11, nodeVar12 );

	return fn60( nodeVar13 );

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


fn fn88 ( p : vec3<f32>, octaves : i32, lacunarity : f32, diminish : f32 ) -> vec3<f32> {

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

		nodeVar3 = ( nodeVar3 + ( vec3<f32>( nodeVar4 ) * fn76( nodeVar2 ) ) );
		nodeVar4 = ( nodeVar4 * nodeVar0 );
		nodeVar2 = ( nodeVar2 * vec3<f32>( nodeVar1 ) );

	}


	return nodeVar3;

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


fn fn34 ( color : vec3<f32>, adjustment : f32 ) -> vec3<f32> {

	

	let nodeConst0 = cos( adjustment );

	return max( ( ( color * vec3<f32>( nodeConst0 ) ) + ( ( cross( vec3<f32>( 0.57735, 0.57735, 0.57735 ), color ) * vec3<f32>( sin( adjustment ) ) ) + ( vec3<f32>( 0.57735, 0.57735, 0.57735 ) * vec3<f32>( ( dot( vec3<f32>( 0.57735, 0.57735, 0.57735 ), color ) * ( 1.0 - nodeConst0 ) ) ) ) ) ), vec3<f32>( 0.0 ) );

}




@compute @workgroup_size( 64, 1, 1 )
fn main( @builtin( global_invocation_id ) globalId : vec3<u32>,
	@builtin( workgroup_id ) workgroupId : vec3<u32>,
	@builtin( local_invocation_id ) localId : vec3<u32>,
	@builtin( num_workgroups ) numWorkgroups : vec3<u32> ) {

	// local vars
	

	// system
	instanceIndex = globalId.x
		+ globalId.y * ( 64 * numWorkgroups.x )
		+ globalId.z * ( 64 * numWorkgroups.x ) * ( 1 * numWorkgroups.y );

	// flow
	// code


	// flow -> Update Particles
	if ( instanceIndex >= object.nodeUniform16 ) { return; }


	if ( ( nodeUniform0.value[ instanceIndex ].w > 0.0 ) ) {

		nodeVar0 = ( nodeUniform1.value[ instanceIndex ].xyz + ( ( fn88( ( nodeUniform0.value[ instanceIndex ].xyz * vec3<f32>( object.nodeUniform2 ) ), i32( object.nodeUniform3 ), object.nodeUniform4, object.nodeUniform5 ) * vec3<f32>( object.nodeUniform6 ) ) * vec3<f32>( ( nodeUniform0.value[ instanceIndex ].w + 0.01 ) ) ) );
		nodeUniform1.value[ instanceIndex ].x = nodeVar0[ 0 ];
		nodeUniform1.value[ instanceIndex ].y = nodeVar0[ 1 ];
		nodeUniform1.value[ instanceIndex ].z = nodeVar0[ 2 ];
		nodeVar1 = ( nodeUniform1.value[ instanceIndex ].xyz * vec3<f32>( ( 1.0 - object.nodeUniform7 ) ) );
		nodeUniform1.value[ instanceIndex ].x = nodeVar1[ 0 ];
		nodeUniform1.value[ instanceIndex ].y = nodeVar1[ 1 ];
		nodeUniform1.value[ instanceIndex ].z = nodeVar1[ 2 ];
		let nodeConst0 = ( ( render.nodeUniform8 * 0.1 ) * object.nodeUniform9 );
		nodeVar2 = ( nodeUniform0.value[ instanceIndex ].xyz + ( nodeUniform1.value[ instanceIndex ].xyz * vec3<f32>( nodeConst0 ) ) );
		nodeUniform0.value[ instanceIndex ].x = nodeVar2[ 0 ];
		nodeUniform0.value[ instanceIndex ].y = nodeVar2[ 1 ];
		nodeUniform0.value[ instanceIndex ].z = nodeVar2[ 2 ];
		nodeUniform0.value[ instanceIndex ].w = ( nodeUniform0.value[ instanceIndex ].w - ( nodeConst0 * ( 1.0 / object.nodeUniform10 ) ) );
		nodeVar3 = 10000.0;
		nodeVar4 = vec3<f32>( 0.0, 0.0, 0.0 );
		nodeVar5 = 0.0;
		nodeVar6 = 10000.0;
		nodeVar7 = vec3<f32>( 0.0, 0.0, 0.0 );
		nodeVar8 = 0.0;

		for ( var i : i32 = 0; i < 8192; i ++ ) {


			if ( ( ( i != i32( instanceIndex ) ) && ( nodeUniform0.value[ i ].w > 0.0 ) ) ) {

				let nodeConst1 = ( nodeUniform0.value[ instanceIndex ].xyz - nodeUniform0.value[ i ].xyz );
				let nodeConst2 = dot( nodeConst1, nodeConst1 );
				let nodeConst3 = ( nodeConst2 > 0.0 );

				if ( ( ( nodeConst2 < nodeVar3 ) && nodeConst3 ) ) {

					nodeVar3 = nodeConst2;
					nodeVar4 = nodeUniform0.value[ i ].xyz;
					nodeVar5 = nodeUniform0.value[ i ].w;
					

				} else {


					if ( ( ( nodeConst2 < nodeVar6 ) && nodeConst3 ) ) {

						nodeVar6 = nodeConst2;
						nodeVar7 = nodeUniform0.value[ i ].xyz;
						nodeVar8 = nodeUniform0.value[ i ].w;
						

					}

					

				}

				

			}


		}

		let nodeConst4 = ( instanceIndex * 8u );
		nodeVar9 = nodeUniform0.value[ instanceIndex ].xyz;
		nodeUniform11.value[ nodeConst4 ].x = nodeVar9[ 0 ];
		nodeUniform11.value[ nodeConst4 ].y = nodeVar9[ 1 ];
		nodeUniform11.value[ nodeConst4 ].z = nodeVar9[ 2 ];
		nodeUniform11.value[ nodeConst4 ].y = ( nodeUniform11.value[ nodeConst4 ].y + object.nodeUniform12 );
		nodeVar10 = nodeUniform0.value[ instanceIndex ].xyz;
		nodeUniform11.value[ ( nodeConst4 + 1u ) ].x = nodeVar10[ 0 ];
		nodeUniform11.value[ ( nodeConst4 + 1u ) ].y = nodeVar10[ 1 ];
		nodeUniform11.value[ ( nodeConst4 + 1u ) ].z = nodeVar10[ 2 ];
		nodeUniform11.value[ ( nodeConst4 + 1u ) ].y = ( nodeUniform11.value[ ( nodeConst4 + 1u ) ].y + ( - object.nodeUniform12 ) );
		nodeVar11 = nodeVar4;
		nodeUniform11.value[ ( nodeConst4 + 2u ) ].x = nodeVar11[ 0 ];
		nodeUniform11.value[ ( nodeConst4 + 2u ) ].y = nodeVar11[ 1 ];
		nodeUniform11.value[ ( nodeConst4 + 2u ) ].z = nodeVar11[ 2 ];
		nodeUniform11.value[ ( nodeConst4 + 2u ) ].y = ( nodeUniform11.value[ ( nodeConst4 + 2u ) ].y + ( - object.nodeUniform12 ) );
		nodeVar12 = nodeVar4;
		nodeUniform11.value[ ( nodeConst4 + 3u ) ].x = nodeVar12[ 0 ];
		nodeUniform11.value[ ( nodeConst4 + 3u ) ].y = nodeVar12[ 1 ];
		nodeUniform11.value[ ( nodeConst4 + 3u ) ].z = nodeVar12[ 2 ];
		nodeUniform11.value[ ( nodeConst4 + 3u ) ].y = ( nodeUniform11.value[ ( nodeConst4 + 3u ) ].y + object.nodeUniform12 );
		let nodeConst5 = ( nodeConst4 + 4u );
		nodeVar13 = nodeUniform0.value[ instanceIndex ].xyz;
		nodeUniform11.value[ nodeConst5 ].x = nodeVar13[ 0 ];
		nodeUniform11.value[ nodeConst5 ].y = nodeVar13[ 1 ];
		nodeUniform11.value[ nodeConst5 ].z = nodeVar13[ 2 ];
		nodeUniform11.value[ nodeConst5 ].y = ( nodeUniform11.value[ nodeConst5 ].y + object.nodeUniform12 );
		nodeVar14 = nodeUniform0.value[ instanceIndex ].xyz;
		nodeUniform11.value[ ( nodeConst5 + 1u ) ].x = nodeVar14[ 0 ];
		nodeUniform11.value[ ( nodeConst5 + 1u ) ].y = nodeVar14[ 1 ];
		nodeUniform11.value[ ( nodeConst5 + 1u ) ].z = nodeVar14[ 2 ];
		nodeUniform11.value[ ( nodeConst5 + 1u ) ].y = ( nodeUniform11.value[ ( nodeConst5 + 1u ) ].y + ( - object.nodeUniform12 ) );
		nodeVar15 = nodeVar7;
		nodeUniform11.value[ ( nodeConst5 + 2u ) ].x = nodeVar15[ 0 ];
		nodeUniform11.value[ ( nodeConst5 + 2u ) ].y = nodeVar15[ 1 ];
		nodeUniform11.value[ ( nodeConst5 + 2u ) ].z = nodeVar15[ 2 ];
		nodeUniform11.value[ ( nodeConst5 + 2u ) ].y = ( nodeUniform11.value[ ( nodeConst5 + 2u ) ].y + ( - object.nodeUniform12 ) );
		nodeVar16 = nodeVar7;
		nodeUniform11.value[ ( nodeConst5 + 3u ) ].x = nodeVar16[ 0 ];
		nodeUniform11.value[ ( nodeConst5 + 3u ) ].y = nodeVar16[ 1 ];
		nodeUniform11.value[ ( nodeConst5 + 3u ) ].z = nodeVar16[ 2 ];
		nodeUniform11.value[ ( nodeConst5 + 3u ) ].y = ( nodeUniform11.value[ ( nodeConst5 + 3u ) ].y + object.nodeUniform12 );
		var nodeVar17 : vec3<f32> = fn34( vec3<f32>( 0.0, 0.0, 1.0 ), ( object.nodeUniform13 + ( fn87( vec3<f32>( ( f32( instanceIndex ) * 0.1 ) ), 2, 2.0, 0.5 ) * object.nodeUniform14 ) ) );

		for ( var i : i32 = 0; i < 4; i ++ ) {

			nodeVar18 = nodeVar17;
			nodeUniform15.value[ ( i32( nodeConst4 ) + i ) ].x = nodeVar18[ 0 ];
			nodeUniform15.value[ ( i32( nodeConst4 ) + i ) ].y = nodeVar18[ 1 ];
			nodeUniform15.value[ ( i32( nodeConst4 ) + i ) ].z = nodeVar18[ 2 ];
			nodeUniform15.value[ ( i32( nodeConst4 ) + i ) ].w = pow( max( 0.0, min( nodeVar5, nodeUniform0.value[ instanceIndex ].w ) ), 0.8 );
			nodeVar19 = nodeVar17;
			nodeUniform15.value[ ( i32( nodeConst5 ) + i ) ].x = nodeVar19[ 0 ];
			nodeUniform15.value[ ( i32( nodeConst5 ) + i ) ].y = nodeVar19[ 1 ];
			nodeUniform15.value[ ( i32( nodeConst5 ) + i ) ].z = nodeVar19[ 2 ];
			nodeUniform15.value[ ( i32( nodeConst5 ) + i ) ].w = pow( max( 0.0, min( nodeVar8, nodeUniform0.value[ instanceIndex ].w ) ), 0.8 );

		}

		

	}


	

}
