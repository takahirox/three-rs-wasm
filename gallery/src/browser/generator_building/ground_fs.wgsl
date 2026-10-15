// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform10_sampler : sampler_comparison;
@binding( 2 ) @group( 1 ) var nodeUniform10 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform4 : mat3x3<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform8 : mat4x4<f32>,
	nodeUniform7 : vec4<f32>,
	nodeUniform14 : mat4x4<f32>,
	nodeUniform13 : vec4<f32>,
	nodeUniform6 : f32,
	nodeUniform9 : f32,
	nodeUniform11 : f32,
	nodeUniform12 : vec2<f32>,
	nodeUniform15 : f32,
	nodeUniform16 : f32,
	nodeUniform17 : vec2<f32>,
	nodeUniform18 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> shadowMask : f32;
var<private> Output : vec4<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : f32;
var<private> shadowValue : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : vec2<f32>;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : vec2<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec2<f32>;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec2<f32>;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec2<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec2<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec2<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec2<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec2<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : vec2<f32>;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : vec4<f32>;

// codes
fn interleavedGradientNoise ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn vogelDiskSample ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	var nodeVar0 : f32;

	nodeVar0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeVar0 ), sin( nodeVar0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

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
	shadowMask = 1.0;
	shadowPositionWorld = v_positionWorld;
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar0 = vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform6 ) ) ), 1.0 );
	nodeVar1 = ( - v_positionView.z );
	shadowValue = 1.0;

	if ( ( ( nodeVar1 >= render.nodeUniform7.x ) && ( nodeVar1 < render.nodeUniform7.y ) ) ) {

		nodeVar3 = ( render.nodeUniform8 * nodeVar0 );
		nodeVar4 = ( nodeVar3.xyz / vec3<f32>( nodeVar3.w ) );
		nodeVar5 = vec3<f32>( nodeVar4.x, ( 1.0 - nodeVar4.y ), ( nodeVar4.z + render.nodeUniform9 ) );

		if ( ( ( ( ( ( nodeVar5.x >= 0.0 ) && ( nodeVar5.x <= 1.0 ) ) && ( nodeVar5.y >= 0.0 ) ) && ( nodeVar5.y <= 1.0 ) ) && ( nodeVar5.z <= 1.0 ) ) ) {

			nodeVar6 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar7 = ( render.nodeUniform11 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform12 ).x );
			nodeVar8 = ( nodeVar5.xy + ( vogelDiskSample( 0, 5, nodeVar6 ) * vec2<f32>( nodeVar7 ) ) );
			nodeVar9 = textureSampleCompare( nodeUniform10, nodeUniform10_sampler, nodeVar8, nodeVar5.z );
			nodeVar10 = ( nodeVar5.xy + ( vogelDiskSample( 1, 5, nodeVar6 ) * vec2<f32>( nodeVar7 ) ) );
			nodeVar11 = textureSampleCompare( nodeUniform10, nodeUniform10_sampler, nodeVar10, nodeVar5.z );
			nodeVar12 = ( nodeVar5.xy + ( vogelDiskSample( 2, 5, nodeVar6 ) * vec2<f32>( nodeVar7 ) ) );
			nodeVar13 = textureSampleCompare( nodeUniform10, nodeUniform10_sampler, nodeVar12, nodeVar5.z );
			nodeVar14 = ( nodeVar5.xy + ( vogelDiskSample( 3, 5, nodeVar6 ) * vec2<f32>( nodeVar7 ) ) );
			nodeVar15 = textureSampleCompare( nodeUniform10, nodeUniform10_sampler, nodeVar14, nodeVar5.z );
			nodeVar16 = ( nodeVar5.xy + ( vogelDiskSample( 4, 5, nodeVar6 ) * vec2<f32>( nodeVar7 ) ) );
			nodeVar17 = textureSampleCompare( nodeUniform10, nodeUniform10_sampler, nodeVar16, nodeVar5.z );
			nodeVar2 = ( ( ( ( ( nodeVar9 + nodeVar11 ) + nodeVar13 ) + nodeVar15 ) + nodeVar17 ) * 0.2 );

		} else {

			nodeVar2 = 1.0;

		}

		shadowValue = mix( nodeVar2, shadowValue, smoothstep( render.nodeUniform7.z, render.nodeUniform7.y, nodeVar1 ) );
		

	}


	if ( ( ( nodeVar1 >= render.nodeUniform13.x ) && ( nodeVar1 < render.nodeUniform13.y ) ) ) {

		nodeVar19 = ( render.nodeUniform14 * nodeVar0 );
		nodeVar20 = ( nodeVar19.xyz / vec3<f32>( nodeVar19.w ) );
		nodeVar21 = vec3<f32>( nodeVar20.x, ( 1.0 - nodeVar20.y ), ( nodeVar20.z + render.nodeUniform15 ) );

		if ( ( ( ( ( ( nodeVar21.x >= 0.0 ) && ( nodeVar21.x <= 1.0 ) ) && ( nodeVar21.y >= 0.0 ) ) && ( nodeVar21.y <= 1.0 ) ) && ( nodeVar21.z <= 1.0 ) ) ) {

			nodeVar22 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar23 = ( render.nodeUniform16 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform17 ).x );
			nodeVar24 = ( nodeVar21.xy + ( vogelDiskSample( 0, 5, nodeVar22 ) * vec2<f32>( nodeVar23 ) ) );
			nodeVar25 = textureSampleCompare( nodeUniform10, nodeUniform10_sampler, nodeVar24, nodeVar21.z );
			nodeVar26 = ( nodeVar21.xy + ( vogelDiskSample( 1, 5, nodeVar22 ) * vec2<f32>( nodeVar23 ) ) );
			nodeVar27 = textureSampleCompare( nodeUniform10, nodeUniform10_sampler, nodeVar26, nodeVar21.z );
			nodeVar28 = ( nodeVar21.xy + ( vogelDiskSample( 2, 5, nodeVar22 ) * vec2<f32>( nodeVar23 ) ) );
			nodeVar29 = textureSampleCompare( nodeUniform10, nodeUniform10_sampler, nodeVar28, nodeVar21.z );
			nodeVar30 = ( nodeVar21.xy + ( vogelDiskSample( 3, 5, nodeVar22 ) * vec2<f32>( nodeVar23 ) ) );
			nodeVar31 = textureSampleCompare( nodeUniform10, nodeUniform10_sampler, nodeVar30, nodeVar21.z );
			nodeVar32 = ( nodeVar21.xy + ( vogelDiskSample( 4, 5, nodeVar22 ) * vec2<f32>( nodeVar23 ) ) );
			nodeVar33 = textureSampleCompare( nodeUniform10, nodeUniform10_sampler, nodeVar32, nodeVar21.z );
			nodeVar18 = ( ( ( ( ( nodeVar25 + nodeVar27 ) + nodeVar29 ) + nodeVar31 ) + nodeVar33 ) * 0.2 );

		} else {

			nodeVar18 = 1.0;

		}

		shadowValue = mix( nodeVar18, shadowValue, smoothstep( render.nodeUniform13.z, render.nodeUniform13.y, nodeVar1 ) );
		

	}

	nodeVar34 = mix( 1.0, shadowValue, render.nodeUniform18 );
	nodeVar35 = ( shadowMask * nodeVar34 );
	shadowMask = nodeVar35;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar36 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar36;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar37 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar37;
	nodeVar38 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar38;
	nodeVar39 = ( 1.0 - shadowMask );
	nodeVar40 = nodeVar39;
	nodeVar41 = ( DiffuseColor.w * nodeVar40 );
	DiffuseColor.w = nodeVar41;
	outgoingLight = DiffuseColor.xyz;
	nodeVar42 = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar42;

	// result

	output.color = nodeVar42;

	return output;

}
