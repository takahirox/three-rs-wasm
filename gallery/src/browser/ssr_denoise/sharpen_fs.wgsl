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
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;

// codes


@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	let nodeConst0 = vec2<i32>( i32( floor( ( nodeVarying0.x * f32( textureDimensions( nodeUniform0, 0 ).x ) ) ) ), i32( floor( ( nodeVarying0.y * f32( textureDimensions( nodeUniform0, 0 ).y ) ) ) ) );
	let nodeConst1 = exp2( ( - 0.0 ) );
	nodeVar0 = textureLoad( nodeUniform0, ( nodeConst0 + vec2<i32>( 0, -1 ) ), u32( 0u ) );
	nodeVar1 = textureLoad( nodeUniform0, ( nodeConst0 + vec2<i32>( -1, 0 ) ), u32( 0u ) );
	nodeVar2 = textureLoad( nodeUniform0, ( nodeConst0 + vec2<i32>( 1, 0 ) ), u32( 0u ) );
	nodeVar3 = textureLoad( nodeUniform0, ( nodeConst0 + vec2<i32>( 0, 1 ) ), u32( 0u ) );
	let nodeConst2 = min( min( nodeVar0.xyz, nodeVar1.xyz ), min( nodeVar2.xyz, nodeVar3.xyz ) );
	let nodeConst3 = max( max( nodeVar0.xyz, nodeVar1.xyz ), max( nodeVar2.xyz, nodeVar3.xyz ) );
	let nodeConst4 = 0.1875;
	nodeVar4 = textureLoad( nodeUniform0, nodeConst0, u32( 0u ) );
	let nodeConst5 = ( min( nodeConst2, nodeVar4.xyz ) / ( nodeConst3 * vec3<f32>( 4.0 ) ) );
	let nodeConst6 = ( ( vec3<f32>( 1.0, 1.0, 1.0 ) - max( nodeConst3, nodeVar4.xyz ) ) / ( ( nodeConst2 * vec3<f32>( 4.0 ) ) - vec3<f32>( 4.0 ) ) );
	let nodeConst7 = max( ( - nodeConst5 ), nodeConst6 );
	let nodeConst8 = ( max( ( - nodeConst4 ), min( max( nodeConst7.x, max( nodeConst7.y, nodeConst7.z ) ), 0.0 ) ) * nodeConst1 );
	nodeVar5 = ( nodeVar0.y + ( ( nodeVar0.z + nodeVar0.x ) * 0.5 ) );
	nodeVar6 = ( nodeVar1.y + ( ( nodeVar1.z + nodeVar1.x ) * 0.5 ) );
	nodeVar7 = ( nodeVar2.y + ( ( nodeVar2.z + nodeVar2.x ) * 0.5 ) );
	nodeVar8 = ( nodeVar3.y + ( ( nodeVar3.z + nodeVar3.x ) * 0.5 ) );
	nodeVar9 = ( nodeVar4.y + ( ( nodeVar4.z + nodeVar4.x ) * 0.5 ) );
	let nodeConst9 = ( ( ( ( ( nodeVar5 + nodeVar6 ) + nodeVar7 ) + nodeVar8 ) * 0.25 ) - nodeVar9 );
	let nodeConst10 = ( max( max( nodeVar5, nodeVar6 ), max( nodeVar9, max( nodeVar7, nodeVar8 ) ) ) - min( min( nodeVar5, nodeVar6 ), min( nodeVar9, min( nodeVar7, nodeVar8 ) ) ) );
	let nodeConst11 = ( 1.0 - ( clamp( ( abs( nodeConst9 ) / max( nodeConst10, 0.0000152587890625 ) ), 0.0, 1.0 ) * 0.5 ) );

	if ( ( false == true ) ) {

		nodeVar10 = ( nodeConst8 * nodeConst11 );

	} else {

		nodeVar10 = nodeConst8;

	}

	let nodeConst12 = nodeVar10;
	let nodeConst13 = ( ( ( ( ( ( nodeVar0.xyz + nodeVar1.xyz ) + nodeVar2.xyz ) + nodeVar3.xyz ) * vec3<f32>( nodeConst12 ) ) + nodeVar4.xyz ) / vec3<f32>( ( ( nodeConst12 * 4.0 ) + 1.0 ) ) );

	// result

	output.color = vec4<f32>( nodeConst13, nodeVar4.w );

	return output;

}
