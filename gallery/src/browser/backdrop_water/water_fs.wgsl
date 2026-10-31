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
var<private> Output : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec2<u32>;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec2<u32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec2<u32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar8 : vec4<f32>;

// codes
fn fn130 ( p : vec2<f32> ) -> vec2<f32> {

	


	return fract( ( sin( vec2<f32>( dot( p, vec2<f32>( 127.1, 311.7 ) ), dot( p, vec2<f32>( 269.5, 183.3 ) ) ) ) * vec2<f32>( 18.5453 ) ) );

}


fn fn131 ( p : vec2<f32>, time : f32 ) -> f32 {

	var nodeVar0 : f32;

	let nodeConst0 = floor( p );
	let nodeConst1 = fract( p );
	nodeVar0 = 8.0;

	for ( var x : i32 = -1; x <= 1; x ++ ) {


		for ( var y : i32 = -1; y <= 1; y ++ ) {

			let nodeConst2 = vec2<f32>( f32( x ), f32( y ) );
			let nodeConst3 = fn130( ( nodeConst0 + nodeConst2 ) );
			let nodeConst4 = ( ( nodeConst2 - nodeConst1 ) + ( ( sin( ( vec2<f32>( time ) + ( nodeConst3 * vec2<f32>( 6.283185307179586 ) ) ) ) * vec2<f32>( 0.5 ) ) + vec2<f32>( 0.5 ) ) );
			nodeVar0 = min( nodeVar0, dot( nodeConst4, nodeConst4 ) );

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

	let nodeConst0 = ( render.nodeUniform1 * 0.8 );
	let nodeConst1 = ( fn131( ( v_positionWorld.xz * vec2<f32>( 6.0 ) ), nodeConst0 ) * fn131( ( v_positionWorld.xz * vec2<f32>( 3.0 ) ), nodeConst0 ) );
	let nodeConst2 = mix( vec3<f32>( 0.001214107934117647, 0.24228112245478564, 0.76052450467022 ), vec3<f32>( 0.174647403645279, 0.6038273388475408, 0.9046611743890203 ), ( nodeConst1 * 1.4 ) );
	DiffuseColor = vec4<f32>( nodeConst2, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform2 );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	normalView = normalViewGeometry;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	irradiance = ( irradiance + mix( render.nodeUniform3, render.nodeUniform4, ( ( dot( normalWorld, normalize( render.nodeUniform8 ) ) * 0.5 ) + 0.5 ) ) );
	irradiance = ( irradiance + mix( render.nodeUniform9, render.nodeUniform10, ( ( dot( normalWorld, normalize( render.nodeUniform11 ) ) * 0.5 ) + 0.5 ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = vec4<f32>( 0.0, 0.0, 0.0, 0.0 ).xyz;
	indirectDiffuse = ( vec4<f32>( indirectDiffuse, 1.0 ) + vec4<f32>( 1.0, 1.0, 1.0, 0.0 ) ).xyz;
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = ( indirectDiffuse * DiffuseColor.xyz );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar1 = textureDimensions( nodeUniform12, u32( 0 ) );
	nodeVar0 = textureLoad( nodeUniform12, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( fragCoord.xy / render.nodeUniform13 ) ) * vec2<f32>( nodeVar1 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar1 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst3 = ( ( fragCoord.xy / render.nodeUniform13 ) + vec2<f32>( 0.0, ( nodeConst1 * 0.1 ) ) );
	nodeVar4 = textureDimensions( nodeUniform16, u32( 0 ) );
	nodeVar3 = textureLoad( nodeUniform16, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst3 ) * vec2<f32>( nodeVar4 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar4 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst4 = ( ( ( ( ( render.cameraNear * render.cameraFar ) / ( ( ( render.cameraFar - render.cameraNear ) * nodeVar3 ) - render.cameraFar ) ) + render.cameraNear ) / ( render.cameraNear - render.cameraFar ) ) - ( ( v_positionView.z + render.cameraNear ) / ( render.cameraNear - render.cameraFar ) ) );

	if ( ( nodeConst4 < 0.0 ) ) {

		nodeVar2 = ( fragCoord.xy / render.nodeUniform13 );

	} else {

		nodeVar2 = nodeConst3;

	}

	nodeVar5 = textureLoad( nodeUniform12, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar2 ) * vec2<f32>( nodeVar1 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar1 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst5 = ( ( clamp( ( ( nodeConst4 - 0.0 ) / ( 0.1 - 0.0 ) ), 0.0, 1.0 ) * ( 1.0 - 0.0 ) ) + 0.0 );
	nodeVar7 = textureDimensions( nodeUniform17, u32( 0 ) );
	nodeVar6 = textureLoad( nodeUniform17, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( fragCoord.xy / render.nodeUniform13 ) ) * vec2<f32>( nodeVar7 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar7 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	totalDiffuse = mix( vec4<f32>( ( directDiffuse + indirectDiffuse ), 1.0 ), ( mix( nodeVar0, ( nodeVar5 * vec4<f32>( mix( vec3<f32>( 1.0 ), nodeConst2, nodeConst5 ), 1.0 ) ), ( ( clamp( ( ( ( ( ( ( ( render.cameraNear * render.cameraFar ) / ( ( ( render.cameraFar - render.cameraNear ) * nodeVar6 ) - render.cameraFar ) ) + render.cameraNear ) / ( render.cameraNear - render.cameraFar ) ) - ( ( v_positionView.z + render.cameraNear ) / ( render.cameraNear - render.cameraFar ) ) ) - -0.002 ) / ( 0.04 - -0.002 ) ), 0.0, 1.0 ) * ( 1.0 - 0.0 ) ) + 0.0 ) ) * vec4<f32>( vec3<f32>( 0.6514056374127929, 0.8307698767709715, 0.9386857284565036 ), 1.0 ) ), ( 1.0 - nodeConst5 ) ).xyz;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	Output = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar8 = vec4<f32>( mix( Output.xyz, render.nodeUniform18, smoothstep( render.nodeUniform19, render.nodeUniform20, ( - v_positionView.z ) ) ), Output.w );
	let nodeConst6 = nodeVar8;
	Output = nodeConst6;

	// result

	output.color = nodeConst6;

	return output;

}
