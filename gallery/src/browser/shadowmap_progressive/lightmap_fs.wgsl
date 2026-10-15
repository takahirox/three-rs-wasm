// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform16_sampler : sampler_comparison;
@binding( 2 ) @group( 1 ) var nodeUniform16 : texture_depth_2d;
@binding( 3 ) @group( 1 ) var nodeUniform26_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform26 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform36_sampler : sampler_comparison;
@binding( 6 ) @group( 1 ) var nodeUniform36 : texture_depth_2d;
@binding( 7 ) @group( 1 ) var nodeUniform46_sampler : sampler_comparison;
@binding( 8 ) @group( 1 ) var nodeUniform46 : texture_depth_2d;
@binding( 9 ) @group( 1 ) var nodeUniform50_sampler : sampler;
@binding( 10 ) @group( 1 ) var nodeUniform50 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform51 : mat3x3<f32>,
	nodeUniform52 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform11 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform32 : vec3<f32>,
	nodeUniform42 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform30 : vec3<f32>,
	nodeUniform31 : vec3<f32>,
	nodeUniform40 : vec3<f32>,
	nodeUniform41 : vec3<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform17 : f32,
	nodeUniform18 : vec2<f32>,
	nodeUniform19 : f32,
	nodeUniform23 : mat4x4<f32>,
	nodeUniform24 : f32,
	nodeUniform25 : f32,
	nodeUniform27 : f32,
	nodeUniform28 : vec2<f32>,
	nodeUniform29 : f32,
	nodeUniform33 : mat4x4<f32>,
	nodeUniform34 : f32,
	nodeUniform35 : f32,
	nodeUniform37 : f32,
	nodeUniform38 : vec2<f32>,
	nodeUniform39 : f32,
	nodeUniform43 : mat4x4<f32>,
	nodeUniform44 : f32,
	nodeUniform45 : f32,
	nodeUniform47 : f32,
	nodeUniform48 : vec2<f32>,
	nodeUniform49 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Shininess : f32;
