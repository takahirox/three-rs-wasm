// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );

// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform23_sampler : sampler_comparison;
@binding( 2 ) @group( 1 ) var nodeUniform23 : texture_depth_2d;
@binding( 3 ) @group( 1 ) var nodeUniform30_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform30 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform37_sampler : sampler_comparison;
@binding( 6 ) @group( 1 ) var nodeUniform37 : texture_depth_2d;
@binding( 7 ) @group( 1 ) var nodeUniform44_sampler : sampler_comparison;
@binding( 8 ) @group( 1 ) var nodeUniform44 : texture_depth_2d;

struct cascadesStruct {
	value : array< vec4<f32>, 4 >
};
@binding( 1 ) @group( 0 )
var<uniform> cascades : cascadesStruct;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform13 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	shadowFar : f32,
	nodeUniform18 : f32,
	nodeUniform20 : mat4x4<f32>,
	nodeUniform21 : f32,
	nodeUniform22 : f32,
	nodeUniform24 : f32,
	nodeUniform25 : vec2<f32>,
	nodeUniform26 : f32,
	nodeUniform27 : mat4x4<f32>,
	nodeUniform28 : f32,
	nodeUniform29 : f32,
	nodeUniform31 : f32,
	nodeUniform32 : vec2<f32>,
	nodeUniform33 : f32,
	nodeUniform34 : mat4x4<f32>,
	nodeUniform35 : f32,
	nodeUniform36 : f32,
	nodeUniform38 : f32,
	nodeUniform39 : vec2<f32>,
	nodeUniform40 : f32,
	nodeUniform41 : mat4x4<f32>,
	nodeUniform42 : f32,
	nodeUniform43 : f32,
	nodeUniform45 : f32,
	nodeUniform46 : vec2<f32>,
	nodeUniform47 : f32
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
var<private> nodeVar0 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> shadowValue : vec4<f32>;
var<private> cascade : vec2<f32>;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> linearDepth : f32;
var<private> nodeVar27 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar28 : vec4<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec2<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec2<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : vec2<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : vec2<f32>;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : vec2<f32>;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : vec4<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : vec2<f32>;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : vec2<f32>;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : vec2<f32>;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : vec2<f32>;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : vec2<f32>;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : vec4<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : vec2<f32>;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : vec2<f32>;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : vec2<f32>;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : vec2<f32>;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : vec2<f32>;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : vec4<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : vec2<f32>;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : vec2<f32>;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : vec2<f32>;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : vec2<f32>;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : vec2<f32>;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : vec4<f32>;
var<private> nodeVar96 : vec4<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec4<f32>;
var<private> nodeVar99 : vec4<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec4<f32>;
var<private> nodeVar105 : vec4<f32>;
var<private> nodeVar106 : vec4<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar107 : vec4<f32>;
var<private> nodeVar108 : vec4<f32>;
var<private> nodeVar109 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar110 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec4<f32>;

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
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@location( 2 ) v_positionWorld : vec3<f32>,
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
	nodeVar0 = ( irradiance + render.nodeUniform6 );
	irradiance = nodeVar0;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	nodeVar1 = ( render.nodeUniform10 - render.nodeUniform11 );
	nodeVar2 = vec4<f32>( nodeVar1, 0.0 );
	nodeVar3 = ( render.cameraViewMatrix * nodeVar2 );
	nodeVar4 = normalize( nodeVar3.xyz );
	nodeVar5 = nodeVar4;
	nodeVar6 = dot( normalView, nodeVar5 );
	nodeVar7 = ( vec3<f32>( clamp( nodeVar6, 0.0, 1.0 ) ) * render.nodeUniform12 );
	nodeVar8 = ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) );
	nodeVar9 = ( nodeVar7 * nodeVar8 );
	nodeVar10 = ( directDiffuse + nodeVar9 );
	directDiffuse = nodeVar10;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar11 = normalize( ( nodeVar5 + positionViewDirection ) );
	nodeVar12 = clamp( dot( positionViewDirection, nodeVar11 ), 0.0, 1.0 );
	nodeVar13 = exp2( ( ( ( nodeVar12 * -5.55473 ) - 6.98316 ) * nodeVar12 ) );
	nodeVar14 = ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar13 ) ) ) + vec3<f32>( ( 1.0 * nodeVar13 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeVar11 ), 0.0, 1.0 ), Shininess ) ) ) );
	nodeVar15 = ( nodeVar7 * nodeVar14 );
	nodeVar16 = ( nodeVar15 * vec3<f32>( 1.0 ) );
	nodeVar17 = ( directSpecular + nodeVar16 );
	directSpecular = nodeVar17;
	nodeVar18 = ( render.nodeUniform14 - render.nodeUniform15 );
	nodeVar19 = vec4<f32>( nodeVar18, 0.0 );
	nodeVar20 = ( render.cameraViewMatrix * nodeVar19 );
	nodeVar21 = normalize( nodeVar20.xyz );
	nodeVar22 = nodeVar21;
	nodeVar23 = dot( normalView, nodeVar22 );
	shadowPositionWorld = v_positionWorld;
	shadowValue = vec4<f32>( 1.0, 1.0, 1.0, 1.0 );
	cascade = vec2<f32>( 0.0, 0.0 );
	cascade = cascades.value[ 0u ].xy;
	nodeVar24 = ( v_positionView.z + render.nodeUniform18 );
	nodeVar25 = ( render.nodeUniform18 - render.shadowFar );
	nodeVar26 = ( nodeVar24 / nodeVar25 );
	linearDepth = nodeVar26;

	if ( ( ( linearDepth >= cascade.x ) && ( linearDepth <= cascade.y ) ) ) {

		shadowPositionWorld = v_positionWorld;
		normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
		nodeVar28 = ( render.nodeUniform20 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform21 ) ) ), 1.0 ) );
		nodeVar29 = ( nodeVar28.xyz / vec3<f32>( nodeVar28.w ) );
		nodeVar30 = vec3<f32>( nodeVar29.x, ( 1.0 - nodeVar29.y ), ( nodeVar29.z + render.nodeUniform22 ) );

		if ( ( ( ( ( ( nodeVar30.x >= 0.0 ) && ( nodeVar30.x <= 1.0 ) ) && ( nodeVar30.y >= 0.0 ) ) && ( nodeVar30.y <= 1.0 ) ) && ( nodeVar30.z <= 1.0 ) ) ) {

			nodeVar31 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar32 = ( render.nodeUniform24 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform25 ).x );
			nodeVar33 = ( nodeVar30.xy + ( vogelDiskSample( 0, 5, nodeVar31 ) * vec2<f32>( nodeVar32 ) ) );
			nodeVar34 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeVar33, nodeVar30.z );
			nodeVar35 = ( nodeVar30.xy + ( vogelDiskSample( 1, 5, nodeVar31 ) * vec2<f32>( nodeVar32 ) ) );
			nodeVar36 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeVar35, nodeVar30.z );
			nodeVar37 = ( nodeVar30.xy + ( vogelDiskSample( 2, 5, nodeVar31 ) * vec2<f32>( nodeVar32 ) ) );
			nodeVar38 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeVar37, nodeVar30.z );
			nodeVar39 = ( nodeVar30.xy + ( vogelDiskSample( 3, 5, nodeVar31 ) * vec2<f32>( nodeVar32 ) ) );
			nodeVar40 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeVar39, nodeVar30.z );
			nodeVar41 = ( nodeVar30.xy + ( vogelDiskSample( 4, 5, nodeVar31 ) * vec2<f32>( nodeVar32 ) ) );
			nodeVar42 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeVar41, nodeVar30.z );
			nodeVar27 = ( ( ( ( ( nodeVar34 + nodeVar36 ) + nodeVar38 ) + nodeVar40 ) + nodeVar42 ) * 0.2 );

		} else {

			nodeVar27 = 1.0;

		}

		nodeVar43 = mix( 1.0, nodeVar27, render.nodeUniform26 );
		shadowValue = vec4<f32>( nodeVar43 );
		

	}

	cascade = cascades.value[ 1u ].xy;

	if ( ( ( linearDepth >= cascade.x ) && ( linearDepth <= cascade.y ) ) ) {

		shadowPositionWorld = v_positionWorld;
		normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
		nodeVar45 = ( render.nodeUniform27 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform28 ) ) ), 1.0 ) );
		nodeVar46 = ( nodeVar45.xyz / vec3<f32>( nodeVar45.w ) );
		nodeVar47 = vec3<f32>( nodeVar46.x, ( 1.0 - nodeVar46.y ), ( nodeVar46.z + render.nodeUniform29 ) );

		if ( ( ( ( ( ( nodeVar47.x >= 0.0 ) && ( nodeVar47.x <= 1.0 ) ) && ( nodeVar47.y >= 0.0 ) ) && ( nodeVar47.y <= 1.0 ) ) && ( nodeVar47.z <= 1.0 ) ) ) {

			nodeVar48 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar49 = ( render.nodeUniform31 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform32 ).x );
			nodeVar50 = ( nodeVar47.xy + ( vogelDiskSample( 0, 5, nodeVar48 ) * vec2<f32>( nodeVar49 ) ) );
			nodeVar51 = textureSampleCompare( nodeUniform30, nodeUniform30_sampler, nodeVar50, nodeVar47.z );
			nodeVar52 = ( nodeVar47.xy + ( vogelDiskSample( 1, 5, nodeVar48 ) * vec2<f32>( nodeVar49 ) ) );
			nodeVar53 = textureSampleCompare( nodeUniform30, nodeUniform30_sampler, nodeVar52, nodeVar47.z );
			nodeVar54 = ( nodeVar47.xy + ( vogelDiskSample( 2, 5, nodeVar48 ) * vec2<f32>( nodeVar49 ) ) );
			nodeVar55 = textureSampleCompare( nodeUniform30, nodeUniform30_sampler, nodeVar54, nodeVar47.z );
			nodeVar56 = ( nodeVar47.xy + ( vogelDiskSample( 3, 5, nodeVar48 ) * vec2<f32>( nodeVar49 ) ) );
			nodeVar57 = textureSampleCompare( nodeUniform30, nodeUniform30_sampler, nodeVar56, nodeVar47.z );
			nodeVar58 = ( nodeVar47.xy + ( vogelDiskSample( 4, 5, nodeVar48 ) * vec2<f32>( nodeVar49 ) ) );
			nodeVar59 = textureSampleCompare( nodeUniform30, nodeUniform30_sampler, nodeVar58, nodeVar47.z );
			nodeVar44 = ( ( ( ( ( nodeVar51 + nodeVar53 ) + nodeVar55 ) + nodeVar57 ) + nodeVar59 ) * 0.2 );

		} else {

			nodeVar44 = 1.0;

		}

		nodeVar60 = mix( 1.0, nodeVar44, render.nodeUniform33 );
		shadowValue = vec4<f32>( nodeVar60 );
		

	}

	cascade = cascades.value[ 2u ].xy;

	if ( ( ( linearDepth >= cascade.x ) && ( linearDepth <= cascade.y ) ) ) {

		shadowPositionWorld = v_positionWorld;
		normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
		nodeVar62 = ( render.nodeUniform34 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform35 ) ) ), 1.0 ) );
		nodeVar63 = ( nodeVar62.xyz / vec3<f32>( nodeVar62.w ) );
		nodeVar64 = vec3<f32>( nodeVar63.x, ( 1.0 - nodeVar63.y ), ( nodeVar63.z + render.nodeUniform36 ) );

		if ( ( ( ( ( ( nodeVar64.x >= 0.0 ) && ( nodeVar64.x <= 1.0 ) ) && ( nodeVar64.y >= 0.0 ) ) && ( nodeVar64.y <= 1.0 ) ) && ( nodeVar64.z <= 1.0 ) ) ) {

			nodeVar65 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar66 = ( render.nodeUniform38 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform39 ).x );
			nodeVar67 = ( nodeVar64.xy + ( vogelDiskSample( 0, 5, nodeVar65 ) * vec2<f32>( nodeVar66 ) ) );
			nodeVar68 = textureSampleCompare( nodeUniform37, nodeUniform37_sampler, nodeVar67, nodeVar64.z );
			nodeVar69 = ( nodeVar64.xy + ( vogelDiskSample( 1, 5, nodeVar65 ) * vec2<f32>( nodeVar66 ) ) );
			nodeVar70 = textureSampleCompare( nodeUniform37, nodeUniform37_sampler, nodeVar69, nodeVar64.z );
			nodeVar71 = ( nodeVar64.xy + ( vogelDiskSample( 2, 5, nodeVar65 ) * vec2<f32>( nodeVar66 ) ) );
			nodeVar72 = textureSampleCompare( nodeUniform37, nodeUniform37_sampler, nodeVar71, nodeVar64.z );
			nodeVar73 = ( nodeVar64.xy + ( vogelDiskSample( 3, 5, nodeVar65 ) * vec2<f32>( nodeVar66 ) ) );
			nodeVar74 = textureSampleCompare( nodeUniform37, nodeUniform37_sampler, nodeVar73, nodeVar64.z );
			nodeVar75 = ( nodeVar64.xy + ( vogelDiskSample( 4, 5, nodeVar65 ) * vec2<f32>( nodeVar66 ) ) );
			nodeVar76 = textureSampleCompare( nodeUniform37, nodeUniform37_sampler, nodeVar75, nodeVar64.z );
			nodeVar61 = ( ( ( ( ( nodeVar68 + nodeVar70 ) + nodeVar72 ) + nodeVar74 ) + nodeVar76 ) * 0.2 );

		} else {

			nodeVar61 = 1.0;

		}

		nodeVar77 = mix( 1.0, nodeVar61, render.nodeUniform40 );
		shadowValue = vec4<f32>( nodeVar77 );
		

	}

	cascade = cascades.value[ 3u ].xy;

	if ( ( ( linearDepth >= cascade.x ) && ( linearDepth <= cascade.y ) ) ) {

		shadowPositionWorld = v_positionWorld;
		normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
		nodeVar79 = ( render.nodeUniform41 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform42 ) ) ), 1.0 ) );
		nodeVar80 = ( nodeVar79.xyz / vec3<f32>( nodeVar79.w ) );
		nodeVar81 = vec3<f32>( nodeVar80.x, ( 1.0 - nodeVar80.y ), ( nodeVar80.z + render.nodeUniform43 ) );

		if ( ( ( ( ( ( nodeVar81.x >= 0.0 ) && ( nodeVar81.x <= 1.0 ) ) && ( nodeVar81.y >= 0.0 ) ) && ( nodeVar81.y <= 1.0 ) ) && ( nodeVar81.z <= 1.0 ) ) ) {

			nodeVar82 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar83 = ( render.nodeUniform45 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform46 ).x );
			nodeVar84 = ( nodeVar81.xy + ( vogelDiskSample( 0, 5, nodeVar82 ) * vec2<f32>( nodeVar83 ) ) );
			nodeVar85 = textureSampleCompare( nodeUniform44, nodeUniform44_sampler, nodeVar84, nodeVar81.z );
			nodeVar86 = ( nodeVar81.xy + ( vogelDiskSample( 1, 5, nodeVar82 ) * vec2<f32>( nodeVar83 ) ) );
			nodeVar87 = textureSampleCompare( nodeUniform44, nodeUniform44_sampler, nodeVar86, nodeVar81.z );
			nodeVar88 = ( nodeVar81.xy + ( vogelDiskSample( 2, 5, nodeVar82 ) * vec2<f32>( nodeVar83 ) ) );
			nodeVar89 = textureSampleCompare( nodeUniform44, nodeUniform44_sampler, nodeVar88, nodeVar81.z );
			nodeVar90 = ( nodeVar81.xy + ( vogelDiskSample( 3, 5, nodeVar82 ) * vec2<f32>( nodeVar83 ) ) );
			nodeVar91 = textureSampleCompare( nodeUniform44, nodeUniform44_sampler, nodeVar90, nodeVar81.z );
			nodeVar92 = ( nodeVar81.xy + ( vogelDiskSample( 4, 5, nodeVar82 ) * vec2<f32>( nodeVar83 ) ) );
			nodeVar93 = textureSampleCompare( nodeUniform44, nodeUniform44_sampler, nodeVar92, nodeVar81.z );
			nodeVar78 = ( ( ( ( ( nodeVar85 + nodeVar87 ) + nodeVar89 ) + nodeVar91 ) + nodeVar93 ) * 0.2 );

		} else {

			nodeVar78 = 1.0;

		}

		nodeVar94 = mix( 1.0, nodeVar78, render.nodeUniform47 );
		shadowValue = vec4<f32>( nodeVar94 );
		

	}

	nodeVar95 = ( vec4<f32>( render.nodeUniform16, 1.0 ) * shadowValue );
	nodeVar96 = ( vec4<f32>( clamp( nodeVar23, 0.0, 1.0 ) ) * nodeVar95 );
	nodeVar97 = ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) );
	nodeVar98 = ( nodeVar96 * vec4<f32>( nodeVar97, 1.0 ) );
	nodeVar99 = ( vec4<f32>( directDiffuse, 1.0 ) + nodeVar98 );
	directDiffuse = nodeVar99.xyz;
	nodeVar100 = normalize( ( nodeVar22 + positionViewDirection ) );
	nodeVar101 = clamp( dot( positionViewDirection, nodeVar100 ), 0.0, 1.0 );
	nodeVar102 = exp2( ( ( ( nodeVar101 * -5.55473 ) - 6.98316 ) * nodeVar101 ) );
	nodeVar103 = ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar102 ) ) ) + vec3<f32>( ( 1.0 * nodeVar102 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeVar100 ), 0.0, 1.0 ), Shininess ) ) ) );
	nodeVar104 = ( nodeVar96 * vec4<f32>( nodeVar103, 1.0 ) );
	nodeVar105 = ( nodeVar104 * vec4<f32>( 1.0 ) );
	nodeVar106 = ( vec4<f32>( directSpecular, 1.0 ) + nodeVar105 );
	directSpecular = nodeVar106.xyz;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar107 = ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) );
	nodeVar108 = ( vec4<f32>( irradiance, 1.0 ) * nodeVar107 );
	nodeVar109 = ( vec4<f32>( indirectDiffuse, 1.0 ) + nodeVar108 );
	indirectDiffuse = nodeVar109.xyz;
	ambientOcclusion = 1.0;
	nodeVar110 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar110;
	nodeVar111 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar111;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar112 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar112;
	nodeVar113 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar113;
	nodeVar114 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar114;

	// result

	output.color = nodeVar114;

	return output;

}
