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
@binding( 1 ) @group( 1 ) var nodeUniform11_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform11 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform19_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform19 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform27_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform27 : texture_3d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform29_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform29 : texture_cube<f32>;

struct objectStruct {
	nodeUniform6 : mat3x3<f32>,
	nodeUniform7 : mat4x4<f32>,
	nodeUniform8 : f32,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : f32,
	nodeUniform23 : vec3<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform30 : mat4x4<f32>,
	nodeUniform32 : f32,
	nodeUniform33 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform4 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform16 : mat4x4<f32>,
	nodeUniform17 : f32,
	nodeUniform18 : f32,
	nodeUniform20 : f32,
	nodeUniform21 : vec2<f32>,
	nodeUniform22 : f32,
	nodeUniform28 : f32,
	nodeUniform26 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> normalWorldGeometry : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> Metalness : f32;
var<private> nodeVar3 : f32;
var<private> Roughness : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> positionViewDirection : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> nodeVar8 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar9 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> lightProbeIrradiance : vec3<f32>;
var<private> lightProbePriority : f32;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : vec4<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : vec4<f32>;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : vec4<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar24 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar25 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
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
	@location( 1 ) @interpolate( flat, either ) nodeVarying3 : f32,
	@location( 2 ) v_normalWorldGeometry : vec3<f32>,
	@location( 3 ) v_normalViewGeometry : vec3<f32>,
	@location( 4 ) v_positionWorld : vec3<f32>,
	@location( 5 ) v_positionViewDirection : vec3<f32>,
	@location( 6 ) @interpolate(flat, either) nodeVarying9 : u32,
	@location( 7 ) nodeVarying10 : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	let nodeConst3 = ( nodeVarying3 == 1.0 );