var<private> SpecularColor : vec3<f32>;
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
var<private> directSpecular : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec4<f32>;
var<private> nodeVar37 : vec4<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : vec4<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : vec2<f32>;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : vec2<f32>;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec2<f32>;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : vec2<f32>;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : vec2<f32>;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec4<f32>;
var<private> nodeVar72 : vec4<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : vec4<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : vec2<f32>;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : vec2<f32>;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : vec2<f32>;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : vec2<f32>;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : vec2<f32>;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec4<f32>;
var<private> nodeVar107 : vec4<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : f32;
var<private> nodeVar111 : f32;
var<private> nodeVar112 : vec4<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : vec2<f32>;
var<private> nodeVar118 : f32;
var<private> nodeVar119 : vec2<f32>;
var<private> nodeVar120 : f32;
var<private> nodeVar121 : vec2<f32>;
var<private> nodeVar122 : f32;
var<private> nodeVar123 : vec2<f32>;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : vec2<f32>;
var<private> nodeVar126 : f32;
var<private> nodeVar127 : f32;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar140 : vec4<f32>;
var<private> nodeVar141 : vec4<f32>;
var<private> nodeVar142 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar143 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec4<f32>;

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
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) nodeVarying5 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Shininess = max( object.nodeUniform2, 0.0001 );
	SpecularColor = object.nodeUniform3;
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	nodeVar0 = ( render.nodeUniform9 - render.nodeUniform10 );
	nodeVar1 = vec4<f32>( nodeVar0, 0.0 );
	nodeVar2 = ( render.cameraViewMatrix * nodeVar1 );
	nodeVar3 = normalize( nodeVar2.xyz );
	nodeVar4 = nodeVar3;
	nodeVar5 = dot( normalView, nodeVar4 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar7 = ( render.nodeUniform13 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform14 ) ) ), 1.0 ) );
	nodeVar8 = ( nodeVar7.xyz / vec3<f32>( nodeVar7.w ) );
	nodeVar9 = vec3<f32>( nodeVar8.x, ( 1.0 - nodeVar8.y ), ( nodeVar8.z + render.nodeUniform15 ) );

	if ( ( ( ( ( ( nodeVar9.x >= 0.0 ) && ( nodeVar9.x <= 1.0 ) ) && ( nodeVar9.y >= 0.0 ) ) && ( nodeVar9.y <= 1.0 ) ) && ( nodeVar9.z <= 1.0 ) ) ) {

		nodeVar10 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar11 = ( render.nodeUniform17 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform18 ).x );
		nodeVar12 = ( nodeVar9.xy + ( vogelDiskSample( 0, 5, nodeVar10 ) * vec2<f32>( nodeVar11 ) ) );
		nodeVar13 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar12, nodeVar9.z );
		nodeVar14 = ( nodeVar9.xy + ( vogelDiskSample( 1, 5, nodeVar10 ) * vec2<f32>( nodeVar11 ) ) );
		nodeVar15 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar14, nodeVar9.z );
		nodeVar16 = ( nodeVar9.xy + ( vogelDiskSample( 2, 5, nodeVar10 ) * vec2<f32>( nodeVar11 ) ) );
		nodeVar17 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar16, nodeVar9.z );
		nodeVar18 = ( nodeVar9.xy + ( vogelDiskSample( 3, 5, nodeVar10 ) * vec2<f32>( nodeVar11 ) ) );
		nodeVar19 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar18, nodeVar9.z );
		nodeVar20 = ( nodeVar9.xy + ( vogelDiskSample( 4, 5, nodeVar10 ) * vec2<f32>( nodeVar11 ) ) );
		nodeVar21 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeVar20, nodeVar9.z );
		nodeVar6 = ( ( ( ( ( nodeVar13 + nodeVar15 ) + nodeVar17 ) + nodeVar19 ) + nodeVar21 ) * 0.2 );

	} else {

		nodeVar6 = 1.0;

	}

	nodeVar22 = mix( 1.0, nodeVar6, render.nodeUniform19 );
	nodeVar23 = ( render.nodeUniform11 * vec3<f32>( nodeVar22 ) );
	nodeVar24 = ( vec3<f32>( clamp( nodeVar5, 0.0, 1.0 ) ) * nodeVar23 );
	nodeVar25 = ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) );
	nodeVar26 = ( nodeVar24 * nodeVar25 );
	nodeVar27 = ( directDiffuse + nodeVar26 );
	directDiffuse = nodeVar27;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar28 = normalize( ( nodeVar4 + positionViewDirection ) );
	nodeVar29 = clamp( dot( positionViewDirection, nodeVar28 ), 0.0, 1.0 );
	nodeVar30 = exp2( ( ( ( nodeVar29 * -5.55473 ) - 6.98316 ) * nodeVar29 ) );
	nodeVar31 = ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar30 ) ) ) + vec3<f32>( ( 1.0 * nodeVar30 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeVar28 ), 0.0, 1.0 ), Shininess ) ) ) );
	nodeVar32 = ( nodeVar24 * nodeVar31 );
	nodeVar33 = ( nodeVar32 * vec3<f32>( 1.0 ) );
	nodeVar34 = ( directSpecular + nodeVar33 );
	directSpecular = nodeVar34;
	nodeVar35 = ( render.nodeUniform20 - render.nodeUniform21 );
	nodeVar36 = vec4<f32>( nodeVar35, 0.0 );
	nodeVar37 = ( render.cameraViewMatrix * nodeVar36 );
	nodeVar38 = normalize( nodeVar37.xyz );
	nodeVar39 = nodeVar38;
	nodeVar40 = dot( normalView, nodeVar39 );
	shadowPositionWorld = v_positionWorld;
	nodeVar42 = ( render.nodeUniform23 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform24 ) ) ), 1.0 ) );
	nodeVar43 = ( nodeVar42.xyz / vec3<f32>( nodeVar42.w ) );
	nodeVar44 = vec3<f32>( nodeVar43.x, ( 1.0 - nodeVar43.y ), ( nodeVar43.z + render.nodeUniform25 ) );

	if ( ( ( ( ( ( nodeVar44.x >= 0.0 ) && ( nodeVar44.x <= 1.0 ) ) && ( nodeVar44.y >= 0.0 ) ) && ( nodeVar44.y <= 1.0 ) ) && ( nodeVar44.z <= 1.0 ) ) ) {

		nodeVar45 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar46 = ( render.nodeUniform27 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform28 ).x );
		nodeVar47 = ( nodeVar44.xy + ( vogelDiskSample( 0, 5, nodeVar45 ) * vec2<f32>( nodeVar46 ) ) );
		nodeVar48 = textureSampleCompare( nodeUniform26, nodeUniform26_sampler, nodeVar47, nodeVar44.z );
		nodeVar49 = ( nodeVar44.xy + ( vogelDiskSample( 1, 5, nodeVar45 ) * vec2<f32>( nodeVar46 ) ) );
		nodeVar50 = textureSampleCompare( nodeUniform26, nodeUniform26_sampler, nodeVar49, nodeVar44.z );
		nodeVar51 = ( nodeVar44.xy + ( vogelDiskSample( 2, 5, nodeVar45 ) * vec2<f32>( nodeVar46 ) ) );
		nodeVar52 = textureSampleCompare( nodeUniform26, nodeUniform26_sampler, nodeVar51, nodeVar44.z );
		nodeVar53 = ( nodeVar44.xy + ( vogelDiskSample( 3, 5, nodeVar45 ) * vec2<f32>( nodeVar46 ) ) );
		nodeVar54 = textureSampleCompare( nodeUniform26, nodeUniform26_sampler, nodeVar53, nodeVar44.z );
		nodeVar55 = ( nodeVar44.xy + ( vogelDiskSample( 4, 5, nodeVar45 ) * vec2<f32>( nodeVar46 ) ) );
		nodeVar56 = textureSampleCompare( nodeUniform26, nodeUniform26_sampler, nodeVar55, nodeVar44.z );
		nodeVar41 = ( ( ( ( ( nodeVar48 + nodeVar50 ) + nodeVar52 ) + nodeVar54 ) + nodeVar56 ) * 0.2 );

	} else {

		nodeVar41 = 1.0;

	}

	nodeVar57 = mix( 1.0, nodeVar41, render.nodeUniform29 );
	nodeVar58 = ( render.nodeUniform22 * vec3<f32>( nodeVar57 ) );
	nodeVar59 = ( vec3<f32>( clamp( nodeVar40, 0.0, 1.0 ) ) * nodeVar58 );
	nodeVar60 = ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) );
	nodeVar61 = ( nodeVar59 * nodeVar60 );
	nodeVar62 = ( directDiffuse + nodeVar61 );
	directDiffuse = nodeVar62;
	nodeVar63 = normalize( ( nodeVar39 + positionViewDirection ) );
	nodeVar64 = clamp( dot( positionViewDirection, nodeVar63 ), 0.0, 1.0 );
	nodeVar65 = exp2( ( ( ( nodeVar64 * -5.55473 ) - 6.98316 ) * nodeVar64 ) );
	nodeVar66 = ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar65 ) ) ) + vec3<f32>( ( 1.0 * nodeVar65 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeVar63 ), 0.0, 1.0 ), Shininess ) ) ) );
	nodeVar67 = ( nodeVar59 * nodeVar66 );
	nodeVar68 = ( nodeVar67 * vec3<f32>( 1.0 ) );
	nodeVar69 = ( directSpecular + nodeVar68 );
	directSpecular = nodeVar69;
	nodeVar70 = ( render.nodeUniform30 - render.nodeUniform31 );
	nodeVar71 = vec4<f32>( nodeVar70, 0.0 );
	nodeVar72 = ( render.cameraViewMatrix * nodeVar71 );
	nodeVar73 = normalize( nodeVar72.xyz );
	nodeVar74 = nodeVar73;
	nodeVar75 = dot( normalView, nodeVar74 );
	shadowPositionWorld = v_positionWorld;
	nodeVar77 = ( render.nodeUniform33 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform34 ) ) ), 1.0 ) );
	nodeVar78 = ( nodeVar77.xyz / vec3<f32>( nodeVar77.w ) );
	nodeVar79 = vec3<f32>( nodeVar78.x, ( 1.0 - nodeVar78.y ), ( nodeVar78.z + render.nodeUniform35 ) );

	if ( ( ( ( ( ( nodeVar79.x >= 0.0 ) && ( nodeVar79.x <= 1.0 ) ) && ( nodeVar79.y >= 0.0 ) ) && ( nodeVar79.y <= 1.0 ) ) && ( nodeVar79.z <= 1.0 ) ) ) {

		nodeVar80 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar81 = ( render.nodeUniform37 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform38 ).x );
		nodeVar82 = ( nodeVar79.xy + ( vogelDiskSample( 0, 5, nodeVar80 ) * vec2<f32>( nodeVar81 ) ) );
		nodeVar83 = textureSampleCompare( nodeUniform36, nodeUniform36_sampler, nodeVar82, nodeVar79.z );
		nodeVar84 = ( nodeVar79.xy + ( vogelDiskSample( 1, 5, nodeVar80 ) * vec2<f32>( nodeVar81 ) ) );
		nodeVar85 = textureSampleCompare( nodeUniform36, nodeUniform36_sampler, nodeVar84, nodeVar79.z );
		nodeVar86 = ( nodeVar79.xy + ( vogelDiskSample( 2, 5, nodeVar80 ) * vec2<f32>( nodeVar81 ) ) );
		nodeVar87 = textureSampleCompare( nodeUniform36, nodeUniform36_sampler, nodeVar86, nodeVar79.z );
		nodeVar88 = ( nodeVar79.xy + ( vogelDiskSample( 3, 5, nodeVar80 ) * vec2<f32>( nodeVar81 ) ) );
		nodeVar89 = textureSampleCompare( nodeUniform36, nodeUniform36_sampler, nodeVar88, nodeVar79.z );
		nodeVar90 = ( nodeVar79.xy + ( vogelDiskSample( 4, 5, nodeVar80 ) * vec2<f32>( nodeVar81 ) ) );
		nodeVar91 = textureSampleCompare( nodeUniform36, nodeUniform36_sampler, nodeVar90, nodeVar79.z );
		nodeVar76 = ( ( ( ( ( nodeVar83 + nodeVar85 ) + nodeVar87 ) + nodeVar89 ) + nodeVar91 ) * 0.2 );

	} else {

		nodeVar76 = 1.0;

	}

	nodeVar92 = mix( 1.0, nodeVar76, render.nodeUniform39 );
	nodeVar93 = ( render.nodeUniform32 * vec3<f32>( nodeVar92 ) );
	nodeVar94 = ( vec3<f32>( clamp( nodeVar75, 0.0, 1.0 ) ) * nodeVar93 );
	nodeVar95 = ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) );
	nodeVar96 = ( nodeVar94 * nodeVar95 );
	nodeVar97 = ( directDiffuse + nodeVar96 );
	directDiffuse = nodeVar97;
	nodeVar98 = normalize( ( nodeVar74 + positionViewDirection ) );
	nodeVar99 = clamp( dot( positionViewDirection, nodeVar98 ), 0.0, 1.0 );
	nodeVar100 = exp2( ( ( ( nodeVar99 * -5.55473 ) - 6.98316 ) * nodeVar99 ) );
	nodeVar101 = ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar100 ) ) ) + vec3<f32>( ( 1.0 * nodeVar100 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeVar98 ), 0.0, 1.0 ), Shininess ) ) ) );
	nodeVar102 = ( nodeVar94 * nodeVar101 );
	nodeVar103 = ( nodeVar102 * vec3<f32>( 1.0 ) );
	nodeVar104 = ( directSpecular + nodeVar103 );
	directSpecular = nodeVar104;
	nodeVar105 = ( render.nodeUniform40 - render.nodeUniform41 );
	nodeVar106 = vec4<f32>( nodeVar105, 0.0 );
	nodeVar107 = ( render.cameraViewMatrix * nodeVar106 );
	nodeVar108 = normalize( nodeVar107.xyz );
	nodeVar109 = nodeVar108;
	nodeVar110 = dot( normalView, nodeVar109 );
	shadowPositionWorld = v_positionWorld;
	nodeVar112 = ( render.nodeUniform43 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform44 ) ) ), 1.0 ) );
	nodeVar113 = ( nodeVar112.xyz / vec3<f32>( nodeVar112.w ) );
	nodeVar114 = vec3<f32>( nodeVar113.x, ( 1.0 - nodeVar113.y ), ( nodeVar113.z + render.nodeUniform45 ) );

	if ( ( ( ( ( ( nodeVar114.x >= 0.0 ) && ( nodeVar114.x <= 1.0 ) ) && ( nodeVar114.y >= 0.0 ) ) && ( nodeVar114.y <= 1.0 ) ) && ( nodeVar114.z <= 1.0 ) ) ) {

		nodeVar115 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar116 = ( render.nodeUniform47 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform48 ).x );
		nodeVar117 = ( nodeVar114.xy + ( vogelDiskSample( 0, 5, nodeVar115 ) * vec2<f32>( nodeVar116 ) ) );
		nodeVar118 = textureSampleCompare( nodeUniform46, nodeUniform46_sampler, nodeVar117, nodeVar114.z );
		nodeVar119 = ( nodeVar114.xy + ( vogelDiskSample( 1, 5, nodeVar115 ) * vec2<f32>( nodeVar116 ) ) );
		nodeVar120 = textureSampleCompare( nodeUniform46, nodeUniform46_sampler, nodeVar119, nodeVar114.z );
		nodeVar121 = ( nodeVar114.xy + ( vogelDiskSample( 2, 5, nodeVar115 ) * vec2<f32>( nodeVar116 ) ) );
		nodeVar122 = textureSampleCompare( nodeUniform46, nodeUniform46_sampler, nodeVar121, nodeVar114.z );
		nodeVar123 = ( nodeVar114.xy + ( vogelDiskSample( 3, 5, nodeVar115 ) * vec2<f32>( nodeVar116 ) ) );
		nodeVar124 = textureSampleCompare( nodeUniform46, nodeUniform46_sampler, nodeVar123, nodeVar114.z );
		nodeVar125 = ( nodeVar114.xy + ( vogelDiskSample( 4, 5, nodeVar115 ) * vec2<f32>( nodeVar116 ) ) );
		nodeVar126 = textureSampleCompare( nodeUniform46, nodeUniform46_sampler, nodeVar125, nodeVar114.z );
		nodeVar111 = ( ( ( ( ( nodeVar118 + nodeVar120 ) + nodeVar122 ) + nodeVar124 ) + nodeVar126 ) * 0.2 );

	} else {

		nodeVar111 = 1.0;

	}

	nodeVar127 = mix( 1.0, nodeVar111, render.nodeUniform49 );
	nodeVar128 = ( render.nodeUniform42 * vec3<f32>( nodeVar127 ) );
	nodeVar129 = ( vec3<f32>( clamp( nodeVar110, 0.0, 1.0 ) ) * nodeVar128 );
	nodeVar130 = ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) );
	nodeVar131 = ( nodeVar129 * nodeVar130 );
	nodeVar132 = ( directDiffuse + nodeVar131 );
	directDiffuse = nodeVar132;
	nodeVar133 = normalize( ( nodeVar109 + positionViewDirection ) );
	nodeVar134 = clamp( dot( positionViewDirection, nodeVar133 ), 0.0, 1.0 );
	nodeVar135 = exp2( ( ( ( nodeVar134 * -5.55473 ) - 6.98316 ) * nodeVar134 ) );
	nodeVar136 = ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar135 ) ) ) + vec3<f32>( ( 1.0 * nodeVar135 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeVar133 ), 0.0, 1.0 ), Shininess ) ) ) );
	nodeVar137 = ( nodeVar129 * nodeVar136 );
	nodeVar138 = ( nodeVar137 * vec3<f32>( 1.0 ) );
	nodeVar139 = ( directSpecular + nodeVar138 );
	directSpecular = nodeVar139;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar140 = ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) );
	nodeVar141 = ( vec4<f32>( irradiance, 1.0 ) * nodeVar140 );
	nodeVar142 = ( vec4<f32>( indirectDiffuse, 1.0 ) + nodeVar141 );
	indirectDiffuse = nodeVar142.xyz;
	ambientOcclusion = 1.0;
	nodeVar143 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar143;
	nodeVar144 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar144;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar145 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar145;
	nodeVar146 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar146;
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar147 = textureSample( nodeUniform50, nodeUniform50_sampler, ( object.nodeUniform51 * vec3<f32>( nodeVarying5, 1.0 ) ).xy );

	// result

	output.color = mix( nodeVar147, Output, ( 1.0 / object.nodeUniform52 ) );

	return output;

}
