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
@binding( 4 ) @group( 1 ) var nodeUniform23_sampler : sampler;
@binding( 5 ) @group( 1 ) var nodeUniform23 : texture_3d<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : i32,
	nodeUniform25 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraNear : f32,
	cameraFar : f32,
	nodeUniform24 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform18 : vec3<f32>,
	nodeUniform17 : vec3<f32>,
	nodeUniform19 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform20 : vec3<f32>,
	cameraPosition : vec3<f32>,
	nodeUniform10 : vec2<f32>
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
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : vec4<f32>;

// codes
fn LTC_EdgeVectorFormFactor ( v1 : vec3<f32>, v2 : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;

	nodeVar0 = dot( v1, v2 );
	nodeVar1 = abs( nodeVar0 );
	nodeVar2 = ( ( ( ( nodeVar1 * 0.0145206 ) + 0.4965155 ) * nodeVar1 ) + 0.8543985 );
	nodeVar3 = ( ( ( nodeVar1 + 4.1616724 ) * nodeVar1 ) + 3.417594 );

	if ( ( nodeVar0 > 0.0 ) ) {

		nodeVar4 = ( nodeVar2 / nodeVar3 );

	} else {

		nodeVar4 = ( ( inverseSqrt( max( ( 1.0 - ( nodeVar0 * nodeVar0 ) ), 1e-7 ) ) * 0.5 ) - ( nodeVar2 / nodeVar3 ) );

	}


	return ( cross( v1, v2 ) * vec3<f32>( nodeVar4 ) );

}


fn LTC_ClippedSphereFormFactor ( f : vec3<f32> ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = length( f );

	return max( ( ( ( nodeVar0 * nodeVar0 ) + f.z ) / ( nodeVar0 + 1.0 ) ), 0.0 );

}


fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}

fn LTC_Evaluate_Volume ( P : vec3<f32>, p0 : vec3<f32>, p1 : vec3<f32>, p2 : vec3<f32>, p3 : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : vec3<f32>;
	var nodeVar2 : vec3<f32>;
	var nodeVar3 : vec3<f32>;
	var nodeVar4 : vec3<f32>;
	var nodeVar5 : vec3<f32>;
	var nodeVar6 : vec3<f32>;
	var nodeVar7 : vec3<f32>;

	nodeVar0 = ( p1 - p0 );
	nodeVar1 = ( p3 - p0 );
	nodeVar2 = vec3<f32>( 0.0, 0.0, 0.0 );

	if ( ( dot( cross( nodeVar0, nodeVar1 ), ( P - p0 ) ) >= 0.0 ) ) {

		nodeVar3 = normalize( ( p0 - P ) );
		nodeVar4 = normalize( ( p1 - P ) );
		nodeVar5 = normalize( ( p2 - P ) );
		nodeVar6 = normalize( ( p3 - P ) );
		nodeVar7 = vec3<f32>( 0.0, 0.0, 0.0 );
		nodeVar7 = ( nodeVar7 + LTC_EdgeVectorFormFactor( nodeVar3, nodeVar4 ) );
		nodeVar7 = ( nodeVar7 + LTC_EdgeVectorFormFactor( nodeVar4, nodeVar5 ) );
		nodeVar7 = ( nodeVar7 + LTC_EdgeVectorFormFactor( nodeVar5, nodeVar6 ) );
		nodeVar7 = ( nodeVar7 + LTC_EdgeVectorFormFactor( nodeVar6, nodeVar3 ) );
		nodeVar2 = vec3<f32>( LTC_ClippedSphereFormFactor( abs( nodeVar7 ) ) );
		

	}


	return nodeVar2;

}


fn tsl_mod_vec3( x : vec3f, y : vec3f ) -> vec3f { return x - y * floor( x / y ); }


