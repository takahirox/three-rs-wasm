// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform12 : texture_2d<f32>;
@binding( 2 ) @group( 1 ) var nodeUniform16 : texture_depth_2d;
@binding( 3 ) @group( 1 ) var nodeUniform17 : texture_depth_2d;

struct renderStruct {
	cameraNear : f32,
	cameraFar : f32,
	nodeUniform1 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform8 : vec3<f32>,
	nodeUniform3 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform18 : vec3<f32>,
	nodeUniform19 : f32,
	nodeUniform20 : f32,
	nodeUniform13 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform2 : f32,
	nodeUniform6 : mat3x3<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : vec2<u32>;
var<private> nodeVar19 : vec2<f32>;
var<private> nodeVar20 : vec2<f32>;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec2<u32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec4<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : vec2<u32>;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec4<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec4<f32>;

// codes
fn fn4 ( p : vec2<f32> ) -> vec2<f32> {

	


	return fract( ( sin( vec2<f32>( dot( p, vec2<f32>( 127.1, 311.7 ) ), dot( p, vec2<f32>( 269.5, 183.3 ) ) ) ) * vec2<f32>( 18.5453 ) ) );

}


fn fn5 ( p : vec2<f32>, time : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : vec2<f32>;

	let nodeConst0 = floor( p );
	let nodeConst1 = fract( p );
	nodeVar0 = 8.0;

	for ( var x : i32 = -1; x <= 1; x ++ ) {


		for ( var y : i32 = -1; y <= 1; y ++ ) {

			let nodeConst2 = vec2<f32>( f32( x ), f32( y ) );
			let nodeConst3 = fn4( ( nodeConst0 + nodeConst2 ) );
			nodeVar1 = ( ( nodeConst2 - nodeConst1 ) + ( ( sin( ( vec2<f32>( time ) + ( nodeConst3 * vec2<f32>( 6.283185307179586 ) ) ) ) * vec2<f32>( 0.5 ) ) + vec2<f32>( 0.5 ) ) );
			nodeVar0 = min( nodeVar0, dot( nodeVar1, nodeVar1 ) );

		}


	}


	return nodeVar0;

}


fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}



@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = ( render.nodeUniform1 * 0.8 );
	nodeVar1 = ( fn5( ( v_positionWorld.xz * vec2<f32>( 6.0 ) ), nodeVar0 ) * fn5( ( v_positionWorld.xz * vec2<f32>( 3.0 ) ), nodeVar0 ) );
	nodeVar2 = mix( vec3<f32>( 0.001214107934117647, 0.24228112245478564, 0.76052450467022 ), vec3<f32>( 0.174647403645279, 0.6038273388475408, 0.9046611743890203 ), ( nodeVar1 * 1.4 ) );
	DiffuseColor = vec4<f32>( nodeVar2, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform2 );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	normalView = normalViewGeometry;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar3 = dot( normalWorld, normalize( render.nodeUniform8 ) );
	nodeVar4 = ( nodeVar3 * 0.5 );
	nodeVar5 = ( nodeVar4 + 0.5 );
	nodeVar6 = mix( render.nodeUniform3, render.nodeUniform4, nodeVar5 );
	nodeVar7 = ( irradiance + nodeVar6 );
	irradiance = nodeVar7;
	nodeVar8 = dot( normalWorld, normalize( render.nodeUniform11 ) );
	nodeVar9 = ( nodeVar8 * 0.5 );
	nodeVar10 = ( nodeVar9 + 0.5 );
	nodeVar11 = mix( render.nodeUniform9, render.nodeUniform10, nodeVar10 );
	nodeVar12 = ( irradiance + nodeVar11 );
	irradiance = nodeVar12;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = vec4<f32>( 0.0, 0.0, 0.0, 0.0 ).xyz;
	nodeVar13 = ( vec4<f32>( indirectDiffuse, 1.0 ) + vec4<f32>( 1.0, 1.0, 1.0, 0.0 ) );
	indirectDiffuse = nodeVar13.xyz;
	ambientOcclusion = 1.0;
	nodeVar14 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar14;
	nodeVar15 = ( indirectDiffuse * DiffuseColor.xyz );
	indirectDiffuse = nodeVar15;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar16 = ( directDiffuse + indirectDiffuse );
	nodeVar18 = textureDimensions( nodeUniform12, u32( 0 ) );
	nodeVar17 = textureLoad( nodeUniform12, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( fragCoord.xy / render.nodeUniform13 ) ) * vec2<f32>( nodeVar18 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar18 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar20 = ( ( fragCoord.xy / render.nodeUniform13 ) + vec2<f32>( 0.0, ( nodeVar1 * 0.1 ) ) );
	nodeVar22 = textureDimensions( nodeUniform16, u32( 0 ) );
	nodeVar21 = textureLoad( nodeUniform16, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar20 ) * vec2<f32>( nodeVar22 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar22 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar23 = ( ( ( ( ( render.cameraNear * render.cameraFar ) / ( ( ( render.cameraFar - render.cameraNear ) * nodeVar21 ) - render.cameraFar ) ) + render.cameraNear ) / ( render.cameraNear - render.cameraFar ) ) - ( ( v_positionView.z + render.cameraNear ) / ( render.cameraNear - render.cameraFar ) ) );

	if ( ( nodeVar23 < 0.0 ) ) {

		nodeVar19 = ( fragCoord.xy / render.nodeUniform13 );

	} else {

		nodeVar19 = nodeVar20;

	}

	nodeVar24 = textureLoad( nodeUniform12, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar19 ) * vec2<f32>( nodeVar18 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar18 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar25 = ( ( clamp( ( ( nodeVar23 - 0.0 ) / ( 0.1 - 0.0 ) ), 0.0, 1.0 ) * ( 1.0 - 0.0 ) ) + 0.0 );
	nodeVar27 = textureDimensions( nodeUniform17, u32( 0 ) );
	nodeVar26 = textureLoad( nodeUniform17, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( fragCoord.xy / render.nodeUniform13 ) ) * vec2<f32>( nodeVar27 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar27 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar28 = ( 1.0 - nodeVar25 );
	nodeVar29 = mix( vec4<f32>( nodeVar16, 1.0 ), ( mix( nodeVar17, ( nodeVar24 * vec4<f32>( mix( vec3<f32>( 1.0 ), nodeVar2, nodeVar25 ), 1.0 ) ), ( ( clamp( ( ( ( ( ( ( ( render.cameraNear * render.cameraFar ) / ( ( ( render.cameraFar - render.cameraNear ) * nodeVar26 ) - render.cameraFar ) ) + render.cameraNear ) / ( render.cameraNear - render.cameraFar ) ) - ( ( v_positionView.z + render.cameraNear ) / ( render.cameraNear - render.cameraFar ) ) ) - -0.002 ) / ( 0.04 - -0.002 ) ), 0.0, 1.0 ) * ( 1.0 - 0.0 ) ) + 0.0 ) ) * vec4<f32>( vec3<f32>( 0.6514056374127929, 0.8307698767709715, 0.9386857284565036 ), 1.0 ) ), nodeVar28 );
	totalDiffuse = nodeVar29.xyz;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar30 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar30;
	nodeVar31 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar31;
	Output = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar32 = vec4<f32>( mix( Output.xyz, render.nodeUniform18, smoothstep( render.nodeUniform19, render.nodeUniform20, ( - v_positionView.z ) ) ), Output.w );
	Output = nodeVar32;

	// result

	output.color = nodeVar32;

	return output;

}
