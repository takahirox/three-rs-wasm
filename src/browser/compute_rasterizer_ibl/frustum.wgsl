// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct NodeBuffer_1013Struct {
	value : array< mat4x4<f32> >
};
@binding( 0 ) @group( 1 )
var<storage, read_write> NodeBuffer_1013 : NodeBuffer_1013Struct;

struct NodeBuffer_1010Struct {
	value : array< mat4x4<f32> >
};
@binding( 1 ) @group( 1 )
var<storage, read_write> NodeBuffer_1010 : NodeBuffer_1010Struct;

struct NodeBuffer_1023Struct {
	value : array< vec4<f32>, 6 >
};
@binding( 2 ) @group( 1 )
var<uniform> NodeBuffer_1023 : NodeBuffer_1023Struct;

struct NodeBuffer_1001Struct {
	value : array< vec4<f32> >
};
@binding( 3 ) @group( 1 )
var<storage, read_write> NodeBuffer_1001 : NodeBuffer_1001Struct;

struct NodeBuffer_1009Struct {
	value : array< f32 >
};
@binding( 5 ) @group( 1 )
var<storage, read> NodeBuffer_1009 : NodeBuffer_1009Struct;

struct NodeBuffer_1002Struct {
	value : array< vec4<f32>, 16 >
};
@binding( 6 ) @group( 1 )
var<uniform> NodeBuffer_1002 : NodeBuffer_1002Struct;

struct NodeBuffer_991Struct {
	value : array< vec4<f32>, 6 >
};
@binding( 7 ) @group( 1 )
var<uniform> NodeBuffer_991 : NodeBuffer_991Struct;

struct NodeBuffer_992Struct {
	value : array< vec4<f32> >
};
@binding( 8 ) @group( 1 )
var<storage, read> NodeBuffer_992 : NodeBuffer_992Struct;

struct NodeBuffer_1014Struct {
	value : array< atomic<u32> >
};
@binding( 9 ) @group( 1 )
var<storage, read_write> NodeBuffer_1014 : NodeBuffer_1014Struct;

struct NodeBuffer_1017Struct {
	value : array< vec4<u32> >
};
@binding( 10 ) @group( 1 )
var<storage, read_write> NodeBuffer_1017 : NodeBuffer_1017Struct;

struct NodeBuffer_1011Struct {
	value : array< mat4x4<f32> >
};
@binding( 11 ) @group( 1 )
var<storage, read_write> NodeBuffer_1011 : NodeBuffer_1011Struct;

