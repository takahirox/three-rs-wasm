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
@binding( 1 ) @group( 1 ) var nodeUniform41_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform41 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform47_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform47 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : i32,
	nodeUniform44 : mat3x3<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform5 : u32,
	nodeUniform18 : f32,
	nodeUniform21 : f32,
	nodeUniform7 : f32,
	nodeUniform13 : vec3<f32>,
	nodeUniform22 : f32,
	nodeUniform10 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform23 : f32,
	nodeUniform24 : f32,
	nodeUniform26 : f32,
	nodeUniform27 : f32,
	nodeUniform28 : f32,
	nodeUniform16 : f32,
	nodeUniform17 : f32,
	nodeUniform20 : f32,
	nodeUniform19 : f32,
	nodeUniform15 : f32,
	nodeUniform14 : vec3<f32>,
	nodeUniform25 : f32,
	nodeUniform8 : f32,
	nodeUniform11 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform31 : f32,
	nodeUniform32 : f32,
	nodeUniform34 : f32,
	nodeUniform35 : f32,
	nodeUniform39 : f32,
	nodeUniform40 : f32,
	nodeUniform33 : vec3<f32>,
	nodeUniform37 : vec3<f32>,
	nodeUniform38 : vec3<f32>,
	nodeUniform29 : vec3<f32>,
	nodeUniform36 : vec3<f32>,
	nodeUniform42 : mat4x4<f32>,
	nodeUniform45 : f32,
	nodeUniform46 : f32,
	nodeUniform48 : f32,
	nodeUniform49 : vec2<f32>,
	nodeUniform50 : f32,
	cameraPosition : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : bool;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> workingLuminanceCoefficients : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : vec4<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn24 ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn fn32 ( color : vec3<f32>, adjustment : f32, luminanceCoefficients : vec3<f32> ) -> vec3<f32> {

	


	return max( mix( vec3<f32>( dot( color, luminanceCoefficients ) ), color, adjustment ), vec3<f32>( 0.0 ) );

}


fn fn34 ( color : vec3<f32>, adjustment : f32 ) -> vec3<f32> {

	

	let nodeConst0 = cos( adjustment );

	return max( ( ( color * vec3<f32>( nodeConst0 ) ) + ( ( cross( vec3<f32>( 0.57735, 0.57735, 0.57735 ), color ) * vec3<f32>( sin( adjustment ) ) ) + ( vec3<f32>( 0.57735, 0.57735, 0.57735 ) * vec3<f32>( ( dot( vec3<f32>( 0.57735, 0.57735, 0.57735 ), color ) * ( 1.0 - nodeConst0 ) ) ) ) ) ), vec3<f32>( 0.0 ) );

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


fn fn44 ( lightDistance : f32, cutoffDistance : f32, decayExponent : f32 ) -> f32 {

	var nodeVar0 : f32;


	if ( ( cutoffDistance > 0.0 ) ) {

		let nodeConst0 = ( 1.0 / max( pow( lightDistance, decayExponent ), 0.01 ) );
		let nodeConst1 = ( lightDistance / cutoffDistance );
		let nodeConst2 = clamp( ( 1.0 - ( ( ( nodeConst1 * nodeConst1 ) * nodeConst1 ) * nodeConst1 ) ), 0.0, 1.0 );
		nodeVar0 = ( nodeConst0 * ( nodeConst2 * nodeConst2 ) );

	} else {

		let nodeConst0 = ( 1.0 / max( pow( lightDistance, decayExponent ), 0.01 ) );
		nodeVar0 = nodeConst0;

	}


	return nodeVar0;

}


fn fn25 ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	

	let nodeConst0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeConst0 ), sin( nodeConst0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

}




