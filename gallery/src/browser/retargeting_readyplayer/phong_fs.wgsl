// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : vec3<f32>,
	nodeUniform8 : f32,
	nodeUniform12 : mat3x3<f32>,
	nodeUniform18 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform17 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform19 : vec3<f32>,
	nodeUniform20 : vec3<f32>
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
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec4<f32>;
var<private> nodeVar25 : vec4<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar40 : vec4<f32>;
var<private> nodeVar41 : vec4<f32>;
var<private> nodeVar42 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar43 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_positionViewDirection : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform3, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform4 );
	DiffuseColor.w = 1.0;
	Shininess = max( object.nodeUniform5, 0.0001 );
	SpecularColor = object.nodeUniform6;
	EmissiveColor = ( object.nodeUniform7 * vec3<f32>( object.nodeUniform8 ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar1 = dot( normalWorld, normalize( render.nodeUniform14 ) );
	nodeVar2 = ( nodeVar1 * 0.5 );
	nodeVar3 = ( nodeVar2 + 0.5 );
	nodeVar4 = mix( render.nodeUniform9, render.nodeUniform10, nodeVar3 );
	nodeVar5 = ( irradiance + nodeVar4 );
	irradiance = nodeVar5;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar6 = ( render.nodeUniform15 - render.nodeUniform16 );
	nodeVar7 = vec4<f32>( nodeVar6, 0.0 );
	nodeVar8 = ( render.cameraViewMatrix * nodeVar7 );
	nodeVar9 = normalize( nodeVar8.xyz );
	nodeVar10 = nodeVar9;
	nodeVar11 = dot( normalView, nodeVar10 );
	nodeVar12 = ( vec3<f32>( clamp( nodeVar11, 0.0, 1.0 ) ) * render.nodeUniform17 );
	nodeVar13 = ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) );
	nodeVar14 = ( nodeVar12 * nodeVar13 );
	nodeVar15 = ( directDiffuse + nodeVar14 );
	directDiffuse = nodeVar15;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar16 = normalize( ( nodeVar10 + positionViewDirection ) );
	nodeVar17 = clamp( dot( positionViewDirection, nodeVar16 ), 0.0, 1.0 );
	nodeVar18 = exp2( ( ( ( nodeVar17 * -5.55473 ) - 6.98316 ) * nodeVar17 ) );
	nodeVar19 = ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar18 ) ) ) + vec3<f32>( ( 1.0 * nodeVar18 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeVar16 ), 0.0, 1.0 ), Shininess ) ) ) );
	nodeVar20 = ( nodeVar12 * nodeVar19 );
	nodeVar21 = ( nodeVar20 * vec3<f32>( 1.0 ) );
	nodeVar22 = ( directSpecular + nodeVar21 );
	directSpecular = nodeVar22;
	nodeVar23 = ( render.nodeUniform19 - render.nodeUniform20 );
	nodeVar24 = vec4<f32>( nodeVar23, 0.0 );
	nodeVar25 = ( render.cameraViewMatrix * nodeVar24 );
	nodeVar26 = normalize( nodeVar25.xyz );
	nodeVar27 = nodeVar26;
	nodeVar28 = dot( normalView, nodeVar27 );
	nodeVar29 = ( vec3<f32>( clamp( nodeVar28, 0.0, 1.0 ) ) * render.nodeUniform21 );
	nodeVar30 = ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) );
	nodeVar31 = ( nodeVar29 * nodeVar30 );
	nodeVar32 = ( directDiffuse + nodeVar31 );
	directDiffuse = nodeVar32;
	nodeVar33 = normalize( ( nodeVar27 + positionViewDirection ) );
	nodeVar34 = clamp( dot( positionViewDirection, nodeVar33 ), 0.0, 1.0 );
	nodeVar35 = exp2( ( ( ( nodeVar34 * -5.55473 ) - 6.98316 ) * nodeVar34 ) );
	nodeVar36 = ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar35 ) ) ) + vec3<f32>( ( 1.0 * nodeVar35 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeVar33 ), 0.0, 1.0 ), Shininess ) ) ) );
	nodeVar37 = ( nodeVar29 * nodeVar36 );
	nodeVar38 = ( nodeVar37 * vec3<f32>( 1.0 ) );
	nodeVar39 = ( directSpecular + nodeVar38 );
	directSpecular = nodeVar39;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar40 = ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) );
	nodeVar41 = ( vec4<f32>( irradiance, 1.0 ) * nodeVar40 );
	nodeVar42 = ( vec4<f32>( indirectDiffuse, 1.0 ) + nodeVar41 );
	indirectDiffuse = nodeVar42.xyz;
	ambientOcclusion = 1.0;
	nodeVar43 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar43;
	nodeVar44 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar44;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar45 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar45;
	nodeVar46 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar46;
	nodeVar47 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar47;

	// result

	output.color = nodeVar47;

	return output;

}
