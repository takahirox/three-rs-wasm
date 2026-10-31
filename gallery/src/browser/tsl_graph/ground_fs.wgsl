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

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform1 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Output : vec4<f32>;
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
fn main( @location( 0 ) v_positionWorld : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	let nodeConst0 = fwidth( v_positionWorld.xz );
	let nodeConst1 = max( nodeConst0.x, nodeConst0.y );
	let nodeConst2 = fract( v_positionWorld.xz );
	let nodeConst3 = abs( ( nodeConst2.x - 0.5 ) );
	let nodeConst4 = abs( ( nodeConst2.y - 0.5 ) );
	DiffuseColor = ( mix( vec4<f32>( vec3<f32>( 0.4, 0.4, 0.4 ), 0.0 ), vec4<f32>( vec3<f32>( 0.2, 0.2, 0.2 ), 1.0 ), max( smoothstep( ( 0.03 + nodeConst1 ), ( 0.03 - nodeConst1 ), max( nodeConst3, nodeConst4 ) ), max( clamp( ( ( ( 0.007 - nodeConst3 ) / nodeConst0.x ) + 0.5 ), 0.0, 1.0 ), clamp( ( ( ( 0.007 - nodeConst4 ) / nodeConst0.y ) + 0.5 ), 0.0, 1.0 ) ) ) ) * vec4<f32>( smoothstep( 30.0, ( 30.0 - 20.0 ), length( v_positionWorld ) ) ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
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
	let nodeConst5 = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst5;

	// result

	output.color = nodeConst5;

	return output;

}
