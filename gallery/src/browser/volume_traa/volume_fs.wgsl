// Three.js r187 - Node System

// global
diagnostic( off, derivative_uniformity );


// directives


// structs

struct OutputType {
	@location( 0 ) m0 : vec4<f32>,
	@location( 1 ) m1 : vec4<f32>,
	
};
var<private> output : OutputType;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform10 : texture_depth_2d;
@binding( 2 ) @group( 1 ) var nodeUniform25_sampler : sampler_comparison;
@binding( 3 ) @group( 1 ) var nodeUniform25 : texture_depth_cube;
@binding( 4 ) @group( 1 ) var nodeUniform34_sampler : sampler_comparison;
@binding( 5 ) @group( 1 ) var nodeUniform34 : texture_depth_2d;
@binding( 6 ) @group( 1 ) var nodeUniform45_sampler : sampler;
@binding( 7 ) @group( 1 ) var nodeUniform45 : texture_2d<f32>;
@binding( 8 ) @group( 1 ) var nodeUniform46_sampler : sampler;
@binding( 9 ) @group( 1 ) var nodeUniform46 : texture_3d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : i32,
	nodeUniform5 : f32,
	nodeUniform6 : f32,
	nodeUniform15 : mat3x3<f32>,
	nodeUniform47 : f32,
	nodeUniform48 : f32,
	nodeUniform50 : mat4x4<f32>,
	nodeUniform53 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraNear : f32,
	cameraFar : f32,
	nodeUniform49 : mat4x4<f32>,
	nodeUniform51 : mat4x4<f32>,
	nodeUniform52 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform28 : f32,
	nodeUniform29 : f32,
	nodeUniform38 : f32,
	nodeUniform39 : f32,
	nodeUniform43 : f32,
	nodeUniform44 : f32,
	nodeUniform31 : vec3<f32>,
	nodeUniform41 : vec3<f32>,
	nodeUniform42 : vec3<f32>,
	nodeUniform30 : mat4x4<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform40 : vec3<f32>,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform16 : f32,
	nodeUniform26 : f32,
	nodeUniform32 : f32,
	nodeUniform33 : f32,
	nodeUniform35 : f32,
	nodeUniform36 : vec2<f32>,
	nodeUniform37 : f32,
	cameraPosition : vec3<f32>,
	nodeUniform11 : vec2<f32>,
	nodeUniform18 : f32,
	nodeUniform17 : f32,
	nodeUniform19 : f32,
	nodeUniform20 : f32,
	nodeUniform21 : vec2<f32>,
	nodeUniform22 : f32,
	nodeUniform23 : f32,
	nodeUniform24 : f32
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
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec2<u32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec4<f32>;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;

