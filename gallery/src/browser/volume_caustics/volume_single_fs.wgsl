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
@binding( 1 ) @group( 1 ) var nodeUniform5_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform5 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform10 : texture_depth_2d;
@binding( 4 ) @group( 1 ) var nodeUniform13_sampler : sampler;
@binding( 5 ) @group( 1 ) var nodeUniform13 : texture_2d<f32>;
@binding( 6 ) @group( 1 ) var nodeUniform19_sampler : sampler_comparison;
@binding( 7 ) @group( 1 ) var nodeUniform19 : texture_depth_2d;
@binding( 8 ) @group( 1 ) var nodeUniform30_sampler : sampler;
@binding( 9 ) @group( 1 ) var nodeUniform30 : texture_3d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : i32,
	nodeUniform16 : mat3x3<f32>,
	nodeUniform32 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraNear : f32,
	cameraFar : f32,
	nodeUniform31 : f32,
	nodeUniform6 : u32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform23 : f32,
	nodeUniform24 : f32,
	nodeUniform28 : f32,
	nodeUniform29 : f32,
	nodeUniform12 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform14 : mat4x4<f32>,
	nodeUniform17 : f32,
	nodeUniform18 : f32,
	nodeUniform20 : f32,
	nodeUniform21 : vec2<f32>,
	nodeUniform22 : f32,
	cameraPosition : vec3<f32>,
	nodeUniform11 : vec2<f32>
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
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : vec2<u32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}

