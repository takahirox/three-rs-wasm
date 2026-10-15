// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform2_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform2 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform5_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform5 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform8 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform3 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> AmbientOcclusion : f32;
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> Output : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	nodeVar0 = textureSample( nodeUniform2, nodeUniform2_sampler, ( fragCoord.xy / render.nodeUniform3 ) ).x;
	nodeVar1 = max( nodeVar0, 0.001 );
	AmbientOcclusion = nodeVar1;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar2 = ( irradiance + render.nodeUniform4 );
	irradiance = nodeVar2;
	ambientOcclusion = 1.0;
	nodeVar3 = ( ambientOcclusion * AmbientOcclusion );
	ambientOcclusion = nodeVar3;
	nodeVar4 = textureSample( nodeUniform5, nodeUniform5_sampler, ( fragCoord.xy / render.nodeUniform3 ) );
	nodeVar5 = ( irradiance + ( nodeVar4.xyz / vec3<f32>( nodeVar1 ) ) );
	irradiance = nodeVar5;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = vec4<f32>( 0.0, 0.0, 0.0, 0.0 ).xyz;
	nodeVar6 = ( vec4<f32>( indirectDiffuse, 1.0 ) + vec4<f32>( 1.0, 1.0, 1.0, 0.0 ) );
	indirectDiffuse = nodeVar6.xyz;
	nodeVar7 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar7;
	nodeVar8 = ( indirectDiffuse * DiffuseColor.xyz );
	indirectDiffuse = nodeVar8;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar9 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar9;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar10 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar10;
	nodeVar11 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar11;
	nodeVar12 = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar12;

	// result

	output.color = nodeVar12;

	return output;

}
