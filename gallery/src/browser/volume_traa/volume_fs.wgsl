// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputType {
	@location( 0 ) m0 : vec4<f32>,
	@location( 1 ) m1 : vec4<f32>,
	
};
var<private> output : OutputType;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform10 : texture_depth_2d;
@binding( 2 ) @group( 1 ) var nodeUniform20_sampler : sampler_comparison;
@binding( 3 ) @group( 1 ) var nodeUniform20 : texture_depth_cube;
@binding( 4 ) @group( 1 ) var nodeUniform31_sampler : sampler_comparison;
@binding( 5 ) @group( 1 ) var nodeUniform31 : texture_depth_2d;
@binding( 6 ) @group( 1 ) var nodeUniform42_sampler : sampler;
@binding( 7 ) @group( 1 ) var nodeUniform42 : texture_2d<f32>;
@binding( 8 ) @group( 1 ) var nodeUniform43_sampler : sampler;
@binding( 9 ) @group( 1 ) var nodeUniform43 : texture_3d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : i32,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform15 : mat3x3<f32>,
	nodeUniform44 : f32,
	nodeUniform45 : f32,
	nodeUniform46 : mat4x4<f32>,
	nodeUniform47 : mat4x4<f32>,
	nodeUniform49 : mat4x4<f32>,
	nodeUniform50 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraNear : f32,
	cameraFar : f32,
	nodeUniform48 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform24 : f32,
	nodeUniform26 : f32,
	nodeUniform35 : f32,
	nodeUniform36 : f32,
	nodeUniform40 : f32,
	nodeUniform41 : f32,
	nodeUniform28 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform37 : vec3<f32>,
	nodeUniform38 : vec3<f32>,
	nodeUniform39 : vec3<f32>,
	nodeUniform27 : mat4x4<f32>,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform16 : f32,
	nodeUniform23 : f32,
	nodeUniform29 : f32,
	nodeUniform30 : f32,
	nodeUniform34 : f32,
	cameraPosition : vec3<f32>,
	nodeUniform11 : vec2<f32>,
	nodeUniform18 : f32,
	nodeUniform17 : f32,
	nodeUniform19 : f32,
	nodeUniform21 : f32,
	nodeUniform22 : vec2<f32>,
	nodeUniform32 : f32,
	nodeUniform33 : vec2<f32>
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
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec2<u32>;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : vec2<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : vec2<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec2<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : vec2<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : vec2<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec4<f32>;
var<private> nodeVar56 : vec4<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : vec4<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : vec2<f32>;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : vec2<f32>;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : vec2<f32>;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : vec2<f32>;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : vec2<f32>;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : vec4<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> nodeVar96 : vec4<f32>;
var<private> nodeVar97 : vec4<f32>;
var<private> nodeVar98 : vec2<f32>;

// codes
fn interleavedGradientNoise ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}

fn vogelDiskSample ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	var nodeVar0 : f32;

	nodeVar0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeVar0 ), sin( nodeVar0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

}


fn tsl_mod_vec3( x : vec3f, y : vec3f ) -> vec3f { return x - y * floor( x / y ); }