@fragment
fn main( @location( 0 ) v_positionWorld : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );

	if ( ( length( ( render.cameraPosition - v_positionWorld ) ) > ( object.nodeUniform3 * 2.0 ) ) ) {

		nodeVar0 = render.cameraPosition;
		nodeVar1 = v_positionWorld;
		nodeVar2 = true;
		

	} else {

		nodeVar0 = v_positionWorld;
		nodeVar1 = render.cameraPosition;
		nodeVar2 = false;
		

	}

	let nodeConst0 = ( nodeVar1 - nodeVar0 );
	nodeVar3 = ( length( nodeConst0 ) / f32( object.nodeUniform4 ) );
	nodeVar4 = normalize( nodeConst0 );
	nodeVar5 = 0.0;
	nodeVar6 = vec3<f32>( 1.0, 1.0, 1.0 );
	nodeVar5 = ( nodeVar5 + ( fract( ( fn24( fragCoord.xy ) + ( f32( render.nodeUniform5 ) * 0.618033988749895 ) ) ) * nodeVar3 ) );

	for ( var i : i32 = 0; i < object.nodeUniform4; i ++ ) {

		nodeVar7 = vec3<f32>( 0.0 );

		if ( ( 1.0 == 1.0 ) ) {

			nodeVar9 = vec3<f32>( 0.0, 0.0, 0.0 );
			let nodeConst1 = clamp( ( ( ( ( render.nodeUniform7 / 8.34 ) * 0.5 ) + 0.2 ) + render.nodeUniform8 ), 0.0, 1.0 );
			nodeVar9 = mix( vec3<f32>( 0.0, 0.0, 0.0 ), render.nodeUniform6, smoothstep( 0.05, 0.35, nodeConst1 ) );
			nodeVar9 = mix( nodeVar9, render.nodeUniform9, smoothstep( 0.35, 0.65, nodeConst1 ) );
			nodeVar9 = mix( nodeVar9, render.nodeUniform10, smoothstep( 0.65, 1.0, nodeConst1 ) );
			workingLuminanceCoefficients = vec3<f32>( 0.2126, 0.7152, 0.0722 );
			nodeVar8 = fn34( fn32( nodeVar9, render.nodeUniform11, workingLuminanceCoefficients ), render.nodeUniform12 );

		} else {

			nodeVar10 = vec3<f32>( 0.0, 0.0, 0.0 );
			let nodeConst2 = vec3<f32>( 0.0, render.nodeUniform15, 0.0 );
			let nodeConst3 = length( ( v_positionWorld - ( ( render.nodeUniform13 + render.nodeUniform14 ) + ( nodeConst2 * vec3<f32>( clamp( ( dot( ( v_positionWorld - render.nodeUniform13 ), nodeConst2 ) / dot( nodeConst2, nodeConst2 ) ), 0.0, 1.0 ) ) ) ) ) );
			let nodeConst4 = ( v_positionWorld.xz - render.nodeUniform13.xz );
			let nodeConst5 = atan2( nodeConst4.y, nodeConst4.x );
			let nodeConst6 = clamp( ( clamp( ( 1.0 - ( nodeConst3 / render.nodeUniform16 ) ), 0.0, 1.0 ) * mix( ( ( ( ( ( ( ( fn74( vec3<f32>( ( v_positionWorld.x * ( 0.6 * render.nodeUniform17 ) ), ( render.nodeUniform18 * 1.2 ), ( v_positionWorld.z * ( 0.6 * render.nodeUniform17 ) ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) * 0.65 ) + ( ( ( ( ( fn74( vec3<f32>( ( v_positionWorld.x * ( 1.5 * render.nodeUniform17 ) ), ( render.nodeUniform18 * 2.5 ), ( v_positionWorld.z * ( 1.5 * render.nodeUniform17 ) ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ) * 0.35 ) ) * ( ( mix( 1.0, ( ( ( ( fn74( vec3<f32>( ( cos( nodeConst5 ) * ( 1.5 * render.nodeUniform17 ) ), ( sin( nodeConst5 ) * ( 1.5 * render.nodeUniform17 ) ), ( render.nodeUniform18 * 0.6 ) ) ) * 1.0 ) + 0.0 ) * 0.5 ) + 0.5 ), smoothstep( 0.0, render.nodeUniform19, length( nodeConst4 ) ) ) * 0.5 ) + 0.5 ) ), 1.0, clamp( ( nodeConst3 / render.nodeUniform20 ), 0.0, 1.0 ) ) ), 0.0, 1.0 );
			nodeVar10 = mix( vec3<f32>( 0.0, 0.0, 0.0 ), render.nodeUniform6, smoothstep( 0.05, 0.35, nodeConst6 ) );
			nodeVar10 = mix( nodeVar10, render.nodeUniform9, smoothstep( 0.35, 0.65, nodeConst6 ) );
			nodeVar10 = mix( nodeVar10, render.nodeUniform10, smoothstep( 0.65, 1.0, nodeConst6 ) );
			workingLuminanceCoefficients = vec3<f32>( 0.2126, 0.7152, 0.0722 );
			nodeVar8 = fn34( fn32( nodeVar10, render.nodeUniform11, workingLuminanceCoefficients ), render.nodeUniform12 );

		}


		if ( ( 1.0 == 1.0 ) ) {

			nodeVar11 = render.nodeUniform23;

		} else {

			nodeVar11 = render.nodeUniform24;

		}


		if ( ( 1.0 == 1.0 ) ) {

			let nodeConst2 = vec3<f32>( 0.0, render.nodeUniform15, 0.0 );
			let nodeConst3 = length( ( v_positionWorld - ( ( render.nodeUniform13 + render.nodeUniform14 ) + ( nodeConst2 * vec3<f32>( clamp( ( dot( ( v_positionWorld - render.nodeUniform13 ), nodeConst2 ) / dot( nodeConst2, nodeConst2 ) ), 0.0, 1.0 ) ) ) ) ) );
			nodeVar12 = ( ( 1.0 / ( pow( nodeConst3, 2.0 ) + pow( 1.2, 2.0 ) ) ) / ( 1.0 / max( pow( length( ( v_positionWorld - render.nodeUniform13 ) ), 2.0 ), 0.01 ) ) );

		} else {

			nodeVar12 = 1.0;

		}


		if ( ( 1.0 == 1.0 ) ) {

			let nodeConst2 = vec3<f32>( 0.0, render.nodeUniform15, 0.0 );
			let nodeConst3 = length( ( v_positionWorld - ( ( render.nodeUniform13 + render.nodeUniform14 ) + ( nodeConst2 * vec3<f32>( clamp( ( dot( ( v_positionWorld - render.nodeUniform13 ), nodeConst2 ) / dot( nodeConst2, nodeConst2 ) ), 0.0, 1.0 ) ) ) ) ) );
			nodeVar13 = mix( render.nodeUniform26, render.nodeUniform27, smoothstep( 0.0, 1.0, clamp( ( nodeConst3 / render.nodeUniform28 ), 0.0, 1.0 ) ) );

		} else {

			nodeVar13 = 1.0;

		}

		let nodeConst7 = ( nodeVar0 + ( nodeVar4 * vec3<f32>( nodeVar5 ) ) );
		nodeVar14 = ( ( ( ( ( ( ( ( ( nodeVar8 * vec3<f32>( pow( max( ( render.nodeUniform7 / 8.34 ), 0.0 ), 4.0 ) ) ) * vec3<f32>( max( ( render.nodeUniform21 / 11.02 ), 0.0 ) ) ) * vec3<f32>( render.nodeUniform22 ) ) * vec3<f32>( nodeVar11 ) ) * vec3<f32>( render.nodeUniform25 ) ) * vec3<f32>( smoothstep( 0.0, 3.0, render.nodeUniform18 ) ) ) * vec3<f32>( nodeVar12 ) ) * vec3<f32>( nodeVar13 ) ) * vec3<f32>( fn44( length( ( render.nodeUniform29 - ( render.cameraViewMatrix * vec4<f32>( nodeConst7, 1.0 ) ).xyz ) ), render.nodeUniform31, render.nodeUniform32 ) ) );
		nodeVar7 = ( nodeVar7 + nodeVar14 );
		let nodeConst8 = ( render.nodeUniform36 - ( render.cameraViewMatrix * vec4<f32>( nodeConst7, 1.0 ) ).xyz );
		nodeVar15 = ( ( render.nodeUniform33 * vec3<f32>( smoothstep( render.nodeUniform34, render.nodeUniform35, dot( normalize( nodeConst8 ), normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform37 - render.nodeUniform38 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( fn44( length( nodeConst8 ), render.nodeUniform39, render.nodeUniform40 ) ) );
		shadowPositionWorld = nodeConst7;
		normalViewGeometry = normalize( v_normalViewGeometry );
		NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
		normalView = NORMAL_normalView;
		normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
		let nodeConst9 = ( render.nodeUniform42 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform45 ) ) ), 1.0 ) );
		let nodeConst10 = ( nodeConst9.xyz / vec3<f32>( nodeConst9.w ) );
		let nodeConst11 = vec3<f32>( nodeConst10.x, ( 1.0 - nodeConst10.y ), ( nodeConst10.z + render.nodeUniform46 ) );
		nodeVar16 = textureSample( nodeUniform41, nodeUniform41_sampler, nodeConst11.xy );

		if ( ( ( ( ( ( nodeConst11.x >= 0.0 ) && ( nodeConst11.x <= 1.0 ) ) && ( nodeConst11.y >= 0.0 ) ) && ( nodeConst11.y <= 1.0 ) ) && ( nodeConst11.z <= 1.0 ) ) ) {

			let nodeConst12 = ( fn24( fragCoord.xy ) * 6.28318530718 );
			let nodeConst13 = ( vec2<f32>( render.nodeUniform48 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform49 ) );
			let nodeConst14 = ( nodeConst11.xy + ( fn25( 0, 5, nodeConst12 ) * nodeConst13 ) );
			nodeVar18 = textureSampleCompare( nodeUniform47, nodeUniform47_sampler, nodeConst14, nodeConst11.z );
			let nodeConst15 = ( nodeConst11.xy + ( fn25( 1, 5, nodeConst12 ) * nodeConst13 ) );
			nodeVar19 = textureSampleCompare( nodeUniform47, nodeUniform47_sampler, nodeConst15, nodeConst11.z );
			let nodeConst16 = ( nodeConst11.xy + ( fn25( 2, 5, nodeConst12 ) * nodeConst13 ) );
			nodeVar20 = textureSampleCompare( nodeUniform47, nodeUniform47_sampler, nodeConst16, nodeConst11.z );
			let nodeConst17 = ( nodeConst11.xy + ( fn25( 3, 5, nodeConst12 ) * nodeConst13 ) );
			nodeVar21 = textureSampleCompare( nodeUniform47, nodeUniform47_sampler, nodeConst17, nodeConst11.z );
			let nodeConst18 = ( nodeConst11.xy + ( fn25( 4, 5, nodeConst12 ) * nodeConst13 ) );
			nodeVar22 = textureSampleCompare( nodeUniform47, nodeUniform47_sampler, nodeConst18, nodeConst11.z );
			nodeVar17 = ( ( ( ( ( nodeVar18 + nodeVar19 ) + nodeVar20 ) + nodeVar21 ) + nodeVar22 ) * 0.2 );

		} else {

			nodeVar17 = 1.0;

		}

		nodeVar23 = mix( vec4<f32>( 1.0 ), mix( nodeVar16, vec4<f32>( 1.0 ), vec4<f32>( nodeVar17 ) ), ( render.nodeUniform50 * nodeVar16.w ) );
		nodeVar15 = ( vec4<f32>( nodeVar15, 1.0 ) * nodeVar23 ).xyz;
		nodeVar7 = ( nodeVar7 + nodeVar15 );
		nodeVar24 = ( nodeVar7 * vec3<f32>( 0.01 ) );

		if ( nodeVar2 ) {

			nodeVar25 = ( nodeVar25 + ( ( nodeVar24 * nodeVar6 ) * vec3<f32>( nodeVar3 ) ) );
			

		} else {

			let nodeConst19 = exp( ( ( - ( nodeVar7 * vec3<f32>( 0.01 ) ) ) * vec3<f32>( nodeVar3 ) ) );
			nodeVar25 = ( ( nodeVar25 * nodeConst19 ) + ( nodeVar24 * vec3<f32>( nodeVar3 ) ) );
			

		}

		let nodeConst19 = exp( ( ( - ( nodeVar7 * vec3<f32>( 0.01 ) ) ) * vec3<f32>( nodeVar3 ) ) );
		nodeVar6 = ( nodeVar6 * nodeConst19 );
		nodeVar5 = ( nodeVar5 + nodeVar3 );

	}

	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar25;
	let nodeConst20 = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst20;

	// result

	output.color = nodeConst20;

	return output;

}