struct objectStruct {
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : mat4x4<f32>,
	nodeUniform8 : f32,
	nodeUniform10 : f32,
	nodeUniform11 : f32,
	nodeUniform12 : vec3<f32>,
	nodeUniform14 : f32,
	nodeUniform17 : f32,
	nodeUniform22 : mat4x4<f32>,
	nodeUniform23 : u32
};
@binding( 4 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform16 : f32,
	nodeUniform9 : vec2<f32>,
	nodeUniform13 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> nodeVar0 : bool;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec2<f32>;
var<private> nodeVar7 : u32;
var<private> nodeVar8 : u32;
var<private> nodeVar9 : u32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : mat4x4<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : bool;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : vec4<f32>;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : vec2<f32>;
var<private> nodeVar24 : u32;
var<private> nodeVar25 : u32;

// codes


@compute @workgroup_size( 64, 1, 1 )
fn main( @builtin( global_invocation_id ) globalId : vec3<u32>,
	@builtin( workgroup_id ) workgroupId : vec3<u32>,
	@builtin( local_invocation_id ) localId : vec3<u32>,
	@builtin( num_workgroups ) numWorkgroups : vec3<u32> ) {

	// local vars
	

	// system
	instanceIndex = globalId.x
		+ globalId.y * ( 64 * numWorkgroups.x )
		+ globalId.z * ( 64 * numWorkgroups.x ) * ( 1 * numWorkgroups.y );

	// flow
	// code


	// flow -> Compute Frustum
	if ( instanceIndex >= object.nodeUniform23 ) { return; }

	NodeBuffer_1013.value[ instanceIndex ] = NodeBuffer_1010.value[ instanceIndex ];
	nodeVar0 = true;

	for ( var i : i32 = 0; i < 6; i ++ ) {

		nodeVar1 = ( NodeBuffer_1001.value[ instanceIndex ].w * 1.335795155406091 );

		if ( ( ( dot( NodeBuffer_1023.value[ i ].xyz, NodeBuffer_1001.value[ instanceIndex ].xyz ) + NodeBuffer_1023.value[ i ].w ) < ( - nodeVar1 ) ) ) {

			nodeVar0 = false;
			

		}


	}


	if ( nodeVar0 ) {

		nodeVar2 = ( object.nodeUniform4 - NodeBuffer_1001.value[ instanceIndex ].xyz );
		nodeVar3 = length( nodeVar2 );
		nodeVar4 = ( object.nodeUniform5 * vec4<f32>( ( NodeBuffer_1001.value[ instanceIndex ].xyz + ( ( nodeVar2 / vec3<f32>( nodeVar3 ) ) * vec3<f32>( nodeVar1 ) ) ), 1.0 ) );
		nodeVar5 = ( object.nodeUniform5 * vec4<f32>( NodeBuffer_1001.value[ instanceIndex ].xyz, 1.0 ) );
		nodeVar6 = ( nodeVar5.xy / vec2<f32>( nodeVar5.w ) );
		nodeVar7 = min( ( u32( clamp( ( ( ( ( nodeVar6.x * 0.5 ) + 0.5 ) * f32( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar1 * object.nodeUniform8 ) * render.nodeUniform9.y ) / 4.0 ) / nodeVar3 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].y ) ) ) - 0.5 ), 0.0, f32( ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar1 * object.nodeUniform8 ) * render.nodeUniform9.y ) / 4.0 ) / nodeVar3 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].y ) - 1u ) ) ) ) + 1u ), ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar1 * object.nodeUniform8 ) * render.nodeUniform9.y ) / 4.0 ) / nodeVar3 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].y ) - 1u ) );
		nodeVar8 = min( ( u32( clamp( ( ( ( 0.5 - ( nodeVar6.y * 0.5 ) ) * f32( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar1 * object.nodeUniform8 ) * render.nodeUniform9.y ) / 4.0 ) / nodeVar3 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].z ) ) ) - 0.5 ), 0.0, f32( ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar1 * object.nodeUniform8 ) * render.nodeUniform9.y ) / 4.0 ) / nodeVar3 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].z ) - 1u ) ) ) ) + 1u ), ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar1 * object.nodeUniform8 ) * render.nodeUniform9.y ) / 4.0 ) / nodeVar3 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].z ) - 1u ) );
		nodeVar0 = ( ! ( ( ( ( nodeVar3 > ( nodeVar1 * 2.0 ) ) && ( nodeVar4.w > 0.0 ) ) && ( nodeVar5.w > 0.0 ) ) && ( ( nodeVar4.z / nodeVar4.w ) > ( max( max( NodeBuffer_1009.value[ ( ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar1 * object.nodeUniform8 ) * render.nodeUniform9.y ) / 4.0 ) / nodeVar3 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].x ) + ( u32( clamp( ( ( ( 0.5 - ( nodeVar6.y * 0.5 ) ) * f32( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar1 * object.nodeUniform8 ) * render.nodeUniform9.y ) / 4.0 ) / nodeVar3 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].z ) ) ) - 0.5 ), 0.0, f32( ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar1 * object.nodeUniform8 ) * render.nodeUniform9.y ) / 4.0 ) / nodeVar3 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].z ) - 1u ) ) ) ) * u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar1 * object.nodeUniform8 ) * render.nodeUniform9.y ) / 4.0 ) / nodeVar3 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].y ) ) ) + u32( clamp( ( ( ( ( nodeVar6.x * 0.5 ) + 0.5 ) * f32( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar1 * object.nodeUniform8 ) * render.nodeUniform9.y ) / 4.0 ) / nodeVar3 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].y ) ) ) - 0.5 ), 0.0, f32( ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar1 * object.nodeUniform8 ) * render.nodeUniform9.y ) / 4.0 ) / nodeVar3 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].y ) - 1u ) ) ) ) ) ], NodeBuffer_1009.value[ ( ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar1 * object.nodeUniform8 ) * render.nodeUniform9.y ) / 4.0 ) / nodeVar3 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].x ) + ( u32( clamp( ( ( ( 0.5 - ( nodeVar6.y * 0.5 ) ) * f32( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar1 * object.nodeUniform8 ) * render.nodeUniform9.y ) / 4.0 ) / nodeVar3 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].z ) ) ) - 0.5 ), 0.0, f32( ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar1 * object.nodeUniform8 ) * render.nodeUniform9.y ) / 4.0 ) / nodeVar3 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].z ) - 1u ) ) ) ) * u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar1 * object.nodeUniform8 ) * render.nodeUniform9.y ) / 4.0 ) / nodeVar3 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].y ) ) ) + nodeVar7 ) ] ), max( NodeBuffer_1009.value[ ( ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar1 * object.nodeUniform8 ) * render.nodeUniform9.y ) / 4.0 ) / nodeVar3 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].x ) + ( nodeVar8 * u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar1 * object.nodeUniform8 ) * render.nodeUniform9.y ) / 4.0 ) / nodeVar3 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].y ) ) ) + u32( clamp( ( ( ( ( nodeVar6.x * 0.5 ) + 0.5 ) * f32( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar1 * object.nodeUniform8 ) * render.nodeUniform9.y ) / 4.0 ) / nodeVar3 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].y ) ) ) - 0.5 ), 0.0, f32( ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar1 * object.nodeUniform8 ) * render.nodeUniform9.y ) / 4.0 ) / nodeVar3 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].y ) - 1u ) ) ) ) ) ], NodeBuffer_1009.value[ ( ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar1 * object.nodeUniform8 ) * render.nodeUniform9.y ) / 4.0 ) / nodeVar3 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].x ) + ( nodeVar8 * u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar1 * object.nodeUniform8 ) * render.nodeUniform9.y ) / 4.0 ) / nodeVar3 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].y ) ) ) + nodeVar7 ) ] ) ) + object.nodeUniform11 ) ) ) );
		

	}


	if ( nodeVar0 ) {

		nodeVar9 = 0u;
		nodeVar10 = ( ( ( object.nodeUniform8 / max( 0.01, distance( object.nodeUniform12, NodeBuffer_1001.value[ instanceIndex ].xyz ) ) ) * render.nodeUniform13.y ) / 2.0 );

		if ( ( ( ( 0.6034520117024409 * NodeBuffer_1001.value[ instanceIndex ].w ) * nodeVar10 ) <= object.nodeUniform14 ) ) {

			nodeVar9 = 5u;
			

		} else {


			if ( ( ( ( 0.22799676627254356 * NodeBuffer_1001.value[ instanceIndex ].w ) * nodeVar10 ) <= object.nodeUniform14 ) ) {

				nodeVar9 = 4u;
				

			} else {


				if ( ( ( ( 0.135767187809756 * NodeBuffer_1001.value[ instanceIndex ].w ) * nodeVar10 ) <= object.nodeUniform14 ) ) {

					nodeVar9 = 3u;
					

				} else {


					if ( ( ( ( 0.03710348089155274 * NodeBuffer_1001.value[ instanceIndex ].w ) * nodeVar10 ) <= object.nodeUniform14 ) ) {

						nodeVar9 = 2u;
						

					} else {


						if ( ( ( ( 0.0073041109693479656 * NodeBuffer_1001.value[ instanceIndex ].w ) * nodeVar10 ) <= object.nodeUniform14 ) ) {

							nodeVar9 = 1u;
							

						}

						

					}

					

				}

				

			}

			

		}


		for ( var cIdx : u32 = 0u; cIdx < ( ( u32( NodeBuffer_991.value[ nodeVar9 ].y ) + 63u ) / 64u ); cIdx ++ ) {

			nodeVar11 = ( ( render.nodeUniform16 * object.nodeUniform17 ) + f32( instanceIndex ) );
			nodeVar12 = cos( nodeVar11 );
			nodeVar13 = sin( nodeVar11 );
			nodeVar14 = mat4x4<f32>( vec4<f32>( ( nodeVar12 * NodeBuffer_1001.value[ instanceIndex ].w ), 0.0, ( nodeVar13 * NodeBuffer_1001.value[ instanceIndex ].w ), 0.0 ), vec4<f32>( 0.0, NodeBuffer_1001.value[ instanceIndex ].w, 0.0, 0.0 ), vec4<f32>( ( ( - nodeVar13 ) * NodeBuffer_1001.value[ instanceIndex ].w ), 0.0, ( nodeVar12 * NodeBuffer_1001.value[ instanceIndex ].w ), 0.0 ), vec4<f32>( NodeBuffer_1001.value[ instanceIndex ].xyz, 1.0 ) );
			nodeVar15 = ( nodeVar14 * vec4<f32>( NodeBuffer_992.value[ ( u32( NodeBuffer_991.value[ nodeVar9 ].z ) + cIdx ) ].xyz, 1.0 ) ).xyz;
			nodeVar16 = ( NodeBuffer_992.value[ ( u32( NodeBuffer_991.value[ nodeVar9 ].z ) + cIdx ) ].w * NodeBuffer_1001.value[ instanceIndex ].w );
			nodeVar17 = true;

			for ( var pIdx : i32 = 0; pIdx < 6; pIdx ++ ) {


				if ( ( ( dot( NodeBuffer_1023.value[ pIdx ].xyz, nodeVar15 ) + NodeBuffer_1023.value[ pIdx ].w ) < ( - nodeVar16 ) ) ) {

					nodeVar17 = false;
					

				}


			}


			if ( nodeVar17 ) {

				nodeVar18 = ( NodeBuffer_1013.value[ instanceIndex ] * vec4<f32>( NodeBuffer_992.value[ ( u32( NodeBuffer_991.value[ nodeVar9 ].z ) + cIdx ) ].xyz, 1.0 ) ).xyz;
				nodeVar19 = ( object.nodeUniform4 - nodeVar18 );
				nodeVar20 = length( nodeVar19 );
				nodeVar21 = ( object.nodeUniform5 * vec4<f32>( ( nodeVar18 + ( ( nodeVar19 / vec3<f32>( nodeVar20 ) ) * vec3<f32>( nodeVar16 ) ) ), 1.0 ) );
				nodeVar22 = ( object.nodeUniform5 * vec4<f32>( nodeVar18, 1.0 ) );
				nodeVar23 = ( nodeVar22.xy / vec2<f32>( nodeVar22.w ) );
				nodeVar24 = min( ( u32( clamp( ( ( ( ( nodeVar23.x * 0.5 ) + 0.5 ) * f32( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar16 * object.nodeUniform8 ) * render.nodeUniform13.y ) / 4.0 ) / nodeVar20 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].y ) ) ) - 0.5 ), 0.0, f32( ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar16 * object.nodeUniform8 ) * render.nodeUniform13.y ) / 4.0 ) / nodeVar20 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].y ) - 1u ) ) ) ) + 1u ), ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar16 * object.nodeUniform8 ) * render.nodeUniform13.y ) / 4.0 ) / nodeVar20 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].y ) - 1u ) );
				nodeVar25 = min( ( u32( clamp( ( ( ( 0.5 - ( nodeVar23.y * 0.5 ) ) * f32( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar16 * object.nodeUniform8 ) * render.nodeUniform13.y ) / 4.0 ) / nodeVar20 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].z ) ) ) - 0.5 ), 0.0, f32( ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar16 * object.nodeUniform8 ) * render.nodeUniform13.y ) / 4.0 ) / nodeVar20 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].z ) - 1u ) ) ) ) + 1u ), ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar16 * object.nodeUniform8 ) * render.nodeUniform13.y ) / 4.0 ) / nodeVar20 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].z ) - 1u ) );
				nodeVar17 = ( ! ( ( ( ( nodeVar20 > ( nodeVar16 * 2.0 ) ) && ( nodeVar21.w > 0.0 ) ) && ( nodeVar22.w > 0.0 ) ) && ( ( nodeVar21.z / nodeVar21.w ) > ( max( max( NodeBuffer_1009.value[ ( ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar16 * object.nodeUniform8 ) * render.nodeUniform13.y ) / 4.0 ) / nodeVar20 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].x ) + ( u32( clamp( ( ( ( 0.5 - ( nodeVar23.y * 0.5 ) ) * f32( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar16 * object.nodeUniform8 ) * render.nodeUniform13.y ) / 4.0 ) / nodeVar20 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].z ) ) ) - 0.5 ), 0.0, f32( ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar16 * object.nodeUniform8 ) * render.nodeUniform13.y ) / 4.0 ) / nodeVar20 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].z ) - 1u ) ) ) ) * u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar16 * object.nodeUniform8 ) * render.nodeUniform13.y ) / 4.0 ) / nodeVar20 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].y ) ) ) + u32( clamp( ( ( ( ( nodeVar23.x * 0.5 ) + 0.5 ) * f32( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar16 * object.nodeUniform8 ) * render.nodeUniform13.y ) / 4.0 ) / nodeVar20 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].y ) ) ) - 0.5 ), 0.0, f32( ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar16 * object.nodeUniform8 ) * render.nodeUniform13.y ) / 4.0 ) / nodeVar20 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].y ) - 1u ) ) ) ) ) ], NodeBuffer_1009.value[ ( ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar16 * object.nodeUniform8 ) * render.nodeUniform13.y ) / 4.0 ) / nodeVar20 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].x ) + ( u32( clamp( ( ( ( 0.5 - ( nodeVar23.y * 0.5 ) ) * f32( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar16 * object.nodeUniform8 ) * render.nodeUniform13.y ) / 4.0 ) / nodeVar20 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].z ) ) ) - 0.5 ), 0.0, f32( ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar16 * object.nodeUniform8 ) * render.nodeUniform13.y ) / 4.0 ) / nodeVar20 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].z ) - 1u ) ) ) ) * u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar16 * object.nodeUniform8 ) * render.nodeUniform13.y ) / 4.0 ) / nodeVar20 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].y ) ) ) + nodeVar24 ) ] ), max( NodeBuffer_1009.value[ ( ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar16 * object.nodeUniform8 ) * render.nodeUniform13.y ) / 4.0 ) / nodeVar20 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].x ) + ( nodeVar25 * u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar16 * object.nodeUniform8 ) * render.nodeUniform13.y ) / 4.0 ) / nodeVar20 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].y ) ) ) + u32( clamp( ( ( ( ( nodeVar23.x * 0.5 ) + 0.5 ) * f32( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar16 * object.nodeUniform8 ) * render.nodeUniform13.y ) / 4.0 ) / nodeVar20 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].y ) ) ) - 0.5 ), 0.0, f32( ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar16 * object.nodeUniform8 ) * render.nodeUniform13.y ) / 4.0 ) / nodeVar20 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].y ) - 1u ) ) ) ) ) ], NodeBuffer_1009.value[ ( ( u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar16 * object.nodeUniform8 ) * render.nodeUniform13.y ) / 4.0 ) / nodeVar20 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].x ) + ( nodeVar25 * u32( NodeBuffer_1002.value[ i32( clamp( ceil( log2( max( ( ( ( ( ( nodeVar16 * object.nodeUniform8 ) * render.nodeUniform13.y ) / 4.0 ) / nodeVar20 ) * 2.0 ), 1.0 ) ) ), 0.0, ( object.nodeUniform10 - 1.0 ) ) ) ].y ) ) ) + nodeVar24 ) ] ) ) + object.nodeUniform11 ) ) ) );
				

			}


			if ( nodeVar17 ) {

				let nodeConst0 = atomicAdd( &NodeBuffer_1014.value[ 0u ], 1u );

				if ( ( f32( nodeConst0 ) < 2820000.0 ) ) {

					NodeBuffer_1017.value[ nodeConst0 ] = vec4<u32>( instanceIndex, u32( NodeBuffer_991.value[ nodeVar9 ].x ), u32( NodeBuffer_991.value[ nodeVar9 ].y ), cIdx );
					

				}

				

			}


		}

		NodeBuffer_1010.value[ instanceIndex ] = nodeVar14;
		NodeBuffer_1011.value[ instanceIndex ] = ( object.nodeUniform22 * nodeVar14 );
		

	}


	

}
