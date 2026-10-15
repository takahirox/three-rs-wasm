// Three.js r186 - Node System

// directives


// structs


// uniforms

struct NodeBuffer_1003Struct {
	value : array< u32 >
};
@binding( 1 ) @group( 1 )
var<storage, read> NodeBuffer_1003 : NodeBuffer_1003Struct;

struct NodeBuffer_991Struct {
	value : array< vec4<f32> >
};
@binding( 2 ) @group( 1 )
var<storage, read> NodeBuffer_991 : NodeBuffer_991Struct;

struct NodeBuffer_992Struct {
	value : array< vec4<f32> >
};
@binding( 3 ) @group( 1 )
var<storage, read> NodeBuffer_992 : NodeBuffer_992Struct;

struct NodeBuffer_993Struct {
	value : array< vec4<f32> >
};
@binding( 4 ) @group( 1 )
var<storage, read> NodeBuffer_993 : NodeBuffer_993Struct;

struct NodeBuffer_994Struct {
	value : array< u32 >
};
@binding( 5 ) @group( 1 )
var<storage, read> NodeBuffer_994 : NodeBuffer_994Struct;

struct NodeBuffer_1144Struct {
	value : array< vec4<f32> >
};
@binding( 6 ) @group( 1 )
var<storage, read> NodeBuffer_1144 : NodeBuffer_1144Struct;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform7 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	nodeUniform9 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// varyings

struct VaryingsStruct {
	@location( 0 ) vSplatUv : vec2<f32>,
	@location( 1 ) vSplatColor : vec4<f32>,
	@builtin( position ) builtinClipSpace : vec4<f32>
};
var<private> varyings : VaryingsStruct;

// vars
var<private> splatIndex : u32;
var<private> center : vec3<f32>;
var<private> covA : vec4<f32>;
var<private> covB : vec4<f32>;
var<private> splatColor : vec4<f32>;
var<private> splatRgb : vec3<f32>;
var<private> highpModelViewMatrix : mat4x4<f32>;
var<private> viewCenter4 : vec4<f32>;
var<private> viewCenter : vec3<f32>;
var<private> centerClip : vec4<f32>;
var<private> r0 : vec3<f32>;
var<private> r1 : vec3<f32>;
var<private> r2 : vec3<f32>;
var<private> cov0 : vec3<f32>;
var<private> cov1 : vec3<f32>;
var<private> cov2 : vec3<f32>;
var<private> vc0 : vec3<f32>;
var<private> vc1 : vec3<f32>;
var<private> vc2 : vec3<f32>;
var<private> c00 : f32;
var<private> c01 : f32;
var<private> c02 : f32;
var<private> c11 : f32;
var<private> c12 : f32;
var<private> c22 : f32;
var<private> z : f32;
var<private> invZ : f32;
var<private> invZ2 : f32;
var<private> focal : vec2<f32>;
var<private> j00 : f32;
var<private> j11 : f32;
var<private> j02 : f32;
var<private> j12 : f32;
var<private> cov2dABase : f32;
var<private> cov2dB : f32;
var<private> cov2dCBase : f32;
var<private> cov2dA : f32;
var<private> cov2dC : f32;
var<private> detBase : f32;
var<private> det : f32;
var<private> alphaScale : f32;
var<private> halfTrace : f32;
var<private> nodeVar1 : f32;
var<private> radius : f32;
var<private> lambda1 : f32;
var<private> lambda2 : f32;
var<private> axis1 : vec2<f32>;
var<private> angle : f32;
var<private> axis2 : vec2<f32>;
var<private> scale1 : f32;
var<private> scale2 : f32;
var<private> offsetPixels : vec2<f32>;
var<private> offsetNdc : vec2<f32>;
var<private> clip : vec4<f32>;
var<private> clipLimit : f32;

// codes


