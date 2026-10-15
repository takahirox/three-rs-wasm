// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>,
	@builtin( frag_depth ) depth : f32
};
var<private> output : OutputStruct;

// uniforms
@binding( 4 ) @group( 1 ) var nodeUniform5_sampler : sampler;
@binding( 5 ) @group( 1 ) var nodeUniform5 : texture_2d<f32>;

struct NodeBuffer_1000Struct {
	value : array< u32 >
};
@binding( 0 ) @group( 1 )
var<storage, read> NodeBuffer_1000 : NodeBuffer_1000Struct;

struct NodeBuffer_996Struct {
	value : array< u32 >
};
@binding( 2 ) @group( 1 )
var<storage, read> NodeBuffer_996 : NodeBuffer_996Struct;

struct NodeBuffer_1002Struct {
	value : array< u32 >
};
@binding( 3 ) @group( 1 )
var<storage, read> NodeBuffer_1002 : NodeBuffer_1002Struct;

struct NodeBuffer_994Struct {
	value : array< vec2<f32> >
};
@binding( 6 ) @group( 1 )
var<storage, read> NodeBuffer_994 : NodeBuffer_994Struct;

struct NodeBuffer_995Struct {
	value : array< u32 >
};
@binding( 7 ) @group( 1 )
var<storage, read> NodeBuffer_995 : NodeBuffer_995Struct;

struct NodeBuffer_1005Struct {
	value : array< mat4x4<f32> >
};
@binding( 8 ) @group( 1 )
var<storage, read> NodeBuffer_1005 : NodeBuffer_1005Struct;

struct NodeBuffer_993Struct {
	value : array< vec4<f32> >
};
@binding( 9 ) @group( 1 )
var<storage, read> NodeBuffer_993 : NodeBuffer_993Struct;

