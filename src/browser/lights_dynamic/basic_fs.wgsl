// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );

// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms

struct NodeBuffer_1163Struct {
	value : array< vec4<f32>, MAX_LIGHTS >
};
@binding( 1 ) @group( 0 )
var<uniform> NodeBuffer_1163 : NodeBuffer_1163Struct;

struct NodeBuffer_1162Struct {
	value : array< vec4<f32>, MAX_LIGHTS >
};
@binding( 2 ) @group( 0 )
var<uniform> NodeBuffer_1162 : NodeBuffer_1162Struct;

struct NodeBuffer_1164Struct {
	value : array< vec4<f32>, MAX_LIGHTS >
};
@binding( 3 ) @group( 0 )
var<uniform> NodeBuffer_1164 : NodeBuffer_1164Struct;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform6 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform2 : vec3<f32>,
	nodeUniform3 : i32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> dynPointDiffuse : vec3<f32>;
var<private> dynPointSpecular : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec4<f32>;

// codes

@fragment
fn main( @location( 0 ) v_positionView : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar0 = ( irradiance + render.nodeUniform2 );
	irradiance = nodeVar0;
	dynPointDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	dynPointSpecular = vec3<f32>( 0.0, 0.0, 0.0 );

	for ( var i : i32 = 0; i < render.nodeUniform3; i ++ ) {

		nodeVar1 = ( NodeBuffer_1163.value[ i ].xyz - v_positionView );
		nodeVar2 = normalize( nodeVar1 );

		if ( ( NodeBuffer_1163.value[ i ].w > 0.0 ) ) {

			nodeVar4 = length( nodeVar1 );
			nodeVar5 = ( nodeVar4 / NodeBuffer_1163.value[ i ].w );
			nodeVar6 = clamp( ( 1.0 - ( ( ( nodeVar5 * nodeVar5 ) * nodeVar5 ) * nodeVar5 ) ), 0.0, 1.0 );
			nodeVar3 = ( ( 1.0 / max( pow( nodeVar4, NodeBuffer_1164.value[ i ].x ), 0.01 ) ) * ( nodeVar6 * nodeVar6 ) );

		} else {

			nodeVar3 = ( 1.0 / max( pow( length( nodeVar1 ), NodeBuffer_1164.value[ i ].x ), 0.01 ) );

		}

		nodeVar7 = ( NodeBuffer_1162.value[ i ].xyz * vec3<f32>( nodeVar3 ) );

	}

	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar8 = ( directDiffuse + dynPointDiffuse );
	directDiffuse = nodeVar8;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar9 = ( directSpecular + dynPointSpecular );
	directSpecular = nodeVar9;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = vec4<f32>( 0.0, 0.0, 0.0, 0.0 ).xyz;
	nodeVar10 = ( vec4<f32>( indirectDiffuse, 1.0 ) + vec4<f32>( 1.0, 1.0, 1.0, 0.0 ) );
	indirectDiffuse = nodeVar10.xyz;
	ambientOcclusion = 1.0;
	nodeVar11 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar11;
	nodeVar12 = ( indirectDiffuse * DiffuseColor.xyz );
	indirectDiffuse = nodeVar12;
	nodeVar13 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar13;
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
