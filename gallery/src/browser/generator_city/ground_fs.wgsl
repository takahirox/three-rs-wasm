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
@binding( 1 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform8 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform16_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform16 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform24_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform24 : texture_3d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform26_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform26 : texture_cube<f32>;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraPosition : vec3<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform17 : f32,
	nodeUniform18 : vec2<f32>,
	nodeUniform19 : f32,
	nodeUniform25 : f32,
	nodeUniform23 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform20 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform27 : mat4x4<f32>,
	nodeUniform29 : f32,
	nodeUniform30 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar4 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> lightProbeIrradiance : vec3<f32>;
var<private> lightProbePriority : f32;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : vec4<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar20 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar21 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
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


fn tsl_mod_float( x : f32, y : f32 ) -> f32 { return x - y * floor( x / y ); }
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
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = 0.0;
	nodeVar1 = 0.0;
	nodeVar2 = 1.0;
	nodeVar3 = 0.0;
	let nodeConst0 = distance( v_positionWorld, render.cameraPosition );
	let nodeConst1 = smoothstep( 240.0, 25.0, nodeConst0 );

	if ( ( nodeConst1 > 0.0 ) ) {

		nodeVar0 = ( ( ( ( fn74( ( v_positionWorld * vec3<f32>( 7.0 ) ) ) * 1.0 ) + 0.0 ) + ( ( fn74( ( v_positionWorld * vec3<f32>( 23.0 ) ) ) * 1.0 ) + 0.0 ) ) * 0.5 );
		nodeVar1 = smoothstep( 0.5, 0.85, ( ( ( fn87( ( v_positionWorld * vec3<f32>( 0.45 ) ), 3, 2.0, 0.5 ) * 1.0 ) * 0.5 ) + 0.5 ) );
		nodeVar2 = ( ( smoothstep( 0.25, 0.7, ( ( ( fn87( ( v_positionWorld * vec3<f32>( 0.7 ) ), 3, 2.0, 0.5 ) * 1.0 ) * 0.5 ) + 0.5 ) ) * 0.55 ) + 0.35 );
		

	}

	let nodeConst2 = smoothstep( 22.0, 4.0, nodeConst0 );

	if ( ( nodeConst2 > 0.0 ) ) {

		nodeVar3 = ( ( ( ( fn74( ( v_positionWorld * vec3<f32>( 45.0 ) ) ) * 1.0 ) + 0.0 ) * 0.6 ) + ( ( ( fn74( ( v_positionWorld * vec3<f32>( 80.0 ) ) ) * 1.0 ) + 0.0 ) * 0.4 ) );
		

	}

	let nodeConst3 = vec4<f32>( nodeVar0, nodeVar1, nodeVar2, nodeVar3 );
	let nodeConst4 = ( mix( vec3<f32>( 0.01764195448412081, 0.019382360952473074, 0.024157632443547246 ), vec3<f32>( 0.04373502925049377, 0.04970656597728775, 0.061246054224174035 ), ( ( ( fn87( ( v_positionWorld * vec3<f32>( 0.2 ) ), 3, 2.0, 0.5 ) * 1.0 ) * 0.5 ) + 0.5 ) ) * vec3<f32>( ( ( ( nodeConst3.x * 0.22 ) * nodeConst1 ) + 1.0 ) ) );
	let nodeConst5 = mix( nodeConst4, ( nodeConst4 * vec3<f32>( 0.5 ) ), ( ( nodeConst3.y * 0.5 ) * nodeConst1 ) );
	let nodeConst6 = smoothstep( 0.6, 0.85, ( ( ( fn87( ( v_positionWorld * vec3<f32>( 0.14 ) ), 2, 2.0, 0.5 ) * 1.0 ) * 0.5 ) + 0.5 ) );
	let nodeConst7 = tsl_mod_float( ( v_positionWorld.x + 101.0 ), 112.0 );
	let nodeConst8 = ( nodeConst7 - 90.0 );
	let nodeConst9 = ( nodeConst8 - 11.0 );
	let nodeConst10 = max( fwidth( nodeConst9 ), 0.0001 );
	let nodeConst11 = ( nodeConst8 - 5.5 );
	let nodeConst12 = max( fwidth( nodeConst11 ), 0.0001 );
	let nodeConst13 = ( nodeConst8 - 16.5 );
	let nodeConst14 = max( fwidth( nodeConst13 ), 0.0001 );
	let nodeConst15 = step( 90.0, nodeConst7 );
	let nodeConst16 = tsl_mod_float( ( v_positionWorld.z + 71.0 ), 82.0 );
	let nodeConst17 = step( 60.0, nodeConst16 );
	let nodeConst18 = ( nodeConst16 - 60.0 );
	let nodeConst19 = ( nodeConst18 - 11.0 );
	let nodeConst20 = max( fwidth( nodeConst19 ), 0.0001 );
	let nodeConst21 = ( nodeConst18 - 5.5 );
	let nodeConst22 = max( fwidth( nodeConst21 ), 0.0001 );
	let nodeConst23 = ( nodeConst18 - 16.5 );
	let nodeConst24 = max( fwidth( nodeConst23 ), 0.0001 );
	let nodeConst25 = ( nodeConst8 / 1.2 );
	let nodeConst26 = max( fwidth( nodeConst25 ), 0.0001 );
	let nodeConst27 = ( nodeConst18 / 1.2 );
	let nodeConst28 = max( fwidth( nodeConst27 ), 0.0001 );
	let nodeConst29 = ( ( max( max( max( ( ( max( smoothstep( ( 0.12 + nodeConst10 ), ( 0.12 - nodeConst10 ), abs( nodeConst9 ) ), ( max( smoothstep( ( 0.1 + nodeConst12 ), ( 0.1 - nodeConst12 ), abs( nodeConst11 ) ), smoothstep( ( 0.1 + nodeConst14 ), ( 0.1 - nodeConst14 ), abs( nodeConst13 ) ) ) * step( fract( ( v_positionWorld.z / 7.0 ) ), 0.5 ) ) ) * nodeConst15 ) * ( 1.0 - nodeConst17 ) ), ( ( max( smoothstep( ( 0.12 + nodeConst20 ), ( 0.12 - nodeConst20 ), abs( nodeConst19 ) ), ( max( smoothstep( ( 0.1 + nodeConst22 ), ( 0.1 - nodeConst22 ), abs( nodeConst21 ) ), smoothstep( ( 0.1 + nodeConst24 ), ( 0.1 - nodeConst24 ), abs( nodeConst23 ) ) ) * step( fract( ( v_positionWorld.x / 7.0 ) ), 0.5 ) ) ) * nodeConst17 ) * ( 1.0 - nodeConst15 ) ) ), ( ( ( smoothstep( ( 0.3166666666666667 + nodeConst26 ), ( 0.3166666666666667 - nodeConst26 ), ( 0.5 - abs( ( fract( nodeConst25 ) - 0.5 ) ) ) ) * nodeConst15 ) * ( 1.0 - nodeConst17 ) ) * max( step( nodeConst16, 5.0 ), step( 55.0, nodeConst16 ) ) ) ), ( ( ( smoothstep( ( 0.3166666666666667 + nodeConst28 ), ( 0.3166666666666667 - nodeConst28 ), ( 0.5 - abs( ( fract( nodeConst27 ) - 0.5 ) ) ) ) * nodeConst17 ) * ( 1.0 - nodeConst15 ) ) * max( step( nodeConst7, 5.0 ), step( 85.0, nodeConst7 ) ) ) ) * nodeConst1 ) * nodeConst3.z );
	DiffuseColor = vec4<f32>( mix( mix( nodeConst5, ( nodeConst5 * vec3<f32>( 0.6 ) ), nodeConst6 ), vec3<f32>( 0.6307571363387763, 0.6038273388475408, 0.5271151256969157 ), nodeConst29 ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform2 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform3;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst30 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( mix( ( 0.95 - ( nodeConst29 * 0.2 ) ), 0.32, nodeConst6 ), 0.045 ) + max( max( nodeConst30.x, nodeConst30.y ), nodeConst30.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform3 ) ) );
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	let nodeConst31 = dpdx( v_positionView );
	let nodeConst32 = - dpdy( v_positionView );
	NORMAL_normalView = normalViewGeometry;
	let NORMAL_nodeConst33 = cross( nodeConst32, NORMAL_normalView );
	let NORMAL_nodeConst34 = dot( nodeConst31, NORMAL_nodeConst33 );
	let nodeConst35 = ( ( ( nodeConst3.x * 0.003 ) * nodeConst1 ) + ( ( nodeConst3.w * 0.0016 ) * nodeConst2 ) );
	let nodeConst36 = ( vec3<f32>( sign( NORMAL_nodeConst34 ) ) * ( ( vec3<f32>( dpdx( nodeConst35 ) ) * NORMAL_nodeConst33 ) + ( vec3<f32>( - dpdy( nodeConst35 ) ) * cross( NORMAL_normalView, nodeConst31 ) ) ) );
	normalView = normalize( ( ( vec3<f32>( abs( NORMAL_nodeConst34 ) ) * NORMAL_normalView ) - nodeConst36 ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar4 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar4;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst37 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst37 );
	let nodeConst38 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst39 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst37 * nodeConst38 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst39 ) * nodeConst38 ) ) ) * vec3<f32>( nodeConst39 ) ) );
	let nodeConst40 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform10 - render.nodeUniform11 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst41 = ( render.nodeUniform13 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform14 ) ) ), 1.0 ) );
	let nodeConst42 = ( nodeConst41.xyz / vec3<f32>( nodeConst41.w ) );
	let nodeConst43 = vec3<f32>( nodeConst42.x, ( 1.0 - nodeConst42.y ), ( nodeConst42.z + render.nodeUniform15 ) );

	if ( ( ( ( ( ( nodeConst43.x >= 0.0 ) && ( nodeConst43.x <= 1.0 ) ) && ( nodeConst43.y >= 0.0 ) ) && ( nodeConst43.y <= 1.0 ) ) && ( nodeConst43.z <= 1.0 ) ) ) {

		let nodeConst44 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst45 = ( vec2<f32>( render.nodeUniform17 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform18 ) );
		let nodeConst46 = ( nodeConst43.xy + ( fn25( 0, 5, nodeConst44 ) * nodeConst45 ) );
		nodeVar6 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeConst46, nodeConst43.z );
		let nodeConst47 = ( nodeConst43.xy + ( fn25( 1, 5, nodeConst44 ) * nodeConst45 ) );
		nodeVar7 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeConst47, nodeConst43.z );
		let nodeConst48 = ( nodeConst43.xy + ( fn25( 2, 5, nodeConst44 ) * nodeConst45 ) );
		nodeVar8 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeConst48, nodeConst43.z );
		let nodeConst49 = ( nodeConst43.xy + ( fn25( 3, 5, nodeConst44 ) * nodeConst45 ) );
		nodeVar9 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeConst49, nodeConst43.z );
		let nodeConst50 = ( nodeConst43.xy + ( fn25( 4, 5, nodeConst44 ) * nodeConst45 ) );
		nodeVar10 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeConst50, nodeConst43.z );
		nodeVar5 = ( ( ( ( ( nodeVar6 + nodeVar7 ) + nodeVar8 ) + nodeVar9 ) + nodeVar10 ) * 0.2 );

	} else {

		nodeVar5 = 1.0;

	}

	nodeVar11 = mix( 1.0, nodeVar5, render.nodeUniform19 );
	nodeVar12 = ( vec3<f32>( clamp( dot( normalView, nodeConst40 ), 0.0, 1.0 ) ) * ( render.nodeUniform12 * vec3<f32>( nodeVar11 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst51 = clamp( dot( positionViewDirection, normalize( ( nodeConst40 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst52 = exp2( ( ( ( nodeConst51 * -5.55473 ) - 6.98316 ) * nodeConst51 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar12 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst52 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst52 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst53 = normalize( ( nodeConst40 + positionViewDirection ) );
	let nodeConst54 = clamp( dot( positionViewDirection, nodeConst53 ), 0.0, 1.0 );
	let nodeConst55 = exp2( ( ( ( nodeConst54 * -5.55473 ) - 6.98316 ) * nodeConst54 ) );
	let nodeConst56 = max( Roughness, 0.045 );
	let nodeConst57 = ( nodeConst56 * nodeConst56 );
	directSpecular = ( directSpecular + ( ( nodeVar12 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst55 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst55 ) ) ) * vec3<f32>( fn110( nodeConst57, clamp( dot( normalView, nodeConst40 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst57, clamp( dot( normalView, nodeConst53 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	lightProbeIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	lightProbePriority = ( - 1000000.0 );
	let nodeConst58 = ( ( object.nodeUniform20 - object.nodeUniform21 ) / object.nodeUniform22 );
	let nodeConst59 = ( v_positionWorld + ( ( normalWorld * nodeConst58 ) * vec3<f32>( 0.5 ) ) );
	let nodeConst60 = ( nodeConst58 * vec3<f32>( 0.01 ) );

	if ( ( ( all( ( nodeConst59 >= ( object.nodeUniform21 - nodeConst60 ) ) ) && all( ( nodeConst59 <= ( object.nodeUniform20 + nodeConst60 ) ) ) ) && ( render.nodeUniform23 >= lightProbePriority ) ) ) {

		let nodeConst61 = ( object.nodeUniform20 - object.nodeUniform21 );
		let nodeConst62 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform22 );
		let nodeConst63 = clamp( ( ( ( v_positionWorld + ( ( normalWorld * ( nodeConst61 / object.nodeUniform22 ) ) * vec3<f32>( 0.5 ) ) ) - object.nodeUniform21 ) / nodeConst61 ), nodeConst62, ( vec3<f32>( 1.0 ) - nodeConst62 ) );
		let nodeConst64 = ( ( nodeConst63.z * object.nodeUniform22.z ) + 1.0 );
		let nodeConst65 = ( object.nodeUniform22.z + 2.0 );
		let nodeConst66 = ( nodeConst65 * 7.0 );
		nodeVar13 = textureSample( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeConst63.xy, ( ( nodeConst64 + ( nodeConst65 * 0.0 ) ) / nodeConst66 ) ) );
		nodeVar14 = textureSample( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeConst63.xy, ( ( nodeConst64 + ( nodeConst65 * 1.0 ) ) / nodeConst66 ) ) );
		nodeVar15 = textureSample( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeConst63.xy, ( ( nodeConst64 + ( nodeConst65 * 2.0 ) ) / nodeConst66 ) ) );
		nodeVar16 = textureSample( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeConst63.xy, ( ( nodeConst64 + ( nodeConst65 * 3.0 ) ) / nodeConst66 ) ) );
		nodeVar17 = textureSample( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeConst63.xy, ( ( nodeConst64 + ( nodeConst65 * 4.0 ) ) / nodeConst66 ) ) );
		nodeVar18 = textureSample( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeConst63.xy, ( ( nodeConst64 + ( nodeConst65 * 5.0 ) ) / nodeConst66 ) ) );
		nodeVar19 = textureSample( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeConst63.xy, ( ( nodeConst64 + ( nodeConst65 * 6.0 ) ) / nodeConst66 ) ) );
		let nodeConst67 = array< vec3<f32>, 9 >( nodeVar13.xyz, vec3<f32>( nodeVar13.w, nodeVar14.xy ), vec3<f32>( nodeVar14.zw, nodeVar15.x ), nodeVar15.yzw, nodeVar16.xyz, vec3<f32>( nodeVar16.w, nodeVar17.xy ), vec3<f32>( nodeVar17.zw, nodeVar18.x ), nodeVar18.yzw, nodeVar19.xyz );
		let nodeConst68 = ( max( ( ( ( ( ( ( ( ( ( nodeConst67[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeConst67[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeConst67[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst67[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeConst67[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeConst67[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeConst67[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeConst67[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst67[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ) * vec3<f32>( render.nodeUniform25 ) );
		lightProbeIrradiance = nodeConst68;
		lightProbePriority = render.nodeUniform23;
		

	}

	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst69 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst70 = ( object.nodeUniform27 * vec4<f32>( nodeConst69, 1.0 ) );
	let nodeConst71 = clamp( Roughness, 0.0, 1.0 );
	nodeVar20 = textureSampleLevel( nodeUniform26, nodeUniform26_sampler, vec3<f32>( ( - nodeConst70.x ), nodeConst70.yz ), ( ( object.nodeUniform29 * nodeConst71 ) * ( 2.0 - nodeConst71 ) ) );
	radiance = ( radiance + ( nodeVar20.xyz * vec3<f32>( object.nodeUniform30 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst72 = ( object.nodeUniform27 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst73 = clamp( 1.0, 0.0, 1.0 );
	nodeVar21 = textureSampleLevel( nodeUniform26, nodeUniform26_sampler, vec3<f32>( ( - nodeConst72.x ), nodeConst72.yz ), ( ( object.nodeUniform29 * nodeConst73 ) * ( 2.0 - nodeConst73 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar21.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform30 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + lightProbeIrradiance );
	nodeVar22 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar22 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst74 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst74 );
	let nodeConst75 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst76 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst74 * nodeConst75 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst76 ) * nodeConst75 ) ) ) * vec3<f32>( nodeConst76 ) ) );
	let nodeConst77 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar23 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst77 ) );
	nodeVar24 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst77 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar23 );
	indirectDiffuse = ( indirectDiffuse + nodeVar24 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst78 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst78;

	// result

	output.color = nodeConst78;

	return output;

}
