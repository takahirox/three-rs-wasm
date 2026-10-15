// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform13_sampler : sampler_comparison;
@binding( 2 ) @group( 1 ) var nodeUniform13 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform8 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform3 : vec3<f32>,
	nodeUniform7 : vec3<f32>,
	nodeUniform2 : vec3<f32>,
	nodeUniform11 : mat4x4<f32>,
	nodeUniform10 : vec4<f32>,
	nodeUniform17 : mat4x4<f32>,
	nodeUniform16 : vec4<f32>,
	nodeUniform9 : f32,
	nodeUniform12 : f32,
	nodeUniform14 : f32,
	nodeUniform15 : vec2<f32>,
	nodeUniform18 : f32,
	nodeUniform19 : f32,
	nodeUniform20 : vec2<f32>,
	nodeUniform21 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> shadowMask : f32;
var<private> Output : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : f32;
var<private> shadowValue : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : vec2<f32>;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec2<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec2<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec2<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : vec2<f32>;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec4<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec2<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec2<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec2<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec2<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : vec2<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : vec4<f32>;

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
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar0 = dot( normalWorld, normalize( render.nodeUniform7 ) );
	nodeVar1 = ( nodeVar0 * 0.5 );
	nodeVar2 = ( nodeVar1 + 0.5 );
	nodeVar3 = mix( render.nodeUniform2, render.nodeUniform3, nodeVar2 );
	nodeVar4 = ( irradiance + nodeVar3 );
	irradiance = nodeVar4;
	shadowPositionWorld = v_positionWorld;
	nodeVar5 = vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform9 ) ) ), 1.0 );
	nodeVar6 = ( - v_positionView.z );
	shadowValue = 1.0;

	if ( ( ( nodeVar6 >= render.nodeUniform10.x ) && ( nodeVar6 < render.nodeUniform10.y ) ) ) {

		nodeVar8 = ( render.nodeUniform11 * nodeVar5 );
		nodeVar9 = ( nodeVar8.xyz / vec3<f32>( nodeVar8.w ) );
		nodeVar10 = vec3<f32>( nodeVar9.x, ( 1.0 - nodeVar9.y ), ( nodeVar9.z + render.nodeUniform12 ) );

		if ( ( ( ( ( ( nodeVar10.x >= 0.0 ) && ( nodeVar10.x <= 1.0 ) ) && ( nodeVar10.y >= 0.0 ) ) && ( nodeVar10.y <= 1.0 ) ) && ( nodeVar10.z <= 1.0 ) ) ) {

			nodeVar11 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar12 = ( render.nodeUniform14 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform15 ).x );
			nodeVar13 = ( nodeVar10.xy + ( vogelDiskSample( 0, 5, nodeVar11 ) * vec2<f32>( nodeVar12 ) ) );
			nodeVar14 = textureSampleCompare( nodeUniform13, nodeUniform13_sampler, nodeVar13, nodeVar10.z );
			nodeVar15 = ( nodeVar10.xy + ( vogelDiskSample( 1, 5, nodeVar11 ) * vec2<f32>( nodeVar12 ) ) );
			nodeVar16 = textureSampleCompare( nodeUniform13, nodeUniform13_sampler, nodeVar15, nodeVar10.z );
			nodeVar17 = ( nodeVar10.xy + ( vogelDiskSample( 2, 5, nodeVar11 ) * vec2<f32>( nodeVar12 ) ) );
			nodeVar18 = textureSampleCompare( nodeUniform13, nodeUniform13_sampler, nodeVar17, nodeVar10.z );
			nodeVar19 = ( nodeVar10.xy + ( vogelDiskSample( 3, 5, nodeVar11 ) * vec2<f32>( nodeVar12 ) ) );
			nodeVar20 = textureSampleCompare( nodeUniform13, nodeUniform13_sampler, nodeVar19, nodeVar10.z );
			nodeVar21 = ( nodeVar10.xy + ( vogelDiskSample( 4, 5, nodeVar11 ) * vec2<f32>( nodeVar12 ) ) );
			nodeVar22 = textureSampleCompare( nodeUniform13, nodeUniform13_sampler, nodeVar21, nodeVar10.z );
			nodeVar7 = ( ( ( ( ( nodeVar14 + nodeVar16 ) + nodeVar18 ) + nodeVar20 ) + nodeVar22 ) * 0.2 );

		} else {

			nodeVar7 = 1.0;

		}

		shadowValue = mix( nodeVar7, shadowValue, smoothstep( render.nodeUniform10.z, render.nodeUniform10.y, nodeVar6 ) );
		

	}


	if ( ( ( nodeVar6 >= render.nodeUniform16.x ) && ( nodeVar6 < render.nodeUniform16.y ) ) ) {

		nodeVar24 = ( render.nodeUniform17 * nodeVar5 );
		nodeVar25 = ( nodeVar24.xyz / vec3<f32>( nodeVar24.w ) );
		nodeVar26 = vec3<f32>( nodeVar25.x, ( 1.0 - nodeVar25.y ), ( nodeVar25.z + render.nodeUniform18 ) );

		if ( ( ( ( ( ( nodeVar26.x >= 0.0 ) && ( nodeVar26.x <= 1.0 ) ) && ( nodeVar26.y >= 0.0 ) ) && ( nodeVar26.y <= 1.0 ) ) && ( nodeVar26.z <= 1.0 ) ) ) {

			nodeVar27 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar28 = ( render.nodeUniform19 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform20 ).x );
			nodeVar29 = ( nodeVar26.xy + ( vogelDiskSample( 0, 5, nodeVar27 ) * vec2<f32>( nodeVar28 ) ) );
			nodeVar30 = textureSampleCompare( nodeUniform13, nodeUniform13_sampler, nodeVar29, nodeVar26.z );
			nodeVar31 = ( nodeVar26.xy + ( vogelDiskSample( 1, 5, nodeVar27 ) * vec2<f32>( nodeVar28 ) ) );
			nodeVar32 = textureSampleCompare( nodeUniform13, nodeUniform13_sampler, nodeVar31, nodeVar26.z );
			nodeVar33 = ( nodeVar26.xy + ( vogelDiskSample( 2, 5, nodeVar27 ) * vec2<f32>( nodeVar28 ) ) );
			nodeVar34 = textureSampleCompare( nodeUniform13, nodeUniform13_sampler, nodeVar33, nodeVar26.z );
			nodeVar35 = ( nodeVar26.xy + ( vogelDiskSample( 3, 5, nodeVar27 ) * vec2<f32>( nodeVar28 ) ) );
			nodeVar36 = textureSampleCompare( nodeUniform13, nodeUniform13_sampler, nodeVar35, nodeVar26.z );
			nodeVar37 = ( nodeVar26.xy + ( vogelDiskSample( 4, 5, nodeVar27 ) * vec2<f32>( nodeVar28 ) ) );
			nodeVar38 = textureSampleCompare( nodeUniform13, nodeUniform13_sampler, nodeVar37, nodeVar26.z );
			nodeVar23 = ( ( ( ( ( nodeVar30 + nodeVar32 ) + nodeVar34 ) + nodeVar36 ) + nodeVar38 ) * 0.2 );

		} else {

			nodeVar23 = 1.0;

		}

		shadowValue = mix( nodeVar23, shadowValue, smoothstep( render.nodeUniform16.z, render.nodeUniform16.y, nodeVar6 ) );
		

	}

	nodeVar39 = mix( 1.0, shadowValue, render.nodeUniform21 );
	nodeVar40 = ( shadowMask * nodeVar39 );
	shadowMask = nodeVar40;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar41 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar41;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar42 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar42;
	nodeVar43 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar43;
	nodeVar44 = ( 1.0 - shadowMask );
	nodeVar45 = nodeVar44;
	nodeVar46 = ( DiffuseColor.w * nodeVar45 );
	DiffuseColor.w = nodeVar46;
	outgoingLight = DiffuseColor.xyz;
	nodeVar47 = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar47;

	// result

	output.color = nodeVar47;

	return output;

}
