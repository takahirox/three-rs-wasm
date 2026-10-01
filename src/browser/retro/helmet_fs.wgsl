// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform1_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform4_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform4 : texture_cube<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform13_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform13 : texture_2d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform17_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform17 : texture_2d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform20_sampler : sampler;
@binding( 10 ) @group( 1 ) var nodeUniform20 : texture_2d<f32>;
@binding( 11 ) @group( 1 ) var nodeUniform27_sampler : sampler;
@binding( 12 ) @group( 1 ) var nodeUniform27 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform5 : mat4x4<f32>,
	nodeUniform8 : mat4x4<f32>,
	nodeUniform12 : mat3x3<f32>,
	nodeUniform14 : mat3x3<f32>,
	nodeUniform15 : vec2<f32>,
	nodeUniform16 : f32,
	nodeUniform18 : mat3x3<f32>,
	nodeUniform19 : f32,
	nodeUniform21 : mat3x3<f32>,
	nodeUniform22 : f32,
	nodeUniform23 : f32,
	nodeUniform24 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform26 : f32,
	nodeUniform28 : mat3x3<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform29 : vec3<f32>,
	nodeUniform33 : vec3<f32>,
	nodeUniform35 : vec3<f32>,
	nodeUniform36 : f32,
	nodeUniform37 : f32,
	nodeUniform31 : vec3<f32>,
	nodeUniform32 : vec3<f32>,
	nodeUniform34 : vec3<f32>,
	cameraProjectionMatrixInverse : mat4x4<f32>,
	nodeUniform11 : vec2<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> positionView : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec2<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec2<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> tangentViewFrame : vec3<f32>;