@fragment
fn main( @location( 0 ) positionLocal : vec3<f32>,
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) positionPrevious : vec3<f32>,
	@location( 3 ) v_normalViewGeometry : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputType {

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
	nodeVar15 = ( fract( ( interleavedGradientNoise( ( fragCoord.xy + ( vec2<f32>( object.nodeUniform5, object.nodeUniform6 ) * vec2<f32>( 100.0 ) ) ) ) + object.nodeUniform5 ) ) * nodeVar10 );
	nodeVar16 = ( nodeVar13 + nodeVar15 );
	nodeVar13 = nodeVar16;

	for ( var i : i32 = 0; i < object.nodeUniform4; i ++ ) {

		nodeVar18 = ( nodeVar4 + ( nodeVar12 * vec3<f32>( nodeVar13 ) ) );
		nodeVar17 = vec3<f32>( ( ( ( ( render.cameraNear * render.cameraFar ) / ( ( ( render.cameraFar - render.cameraNear ) * ( ( ( render.cameraNear + ( render.cameraViewMatrix * vec4<f32>( nodeVar18, 1.0 ) ).xyz.z ) * render.cameraFar ) / ( ( render.cameraFar - render.cameraNear ) * ( render.cameraViewMatrix * vec4<f32>( nodeVar18, 1.0 ) ).xyz.z ) ) ) - render.cameraFar ) ) + render.cameraNear ) / ( render.cameraNear - render.cameraFar ) ) );
		nodeVar20 = textureDimensions( nodeUniform10, u32( 0 ) );
		nodeVar19 = textureLoad( nodeUniform10, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( fragCoord.xy / render.nodeUniform11 ) ) * vec2<f32>( nodeVar20 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar20 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		nodeVar21 = ( ( ( ( render.cameraNear * render.cameraFar ) / ( ( ( render.cameraFar - render.cameraNear ) * nodeVar19 ) - render.cameraFar ) ) + render.cameraNear ) / ( render.cameraNear - render.cameraFar ) );
		nodeVar22 = vec3<f32>( 0.0 );
		shadowPositionWorld = nodeVar18;
		normalViewGeometry = normalize( v_normalViewGeometry );
		NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
		normalView = NORMAL_normalView;
		normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
		let nodeConst0 = ( render.nodeUniform13 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform16 ) ) ), 1.0 ) ).xyz;
		let nodeConst1 = abs( nodeConst0 );
		nodeVar23 = 1.0;
		nodeVar24 = max( max( nodeConst1.x, nodeConst1.y ), nodeConst1.z );

		if ( ( ( ( nodeVar24 - render.nodeUniform17 ) <= 0.0 ) && ( ( nodeVar24 - render.nodeUniform18 ) >= 0.0 ) ) ) {

			nodeVar25 = ( - nodeVar24 );
			nodeVar26 = ( ( ( render.nodeUniform18 + nodeVar25 ) * render.nodeUniform17 ) / ( ( render.nodeUniform17 - render.nodeUniform18 ) * nodeVar25 ) );
			nodeVar26 = ( nodeVar26 + render.nodeUniform19 );
			nodeVar27 = normalize( nodeConst0 );
			nodeVar29 = abs( nodeVar27 );

			if ( ( nodeVar29.x > nodeVar29.z ) ) {

				nodeVar28 = vec3<f32>( 0.0, 1.0, 0.0 );

			} else {

				nodeVar28 = vec3<f32>( 1.0, 0.0, 0.0 );

			}

			nodeVar30 = normalize( cross( nodeVar27, nodeVar28 ) );
			nodeVar31 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar32 = vogelDiskSample( 0, 5, nodeVar31 );
			nodeVar33 = cross( nodeVar27, nodeVar30 );
			nodeVar34 = ( render.nodeUniform21 / render.nodeUniform22.x );
			nodeVar35 = ( nodeVar27 + ( ( ( nodeVar30 * vec3<f32>( nodeVar32.x ) ) + ( nodeVar33 * vec3<f32>( nodeVar32.y ) ) ) * vec3<f32>( nodeVar34 ) ) );
			nodeVar36 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, vec3<f32>( nodeVar35.x, ( - nodeVar35.y ), nodeVar35.z ), nodeVar26 );
			nodeVar37 = vogelDiskSample( 1, 5, nodeVar31 );
			nodeVar38 = ( nodeVar27 + ( ( ( nodeVar30 * vec3<f32>( nodeVar37.x ) ) + ( nodeVar33 * vec3<f32>( nodeVar37.y ) ) ) * vec3<f32>( nodeVar34 ) ) );
			nodeVar39 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, vec3<f32>( nodeVar38.x, ( - nodeVar38.y ), nodeVar38.z ), nodeVar26 );
			nodeVar40 = vogelDiskSample( 2, 5, nodeVar31 );
			nodeVar41 = ( nodeVar27 + ( ( ( nodeVar30 * vec3<f32>( nodeVar40.x ) ) + ( nodeVar33 * vec3<f32>( nodeVar40.y ) ) ) * vec3<f32>( nodeVar34 ) ) );
			nodeVar42 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, vec3<f32>( nodeVar41.x, ( - nodeVar41.y ), nodeVar41.z ), nodeVar26 );
			nodeVar43 = vogelDiskSample( 3, 5, nodeVar31 );
			nodeVar44 = ( nodeVar27 + ( ( ( nodeVar30 * vec3<f32>( nodeVar43.x ) ) + ( nodeVar33 * vec3<f32>( nodeVar43.y ) ) ) * vec3<f32>( nodeVar34 ) ) );
			nodeVar45 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, vec3<f32>( nodeVar44.x, ( - nodeVar44.y ), nodeVar44.z ), nodeVar26 );
			nodeVar46 = vogelDiskSample( 4, 5, nodeVar31 );
			nodeVar47 = ( nodeVar27 + ( ( ( nodeVar30 * vec3<f32>( nodeVar46.x ) ) + ( nodeVar33 * vec3<f32>( nodeVar46.y ) ) ) * vec3<f32>( nodeVar34 ) ) );
			nodeVar48 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, vec3<f32>( nodeVar47.x, ( - nodeVar47.y ), nodeVar47.z ), nodeVar26 );
			nodeVar23 = ( ( ( ( ( nodeVar36 + nodeVar39 ) + nodeVar42 ) + nodeVar45 ) + nodeVar48 ) * 0.2 );
			

		}

		nodeVar49 = mix( 1.0, nodeVar23, render.nodeUniform23 );

		if ( ( render.nodeUniform24 > 0.0 ) ) {

			nodeVar51 = length( ( render.nodeUniform25 - ( render.cameraViewMatrix * vec4<f32>( nodeVar18, 1.0 ) ).xyz ) );
			nodeVar52 = ( nodeVar51 / render.nodeUniform24 );
			nodeVar53 = clamp( ( 1.0 - ( ( ( nodeVar52 * nodeVar52 ) * nodeVar52 ) * nodeVar52 ) ), 0.0, 1.0 );
			nodeVar50 = ( ( 1.0 / max( pow( nodeVar51, render.nodeUniform26 ), 0.01 ) ) * ( nodeVar53 * nodeVar53 ) );

		} else {

			nodeVar50 = ( 1.0 / max( pow( length( ( render.nodeUniform25 - ( render.cameraViewMatrix * vec4<f32>( nodeVar18, 1.0 ) ).xyz ) ), render.nodeUniform26 ), 0.01 ) );

		}

		nodeVar54 = ( ( render.nodeUniform12 * vec3<f32>( nodeVar49 ) ) * vec3<f32>( nodeVar50 ) );
		nodeVar54 = ( nodeVar54 * vec3<f32>( nodeVar49 ) );

		if ( all( ( vec3<f32>( nodeVar21 ) >= nodeVar17 ) ) ) {

			nodeVar22 = ( nodeVar22 + nodeVar54 );
			

		}

		nodeVar56 = ( render.nodeUniform27 * vec4<f32>( nodeVar18, 1.0 ) );
		nodeVar57 = ( nodeVar56.xyz / vec3<f32>( nodeVar56.w ) );

		if ( all( ( abs( ( ( nodeVar57 * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ) ) < vec3<f32>( 1.0 ) ) ) ) {

			shadowPositionWorld = nodeVar18;
			nodeVar59 = ( render.nodeUniform27 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform29 ) ) ), 1.0 ) );
			nodeVar60 = ( nodeVar59.xyz / vec3<f32>( nodeVar59.w ) );
			nodeVar61 = vec3<f32>( nodeVar60.x, ( 1.0 - nodeVar60.y ), ( nodeVar60.z + render.nodeUniform30 ) );

			if ( ( ( ( ( ( nodeVar61.x >= 0.0 ) && ( nodeVar61.x <= 1.0 ) ) && ( nodeVar61.y >= 0.0 ) ) && ( nodeVar61.y <= 1.0 ) ) && ( nodeVar61.z <= 1.0 ) ) ) {

				nodeVar62 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
				nodeVar63 = ( render.nodeUniform32 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform33 ).x );
				nodeVar64 = ( nodeVar61.xy + ( vogelDiskSample( 0, 5, nodeVar62 ) * vec2<f32>( nodeVar63 ) ) );
				nodeVar65 = textureSampleCompare( nodeUniform31, nodeUniform31_sampler, nodeVar64, nodeVar61.z );
				nodeVar66 = ( nodeVar61.xy + ( vogelDiskSample( 1, 5, nodeVar62 ) * vec2<f32>( nodeVar63 ) ) );
				nodeVar67 = textureSampleCompare( nodeUniform31, nodeUniform31_sampler, nodeVar66, nodeVar61.z );
				nodeVar68 = ( nodeVar61.xy + ( vogelDiskSample( 2, 5, nodeVar62 ) * vec2<f32>( nodeVar63 ) ) );
				nodeVar69 = textureSampleCompare( nodeUniform31, nodeUniform31_sampler, nodeVar68, nodeVar61.z );
				nodeVar70 = ( nodeVar61.xy + ( vogelDiskSample( 3, 5, nodeVar62 ) * vec2<f32>( nodeVar63 ) ) );
				nodeVar71 = textureSampleCompare( nodeUniform31, nodeUniform31_sampler, nodeVar70, nodeVar61.z );
				nodeVar72 = ( nodeVar61.xy + ( vogelDiskSample( 4, 5, nodeVar62 ) * vec2<f32>( nodeVar63 ) ) );
				nodeVar73 = textureSampleCompare( nodeUniform31, nodeUniform31_sampler, nodeVar72, nodeVar61.z );
				nodeVar58 = ( ( ( ( ( nodeVar65 + nodeVar67 ) + nodeVar69 ) + nodeVar71 ) + nodeVar73 ) * 0.2 );

			} else {

				nodeVar58 = 1.0;

			}

			nodeVar74 = mix( 1.0, nodeVar58, render.nodeUniform34 );
			nodeVar75 = ( render.nodeUniform37 - ( render.cameraViewMatrix * vec4<f32>( nodeVar18, 1.0 ) ).xyz );

			if ( ( render.nodeUniform40 > 0.0 ) ) {

				nodeVar77 = length( nodeVar75 );
				nodeVar78 = ( nodeVar77 / render.nodeUniform40 );
				nodeVar79 = clamp( ( 1.0 - ( ( ( nodeVar78 * nodeVar78 ) * nodeVar78 ) * nodeVar78 ) ), 0.0, 1.0 );
				nodeVar76 = ( ( 1.0 / max( pow( nodeVar77, render.nodeUniform41 ), 0.01 ) ) * ( nodeVar79 * nodeVar79 ) );

			} else {

				nodeVar76 = ( 1.0 / max( pow( length( nodeVar75 ), render.nodeUniform41 ), 0.01 ) );

			}

			nodeVar80 = textureSample( nodeUniform42, nodeUniform42_sampler, nodeVar57.xy );
			nodeVar55 = ( vec4<f32>( ( ( ( render.nodeUniform28 * vec3<f32>( nodeVar74 ) ) * vec3<f32>( smoothstep( render.nodeUniform35, render.nodeUniform36, dot( normalize( nodeVar75 ), normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform38 - render.nodeUniform39 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( nodeVar76 ) ), 1.0 ) * nodeVar80 );

		} else {

			shadowPositionWorld = nodeVar18;
			nodeVar74 = mix( 1.0, nodeVar58, render.nodeUniform34 );
			nodeVar74 = mix( 1.0, nodeVar58, render.nodeUniform34 );
			nodeVar81 = ( render.nodeUniform37 - ( render.cameraViewMatrix * vec4<f32>( nodeVar18, 1.0 ) ).xyz );

			if ( ( render.nodeUniform40 > 0.0 ) ) {

				nodeVar83 = length( nodeVar81 );
				nodeVar84 = ( nodeVar83 / render.nodeUniform40 );
				nodeVar85 = clamp( ( 1.0 - ( ( ( nodeVar84 * nodeVar84 ) * nodeVar84 ) * nodeVar84 ) ), 0.0, 1.0 );
				nodeVar82 = ( ( 1.0 / max( pow( nodeVar83, render.nodeUniform41 ), 0.01 ) ) * ( nodeVar85 * nodeVar85 ) );

			} else {

				nodeVar82 = ( 1.0 / max( pow( length( nodeVar81 ), render.nodeUniform41 ), 0.01 ) );

			}

			nodeVar55 = vec4<f32>( ( ( ( render.nodeUniform28 * vec3<f32>( nodeVar74 ) ) * vec3<f32>( smoothstep( render.nodeUniform35, render.nodeUniform36, dot( normalize( nodeVar81 ), normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform38 - render.nodeUniform39 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( nodeVar82 ) ), 1.0 );

		}

		nodeVar86 = nodeVar55.xyz;
		shadowPositionWorld = nodeVar18;
		nodeVar86 = ( nodeVar86 * vec3<f32>( nodeVar74 ) );

		if ( all( ( vec3<f32>( nodeVar21 ) >= nodeVar17 ) ) ) {

			nodeVar22 = ( nodeVar22 + nodeVar86 );
			

		}

		nodeVar87 = vec3<f32>( object.nodeUniform44, 0.0, ( object.nodeUniform44 * 0.3 ) );
		nodeVar88 = textureSampleLevel( nodeUniform43, nodeUniform43_sampler, tsl_mod_vec3( ( ( nodeVar18 + ( nodeVar87 * vec3<f32>( 1.0 ) ) ) * vec3<f32>( 0.1 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar89 = textureSampleLevel( nodeUniform43, nodeUniform43_sampler, tsl_mod_vec3( ( ( nodeVar18 + ( nodeVar87 * vec3<f32>( 1.0 ) ) ) * vec3<f32>( 0.05 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar90 = textureSampleLevel( nodeUniform43, nodeUniform43_sampler, tsl_mod_vec3( ( ( nodeVar18 + ( nodeVar87 * vec3<f32>( 2.0 ) ) ) * vec3<f32>( 0.02 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar22 = ( nodeVar22 * vec3<f32>( mix( 1.0, ( ( ( nodeVar88 + 0.5 ) * ( nodeVar89 + 0.5 ) ) * ( nodeVar90 + 0.5 ) ), object.nodeUniform45 ) ) );
		nodeVar91 = ( nodeVar22 * vec3<f32>( 0.01 ) );

		if ( nodeVar6 ) {

			nodeVar92 = ( nodeVar92 + ( ( nodeVar91 * nodeVar14 ) * vec3<f32>( nodeVar10 ) ) );
			

		} else {

			nodeVar92 = ( ( nodeVar92 * exp( ( ( - ( nodeVar22 * vec3<f32>( 0.01 ) ) ) * vec3<f32>( nodeVar10 ) ) ) ) + ( nodeVar91 * vec3<f32>( nodeVar10 ) ) );
			

		}

		nodeVar14 = ( nodeVar14 * exp( ( ( - ( nodeVar22 * vec3<f32>( 0.01 ) ) ) * vec3<f32>( nodeVar10 ) ) ) );
		nodeVar13 = ( nodeVar13 + nodeVar10 );

	}

	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar93 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar93;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar94 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar94;
	nodeVar95 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar95;
	outgoingLight = nodeVar92;
	Output = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	output.m0 = Output;
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform47 );
	nodeVar96 = ( ( object.nodeUniform46 * modelViewMatrix ) * vec4<f32>( positionLocal, 1.0 ) );
	nodeVar97 = ( ( render.nodeUniform48 * ( object.nodeUniform49 * object.nodeUniform50 ) ) * vec4<f32>( positionPrevious, 1.0 ) );
	nodeVar98 = ( ( nodeVar96.xy / vec2<f32>( nodeVar96.w ) ) - ( nodeVar97.xy / vec2<f32>( nodeVar97.w ) ) );
	output.m1 = vec4<f32>( vec3<f32>( nodeVar98, 0.0 ), 1.0 );

	// result

	return output;

}