fn fn24 ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn fn25 ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	

	let nodeConst0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeConst0 ), sin( nodeConst0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

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


fn tsl_mod_vec3( x : vec3f, y : vec3f ) -> vec3f { return x - y * floor( x / y ); }


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
	nodeVar7 = textureLoad( nodeUniform5, ( vec2<i32>( ( fragCoord.xy + vec2<f32>( f32( render.nodeUniform6 ) ) ) ) % vec2<i32>( 16 ) ), u32( 0u ) );
	nodeVar5 = ( vec4<f32>( nodeVar5 ) + ( nodeVar7 * vec4<f32>( nodeVar3 ) ) ).x;

	for ( var i : i32 = 0; i < object.nodeUniform4; i ++ ) {

		let nodeConst1 = ( nodeVar0 + ( nodeVar4 * vec3<f32>( nodeVar5 ) ) );
		nodeVar8 = ( ( ( ( render.cameraNear * render.cameraFar ) / ( ( ( render.cameraFar - render.cameraNear ) * ( ( ( render.cameraNear + ( render.cameraViewMatrix * vec4<f32>( nodeConst1, 1.0 ) ).xyz.z ) * render.cameraFar ) / ( ( render.cameraFar - render.cameraNear ) * ( render.cameraViewMatrix * vec4<f32>( nodeConst1, 1.0 ) ).xyz.z ) ) ) - render.cameraFar ) ) + render.cameraNear ) / ( render.cameraNear - render.cameraFar ) );
		nodeVar10 = textureDimensions( nodeUniform10, u32( 0 ) );
		nodeVar9 = textureLoad( nodeUniform10, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( fragCoord.xy / render.nodeUniform11 ) ) * vec2<f32>( nodeVar10 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar10 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		nodeVar11 = ( ( ( ( render.cameraNear * render.cameraFar ) / ( ( ( render.cameraFar - render.cameraNear ) * nodeVar9 ) - render.cameraFar ) ) + render.cameraNear ) / ( render.cameraNear - render.cameraFar ) );
		nodeVar12 = vec3<f32>( 0.0 );
		shadowPositionWorld = nodeConst1;
		normalViewGeometry = normalize( v_normalViewGeometry );
		NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
		normalView = NORMAL_normalView;
		normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
		let nodeConst2 = ( render.nodeUniform14 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform17 ) ) ), 1.0 ) );
		let nodeConst3 = ( nodeConst2.xyz / vec3<f32>( nodeConst2.w ) );
		let nodeConst4 = vec3<f32>( nodeConst3.x, ( 1.0 - nodeConst3.y ), ( nodeConst3.z + render.nodeUniform18 ) );
		nodeVar13 = textureSample( nodeUniform13, nodeUniform13_sampler, nodeConst4.xy );

		if ( ( ( ( ( ( nodeConst4.x >= 0.0 ) && ( nodeConst4.x <= 1.0 ) ) && ( nodeConst4.y >= 0.0 ) ) && ( nodeConst4.y <= 1.0 ) ) && ( nodeConst4.z <= 1.0 ) ) ) {

			let nodeConst5 = ( fn24( fragCoord.xy ) * 6.28318530718 );
			let nodeConst6 = ( vec2<f32>( render.nodeUniform20 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform21 ) );
			let nodeConst7 = ( nodeConst4.xy + ( fn25( 0, 5, nodeConst5 ) * nodeConst6 ) );
			nodeVar15 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst7, nodeConst4.z );
			let nodeConst8 = ( nodeConst4.xy + ( fn25( 1, 5, nodeConst5 ) * nodeConst6 ) );
			nodeVar16 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst8, nodeConst4.z );
			let nodeConst9 = ( nodeConst4.xy + ( fn25( 2, 5, nodeConst5 ) * nodeConst6 ) );
			nodeVar17 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst9, nodeConst4.z );
			let nodeConst10 = ( nodeConst4.xy + ( fn25( 3, 5, nodeConst5 ) * nodeConst6 ) );
			nodeVar18 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst10, nodeConst4.z );
			let nodeConst11 = ( nodeConst4.xy + ( fn25( 4, 5, nodeConst5 ) * nodeConst6 ) );
			nodeVar19 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst11, nodeConst4.z );
			nodeVar14 = ( ( ( ( ( nodeVar15 + nodeVar16 ) + nodeVar17 ) + nodeVar18 ) + nodeVar19 ) * 0.2 );

		} else {

			nodeVar14 = 1.0;

		}

		nodeVar20 = mix( vec4<f32>( 1.0 ), mix( nodeVar13, vec4<f32>( 1.0 ), vec4<f32>( nodeVar14 ) ), ( render.nodeUniform22 * nodeVar13.w ) );
		let nodeConst12 = ( render.nodeUniform25 - ( render.cameraViewMatrix * vec4<f32>( nodeConst1, 1.0 ) ).xyz );
		nodeVar21 = ( ( ( vec4<f32>( render.nodeUniform12, 1.0 ) * nodeVar20 ) * vec4<f32>( smoothstep( render.nodeUniform23, render.nodeUniform24, dot( normalize( nodeConst12 ), normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform26 - render.nodeUniform27 ), 0.0 ) ).xyz ) ) ) ) ) * vec4<f32>( fn44( length( nodeConst12 ), render.nodeUniform28, render.nodeUniform29 ) ) ).xyz;
		nodeVar21 = ( vec4<f32>( nodeVar21, 1.0 ) * nodeVar20 ).xyz;

		if ( ( nodeVar11 >= nodeVar8 ) ) {

			nodeVar12 = ( nodeVar12 + nodeVar21 );
			

		}

		let nodeConst13 = vec3<f32>( ( render.nodeUniform31 * 0.01 ), 0.0, ( render.nodeUniform31 * 0.03 ) );
		nodeVar22 = textureSampleLevel( nodeUniform30, nodeUniform30_sampler, tsl_mod_vec3( ( ( nodeConst1 + ( nodeConst13 * vec3<f32>( 1.0 ) ) ) * vec3<f32>( 1.0 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar23 = textureSampleLevel( nodeUniform30, nodeUniform30_sampler, tsl_mod_vec3( ( ( nodeConst1 + ( nodeConst13 * vec3<f32>( 1.0 ) ) ) * vec3<f32>( 0.5 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar24 = textureSampleLevel( nodeUniform30, nodeUniform30_sampler, tsl_mod_vec3( ( ( nodeConst1 + ( nodeConst13 * vec3<f32>( 2.0 ) ) ) * vec3<f32>( 0.2 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar12 = ( nodeVar12 * vec3<f32>( mix( 1.0, ( ( ( nodeVar22 + 0.5 ) * ( nodeVar23 + 0.5 ) ) * ( nodeVar24 + 0.5 ) ), object.nodeUniform32 ) ) );
		nodeVar25 = ( nodeVar12 * vec3<f32>( 0.01 ) );

		if ( nodeVar2 ) {

			nodeVar26 = ( nodeVar26 + ( ( nodeVar25 * nodeVar6 ) * vec3<f32>( nodeVar3 ) ) );
			

		} else {

			let nodeConst14 = exp( ( ( - ( nodeVar12 * vec3<f32>( 0.01 ) ) ) * vec3<f32>( nodeVar3 ) ) );
			nodeVar26 = ( ( nodeVar26 * nodeConst14 ) + ( nodeVar25 * vec3<f32>( nodeVar3 ) ) );
			

		}

		let nodeConst14 = exp( ( ( - ( nodeVar12 * vec3<f32>( 0.01 ) ) ) * vec3<f32>( nodeVar3 ) ) );
		nodeVar6 = ( nodeVar6 * nodeConst14 );
		nodeVar5 = ( nodeVar5 + nodeVar3 );

	}

	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar26;
	let nodeConst15 = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst15;

	// result

	output.color = nodeConst15;

	return output;

}
