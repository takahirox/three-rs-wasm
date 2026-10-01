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
@binding( 3 ) @group( 1 ) var nodeUniform10 : texture_depth_multisampled_2d;
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
	nodeUniform25 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform14 : mat4x4<f32>,
	nodeUniform17 : f32,
	nodeUniform18 : f32,
	nodeUniform22 : f32,
	cameraPosition : vec3<f32>,
	nodeUniform11 : vec2<f32>,
	nodeUniform20 : f32,
	nodeUniform21 : vec2<f32>
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
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : vec2<u32>;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar24 : vec4<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec4<f32>;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec2<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec2<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec2<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : vec2<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : vec2<f32>;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : vec4<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec4<f32>;

// codes
fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}

fn interleavedGradientNoise ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn vogelDiskSample ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	var nodeVar0 : f32;

	nodeVar0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeVar0 ), sin( nodeVar0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

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
	nodeVar15 = textureLoad( nodeUniform5, ( vec2<i32>( ( fragCoord.xy + vec2<f32>( f32( render.nodeUniform6 ) ) ) ) % 16 ), u32( 0u ) );
	nodeVar16 = ( nodeVar15 * vec4<f32>( nodeVar10 ) );
	nodeVar17 = ( vec4<f32>( nodeVar13 ) + nodeVar16 );
	nodeVar13 = nodeVar17.x;

	for ( var i : i32 = 0; i < object.nodeUniform4; i ++ ) {

		nodeVar19 = ( nodeVar4 + ( nodeVar12 * vec3<f32>( nodeVar13 ) ) );
		nodeVar18 = vec3<f32>( ( ( ( ( render.cameraNear * render.cameraFar ) / ( ( ( render.cameraFar - render.cameraNear ) * ( ( ( render.cameraNear + ( render.cameraViewMatrix * vec4<f32>( nodeVar19, 1.0 ) ).xyz.z ) * render.cameraFar ) / ( ( render.cameraFar - render.cameraNear ) * ( render.cameraViewMatrix * vec4<f32>( nodeVar19, 1.0 ) ).xyz.z ) ) ) - render.cameraFar ) ) + render.cameraNear ) / ( render.cameraNear - render.cameraFar ) ) );
		nodeVar21 = textureDimensions( nodeUniform10 );
		nodeVar20 = textureLoad( nodeUniform10, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( fragCoord.xy / render.nodeUniform11 ) ) * vec2<f32>( nodeVar21 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar21 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		nodeVar22 = ( ( ( ( render.cameraNear * render.cameraFar ) / ( ( ( render.cameraFar - render.cameraNear ) * nodeVar20 ) - render.cameraFar ) ) + render.cameraNear ) / ( render.cameraNear - render.cameraFar ) );
		nodeVar23 = vec3<f32>( 0.0 );
		shadowPositionWorld = nodeVar19;
		normalViewGeometry = normalize( v_normalViewGeometry );
		NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
		normalView = NORMAL_normalView;
		normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
		nodeVar24 = ( render.nodeUniform14 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform17 ) ) ), 1.0 ) );
		nodeVar25 = ( nodeVar24.xyz / vec3<f32>( nodeVar24.w ) );
		nodeVar26 = vec3<f32>( nodeVar25.x, ( 1.0 - nodeVar25.y ), ( nodeVar25.z + render.nodeUniform18 ) );
		nodeVar27 = textureSample( nodeUniform13, nodeUniform13_sampler, nodeVar26.xy );

		if ( ( ( ( ( ( nodeVar26.x >= 0.0 ) && ( nodeVar26.x <= 1.0 ) ) && ( nodeVar26.y >= 0.0 ) ) && ( nodeVar26.y <= 1.0 ) ) && ( nodeVar26.z <= 1.0 ) ) ) {

			nodeVar29 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar30 = ( render.nodeUniform20 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform21 ).x );
			nodeVar31 = ( nodeVar26.xy + ( vogelDiskSample( 0, 5, nodeVar29 ) * vec2<f32>( nodeVar30 ) ) );
			nodeVar32 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar31, nodeVar26.z );
			nodeVar33 = ( nodeVar26.xy + ( vogelDiskSample( 1, 5, nodeVar29 ) * vec2<f32>( nodeVar30 ) ) );
			nodeVar34 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar33, nodeVar26.z );
			nodeVar35 = ( nodeVar26.xy + ( vogelDiskSample( 2, 5, nodeVar29 ) * vec2<f32>( nodeVar30 ) ) );
			nodeVar36 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar35, nodeVar26.z );
			nodeVar37 = ( nodeVar26.xy + ( vogelDiskSample( 3, 5, nodeVar29 ) * vec2<f32>( nodeVar30 ) ) );
			nodeVar38 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar37, nodeVar26.z );
			nodeVar39 = ( nodeVar26.xy + ( vogelDiskSample( 4, 5, nodeVar29 ) * vec2<f32>( nodeVar30 ) ) );
			nodeVar40 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar39, nodeVar26.z );
			nodeVar28 = ( ( ( ( ( nodeVar32 + nodeVar34 ) + nodeVar36 ) + nodeVar38 ) + nodeVar40 ) * 0.2 );

		} else {

			nodeVar28 = 1.0;

		}

		nodeVar41 = mix( vec4<f32>( 1.0 ), mix( nodeVar27, vec4<f32>( 1.0 ), vec4<f32>( nodeVar28 ) ), ( render.nodeUniform22 * nodeVar27.w ) );
		nodeVar42 = ( render.nodeUniform25 - ( render.cameraViewMatrix * vec4<f32>( nodeVar19, 1.0 ) ).xyz );

		if ( ( render.nodeUniform28 > 0.0 ) ) {

			nodeVar44 = length( nodeVar42 );
			nodeVar45 = ( nodeVar44 / render.nodeUniform28 );
			nodeVar46 = clamp( ( 1.0 - ( ( ( nodeVar45 * nodeVar45 ) * nodeVar45 ) * nodeVar45 ) ), 0.0, 1.0 );
			nodeVar43 = ( ( 1.0 / max( pow( nodeVar44, render.nodeUniform29 ), 0.01 ) ) * ( nodeVar46 * nodeVar46 ) );

		} else {

			nodeVar43 = ( 1.0 / max( pow( length( nodeVar42 ), render.nodeUniform29 ), 0.01 ) );

		}

		nodeVar47 = ( ( ( vec4<f32>( render.nodeUniform12, 1.0 ) * nodeVar41 ) * vec4<f32>( smoothstep( render.nodeUniform23, render.nodeUniform24, dot( normalize( nodeVar42 ), normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform26 - render.nodeUniform27 ), 0.0 ) ).xyz ) ) ) ) ) * vec4<f32>( nodeVar43 ) ).xyz;
		nodeVar47 = ( vec4<f32>( nodeVar47, 1.0 ) * nodeVar41 ).xyz;

		if ( all( ( vec3<f32>( nodeVar22 ) >= nodeVar18 ) ) ) {

			nodeVar23 = ( nodeVar23 + nodeVar47 );
			

		}

		nodeVar48 = vec3<f32>( ( render.nodeUniform31 * 0.01 ), 0.0, ( render.nodeUniform31 * 0.03 ) );
		nodeVar49 = textureSampleLevel( nodeUniform30, nodeUniform30_sampler, tsl_mod_vec3( ( ( nodeVar19 + ( nodeVar48 * vec3<f32>( 1.0 ) ) ) * vec3<f32>( 1.0 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar50 = textureSampleLevel( nodeUniform30, nodeUniform30_sampler, tsl_mod_vec3( ( ( nodeVar19 + ( nodeVar48 * vec3<f32>( 1.0 ) ) ) * vec3<f32>( 0.5 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar51 = textureSampleLevel( nodeUniform30, nodeUniform30_sampler, tsl_mod_vec3( ( ( nodeVar19 + ( nodeVar48 * vec3<f32>( 2.0 ) ) ) * vec3<f32>( 0.2 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar23 = ( nodeVar23 * vec3<f32>( mix( 1.0, ( ( ( nodeVar49 + 0.5 ) * ( nodeVar50 + 0.5 ) ) * ( nodeVar51 + 0.5 ) ), object.nodeUniform32 ) ) );
		nodeVar52 = ( nodeVar23 * vec3<f32>( 0.01 ) );

		if ( nodeVar6 ) {

			nodeVar53 = ( nodeVar53 + ( ( nodeVar52 * nodeVar14 ) * vec3<f32>( nodeVar10 ) ) );
			

		} else {

			nodeVar53 = ( ( nodeVar53 * exp( ( ( - ( nodeVar23 * vec3<f32>( 0.01 ) ) ) * vec3<f32>( nodeVar10 ) ) ) ) + ( nodeVar52 * vec3<f32>( nodeVar10 ) ) );
			

		}

		nodeVar14 = ( nodeVar14 * exp( ( ( - ( nodeVar23 * vec3<f32>( 0.01 ) ) ) * vec3<f32>( nodeVar10 ) ) ) );
		nodeVar13 = ( nodeVar13 + nodeVar10 );

	}

	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar54 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar54;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar55 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar55;
	nodeVar56 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar56;
	outgoingLight = nodeVar53;
	nodeVar57 = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar57;

	// result

	output.color = nodeVar57;

	return output;

}