	if ( nodeConst3 ) {

		normalWorldGeometry = normalize( v_normalWorldGeometry );
		let nodeConst5 = ( ( ( fn87( ( v_positionWorld * vec3<f32>( 2.2 ) ), 2, 2.0, 0.5 ) * 1.0 ) * 0.5 ) + 0.5 );
		let nodeConst6 = ( ( ( ( fn74( ( v_positionWorld * vec3<f32>( 9.0 ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 );
		nodeVar1 = ( ( ( mix( mix( mix( vec3<f32>( 0.033104766565152086, 0.08228270712149792, 0.013702083043526807 ), vec3<f32>( 0.10946171076915331, 0.20155625378383743, 0.033104766565152086 ), ( ( normalWorldGeometry.y * 0.5 ) + 0.5 ) ), vec3<f32>( 0.1844749944900301, 0.22322795730611386, 0.028426039499072558 ), ( ( ( ( fn87( ( v_positionWorld * vec3<f32>( 0.55 ) ), 2, 2.0, 0.5 ) * 1.0 ) * 0.5 ) + 0.5 ) * 0.55 ) ), vec3<f32>( 0.20507873637973145, 0.20155625378383743, 0.025186859622305935 ), ( fract( ( sin( ( f32( nodeVarying9 ) * 12.9898 ) ) * 43758.5453 ) ) * 0.3 ) ) * vec3<f32>( ( ( nodeConst5 * 0.35 ) + 0.75 ) ) ) * vec3<f32>( ( ( nodeConst6 * 0.24 ) + 0.88 ) ) ) * vec3<f32>( ( ( smoothstep( 0.1, 1.05, ( length( ( nodeVarying10 - vec3<f32>( 0.0, 3.9, 0.1 ) ) ) / 3.1 ) ) * 0.5 ) + 0.5 ) ) );

	} else {

		let nodeConst7 = ( nodeVarying3 == 2.0 );

		if ( nodeConst7 ) {

			nodeVar2 = mix( vec3<f32>( 0.006048833020386069, 0.005605391621829108, 0.005181516700061659 ), vec3<f32>( 0.04231141061442144, 0.03954623527052923, 0.035601314869097636 ), smoothstep( 0.35, 0.5, fract( ( length( nodeVarying10.xz ) * 11.0 ) ) ) );

		} else {

			let nodeConst8 = ( 1.0 - abs( ( fn87( vec3<f32>( ( nodeVarying10.x * 9.0 ), ( nodeVarying10.y * 1.4 ), ( nodeVarying10.z * 9.0 ) ), 2, 2.0, 0.5 ) * 1.0 ) ) );
			nodeVar2 = mix( vec3<f32>( 0.06847816983662762, 0.04231141061442144, 0.023153366173251363 ), vec3<f32>( 0.158960835050774, 0.11193242782769693, 0.061246054224174035 ), ( smoothstep( 0.55, 1.0, nodeConst8 ) * 0.7 ) );

		}

		nodeVar1 = nodeVar2;

	}

	DiffuseColor = vec4<f32>( nodeVar1, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform8 );
	DiffuseColor.w = 1.0;
	let nodeConst7 = ( nodeVarying3 == 2.0 );

	if ( nodeConst7 ) {

		nodeVar3 = 0.4;

	} else {

		nodeVar3 = 0.0;

	}

	let nodeConst9 = nodeVar3;
	Metalness = nodeConst9;

	if ( nodeConst7 ) {

		nodeVar4 = 0.7;

	} else {


		if ( nodeConst3 ) {

			let nodeConst6 = ( ( ( ( fn74( ( v_positionWorld * vec3<f32>( 9.0 ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 );
			nodeVar5 = ( ( nodeConst6 * 0.15 ) + 0.8 );

		} else {

			let nodeConst8 = ( 1.0 - abs( ( fn87( vec3<f32>( ( nodeVarying10.x * 9.0 ), ( nodeVarying10.y * 1.4 ), ( nodeVarying10.z * 9.0 ) ), 2, 2.0, 0.5 ) * 1.0 ) ) );
			nodeVar5 = ( ( nodeConst8 * 0.12 ) + 0.82 );

		}

		nodeVar4 = nodeVar5;

	}

	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst10 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( nodeVar4, 0.045 ) + max( max( nodeConst10.x, nodeConst10.y ), nodeConst10.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - nodeConst9 ) ) );
	EmissiveColor = ( object.nodeUniform9 * vec3<f32>( object.nodeUniform10 ) );
	let nodeConst11 = dpdx( v_positionView );
	let nodeConst12 = - dpdy( v_positionView );
	NORMAL_normalView = normalViewGeometry;
	let NORMAL_nodeConst13 = cross( nodeConst12, NORMAL_normalView );
	let NORMAL_nodeConst14 = dot( nodeConst11, NORMAL_nodeConst13 );

	if ( nodeConst3 ) {

		let nodeConst5 = ( ( ( fn87( ( v_positionWorld * vec3<f32>( 2.2 ) ), 2, 2.0, 0.5 ) * 1.0 ) * 0.5 ) + 0.5 );
		nodeVar6 = ( nodeConst5 * 0.07 );

	} else {


		if ( nodeConst7 ) {

			nodeVar7 = 0.0;

		} else {

			let nodeConst8 = ( 1.0 - abs( ( fn87( vec3<f32>( ( nodeVarying10.x * 9.0 ), ( nodeVarying10.y * 1.4 ), ( nodeVarying10.z * 9.0 ) ), 2, 2.0, 0.5 ) * 1.0 ) ) );
			nodeVar7 = ( nodeConst8 * 0.02 );

		}

		nodeVar6 = nodeVar7;

	}

	positionViewDirection = normalize( v_positionViewDirection );
	let nodeConst15 = ( nodeVar6 * smoothstep( 0.08, 0.35, abs( dot( NORMAL_normalView, positionViewDirection ) ) ) );
	let nodeConst16 = ( vec3<f32>( sign( NORMAL_nodeConst14 ) ) * ( ( vec3<f32>( dpdx( nodeConst15 ) ) * NORMAL_nodeConst13 ) + ( vec3<f32>( - dpdy( nodeConst15 ) ) * cross( NORMAL_normalView, nodeConst11 ) ) ) );
	normalView = normalize( ( ( vec3<f32>( abs( NORMAL_nodeConst14 ) ) * NORMAL_normalView ) - nodeConst16 ) );
	nodeVar8 = textureSample( nodeUniform11, nodeUniform11_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar8;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst17 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst17 );
	let nodeConst18 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst19 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst17 * nodeConst18 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst19 ) * nodeConst18 ) ) ) * vec3<f32>( nodeConst19 ) ) );
	let nodeConst20 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform13 - render.nodeUniform14 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst21 = ( render.nodeUniform16 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform17 ) ) ), 1.0 ) );
	let nodeConst22 = ( nodeConst21.xyz / vec3<f32>( nodeConst21.w ) );
	let nodeConst23 = vec3<f32>( nodeConst22.x, ( 1.0 - nodeConst22.y ), ( nodeConst22.z + render.nodeUniform18 ) );

