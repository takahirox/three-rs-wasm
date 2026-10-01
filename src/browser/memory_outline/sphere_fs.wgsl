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

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform13 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
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
var<private> indirectDiffuse : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec4<f32>;

// codes

@fragment
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) nodeVarying5 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying5, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	nodeVar1 = ( render.nodeUniform9 - render.nodeUniform10 );
	nodeVar2 = vec4<f32>( nodeVar1, 0.0 );
	nodeVar3 = ( render.cameraViewMatrix * nodeVar2 );
	nodeVar4 = normalize( nodeVar3.xyz );
	nodeVar5 = nodeVar4;
	nodeVar6 = dot( normalView, nodeVar5 );
	nodeVar7 = ( vec3<f32>( clamp( nodeVar6, 0.0, 1.0 ) ) * render.nodeUniform11 );
	nodeVar8 = ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) );
	nodeVar9 = ( nodeVar7 * nodeVar8 );
	nodeVar10 = ( directDiffuse + nodeVar9 );
	directDiffuse = nodeVar10;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar11 = ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) );
	nodeVar12 = ( vec4<f32>( irradiance, 1.0 ) * nodeVar11 );
	nodeVar13 = ( vec4<f32>( indirectDiffuse, 1.0 ) + nodeVar12 );
	indirectDiffuse = nodeVar13.xyz;
	ambientOcclusion = 1.0;
	nodeVar14 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar14;
	nodeVar15 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar15;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar16 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar16;
	nodeVar17 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar17;
	nodeVar18 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar18;

	// result

	output.color = nodeVar18;

	return output;

}
