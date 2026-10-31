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
@binding( 3 ) @group( 1 ) var nodeUniform9 : texture_depth_2d;
@binding( 4 ) @group( 1 ) var nodeUniform24_sampler : sampler_comparison;
@binding( 5 ) @group( 1 ) var nodeUniform24 : texture_depth_cube;
@binding( 6 ) @group( 1 ) var nodeUniform33_sampler : sampler_comparison;
@binding( 7 ) @group( 1 ) var nodeUniform33 : texture_depth_2d;
@binding( 8 ) @group( 1 ) var nodeUniform44_sampler : sampler;
@binding( 9 ) @group( 1 ) var nodeUniform44 : texture_2d<f32>;
@binding( 10 ) @group( 1 ) var nodeUniform45_sampler : sampler;
@binding( 11 ) @group( 1 ) var nodeUniform45 : texture_3d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : i32,
	nodeUniform14 : mat3x3<f32>,
	nodeUniform47 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraNear : f32,
	cameraFar : f32,
	nodeUniform46 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform27 : f32,
	nodeUniform28 : f32,
	nodeUniform37 : f32,
	nodeUniform38 : f32,
	nodeUniform42 : f32,
	nodeUniform43 : f32,
	nodeUniform30 : vec3<f32>,
	nodeUniform40 : vec3<f32>,
	nodeUniform41 : vec3<f32>,
	nodeUniform29 : mat4x4<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform39 : vec3<f32>,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform15 : f32,
	nodeUniform25 : f32,
	nodeUniform31 : f32,
	nodeUniform32 : f32,
	nodeUniform34 : f32,
	nodeUniform35 : vec2<f32>,
	nodeUniform36 : f32,
	cameraPosition : vec3<f32>,
	nodeUniform10 : vec2<f32>,
	nodeUniform17 : f32,
	nodeUniform16 : f32,
	nodeUniform18 : f32,
	nodeUniform19 : f32,
	nodeUniform20 : vec2<f32>,
	nodeUniform21 : f32,
	nodeUniform22 : f32,
	nodeUniform23 : f32
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
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : vec4<f32>;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
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
	nodeVar7 = textureLoad( nodeUniform5, ( vec2<i32>( fragCoord.xy ) % vec2<i32>( 16 ) ), u32( 0u ) );
	nodeVar5 = ( vec4<f32>( nodeVar5 ) + ( nodeVar7 * vec4<f32>( nodeVar3 ) ) ).x;

	for ( var i : i32 = 0; i < object.nodeUniform4; i ++ ) {

		let nodeConst1 = ( nodeVar0 + ( nodeVar4 * vec3<f32>( nodeVar5 ) ) );
		nodeVar8 = ( ( ( ( render.cameraNear * render.cameraFar ) / ( ( ( render.cameraFar - render.cameraNear ) * ( ( ( render.cameraNear + ( render.cameraViewMatrix * vec4<f32>( nodeConst1, 1.0 ) ).xyz.z ) * render.cameraFar ) / ( ( render.cameraFar - render.cameraNear ) * ( render.cameraViewMatrix * vec4<f32>( nodeConst1, 1.0 ) ).xyz.z ) ) ) - render.cameraFar ) ) + render.cameraNear ) / ( render.cameraNear - render.cameraFar ) );
		nodeVar10 = textureDimensions( nodeUniform9, u32( 0 ) );
		nodeVar9 = textureLoad( nodeUniform9, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( fragCoord.xy / render.nodeUniform10 ) ) * vec2<f32>( nodeVar10 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar10 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		nodeVar11 = ( ( ( ( render.cameraNear * render.cameraFar ) / ( ( ( render.cameraFar - render.cameraNear ) * nodeVar9 ) - render.cameraFar ) ) + render.cameraNear ) / ( render.cameraNear - render.cameraFar ) );
		nodeVar12 = vec3<f32>( 0.0 );
		shadowPositionWorld = nodeConst1;
		normalViewGeometry = normalize( v_normalViewGeometry );
		NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
		normalView = NORMAL_normalView;
		normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
		let nodeConst2 = ( render.nodeUniform12 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform15 ) ) ), 1.0 ) ).xyz;
		let nodeConst3 = abs( nodeConst2 );
		nodeVar13 = 1.0;
		let nodeConst4 = max( max( nodeConst3.x, nodeConst3.y ), nodeConst3.z );

		if ( ( ( ( nodeConst4 - render.nodeUniform16 ) <= 0.0 ) && ( ( nodeConst4 - render.nodeUniform17 ) >= 0.0 ) ) ) {

			let nodeConst5 = ( render.nodeUniform16 - render.nodeUniform17 );
			let nodeConst6 = ( ( render.nodeUniform16 / nodeConst5 ) + render.nodeUniform18 );
			let nodeConst7 = length( nodeConst2 );
			let nodeConst8 = ( - ( ( render.nodeUniform17 * render.nodeUniform16 ) / ( nodeConst5 * nodeConst7 ) ) );
			let nodeConst9 = ( render.nodeUniform19 / render.nodeUniform20.x );
			let nodeConst10 = normalize( nodeConst2 );
			let nodeConst11 = abs( nodeConst10 );

			if ( ( nodeConst11.x > nodeConst11.z ) ) {

				nodeVar14 = vec3<f32>( 0.0, 1.0, 0.0 );

			} else {

				nodeVar14 = vec3<f32>( 1.0, 0.0, 0.0 );

			}

			let nodeConst12 = normalize( cross( nodeConst10, nodeVar14 ) );
			let nodeConst13 = cross( nodeConst10, nodeConst12 );
			let nodeConst14 = ( fn24( fragCoord.xy ) * 6.28318530718 );
			let nodeConst15 = ( render.nodeUniform21 - render.nodeUniform22 );
			let nodeConst16 = ( ( render.nodeUniform21 / nodeConst15 ) + render.nodeUniform23 );
			let nodeConst17 = ( - ( ( render.nodeUniform22 * render.nodeUniform21 ) / ( nodeConst15 * nodeConst7 ) ) );
			nodeVar15 = 0.0;

			for ( var i : i32 = 0; i < 5; i ++ ) {

				let nodeConst18 = fn25( i, 5, nodeConst14 );
				let nodeConst19 = ( nodeConst10 + ( ( ( nodeConst12 * vec3<f32>( nodeConst18.x ) ) + ( nodeConst13 * vec3<f32>( nodeConst18.y ) ) ) * vec3<f32>( nodeConst9 ) ) );
				let nodeConst20 = abs( nodeConst19 );
				let nodeConst21 = max( max( nodeConst20.x, nodeConst20.y ), nodeConst20.z );
				nodeVar16 = textureSampleCompare( nodeUniform24, nodeUniform24_sampler, vec3<f32>( ( - nodeConst19.x ), nodeConst19.yz ), ( nodeConst16 + ( nodeConst17 / nodeConst21 ) ) );
				nodeVar15 = ( nodeVar15 + nodeVar16 );

			}

			nodeVar13 = ( nodeVar15 * 0.2 );
			

		}

		nodeVar17 = mix( 1.0, nodeVar13, render.nodeUniform25 );
		nodeVar18 = ( ( render.nodeUniform11 * vec3<f32>( nodeVar17 ) ) * vec3<f32>( fn44( length( ( render.nodeUniform26 - ( render.cameraViewMatrix * vec4<f32>( nodeConst1, 1.0 ) ).xyz ) ), render.nodeUniform27, render.nodeUniform28 ) ) );
		nodeVar18 = ( nodeVar18 * vec3<f32>( nodeVar17 ) );

		if ( ( nodeVar11 >= nodeVar8 ) ) {

			nodeVar12 = ( nodeVar12 + nodeVar18 );
			

		}

		let nodeConst22 = ( render.nodeUniform29 * vec4<f32>( nodeConst1, 1.0 ) );
		let nodeConst23 = ( nodeConst22.xyz / vec3<f32>( nodeConst22.w ) );

		if ( all( ( abs( ( ( nodeConst23 * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ) ) < vec3<f32>( 1.0 ) ) ) ) {

			shadowPositionWorld = nodeConst1;
			let nodeConst24 = ( render.nodeUniform29 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform31 ) ) ), 1.0 ) );
			let nodeConst25 = ( nodeConst24.xyz / vec3<f32>( nodeConst24.w ) );
			let nodeConst26 = vec3<f32>( nodeConst25.x, ( 1.0 - nodeConst25.y ), ( nodeConst25.z + render.nodeUniform32 ) );

			if ( ( ( ( ( ( nodeConst26.x >= 0.0 ) && ( nodeConst26.x <= 1.0 ) ) && ( nodeConst26.y >= 0.0 ) ) && ( nodeConst26.y <= 1.0 ) ) && ( nodeConst26.z <= 1.0 ) ) ) {

				let nodeConst27 = ( fn24( fragCoord.xy ) * 6.28318530718 );
				let nodeConst28 = ( vec2<f32>( render.nodeUniform34 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform35 ) );
				let nodeConst29 = ( nodeConst26.xy + ( fn25( 0, 5, nodeConst27 ) * nodeConst28 ) );
				nodeVar21 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst29, nodeConst26.z );
				let nodeConst30 = ( nodeConst26.xy + ( fn25( 1, 5, nodeConst27 ) * nodeConst28 ) );
				nodeVar22 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst30, nodeConst26.z );
				let nodeConst31 = ( nodeConst26.xy + ( fn25( 2, 5, nodeConst27 ) * nodeConst28 ) );
				nodeVar23 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst31, nodeConst26.z );
				let nodeConst32 = ( nodeConst26.xy + ( fn25( 3, 5, nodeConst27 ) * nodeConst28 ) );
				nodeVar24 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst32, nodeConst26.z );
				let nodeConst33 = ( nodeConst26.xy + ( fn25( 4, 5, nodeConst27 ) * nodeConst28 ) );
				nodeVar25 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst33, nodeConst26.z );
				nodeVar20 = ( ( ( ( ( nodeVar21 + nodeVar22 ) + nodeVar23 ) + nodeVar24 ) + nodeVar25 ) * 0.2 );

			} else {

				nodeVar20 = 1.0;

			}

			nodeVar26 = mix( 1.0, nodeVar20, render.nodeUniform36 );
			let nodeConst34 = ( render.nodeUniform39 - ( render.cameraViewMatrix * vec4<f32>( nodeConst1, 1.0 ) ).xyz );
			let nodeConst35 = ( ( ( render.nodeUniform30 * vec3<f32>( nodeVar26 ) ) * vec3<f32>( smoothstep( render.nodeUniform37, render.nodeUniform38, dot( normalize( nodeConst34 ), normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform40 - render.nodeUniform41 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( fn44( length( nodeConst34 ), render.nodeUniform42, render.nodeUniform43 ) ) );
			nodeVar27 = textureSample( nodeUniform44, nodeUniform44_sampler, nodeConst23.xy );
			nodeVar19 = ( vec4<f32>( nodeConst35, 1.0 ) * nodeVar27 );

		} else {

			shadowPositionWorld = nodeConst1;
			let nodeConst24 = ( render.nodeUniform29 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform31 ) ) ), 1.0 ) );
			let nodeConst25 = ( nodeConst24.xyz / vec3<f32>( nodeConst24.w ) );
			let nodeConst26 = vec3<f32>( nodeConst25.x, ( 1.0 - nodeConst25.y ), ( nodeConst25.z + render.nodeUniform32 ) );

			if ( ( ( ( ( ( nodeConst26.x >= 0.0 ) && ( nodeConst26.x <= 1.0 ) ) && ( nodeConst26.y >= 0.0 ) ) && ( nodeConst26.y <= 1.0 ) ) && ( nodeConst26.z <= 1.0 ) ) ) {

				let nodeConst27 = ( fn24( fragCoord.xy ) * 6.28318530718 );
				let nodeConst28 = ( vec2<f32>( render.nodeUniform34 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform35 ) );
				let nodeConst29 = ( nodeConst26.xy + ( fn25( 0, 5, nodeConst27 ) * nodeConst28 ) );
				nodeVar21 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst29, nodeConst26.z );
				let nodeConst30 = ( nodeConst26.xy + ( fn25( 1, 5, nodeConst27 ) * nodeConst28 ) );
				nodeVar22 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst30, nodeConst26.z );
				let nodeConst31 = ( nodeConst26.xy + ( fn25( 2, 5, nodeConst27 ) * nodeConst28 ) );
				nodeVar23 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst31, nodeConst26.z );
				let nodeConst32 = ( nodeConst26.xy + ( fn25( 3, 5, nodeConst27 ) * nodeConst28 ) );
				nodeVar24 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst32, nodeConst26.z );
				let nodeConst33 = ( nodeConst26.xy + ( fn25( 4, 5, nodeConst27 ) * nodeConst28 ) );
				nodeVar25 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst33, nodeConst26.z );
				nodeVar28 = ( ( ( ( ( nodeVar21 + nodeVar22 ) + nodeVar23 ) + nodeVar24 ) + nodeVar25 ) * 0.2 );

			} else {

				nodeVar28 = 1.0;

			}

			nodeVar26 = mix( 1.0, nodeVar28, render.nodeUniform36 );
			let nodeConst34 = ( render.nodeUniform39 - ( render.cameraViewMatrix * vec4<f32>( nodeConst1, 1.0 ) ).xyz );
			let nodeConst35 = ( ( ( render.nodeUniform30 * vec3<f32>( nodeVar26 ) ) * vec3<f32>( smoothstep( render.nodeUniform37, render.nodeUniform38, dot( normalize( nodeConst34 ), normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform40 - render.nodeUniform41 ), 0.0 ) ).xyz ) ) ) ) ) * vec3<f32>( fn44( length( nodeConst34 ), render.nodeUniform42, render.nodeUniform43 ) ) );
			nodeVar19 = vec4<f32>( nodeConst35, 1.0 );

		}

		nodeVar29 = nodeVar19.xyz;
		shadowPositionWorld = nodeConst1;
		let nodeConst24 = ( render.nodeUniform29 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform31 ) ) ), 1.0 ) );
		let nodeConst25 = ( nodeConst24.xyz / vec3<f32>( nodeConst24.w ) );
		let nodeConst26 = vec3<f32>( nodeConst25.x, ( 1.0 - nodeConst25.y ), ( nodeConst25.z + render.nodeUniform32 ) );

		if ( ( ( ( ( ( nodeConst26.x >= 0.0 ) && ( nodeConst26.x <= 1.0 ) ) && ( nodeConst26.y >= 0.0 ) ) && ( nodeConst26.y <= 1.0 ) ) && ( nodeConst26.z <= 1.0 ) ) ) {

			let nodeConst27 = ( fn24( fragCoord.xy ) * 6.28318530718 );
			let nodeConst28 = ( vec2<f32>( render.nodeUniform34 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform35 ) );
			let nodeConst29 = ( nodeConst26.xy + ( fn25( 0, 5, nodeConst27 ) * nodeConst28 ) );
			nodeVar21 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst29, nodeConst26.z );
			let nodeConst30 = ( nodeConst26.xy + ( fn25( 1, 5, nodeConst27 ) * nodeConst28 ) );
			nodeVar22 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst30, nodeConst26.z );
			let nodeConst31 = ( nodeConst26.xy + ( fn25( 2, 5, nodeConst27 ) * nodeConst28 ) );
			nodeVar23 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst31, nodeConst26.z );
			let nodeConst32 = ( nodeConst26.xy + ( fn25( 3, 5, nodeConst27 ) * nodeConst28 ) );
			nodeVar24 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst32, nodeConst26.z );
			let nodeConst33 = ( nodeConst26.xy + ( fn25( 4, 5, nodeConst27 ) * nodeConst28 ) );
			nodeVar25 = textureSampleCompare( nodeUniform33, nodeUniform33_sampler, nodeConst33, nodeConst26.z );
			nodeVar30 = ( ( ( ( ( nodeVar21 + nodeVar22 ) + nodeVar23 ) + nodeVar24 ) + nodeVar25 ) * 0.2 );

		} else {

			nodeVar30 = 1.0;

		}

		nodeVar26 = mix( 1.0, nodeVar30, render.nodeUniform36 );
		nodeVar29 = ( nodeVar29 * vec3<f32>( nodeVar26 ) );

		if ( ( nodeVar11 >= nodeVar8 ) ) {

			nodeVar12 = ( nodeVar12 + nodeVar29 );
			

		}

		let nodeConst36 = vec3<f32>( render.nodeUniform46, 0.0, ( render.nodeUniform46 * 0.3 ) );
		nodeVar31 = textureSampleLevel( nodeUniform45, nodeUniform45_sampler, tsl_mod_vec3( ( ( nodeConst1 + ( nodeConst36 * vec3<f32>( 1.0 ) ) ) * vec3<f32>( 0.1 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar32 = textureSampleLevel( nodeUniform45, nodeUniform45_sampler, tsl_mod_vec3( ( ( nodeConst1 + ( nodeConst36 * vec3<f32>( 1.0 ) ) ) * vec3<f32>( 0.05 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar33 = textureSampleLevel( nodeUniform45, nodeUniform45_sampler, tsl_mod_vec3( ( ( nodeConst1 + ( nodeConst36 * vec3<f32>( 2.0 ) ) ) * vec3<f32>( 0.02 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar12 = ( nodeVar12 * vec3<f32>( mix( 1.0, ( ( ( nodeVar31 + 0.5 ) * ( nodeVar32 + 0.5 ) ) * ( nodeVar33 + 0.5 ) ), object.nodeUniform47 ) ) );
		nodeVar34 = ( nodeVar12 * vec3<f32>( 0.01 ) );

		if ( nodeVar2 ) {

			nodeVar35 = ( nodeVar35 + ( ( nodeVar34 * nodeVar6 ) * vec3<f32>( nodeVar3 ) ) );
			

		} else {

			let nodeConst37 = exp( ( ( - ( nodeVar12 * vec3<f32>( 0.01 ) ) ) * vec3<f32>( nodeVar3 ) ) );
			nodeVar35 = ( ( nodeVar35 * nodeConst37 ) + ( nodeVar34 * vec3<f32>( nodeVar3 ) ) );
			

		}

		let nodeConst37 = exp( ( ( - ( nodeVar12 * vec3<f32>( 0.01 ) ) ) * vec3<f32>( nodeVar3 ) ) );
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
	outgoingLight = nodeVar35;
	let nodeConst38 = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst38;

	// result

	output.color = nodeConst38;

	return output;

}
