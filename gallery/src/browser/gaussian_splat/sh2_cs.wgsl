// Three.js r186 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct NodeBuffer_991Struct {
	value : array< vec4<f32> >
};
@binding( 0 ) @group( 0 )
var<storage, read> NodeBuffer_991 : NodeBuffer_991Struct;

struct NodeBuffer_995Struct {
	value : array< u32 >
};
@binding( 2 ) @group( 0 )
var<storage, read> NodeBuffer_995 : NodeBuffer_995Struct;

struct NodeBuffer_996Struct {
	value : array< u32 >
};
@binding( 3 ) @group( 0 )
var<storage, read> NodeBuffer_996 : NodeBuffer_996Struct;

struct NodeBuffer_1145Struct {
	value : array< vec4<f32> >
};
@binding( 4 ) @group( 0 )
var<storage, read_write> NodeBuffer_1145 : NodeBuffer_1145Struct;

struct objectStruct {
	nodeUniform1 : vec3<f32>,
	nodeUniform5 : u32
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
	if ( instanceIndex >= object.nodeUniform5 ) { return; }

	center = NodeBuffer_991.value[ instanceIndex ].xyz;
	sphericalHarmonicsContribution = vec3<f32>( 0.0, 0.0, 0.0 );
	sphericalHarmonicsViewDirection = normalize( ( center - object.nodeUniform1 ) );
	nodeVar0 = NodeBuffer_995.value[ ( ( instanceIndex * 3u ) + 0u ) ];
	nodeVar1 = NodeBuffer_995.value[ ( ( instanceIndex * 3u ) + 1u ) ];
	nodeVar2 = NodeBuffer_995.value[ ( ( instanceIndex * 3u ) + 2u ) ];
	sh1_0 = vec3<f32>( ( ( f32( ( ( nodeVar0 >> 0u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar0 >> 8u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar0 >> 16u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sh1_1 = vec3<f32>( ( ( f32( ( ( nodeVar0 >> 24u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar1 >> 0u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar1 >> 8u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sh1_2 = vec3<f32>( ( ( f32( ( ( nodeVar1 >> 16u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar1 >> 24u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar2 >> 0u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sphericalHarmonicsContribution = ( sphericalHarmonicsContribution + ( ( ( sh1_0 * vec3<f32>( ( sphericalHarmonicsViewDirection.y * -0.4886025 ) ) ) + ( sh1_1 * vec3<f32>( ( sphericalHarmonicsViewDirection.z * 0.4886025 ) ) ) ) + ( sh1_2 * vec3<f32>( ( sphericalHarmonicsViewDirection.x * -0.4886025 ) ) ) ) );
	shXX = ( sphericalHarmonicsViewDirection.x * sphericalHarmonicsViewDirection.x );
	shYY = ( sphericalHarmonicsViewDirection.y * sphericalHarmonicsViewDirection.y );
	shZZ = ( sphericalHarmonicsViewDirection.z * sphericalHarmonicsViewDirection.z );
	nodeVar3 = NodeBuffer_996.value[ ( ( instanceIndex * 4u ) + 0u ) ];
	nodeVar4 = NodeBuffer_996.value[ ( ( instanceIndex * 4u ) + 1u ) ];
	nodeVar5 = NodeBuffer_996.value[ ( ( instanceIndex * 4u ) + 2u ) ];
	nodeVar6 = NodeBuffer_996.value[ ( ( instanceIndex * 4u ) + 3u ) ];
	sh2_0 = vec3<f32>( ( ( f32( ( ( nodeVar3 >> 0u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar3 >> 8u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar3 >> 16u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sh2_1 = vec3<f32>( ( ( f32( ( ( nodeVar3 >> 24u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar4 >> 0u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar4 >> 8u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sh2_2 = vec3<f32>( ( ( f32( ( ( nodeVar4 >> 16u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar4 >> 24u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar5 >> 0u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sh2_3 = vec3<f32>( ( ( f32( ( ( nodeVar5 >> 8u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar5 >> 16u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar5 >> 24u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sh2_4 = vec3<f32>( ( ( f32( ( ( nodeVar6 >> 0u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar6 >> 8u ) & 255u ) ) - 128.0 ) / 128.0 ), ( ( f32( ( ( nodeVar6 >> 16u ) & 255u ) ) - 128.0 ) / 128.0 ) );
	sphericalHarmonicsContribution = ( sphericalHarmonicsContribution + ( ( ( ( ( sh2_0 * vec3<f32>( ( ( sphericalHarmonicsViewDirection.x * sphericalHarmonicsViewDirection.y ) * 1.0925484 ) ) ) + ( sh2_1 * vec3<f32>( ( ( sphericalHarmonicsViewDirection.y * sphericalHarmonicsViewDirection.z ) * -1.0925484 ) ) ) ) + ( sh2_2 * vec3<f32>( ( ( ( ( shZZ * 2.0 ) - shXX ) - shYY ) * 0.3153915 ) ) ) ) + ( sh2_3 * vec3<f32>( ( ( sphericalHarmonicsViewDirection.x * sphericalHarmonicsViewDirection.z ) * -1.0925484 ) ) ) ) + ( sh2_4 * vec3<f32>( ( ( shXX - shYY ) * 0.5462742 ) ) ) ) );
	NodeBuffer_1145.value[ instanceIndex ] = vec4<f32>( sphericalHarmonicsContribution, 0.0 );

	

}
