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
@binding( 3 ) @group( 1 ) var nodeUniform9 : texture_depth_2d;
@binding( 4 ) @group( 1 ) var nodeUniform19_sampler : sampler_comparison;
@binding( 5 ) @group( 1 ) var nodeUniform19 : texture_depth_cube;
@binding( 6 ) @group( 1 ) var nodeUniform30_sampler : sampler_comparison;
@binding( 7 ) @group( 1 ) var nodeUniform30 : texture_depth_2d;
@binding( 8 ) @group( 1 ) var nodeUniform41_sampler : sampler;
@binding( 9 ) @group( 1 ) var nodeUniform41 : texture_2d<f32>;
@binding( 10 ) @group( 1 ) var nodeUniform42_sampler : sampler;
@binding( 11 ) @group( 1 ) var nodeUniform42 : texture_3d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : i32,
	nodeUniform14 : mat3x3<f32>,
	nodeUniform44 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraNear : f32,
	cameraFar : f32,
	nodeUniform43 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform23 : f32,
	nodeUniform25 : f32,
	nodeUniform34 : f32,
	nodeUniform35 : f32,
	nodeUniform39 : f32,
	nodeUniform40 : f32,
	nodeUniform27 : vec3<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform36 : vec3<f32>,
	nodeUniform37 : vec3<f32>,
	nodeUniform38 : vec3<f32>,
	nodeUniform26 : mat4x4<f32>,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform15 : f32,
	nodeUniform22 : f32,
	nodeUniform28 : f32,
	nodeUniform29 : f32,
	nodeUniform33 : f32,
	cameraPosition : vec3<f32>,
	nodeUniform10 : vec2<f32>,
	nodeUniform17 : f32,
	nodeUniform16 : f32,
	nodeUniform18 : f32,
	nodeUniform20 : f32,
	nodeUniform21 : vec2<f32>,
	nodeUniform31 : f32,
	nodeUniform32 : vec2<f32>
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
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec2<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec2<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : vec2<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : vec2<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : vec2<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec4<f32>;
var<private> nodeVar57 : vec4<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : vec4<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : vec2<f32>;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : vec2<f32>;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : vec2<f32>;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : vec2<f32>;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : vec2<f32>;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : vec4<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec4<f32>;

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
	nodeVar15 = textureLoad( nodeUniform5, ( vec2<i32>( fragCoord.xy ) % 16 ), u32( 0u ) );
	nodeVar16 = ( nodeVar15 * vec4<f32>( nodeVar10 ) );
	nodeVar17 = ( vec4<f32>( nodeVar13 ) + nodeVar16 );
	nodeVar13 = nodeVar17.x;

	for ( var i : i32 = 0; i < object.nodeUniform4; i ++ ) {

		nodeVar19 = ( nodeVar4 + ( nodeVar12 * vec3<f32>( nodeVar13 ) ) );
		nodeVar18 = vec3<f32>( ( ( ( ( render.cameraNear * render.cameraFar ) / ( ( ( render.cameraFar - render.cameraNear ) * ( ( ( render.cameraNear + ( render.cameraViewMatrix * vec4<f32>( nodeVar19, 1.0 ) ).xyz.z ) * render.cameraFar ) / ( ( render.cameraFar - render.cameraNear ) * ( render.cameraViewMatrix * vec4<f32>( nodeVar19, 1.0 ) ).xyz.z ) ) ) - render.cameraFar ) ) + render.cameraNear ) / ( render.cameraNear - render.cameraFar ) ) );
		nodeVar21 = textureDimensions( nodeUniform9, u32( 0 ) );
		nodeVar20 = textureLoad( nodeUniform9, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( fragCoord.xy / render.nodeUniform10 ) ) * vec2<f32>( nodeVar21 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar21 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		nodeVar22 = ( ( ( ( render.cameraNear * render.cameraFar ) / ( ( ( render.cameraFar - render.cameraNear ) * nodeVar20 ) - render.cameraFar ) ) + render.cameraNear ) / ( render.cameraNear - render.cameraFar ) );
		nodeVar23 = vec3<f32>( 0.0 );
		shadowPositionWorld = nodeVar19;
		normalViewGeometry = normalize( v_normalViewGeometry );
		NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
		normalView = NORMAL_normalView;
		normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
		let nodeConst0 = ( render.nodeUniform12 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform15 ) ) ), 1.0 ) ).xyz;
		let nodeConst1 = abs( nodeConst0 );
		nodeVar24 = 1.0;
		nodeVar25 = max( max( nodeConst1.x, nodeConst1.y ), nodeConst1.z );

		if ( ( ( ( nodeVar25 - render.nodeUniform16 ) <= 0.0 ) && ( ( nodeVar25 - render.nodeUniform17 ) >= 0.0 ) ) ) {

			nodeVar26 = ( - nodeVar25 );
			nodeVar27 = ( ( ( render.nodeUniform17 + nodeVar26 ) * render.nodeUniform16 ) / ( ( render.nodeUniform16 - render.nodeUniform17 ) * nodeVar26 ) );
			nodeVar27 = ( nodeVar27 + render.nodeUniform18 );
			nodeVar28 = normalize( nodeConst0 );
			nodeVar30 = abs( nodeVar28 );

			if ( ( nodeVar30.x > nodeVar30.z ) ) {

				nodeVar29 = vec3<f32>( 0.0, 1.0, 0.0 );

			} else {

				nodeVar29 = vec3<f32>( 1.0, 0.0, 0.0 );

			}

			nodeVar31 = normalize( cross( nodeVar28, nodeVar29 ) );
			nodeVar32 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar33 = vogelDiskSample( 0, 5, nodeVar32 );
			nodeVar34 = cross( nodeVar28, nodeVar31 );
			nodeVar35 = ( render.nodeUniform20 / render.nodeUniform21.x );
			nodeVar36 = ( nodeVar28 + ( ( ( nodeVar31 * vec3<f32>( nodeVar33.x ) ) + ( nodeVar34 * vec3<f32>( nodeVar33.y ) ) ) * vec3<f32>( nodeVar35 ) ) );
			nodeVar37 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, vec3<f32>( nodeVar36.x, ( - nodeVar36.y ), nodeVar36.z ), nodeVar27 );
			nodeVar38 = vogelDiskSample( 1, 5, nodeVar32 );
			nodeVar39 = ( nodeVar28 + ( ( ( nodeVar31 * vec3<f32>( nodeVar38.x ) ) + ( nodeVar34 * vec3<f32>( nodeVar38.y ) ) ) * vec3<f32>( nodeVar35 ) ) );
			nodeVar40 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, vec3<f32>( nodeVar39.x, ( - nodeVar39.y ), nodeVar39.z ), nodeVar27 );
			nodeVar41 = vogelDiskSample( 2, 5, nodeVar32 );
			nodeVar42 = ( nodeVar28 + ( ( ( nodeVar31 * vec3<f32>( nodeVar41.x ) ) + ( nodeVar34 * vec3<f32>( nodeVar41.y ) ) ) * vec3<f32>( nodeVar35 ) ) );
			nodeVar43 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, vec3<f32>( nodeVar42.x, ( - nodeVar42.y ), nodeVar42.z ), nodeVar27 );
			nodeVar44 = vogelDiskSample( 3, 5, nodeVar32 );
			nodeVar45 = ( nodeVar28 + ( ( ( nodeVar31 * vec3<f32>( nodeVar44.x ) ) + ( nodeVar34 * vec3<f32>( nodeVar44.y ) ) ) * vec3<f32>( nodeVar35 ) ) );
			nodeVar46 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, vec3<f32>( nodeVar45.x, ( - nodeVar45.y ), nodeVar45.z ), nodeVar27 );
			nodeVar47 = vogelDiskSample( 4, 5, nodeVar32 );
			nodeVar48 = ( nodeVar28 + ( ( ( nodeVar31 * vec3<f32>( nodeVar47.x ) ) + ( nodeVar34 * vec3<f32>( nodeVar47.y ) ) ) * vec3<f32>( nodeVar35 ) ) );
			nodeVar49 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, vec3<f32>( nodeVar48.x, ( - nodeVar48.y ), nodeVar48.z ), nodeVar27 );
			nodeVar24 = ( ( ( ( ( nodeVar37 + nodeVar40 ) + nodeVar43 ) + nodeVar46 ) + nodeVar49 ) * 0.2 );
			

		}

		nodeVar50 = mix( 1.0, nodeVar24, render.nodeUniform22 );

		if ( ( render.nodeUniform23 > 0.0 ) ) {

			nodeVar52 = length( ( render.nodeUniform24 - ( render.cameraViewMatrix * vec4<f32>( nodeVar19, 1.0 ) ).xyz ) );
			nodeVar53 = ( nodeVar52 / render.nodeUniform23 );
			nodeVar54 = clamp( ( 1.0 - ( ( ( nodeVar53 * nodeVar53 ) * nodeVar53 ) * nodeVar53 ) ), 0.0, 1.0 );
			nodeVar51 = ( ( 1.0 / max( pow( nodeVar52, render.nodeUniform25 ), 0.01 ) ) * ( nodeVar54 * nodeVar54 ) );

		} else {

			nodeVar51 = ( 1.0 / max( pow( length( ( render.nodeUniform24 - ( render.cameraViewMatrix * vec4<f32>( nodeVar19, 1.0 ) ).xyz ) ), render.nodeUniform25 ), 0.01 ) );

		}

		nodeVar55 = ( ( render.nodeUniform11 * vec3<f32>( nodeVar50 ) ) * vec3<f32>( nodeVar51 ) );
		nodeVar55 = ( nodeVar55 * vec3<f32>( nodeVar50 ) );

		if ( all( ( vec3<f32>( nodeVar22 ) >= nodeVar18 ) ) ) {

			nodeVar23 = ( nodeVar23 + nodeVar55 );
			

		}

		nodeVar57 = ( render.nodeUniform26 * vec4<f32>( nodeVar19, 1.0 ) );
		nodeVar58 = ( nodeVar57.xyz / vec3<f32>( nodeVar57.w ) );

		if ( all( ( abs( ( ( nodeVar58 * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ) ) < vec3<f32>( 1.0 ) ) ) ) {

			shadowPositionWorld = nodeVar19;
			nodeVar60 = ( render.nodeUniform26 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform28 ) ) ), 1.0 ) );
			nodeVar61 = ( nodeVar60.xyz / vec3<f32>( nodeVar60.w ) );
			nodeVar62 = vec3<f32>( nodeVar61.x, ( 1.0 - nodeVar61.y ), ( nodeVar61.z + render.nodeUniform29 ) );

			if ( ( ( ( ( ( nodeVar62.x >= 0.0 ) && ( nodeVar62.x <= 1.0 ) ) && ( nodeVar62.y >= 0.0 ) ) && ( nodeVar62.y <= 1.0 ) ) && ( nodeVar62.z <= 1.0 ) ) ) {

				nodeVar63 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
				nodeVar64 = ( render.nodeUniform31 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform32 ).x );
				nodeVar65 = ( nodeVar62.xy + ( vogelDiskSample( 0, 5, nodeVar63 ) * vec2<f32>( nodeVar64 ) ) );
				nodeVar66 = textureSampleCompare( nodeUniform30, nodeUniform30_sampler, nodeVar65, nodeVar62.z );
				nodeVar67 = ( nodeVar62.xy + ( vogelDiskSample( 1, 5, nodeVar63 ) * vec2<f32>( nodeVar64 ) ) );
				nodeVar68 = textureSampleCompare( nodeUniform30, nodeUniform30_sampler, nodeVar67, nodeVar62.z );
				nodeVar69 = ( nodeVar62.xy + ( vogelDiskSample( 2, 5, nodeVar63 ) * vec2<f32>( nodeVar64 ) ) );
				nodeVar70 = textureSampleCompare( nodeUniform30, nodeUniform30_sampler, nodeVar69, nodeVar62.z );
				nodeVar71 = ( nodeVar62.xy + ( vogelDiskSample( 3, 5, nodeVar63 ) * vec2<f32>( nodeVar64 ) ) );
				nodeVar72 = textureSampleCompare( nodeUniform30, nodeUniform30_sampler, nodeVar71, nodeVar62.z );
				nodeVar73 = ( nodeVar62.xy + ( vogelDiskSample( 4, 5, nodeVar63 ) * vec2<f32>( nodeVar64 ) ) );
				nodeVar74 = textureSampleCompare( nodeUniform30, nodeUniform30_sampler, nodeVar73, nodeVar62.z );
				nodeVar59 = ( ( ( ( ( nodeVar66 + nodeVar68 ) + nodeVar70 ) + nodeVar72 ) + nodeVar74 ) * 0.2 );

			} else {

				nodeVar59 = 1.0;

			}

			nodeVar75 = mix( 1.0, nodeVar59, render.nodeUniform33 );
			nodeVar76 = ( render.nodeUniform36 - ( render.cameraViewMatrix * vec4<f32>( nodeVar19, 1.0 ) ).xyz );

			if ( ( render.nodeUniform39 > 0.0 ) ) {

				nodeVar78 = length( nodeVar76 );
				nodeVar79 = ( nodeVar78 / render.nodeUniform39 );
				nodeVar80 = clamp( ( 1.0 - ( ( ( nodeVar79 * nodeVar79 ) * nodeVar79 ) * nodeVar79 ) ), 0.0, 1.0 );
				nodeVar77 = ( ( 1.0 / max( pow( nodeVar78, render.nodeUniform40 ), 0.01 ) ) * ( nodeVar80 * nodeVar80 ) );

			} else {

				nodeVar77 = ( 1.0 / max( pow( length( nodeVar76 ), render.nodeUniform40 ), 0.01 ) );

			}

			nodeVar81 = textureSample( nodeUniform41, nodeUniform41_sampler, nodeVar58.xy );
			nodeVar56 = ( vec4<f32>( ( ( ( render.nodeUniform27 * vec3<f32>( nodeVar75 ) ) * vec3<f32>( smoothstep( render.nodeUniform34, render.nodeUniform35, dot( normalize( nodeVar76 ), normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform37 - render.nodeUniform38 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( nodeVar77 ) ), 1.0 ) * nodeVar81 );

		} else {

			shadowPositionWorld = nodeVar19;
			nodeVar75 = mix( 1.0, nodeVar59, render.nodeUniform33 );
			nodeVar75 = mix( 1.0, nodeVar59, render.nodeUniform33 );
			nodeVar82 = ( render.nodeUniform36 - ( render.cameraViewMatrix * vec4<f32>( nodeVar19, 1.0 ) ).xyz );

			if ( ( render.nodeUniform39 > 0.0 ) ) {

				nodeVar84 = length( nodeVar82 );
				nodeVar85 = ( nodeVar84 / render.nodeUniform39 );
				nodeVar86 = clamp( ( 1.0 - ( ( ( nodeVar85 * nodeVar85 ) * nodeVar85 ) * nodeVar85 ) ), 0.0, 1.0 );
				nodeVar83 = ( ( 1.0 / max( pow( nodeVar84, render.nodeUniform40 ), 0.01 ) ) * ( nodeVar86 * nodeVar86 ) );

			} else {

				nodeVar83 = ( 1.0 / max( pow( length( nodeVar82 ), render.nodeUniform40 ), 0.01 ) );

			}

			nodeVar56 = vec4<f32>( ( ( ( render.nodeUniform27 * vec3<f32>( nodeVar75 ) ) * vec3<f32>( smoothstep( render.nodeUniform34, render.nodeUniform35, dot( normalize( nodeVar82 ), normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform37 - render.nodeUniform38 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( nodeVar83 ) ), 1.0 );

		}

		nodeVar87 = nodeVar56.xyz;
		shadowPositionWorld = nodeVar19;
		nodeVar87 = ( nodeVar87 * vec3<f32>( nodeVar75 ) );

		if ( all( ( vec3<f32>( nodeVar22 ) >= nodeVar18 ) ) ) {

			nodeVar23 = ( nodeVar23 + nodeVar87 );
			

		}

		nodeVar88 = vec3<f32>( render.nodeUniform43, 0.0, ( render.nodeUniform43 * 0.3 ) );
		nodeVar89 = textureSampleLevel( nodeUniform42, nodeUniform42_sampler, tsl_mod_vec3( ( ( nodeVar19 + ( nodeVar88 * vec3<f32>( 1.0 ) ) ) * vec3<f32>( 0.1 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar90 = textureSampleLevel( nodeUniform42, nodeUniform42_sampler, tsl_mod_vec3( ( ( nodeVar19 + ( nodeVar88 * vec3<f32>( 1.0 ) ) ) * vec3<f32>( 0.05 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar91 = textureSampleLevel( nodeUniform42, nodeUniform42_sampler, tsl_mod_vec3( ( ( nodeVar19 + ( nodeVar88 * vec3<f32>( 2.0 ) ) ) * vec3<f32>( 0.02 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar23 = ( nodeVar23 * vec3<f32>( mix( 1.0, ( ( ( nodeVar89 + 0.5 ) * ( nodeVar90 + 0.5 ) ) * ( nodeVar91 + 0.5 ) ), object.nodeUniform44 ) ) );
		nodeVar92 = ( nodeVar23 * vec3<f32>( 0.01 ) );

		if ( nodeVar6 ) {

			nodeVar93 = ( nodeVar93 + ( ( nodeVar92 * nodeVar14 ) * vec3<f32>( nodeVar10 ) ) );
			

		} else {

			nodeVar93 = ( ( nodeVar93 * exp( ( ( - ( nodeVar23 * vec3<f32>( 0.01 ) ) ) * vec3<f32>( nodeVar10 ) ) ) ) + ( nodeVar92 * vec3<f32>( nodeVar10 ) ) );
			

		}

		nodeVar14 = ( nodeVar14 * exp( ( ( - ( nodeVar23 * vec3<f32>( 0.01 ) ) ) * vec3<f32>( nodeVar10 ) ) ) );
		nodeVar13 = ( nodeVar13 + nodeVar10 );

	}

	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar94 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar94;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar95 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar95;
	nodeVar96 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar96;
	outgoingLight = nodeVar93;
	nodeVar97 = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar97;

	// result

	output.color = nodeVar97;

	return output;

}