var<private> NORMAL_tangentView : vec3<f32>;
var<private> bitangentViewFrame : vec3<f32>;
var<private> NORMAL_bitangentView : vec3<f32>;
var<private> NORMAL_TBNViewMatrix : mat3x3<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> normalView : vec3<f32>;
var<private> reflectVector : vec3<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> AmbientOcclusion : f32;
var<private> nodeVar17 : vec4<f32>;
var<private> Shininess : f32;
var<private> SpecularColor : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> nodeVar18 : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec4<f32>;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar56 : f32;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar57 : vec4<f32>;
var<private> nodeVar58 : vec4<f32>;
var<private> nodeVar59 : vec4<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) nodeVarying2 : vec2<f32>,
	@location( 1 ) nodeVarying3 : f32,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_clipSpace : vec4<f32>,
	@location( 4 ) v_normalViewGeometry : vec3<f32>,
	@location( 5 ) nodeVarying8 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSampleLevel( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( mix( nodeVarying8, ( nodeVarying2 / vec2<f32>( nodeVarying3 ) ), object.nodeUniform3 ), 1.0 ) ).xy, 0.0 );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar3 = ( render.cameraProjectionMatrixInverse * v_clipSpace );
	positionView = ( nodeVar3.xyz / vec3<f32>( nodeVar3.w ) );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	nodeVar4 = cross( - dpdy( positionView ), NORMAL_normalView );
	nodeVar5 = dpdx( nodeVarying8 );
	nodeVar6 = cross( NORMAL_normalView, dpdx( positionView ) );
	nodeVar7 = - dpdy( nodeVarying8 );
	nodeVar8 = ( ( nodeVar4 * vec3<f32>( nodeVar5.x ) ) + ( nodeVar6 * vec3<f32>( nodeVar7.x ) ) );
	nodeVar10 = ( ( nodeVar4 * vec3<f32>( nodeVar5.y ) ) + ( nodeVar6 * vec3<f32>( nodeVar7.y ) ) );
	nodeVar11 = max( dot( nodeVar8, nodeVar8 ), dot( nodeVar10, nodeVar10 ) );

	if ( ( nodeVar11 == 0.0 ) ) {

		nodeVar9 = 0.0;

	} else {

		nodeVar9 = inverseSqrt( nodeVar11 );

	}

	tangentViewFrame = ( nodeVar8 * vec3<f32>( nodeVar9 ) );
	NORMAL_tangentView = tangentViewFrame;
	bitangentViewFrame = ( nodeVar10 * nodeVar9 );
	NORMAL_bitangentView = bitangentViewFrame;
	NORMAL_TBNViewMatrix = mat3x3<f32>( NORMAL_tangentView, NORMAL_bitangentView, NORMAL_normalView );
	nodeVar12 = textureSample( nodeUniform13, nodeUniform13_sampler, ( object.nodeUniform14 * vec3<f32>( nodeVarying8, 1.0 ) ).xy );
	nodeVar13 = ( ( nodeVar12 * vec4<f32>( 2.0 ) ) - vec4<f32>( 1.0 ) );
	normalView = normalize( ( NORMAL_TBNViewMatrix * vec3<f32>( ( nodeVar13.xy * object.nodeUniform15 ), nodeVar13.z ) ) );
	reflectVector = normalize( ( render.cameraWorldMatrix * vec4<f32>( reflect( ( - positionViewDirection ), normalView ), 0.0 ) ).xyz );
	nodeVar14 = ( object.nodeUniform5 * vec4<f32>( reflectVector, 1.0 ) );
	nodeVar15 = textureSampleLevel( nodeUniform4, nodeUniform4_sampler, vec3<f32>( ( - nodeVar14.x ), nodeVar14.yz ), 0.0 );
	nodeVar16 = textureSampleLevel( nodeUniform17, nodeUniform17_sampler, ( object.nodeUniform18 * vec3<f32>( mix( nodeVarying8, ( nodeVarying2 / vec2<f32>( nodeVarying3 ) ), object.nodeUniform3 ), 1.0 ) ).xy, 0.0 );
	DiffuseColor = mix( ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 ), nodeVar15, ( object.nodeUniform16 * nodeVar16.z ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform19 );
	DiffuseColor.w = 1.0;
	nodeVar17 = textureSampleLevel( nodeUniform20, nodeUniform20_sampler, ( object.nodeUniform21 * vec3<f32>( mix( nodeVarying8, ( nodeVarying2 / vec2<f32>( nodeVarying3 ) ), object.nodeUniform3 ), 1.0 ) ).xy, 0.0 );
	AmbientOcclusion = ( ( ( nodeVar17.x - 1.0 ) * object.nodeUniform22 ) + 1.0 );
	Shininess = max( object.nodeUniform23, 0.0001 );
	SpecularColor = object.nodeUniform24;
	nodeVar18 = textureSampleLevel( nodeUniform27, nodeUniform27_sampler, ( object.nodeUniform28 * vec3<f32>( mix( nodeVarying8, ( nodeVarying2 / vec2<f32>( nodeVarying3 ) ), object.nodeUniform3 ), 1.0 ) ).xy, 0.0 );
	EmissiveColor = ( vec4<f32>( ( object.nodeUniform25 * vec3<f32>( object.nodeUniform26 ) ), 1.0 ) * nodeVar18 ).xyz;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar19 = ( irradiance + render.nodeUniform29 );
	irradiance = nodeVar19;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar20 = ( render.nodeUniform31 - render.nodeUniform32 );
	nodeVar21 = vec4<f32>( nodeVar20, 0.0 );
	nodeVar22 = ( render.cameraViewMatrix * nodeVar21 );
	nodeVar23 = normalize( nodeVar22.xyz );
	nodeVar24 = nodeVar23;
	nodeVar25 = dot( normalView, nodeVar24 );
	nodeVar26 = ( vec3<f32>( clamp( nodeVar25, 0.0, 1.0 ) ) * render.nodeUniform33 );
	nodeVar27 = ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) );
	nodeVar28 = ( nodeVar26 * nodeVar27 );
	nodeVar29 = ( directDiffuse + nodeVar28 );
	directDiffuse = nodeVar29;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar30 = normalize( ( nodeVar24 + positionViewDirection ) );
	nodeVar31 = clamp( dot( positionViewDirection, nodeVar30 ), 0.0, 1.0 );
	nodeVar32 = exp2( ( ( ( nodeVar31 * -5.55473 ) - 6.98316 ) * nodeVar31 ) );
	nodeVar33 = ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar32 ) ) ) + vec3<f32>( ( 1.0 * nodeVar32 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeVar30 ), 0.0, 1.0 ), Shininess ) ) ) );
	nodeVar34 = ( nodeVar26 * nodeVar33 );
	nodeVar35 = ( nodeVar34 * vec3<f32>( 1.0 ) );
	nodeVar36 = ( directSpecular + nodeVar35 );
	directSpecular = nodeVar36;
	nodeVar37 = ( render.nodeUniform34 - positionView );
	nodeVar38 = normalize( nodeVar37 );
	nodeVar39 = dot( normalView, nodeVar38 );

	if ( ( render.nodeUniform36 > 0.0 ) ) {

		nodeVar41 = length( nodeVar37 );
		nodeVar42 = ( nodeVar41 / render.nodeUniform36 );
		nodeVar43 = clamp( ( 1.0 - ( ( ( nodeVar42 * nodeVar42 ) * nodeVar42 ) * nodeVar42 ) ), 0.0, 1.0 );
		nodeVar40 = ( ( 1.0 / max( pow( nodeVar41, render.nodeUniform37 ), 0.01 ) ) * ( nodeVar43 * nodeVar43 ) );

	} else {

		nodeVar40 = ( 1.0 / max( pow( length( nodeVar37 ), render.nodeUniform37 ), 0.01 ) );

	}

	nodeVar44 = ( render.nodeUniform35 * vec3<f32>( nodeVar40 ) );
	nodeVar45 = ( vec3<f32>( clamp( nodeVar39, 0.0, 1.0 ) ) * nodeVar44 );
	nodeVar46 = ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) );
	nodeVar47 = ( nodeVar45 * nodeVar46 );
	nodeVar48 = ( directDiffuse + nodeVar47 );
	directDiffuse = nodeVar48;
	nodeVar49 = normalize( ( nodeVar38 + positionViewDirection ) );
	nodeVar50 = clamp( dot( positionViewDirection, nodeVar49 ), 0.0, 1.0 );
	nodeVar51 = exp2( ( ( ( nodeVar50 * -5.55473 ) - 6.98316 ) * nodeVar50 ) );
	nodeVar52 = ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar51 ) ) ) + vec3<f32>( ( 1.0 * nodeVar51 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeVar49 ), 0.0, 1.0 ), Shininess ) ) ) );
	nodeVar53 = ( nodeVar45 * nodeVar52 );
	nodeVar54 = ( nodeVar53 * vec3<f32>( 1.0 ) );
	nodeVar55 = ( directSpecular + nodeVar54 );
	directSpecular = nodeVar55;
	ambientOcclusion = 1.0;
	nodeVar56 = ( ambientOcclusion * AmbientOcclusion );
	ambientOcclusion = nodeVar56;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar57 = ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) );
	nodeVar58 = ( vec4<f32>( irradiance, 1.0 ) * nodeVar57 );
	nodeVar59 = ( vec4<f32>( indirectDiffuse, 1.0 ) + nodeVar58 );
	indirectDiffuse = nodeVar59.xyz;
	nodeVar60 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar60;
	nodeVar61 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar61;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar62 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar62;
	nodeVar63 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar63;
	nodeVar64 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar64;

	// result

	output.color = nodeVar64;

	return output;

}