@fragment
fn main( @location( 0 ) v_positionWorld : vec3<f32>,
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

		if ( all( ( vec3<f32>( nodeVar22 ) >= nodeVar18 ) ) ) {

			nodeVar23 = ( nodeVar23 + pow( ( render.nodeUniform11 * LTC_Evaluate_Volume( ( render.cameraViewMatrix * vec4<f32>( nodeVar19, 1.0 ) ).xyz, ( ( render.nodeUniform12 + render.nodeUniform13 ) - render.nodeUniform14 ), ( ( render.nodeUniform12 - render.nodeUniform13 ) - render.nodeUniform14 ), ( ( render.nodeUniform12 - render.nodeUniform13 ) + render.nodeUniform14 ), ( ( render.nodeUniform12 + render.nodeUniform13 ) + render.nodeUniform14 ) ) ), vec3<f32>( 1.5 ) ) );
			

		}


		if ( all( ( vec3<f32>( nodeVar22 ) >= nodeVar18 ) ) ) {

			nodeVar23 = ( nodeVar23 + pow( ( render.nodeUniform15 * LTC_Evaluate_Volume( ( render.cameraViewMatrix * vec4<f32>( nodeVar19, 1.0 ) ).xyz, ( ( render.nodeUniform16 + render.nodeUniform17 ) - render.nodeUniform18 ), ( ( render.nodeUniform16 - render.nodeUniform17 ) - render.nodeUniform18 ), ( ( render.nodeUniform16 - render.nodeUniform17 ) + render.nodeUniform18 ), ( ( render.nodeUniform16 + render.nodeUniform17 ) + render.nodeUniform18 ) ) ), vec3<f32>( 1.5 ) ) );
			

		}


		if ( all( ( vec3<f32>( nodeVar22 ) >= nodeVar18 ) ) ) {

			nodeVar23 = ( nodeVar23 + pow( ( render.nodeUniform19 * LTC_Evaluate_Volume( ( render.cameraViewMatrix * vec4<f32>( nodeVar19, 1.0 ) ).xyz, ( ( render.nodeUniform20 + render.nodeUniform21 ) - render.nodeUniform22 ), ( ( render.nodeUniform20 - render.nodeUniform21 ) - render.nodeUniform22 ), ( ( render.nodeUniform20 - render.nodeUniform21 ) + render.nodeUniform22 ), ( ( render.nodeUniform20 + render.nodeUniform21 ) + render.nodeUniform22 ) ) ), vec3<f32>( 1.5 ) ) );
			

		}

		nodeVar24 = vec3<f32>( render.nodeUniform24, 0.0, ( render.nodeUniform24 * 0.3 ) );
		nodeVar25 = textureSampleLevel( nodeUniform23, nodeUniform23_sampler, tsl_mod_vec3( ( ( nodeVar19 + ( nodeVar24 * vec3<f32>( 1.0 ) ) ) * vec3<f32>( 0.1 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar26 = textureSampleLevel( nodeUniform23, nodeUniform23_sampler, tsl_mod_vec3( ( ( nodeVar19 + ( nodeVar24 * vec3<f32>( 1.0 ) ) ) * vec3<f32>( 0.05 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar27 = textureSampleLevel( nodeUniform23, nodeUniform23_sampler, tsl_mod_vec3( ( ( nodeVar19 + ( nodeVar24 * vec3<f32>( 2.0 ) ) ) * vec3<f32>( 0.02 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar23 = ( nodeVar23 * vec3<f32>( mix( 1.0, ( ( ( nodeVar25 + 0.5 ) * ( nodeVar26 + 0.5 ) ) * ( nodeVar27 + 0.5 ) ), object.nodeUniform25 ) ) );
		nodeVar28 = ( nodeVar23 * vec3<f32>( 0.01 ) );

		if ( nodeVar6 ) {

			nodeVar29 = ( nodeVar29 + ( ( nodeVar28 * nodeVar14 ) * vec3<f32>( nodeVar10 ) ) );
			

		} else {

			nodeVar29 = ( ( nodeVar29 * exp( ( ( - ( nodeVar23 * vec3<f32>( 0.01 ) ) ) * vec3<f32>( nodeVar10 ) ) ) ) + ( nodeVar28 * vec3<f32>( nodeVar10 ) ) );
			

		}

		nodeVar14 = ( nodeVar14 * exp( ( ( - ( nodeVar23 * vec3<f32>( 0.01 ) ) ) * vec3<f32>( nodeVar10 ) ) ) );
		nodeVar13 = ( nodeVar13 + nodeVar10 );

	}

	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar30 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar30;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar31 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar31;
	nodeVar32 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar32;
	outgoingLight = nodeVar29;
	nodeVar33 = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar33;

	// result

	output.color = nodeVar33;

	return output;

}