struct renderStruct {
	nodeUniform1 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform2 : u32,
	nodeUniform10 : f32
};
@binding( 1 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : u32;
var<private> nodeVar4 : u32;
var<private> nodeVar5 : u32;
var<private> nodeVar6 : u32;
var<private> nodeVar7 : u32;
var<private> nodeVar8 : u32;
var<private> nodeVar9 : vec2<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> nodeVar11 : vec2<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec2<f32>;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec2<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : vec2<f32>;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : vec4<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar33 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = ( f32( ( NodeBuffer_1000.value[ ( ( u32( floor( ( ( 1.0 - nodeVarying0.y ) * render.nodeUniform1.y ) ) ) * u32( render.nodeUniform1.x ) ) + u32( floor( ( nodeVarying0.x * render.nodeUniform1.x ) ) ) ) ] >> 14u ) ) / 262143.0 );
	nodeVar1 = ( nodeVar0 * nodeVar0 );
	output.depth = ( 1.0 - ( nodeVar1 * nodeVar1 ) );
	nodeVar2 = vec4<f32>( vec3<f32>( 0.1, 0.1, 0.1 ), 1.0 );
	nodeVar3 = ( ( u32( floor( ( ( 1.0 - nodeVarying0.y ) * render.nodeUniform1.y ) ) ) * u32( render.nodeUniform1.x ) ) + u32( floor( ( nodeVarying0.x * render.nodeUniform1.x ) ) ) );

	if ( ( f32( ( NodeBuffer_1000.value[ nodeVar3 ] >> 14u ) ) > 0.0 ) ) {


		if ( ( f32( object.nodeUniform2 ) == 0.0 ) ) {

			nodeVar4 = ( NodeBuffer_996.value[ ( NodeBuffer_1000.value[ nodeVar3 ] & 16383u ) ] + ( ( NodeBuffer_1002.value[ nodeVar3 ] & 262143u ) * 1000u ) );
			nodeVar5 = ( ( nodeVar4 * 747796405u ) + 289559509u );
			nodeVar6 = ( ( ( nodeVar5 >> 16u ) ^ nodeVar5 ) * 277803737u );
			nodeVar7 = ( ( nodeVar6 >> 16u ) ^ nodeVar6 );
			nodeVar2 = vec4<f32>( ( ( ( f32( ( nodeVar7 & 255u ) ) / 255.0 ) * 0.8 ) + 0.2 ), ( ( ( f32( ( ( nodeVar7 >> 8u ) & 255u ) ) / 255.0 ) * 0.8 ) + 0.2 ), ( ( ( f32( ( ( nodeVar7 >> 16u ) & 255u ) ) / 255.0 ) * 0.8 ) + 0.2 ), 1.0 );
			

		} else {

			nodeVar8 = ( NodeBuffer_1000.value[ nodeVar3 ] & 16383u );
			nodeVar9 = vec2<f32>( ( nodeVarying0.x * render.nodeUniform1.x ), ( ( 1.0 - nodeVarying0.y ) * render.nodeUniform1.y ) );
			nodeVar10 = ( NodeBuffer_1005.value[ ( NodeBuffer_1002.value[ nodeVar3 ] & 262143u ) ] * NodeBuffer_993.value[ NodeBuffer_995.value[ ( ( nodeVar8 * 3u ) + 1u ) ] ] );
			nodeVar11 = ( ( ( ( nodeVar10.xyz / vec3<f32>( nodeVar10.w ) ).xy + vec2<f32>( 1.0 ) ) * vec2<f32>( 0.5 ) ) * vec2<f32>( render.nodeUniform1.x, render.nodeUniform1.y ) );
			nodeVar12 = ( NodeBuffer_1005.value[ ( NodeBuffer_1002.value[ nodeVar3 ] & 262143u ) ] * NodeBuffer_993.value[ NodeBuffer_995.value[ ( ( nodeVar8 * 3u ) + 2u ) ] ] );
			nodeVar13 = ( ( ( ( nodeVar12.xyz / vec3<f32>( nodeVar12.w ) ).xy + vec2<f32>( 1.0 ) ) * vec2<f32>( 0.5 ) ) * vec2<f32>( render.nodeUniform1.x, render.nodeUniform1.y ) );
			nodeVar15 = ( NodeBuffer_1005.value[ ( NodeBuffer_1002.value[ nodeVar3 ] & 262143u ) ] * NodeBuffer_993.value[ NodeBuffer_995.value[ ( ( nodeVar8 * 3u ) + 0u ) ] ] );
			nodeVar16 = ( ( ( ( nodeVar15.xyz / vec3<f32>( nodeVar15.w ) ).xy + vec2<f32>( 1.0 ) ) * vec2<f32>( 0.5 ) ) * vec2<f32>( render.nodeUniform1.x, render.nodeUniform1.y ) );
			nodeVar17 = ( ( ( nodeVar13.y - nodeVar16.y ) * ( nodeVar11.x - nodeVar16.x ) ) - ( ( nodeVar13.x - nodeVar16.x ) * ( nodeVar11.y - nodeVar16.y ) ) );

			if ( ( nodeVar17 == 0.0 ) ) {

				nodeVar14 = 1.0;

			} else {

				nodeVar14 = nodeVar17;

			}

			nodeVar18 = ( ( ( ( nodeVar9.y - nodeVar11.y ) * ( nodeVar13.x - nodeVar11.x ) ) - ( ( nodeVar9.x - nodeVar11.x ) * ( nodeVar13.y - nodeVar11.y ) ) ) / nodeVar14 );
			nodeVar20 = ( ( ( ( nodeVar9.y - nodeVar13.y ) * ( nodeVar16.x - nodeVar13.x ) ) - ( ( nodeVar9.x - nodeVar13.x ) * ( nodeVar16.y - nodeVar13.y ) ) ) / nodeVar14 );
			nodeVar21 = ( ( ( ( nodeVar9.y - nodeVar16.y ) * ( nodeVar11.x - nodeVar16.x ) ) - ( ( nodeVar9.x - nodeVar16.x ) * ( nodeVar11.y - nodeVar16.y ) ) ) / nodeVar14 );
			nodeVar22 = ( ( ( nodeVar18 / nodeVar15.w ) + ( nodeVar20 / nodeVar10.w ) ) + ( nodeVar21 / nodeVar12.w ) );

			if ( ( nodeVar22 == 0.0 ) ) {

				nodeVar19 = 1.0;

			} else {

				nodeVar19 = nodeVar22;

			}

			nodeVar23 = ( ( ( NodeBuffer_994.value[ NodeBuffer_995.value[ ( ( nodeVar8 * 3u ) + 0u ) ] ] * vec2<f32>( ( ( nodeVar18 / nodeVar15.w ) / nodeVar19 ) ) ) + ( NodeBuffer_994.value[ NodeBuffer_995.value[ ( ( nodeVar8 * 3u ) + 1u ) ] ] * vec2<f32>( ( ( nodeVar20 / nodeVar10.w ) / nodeVar19 ) ) ) ) + ( NodeBuffer_994.value[ NodeBuffer_995.value[ ( ( nodeVar8 * 3u ) + 2u ) ] ] * vec2<f32>( ( ( nodeVar21 / nodeVar12.w ) / nodeVar19 ) ) ) );
			nodeVar24 = ( 1.0 / nodeVar15.w );
			nodeVar25 = ( 1.0 / nodeVar10.w );
			nodeVar26 = ( 1.0 / nodeVar12.w );
			nodeVar28 = ( ( ( nodeVar9.y - nodeVar11.y ) * ( nodeVar13.x - nodeVar11.x ) ) - ( ( nodeVar9.x - nodeVar11.x ) * ( nodeVar13.y - nodeVar11.y ) ) );
			nodeVar29 = ( ( ( nodeVar9.y - nodeVar13.y ) * ( nodeVar16.x - nodeVar13.x ) ) - ( ( nodeVar9.x - nodeVar13.x ) * ( nodeVar16.y - nodeVar13.y ) ) );
			nodeVar30 = ( ( ( nodeVar9.y - nodeVar16.y ) * ( nodeVar11.x - nodeVar16.x ) ) - ( ( nodeVar9.x - nodeVar16.x ) * ( nodeVar11.y - nodeVar16.y ) ) );
			nodeVar31 = ( ( ( nodeVar28 * nodeVar24 ) + ( nodeVar29 * nodeVar25 ) ) + ( nodeVar30 * nodeVar26 ) );

			if ( ( nodeVar31 == 0.0 ) ) {

				nodeVar27 = 1.0;

			} else {

				nodeVar27 = nodeVar31;

			}

			nodeVar32 = textureSampleGrad( nodeUniform5, nodeUniform5_sampler, nodeVar23, ( ( ( ( vec2<f32>( ( ( nodeVar13.y - nodeVar11.y ) * nodeVar24 ) ) * ( NodeBuffer_994.value[ NodeBuffer_995.value[ ( ( nodeVar8 * 3u ) + 0u ) ] ] - nodeVar23 ) ) + ( vec2<f32>( ( ( nodeVar16.y - nodeVar13.y ) * nodeVar25 ) ) * ( NodeBuffer_994.value[ NodeBuffer_995.value[ ( ( nodeVar8 * 3u ) + 1u ) ] ] - nodeVar23 ) ) ) + ( vec2<f32>( ( ( nodeVar11.y - nodeVar16.y ) * nodeVar26 ) ) * ( NodeBuffer_994.value[ NodeBuffer_995.value[ ( ( nodeVar8 * 3u ) + 2u ) ] ] - nodeVar23 ) ) ) / vec2<f32>( nodeVar27 ) ), ( ( ( ( vec2<f32>( ( ( nodeVar11.x - nodeVar13.x ) * nodeVar24 ) ) * ( NodeBuffer_994.value[ NodeBuffer_995.value[ ( ( nodeVar8 * 3u ) + 0u ) ] ] - nodeVar23 ) ) + ( vec2<f32>( ( ( nodeVar13.x - nodeVar16.x ) * nodeVar25 ) ) * ( NodeBuffer_994.value[ NodeBuffer_995.value[ ( ( nodeVar8 * 3u ) + 1u ) ] ] - nodeVar23 ) ) ) + ( vec2<f32>( ( ( nodeVar16.x - nodeVar11.x ) * nodeVar26 ) ) * ( NodeBuffer_994.value[ NodeBuffer_995.value[ ( ( nodeVar8 * 3u ) + 2u ) ] ] - nodeVar23 ) ) ) / nodeVar27 ) );
			nodeVar2 = nodeVar32;
			

		}

		

	}

	DiffuseColor = nodeVar2;
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform10 );
	DiffuseColor.w = 1.0;
	nodeVar33 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar33;

	// result

	output.color = nodeVar33;

	return output;

}