	if ( ( ( ( ( ( nodeConst23.x >= 0.0 ) && ( nodeConst23.x <= 1.0 ) ) && ( nodeConst23.y >= 0.0 ) ) && ( nodeConst23.y <= 1.0 ) ) && ( nodeConst23.z <= 1.0 ) ) ) {

		let nodeConst24 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst25 = ( vec2<f32>( render.nodeUniform20 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform21 ) );
		let nodeConst26 = ( nodeConst23.xy + ( fn25( 0, 5, nodeConst24 ) * nodeConst25 ) );
		nodeVar10 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst26, nodeConst23.z );
		let nodeConst27 = ( nodeConst23.xy + ( fn25( 1, 5, nodeConst24 ) * nodeConst25 ) );
		nodeVar11 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst27, nodeConst23.z );
		let nodeConst28 = ( nodeConst23.xy + ( fn25( 2, 5, nodeConst24 ) * nodeConst25 ) );
		nodeVar12 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst28, nodeConst23.z );
		let nodeConst29 = ( nodeConst23.xy + ( fn25( 3, 5, nodeConst24 ) * nodeConst25 ) );
		nodeVar13 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst29, nodeConst23.z );
		let nodeConst30 = ( nodeConst23.xy + ( fn25( 4, 5, nodeConst24 ) * nodeConst25 ) );
		nodeVar14 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst30, nodeConst23.z );
		nodeVar9 = ( ( ( ( ( nodeVar10 + nodeVar11 ) + nodeVar12 ) + nodeVar13 ) + nodeVar14 ) * 0.2 );

	} else {

		nodeVar9 = 1.0;

	}

	nodeVar15 = mix( 1.0, nodeVar9, render.nodeUniform22 );
	nodeVar16 = ( vec3<f32>( clamp( dot( normalView, nodeConst20 ), 0.0, 1.0 ) ) * ( render.nodeUniform15 * vec3<f32>( nodeVar15 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst31 = clamp( dot( positionViewDirection, normalize( ( nodeConst20 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst32 = exp2( ( ( ( nodeConst31 * -5.55473 ) - 6.98316 ) * nodeConst31 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar16 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst32 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst32 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst33 = normalize( ( nodeConst20 + positionViewDirection ) );
	let nodeConst34 = clamp( dot( positionViewDirection, nodeConst33 ), 0.0, 1.0 );
	let nodeConst35 = exp2( ( ( ( nodeConst34 * -5.55473 ) - 6.98316 ) * nodeConst34 ) );
	let nodeConst36 = max( Roughness, 0.045 );
	let nodeConst37 = ( nodeConst36 * nodeConst36 );
	directSpecular = ( directSpecular + ( ( nodeVar16 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst35 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst35 ) ) ) * vec3<f32>( fn110( nodeConst37, clamp( dot( normalView, nodeConst20 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst37, clamp( dot( normalView, nodeConst33 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	lightProbeIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	lightProbePriority = ( - 1000000.0 );
	let nodeConst38 = ( ( object.nodeUniform23 - object.nodeUniform24 ) / object.nodeUniform25 );
	let nodeConst39 = ( v_positionWorld + ( ( normalWorld * nodeConst38 ) * vec3<f32>( 0.5 ) ) );
	let nodeConst40 = ( nodeConst38 * vec3<f32>( 0.01 ) );

	if ( ( ( all( ( nodeConst39 >= ( object.nodeUniform24 - nodeConst40 ) ) ) && all( ( nodeConst39 <= ( object.nodeUniform23 + nodeConst40 ) ) ) ) && ( render.nodeUniform26 >= lightProbePriority ) ) ) {

		let nodeConst41 = ( object.nodeUniform23 - object.nodeUniform24 );
		let nodeConst42 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform25 );
		let nodeConst43 = clamp( ( ( ( v_positionWorld + ( ( normalWorld * ( nodeConst41 / object.nodeUniform25 ) ) * vec3<f32>( 0.5 ) ) ) - object.nodeUniform24 ) / nodeConst41 ), nodeConst42, ( vec3<f32>( 1.0 ) - nodeConst42 ) );
		let nodeConst44 = ( ( nodeConst43.z * object.nodeUniform25.z ) + 1.0 );
		let nodeConst45 = ( object.nodeUniform25.z + 2.0 );
		let nodeConst46 = ( nodeConst45 * 7.0 );
		nodeVar17 = textureSample( nodeUniform27, nodeUniform27_sampler, vec3<f32>( nodeConst43.xy, ( ( nodeConst44 + ( nodeConst45 * 0.0 ) ) / nodeConst46 ) ) );
		nodeVar18 = textureSample( nodeUniform27, nodeUniform27_sampler, vec3<f32>( nodeConst43.xy, ( ( nodeConst44 + ( nodeConst45 * 1.0 ) ) / nodeConst46 ) ) );
		nodeVar19 = textureSample( nodeUniform27, nodeUniform27_sampler, vec3<f32>( nodeConst43.xy, ( ( nodeConst44 + ( nodeConst45 * 2.0 ) ) / nodeConst46 ) ) );
		nodeVar20 = textureSample( nodeUniform27, nodeUniform27_sampler, vec3<f32>( nodeConst43.xy, ( ( nodeConst44 + ( nodeConst45 * 3.0 ) ) / nodeConst46 ) ) );
		nodeVar21 = textureSample( nodeUniform27, nodeUniform27_sampler, vec3<f32>( nodeConst43.xy, ( ( nodeConst44 + ( nodeConst45 * 4.0 ) ) / nodeConst46 ) ) );
		nodeVar22 = textureSample( nodeUniform27, nodeUniform27_sampler, vec3<f32>( nodeConst43.xy, ( ( nodeConst44 + ( nodeConst45 * 5.0 ) ) / nodeConst46 ) ) );
		nodeVar23 = textureSample( nodeUniform27, nodeUniform27_sampler, vec3<f32>( nodeConst43.xy, ( ( nodeConst44 + ( nodeConst45 * 6.0 ) ) / nodeConst46 ) ) );
		let nodeConst47 = array< vec3<f32>, 9 >( nodeVar17.xyz, vec3<f32>( nodeVar17.w, nodeVar18.xy ), vec3<f32>( nodeVar18.zw, nodeVar19.x ), nodeVar19.yzw, nodeVar20.xyz, vec3<f32>( nodeVar20.w, nodeVar21.xy ), vec3<f32>( nodeVar21.zw, nodeVar22.x ), nodeVar22.yzw, nodeVar23.xyz );
		let nodeConst48 = ( max( ( ( ( ( ( ( ( ( ( nodeConst47[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeConst47[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeConst47[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst47[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeConst47[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeConst47[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeConst47[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeConst47[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst47[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ) * vec3<f32>( render.nodeUniform28 ) );
		lightProbeIrradiance = nodeConst48;
		lightProbePriority = render.nodeUniform26;
		

	}

	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst49 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst50 = ( object.nodeUniform30 * vec4<f32>( nodeConst49, 1.0 ) );
	let nodeConst51 = clamp( Roughness, 0.0, 1.0 );
	nodeVar24 = textureSampleLevel( nodeUniform29, nodeUniform29_sampler, vec3<f32>( ( - nodeConst50.x ), nodeConst50.yz ), ( ( object.nodeUniform32 * nodeConst51 ) * ( 2.0 - nodeConst51 ) ) );
	radiance = ( radiance + ( nodeVar24.xyz * vec3<f32>( object.nodeUniform33 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst52 = ( object.nodeUniform30 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst53 = clamp( 1.0, 0.0, 1.0 );
	nodeVar25 = textureSampleLevel( nodeUniform29, nodeUniform29_sampler, vec3<f32>( ( - nodeConst52.x ), nodeConst52.yz ), ( ( object.nodeUniform32 * nodeConst53 ) * ( 2.0 - nodeConst53 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar25.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform33 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + lightProbeIrradiance );
	nodeVar26 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar26 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst54 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst54 );
	let nodeConst55 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst56 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst54 * nodeConst55 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst56 ) * nodeConst55 ) ) ) * vec3<f32>( nodeConst56 ) ) );
	let nodeConst57 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar27 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst57 ) );
	nodeVar28 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst57 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar27 );
	indirectDiffuse = ( indirectDiffuse + nodeVar28 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst58 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst58;

	// result

	output.color = nodeConst58;

	return output;

}
