// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 0 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 0 ) var nodeUniform0 : texture_2d<f32>;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec2<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec4<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;

// codes


@fragment
fn main( @location( 0 ) nodeVarying0 : vec4<f32>,
	@location( 1 ) nodeVarying1 : vec4<f32>,
	@location( 2 ) nodeVarying2 : vec4<f32>,
	@location( 3 ) nodeVarying3 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = vec4<f32>( 0.0, 0.0, 0.0, 1.0 );
	nodeVar1 = textureSample( nodeUniform0, nodeUniform0_sampler, nodeVarying3 );
	nodeVar2 = nodeVar1.xyz;
	nodeVar4 = textureSample( nodeUniform0, nodeUniform0_sampler, nodeVarying0.xy );
	nodeVar5 = nodeVar4.xyz;
	nodeVar6 = abs( ( nodeVar2 - nodeVar5 ) );
	nodeVar0.x = max( max( nodeVar6.x, nodeVar6.y ), nodeVar6.z );
	nodeVar7 = textureSample( nodeUniform0, nodeUniform0_sampler, nodeVarying0.zw );
	nodeVar8 = nodeVar7.xyz;
	nodeVar9 = abs( ( nodeVar2 - nodeVar8 ) );
	nodeVar0.y = max( max( nodeVar9.x, nodeVar9.y ), nodeVar9.z );
	nodeVar10 = step( vec2<f32>( 0.1, 0.1 ), nodeVar0.xy );

	if ( ( dot( nodeVar10, vec2<f32>( 1.0, 1.0 ) ) == 0.0 ) ) {

		discard;
		

	}

	nodeVar12 = textureSample( nodeUniform0, nodeUniform0_sampler, nodeVarying1.xy );
	nodeVar13 = nodeVar12.xyz;
	nodeVar14 = abs( ( nodeVar2 - nodeVar13 ) );
	nodeVar0.z = max( max( nodeVar14.x, nodeVar14.y ), nodeVar14.z );
	nodeVar15 = textureSample( nodeUniform0, nodeUniform0_sampler, nodeVarying1.zw );
	nodeVar16 = nodeVar15.xyz;
	nodeVar17 = abs( ( nodeVar2 - nodeVar16 ) );
	nodeVar0.w = max( max( nodeVar17.x, nodeVar17.y ), nodeVar17.z );
	nodeVar18 = max( max( max( nodeVar0.x, nodeVar0.y ), nodeVar0.z ), nodeVar0.w );
	nodeVar20 = textureSample( nodeUniform0, nodeUniform0_sampler, nodeVarying2.xy );
	nodeVar21 = nodeVar20.xyz;
	nodeVar22 = abs( ( nodeVar2 - nodeVar21 ) );
	nodeVar0.z = max( max( nodeVar22.x, nodeVar22.y ), nodeVar22.z );
	nodeVar23 = textureSample( nodeUniform0, nodeUniform0_sampler, nodeVarying2.zw );
	nodeVar24 = nodeVar23.xyz;
	nodeVar25 = abs( ( nodeVar2 - nodeVar24 ) );
	nodeVar0.w = max( max( nodeVar25.x, nodeVar25.y ), nodeVar25.z );
	nodeVar10 = ( nodeVar10 * step( vec2<f32>( ( 0.5 * max( max( nodeVar18, nodeVar0.z ), nodeVar0.w ) ) ), nodeVar0.xy ) );

	// result

	output.color = vec4<f32>( nodeVar10, 0.0, 0.0 );

	return output;

}
