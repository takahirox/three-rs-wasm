// Three.js r187 - Node System

// directives

// system
var<private> instanceIndex : u32;

// locals


// structs


// uniforms

struct nodeUniform0Struct {
	value : array< vec4<f32> >
};
@binding( 0 ) @group( 0 )
var<storage, read> nodeUniform0 : nodeUniform0Struct;

struct nodeUniform3Struct {
	value : array< u32 >
};
@binding( 2 ) @group( 0 )
var<storage, read_write> nodeUniform3 : nodeUniform3Struct;

struct nodeUniform4Struct {
	value : array< atomic<u32> >
};
@binding( 3 ) @group( 0 )
var<storage, read_write> nodeUniform4 : nodeUniform4Struct;

struct objectStruct {
	nodeUniform1 : mat4x4<f32>,
	nodeUniform2 : vec2<f32>,
	nodeUniform5 : u32
};
@binding( 1 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> center : vec3<f32>;
var<private> viewCenter : vec3<f32>;
var<private> depth : f32;
var<private> range : f32;
var<private> normalized : f32;
var<private> depthBin : u32;
var<private> bin : u32;

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


	// flow -> CountingSortHistogram
	if ( instanceIndex >= object.nodeUniform5 ) { return; }

	center = nodeUniform0.value[ instanceIndex ].xyz;
	viewCenter = ( object.nodeUniform1 * vec4<f32>( center, 1.0 ) ).xyz;
	depth = ( - viewCenter.z );
	range = max( ( object.nodeUniform2.y - object.nodeUniform2.x ), 0.0001 );
	normalized = clamp( ( ( depth - object.nodeUniform2.x ) / range ), 0.0, 1.0 );
	depthBin = u32( ( normalized * 4095.0 ) );
	bin = ( 4095u - depthBin );
	nodeUniform3.value[ instanceIndex ] = bin;
	atomicAdd( &nodeUniform4.value[ bin ], 1u );

	

}