@vertex
fn main( @builtin( instance_index ) instanceIndex : u32,
	@location( 0 ) position : vec3<f32> ) -> VaryingsStruct {

	// flow
	// code

	splatIndex = NodeBuffer_1003.value[ instanceIndex ];
	center = NodeBuffer_991.value[ splatIndex ].xyz;
	covA = NodeBuffer_992.value[ splatIndex ];
	covB = NodeBuffer_993.value[ splatIndex ];
	splatColor = unpack4x8unorm(NodeBuffer_994.value[ splatIndex ]);
	splatRgb = splatColor.xyz;
	splatRgb = ( splatRgb + NodeBuffer_1144.value[ splatIndex ].xyz );
	varyings.vSplatUv = position.xy;
	highpModelViewMatrix = object.nodeUniform7;
	viewCenter4 = ( highpModelViewMatrix * vec4<f32>( center, 1.0 ) );
	viewCenter = viewCenter4.xyz;
	centerClip = ( render.cameraProjectionMatrix * viewCenter4 );
	r0 = vec3<f32>( highpModelViewMatrix[ 0u ].x, highpModelViewMatrix[ 1u ].x, highpModelViewMatrix[ 2u ].x );
	r1 = vec3<f32>( highpModelViewMatrix[ 0u ].y, highpModelViewMatrix[ 1u ].y, highpModelViewMatrix[ 2u ].y );
	r2 = vec3<f32>( highpModelViewMatrix[ 0u ].z, highpModelViewMatrix[ 1u ].z, highpModelViewMatrix[ 2u ].z );
	cov0 = vec3<f32>( covA.x, covA.y, covA.z );
	cov1 = vec3<f32>( covA.y, covA.w, covB.x );
	cov2 = vec3<f32>( covA.z, covB.x, covB.y );
	vc0 = vec3<f32>( dot( r0, cov0 ), dot( r0, cov1 ), dot( r0, cov2 ) );
	vc1 = vec3<f32>( dot( r1, cov0 ), dot( r1, cov1 ), dot( r1, cov2 ) );
	vc2 = vec3<f32>( dot( r2, cov0 ), dot( r2, cov1 ), dot( r2, cov2 ) );
	c00 = dot( vc0, r0 );
	c01 = dot( vc0, r1 );
	c02 = dot( vc0, r2 );
	c11 = dot( vc1, r1 );
	c12 = dot( vc1, r2 );
	c22 = dot( vc2, r2 );
	z = min( viewCenter.z, -0.01 );
	invZ = ( 1.0 / z );
	invZ2 = ( invZ * invZ );
	let cameraViewport = vec4<f32>( 0.0, 0.0, render.nodeUniform9.x, render.nodeUniform9.y );
	focal = ( ( cameraViewport.zw * vec2<f32>( 0.5 ) ) * vec2<f32>( render.cameraProjectionMatrix[ 0u ].x, render.cameraProjectionMatrix[ 1u ].y ) );
	j00 = ( ( - focal.x ) * invZ );
	j11 = ( ( - focal.y ) * invZ );
	j02 = ( ( focal.x * viewCenter.x ) * invZ2 );
	j12 = ( ( focal.y * viewCenter.y ) * invZ2 );
	cov2dABase = ( ( ( ( j00 * j00 ) * c00 ) + ( ( ( j00 * j02 ) * c02 ) * 2.0 ) ) + ( ( j02 * j02 ) * c22 ) );
	cov2dB = ( ( ( ( ( j00 * j11 ) * c01 ) + ( ( j00 * j12 ) * c02 ) ) + ( ( j02 * j11 ) * c12 ) ) + ( ( j02 * j12 ) * c22 ) );
	cov2dCBase = ( ( ( ( j11 * j11 ) * c11 ) + ( ( ( j11 * j12 ) * c12 ) * 2.0 ) ) + ( ( j12 * j12 ) * c22 ) );
	cov2dA = ( cov2dABase + 0.3 );
	cov2dC = ( cov2dCBase + 0.3 );
	detBase = ( ( cov2dABase * cov2dCBase ) - ( cov2dB * cov2dB ) );
	det = ( ( cov2dA * cov2dC ) - ( cov2dB * cov2dB ) );
	alphaScale = sqrt( max( ( detBase / max( det, 0.000001 ) ), 0.0 ) );
	varyings.vSplatColor = vec4<f32>( clamp( splatRgb, vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) ), ( splatColor.w * alphaScale ) );
	halfTrace = ( ( cov2dA + cov2dC ) * 0.5 );
	nodeVar1 = ( ( cov2dA - cov2dC ) * 0.5 );
	radius = sqrt( max( ( ( nodeVar1 * nodeVar1 ) + ( cov2dB * cov2dB ) ), 1e-7 ) );
	lambda1 = max( ( halfTrace + radius ), 1e-7 );
	lambda2 = max( ( halfTrace - radius ), 1e-7 );
	axis1 = vec2<f32>( 1.0, 0.0 );

	if ( ( radius > 0.00001 ) ) {

		angle = ( atan2( ( cov2dB * 2.0 ), ( cov2dA - cov2dC ) ) * 0.5 );
		axis1 = vec2<f32>( cos( angle ), sin( angle ) );
		

	}

	axis2 = vec2<f32>( ( - axis1.y ), axis1.x );
	scale1 = min( sqrt( lambda1 ), 1024.0 );
	scale2 = min( sqrt( lambda2 ), 1024.0 );
	offsetPixels = ( ( ( axis1 * vec2<f32>( position.x ) ) * vec2<f32>( scale1 ) ) + ( ( axis2 * vec2<f32>( position.y ) ) * vec2<f32>( scale2 ) ) );
	offsetNdc = ( ( offsetPixels * vec2<f32>( 2.0 ) ) / cameraViewport.zw );
	clip = ( centerClip + vec4<f32>( ( offsetNdc * vec2<f32>( centerClip.w ) ), 0.0, 0.0 ) );
	clipLimit = ( centerClip.w * 1.4 );

	if ( ( ( ( ( ( ( ( viewCenter.z >= -0.01 ) || ( centerClip.z < ( - centerClip.w ) ) ) || ( centerClip.z > centerClip.w ) ) || ( centerClip.x < ( - clipLimit ) ) ) || ( centerClip.x > clipLimit ) ) || ( centerClip.y < ( - clipLimit ) ) ) || ( centerClip.y > clipLimit ) ) ) {

		clip = vec4<f32>( 2.0, 2.0, 2.0, 1.0 );
		

	}


	// result

	varyings.builtinClipSpace = clip;

	return varyings;

}
