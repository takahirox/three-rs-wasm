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
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn119 ( v1 : vec3<f32>, v2 : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;

	let nodeConst0 = dot( v1, v2 );
	nodeVar0 = abs( nodeConst0 );
	nodeVar1 = ( ( ( ( nodeVar0 * 0.0145206 ) + 0.4965155 ) * nodeVar0 ) + 0.8543985 );
	nodeVar2 = ( ( ( nodeVar0 + 4.1616724 ) * nodeVar0 ) + 3.417594 );

	if ( ( nodeConst0 > 0.0 ) ) {

		let nodeConst1 = ( nodeVar1 / nodeVar2 );
		nodeVar3 = nodeConst1;

	} else {

		let nodeConst1 = ( nodeVar1 / nodeVar2 );
		nodeVar3 = ( ( inverseSqrt( max( ( 1.0 - ( nodeConst0 * nodeConst0 ) ), 1e-7 ) ) * 0.5 ) - nodeConst1 );

	}


	return ( cross( v1, v2 ) * vec3<f32>( nodeVar3 ) );

}


fn fn118 ( f : vec3<f32> ) -> f32 {

	

	let nodeConst0 = length( f );

	return max( ( ( ( nodeConst0 * nodeConst0 ) + f.z ) / ( nodeConst0 + 1.0 ) ), 0.0 );

}


fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}

fn fn121 ( P : vec3<f32>, p0 : vec3<f32>, p1 : vec3<f32>, p2 : vec3<f32>, p3 : vec3<f32> ) -> vec3<f32> {

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
		nodeVar7 = ( nodeVar7 + fn119( nodeVar3, nodeVar4 ) );
		nodeVar7 = ( nodeVar7 + fn119( nodeVar4, nodeVar5 ) );
		nodeVar7 = ( nodeVar7 + fn119( nodeVar5, nodeVar6 ) );
		nodeVar7 = ( nodeVar7 + fn119( nodeVar6, nodeVar3 ) );
		nodeVar2 = vec3<f32>( fn118( abs( nodeVar7 ) ) );
		

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

		if ( ( nodeVar11 >= nodeVar8 ) ) {

			nodeVar12 = ( nodeVar12 + pow( ( render.nodeUniform11 * fn121( ( render.cameraViewMatrix * vec4<f32>( nodeConst1, 1.0 ) ).xyz, ( ( render.nodeUniform12 + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform13, 0.0 ) ).xyz ) - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform14, 0.0 ) ).xyz ), ( ( render.nodeUniform12 - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform13, 0.0 ) ).xyz ) - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform14, 0.0 ) ).xyz ), ( ( render.nodeUniform12 - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform13, 0.0 ) ).xyz ) + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform14, 0.0 ) ).xyz ), ( ( render.nodeUniform12 + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform13, 0.0 ) ).xyz ) + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform14, 0.0 ) ).xyz ) ) ), vec3<f32>( 1.5 ) ) );
			

		}


		if ( ( nodeVar11 >= nodeVar8 ) ) {

			nodeVar12 = ( nodeVar12 + pow( ( render.nodeUniform15 * fn121( ( render.cameraViewMatrix * vec4<f32>( nodeConst1, 1.0 ) ).xyz, ( ( render.nodeUniform16 + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform17, 0.0 ) ).xyz ) - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform18, 0.0 ) ).xyz ), ( ( render.nodeUniform16 - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform17, 0.0 ) ).xyz ) - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform18, 0.0 ) ).xyz ), ( ( render.nodeUniform16 - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform17, 0.0 ) ).xyz ) + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform18, 0.0 ) ).xyz ), ( ( render.nodeUniform16 + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform17, 0.0 ) ).xyz ) + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform18, 0.0 ) ).xyz ) ) ), vec3<f32>( 1.5 ) ) );
			

		}


		if ( ( nodeVar11 >= nodeVar8 ) ) {

			nodeVar12 = ( nodeVar12 + pow( ( render.nodeUniform19 * fn121( ( render.cameraViewMatrix * vec4<f32>( nodeConst1, 1.0 ) ).xyz, ( ( render.nodeUniform20 + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform21, 0.0 ) ).xyz ) - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform22, 0.0 ) ).xyz ), ( ( render.nodeUniform20 - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform21, 0.0 ) ).xyz ) - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform22, 0.0 ) ).xyz ), ( ( render.nodeUniform20 - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform21, 0.0 ) ).xyz ) + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform22, 0.0 ) ).xyz ), ( ( render.nodeUniform20 + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform21, 0.0 ) ).xyz ) + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform22, 0.0 ) ).xyz ) ) ), vec3<f32>( 1.5 ) ) );
			

		}

		let nodeConst2 = vec3<f32>( render.nodeUniform24, 0.0, ( render.nodeUniform24 * 0.3 ) );
		nodeVar13 = textureSampleLevel( nodeUniform23, nodeUniform23_sampler, tsl_mod_vec3( ( ( nodeConst1 + ( nodeConst2 * vec3<f32>( 1.0 ) ) ) * vec3<f32>( 0.1 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar14 = textureSampleLevel( nodeUniform23, nodeUniform23_sampler, tsl_mod_vec3( ( ( nodeConst1 + ( nodeConst2 * vec3<f32>( 1.0 ) ) ) * vec3<f32>( 0.05 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar15 = textureSampleLevel( nodeUniform23, nodeUniform23_sampler, tsl_mod_vec3( ( ( nodeConst1 + ( nodeConst2 * vec3<f32>( 2.0 ) ) ) * vec3<f32>( 0.02 ) ), vec3<f32>( 1.0 ) ), 0.0 ).x;
		nodeVar12 = ( nodeVar12 * vec3<f32>( mix( 1.0, ( ( ( nodeVar13 + 0.5 ) * ( nodeVar14 + 0.5 ) ) * ( nodeVar15 + 0.5 ) ), object.nodeUniform25 ) ) );
		nodeVar16 = ( nodeVar12 * vec3<f32>( 0.01 ) );

		if ( nodeVar2 ) {

			nodeVar17 = ( nodeVar17 + ( ( nodeVar16 * nodeVar6 ) * vec3<f32>( nodeVar3 ) ) );
			

		} else {

			let nodeConst3 = exp( ( ( - ( nodeVar12 * vec3<f32>( 0.01 ) ) ) * vec3<f32>( nodeVar3 ) ) );
			nodeVar17 = ( ( nodeVar17 * nodeConst3 ) + ( nodeVar16 * vec3<f32>( nodeVar3 ) ) );
			

		}

		let nodeConst3 = exp( ( ( - ( nodeVar12 * vec3<f32>( 0.01 ) ) ) * vec3<f32>( nodeVar3 ) ) );
		nodeVar6 = ( nodeVar6 * nodeConst3 );
		nodeVar5 = ( nodeVar5 + nodeVar3 );

	}

	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar17;
	let nodeConst4 = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst4;

	// result

	output.color = nodeConst4;

	return output;

}
