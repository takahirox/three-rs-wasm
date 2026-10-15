// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );

// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform14_sampler : sampler_comparison;
@binding( 2 ) @group( 1 ) var nodeUniform14 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : vec3<f32>,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform10 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform7 : vec3<f32>,
	nodeUniform8 : vec3<f32>,
	nodeUniform11 : mat4x4<f32>,
	nodeUniform12 : f32,
	nodeUniform13 : f32,
	nodeUniform15 : f32,
	nodeUniform16 : vec2<f32>,
	nodeUniform17 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar6 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec2<f32>;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec2<f32>;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec2<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : vec2<f32>;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec2<f32>;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar28 : vec4<f32>;
var<private> nodeVar29 : vec4<f32>;
var<private> nodeVar30 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar31 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : vec4<f32>;

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
fn main( @location( 0 ) v_positionWorld : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	EmissiveColor = ( object.nodeUniform2 * vec3<f32>( object.nodeUniform3 ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	nodeVar0 = ( render.nodeUniform7 - render.nodeUniform8 );
	nodeVar1 = vec4<f32>( nodeVar0, 0.0 );
	nodeVar2 = ( render.cameraViewMatrix * nodeVar1 );
	nodeVar3 = normalize( nodeVar2.xyz );
	nodeVar4 = nodeVar3;
	nodeVar5 = dot( normalView, nodeVar4 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar7 = ( render.nodeUniform11 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform12 ) ) ), 1.0 ) );
	nodeVar8 = ( nodeVar7.xyz / vec3<f32>( nodeVar7.w ) );
	nodeVar9 = vec3<f32>( nodeVar8.x, ( 1.0 - nodeVar8.y ), ( nodeVar8.z + render.nodeUniform13 ) );

	if ( ( ( ( ( ( nodeVar9.x >= 0.0 ) && ( nodeVar9.x <= 1.0 ) ) && ( nodeVar9.y >= 0.0 ) ) && ( nodeVar9.y <= 1.0 ) ) && ( nodeVar9.z <= 1.0 ) ) ) {

		nodeVar10 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar11 = ( render.nodeUniform15 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform16 ).x );
		nodeVar12 = ( nodeVar9.xy + ( vogelDiskSample( 0, 5, nodeVar10 ) * vec2<f32>( nodeVar11 ) ) );
		nodeVar13 = textureSampleCompare( nodeUniform14, nodeUniform14_sampler, nodeVar12, nodeVar9.z );
		nodeVar14 = ( nodeVar9.xy + ( vogelDiskSample( 1, 5, nodeVar10 ) * vec2<f32>( nodeVar11 ) ) );
		nodeVar15 = textureSampleCompare( nodeUniform14, nodeUniform14_sampler, nodeVar14, nodeVar9.z );
		nodeVar16 = ( nodeVar9.xy + ( vogelDiskSample( 2, 5, nodeVar10 ) * vec2<f32>( nodeVar11 ) ) );
		nodeVar17 = textureSampleCompare( nodeUniform14, nodeUniform14_sampler, nodeVar16, nodeVar9.z );
		nodeVar18 = ( nodeVar9.xy + ( vogelDiskSample( 3, 5, nodeVar10 ) * vec2<f32>( nodeVar11 ) ) );
		nodeVar19 = textureSampleCompare( nodeUniform14, nodeUniform14_sampler, nodeVar18, nodeVar9.z );
		nodeVar20 = ( nodeVar9.xy + ( vogelDiskSample( 4, 5, nodeVar10 ) * vec2<f32>( nodeVar11 ) ) );
		nodeVar21 = textureSampleCompare( nodeUniform14, nodeUniform14_sampler, nodeVar20, nodeVar9.z );
		nodeVar6 = ( ( ( ( ( nodeVar13 + nodeVar15 ) + nodeVar17 ) + nodeVar19 ) + nodeVar21 ) * 0.2 );

	} else {

		nodeVar6 = 1.0;

	}

	nodeVar22 = mix( 1.0, nodeVar6, render.nodeUniform17 );
	nodeVar23 = ( render.nodeUniform9 * vec3<f32>( nodeVar22 ) );
	nodeVar24 = ( vec3<f32>( clamp( nodeVar5, 0.0, 1.0 ) ) * nodeVar23 );
	nodeVar25 = ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) );
	nodeVar26 = ( nodeVar24 * nodeVar25 );
	nodeVar27 = ( directDiffuse + nodeVar26 );
	directDiffuse = nodeVar27;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar28 = ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) );
	nodeVar29 = ( vec4<f32>( irradiance, 1.0 ) * nodeVar28 );
	nodeVar30 = ( vec4<f32>( indirectDiffuse, 1.0 ) + nodeVar29 );
	indirectDiffuse = nodeVar30.xyz;
	ambientOcclusion = 1.0;
	nodeVar31 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar31;
	nodeVar32 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar32;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar33 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar33;
	nodeVar34 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar34;
	nodeVar35 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar35;

	// result

	output.color = nodeVar35;

	return output;

}
