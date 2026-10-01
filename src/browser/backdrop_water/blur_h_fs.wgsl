// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 1 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform0 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform5 : texture_depth_2d;

struct objectStruct {
	nodeUniform1 : vec3<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform6 : vec2<f32>
};
@binding( 2 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform2 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec2<u32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : vec2<u32>;
var<private> nodeVar7 : vec2<f32>;
var<private> nodeVar8 : vec2<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> nodeVar11 : vec2<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec2<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec2<f32>;
var<private> nodeVar18 : vec4<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : vec2<f32>;
var<private> nodeVar21 : vec4<f32>;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : vec2<f32>;
var<private> nodeVar24 : vec4<f32>;
var<private> nodeVar25 : vec4<f32>;
var<private> nodeVar26 : vec2<f32>;
var<private> nodeVar27 : vec4<f32>;
var<private> nodeVar28 : vec4<f32>;
var<private> nodeVar29 : vec2<f32>;
var<private> nodeVar30 : vec4<f32>;
var<private> nodeVar31 : vec4<f32>;
var<private> nodeVar32 : vec2<f32>;
var<private> nodeVar33 : vec4<f32>;
var<private> nodeVar34 : vec4<f32>;
var<private> nodeVar35 : vec2<f32>;
var<private> nodeVar36 : vec4<f32>;
var<private> nodeVar37 : vec4<f32>;

// codes
fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}



@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform0, nodeUniform0_sampler, nodeVarying0 );
	nodeVar1 = ( nodeVar0 * vec4<f32>( 0.10924730377444448 ) );

	if ( ( object.nodeUniform1.y > ( ( ( fragCoord.xy / render.nodeUniform2 ).y - 0.5 ) * 0.25 ) ) ) {

		nodeVar4 = textureDimensions( nodeUniform5, u32( 0 ) );
		nodeVar3 = textureLoad( nodeUniform5, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVarying0 ) * vec2<f32>( nodeVar4 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar4 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		nodeVar2 = ( ( clamp( ( ( ( ( ( ( object.nodeUniform3 * object.nodeUniform4 ) / ( ( ( object.nodeUniform4 - object.nodeUniform3 ) * nodeVar3 ) - object.nodeUniform4 ) ) + object.nodeUniform3 ) / ( object.nodeUniform3 - object.nodeUniform4 ) ) - 0.3 ) / ( 0.5 - 0.3 ) ), 0.0, 1.0 ) * ( 1.0 - 0.0 ) ) + 0.0 );

	} else {

		nodeVar6 = textureDimensions( nodeUniform5, u32( 0 ) );
		nodeVar5 = textureLoad( nodeUniform5, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVarying0 ) * vec2<f32>( nodeVar6 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar6 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		nodeVar2 = ( ( ( ( ( object.nodeUniform3 * object.nodeUniform4 ) / ( ( ( object.nodeUniform4 - object.nodeUniform3 ) * nodeVar5 ) - object.nodeUniform4 ) ) + object.nodeUniform3 ) / ( object.nodeUniform3 - object.nodeUniform4 ) ) * 5.0 );

	}

	nodeVar7 = ( nodeVar2 * vec2<f32>( 1.0, 0.0 ) );
	nodeVar8 = ( nodeVar7 * ( object.nodeUniform6 * vec2<f32>( 1.0 ) ) );
	nodeVar9 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar8 ) );
	nodeVar10 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar8 ) );
	nodeVar1 = ( nodeVar1 + ( ( nodeVar9 + nodeVar10 ) * vec4<f32>( 0.10525900968602987 ) ) );
	nodeVar11 = ( nodeVar7 * ( object.nodeUniform6 * vec2<f32>( 2.0 ) ) );
	nodeVar12 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar11 ) );
	nodeVar13 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar11 ) );
	nodeVar1 = ( nodeVar1 + ( ( nodeVar12 + nodeVar13 ) * vec4<f32>( 0.09414666419084379 ) ) );
	nodeVar14 = ( nodeVar7 * ( object.nodeUniform6 * vec2<f32>( 3.0 ) ) );
	nodeVar15 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar14 ) );
	nodeVar16 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar14 ) );
	nodeVar1 = ( nodeVar1 + ( ( nodeVar15 + nodeVar16 ) * vec4<f32>( 0.0781713655032425 ) ) );
	nodeVar17 = ( nodeVar7 * ( object.nodeUniform6 * vec2<f32>( 4.0 ) ) );
	nodeVar18 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar17 ) );
	nodeVar19 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar17 ) );
	nodeVar1 = ( nodeVar1 + ( ( nodeVar18 + nodeVar19 ) * vec4<f32>( 0.06025423328818032 ) ) );
	nodeVar20 = ( nodeVar7 * ( object.nodeUniform6 * vec2<f32>( 5.0 ) ) );
	nodeVar21 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar20 ) );
	nodeVar22 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar20 ) );
	nodeVar1 = ( nodeVar1 + ( ( nodeVar21 + nodeVar22 ) * vec4<f32>( 0.04311461730179377 ) ) );
	nodeVar23 = ( nodeVar7 * ( object.nodeUniform6 * vec2<f32>( 6.0 ) ) );
	nodeVar24 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar23 ) );
	nodeVar25 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar23 ) );
	nodeVar1 = ( nodeVar1 + ( ( nodeVar24 + nodeVar25 ) * vec4<f32>( 0.028639050226339946 ) ) );
	nodeVar26 = ( nodeVar7 * ( object.nodeUniform6 * vec2<f32>( 7.0 ) ) );
	nodeVar27 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar26 ) );
	nodeVar28 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar26 ) );
	nodeVar1 = ( nodeVar1 + ( ( nodeVar27 + nodeVar28 ) * vec4<f32>( 0.017659963081781176 ) ) );
	nodeVar29 = ( nodeVar7 * ( object.nodeUniform6 * vec2<f32>( 8.0 ) ) );
	nodeVar30 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar29 ) );
	nodeVar31 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar29 ) );
	nodeVar1 = ( nodeVar1 + ( ( nodeVar30 + nodeVar31 ) * vec4<f32>( 0.010109229970567757 ) ) );
	nodeVar32 = ( nodeVar7 * ( object.nodeUniform6 * vec2<f32>( 9.0 ) ) );
	nodeVar33 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar32 ) );
	nodeVar34 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar32 ) );
	nodeVar1 = ( nodeVar1 + ( ( nodeVar33 + nodeVar34 ) * vec4<f32>( 0.005372092299194051 ) ) );
	nodeVar35 = ( nodeVar7 * ( object.nodeUniform6 * vec2<f32>( 10.0 ) ) );
	nodeVar36 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar35 ) );
	nodeVar37 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar35 ) );
	nodeVar1 = ( nodeVar1 + ( ( nodeVar36 + nodeVar37 ) * vec4<f32>( 0.0026501225648045356 ) ) );

	// result

	output.color = nodeVar1;

	return output;

}
