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
	nodeUniform4 : f32,
	nodeUniform8 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform9 : vec2<f32>,
	nodeUniform5 : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) nodeVarying2 : vec2<f32>,
	@location( 1 ) nodeVarying3 : f32,
	@location( 2 ) v_clipSpace : vec4<f32>,
	@location( 3 ) nodeVarying5 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSampleLevel( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( mix( nodeVarying5, ( nodeVarying2 / vec2<f32>( nodeVarying3 ) ), object.nodeUniform3 ), 1.0 ) ).xy, 0.0 );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform4 );
	DiffuseColor.w = 1.0;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar1 = ( irradiance + render.nodeUniform5 );
	irradiance = nodeVar1;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = vec4<f32>( 0.0, 0.0, 0.0, 0.0 ).xyz;
	nodeVar2 = ( vec4<f32>( indirectDiffuse, 1.0 ) + vec4<f32>( 1.0, 1.0, 1.0, 0.0 ) );
	indirectDiffuse = nodeVar2.xyz;
	ambientOcclusion = 1.0;
	nodeVar3 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar3;
	nodeVar4 = ( indirectDiffuse * DiffuseColor.xyz );
	indirectDiffuse = nodeVar4;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar5 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar5;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar6 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar6;
	nodeVar7 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar7;
	nodeVar8 = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar8;

	// result

	output.color = nodeVar8;

	return output;

}
