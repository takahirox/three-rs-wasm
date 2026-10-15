// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform6_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform6 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform8 : f32,
	nodeUniform13 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraViewMatrix : mat4x4<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : f32,
	nodeUniform11 : f32
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
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar6 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) nodeVarying4 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Shininess = max( object.nodeUniform2, 0.0001 );
	SpecularColor = object.nodeUniform3;
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar0 = textureSample( nodeUniform6, nodeUniform6_sampler, ( object.nodeUniform7 * vec3<f32>( nodeVarying4, 1.0 ) ).xy );
	nodeVar1 = ( nodeVar0.xyz * vec3<f32>( object.nodeUniform8 ) );
	nodeVar2 = ( irradiance + nodeVar1 );
	irradiance = nodeVar2;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar3 = ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) );
	nodeVar4 = ( vec4<f32>( irradiance, 1.0 ) * nodeVar3 );
	nodeVar5 = ( vec4<f32>( indirectDiffuse, 1.0 ) + nodeVar4 );
	indirectDiffuse = nodeVar5.xyz;
	ambientOcclusion = 1.0;
	nodeVar6 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar6;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar7 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar7;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar8 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar8;
	nodeVar9 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar9;
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar10 = vec4<f32>( mix( Output.xyz, render.nodeUniform9, smoothstep( render.nodeUniform10, render.nodeUniform11, ( - v_positionView.z ) ) ), Output.w );
	Output = nodeVar10;

	// result

	output.color = nodeVar10;

	return output;

}
