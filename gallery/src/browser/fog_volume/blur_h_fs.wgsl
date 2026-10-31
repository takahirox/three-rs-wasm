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
@binding( 0 ) @group( 0 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 0 ) var nodeUniform0 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : f32,
	nodeUniform2 : vec2<f32>
};
@binding( 2 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : f32;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : vec2<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : vec2<f32>;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : vec2<f32>;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec2<f32>;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec2<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec2<f32>;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : vec2<f32>;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec2<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec2<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;

// codes


@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform0, nodeUniform0_sampler, nodeVarying0 ).x;
	nodeVar1 = vec4<f32>( ( nodeVar0 * 0.10924730377444448 ) );
	let nodeConst0 = ( vec2<f32>( object.nodeUniform1 ) * vec2<f32>( 1.0, 0.0 ) );
	nodeVar2 = ( nodeConst0 * ( object.nodeUniform2 * vec2<f32>( 1.0 ) ) );
	nodeVar3 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar2 ) ).x;
	nodeVar4 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar2 ) ).x;
	nodeVar1 = ( nodeVar1 + vec4<f32>( ( ( nodeVar3 + nodeVar4 ) * 0.10525900968602987 ) ) );
	nodeVar5 = ( nodeConst0 * ( object.nodeUniform2 * vec2<f32>( 2.0 ) ) );
	nodeVar6 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar5 ) ).x;
	nodeVar7 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar5 ) ).x;
	nodeVar1 = ( nodeVar1 + vec4<f32>( ( ( nodeVar6 + nodeVar7 ) * 0.09414666419084379 ) ) );
	nodeVar8 = ( nodeConst0 * ( object.nodeUniform2 * vec2<f32>( 3.0 ) ) );
	nodeVar9 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar8 ) ).x;
	nodeVar10 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar8 ) ).x;
	nodeVar1 = ( nodeVar1 + vec4<f32>( ( ( nodeVar9 + nodeVar10 ) * 0.0781713655032425 ) ) );
	nodeVar11 = ( nodeConst0 * ( object.nodeUniform2 * vec2<f32>( 4.0 ) ) );
	nodeVar12 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar11 ) ).x;
	nodeVar13 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar11 ) ).x;
	nodeVar1 = ( nodeVar1 + vec4<f32>( ( ( nodeVar12 + nodeVar13 ) * 0.06025423328818032 ) ) );
	nodeVar14 = ( nodeConst0 * ( object.nodeUniform2 * vec2<f32>( 5.0 ) ) );
	nodeVar15 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar14 ) ).x;
	nodeVar16 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar14 ) ).x;
	nodeVar1 = ( nodeVar1 + vec4<f32>( ( ( nodeVar15 + nodeVar16 ) * 0.04311461730179377 ) ) );
	nodeVar17 = ( nodeConst0 * ( object.nodeUniform2 * vec2<f32>( 6.0 ) ) );
	nodeVar18 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar17 ) ).x;
	nodeVar19 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar17 ) ).x;
	nodeVar1 = ( nodeVar1 + vec4<f32>( ( ( nodeVar18 + nodeVar19 ) * 0.028639050226339946 ) ) );
	nodeVar20 = ( nodeConst0 * ( object.nodeUniform2 * vec2<f32>( 7.0 ) ) );
	nodeVar21 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar20 ) ).x;
	nodeVar22 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar20 ) ).x;
	nodeVar1 = ( nodeVar1 + vec4<f32>( ( ( nodeVar21 + nodeVar22 ) * 0.017659963081781176 ) ) );
	nodeVar23 = ( nodeConst0 * ( object.nodeUniform2 * vec2<f32>( 8.0 ) ) );
	nodeVar24 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar23 ) ).x;
	nodeVar25 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar23 ) ).x;
	nodeVar1 = ( nodeVar1 + vec4<f32>( ( ( nodeVar24 + nodeVar25 ) * 0.010109229970567757 ) ) );
	nodeVar26 = ( nodeConst0 * ( object.nodeUniform2 * vec2<f32>( 9.0 ) ) );
	nodeVar27 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar26 ) ).x;
	nodeVar28 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar26 ) ).x;
	nodeVar1 = ( nodeVar1 + vec4<f32>( ( ( nodeVar27 + nodeVar28 ) * 0.005372092299194051 ) ) );
	nodeVar29 = ( nodeConst0 * ( object.nodeUniform2 * vec2<f32>( 10.0 ) ) );
	nodeVar30 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar29 ) ).x;
	nodeVar31 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar29 ) ).x;
	nodeVar1 = ( nodeVar1 + vec4<f32>( ( ( nodeVar30 + nodeVar31 ) * 0.0026501225648045356 ) ) );

	// result

	output.color = nodeVar1;

	return output;

}
