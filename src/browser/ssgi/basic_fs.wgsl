// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputType {
	@location( 0 ) m0 : vec4<f32>,
	@location( 1 ) m1 : vec4<f32>,
	@location( 2 ) m2 : vec4<f32>,
	@location( 3 ) m3 : vec4<f32>,
	
};
var<private> output : OutputType;

// uniforms

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform5 : mat4x4<f32>,
	nodeUniform7 : mat4x4<f32>,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform10 : mat4x4<f32>,
	nodeUniform12 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform8 : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform2 : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> modelViewMatrix : mat4x4<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : vec2<f32>;

// codes


@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) positionLocal : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) positionPrevious : vec3<f32> ) -> OutputType {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar0 = ( irradiance + render.nodeUniform2 );
	irradiance = nodeVar0;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = vec4<f32>( 0.0, 0.0, 0.0, 0.0 ).xyz;
	nodeVar1 = ( vec4<f32>( indirectDiffuse, 1.0 ) + vec4<f32>( 1.0, 1.0, 1.0, 0.0 ) );
	indirectDiffuse = nodeVar1.xyz;
	ambientOcclusion = 1.0;
	nodeVar2 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar2;
	nodeVar3 = ( indirectDiffuse * DiffuseColor.xyz );
	indirectDiffuse = nodeVar3;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar4 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar4;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar5 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar5;
	nodeVar6 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar6;
	Output = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	output.m0 = Output;
	output.m1 = DiffuseColor;
	normalViewGeometry = normalize( v_normalViewGeometry );
	normalView = normalViewGeometry;
	nodeVar7 = ( ( normalView * vec3<f32>( 0.5 ) ) + vec3<f32>( 0.5 ) );
	output.m2 = vec4<f32>( nodeVar7, 1.0 );
	modelViewMatrix = ( render.cameraViewMatrix * object.nodeUniform7 );
	nodeVar8 = ( ( object.nodeUniform5 * modelViewMatrix ) * vec4<f32>( positionLocal, 1.0 ) );
	nodeVar9 = ( ( render.nodeUniform8 * ( object.nodeUniform9 * object.nodeUniform10 ) ) * vec4<f32>( positionPrevious, 1.0 ) );
	nodeVar10 = ( ( nodeVar8.xy / vec2<f32>( nodeVar8.w ) ) - ( nodeVar9.xy / vec2<f32>( nodeVar9.w ) ) );
	output.m3 = vec4<f32>( vec3<f32>( nodeVar10, 0.0 ), 1.0 );

	// result

	return output;

}
