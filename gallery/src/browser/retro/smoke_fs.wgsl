// Three.js r187 - Node System

// global
diagnostic( off, derivative_uniformity );


// directives


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 1 ) var nodeUniform2_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform2 : texture_2d<f32>;

struct renderStruct {
	nodeUniform1 : f32,
	nodeUniform3 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform5 : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform4 : f32,
	nodeUniform8 : mat4x4<f32>
};
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes


@fragment
fn main( @location( 0 ) nodeVarying4 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar4 = textureSample( nodeUniform2, nodeUniform2_sampler, ( ( nodeVarying4 * vec2<f32>( 0.5, 0.3 ) ) + vec2<f32>( 0.0, ( - ( render.nodeUniform3 * 0.03 ) ) ) ) );
	let nodeConst0 = ( ( ( ( smoothstep( 0.4, 1.0, nodeVar4.x ) * smoothstep( 0.0, 0.1, nodeVarying4.x ) ) * smoothstep( 0.0, 0.1, ( 1.0 - nodeVarying4.x ) ) ) * smoothstep( 0.0, 0.1, nodeVarying4.y ) ) * smoothstep( 0.0, 0.1, ( 1.0 - nodeVarying4.y ) ) );
	DiffuseColor = vec4<f32>( mix( vec3<f32>( 0.6, 0.3, 0.2 ), vec3<f32>( 1.0, 1.0, 1.0 ), pow( nodeConst0, 3.0 ) ), nodeConst0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform4 );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + render.nodeUniform5 );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = vec4<f32>( 0.0, 0.0, 0.0, 0.0 ).xyz;
	indirectDiffuse = ( vec4<f32>( indirectDiffuse, 1.0 ) + vec4<f32>( 1.0, 1.0, 1.0, 0.0 ) ).xyz;
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = ( indirectDiffuse * DiffuseColor.xyz );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst1 = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst1;

	// result

	output.color = nodeConst1;

	return output;

}
