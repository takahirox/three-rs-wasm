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
var<private> nodeVar3 : vec2<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : vec2<f32>;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec2<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec2<f32>;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec2<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : vec2<f32>;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : vec2<f32>;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec2<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : vec2<f32>;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec2<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;

// codes


@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform0, nodeUniform0_sampler, nodeVarying0 ).x;
	nodeVar1 = vec4<f32>( ( nodeVar0 * 0.10924730377444448 ) );
	nodeVar2 = ( vec2<f32>( object.nodeUniform1 ) * vec2<f32>( 1.0, 0.0 ) );
	nodeVar3 = ( nodeVar2 * ( object.nodeUniform2 * vec2<f32>( 1.0 ) ) );
	nodeVar4 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar3 ) ).x;
	nodeVar5 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar3 ) ).x;
	nodeVar1 = ( nodeVar1 + vec4<f32>( ( ( nodeVar4 + nodeVar5 ) * 0.10525900968602987 ) ) );
	nodeVar6 = ( nodeVar2 * ( object.nodeUniform2 * vec2<f32>( 2.0 ) ) );
	nodeVar7 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar6 ) ).x;
	nodeVar8 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar6 ) ).x;
	nodeVar1 = ( nodeVar1 + vec4<f32>( ( ( nodeVar7 + nodeVar8 ) * 0.09414666419084379 ) ) );
	nodeVar9 = ( nodeVar2 * ( object.nodeUniform2 * vec2<f32>( 3.0 ) ) );
	nodeVar10 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar9 ) ).x;
	nodeVar11 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar9 ) ).x;
	nodeVar1 = ( nodeVar1 + vec4<f32>( ( ( nodeVar10 + nodeVar11 ) * 0.0781713655032425 ) ) );
	nodeVar12 = ( nodeVar2 * ( object.nodeUniform2 * vec2<f32>( 4.0 ) ) );
	nodeVar13 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar12 ) ).x;
	nodeVar14 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar12 ) ).x;
	nodeVar1 = ( nodeVar1 + vec4<f32>( ( ( nodeVar13 + nodeVar14 ) * 0.06025423328818032 ) ) );
	nodeVar15 = ( nodeVar2 * ( object.nodeUniform2 * vec2<f32>( 5.0 ) ) );
	nodeVar16 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar15 ) ).x;
	nodeVar17 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar15 ) ).x;
	nodeVar1 = ( nodeVar1 + vec4<f32>( ( ( nodeVar16 + nodeVar17 ) * 0.04311461730179377 ) ) );
	nodeVar18 = ( nodeVar2 * ( object.nodeUniform2 * vec2<f32>( 6.0 ) ) );
	nodeVar19 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar18 ) ).x;
	nodeVar20 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar18 ) ).x;
	nodeVar1 = ( nodeVar1 + vec4<f32>( ( ( nodeVar19 + nodeVar20 ) * 0.028639050226339946 ) ) );
	nodeVar21 = ( nodeVar2 * ( object.nodeUniform2 * vec2<f32>( 7.0 ) ) );
	nodeVar22 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar21 ) ).x;
	nodeVar23 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar21 ) ).x;
	nodeVar1 = ( nodeVar1 + vec4<f32>( ( ( nodeVar22 + nodeVar23 ) * 0.017659963081781176 ) ) );
	nodeVar24 = ( nodeVar2 * ( object.nodeUniform2 * vec2<f32>( 8.0 ) ) );
	nodeVar25 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar24 ) ).x;
	nodeVar26 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar24 ) ).x;
	nodeVar1 = ( nodeVar1 + vec4<f32>( ( ( nodeVar25 + nodeVar26 ) * 0.010109229970567757 ) ) );
	nodeVar27 = ( nodeVar2 * ( object.nodeUniform2 * vec2<f32>( 9.0 ) ) );
	nodeVar28 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar27 ) ).x;
	nodeVar29 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar27 ) ).x;
	nodeVar1 = ( nodeVar1 + vec4<f32>( ( ( nodeVar28 + nodeVar29 ) * 0.005372092299194051 ) ) );
	nodeVar30 = ( nodeVar2 * ( object.nodeUniform2 * vec2<f32>( 10.0 ) ) );
	nodeVar31 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar30 ) ).x;
	nodeVar32 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar30 ) ).x;
	nodeVar1 = ( nodeVar1 + vec4<f32>( ( ( nodeVar31 + nodeVar32 ) * 0.0026501225648045356 ) ) );

	// result

	output.color = nodeVar1;

	return output;

}
