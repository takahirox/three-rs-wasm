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
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform5 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
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
var<private> nodeVar7 : vec4<f32>;

// codes

@fragment
fn main(  ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
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
	nodeVar7 = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar7;

	// result

	output.color = nodeVar7;

	return output;

}
