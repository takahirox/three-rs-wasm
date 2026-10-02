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
@binding( 8 ) @group( 1 ) var nodeUniform10_sampler : sampler;
@binding( 9 ) @group( 1 ) var nodeUniform10 : texture_2d<f32>;
@binding( 10 ) @group( 1 ) var nodeUniform11_sampler : sampler;
@binding( 11 ) @group( 1 ) var nodeUniform11 : texture_2d<f32>;
@binding( 12 ) @group( 1 ) var nodeUniform12_sampler : sampler;
@binding( 13 ) @group( 1 ) var nodeUniform12 : texture_2d<f32>;
@binding( 14 ) @group( 1 ) var nodeUniform13_sampler : sampler;
@binding( 15 ) @group( 1 ) var nodeUniform13 : texture_2d<f32>;

struct NodeBuffer_1005Struct {
	value : array< u32 >
};
@binding( 0 ) @group( 1 )
var<storage, read> NodeBuffer_1005 : NodeBuffer_1005Struct;

struct NodeBuffer_1012Struct {
	value : array< mat4x4<f32> >
};
@binding( 2 ) @group( 1 )
var<storage, read> NodeBuffer_1012 : NodeBuffer_1012Struct;

struct NodeBuffer_1007Struct {
	value : array< u32 >
};
@binding( 3 ) @group( 1 )
var<storage, read> NodeBuffer_1007 : NodeBuffer_1007Struct;

struct NodeBuffer_994Struct {
	value : array< vec4<f32> >
};
@binding( 4 ) @group( 1 )
var<storage, read> NodeBuffer_994 : NodeBuffer_994Struct;

struct NodeBuffer_996Struct {
	value : array< u32 >
};
@binding( 5 ) @group( 1 )
var<storage, read> NodeBuffer_996 : NodeBuffer_996Struct;

struct NodeBuffer_993Struct {
	value : array< vec4<f32> >
};
@binding( 6 ) @group( 1 )
var<storage, read> NodeBuffer_993 : NodeBuffer_993Struct;

struct NodeBuffer_995Struct {
	value : array< vec2<f32> >
};
@binding( 7 ) @group( 1 )
var<storage, read> NodeBuffer_995 : NodeBuffer_995Struct;

