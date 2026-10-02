// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct NodeBuffer_2193Struct {
	value : array< vec4<f32> >
};
@binding( 0 ) @group( 0 )
var<storage, read> NodeBuffer_2193 : NodeBuffer_2193Struct;

struct NodeBuffer_2197Struct {
	value : array< u32 >
};
@binding( 2 ) @group( 0 )
var<storage, read> NodeBuffer_2197 : NodeBuffer_2197Struct;

struct NodeBuffer_2198Struct {
	value : array< u32 >
};
@binding( 3 ) @group( 0 )
var<storage, read> NodeBuffer_2198 : NodeBuffer_2198Struct;

struct NodeBuffer_2199Struct {
	value : array< u32 >
};
@binding( 4 ) @group( 0 )
var<storage, read> NodeBuffer_2199 : NodeBuffer_2199Struct;

struct NodeBuffer_2347Struct {
	value : array< vec4<f32> >
};
@binding( 5 ) @group( 0 )
var<storage, read_write> NodeBuffer_2347 : NodeBuffer_2347Struct;

struct objectStruct {
	nodeUniform1 : vec3<f32>,
	nodeUniform6 : u32
};
@binding( 1 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> center : vec3<f32>;
var<private> sphericalHarmonicsContribution : vec3<f32>;
var<private> sphericalHarmonicsViewDirection : vec3<f32>;
var<private> nodeVar0 : u32;
var<private> nodeVar1 : u32;
var<private> nodeVar2 : u32;
var<private> sh1_0 : vec3<f32>;
var<private> sh1_1 : vec3<f32>;
var<private> sh1_2 : vec3<f32>;
var<private> shXX : f32;
var<private> shYY : f32;
var<private> shZZ : f32;
var<private> nodeVar3 : u32;
var<private> nodeVar4 : u32;
var<private> nodeVar5 : u32;
var<private> nodeVar6 : u32;
var<private> sh2_0 : vec3<f32>;
var<private> sh2_1 : vec3<f32>;
var<private> sh2_2 : vec3<f32>;
var<private> sh2_3 : vec3<f32>;
var<private> sh2_4 : vec3<f32>;
var<private> shXY : f32;
var<private> nodeVar7 : u32;
var<private> nodeVar8 : u32;
var<private> nodeVar9 : u32;
var<private> nodeVar10 : u32;
var<private> nodeVar11 : u32;
var<private> nodeVar12 : u32;
var<private> sh3_0 : vec3<f32>;
var<private> sh3_1 : vec3<f32>;
var<private> sh3_2 : vec3<f32>;
var<private> sh3_3 : vec3<f32>;
var<private> sh3_4 : vec3<f32>;
var<private> sh3_5 : vec3<f32>;
var<private> sh3_6 : vec3<f32>;

// codes


@compute @workgroup_size( 256, 1, 1 )
fn main( @builtin( global_invocation_id ) globalId : vec3<u32>,
	@builtin( workgroup_id ) workgroupId : vec3<u32>,
	@builtin( local_invocation_id ) localId : vec3<u32>,
	@builtin( num_workgroups ) numWorkgroups : vec3<u32> ) {

	// local vars
	

	// system
	instanceIndex = globalId.x
		+ globalId.y * ( 256 * numWorkgroups.x )
		+ globalId.z * ( 256 * numWorkgroups.x ) * ( 1 * numWorkgroups.y );

	// flow
	// code


	// flow -> GaussianSplatSphericalHarmonics
	if ( instanceIndex >= object.nodeUniform6 ) { return; }

	center = NodeBuffer_2193.value[ instanceIndex ].xyz;
	sphericalHarmonicsContribution = vec3<f32>( 0.0, 0.0, 0.0 );
	sphericalHarmonicsViewDirection = normalize( ( center - object.nodeUniform1 ) );
	nodeVar0 = NodeBuffer_2197.value[ ( ( instanceIndex * 3u ) + 0u ) ];
	nodeVar1 = NodeBuffer_2197.value[ ( ( instanceIndex * 3u ) + 1u ) ];
	nodeVar2 = NodeBuffer_2197.value[ ( ( instanceIndex * 3u ) + 2u ) ];
	sh1_0 = vec3<f32>( ( ( f32( ( ( nodeVar0 >> 0u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar0 >> 8u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar0 >> 16u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sh1_1 = vec3<f32>( ( ( f32( ( ( nodeVar0 >> 24u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar1 >> 0u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar1 >> 8u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sh1_2 = vec3<f32>( ( ( f32( ( ( nodeVar1 >> 16u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar1 >> 24u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar2 >> 0u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sphericalHarmonicsContribution = ( sphericalHarmonicsContribution + ( ( ( sh1_0 * vec3<f32>( ( sphericalHarmonicsViewDirection.y * -0.4886025 ) ) ) + ( sh1_1 * vec3<f32>( ( sphericalHarmonicsViewDirection.z * 0.4886025 ) ) ) ) + ( sh1_2 * vec3<f32>( ( sphericalHarmonicsViewDirection.x * -0.4886025 ) ) ) ) );
	shXX = ( sphericalHarmonicsViewDirection.x * sphericalHarmonicsViewDirection.x );
	shYY = ( sphericalHarmonicsViewDirection.y * sphericalHarmonicsViewDirection.y );
	shZZ = ( sphericalHarmonicsViewDirection.z * sphericalHarmonicsViewDirection.z );
	nodeVar3 = NodeBuffer_2198.value[ ( ( instanceIndex * 4u ) + 0u ) ];
	nodeVar4 = NodeBuffer_2198.value[ ( ( instanceIndex * 4u ) + 1u ) ];
	nodeVar5 = NodeBuffer_2198.value[ ( ( instanceIndex * 4u ) + 2u ) ];
	nodeVar6 = NodeBuffer_2198.value[ ( ( instanceIndex * 4u ) + 3u ) ];
	sh2_0 = vec3<f32>( ( ( f32( ( ( nodeVar3 >> 0u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar3 >> 8u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar3 >> 16u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sh2_1 = vec3<f32>( ( ( f32( ( ( nodeVar3 >> 24u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar4 >> 0u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar4 >> 8u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sh2_2 = vec3<f32>( ( ( f32( ( ( nodeVar4 >> 16u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar4 >> 24u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar5 >> 0u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sh2_3 = vec3<f32>( ( ( f32( ( ( nodeVar5 >> 8u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar5 >> 16u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar5 >> 24u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sh2_4 = vec3<f32>( ( ( f32( ( ( nodeVar6 >> 0u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar6 >> 8u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar6 >> 16u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sphericalHarmonicsContribution = ( sphericalHarmonicsContribution + ( ( ( ( ( sh2_0 * vec3<f32>( ( ( sphericalHarmonicsViewDirection.x * sphericalHarmonicsViewDirection.y ) * 1.0925484 ) ) ) + ( sh2_1 * vec3<f32>( ( ( sphericalHarmonicsViewDirection.y * sphericalHarmonicsViewDirection.z ) * -1.0925484 ) ) ) ) + ( sh2_2 * vec3<f32>( ( ( ( ( shZZ * 2.0 ) - shXX ) - shYY ) * 0.3153915 ) ) ) ) + ( sh2_3 * vec3<f32>( ( ( sphericalHarmonicsViewDirection.x * sphericalHarmonicsViewDirection.z ) * -1.0925484 ) ) ) ) + ( sh2_4 * vec3<f32>( ( ( shXX - shYY ) * 0.5462742 ) ) ) ) );
	shXY = ( sphericalHarmonicsViewDirection.x * sphericalHarmonicsViewDirection.y );
	nodeVar7 = NodeBuffer_2199.value[ ( ( instanceIndex * 6u ) + 0u ) ];
	nodeVar8 = NodeBuffer_2199.value[ ( ( instanceIndex * 6u ) + 1u ) ];
	nodeVar9 = NodeBuffer_2199.value[ ( ( instanceIndex * 6u ) + 2u ) ];
	nodeVar10 = NodeBuffer_2199.value[ ( ( instanceIndex * 6u ) + 3u ) ];
	nodeVar11 = NodeBuffer_2199.value[ ( ( instanceIndex * 6u ) + 4u ) ];
	nodeVar12 = NodeBuffer_2199.value[ ( ( instanceIndex * 6u ) + 5u ) ];
	sh3_0 = vec3<f32>( ( ( f32( ( ( nodeVar7 >> 0u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar7 >> 8u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar7 >> 16u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sh3_1 = vec3<f32>( ( ( f32( ( ( nodeVar7 >> 24u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar8 >> 0u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar8 >> 8u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sh3_2 = vec3<f32>( ( ( f32( ( ( nodeVar8 >> 16u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar8 >> 24u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar9 >> 0u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sh3_3 = vec3<f32>( ( ( f32( ( ( nodeVar9 >> 8u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar9 >> 16u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar9 >> 24u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sh3_4 = vec3<f32>( ( ( f32( ( ( nodeVar10 >> 0u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar10 >> 8u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar10 >> 16u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sh3_5 = vec3<f32>( ( ( f32( ( ( nodeVar10 >> 24u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar11 >> 0u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar11 >> 8u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sh3_6 = vec3<f32>( ( ( f32( ( ( nodeVar11 >> 16u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar11 >> 24u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar12 >> 0u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sphericalHarmonicsContribution = ( sphericalHarmonicsContribution + ( ( ( ( ( ( ( sh3_0 * vec3<f32>( ( ( sphericalHarmonicsViewDirection.y * ( ( shXX * 3.0 ) - shYY ) ) * -0.5900436 ) ) ) + ( sh3_1 * vec3<f32>( ( ( shXY * sphericalHarmonicsViewDirection.z ) * 2.8906114 ) ) ) ) + ( sh3_2 * vec3<f32>( ( ( sphericalHarmonicsViewDirection.y * ( ( ( shZZ * 4.0 ) - shXX ) - shYY ) ) * -0.4570458 ) ) ) ) + ( sh3_3 * vec3<f32>( ( ( sphericalHarmonicsViewDirection.z * ( ( ( shZZ * 2.0 ) - ( shXX * 3.0 ) ) - ( shYY * 3.0 ) ) ) * 0.3731763 ) ) ) ) + ( sh3_4 * vec3<f32>( ( ( sphericalHarmonicsViewDirection.x * ( ( ( shZZ * 4.0 ) - shXX ) - shYY ) ) * -0.4570458 ) ) ) ) + ( sh3_5 * vec3<f32>( ( ( sphericalHarmonicsViewDirection.z * ( shXX - shYY ) ) * 1.4453057 ) ) ) ) + ( sh3_6 * vec3<f32>( ( ( sphericalHarmonicsViewDirection.x * ( shXX - ( shYY * 3.0 ) ) ) * -0.5900436 ) ) ) ) );
	NodeBuffer_2347.value[ instanceIndex ] = vec4<f32>( sphericalHarmonicsContribution, 0.0 );

	

}
