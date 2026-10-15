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
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : vec3<f32>,
	nodeUniform8 : f32,
	nodeUniform11 : mat3x3<f32>,
	nodeUniform16 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform17 : vec3<f32>,
	nodeUniform18 : f32,
	nodeUniform19 : f32
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
var<private> nodeVar7 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar25 : vec4<f32>;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar28 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) @interpolate(flat, either) vBatchIndirectId : u32,
	@location( 4 ) vBatchColor : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = ( vBatchColor * vec4<f32>( object.nodeUniform3, 1.0 ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform4 );
	DiffuseColor.w = 1.0;
	Shininess = max( object.nodeUniform5, 0.0001 );
	SpecularColor = object.nodeUniform6;
	EmissiveColor = ( object.nodeUniform7 * vec3<f32>( object.nodeUniform8 ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar7 = ( irradiance + render.nodeUniform9 );
	irradiance = nodeVar7;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	nodeVar8 = ( render.nodeUniform13 - render.nodeUniform14 );
	nodeVar9 = vec4<f32>( nodeVar8, 0.0 );
	nodeVar10 = ( render.cameraViewMatrix * nodeVar9 );
	nodeVar11 = normalize( nodeVar10.xyz );
	nodeVar12 = nodeVar11;
	nodeVar13 = dot( normalView, nodeVar12 );
	nodeVar14 = ( vec3<f32>( clamp( nodeVar13, 0.0, 1.0 ) ) * render.nodeUniform15 );
	nodeVar15 = ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) );
	nodeVar16 = ( nodeVar14 * nodeVar15 );
	nodeVar17 = ( directDiffuse + nodeVar16 );
	directDiffuse = nodeVar17;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar18 = normalize( ( nodeVar12 + positionViewDirection ) );
	nodeVar19 = clamp( dot( positionViewDirection, nodeVar18 ), 0.0, 1.0 );
	nodeVar20 = exp2( ( ( ( nodeVar19 * -5.55473 ) - 6.98316 ) * nodeVar19 ) );
	nodeVar21 = ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar20 ) ) ) + vec3<f32>( ( 1.0 * nodeVar20 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeVar18 ), 0.0, 1.0 ), Shininess ) ) ) );
	nodeVar22 = ( nodeVar14 * nodeVar21 );
	nodeVar23 = ( nodeVar22 * vec3<f32>( 1.0 ) );
	nodeVar24 = ( directSpecular + nodeVar23 );
	directSpecular = nodeVar24;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar25 = ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) );
	nodeVar26 = ( vec4<f32>( irradiance, 1.0 ) * nodeVar25 );
	nodeVar27 = ( vec4<f32>( indirectDiffuse, 1.0 ) + nodeVar26 );
	indirectDiffuse = nodeVar27.xyz;
	ambientOcclusion = 1.0;
	nodeVar28 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar28;
	nodeVar29 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar29;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar30 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar30;
	nodeVar31 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar31;
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar32 = vec4<f32>( mix( Output.xyz, render.nodeUniform17, smoothstep( render.nodeUniform18, render.nodeUniform19, ( - v_positionView.z ) ) ), Output.w );
	Output = nodeVar32;

	// result

	output.color = nodeVar32;

	return output;

}
