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
@binding( 1 ) @group( 1 ) var nodeUniform17_sampler : sampler_comparison;
@binding( 2 ) @group( 1 ) var nodeUniform17 : texture_depth_2d;
@binding( 3 ) @group( 1 ) var nodeUniform23_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform23 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : vec3<f32>,
	nodeUniform3 : f32,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform11 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform5 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform21 : f32,
	nodeUniform22 : f32,
	nodeUniform14 : vec3<f32>,
	nodeUniform10 : mat4x4<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform15 : f32,
	nodeUniform16 : f32,
	nodeUniform18 : f32,
	nodeUniform19 : vec2<f32>,
	nodeUniform20 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : f32;
var<private> indirectDiffuse : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn129 ( p : vec2<f32>, b : vec2<f32> ) -> f32 {

	

	let nodeConst0 = ( abs( p ) - b );

	return ( length( max( nodeConst0, vec2<f32>( 0.0 ) ) ) + min( max( nodeConst0.x, nodeConst0.y ), 0.0 ) );

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




@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	EmissiveColor = ( object.nodeUniform2 * vec3<f32>( object.nodeUniform3 ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	irradiance = ( irradiance + mix( render.nodeUniform4, render.nodeUniform5, ( ( dot( normalWorld, normalize( render.nodeUniform9 ) ) * 0.5 ) + 0.5 ) ) );
	var nodeVar0 : f32 = 0.0;
	let nodeConst0 = ( render.nodeUniform10 * vec4<f32>( v_positionWorld, 1.0 ) );

	if ( ( nodeConst0.w > 0.0 ) ) {

		nodeVar0 = clamp( ( ( fn129( ( ( nodeConst0.xyz / vec3<f32>( nodeConst0.w ) ).xy - vec2<f32>( 0.5, 0.5 ) ), vec2<f32>( 0.5, 0.5 ) ) * -2.0 ) * ( -1.0 / ( ( 1.0 - acos( render.nodeUniform12 ) ) - 1.0 ) ) ), 0.0, 1.0 );
		

	}

	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst1 = ( render.nodeUniform13 - v_positionView );
	let nodeConst2 = ( render.nodeUniform10 * vec4<f32>( v_positionWorld, 1.0 ) );
	let nodeConst3 = ( nodeConst2.xyz / vec3<f32>( nodeConst2.w ) );

	if ( all( ( abs( ( ( nodeConst3 * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ) ) < vec3<f32>( 1.0 ) ) ) ) {

		shadowPositionWorld = v_positionWorld;
		let nodeConst4 = ( render.nodeUniform10 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform15 ) ) ), 1.0 ) );
		let nodeConst5 = ( nodeConst4.xyz / vec3<f32>( nodeConst4.w ) );
		let nodeConst6 = vec3<f32>( nodeConst5.x, ( 1.0 - nodeConst5.y ), ( nodeConst5.z + render.nodeUniform16 ) );

		if ( ( ( ( ( ( nodeConst6.x >= 0.0 ) && ( nodeConst6.x <= 1.0 ) ) && ( nodeConst6.y >= 0.0 ) ) && ( nodeConst6.y <= 1.0 ) ) && ( nodeConst6.z <= 1.0 ) ) ) {

			let nodeConst7 = ( fn24( fragCoord.xy ) * 6.28318530718 );
			let nodeConst8 = ( vec2<f32>( render.nodeUniform18 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform19 ) );
			let nodeConst9 = ( nodeConst6.xy + ( fn25( 0, 5, nodeConst7 ) * nodeConst8 ) );
			nodeVar3 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeConst9, nodeConst6.z );
			let nodeConst10 = ( nodeConst6.xy + ( fn25( 1, 5, nodeConst7 ) * nodeConst8 ) );
			nodeVar4 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeConst10, nodeConst6.z );
			let nodeConst11 = ( nodeConst6.xy + ( fn25( 2, 5, nodeConst7 ) * nodeConst8 ) );
			nodeVar5 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeConst11, nodeConst6.z );
			let nodeConst12 = ( nodeConst6.xy + ( fn25( 3, 5, nodeConst7 ) * nodeConst8 ) );
			nodeVar6 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeConst12, nodeConst6.z );
			let nodeConst13 = ( nodeConst6.xy + ( fn25( 4, 5, nodeConst7 ) * nodeConst8 ) );
			nodeVar7 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeConst13, nodeConst6.z );
			nodeVar2 = ( ( ( ( ( nodeVar3 + nodeVar4 ) + nodeVar5 ) + nodeVar6 ) + nodeVar7 ) * 0.2 );

		} else {

			nodeVar2 = 1.0;

		}

		nodeVar8 = mix( 1.0, nodeVar2, render.nodeUniform20 );
		let nodeConst14 = ( ( ( render.nodeUniform14 * vec3<f32>( nodeVar8 ) ) * vec3<f32>( nodeVar0 ) ) * vec3<f32>( fn44( length( nodeConst1 ), render.nodeUniform21, render.nodeUniform22 ) ) );
		nodeVar9 = textureSample( nodeUniform23, nodeUniform23_sampler, nodeConst3.xy );
		nodeVar1 = ( vec4<f32>( nodeConst14, 1.0 ) * nodeVar9 );

	} else {

		shadowPositionWorld = v_positionWorld;
		let nodeConst4 = ( render.nodeUniform10 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform15 ) ) ), 1.0 ) );
		let nodeConst5 = ( nodeConst4.xyz / vec3<f32>( nodeConst4.w ) );
		let nodeConst6 = vec3<f32>( nodeConst5.x, ( 1.0 - nodeConst5.y ), ( nodeConst5.z + render.nodeUniform16 ) );

		if ( ( ( ( ( ( nodeConst6.x >= 0.0 ) && ( nodeConst6.x <= 1.0 ) ) && ( nodeConst6.y >= 0.0 ) ) && ( nodeConst6.y <= 1.0 ) ) && ( nodeConst6.z <= 1.0 ) ) ) {

			let nodeConst7 = ( fn24( fragCoord.xy ) * 6.28318530718 );
			let nodeConst8 = ( vec2<f32>( render.nodeUniform18 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform19 ) );
			let nodeConst9 = ( nodeConst6.xy + ( fn25( 0, 5, nodeConst7 ) * nodeConst8 ) );
			nodeVar3 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeConst9, nodeConst6.z );
			let nodeConst10 = ( nodeConst6.xy + ( fn25( 1, 5, nodeConst7 ) * nodeConst8 ) );
			nodeVar4 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeConst10, nodeConst6.z );
			let nodeConst11 = ( nodeConst6.xy + ( fn25( 2, 5, nodeConst7 ) * nodeConst8 ) );
			nodeVar5 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeConst11, nodeConst6.z );
			let nodeConst12 = ( nodeConst6.xy + ( fn25( 3, 5, nodeConst7 ) * nodeConst8 ) );
			nodeVar6 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeConst12, nodeConst6.z );
			let nodeConst13 = ( nodeConst6.xy + ( fn25( 4, 5, nodeConst7 ) * nodeConst8 ) );
			nodeVar7 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeConst13, nodeConst6.z );
			nodeVar10 = ( ( ( ( ( nodeVar3 + nodeVar4 ) + nodeVar5 ) + nodeVar6 ) + nodeVar7 ) * 0.2 );

		} else {

			nodeVar10 = 1.0;

		}

		nodeVar8 = mix( 1.0, nodeVar10, render.nodeUniform20 );
		let nodeConst14 = ( ( ( render.nodeUniform14 * vec3<f32>( nodeVar8 ) ) * vec3<f32>( nodeVar0 ) ) * vec3<f32>( fn44( length( nodeConst1 ), render.nodeUniform21, render.nodeUniform22 ) ) );
		nodeVar1 = vec4<f32>( nodeConst14, 1.0 );

	}

	directDiffuse = ( vec4<f32>( directDiffuse, 1.0 ) + ( ( vec4<f32>( clamp( dot( normalView, normalize( nodeConst1 ) ), 0.0, 1.0 ) ) * nodeVar1 ) * vec4<f32>( ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) ), 1.0 ) ) ).xyz;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( vec4<f32>( indirectDiffuse, 1.0 ) + ( vec4<f32>( irradiance, 1.0 ) * ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) ) ) ).xyz;
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst15 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst15;

	// result

	output.color = nodeConst15;

	return output;

}
