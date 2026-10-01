// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


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
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : f32;
var<private> Output : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) nodeVarying4 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar7 = textureSample( nodeUniform2, nodeUniform2_sampler, ( ( nodeVarying4 * vec2<f32>( 0.5, 0.3 ) ) + vec2<f32>( 0.0, ( - ( render.nodeUniform3 * 0.03 ) ) ) ) );
	nodeVar8 = ( ( ( ( smoothstep( 0.4, 1.0, nodeVar7.x ) * smoothstep( 0.0, 0.1, nodeVarying4.x ) ) * smoothstep( 0.0, 0.1, ( 1.0 - nodeVarying4.x ) ) ) * smoothstep( 0.0, 0.1, nodeVarying4.y ) ) * smoothstep( 0.0, 0.1, ( 1.0 - nodeVarying4.y ) ) );
	DiffuseColor = vec4<f32>( mix( vec3<f32>( 0.6, 0.3, 0.2 ), vec3<f32>( 1.0, 1.0, 1.0 ), pow( nodeVar8, 3.0 ) ), nodeVar8 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform4 );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar9 = ( irradiance + render.nodeUniform5 );
	irradiance = nodeVar9;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = vec4<f32>( 0.0, 0.0, 0.0, 0.0 ).xyz;
	nodeVar10 = ( vec4<f32>( indirectDiffuse, 1.0 ) + vec4<f32>( 1.0, 1.0, 1.0, 0.0 ) );
	indirectDiffuse = nodeVar10.xyz;
	ambientOcclusion = 1.0;
	nodeVar11 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar11;
	nodeVar12 = ( indirectDiffuse * DiffuseColor.xyz );
	indirectDiffuse = nodeVar12;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar13 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar13;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar14 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar14;
	nodeVar15 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar15;
	nodeVar16 = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar16;

	// result

	output.color = nodeVar16;

	return output;

}