struct renderStruct {
	nodeUniform1 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform2 : u32,
	nodeUniform7 : mat4x4<f32>
};
@binding( 1 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : f32;
var<private> nodeVar1 : u32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : u32;
var<private> nodeVar6 : vec2<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : vec2<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : vec2<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec2<f32>;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : u32;
var<private> nodeVar21 : vec2<f32>;
var<private> nodeVar22 : vec2<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec2<f32>;
var<private> nodeVar25 : vec4<f32>;
var<private> nodeVar26 : vec2<f32>;
var<private> nodeVar27 : vec4<f32>;
var<private> nodeVar28 : vec2<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec4<f32>;
var<private> nodeVar31 : vec2<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : vec2<f32>;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : vec4<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec2<f32>;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : vec4<f32>;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : vec4<f32>;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : vec4<f32>;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : vec4<f32>;

// codes


@fragment
fn main( @builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = ( render.nodeUniform1.y - fragCoord.xy.y );
	nodeVar1 = ( ( u32( nodeVar0 ) * u32( render.nodeUniform1.x ) ) + u32( fragCoord.xy.x ) );
	nodeVar2 = ( f32( ( NodeBuffer_1005.value[ nodeVar1 ] >> 16u ) ) / 65535.0 );
	nodeVar3 = ( nodeVar2 * nodeVar2 );
	output.depth = ( 1.0 - ( nodeVar3 * nodeVar3 ) );

	if ( ( f32( ( NodeBuffer_1005.value[ nodeVar1 ] >> 16u ) ) == 0.0 ) ) {

		discard;
		

	}

	nodeVar4 = vec4<f32>( 0.0, 0.0, 0.0, 0.0 );

	if ( ( f32( object.nodeUniform2 ) == 1.0 ) ) {

		nodeVar5 = ( NodeBuffer_1005.value[ nodeVar1 ] & 65535u );
		nodeVar6 = vec2<f32>( fragCoord.xy.x, nodeVar0 );
		nodeVar7 = ( object.nodeUniform7 * vec4<f32>( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar5 * 3u ) + 1u ) ] ] ).xyz, 1.0 ) );
		nodeVar8 = ( ( ( ( nodeVar7.xyz / vec3<f32>( nodeVar7.w ) ).xy + vec2<f32>( 1.0 ) ) * vec2<f32>( 0.5 ) ) * vec2<f32>( render.nodeUniform1.x, render.nodeUniform1.y ) );
		nodeVar9 = ( object.nodeUniform7 * vec4<f32>( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar5 * 3u ) + 2u ) ] ] ).xyz, 1.0 ) );
		nodeVar10 = ( ( ( ( nodeVar9.xyz / vec3<f32>( nodeVar9.w ) ).xy + vec2<f32>( 1.0 ) ) * vec2<f32>( 0.5 ) ) * vec2<f32>( render.nodeUniform1.x, render.nodeUniform1.y ) );
		nodeVar12 = ( object.nodeUniform7 * vec4<f32>( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar5 * 3u ) + 0u ) ] ] ).xyz, 1.0 ) );
		nodeVar13 = ( ( ( ( nodeVar12.xyz / vec3<f32>( nodeVar12.w ) ).xy + vec2<f32>( 1.0 ) ) * vec2<f32>( 0.5 ) ) * vec2<f32>( render.nodeUniform1.x, render.nodeUniform1.y ) );
		nodeVar14 = ( ( ( nodeVar10.y - nodeVar13.y ) * ( nodeVar8.x - nodeVar13.x ) ) - ( ( nodeVar10.x - nodeVar13.x ) * ( nodeVar8.y - nodeVar13.y ) ) );

		if ( ( nodeVar14 == 0.0 ) ) {

			nodeVar11 = 1.0;

		} else {

			nodeVar11 = nodeVar14;

		}

		nodeVar15 = ( ( ( ( nodeVar6.y - nodeVar8.y ) * ( nodeVar10.x - nodeVar8.x ) ) - ( ( nodeVar6.x - nodeVar8.x ) * ( nodeVar10.y - nodeVar8.y ) ) ) / nodeVar11 );
		nodeVar17 = ( ( ( ( nodeVar6.y - nodeVar10.y ) * ( nodeVar13.x - nodeVar10.x ) ) - ( ( nodeVar6.x - nodeVar10.x ) * ( nodeVar13.y - nodeVar10.y ) ) ) / nodeVar11 );
		nodeVar18 = ( ( ( ( nodeVar6.y - nodeVar13.y ) * ( nodeVar8.x - nodeVar13.x ) ) - ( ( nodeVar6.x - nodeVar13.x ) * ( nodeVar8.y - nodeVar13.y ) ) ) / nodeVar11 );
		nodeVar19 = ( ( ( nodeVar15 / nodeVar12.w ) + ( nodeVar17 / nodeVar7.w ) ) + ( nodeVar18 / nodeVar9.w ) );

		if ( ( nodeVar19 == 0.0 ) ) {

			nodeVar16 = 1.0;

		} else {

			nodeVar16 = nodeVar19;

		}

		nodeVar4 = vec4<f32>( ( ( normalize( ( ( ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * vec4<f32>( NodeBuffer_994.value[ NodeBuffer_996.value[ ( ( nodeVar5 * 3u ) + 0u ) ] ].xyz, 0.0 ) ).xyz * vec3<f32>( ( ( nodeVar15 / nodeVar12.w ) / nodeVar16 ) ) ) + ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * vec4<f32>( NodeBuffer_994.value[ NodeBuffer_996.value[ ( ( nodeVar5 * 3u ) + 1u ) ] ].xyz, 0.0 ) ).xyz * vec3<f32>( ( ( nodeVar17 / nodeVar7.w ) / nodeVar16 ) ) ) ) + ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * vec4<f32>( NodeBuffer_994.value[ NodeBuffer_996.value[ ( ( nodeVar5 * 3u ) + 2u ) ] ].xyz, 0.0 ) ).xyz * vec3<f32>( ( ( nodeVar18 / nodeVar9.w ) / nodeVar16 ) ) ) ) ) * vec3<f32>( 0.5 ) ) + vec3<f32>( 0.5 ) ), 1.0 );
		

	} else {


		if ( ( f32( object.nodeUniform2 ) == 2.0 ) ) {

			nodeVar20 = ( NodeBuffer_1005.value[ nodeVar1 ] & 65535u );
			nodeVar21 = ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 2u ) ] ] - NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 0u ) ] ] );
			nodeVar22 = ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 1u ) ] ] - NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 0u ) ] ] );
			nodeVar23 = ( ( ( ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 1u ) ] ] ).xyz - ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 0u ) ] ] ).xyz ) * vec3<f32>( nodeVar21.y ) ) - ( ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 2u ) ] ] ).xyz - ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 0u ) ] ] ).xyz ) * vec3<f32>( nodeVar22.y ) ) ) * vec3<f32>( sign( ( ( nodeVar22.x * nodeVar21.y ) - ( nodeVar22.y * nodeVar21.x ) ) ) ) );
			nodeVar24 = vec2<f32>( fragCoord.xy.x, nodeVar0 );
			nodeVar25 = ( object.nodeUniform7 * vec4<f32>( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 1u ) ] ] ).xyz, 1.0 ) );
			nodeVar26 = ( ( ( ( nodeVar25.xyz / vec3<f32>( nodeVar25.w ) ).xy + vec2<f32>( 1.0 ) ) * vec2<f32>( 0.5 ) ) * vec2<f32>( render.nodeUniform1.x, render.nodeUniform1.y ) );
			nodeVar27 = ( object.nodeUniform7 * vec4<f32>( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 2u ) ] ] ).xyz, 1.0 ) );
			nodeVar28 = ( ( ( ( nodeVar27.xyz / vec3<f32>( nodeVar27.w ) ).xy + vec2<f32>( 1.0 ) ) * vec2<f32>( 0.5 ) ) * vec2<f32>( render.nodeUniform1.x, render.nodeUniform1.y ) );
			nodeVar30 = ( object.nodeUniform7 * vec4<f32>( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * NodeBuffer_993.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 0u ) ] ] ).xyz, 1.0 ) );
			nodeVar31 = ( ( ( ( nodeVar30.xyz / vec3<f32>( nodeVar30.w ) ).xy + vec2<f32>( 1.0 ) ) * vec2<f32>( 0.5 ) ) * vec2<f32>( render.nodeUniform1.x, render.nodeUniform1.y ) );
			nodeVar32 = ( ( ( nodeVar28.y - nodeVar31.y ) * ( nodeVar26.x - nodeVar31.x ) ) - ( ( nodeVar28.x - nodeVar31.x ) * ( nodeVar26.y - nodeVar31.y ) ) );

			if ( ( nodeVar32 == 0.0 ) ) {

				nodeVar29 = 1.0;

			} else {

				nodeVar29 = nodeVar32;

			}

			nodeVar33 = ( ( ( ( nodeVar24.y - nodeVar26.y ) * ( nodeVar28.x - nodeVar26.x ) ) - ( ( nodeVar24.x - nodeVar26.x ) * ( nodeVar28.y - nodeVar26.y ) ) ) / nodeVar29 );
			nodeVar35 = ( ( ( ( nodeVar24.y - nodeVar28.y ) * ( nodeVar31.x - nodeVar28.x ) ) - ( ( nodeVar24.x - nodeVar28.x ) * ( nodeVar31.y - nodeVar28.y ) ) ) / nodeVar29 );
			nodeVar36 = ( ( ( ( nodeVar24.y - nodeVar31.y ) * ( nodeVar26.x - nodeVar31.x ) ) - ( ( nodeVar24.x - nodeVar31.x ) * ( nodeVar26.y - nodeVar31.y ) ) ) / nodeVar29 );
			nodeVar37 = ( ( ( nodeVar33 / nodeVar30.w ) + ( nodeVar35 / nodeVar25.w ) ) + ( nodeVar36 / nodeVar27.w ) );

			if ( ( nodeVar37 == 0.0 ) ) {

				nodeVar34 = 1.0;

			} else {

				nodeVar34 = nodeVar37;

			}

			nodeVar38 = ( ( nodeVar33 / nodeVar30.w ) / nodeVar34 );
			nodeVar39 = ( ( nodeVar35 / nodeVar25.w ) / nodeVar34 );
			nodeVar40 = ( ( nodeVar36 / nodeVar27.w ) / nodeVar34 );
			nodeVar41 = normalize( ( ( ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * vec4<f32>( NodeBuffer_994.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 0u ) ] ].xyz, 0.0 ) ).xyz * vec3<f32>( nodeVar38 ) ) + ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * vec4<f32>( NodeBuffer_994.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 1u ) ] ].xyz, 0.0 ) ).xyz * vec3<f32>( nodeVar39 ) ) ) + ( ( NodeBuffer_1012.value[ ( NodeBuffer_1007.value[ nodeVar1 ] & 131071u ) ] * vec4<f32>( NodeBuffer_994.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 2u ) ] ].xyz, 0.0 ) ).xyz * vec3<f32>( nodeVar40 ) ) ) );
			nodeVar42 = normalize( ( nodeVar23 - ( nodeVar41 * vec3<f32>( dot( nodeVar41, nodeVar23 ) ) ) ) );
			nodeVar43 = ( ( ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 0u ) ] ] * vec2<f32>( nodeVar38 ) ) + ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 1u ) ] ] * vec2<f32>( nodeVar39 ) ) ) + ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 2u ) ] ] * vec2<f32>( nodeVar40 ) ) );
			nodeVar44 = ( 1.0 / nodeVar30.w );
			nodeVar45 = ( 1.0 / nodeVar25.w );
			nodeVar46 = ( 1.0 / nodeVar27.w );
			nodeVar48 = ( ( ( nodeVar24.y - nodeVar26.y ) * ( nodeVar28.x - nodeVar26.x ) ) - ( ( nodeVar24.x - nodeVar26.x ) * ( nodeVar28.y - nodeVar26.y ) ) );
			nodeVar49 = ( ( ( nodeVar24.y - nodeVar28.y ) * ( nodeVar31.x - nodeVar28.x ) ) - ( ( nodeVar24.x - nodeVar28.x ) * ( nodeVar31.y - nodeVar28.y ) ) );
			nodeVar50 = ( ( ( nodeVar24.y - nodeVar31.y ) * ( nodeVar26.x - nodeVar31.x ) ) - ( ( nodeVar24.x - nodeVar31.x ) * ( nodeVar26.y - nodeVar31.y ) ) );
			nodeVar51 = ( ( ( nodeVar48 * nodeVar44 ) + ( nodeVar49 * nodeVar45 ) ) + ( nodeVar50 * nodeVar46 ) );

			if ( ( nodeVar51 == 0.0 ) ) {

				nodeVar47 = 1.0;

			} else {

				nodeVar47 = nodeVar51;

			}

			nodeVar52 = textureSampleGrad( nodeUniform10, nodeUniform10_sampler, nodeVar43, ( ( ( ( vec2<f32>( ( ( nodeVar28.y - nodeVar26.y ) * nodeVar44 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 0u ) ] ] - nodeVar43 ) ) + ( vec2<f32>( ( ( nodeVar31.y - nodeVar28.y ) * nodeVar45 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 1u ) ] ] - nodeVar43 ) ) ) + ( vec2<f32>( ( ( nodeVar26.y - nodeVar31.y ) * nodeVar46 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 2u ) ] ] - nodeVar43 ) ) ) / vec2<f32>( nodeVar47 ) ), ( ( ( ( vec2<f32>( ( ( nodeVar26.x - nodeVar28.x ) * nodeVar44 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 0u ) ] ] - nodeVar43 ) ) + ( vec2<f32>( ( ( nodeVar28.x - nodeVar31.x ) * nodeVar45 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 1u ) ] ] - nodeVar43 ) ) ) + ( vec2<f32>( ( ( nodeVar31.x - nodeVar26.x ) * nodeVar46 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 2u ) ] ] - nodeVar43 ) ) ) / nodeVar47 ) );
			nodeVar53 = ( ( nodeVar52.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) );
			nodeVar4 = vec4<f32>( ( ( normalize( ( ( ( nodeVar42 * vec3<f32>( nodeVar53.x ) ) + ( cross( nodeVar41, nodeVar42 ) * vec3<f32>( nodeVar53.y ) ) ) + ( nodeVar41 * vec3<f32>( nodeVar53.z ) ) ) ) * vec3<f32>( 0.5 ) ) + vec3<f32>( 0.5 ) ), 1.0 );
			

		} else {


			if ( ( f32( object.nodeUniform2 ) == 3.0 ) ) {

				nodeVar54 = ( ( ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 0u ) ] ] * vec2<f32>( nodeVar38 ) ) + ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 1u ) ] ] * vec2<f32>( nodeVar39 ) ) ) + ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 2u ) ] ] * vec2<f32>( nodeVar40 ) ) );
				nodeVar4 = vec4<f32>( nodeVar54, 0.0, 1.0 );
				

			} else {


				if ( ( f32( object.nodeUniform2 ) == 4.0 ) ) {

					nodeVar55 = ( 1.0 / nodeVar30.w );
					nodeVar56 = ( 1.0 / nodeVar25.w );
					nodeVar57 = ( 1.0 / nodeVar27.w );
					nodeVar59 = ( ( ( nodeVar48 * nodeVar55 ) + ( nodeVar49 * nodeVar56 ) ) + ( nodeVar50 * nodeVar57 ) );

					if ( ( nodeVar59 == 0.0 ) ) {

						nodeVar58 = 1.0;

					} else {

						nodeVar58 = nodeVar59;

					}

					nodeVar60 = textureSampleGrad( nodeUniform11, nodeUniform11_sampler, nodeVar54, ( ( ( ( vec2<f32>( ( ( nodeVar28.y - nodeVar26.y ) * nodeVar55 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 0u ) ] ] - nodeVar54 ) ) + ( vec2<f32>( ( ( nodeVar31.y - nodeVar28.y ) * nodeVar56 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 1u ) ] ] - nodeVar54 ) ) ) + ( vec2<f32>( ( ( nodeVar26.y - nodeVar31.y ) * nodeVar57 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 2u ) ] ] - nodeVar54 ) ) ) / vec2<f32>( nodeVar58 ) ), ( ( ( ( vec2<f32>( ( ( nodeVar26.x - nodeVar28.x ) * nodeVar55 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 0u ) ] ] - nodeVar54 ) ) + ( vec2<f32>( ( ( nodeVar28.x - nodeVar31.x ) * nodeVar56 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 1u ) ] ] - nodeVar54 ) ) ) + ( vec2<f32>( ( ( nodeVar31.x - nodeVar26.x ) * nodeVar57 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 2u ) ] ] - nodeVar54 ) ) ) / nodeVar58 ) );
					nodeVar4 = vec4<f32>( nodeVar60.y, nodeVar60.y, nodeVar60.y, 1.0 );
					

				} else {


					if ( ( f32( object.nodeUniform2 ) == 5.0 ) ) {

						nodeVar61 = ( 1.0 / nodeVar30.w );
						nodeVar62 = ( 1.0 / nodeVar25.w );
						nodeVar63 = ( 1.0 / nodeVar27.w );
						nodeVar65 = ( ( ( nodeVar48 * nodeVar61 ) + ( nodeVar49 * nodeVar62 ) ) + ( nodeVar50 * nodeVar63 ) );

						if ( ( nodeVar65 == 0.0 ) ) {

							nodeVar64 = 1.0;

						} else {

							nodeVar64 = nodeVar65;

						}

						nodeVar66 = textureSampleGrad( nodeUniform11, nodeUniform11_sampler, nodeVar54, ( ( ( ( vec2<f32>( ( ( nodeVar28.y - nodeVar26.y ) * nodeVar61 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 0u ) ] ] - nodeVar54 ) ) + ( vec2<f32>( ( ( nodeVar31.y - nodeVar28.y ) * nodeVar62 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 1u ) ] ] - nodeVar54 ) ) ) + ( vec2<f32>( ( ( nodeVar26.y - nodeVar31.y ) * nodeVar63 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 2u ) ] ] - nodeVar54 ) ) ) / vec2<f32>( nodeVar64 ) ), ( ( ( ( vec2<f32>( ( ( nodeVar26.x - nodeVar28.x ) * nodeVar61 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 0u ) ] ] - nodeVar54 ) ) + ( vec2<f32>( ( ( nodeVar28.x - nodeVar31.x ) * nodeVar62 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 1u ) ] ] - nodeVar54 ) ) ) + ( vec2<f32>( ( ( nodeVar31.x - nodeVar26.x ) * nodeVar63 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 2u ) ] ] - nodeVar54 ) ) ) / nodeVar64 ) );
						nodeVar4 = vec4<f32>( nodeVar66.z, nodeVar66.z, nodeVar66.z, 1.0 );
						

					} else {


						if ( ( f32( object.nodeUniform2 ) == 6.0 ) ) {

							nodeVar67 = ( 1.0 / nodeVar30.w );
							nodeVar68 = ( 1.0 / nodeVar25.w );
							nodeVar69 = ( 1.0 / nodeVar27.w );
							nodeVar71 = ( ( ( nodeVar48 * nodeVar67 ) + ( nodeVar49 * nodeVar68 ) ) + ( nodeVar50 * nodeVar69 ) );

							if ( ( nodeVar71 == 0.0 ) ) {

								nodeVar70 = 1.0;

							} else {

								nodeVar70 = nodeVar71;

							}

							nodeVar72 = textureSampleGrad( nodeUniform12, nodeUniform12_sampler, nodeVar54, ( ( ( ( vec2<f32>( ( ( nodeVar28.y - nodeVar26.y ) * nodeVar67 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 0u ) ] ] - nodeVar54 ) ) + ( vec2<f32>( ( ( nodeVar31.y - nodeVar28.y ) * nodeVar68 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 1u ) ] ] - nodeVar54 ) ) ) + ( vec2<f32>( ( ( nodeVar26.y - nodeVar31.y ) * nodeVar69 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 2u ) ] ] - nodeVar54 ) ) ) / vec2<f32>( nodeVar70 ) ), ( ( ( ( vec2<f32>( ( ( nodeVar26.x - nodeVar28.x ) * nodeVar67 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 0u ) ] ] - nodeVar54 ) ) + ( vec2<f32>( ( ( nodeVar28.x - nodeVar31.x ) * nodeVar68 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 1u ) ] ] - nodeVar54 ) ) ) + ( vec2<f32>( ( ( nodeVar31.x - nodeVar26.x ) * nodeVar69 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 2u ) ] ] - nodeVar54 ) ) ) / nodeVar70 ) );
							nodeVar4 = vec4<f32>( nodeVar72.x, nodeVar72.x, nodeVar72.x, 1.0 );
							

						} else {


							if ( ( f32( object.nodeUniform2 ) == 7.0 ) ) {

								nodeVar73 = ( 1.0 / nodeVar30.w );
								nodeVar74 = ( 1.0 / nodeVar25.w );
								nodeVar75 = ( 1.0 / nodeVar27.w );
								nodeVar77 = ( ( ( nodeVar48 * nodeVar73 ) + ( nodeVar49 * nodeVar74 ) ) + ( nodeVar50 * nodeVar75 ) );

								if ( ( nodeVar77 == 0.0 ) ) {

									nodeVar76 = 1.0;

								} else {

									nodeVar76 = nodeVar77;

								}

								nodeVar78 = textureSampleGrad( nodeUniform13, nodeUniform13_sampler, nodeVar54, ( ( ( ( vec2<f32>( ( ( nodeVar28.y - nodeVar26.y ) * nodeVar73 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 0u ) ] ] - nodeVar54 ) ) + ( vec2<f32>( ( ( nodeVar31.y - nodeVar28.y ) * nodeVar74 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 1u ) ] ] - nodeVar54 ) ) ) + ( vec2<f32>( ( ( nodeVar26.y - nodeVar31.y ) * nodeVar75 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 2u ) ] ] - nodeVar54 ) ) ) / vec2<f32>( nodeVar76 ) ), ( ( ( ( vec2<f32>( ( ( nodeVar26.x - nodeVar28.x ) * nodeVar73 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 0u ) ] ] - nodeVar54 ) ) + ( vec2<f32>( ( ( nodeVar28.x - nodeVar31.x ) * nodeVar74 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 1u ) ] ] - nodeVar54 ) ) ) + ( vec2<f32>( ( ( nodeVar31.x - nodeVar26.x ) * nodeVar75 ) ) * ( NodeBuffer_995.value[ NodeBuffer_996.value[ ( ( nodeVar20 * 3u ) + 2u ) ] ] - nodeVar54 ) ) ) / nodeVar76 ) );
								nodeVar4 = vec4<f32>( nodeVar78.xyz, 1.0 );
								

							}

							

						}

						

					}

					

				}

				

			}

			

		}

		

	}


	// result

	output.color = nodeVar4;

	return output;

}
