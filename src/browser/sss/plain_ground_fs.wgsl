// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform19_sampler : sampler_comparison;
@binding( 2 ) @group( 1 ) var nodeUniform19 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform9 : mat3x3<f32>,
	nodeUniform15 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform7 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform16 : mat4x4<f32>,
	nodeUniform17 : f32,
	nodeUniform18 : f32,
	nodeUniform22 : f32,
	nodeUniform23 : vec3<f32>,
	nodeUniform24 : f32,
	nodeUniform25 : f32,
	nodeUniform20 : f32,
	nodeUniform21 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Shininess : f32;
var<private> SpecularColor : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
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
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec2<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec2<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : vec2<f32>;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : vec2<f32>;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : vec2<f32>;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar39 : vec4<f32>;
var<private> nodeVar40 : vec4<f32>;
var<private> nodeVar41 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar42 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec4<f32>;

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
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_normalViewGeometry : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Shininess = max( object.nodeUniform2, 0.0001 );
	SpecularColor = object.nodeUniform3;
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar0 = dot( normalWorld, normalize( render.nodeUniform11 ) );
	nodeVar1 = ( nodeVar0 * 0.5 );
	nodeVar2 = ( nodeVar1 + 0.5 );
	nodeVar3 = mix( render.nodeUniform6, render.nodeUniform7, nodeVar2 );
	nodeVar4 = ( irradiance + nodeVar3 );
	irradiance = nodeVar4;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar5 = ( render.nodeUniform12 - render.nodeUniform13 );
	nodeVar6 = vec4<f32>( nodeVar5, 0.0 );
	nodeVar7 = ( render.cameraViewMatrix * nodeVar6 );
	nodeVar8 = normalize( nodeVar7.xyz );
	nodeVar9 = nodeVar8;
	nodeVar10 = dot( normalView, nodeVar9 );
	shadowPositionWorld = v_positionWorld;
	nodeVar12 = ( render.nodeUniform16 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform17 ) ) ), 1.0 ) );
	nodeVar13 = ( nodeVar12.xyz / vec3<f32>( nodeVar12.w ) );
	nodeVar14 = vec3<f32>( nodeVar13.x, ( 1.0 - nodeVar13.y ), ( nodeVar13.z + render.nodeUniform18 ) );

	if ( ( ( ( ( ( nodeVar14.x >= 0.0 ) && ( nodeVar14.x <= 1.0 ) ) && ( nodeVar14.y >= 0.0 ) ) && ( nodeVar14.y <= 1.0 ) ) && ( nodeVar14.z <= 1.0 ) ) ) {

		nodeVar15 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar16 = ( render.nodeUniform20 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform21 ).x );
		nodeVar17 = ( nodeVar14.xy + ( vogelDiskSample( 0, 5, nodeVar15 ) * vec2<f32>( nodeVar16 ) ) );
		nodeVar18 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar17, nodeVar14.z );
		nodeVar19 = ( nodeVar14.xy + ( vogelDiskSample( 1, 5, nodeVar15 ) * vec2<f32>( nodeVar16 ) ) );
		nodeVar20 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar19, nodeVar14.z );
		nodeVar21 = ( nodeVar14.xy + ( vogelDiskSample( 2, 5, nodeVar15 ) * vec2<f32>( nodeVar16 ) ) );
		nodeVar22 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar21, nodeVar14.z );
		nodeVar23 = ( nodeVar14.xy + ( vogelDiskSample( 3, 5, nodeVar15 ) * vec2<f32>( nodeVar16 ) ) );
		nodeVar24 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar23, nodeVar14.z );
		nodeVar25 = ( nodeVar14.xy + ( vogelDiskSample( 4, 5, nodeVar15 ) * vec2<f32>( nodeVar16 ) ) );
		nodeVar26 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeVar25, nodeVar14.z );
		nodeVar11 = ( ( ( ( ( nodeVar18 + nodeVar20 ) + nodeVar22 ) + nodeVar24 ) + nodeVar26 ) * 0.2 );

	} else {

		nodeVar11 = 1.0;

	}

	nodeVar27 = mix( 1.0, nodeVar11, render.nodeUniform22 );
	nodeVar28 = ( vec3<f32>( clamp( nodeVar10, 0.0, 1.0 ) ) * ( render.nodeUniform14 * vec3<f32>( nodeVar27 ) ) );
	nodeVar29 = ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) );
	nodeVar30 = ( nodeVar28 * nodeVar29 );
	nodeVar31 = ( directDiffuse + nodeVar30 );
	directDiffuse = nodeVar31;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar32 = normalize( ( nodeVar9 + positionViewDirection ) );
	nodeVar33 = clamp( dot( positionViewDirection, nodeVar32 ), 0.0, 1.0 );
	nodeVar34 = exp2( ( ( ( nodeVar33 * -5.55473 ) - 6.98316 ) * nodeVar33 ) );
	nodeVar35 = ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar34 ) ) ) + vec3<f32>( ( 1.0 * nodeVar34 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeVar32 ), 0.0, 1.0 ), Shininess ) ) ) );
	nodeVar36 = ( nodeVar28 * nodeVar35 );
	nodeVar37 = ( nodeVar36 * vec3<f32>( 1.0 ) );
	nodeVar38 = ( directSpecular + nodeVar37 );
	directSpecular = nodeVar38;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar39 = ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) );
	nodeVar40 = ( vec4<f32>( irradiance, 1.0 ) * nodeVar39 );
	nodeVar41 = ( vec4<f32>( indirectDiffuse, 1.0 ) + nodeVar40 );
	indirectDiffuse = nodeVar41.xyz;
	ambientOcclusion = 1.0;
	nodeVar42 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar42;
	nodeVar43 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar43;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar44 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar44;
	nodeVar45 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar45;
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar46 = vec4<f32>( mix( Output.xyz, render.nodeUniform23, smoothstep( render.nodeUniform24, render.nodeUniform25, ( - v_positionView.z ) ) ), Output.w );
	Output = nodeVar46;

	// result

	output.color = nodeVar46;

	return output;

}