// codes
fn fn24 ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

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
fn main( @location( 0 ) positionLocal : vec3<f32>,
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) positionPrevious : vec3<f32>,
	@location( 3 ) v_normalViewGeometry : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputType {

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
	nodeVar5 = ( nodeVar5 + ( fract( ( fn24( ( fragCoord.xy + ( vec2<f32>( object.nodeUniform5, object.nodeUniform6 ) * vec2<f32>( 100.0 ) ) ) ) + object.nodeUniform5 ) ) * nodeVar3 ) );

	for ( var i : i32 = 0; i < object.nodeUniform4; i ++ ) {

		let nodeConst1 = ( nodeVar0 + ( nodeVar4 * vec3<f32>( nodeVar5 ) ) );
		nodeVar7 = ( ( ( ( render.cameraNear * render.cameraFar ) / ( ( ( render.cameraFar - render.cameraNear ) * ( ( ( render.cameraNear + ( render.cameraViewMatrix * vec4<f32>( nodeConst1, 1.0 ) ).xyz.z ) * render.cameraFar ) / ( ( render.cameraFar - render.cameraNear ) * ( render.cameraViewMatrix * vec4<f32>( nodeConst1, 1.0 ) ).xyz.z ) ) ) - render.cameraFar ) ) + render.cameraNear ) / ( render.cameraNear - render.cameraFar ) );
		nodeVar9 = textureDimensions( nodeUniform10, u32( 0 ) );
		nodeVar8 = textureLoad( nodeUniform10, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( fragCoord.xy / render.nodeUniform11 ) ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		nodeVar10 = ( ( ( ( render.cameraNear * render.cameraFar ) / ( ( ( render.cameraFar - render.cameraNear ) * nodeVar8 ) - render.cameraFar ) ) + render.cameraNear ) / ( render.cameraNear - render.cameraFar ) );
		nodeVar11 = vec3<f32>( 0.0 );
		shadowPositionWorld = nodeConst1;
		normalViewGeometry = normalize( v_normalViewGeometry );
		NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
		normalView = NORMAL_normalView;
		normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
		let nodeConst2 = ( render.nodeUniform13 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform16 ) ) ), 1.0 ) ).xyz;
		let nodeConst3 = abs( nodeConst2 );
		nodeVar12 = 1.0;
		let nodeConst4 = max( max( nodeConst3.x, nodeConst3.y ), nodeConst3.z );

		if ( ( ( ( nodeConst4 - render.nodeUniform17 ) <= 0.0 ) && ( ( nodeConst4 - render.nodeUniform18 ) >= 0.0 ) ) ) {

			let nodeConst5 = ( render.nodeUniform17 - render.nodeUniform18 );
			let nodeConst6 = ( ( render.nodeUniform17 / nodeConst5 ) + render.nodeUniform19 );
			let nodeConst7 = length( nodeConst2 );
			let nodeConst8 = ( - ( ( render.nodeUniform18 * render.nodeUniform17 ) / ( nodeConst5 * nodeConst7 ) ) );
			let nodeConst9 = ( render.nodeUniform20 / render.nodeUniform21.x );
			let nodeConst10 = normalize( nodeConst2 );
			let nodeConst11 = abs( nodeConst10 );

			if ( ( nodeConst11.x > nodeConst11.z ) ) {

				nodeVar13 = vec3<f32>( 0.0, 1.0, 0.0 );

			} else {

				nodeVar13 = vec3<f32>( 1.0, 0.0, 0.0 );

			}

			let nodeConst12 = normalize( cross( nodeConst10, nodeVar13 ) );
			let nodeConst13 = cross( nodeConst10, nodeConst12 );
			let nodeConst14 = ( fn24( fragCoord.xy ) * 6.28318530718 );
			let nodeConst15 = ( render.nodeUniform22 - render.nodeUniform23 );
			let nodeConst16 = ( ( render.nodeUniform22 / nodeConst15 ) + render.nodeUniform24 );
			let nodeConst17 = ( - ( ( render.nodeUniform23 * render.nodeUniform22 ) / ( nodeConst15 * nodeConst7 ) ) );
			nodeVar14 = 0.0;

			for ( var i : i32 = 0; i < 5; i ++ ) {

				let nodeConst18 = fn25( i, 5, nodeConst14 );
				let nodeConst19 = ( nodeConst10 + ( ( ( nodeConst12 * vec3<f32>( nodeConst18.x ) ) + ( nodeConst13 * vec3<f32>( nodeConst18.y ) ) ) * vec3<f32>( nodeConst9 ) ) );
				let nodeConst20 = abs( nodeConst19 );
				let nodeConst21 = max( max( nodeConst20.x, nodeConst20.y ), nodeConst20.z );
				nodeVar15 = textureSampleCompare( nodeUniform25, nodeUniform25_sampler, vec3<f32>( ( - nodeConst19.x ), nodeConst19.yz ), ( nodeConst16 + ( nodeConst17 / nodeConst21 ) ) );
				nodeVar14 = ( nodeVar14 + nodeVar15 );

			}

			nodeVar12 = ( nodeVar14 * 0.2 );
			

		}

		nodeVar16 = mix( 1.0, nodeVar12, render.nodeUniform26 );
		nodeVar17 = ( ( render.nodeUniform12 * vec3<f32>( nodeVar16 ) ) * vec3<f32>( fn44( length( ( render.nodeUniform27 - ( render.cameraViewMatrix * vec4<f32>( nodeConst1, 1.0 ) ).xyz ) ), render.nodeUniform28, render.nodeUniform29 ) ) );
		nodeVar17 = ( nodeVar17 * vec3<f32>( nodeVar16 ) );

		if ( ( nodeVar10 >= nodeVar7 ) ) {

			nodeVar11 = ( nodeVar11 + nodeVar17 );
			

		}

		let nodeConst22 = ( render.nodeUniform30 * vec4<f32>( nodeConst1, 1.0 ) );
		let nodeConst23 = ( nodeConst22.xyz / vec3<f32>( nodeConst22.w ) );

		if ( all( ( abs( ( ( nodeConst23 * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ) ) < vec3<f32>( 1.0 ) ) ) ) {

			shadowPositionWorld = nodeConst1;
			let nodeConst24 = ( render.nodeUniform30 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform32 ) ) ), 1.0 ) );
			let nodeConst25 = ( nodeConst24.xyz / vec3<f32>( nodeConst24.w ) );
			let nodeConst26 = vec3<f32>( nodeConst25.x, ( 1.0 - nodeConst25.y ), ( nodeConst25.z + render.nodeUniform33 ) );

			if ( ( ( ( ( ( nodeConst26.x >= 0.0 ) && ( nodeConst26.x <= 1.0 ) ) && ( nodeConst26.y >= 0.0 ) ) && ( nodeConst26.y <= 1.0 ) ) && ( nodeConst26.z <= 1.0 ) ) ) {

				let nodeConst27 = ( fn24( fragCoord.xy ) * 6.28318530718 );
				let nodeConst28 = ( vec2<f32>( render.nodeUniform35 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform36 ) );
				let nodeConst29 = ( nodeConst26.xy + ( fn25( 0, 5, nodeConst27 ) * nodeConst28 ) );
				nodeVar20 = textureSampleCompare( nodeUniform34, nodeUniform34_sampler, nodeConst29, nodeConst26.z );
				let nodeConst30 = ( nodeConst26.xy + ( fn25( 1, 5, nodeConst27 ) * nodeConst28 ) );
				nodeVar21 = textureSampleCompare( nodeUniform34, nodeUniform34_sampler, nodeConst30, nodeConst26.z );
				let nodeConst31 = ( nodeConst26.xy + ( fn25( 2, 5, nodeConst27 ) * nodeConst28 ) );
				nodeVar22 = textureSampleCompare( nodeUniform34, nodeUniform34_sampler, nodeConst31, nodeConst26.z );
				let nodeConst32 = ( nodeConst26.xy + ( fn25( 3, 5, nodeConst27 ) * nodeConst28 ) );
				nodeVar23 = textureSampleCompare( nodeUniform34, nodeUniform34_sampler, nodeConst32, nodeConst26.z );
				let nodeConst33 = ( nodeConst26.xy + ( fn25( 4, 5, nodeConst27 ) * nodeConst28 ) );
				nodeVar24 = textureSampleCompare( nodeUniform34, nodeUniform34_sampler, nodeConst33, nodeConst26.z );
				nodeVar19 = ( ( ( ( ( nodeVar20 + nodeVar21 ) + nodeVar22 ) + nodeVar23 ) + nodeVar24 ) * 0.2 );

			} else {

				nodeVar19 = 1.0;

			}

			nodeVar25 = mix( 1.0, nodeVar19, render.nodeUniform37 );
			let nodeConst34 = ( render.nodeUniform40 - ( render.cameraViewMatrix * vec4<f32>( nodeConst1, 1.0 ) ).xyz );
			let nodeConst35 = ( ( ( render.nodeUniform31 * vec3<f32>( nodeVar25 ) ) * vec3<f32>( smoothstep( render.nodeUniform38, render.nodeUniform39, dot( normalize( nodeConst34 ), normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform41 - render.nodeUniform42 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( fn44( length( nodeConst34 ), render.nodeUniform43, render.nodeUniform44 ) ) );
			nodeVar26 = textureSample( nodeUniform45, nodeUniform45_sampler, nodeConst23.xy );
			nodeVar18 = ( vec4<f32>( nodeConst35, 1.0 ) * nodeVar26 );

		} else {

			shadowPositionWorld = nodeConst1;
			let nodeConst24 = ( render.nodeUniform30 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform32 ) ) ), 1.0 ) );
			let nodeConst25 = ( nodeConst24.xyz / vec3<f32>( nodeConst24.w ) );
			let nodeConst26 = vec3<f32>( nodeConst25.x, ( 1.0 - nodeConst25.y ), ( nodeConst25.z + render.nodeUniform33 ) );

			if ( ( ( ( ( ( nodeConst26.x >= 0.0 ) && ( nodeConst26.x <= 1.0 ) ) && ( nodeConst26.y >= 0.0 ) ) && ( nodeConst26.y <= 1.0 ) ) && ( nodeConst26.z <= 1.0 ) ) ) {

				let nodeConst27 = ( fn24( fragCoord.xy ) * 6.28318530718 );
				let nodeConst28 = ( vec2<f32>( render.nodeUniform35 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform36 ) );
				let nodeConst29 = ( nodeConst26.xy + ( fn25( 0, 5, nodeConst27 ) * nodeConst28 ) );
				nodeVar20 = textureSampleCompare( nodeUniform34, nodeUniform34_sampler, nodeConst29, nodeConst26.z );
				let nodeConst30 = ( nodeConst26.xy + ( fn25( 1, 5, nodeConst27 ) * nodeConst28 ) );
				nodeVar21 = textureSampleCompare( nodeUniform34, nodeUniform34_sampler, nodeConst30, nodeConst26.z );
				let nodeConst31 = ( nodeConst26.xy + ( fn25( 2, 5, nodeConst27 ) * nodeConst28 ) );
				nodeVar22 = textureSampleCompare( nodeUniform34, nodeUniform34_sampler, nodeConst31, nodeConst26.z );
				let nodeConst32 = ( nodeConst26.xy + ( fn25( 3, 5, nodeConst27 ) * nodeConst28 ) );
				nodeVar23 = textureSampleCompare( nodeUniform34, nodeUniform34_sampler, nodeConst32, nodeConst26.z );
				let nodeConst33 = ( nodeConst26.xy + ( fn25( 4, 5, nodeConst27 ) * nodeConst28 ) );
				nodeVar24 = textureSampleCompare( nodeUniform34, nodeUniform34_sampler, nodeConst33, nodeConst26.z );
				nodeVar27 = ( ( ( ( ( nodeVar20 + nodeVar21 ) + nodeVar22 ) + nodeVar23 ) + nodeVar24 ) * 0.2 );

			} else {

				nodeVar27 = 1.0;

			}

			nodeVar25 = mix( 1.0, nodeVar27, render.nodeUniform37 );
			let nodeConst34 = ( render.nodeUniform40 - ( render.cameraViewMatrix * vec4<f32>( nodeConst1, 1.0 ) ).xyz );
			let nodeConst35 = ( ( ( render.nodeUniform31 * vec3<f32>( nodeVar25 ) ) * vec3<f32>( smoothstep( render.nodeUniform38, render.nodeUniform39, dot( normalize( nodeConst34 ), normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform41 - render.nodeUniform42 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( fn44( length( nodeConst34 ), render.nodeUniform43, render.nodeUniform44 ) ) );
			nodeVar18 = vec4<f32>( nodeConst35, 1.0 );

		}

		nodeVar28 = nodeVar18.xyz;
		shadowPositionWorld = nodeConst1;
		let nodeConst24 = ( render.nodeUniform30 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform32 ) ) ), 1.0 ) );
		let nodeConst25 = ( nodeConst24.xyz / vec3<f32>( nodeConst24.w ) );
		let nodeConst26 = vec3<f32>( nodeConst25.x, ( 1.0 - nodeConst25.y ), ( nodeConst25.z + render.nodeUniform33 ) );

		if ( ( ( ( ( ( nodeConst26.x >= 0.0 ) && ( nodeConst26.x <= 1.0 ) ) && ( nodeConst26.y >= 0.0 ) ) && ( nodeConst26.y <= 1.0 ) ) && ( nodeConst26.z <= 1.0 ) ) ) {

			let nodeConst27 = ( fn24( fragCoord.xy ) * 6.28318530718 );
			let nodeConst28 = ( vec2<f32>( render.nodeUniform35 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform36 ) );
			let nodeConst29 = ( nodeConst26.xy + ( fn25( 0, 5, nodeConst27 ) * nodeConst28 ) );
			nodeVar20 = textureSampleCompare( nodeUniform34, nodeUniform34_sampler, nodeConst29, nodeConst26.z );
			let nodeConst30 = ( nodeConst26.xy + ( fn25( 1, 5, nodeConst27 ) * nodeConst28 ) );
			nodeVar21 = textureSampleCompare( nodeUniform34, nodeUniform34_sampler, nodeConst30, nodeConst26.z );
			let nodeConst31 = ( nodeConst26.xy + ( fn25( 2, 5, nodeConst27 ) * nodeConst28 ) );
			nodeVar22 = textureSampleCompare( nodeUniform34, nodeUniform34_sampler, nodeConst31, nodeConst26.z );
			let nodeConst32 = ( nodeConst26.xy + ( fn25( 3, 5, nodeConst27 ) * nodeConst28 ) );
			nodeVar23 = textureSampleCompare( nodeUniform34, nodeUniform34_sampler, nodeConst32, nodeConst26.z );
			let nodeConst33 = ( nodeConst26.xy + ( fn25( 4, 5, nodeConst27 ) * nodeConst28 ) );
			nodeVar24 = textureSampleCompare( nodeUniform34, nodeUniform34_sampler, nodeConst33, nodeConst26.z );
			nodeVar29 = ( ( ( ( ( nodeVar20 + nodeVar21 ) + nodeVar22 ) + nodeVar23 ) + nodeVar24 ) * 0.2 );

		} else {

			nodeVar29 = 1.0;

		}

		nodeVar25 = mix( 1.0, nodeVar29, render.nodeUniform37 );
		nodeVar28 = ( nodeVar28 * vec3<f32>( nodeVar25 ) );

		if ( ( nodeVar10 >= nodeVar7 ) ) {

			nodeVar11 = ( nodeVar11 + nodeVar28 );
			

		}

		let nodeConst36 = vec3<f32>( object.nodeUniform47, 0.0, ( object.nodeUniform47 * 0.3 ) );
		nodeVar30 = textureSampleLevel( nodeUniform46, nodeUniform46_sampler, tsl_mod_vec3( ( ( nodeConst1 + ( nodeConst36 * vec3<f32>( 1.0 ) ) ) * vec3<f32>( 0.1 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar31 = textureSampleLevel( nodeUniform46, nodeUniform46_sampler, tsl_mod_vec3( ( ( nodeConst1 + ( nodeConst36 * vec3<f32>( 1.0 ) ) ) * vec3<f32>( 0.05 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar32 = textureSampleLevel( nodeUniform46, nodeUniform46_sampler, tsl_mod_vec3( ( ( nodeConst1 + ( nodeConst36 * vec3<f32>( 2.0 ) ) ) * vec3<f32>( 0.02 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar11 = ( nodeVar11 * vec3<f32>( mix( 1.0, ( ( ( nodeVar30 + 0.5 ) * ( nodeVar31 + 0.5 ) ) * ( nodeVar32 + 0.5 ) ), object.nodeUniform48 ) ) );
		nodeVar33 = ( nodeVar11 * vec3<f32>( 0.01 ) );

		if ( nodeVar2 ) {

			nodeVar34 = ( nodeVar34 + ( ( nodeVar33 * nodeVar6 ) * vec3<f32>( nodeVar3 ) ) );
			

		} else {

			let nodeConst37 = exp( ( ( - ( nodeVar11 * vec3<f32>( 0.01 ) ) ) * vec3<f32>( nodeVar3 ) ) );
			nodeVar34 = ( ( nodeVar34 * nodeConst37 ) + ( nodeVar33 * vec3<f32>( nodeVar3 ) ) );
			

		}

		let nodeConst37 = exp( ( ( - ( nodeVar11 * vec3<f32>( 0.01 ) ) ) * vec3<f32>( nodeVar3 ) ) );
		nodeVar6 = ( nodeVar6 * nodeConst37 );
		nodeVar5 = ( nodeVar5 + nodeVar3 );

	}

	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar34;
	Output = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	output.m0 = Output;
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform50 );
	let nodeConst38 = ( ( render.nodeUniform49 * modelViewMatrix ) * vec4<f32>( positionLocal, 1.0 ) );
	let nodeConst39 = ( ( render.nodeUniform51 * ( render.nodeUniform52 * object.nodeUniform53 ) ) * vec4<f32>( positionPrevious, 1.0 ) );
	let nodeConst40 = ( ( nodeConst38.xy / vec2<f32>( nodeConst38.w ) ) - ( nodeConst39.xy / vec2<f32>( nodeConst39.w ) ) );
	output.m1 = vec4<f32>( vec3<f32>( nodeConst40, 0.0 ), 1.0 );

	// result

	return output;

}
