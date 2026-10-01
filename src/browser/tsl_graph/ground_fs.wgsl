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
	nodeUniform0 : mat4x4<f32>,
	nodeUniform1 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec2<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> Output : vec4<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_positionWorld : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = fwidth( v_positionWorld.xz );
	nodeVar1 = max( nodeVar0.x, nodeVar0.y );
	nodeVar2 = fract( v_positionWorld.xz );
	nodeVar3 = abs( ( nodeVar2.x - 0.5 ) );
	nodeVar4 = abs( ( nodeVar2.y - 0.5 ) );
	DiffuseColor = ( mix( vec4<f32>( vec3<f32>( 0.4, 0.4, 0.4 ), 0.0 ), vec4<f32>( vec3<f32>( 0.2, 0.2, 0.2 ), 1.0 ), max( smoothstep( ( 0.03 + nodeVar1 ), ( 0.03 - nodeVar1 ), max( nodeVar3, nodeVar4 ) ), max( clamp( ( ( ( 0.007 - nodeVar3 ) / nodeVar0.x ) + 0.5 ), 0.0, 1.0 ), clamp( ( ( ( 0.007 - nodeVar4 ) / nodeVar0.y ) + 0.5 ), 0.0, 1.0 ) ) ) ) * vec4<f32>( smoothstep( 30.0, ( 30.0 - 20.0 ), length( v_positionWorld ) ) ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = vec4<f32>( 0.0, 0.0, 0.0, 0.0 ).xyz;
	nodeVar5 = ( vec4<f32>( indirectDiffuse, 1.0 ) + vec4<f32>( 1.0, 1.0, 1.0, 0.0 ) );
	indirectDiffuse = nodeVar5.xyz;
	ambientOcclusion = 1.0;
	nodeVar6 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar6;
	nodeVar7 = ( indirectDiffuse * DiffuseColor.xyz );
	indirectDiffuse = nodeVar7;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar8 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar8;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar9 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar9;
	nodeVar10 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar10;
	nodeVar11 = max( vec4<f32>( outgoingLight, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar11;

	// result

	output.color = nodeVar11;

	return output;

}
