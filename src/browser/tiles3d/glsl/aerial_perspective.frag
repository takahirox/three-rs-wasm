#version 300 es
#define varying in
layout(location = 0) out highp vec4 pc_fragColor;
#define gl_FragColor pc_fragColor
#define gl_FragDepthEXT gl_FragDepth
#define texture2D texture
#define textureCube texture
#define texture2DProj textureProj
#define texture2DLodEXT textureLod
#define texture2DProjLodEXT textureProjLod
#define textureCubeLodEXT textureLod
#define texture2DGradEXT textureGrad
#define texture2DProjGradEXT textureProjGrad
#define textureCubeGradEXT textureGrad
precision highp float;
	precision highp int;
	precision highp sampler2D;
	precision highp samplerCube;
	precision highp sampler3D;
	precision highp sampler2DArray;
	precision highp sampler2DShadow;
	precision highp samplerCubeShadow;
	precision highp sampler2DArrayShadow;
	precision highp isampler2D;
	precision highp isampler3D;
	precision highp isamplerCube;
	precision highp isampler2DArray;
	precision highp usampler2D;
	precision highp usampler3D;
	precision highp usamplerCube;
	precision highp usampler2DArray;
	
#define HIGH_PRECISION
#define SHADER_TYPE ShaderMaterial
#define SHADER_NAME EffectMaterial
#define THREE_REVISION 186
#define DEPTH_PACKING 3200
#define ENCODE_OUTPUT 1
#define PERSPECTIVE_CAMERA 1
#define e0SKIP_RENDERING 1
#define e1TRANSMITTANCE_TEXTURE_WIDTH 256
#define e1TRANSMITTANCE_TEXTURE_HEIGHT 64
#define e1SCATTERING_TEXTURE_R_SIZE 32
#define e1SCATTERING_TEXTURE_MU_SIZE 128
#define e1SCATTERING_TEXTURE_MU_S_SIZE 32
#define e1SCATTERING_TEXTURE_NU_SIZE 8
#define e1IRRADIANCE_TEXTURE_WIDTH 64
#define e1IRRADIANCE_TEXTURE_HEIGHT 16
#define e1METER_TO_LENGTH_UNIT 0.0010000
#define e1SHADOW_SAMPLE_COUNT 8
#define e1COMBINED_SCATTERING_TEXTURES 1
#define e1CORRECT_GEOMETRIC_ERROR 1
#define e1TRANSMITTANCE 1
#define e1INSCATTER 1
#define e1SUN 1
#define e1MOON 1
#define e1GROUND 1
#define e1SKY 1
#define e1SUN_LIGHT 1
#define e1SKY_LIGHT 1
#define e1HAS_NORMALS 1
#define UV vUv
#define FRAMEBUFFER_PRECISION_HIGH 1
#define e1HAS_OVERLAY 1
#define e1HAS_HIGHER_ORDER_SCATTERING_TEXTURE 1
#define e1HAS_SHADOW 1
#define e1SHADOW_CASCADE_COUNT 2
#define e1HAS_SHADOW_LENGTH 1
uniform mat4 viewMatrix;
uniform vec3 cameraPosition;
uniform bool isOrthographic;


vec4 LinearTransferOETF( in vec4 value ) {
	return value;
}

vec4 sRGBTransferEOTF( in vec4 value ) {
	return vec4( mix( pow( value.rgb * 0.9478672986 + vec3( 0.0521327014 ), vec3( 2.4 ) ), value.rgb * 0.0773993808, vec3( lessThanEqual( value.rgb, vec3( 0.04045 ) ) ) ), value.a );
}

vec4 sRGBTransferOETF( in vec4 value ) {
	return vec4( mix( pow( value.rgb, vec3( 0.41666 ) ) * 1.055 - vec3( 0.055 ), value.rgb * 12.92, vec3( lessThanEqual( value.rgb, vec3( 0.0031308 ) ) ) ), value.a );
}


vec4 linearToOutputTexel( vec4 value ) {
	return LinearTransferOETF( vec4( value.rgb * mat3( 1.0000,-0.0000,-0.0000,-0.0000,1.0000,0.0000,0.0000,0.0000,1.0000 ), value.a ) );
}
float luminance( const in vec3 rgb ) {
	const vec3 weights = vec3( 0.2126, 0.7152, 0.0722 );
	return dot( weights, rgb );
}
#define DEPTH_PACKING 3200


#define PI 3.141592653589793
#define PI2 6.283185307179586
#define PI_HALF 1.5707963267948966
#define RECIPROCAL_PI 0.3183098861837907
#define RECIPROCAL_PI2 0.15915494309189535
#define EPSILON 1e-6

#ifndef saturate
// <tonemapping_pars_fragment> may have defined saturate() already
#define saturate( a ) clamp( a, 0.0, 1.0 )
#endif
#define whiteComplement( a ) ( 1.0 - saturate( a ) )

float pow2( const in float x ) { return x*x; }
vec3 pow2( const in vec3 x ) { return x*x; }
float pow3( const in float x ) { return x*x*x; }
float pow4( const in float x ) { float x2 = x*x; return x2*x2; }
float max3( const in vec3 v ) { return max( max( v.x, v.y ), v.z ); }
float average( const in vec3 v ) { return dot( v, vec3( 0.3333333 ) ); }

// expects values in the range of [0,1]x[0,1], returns values in the [0,1] range.
// do not collapse into a single function per: http://byteblacksmith.com/improvements-to-the-canonical-one-liner-glsl-rand-for-opengl-es-2-0/
highp float rand( const in vec2 uv ) {

	const highp float a = 12.9898, b = 78.233, c = 43758.5453;
	highp float dt = dot( uv.xy, vec2( a,b ) ), sn = mod( dt, PI );

	return fract( sin( sn ) * c );

}

#ifdef HIGH_PRECISION
	float precisionSafeLength( vec3 v ) { return length( v ); }
#else
	float precisionSafeLength( vec3 v ) {
		float maxComponent = max3( abs( v ) );
		return length( v / maxComponent ) * maxComponent;
	}
#endif

struct IncidentLight {
	vec3 color;
	vec3 direction;
	bool visible;
};

struct ReflectedLight {
	vec3 directDiffuse;
	vec3 directSpecular;
	vec3 indirectDiffuse;
	vec3 indirectSpecular;
};

#ifdef USE_ALPHAHASH

	varying vec3 vPosition;

#endif

vec3 transformDirection( in vec3 dir, in mat4 matrix ) {

	return normalize( ( matrix * vec4( dir, 0.0 ) ).xyz );

}

#define inverseTransformDirection transformDirectionByInverseViewMatrix // @deprecated r185

vec3 transformNormalByInverseViewMatrix( in vec3 normal, in mat4 viewMatrix ) {

	// upper-left 3x3 of view matrix is assumed to be orthogonal

	return normalize( ( vec4( normal, 0.0 ) * viewMatrix ).xyz );

}

vec3 transformDirectionByInverseViewMatrix( in vec3 dir, in mat4 viewMatrix ) {

	// upper-left 3x3 of view matrix is assumed to be orthogonal

	return normalize( ( vec4( dir, 0.0 ) * viewMatrix ).xyz );

}

bool isPerspectiveMatrix( mat4 m ) {

	return m[ 2 ][ 3 ] == - 1.0;

}

vec2 equirectUv( in vec3 dir ) {

	// dir is assumed to be unit length

	float u = atan( dir.z, dir.x ) * RECIPROCAL_PI2 + 0.5;

	float v = asin( clamp( dir.y, - 1.0, 1.0 ) ) * RECIPROCAL_PI + 0.5;

	return vec2( u, v );

}

vec3 BRDF_Lambert( const in vec3 diffuseColor ) {

	return RECIPROCAL_PI * diffuseColor;

} // validated

vec3 F_Schlick( const in vec3 f0, const in float f90, const in float dotVH ) {

	// Original approximation by Christophe Schlick '94
	// float fresnel = pow( 1.0 - dotVH, 5.0 );

	// Optimized variant (presented by Epic at SIGGRAPH '13)
	// https://cdn2.unrealengine.com/Resources/files/2013SiggraphPresentationsNotes-26915738.pdf
	float fresnel = exp2( ( - 5.55473 * dotVH - 6.98316 ) * dotVH );

	return f0 * ( 1.0 - fresnel ) + ( f90 * fresnel );

} // validated

float F_Schlick( const in float f0, const in float f90, const in float dotVH ) {

	// Original approximation by Christophe Schlick '94
	// float fresnel = pow( 1.0 - dotVH, 5.0 );

	// Optimized variant (presented by Epic at SIGGRAPH '13)
	// https://cdn2.unrealengine.com/Resources/files/2013SiggraphPresentationsNotes-26915738.pdf
	float fresnel = exp2( ( - 5.55473 * dotVH - 6.98316 ) * dotVH );

	return f0 * ( 1.0 - fresnel ) + ( f90 * fresnel );

} // validated


vec3 packNormalToRGB( const in vec3 normal ) {
	return normalize( normal ) * 0.5 + 0.5;
}

vec3 unpackRGBToNormal( const in vec3 rgb ) {
	return 2.0 * rgb.xyz - 1.0;
}

const float PackUpscale = 256. / 255.; // fraction -> 0..1 (including 1)
const float UnpackDownscale = 255. / 256.; // 0..1 -> fraction (excluding 1)
const float ShiftRight8 = 1. / 256.;
const float Inv255 = 1. / 255.;

const vec4 PackFactors = vec4( 1.0, 256.0, 256.0 * 256.0, 256.0 * 256.0 * 256.0 );

const vec2 UnpackFactors2 = vec2( UnpackDownscale, 1.0 / PackFactors.g );
const vec3 UnpackFactors3 = vec3( UnpackDownscale / PackFactors.rg, 1.0 / PackFactors.b );
const vec4 UnpackFactors4 = vec4( UnpackDownscale / PackFactors.rgb, 1.0 / PackFactors.a );

vec4 packDepthToRGBA( const in float v ) {
	if( v <= 0.0 )
		return vec4( 0., 0., 0., 0. );
	if( v >= 1.0 )
		return vec4( 1., 1., 1., 1. );
	float vuf;
	float af = modf( v * PackFactors.a, vuf );
	float bf = modf( vuf * ShiftRight8, vuf );
	float gf = modf( vuf * ShiftRight8, vuf );
	return vec4( vuf * Inv255, gf * PackUpscale, bf * PackUpscale, af );
}

vec3 packDepthToRGB( const in float v ) {
	if( v <= 0.0 )
		return vec3( 0., 0., 0. );
	if( v >= 1.0 )
		return vec3( 1., 1., 1. );
	float vuf;
	float bf = modf( v * PackFactors.b, vuf );
	float gf = modf( vuf * ShiftRight8, vuf );
	// the 0.9999 tweak is unimportant, very tiny empirical improvement
	// return vec3( vuf * Inv255, gf * PackUpscale, bf * 0.9999 );
	return vec3( vuf * Inv255, gf * PackUpscale, bf );
}

vec2 packDepthToRG( const in float v ) {
	if( v <= 0.0 )
		return vec2( 0., 0. );
	if( v >= 1.0 )
		return vec2( 1., 1. );
	float vuf;
	float gf = modf( v * 256., vuf );
	return vec2( vuf * Inv255, gf );
}

float unpackRGBAToDepth( const in vec4 v ) {
	return dot( v, UnpackFactors4 );
}

float unpackRGBToDepth( const in vec3 v ) {
	return dot( v, UnpackFactors3 );
}

float unpackRGToDepth( const in vec2 v ) {
	return v.r * UnpackFactors2.r + v.g * UnpackFactors2.g;
}

vec4 pack2HalfToRGBA( const in vec2 v ) {
	vec4 r = vec4( v.x, fract( v.x * 255.0 ), v.y, fract( v.y * 255.0 ) );
	return vec4( r.x - r.y / 255.0, r.y, r.z - r.w / 255.0, r.w );
}

vec2 unpackRGBATo2Half( const in vec4 v ) {
	return vec2( v.x + ( v.y / 255.0 ), v.z + ( v.w / 255.0 ) );
}

// NOTE: viewZ, the z-coordinate in camera space, is negative for points in front of the camera

float viewZToOrthographicDepth( const in float viewZ, const in float near, const in float far ) {
	// -near maps to 0; -far maps to 1
	return ( viewZ + near ) / ( near - far );
}

float orthographicDepthToViewZ( const in float depth, const in float near, const in float far ) {

	#ifdef USE_REVERSED_DEPTH_BUFFER
	
		return depth * ( far - near ) - far;

	#else

		return depth * ( near - far ) - near;

	#endif
}

// NOTE: https://twitter.com/gonnavis/status/1377183786949959682

float viewZToPerspectiveDepth( const in float viewZ, const in float near, const in float far ) {
	// -near maps to 0; -far maps to 1
	return ( ( near + viewZ ) * far ) / ( ( far - near ) * viewZ );
}

float perspectiveDepthToViewZ( const in float depth, const in float near, const in float far ) {
	
	#ifdef USE_REVERSED_DEPTH_BUFFER

		return ( near * far ) / ( ( near - far ) * depth - near );

	#else

		return ( near * far ) / ( ( far - near ) * depth - far );

	#endif
}


#ifdef DITHERING

	// based on https://www.shadertoy.com/view/MslGR8
	vec3 dithering( vec3 color ) {
		//Calculate grid position
		float grid_position = rand( gl_FragCoord.xy );

		//Shift the individual colors differently, thus making it even harder to see the dithering pattern
		vec3 dither_shift_RGB = vec3( 0.25 / 255.0, -0.25 / 255.0, 0.25 / 255.0 );

		//modify shift according to grid position.
		dither_shift_RGB = mix( 2.0 * dither_shift_RGB, -2.0 * dither_shift_RGB, grid_position );

		//shift the color by dither_shift
		return color + dither_shift_RGB;
	}

#endif

#define packFloatToRGBA(v) packDepthToRGBA(v)
#define unpackRGBAToFloat(v) unpackRGBAToDepth(v)
#ifdef FRAMEBUFFER_PRECISION_HIGH
uniform mediump sampler2D inputBuffer;
#else
uniform lowp sampler2D inputBuffer;
#endif
#if DEPTH_PACKING == 3201
uniform lowp sampler2D depthBuffer;
#elif defined(GL_FRAGMENT_PRECISION_HIGH)
uniform highp sampler2D depthBuffer;
#else
uniform mediump sampler2D depthBuffer;
#endif
uniform vec2 resolution;uniform vec2 texelSize;uniform float cameraNear;uniform float cameraFar;uniform float aspect;uniform float time;varying vec2 vUv;vec4 sRGBToLinear(const in vec4 value){return vec4(mix(pow(value.rgb*0.9478672986+vec3(0.0521327014),vec3(2.4)),value.rgb*0.0773993808,vec3(lessThanEqual(value.rgb,vec3(0.04045)))),value.a);}float readDepth(const in vec2 uv){
#if DEPTH_PACKING == 3201
float depth=unpackRGBAToDepth(texture2D(depthBuffer,uv));
#else
float depth=texture2D(depthBuffer,uv).r;
#endif
#if defined(USE_LOGARITHMIC_DEPTH_BUFFER) || defined(LOG_DEPTH)
float d=pow(2.0,depth*log2(cameraFar+1.0))-1.0;float a=cameraFar/(cameraFar-cameraNear);float b=cameraFar*cameraNear/(cameraNear-cameraFar);depth=a+b/d;
#elif defined(USE_REVERSED_DEPTH_BUFFER)
depth=1.0-depth;
#endif
return depth;}float getViewZ(const in float depth){
#ifdef PERSPECTIVE_CAMERA
return perspectiveDepthToViewZ(depth,cameraNear,cameraFar);
#else
return orthographicDepthToViewZ(depth,cameraNear,cameraFar);
#endif
}vec3 RGBToHCV(const in vec3 RGB){vec4 P=mix(vec4(RGB.bg,-1.0,2.0/3.0),vec4(RGB.gb,0.0,-1.0/3.0),step(RGB.b,RGB.g));vec4 Q=mix(vec4(P.xyw,RGB.r),vec4(RGB.r,P.yzx),step(P.x,RGB.r));float C=Q.x-min(Q.w,Q.y);float H=abs((Q.w-Q.y)/(6.0*C+EPSILON)+Q.z);return vec3(H,C,Q.x);}vec3 RGBToHSL(const in vec3 RGB){vec3 HCV=RGBToHCV(RGB);float L=HCV.z-HCV.y*0.5;float S=HCV.y/(1.0-abs(L*2.0-1.0)+EPSILON);return vec3(HCV.x,S,L);}vec3 HueToRGB(const in float H){float R=abs(H*6.0-3.0)-1.0;float G=2.0-abs(H*6.0-2.0);float B=2.0-abs(H*6.0-4.0);return clamp(vec3(R,G,B),0.0,1.0);}vec3 HSLToRGB(const in vec3 HSL){vec3 RGB=HueToRGB(HSL.x);float C=(1.0-abs(2.0*HSL.z-1.0))*HSL.y;return(RGB-0.5)*C+HSL.z;}uniform float e0BlendOpacity;

uniform sampler2D e0CloudsBuffer;

void e0MainImage(const vec4 inputColor, const vec2 uv, out vec4 outputColor) {
  #ifdef e0SKIP_RENDERING
  outputColor = inputColor;
  #else // e0SKIP_RENDERING
  vec4 clouds = texture(e0CloudsBuffer, uv);
  outputColor.rgb = inputColor.rgb * (1.0 - clouds.a) + clouds.rgb;
  outputColor.a = inputColor.a * (1.0 - clouds.a) + clouds.a;
  #endif // e0SKIP_RENDERING
}

uniform float e1BlendOpacity;

precision highp sampler2DArray;

// cSpell:words logdepthbuf

#ifdef DEPTH_PACKING
float e1ReadDepthValue(const sampler2D depthBuffer, const vec2 uv) {
  #if DEPTH_PACKING == 3201
  return unpackRGBAToDepth(texture(depthBuffer, uv));
  #else // DEPTH_PACKING == 3201
  return texture(depthBuffer, uv).r;
  #endif // DEPTH_PACKING == 3201
}
#endif // DEPTH_PACKING

float e1ReverseLogDepth(const float depth, const float near, const float far) {
  #if defined(USE_LOGDEPTHBUF) || defined(USE_LOGARITHMIC_DEPTH_BUFFER)
  float d = pow(2.0, depth * log2(far + 1.0)) - 1.0;
  float a = far / (far - near);
  float b = far * near / (near - far);
  return a + b / d;
  #else // defined(USE_LOGARITHMIC_DEPTH_BUFFER) || defined(USE_LOGARITHMIC_DEPTH_BUFFER)
  return depth;
  #endif // defined(USE_LOGARITHMIC_DEPTH_BUFFER) || defined(USE_LOGARITHMIC_DEPTH_BUFFER)
}

float e1LinearizeDepth(const float depth, const float near, const float far) {
  float ndc = depth * 2.0 - 1.0;
  return 2.0 * near * far / (far + near - ndc * (far - near));
}

#if !defined(saturate)
#define saturate(a) clamp(a, 0.0, 1.0)
#endif // !defined(saturate)

float e1Remap(const float x, const float min1, const float max1, const float min2, const float max2) {
  return min2 + (x - min1) / (max1 - min1) * (max2 - min2);
}

vec2 e1Remap(const vec2 x, const vec2 min1, const vec2 max1, const vec2 min2, const vec2 max2) {
  return min2 + (x - min1) / (max1 - min1) * (max2 - min2);
}

vec3 e1Remap(const vec3 x, const vec3 min1, const vec3 max1, const vec3 min2, const vec3 max2) {
  return min2 + (x - min1) / (max1 - min1) * (max2 - min2);
}

vec4 e1Remap(const vec4 x, const vec4 min1, const vec4 max1, const vec4 min2, const vec4 max2) {
  return min2 + (x - min1) / (max1 - min1) * (max2 - min2);
}

float e1RemapClamped(
  const float x,
  const float min1,
  const float max1,
  const float min2,
  const float max2
) {
  return clamp(min2 + (x - min1) / (max1 - min1) * (max2 - min2), min2, max2);
}

vec2 e1RemapClamped(
  const vec2 x,
  const vec2 min1,
  const vec2 max1,
  const vec2 min2,
  const vec2 max2
) {
  return clamp(min2 + (x - min1) / (max1 - min1) * (max2 - min2), min2, max2);
}

vec3 e1RemapClamped(
  const vec3 x,
  const vec3 min1,
  const vec3 max1,
  const vec3 min2,
  const vec3 max2
) {
  return clamp(min2 + (x - min1) / (max1 - min1) * (max2 - min2), min2, max2);
}

vec4 e1RemapClamped(
  const vec4 x,
  const vec4 min1,
  const vec4 max1,
  const vec4 min2,
  const vec4 max2
) {
  return clamp(min2 + (x - min1) / (max1 - min1) * (max2 - min2), min2, max2);
}

// Implicitly e1Remap to 0 and 1
float e1Remap(const float x, const float min1, const float max1) {
  return (x - min1) / (max1 - min1);
}

vec2 e1Remap(const vec2 x, const vec2 min1, const vec2 max1) {
  return (x - min1) / (max1 - min1);
}

vec3 e1Remap(const vec3 x, const vec3 min1, const vec3 max1) {
  return (x - min1) / (max1 - min1);
}

vec4 e1Remap(const vec4 x, const vec4 min1, const vec4 max1) {
  return (x - min1) / (max1 - min1);
}

float e1RemapClamped(const float x, const float min1, const float max1) {
  return saturate((x - min1) / (max1 - min1));
}

vec2 e1RemapClamped(const vec2 x, const vec2 min1, const vec2 max1) {
  return saturate((x - min1) / (max1 - min1));
}

vec3 e1RemapClamped(const vec3 x, const vec3 min1, const vec3 max1) {
  return saturate((x - min1) / (max1 - min1));
}

vec4 e1RemapClamped(const vec4 x, const vec4 min1, const vec4 max1) {
  return saturate((x - min1) / (max1 - min1));
}

// Reference: https://jcgt.org/published/0003/02/01/paper.pdf

vec2 e1SignNotZero(vec2 v) {
  return vec2(v.x >= 0.0 ? 1.0 : -1.0, v.y >= 0.0 ? 1.0 : -1.0);
}

vec2 e1PackNormalToVec2(vec3 v) {
  vec2 p = v.xy * (1.0 / (abs(v.x) + abs(v.y) + abs(v.z)));
  return v.z <= 0.0
    ? (1.0 - abs(p.yx)) * e1SignNotZero(p)
    : p;
}

vec3 e1UnpackVec2ToNormal(vec2 e) {
  vec3 v = vec3(e.xy, 1.0 - abs(e.x) - abs(e.y));
  if (v.z < 0.0) {
    v.xy = (1.0 - abs(v.yx)) * e1SignNotZero(v.xy);
  }
  return normalize(v);
}

vec3 e1ScreenToView(
  const vec2 uv,
  const float depth,
  const float viewZ,
  const mat4 e1ProjectionMatrix,
  const mat4 e1InverseProjectionMatrix
) {
  vec4 clip = vec4(vec3(uv, depth) * 2.0 - 1.0, 1.0);
  float clipW = e1ProjectionMatrix[2][3] * viewZ + e1ProjectionMatrix[3][3];
  clip *= clipW;
  return (e1InverseProjectionMatrix * clip).xyz;
}

#ifdef e1HAS_SHADOW
float e1RaySphereFirstIntersection(
  const vec3 origin,
  const vec3 direction,
  const vec3 center,
  const float radius
) {
  vec3 a = origin - center;
  float b = 2.0 * dot(direction, a);
  float c = dot(a, a) - radius * radius;
  float discriminant = b * b - 4.0 * c;
  return discriminant < 0.0
    ? -1.0
    : (-b - sqrt(discriminant)) * 0.5;
}

float e1RaySphereFirstIntersection(const vec3 origin, const vec3 direction, const float radius) {
  return e1RaySphereFirstIntersection(origin, direction, vec3(0.0), radius);
}

vec4 e1RaySphereFirstIntersection(
  const vec3 origin,
  const vec3 direction,
  const vec3 center,
  const vec4 radius
) {
  vec3 a = origin - center;
  float b = 2.0 * dot(direction, a);
  vec4 c = dot(a, a) - radius * radius;
  vec4 discriminant = b * b - 4.0 * c;
  vec4 mask = step(discriminant, vec4(0.0));
  return mix((-b - sqrt(max(vec4(0.0), discriminant))) * 0.5, vec4(-1.0), mask);
}

vec4 e1RaySphereFirstIntersection(const vec3 origin, const vec3 direction, const vec4 radius) {
  return e1RaySphereFirstIntersection(origin, direction, vec3(0.0), radius);
}

float e1RaySphereSecondIntersection(
  const vec3 origin,
  const vec3 direction,
  const vec3 center,
  const float radius
) {
  vec3 a = origin - center;
  float b = 2.0 * dot(direction, a);
  float c = dot(a, a) - radius * radius;
  float discriminant = b * b - 4.0 * c;
  return discriminant < 0.0
    ? -1.0
    : (-b + sqrt(discriminant)) * 0.5;
}

float e1RaySphereSecondIntersection(const vec3 origin, const vec3 direction, const float radius) {
  return e1RaySphereSecondIntersection(origin, direction, vec3(0.0), radius);
}

vec4 e1RaySphereSecondIntersection(
  const vec3 origin,
  const vec3 direction,
  const vec3 center,
  const vec4 radius
) {
  vec3 a = origin - center;
  float b = 2.0 * dot(direction, a);
  vec4 c = dot(a, a) - radius * radius;
  vec4 discriminant = b * b - 4.0 * c;
  vec4 mask = step(discriminant, vec4(0.0));
  return mix((-b + sqrt(max(vec4(0.0), discriminant))) * 0.5, vec4(-1.0), mask);
}

vec4 e1RaySphereSecondIntersection(const vec3 origin, const vec3 direction, const vec4 radius) {
  return e1RaySphereSecondIntersection(origin, direction, vec3(0.0), radius);
}

void e1RaySphereIntersections(
  const vec3 origin,
  const vec3 direction,
  const vec3 center,
  const float radius,
  out float intersection1,
  out float intersection2
) {
  vec3 a = origin - center;
  float b = 2.0 * dot(direction, a);
  float c = dot(a, a) - radius * radius;
  float discriminant = b * b - 4.0 * c;
  if (discriminant < 0.0) {
    intersection1 = -1.0;
    intersection2 = -1.0;
    return;
  } else {
    float Q = sqrt(discriminant);
    intersection1 = (-b - Q) * 0.5;
    intersection2 = (-b + Q) * 0.5;
  }
}

void e1RaySphereIntersections(
  const vec3 origin,
  const vec3 direction,
  const float radius,
  out float intersection1,
  out float intersection2
) {
  e1RaySphereIntersections(origin, direction, vec3(0.0), radius, intersection1, intersection2);
}

void e1RaySphereIntersections(
  const vec3 origin,
  const vec3 direction,
  const vec3 center,
  const vec4 radius,
  out vec4 intersection1,
  out vec4 intersection2
) {
  vec3 a = origin - center;
  float b = 2.0 * dot(direction, a);
  vec4 c = dot(a, a) - radius * radius;
  vec4 discriminant = b * b - 4.0 * c;
  vec4 mask = step(discriminant, vec4(0.0));
  vec4 Q = sqrt(max(vec4(0.0), discriminant));
  intersection1 = mix((-b - Q) * 0.5, vec4(-1.0), mask);
  intersection2 = mix((-b + Q) * 0.5, vec4(-1.0), mask);
}

void e1RaySphereIntersections(
  const vec3 origin,
  const vec3 direction,
  const vec4 radius,
  out vec4 intersection1,
  out vec4 intersection2
) {
  e1RaySphereIntersections(origin, direction, vec3(0.0), radius, intersection1, intersection2);
}

// Reference: https://github.com/mrdoob/three.js/blob/r171/examples/jsm/csm/CSMShader.js

#ifndef e1SHADOW_CASCADE_COUNT
#error "e1SHADOW_CASCADE_COUNT macro must be defined."
#endif // e1SHADOW_CASCADE_COUNT

int getCascadeIndex(
  const mat4 e1ViewMatrix,
  const vec3 worldPosition,
  const vec2 intervals[e1SHADOW_CASCADE_COUNT],
  const float near,
  const float far
) {
  vec4 viewPosition = e1ViewMatrix * vec4(worldPosition, 1.0);
  float depth = viewZToOrthographicDepth(viewPosition.z, near, far);
  vec2 interval;
  
    #if 0 < e1SHADOW_CASCADE_COUNT
    interval = intervals[0];
    if (depth >= interval.x && depth < interval.y) {
      return 0;
    }
    #endif // 0 < e1SHADOW_CASCADE_COUNT
  
    #if 1 < e1SHADOW_CASCADE_COUNT
    interval = intervals[1];
    if (depth >= interval.x && depth < interval.y) {
      return 1;
    }
    #endif // 1 < e1SHADOW_CASCADE_COUNT
  
    #if 2 < e1SHADOW_CASCADE_COUNT
    interval = intervals[2];
    if (depth >= interval.x && depth < interval.y) {
      return 2;
    }
    #endif // 2 < e1SHADOW_CASCADE_COUNT
  
    #if 3 < e1SHADOW_CASCADE_COUNT
    interval = intervals[3];
    if (depth >= interval.x && depth < interval.y) {
      return 3;
    }
    #endif // 3 < e1SHADOW_CASCADE_COUNT
  
  return e1SHADOW_CASCADE_COUNT - 1;
}

int getFadedCascadeIndex(
  const mat4 e1ViewMatrix,
  const vec3 worldPosition,
  const vec2 intervals[e1SHADOW_CASCADE_COUNT],
  const float near,
  const float far,
  const float jitter
) {
  vec4 viewPosition = e1ViewMatrix * vec4(worldPosition, 1.0);
  float depth = viewZToOrthographicDepth(viewPosition.z, near, far);

  vec2 interval;
  float intervalCenter;
  float closestEdge;
  float margin;
  int nextIndex = -1;
  int prevIndex = -1;
  float alpha;

  
    #if 0 < e1SHADOW_CASCADE_COUNT
    interval = intervals[0];
    intervalCenter = (interval.x + interval.y) * 0.5;
    closestEdge = depth < intervalCenter ? interval.x : interval.y;
    margin = closestEdge * closestEdge * 0.5;
    interval += margin * vec2(-0.5, 0.5);

    #if 0 < e1SHADOW_CASCADE_COUNT - 1
    if (depth >= interval.x && depth < interval.y) {
      prevIndex = nextIndex;
      nextIndex = 0;
      alpha = saturate(min(depth - interval.x, interval.y - depth) / margin);
    }
    #else // 0 < e1SHADOW_CASCADE_COUNT - 1
    // Don't fade out the last cascade.
    if (depth >= interval.x) {
      prevIndex = nextIndex;
      nextIndex = 0;
      alpha = saturate((depth - interval.x) / margin);
    }
    #endif // 0 < e1SHADOW_CASCADE_COUNT - 1
    #endif // 0 < e1SHADOW_CASCADE_COUNT
  
    #if 1 < e1SHADOW_CASCADE_COUNT
    interval = intervals[1];
    intervalCenter = (interval.x + interval.y) * 0.5;
    closestEdge = depth < intervalCenter ? interval.x : interval.y;
    margin = closestEdge * closestEdge * 0.5;
    interval += margin * vec2(-0.5, 0.5);

    #if 1 < e1SHADOW_CASCADE_COUNT - 1
    if (depth >= interval.x && depth < interval.y) {
      prevIndex = nextIndex;
      nextIndex = 1;
      alpha = saturate(min(depth - interval.x, interval.y - depth) / margin);
    }
    #else // 1 < e1SHADOW_CASCADE_COUNT - 1
    // Don't fade out the last cascade.
    if (depth >= interval.x) {
      prevIndex = nextIndex;
      nextIndex = 1;
      alpha = saturate((depth - interval.x) / margin);
    }
    #endif // 1 < e1SHADOW_CASCADE_COUNT - 1
    #endif // 1 < e1SHADOW_CASCADE_COUNT
  
    #if 2 < e1SHADOW_CASCADE_COUNT
    interval = intervals[2];
    intervalCenter = (interval.x + interval.y) * 0.5;
    closestEdge = depth < intervalCenter ? interval.x : interval.y;
    margin = closestEdge * closestEdge * 0.5;
    interval += margin * vec2(-0.5, 0.5);

    #if 2 < e1SHADOW_CASCADE_COUNT - 1
    if (depth >= interval.x && depth < interval.y) {
      prevIndex = nextIndex;
      nextIndex = 2;
      alpha = saturate(min(depth - interval.x, interval.y - depth) / margin);
    }
    #else // 2 < e1SHADOW_CASCADE_COUNT - 1
    // Don't fade out the last cascade.
    if (depth >= interval.x) {
      prevIndex = nextIndex;
      nextIndex = 2;
      alpha = saturate((depth - interval.x) / margin);
    }
    #endif // 2 < e1SHADOW_CASCADE_COUNT - 1
    #endif // 2 < e1SHADOW_CASCADE_COUNT
  
    #if 3 < e1SHADOW_CASCADE_COUNT
    interval = intervals[3];
    intervalCenter = (interval.x + interval.y) * 0.5;
    closestEdge = depth < intervalCenter ? interval.x : interval.y;
    margin = closestEdge * closestEdge * 0.5;
    interval += margin * vec2(-0.5, 0.5);

    #if 3 < e1SHADOW_CASCADE_COUNT - 1
    if (depth >= interval.x && depth < interval.y) {
      prevIndex = nextIndex;
      nextIndex = 3;
      alpha = saturate(min(depth - interval.x, interval.y - depth) / margin);
    }
    #else // 3 < e1SHADOW_CASCADE_COUNT - 1
    // Don't fade out the last cascade.
    if (depth >= interval.x) {
      prevIndex = nextIndex;
      nextIndex = 3;
      alpha = saturate((depth - interval.x) / margin);
    }
    #endif // 3 < e1SHADOW_CASCADE_COUNT - 1
    #endif // 3 < e1SHADOW_CASCADE_COUNT
  

  return jitter <= alpha
    ? nextIndex
    : prevIndex;
}

// Reference: https://advances.realtimerendering.com/s2014/index.html#_NEXT_GENERATION_POST

float e1InterleavedGradientNoise(const vec2 coord) {
  const vec3 magic = vec3(0.06711056, 0.00583715, 52.9829189);
  return fract(magic.z * fract(dot(coord, magic.xy)));
}

// Reference: https://www.gamedev.net/tutorials/programming/graphics/contact-hardening-soft-shadows-made-fast-r4906/

vec2 e1VogelDisk(const int index, const int sampleCount, const float phi) {
  const float goldenAngle = 2.39996322972865332;
  float r = sqrt(float(index) + 0.5) / sqrt(float(sampleCount));
  float theta = float(index) * goldenAngle + phi;
  return r * vec2(cos(theta), sin(theta));
}

#endif // e1HAS_SHADOW

// Based on: https://github.com/ebruneton/precomputed_atmospheric_scattering/blob/master/atmosphere/definitions.glsl

/**
 * Copyright (c) 2017 Eric Bruneton
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the copyright holders nor the names of its
 *    contributors may be used to endorse or promote products derived from
 *    this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
 * THE POSSIBILITY OF SUCH DAMAGE.
 */

#define assert(x)

#define Length float
#define Wavelength float
#define Angle float
#define SolidAngle float
#define Power float
#define LuminousPower float

#define Number float
#define InverseLength float
#define Area float
#define Volume float
#define NumberDensity float
#define Irradiance float
#define Radiance float
#define SpectralPower float
#define SpectralIrradiance float
#define SpectralRadiance float
#define SpectralRadianceDensity float
#define ScatteringCoefficient float
#define InverseSolidAngle float
#define LuminousIntensity float
#define Luminance float
#define Illuminance float

// A generic function from Wavelength to some other type.
#define AbstractSpectrum vec3
// A function from Wavelength to Number.
#define DimensionlessSpectrum vec3
// A function from Wavelength to SpectralPower.
#define PowerSpectrum vec3
// A function from Wavelength to SpectralIrradiance.
#define IrradianceSpectrum vec3
// A function from Wavelength to SpectralRadiance.
#define RadianceSpectrum vec3
// A function from Wavelength to SpectralRadianceDensity.
#define RadianceDensitySpectrum vec3
// A function from Wavelength to ScatteringCoefficient.
#define ScatteringSpectrum vec3

// A position in 3D (3 length values).
#define Position vec3
// A unit direction vector in 3D (3 unit-less values).
#define Direction vec3
// A vector of 3 luminance values.
#define Luminance3 vec3
// A vector of 3 illuminance values.
#define Illuminance3 vec3

#define TransmittanceTexture sampler2D
#define AbstractScatteringTexture sampler3D
#define ReducedScatteringTexture sampler3D
#define ScatteringTexture sampler3D
#define ScatteringDensityTexture sampler3D
#define IrradianceTexture sampler2D

const Length m = 1.0;
const Wavelength nm = 1.0;
const Angle rad = 1.0;
const SolidAngle sr = 1.0;
const Power watt = 1.0;
const LuminousPower lm = 1.0;

#if !defined(PI)
const float PI = 3.14159265358979323846;
#endif // !defined(PI)

const Length km = 1000.0 * m;
const Area m2 = m * m;
const Volume m3 = m * m * m;
const Angle pi = PI * rad;
const Angle deg = pi / 180.0;
const Irradiance watt_per_square_meter = watt / m2;
const Radiance watt_per_square_meter_per_sr = watt / (m2 * sr);
const SpectralIrradiance watt_per_square_meter_per_nm = watt / (m2 * nm);
const SpectralRadiance watt_per_square_meter_per_sr_per_nm = watt / (m2 * sr * nm);
const SpectralRadianceDensity watt_per_cubic_meter_per_sr_per_nm = watt / (m3 * sr * nm);
const LuminousIntensity cd = lm / sr;
const LuminousIntensity kcd = 1000.0 * cd;
const Luminance cd_per_square_meter = cd / m2;
const Luminance kcd_per_square_meter = kcd / m2;

struct DensityProfileLayer {
  Length width;
  Number exp_term;
  InverseLength exp_scale;
  InverseLength linear_term;
  Number constant_term;
};

struct DensityProfile {
  DensityProfileLayer layers[2];
};

// See AtmosphereParameter.ts for further details.
struct AtmosphereParameters {
  IrradianceSpectrum solar_irradiance;
  Angle sun_angular_radius;
  Length bottom_radius;
  Length top_radius;
  DensityProfile rayleigh_density;
  ScatteringSpectrum rayleigh_scattering;
  DensityProfile mie_density;
  ScatteringSpectrum mie_scattering;
  ScatteringSpectrum mie_extinction;
  Number mie_phase_function_g;
  DensityProfile absorption_density;
  ScatteringSpectrum absorption_extinction;
  DimensionlessSpectrum ground_albedo;
  Number mu_s_min;
};


uniform AtmosphereParameters e1ATMOSPHERE;
uniform vec3 e1SUN_SPECTRAL_RADIANCE_TO_LUMINANCE;
uniform vec3 e1SKY_SPECTRAL_RADIANCE_TO_LUMINANCE;

uniform sampler2D e1Transmittance_texture;
uniform sampler3D e1Scattering_texture;
uniform sampler2D e1Irradiance_texture;
uniform sampler3D e1Single_mie_scattering_texture;
uniform sampler3D e1Higher_order_scattering_texture;

// Based on: https://github.com/ebruneton/precomputed_atmospheric_scattering/blob/master/atmosphere/functions.glsl

/**
 * Copyright (c) 2017 Eric Bruneton
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the copyright holders nor the names of its
 *    contributors may be used to endorse or promote products derived from
 *    this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
 * THE POSSIBILITY OF SUCH DAMAGE.
 *
 * Precomputed Atmospheric Scattering
 * Copyright (c) 2008 INRIA
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the copyright holders nor the names of its
 *    contributors may be used to endorse or promote products derived from
 *    this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
 * THE POSSIBILITY OF SUCH DAMAGE.
 */

Number e1ClampCosine(const Number mu) {
  return clamp(mu, Number(-1.0), Number(1.0));
}

Length e1ClampDistance(const Length d) {
  return max(d, 0.0 * m);
}

Length e1ClampRadius(const AtmosphereParameters atmosphere, const Length r) {
  return clamp(r, atmosphere.bottom_radius, atmosphere.top_radius);
}

Length e1SafeSqrt(const Area a) {
  return sqrt(max(a, 0.0 * m2));
}

Length e1DistanceToTopAtmosphereBoundary(const AtmosphereParameters atmosphere,
    const Length r, const Number mu) {
  assert(r <= atmosphere.top_radius);
  assert(mu >= -1.0 && mu <= 1.0);
  Area discriminant = r * r * (mu * mu - 1.0) +
      atmosphere.top_radius * atmosphere.top_radius;
  return e1ClampDistance(-r * mu + e1SafeSqrt(discriminant));
}

Length e1DistanceToBottomAtmosphereBoundary(const AtmosphereParameters atmosphere,
    const Length r, const Number mu) {
  assert(r >= atmosphere.bottom_radius);
  assert(mu >= -1.0 && mu <= 1.0);
  Area discriminant = r * r * (mu * mu - 1.0) +
      atmosphere.bottom_radius * atmosphere.bottom_radius;
  return e1ClampDistance(-r * mu - e1SafeSqrt(discriminant));
}

bool e1RayIntersectsGround(const AtmosphereParameters atmosphere,
    const Length r, const Number mu) {
  assert(r >= atmosphere.bottom_radius);
  assert(mu >= -1.0 && mu <= 1.0);
  return mu < 0.0 && r * r * (mu * mu - 1.0) +
      atmosphere.bottom_radius * atmosphere.bottom_radius >= 0.0 * m2;
}

Number e1GetTextureCoordFromUnitRange(const Number x, const int texture_size) {
  return 0.5 / Number(texture_size) + x * (1.0 - 1.0 / Number(texture_size));
}

vec2 e1GetTransmittanceTextureUvFromRMu(const AtmosphereParameters atmosphere,
    const Length r, const Number mu) {
  assert(r >= atmosphere.bottom_radius && r <= atmosphere.top_radius);
  assert(mu >= -1.0 && mu <= 1.0);
  // Distance to top atmosphere boundary for a horizontal ray at ground level.
  Length H = sqrt(atmosphere.top_radius * atmosphere.top_radius -
      atmosphere.bottom_radius * atmosphere.bottom_radius);
  // Distance to the horizon.
  Length rho =
      e1SafeSqrt(r * r - atmosphere.bottom_radius * atmosphere.bottom_radius);
  // Distance to the top atmosphere boundary for the ray (r,mu), and its minimum
  // and maximum values over all mu - obtained for (r,1) and (r,mu_horizon).
  Length d = e1DistanceToTopAtmosphereBoundary(atmosphere, r, mu);
  Length d_min = atmosphere.top_radius - r;
  Length d_max = rho + H;
  Number x_mu = (d - d_min) / (d_max - d_min);
  Number x_r = rho / H;
  return vec2(e1GetTextureCoordFromUnitRange(x_mu, e1TRANSMITTANCE_TEXTURE_WIDTH),
              e1GetTextureCoordFromUnitRange(x_r, e1TRANSMITTANCE_TEXTURE_HEIGHT));
}

DimensionlessSpectrum e1GetTransmittanceToTopAtmosphereBoundary(
    const AtmosphereParameters atmosphere,
    const TransmittanceTexture e1Transmittance_texture,
    const Length r, const Number mu) {
  assert(r >= atmosphere.bottom_radius && r <= atmosphere.top_radius);
  vec2 uv = e1GetTransmittanceTextureUvFromRMu(atmosphere, r, mu);
  // @shotamatsuda: Added for the precomputation stage in half-float precision.
  #ifdef TRANSMITTANCE_PRECISION_LOG
  // Manually interpolate the transmittance instead of the optical depth.
  const vec2 size = vec2(e1TRANSMITTANCE_TEXTURE_WIDTH, e1TRANSMITTANCE_TEXTURE_HEIGHT);
  const vec3 texel_size = vec3(1.0 / size, 0.0);
  vec2 coord = (uv * size) - 0.5;
  vec2 i = (floor(coord) + 0.5) * texel_size.xy;
  vec2 f = fract(coord);
  vec4 t1 = exp(-texture(e1Transmittance_texture, i));
  vec4 t2 = exp(-texture(e1Transmittance_texture, i + texel_size.xz));
  vec4 t3 = exp(-texture(e1Transmittance_texture, i + texel_size.zy));
  vec4 t4 = exp(-texture(e1Transmittance_texture, i + texel_size.xy));
  return DimensionlessSpectrum(mix(mix(t1, t2, f.x), mix(t3, t4, f.x), f.y));
  #else // TRANSMITTANCE_PRECISION_LOG
  return DimensionlessSpectrum(texture(e1Transmittance_texture, uv));
  #endif // TRANSMITTANCE_PRECISION_LOG
}

DimensionlessSpectrum e1GetTransmittance(
    const AtmosphereParameters atmosphere,
    const TransmittanceTexture e1Transmittance_texture,
    const Length r, const Number mu, const Length d,
    const bool ray_r_mu_intersects_ground) {
  assert(r >= atmosphere.bottom_radius && r <= atmosphere.top_radius);
  assert(mu >= -1.0 && mu <= 1.0);
  assert(d >= 0.0 * m);

  Length r_d = e1ClampRadius(atmosphere, sqrt(d * d + 2.0 * r * mu * d + r * r));
  Number mu_d = e1ClampCosine((r * mu + d) / r_d);

  if (ray_r_mu_intersects_ground) {
    return min(
        e1GetTransmittanceToTopAtmosphereBoundary(
            atmosphere, e1Transmittance_texture, r_d, -mu_d) /
        e1GetTransmittanceToTopAtmosphereBoundary(
            atmosphere, e1Transmittance_texture, r, -mu),
        DimensionlessSpectrum(1.0));
  } else {
    return min(
        e1GetTransmittanceToTopAtmosphereBoundary(
            atmosphere, e1Transmittance_texture, r, mu) /
        e1GetTransmittanceToTopAtmosphereBoundary(
            atmosphere, e1Transmittance_texture, r_d, mu_d),
        DimensionlessSpectrum(1.0));
  }
}

DimensionlessSpectrum e1GetTransmittanceToSun(
    const AtmosphereParameters atmosphere,
    const TransmittanceTexture e1Transmittance_texture,
    const Length r, const Number mu_s) {
  Number sin_theta_h = atmosphere.bottom_radius / r;
  Number cos_theta_h = -sqrt(max(1.0 - sin_theta_h * sin_theta_h, 0.0));
  return e1GetTransmittanceToTopAtmosphereBoundary(
          atmosphere, e1Transmittance_texture, r, mu_s) *
      smoothstep(-sin_theta_h * atmosphere.sun_angular_radius / rad,
                 sin_theta_h * atmosphere.sun_angular_radius / rad,
                 mu_s - cos_theta_h);
}

InverseSolidAngle e1RayleighPhaseFunction(const Number nu) {
  InverseSolidAngle k = 3.0 / (16.0 * PI * sr);
  return k * (1.0 + nu * nu);
}

InverseSolidAngle e1MiePhaseFunction(const Number g, const Number nu) {
  InverseSolidAngle k = 3.0 / (8.0 * PI * sr) * (1.0 - g * g) / (2.0 + g * g);
  return k * (1.0 + nu * nu) / pow(1.0 + g * g - 2.0 * g * nu, 1.5);
}

vec4 e1GetScatteringTextureUvwzFromRMuMuSNu(const AtmosphereParameters atmosphere,
    const Length r, const Number mu, const Number mu_s, const Number nu,
    const bool ray_r_mu_intersects_ground) {
  assert(r >= atmosphere.bottom_radius && r <= atmosphere.top_radius);
  assert(mu >= -1.0 && mu <= 1.0);
  assert(mu_s >= -1.0 && mu_s <= 1.0);
  assert(nu >= -1.0 && nu <= 1.0);

  // Distance to top atmosphere boundary for a horizontal ray at ground level.
  Length H = sqrt(atmosphere.top_radius * atmosphere.top_radius -
      atmosphere.bottom_radius * atmosphere.bottom_radius);
  // Distance to the horizon.
  Length rho =
      e1SafeSqrt(r * r - atmosphere.bottom_radius * atmosphere.bottom_radius);
  Number u_r = e1GetTextureCoordFromUnitRange(rho / H, e1SCATTERING_TEXTURE_R_SIZE);

  // Discriminant of the quadratic equation for the intersections of the ray
  // (r,mu) with the ground (see e1RayIntersectsGround).
  Length r_mu = r * mu;
  Area discriminant =
      r_mu * r_mu - r * r + atmosphere.bottom_radius * atmosphere.bottom_radius;
  Number u_mu;
  if (ray_r_mu_intersects_ground) {
    // Distance to the ground for the ray (r,mu), and its minimum and maximum
    // values over all mu - obtained for (r,-1) and (r,mu_horizon).
    Length d = -r_mu - e1SafeSqrt(discriminant);
    Length d_min = r - atmosphere.bottom_radius;
    Length d_max = rho;
    u_mu = 0.5 - 0.5 * e1GetTextureCoordFromUnitRange(d_max == d_min ? 0.0 :
        (d - d_min) / (d_max - d_min), e1SCATTERING_TEXTURE_MU_SIZE / 2);
  } else {
    // Distance to the top atmosphere boundary for the ray (r,mu), and its
    // minimum and maximum values over all mu - obtained for (r,1) and
    // (r,mu_horizon).
    Length d = -r_mu + e1SafeSqrt(discriminant + H * H);
    Length d_min = atmosphere.top_radius - r;
    Length d_max = rho + H;
    u_mu = 0.5 + 0.5 * e1GetTextureCoordFromUnitRange(
        (d - d_min) / (d_max - d_min), e1SCATTERING_TEXTURE_MU_SIZE / 2);
  }

  Length d = e1DistanceToTopAtmosphereBoundary(
      atmosphere, atmosphere.bottom_radius, mu_s);
  Length d_min = atmosphere.top_radius - atmosphere.bottom_radius;
  Length d_max = H;
  Number a = (d - d_min) / (d_max - d_min);
  Length D = e1DistanceToTopAtmosphereBoundary(
      atmosphere, atmosphere.bottom_radius, atmosphere.mu_s_min);
  Number A = (D - d_min) / (d_max - d_min);
  // An ad-hoc function equal to 0 for mu_s = mu_s_min (because then d = D and
  // thus a = A), equal to 1 for mu_s = 1 (because then d = d_min and thus
  // a = 0), and with a large slope around mu_s = 0, to get more texture
  // samples near the horizon.
  Number u_mu_s = e1GetTextureCoordFromUnitRange(
      max(1.0 - a / A, 0.0) / (1.0 + a), e1SCATTERING_TEXTURE_MU_S_SIZE);

  Number u_nu = (nu + 1.0) / 2.0;
  return vec4(u_nu, u_mu_s, u_mu, u_r);
}

vec2 e1GetIrradianceTextureUvFromRMuS(const AtmosphereParameters atmosphere,
    const Length r, const Number mu_s) {
  assert(r >= atmosphere.bottom_radius && r <= atmosphere.top_radius);
  assert(mu_s >= -1.0 && mu_s <= 1.0);
  Number x_r = (r - atmosphere.bottom_radius) /
      (atmosphere.top_radius - atmosphere.bottom_radius);
  Number x_mu_s = mu_s * 0.5 + 0.5;
  return vec2(e1GetTextureCoordFromUnitRange(x_mu_s, e1IRRADIANCE_TEXTURE_WIDTH),
              e1GetTextureCoordFromUnitRange(x_r, e1IRRADIANCE_TEXTURE_HEIGHT));
}

IrradianceSpectrum e1GetIrradiance(
    const AtmosphereParameters atmosphere,
    const IrradianceTexture e1Irradiance_texture,
    const Length r, const Number mu_s) {
  vec2 uv = e1GetIrradianceTextureUvFromRMuS(atmosphere, r, mu_s);
  return IrradianceSpectrum(texture(e1Irradiance_texture, uv));
}

// Based on: https://github.com/ebruneton/precomputed_atmospheric_scattering/blob/master/atmosphere/functions.glsl

/**
 * Copyright (c) 2017 Eric Bruneton
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the copyright holders nor the names of its
 *    contributors may be used to endorse or promote products derived from
 *    this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
 * THE POSSIBILITY OF SUCH DAMAGE.
 *
 * Precomputed Atmospheric Scattering
 * Copyright (c) 2008 INRIA
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the copyright holders nor the names of its
 *    contributors may be used to endorse or promote products derived from
 *    this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
 * THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifdef e1COMBINED_SCATTERING_TEXTURES
vec3 e1GetExtrapolatedSingleMieScattering(
    const AtmosphereParameters atmosphere, const vec4 scattering) {
  // Algebraically this can never be negative, but rounding errors can produce
  // that effect for sufficiently short view rays.
  // @shotamatsuda: Avoid division by infinitesimal values.
  // See https://github.com/takram-design-engineering/three-geospatial/issues/47
  if (scattering.r < 1e-5) {
    return vec3(0.0);
  }
  return scattering.rgb * scattering.a / scattering.r *
	    (atmosphere.rayleigh_scattering.r / atmosphere.mie_scattering.r) *
	    (atmosphere.mie_scattering / atmosphere.rayleigh_scattering);
}
#endif // e1COMBINED_SCATTERING_TEXTURES

IrradianceSpectrum e1GetCombinedScattering(
    const AtmosphereParameters atmosphere,
    const ReducedScatteringTexture e1Scattering_texture,
    const ReducedScatteringTexture e1Single_mie_scattering_texture,
    const Length r, const Number mu, const Number mu_s, const Number nu,
    const bool ray_r_mu_intersects_ground,
    out IrradianceSpectrum single_mie_scattering) {
  vec4 uvwz = e1GetScatteringTextureUvwzFromRMuMuSNu(
      atmosphere, r, mu, mu_s, nu, ray_r_mu_intersects_ground);
  Number tex_coord_x = uvwz.x * Number(e1SCATTERING_TEXTURE_NU_SIZE - 1);
  Number tex_x = floor(tex_coord_x);
  Number lerp = tex_coord_x - tex_x;
  vec3 uvw0 = vec3((tex_x + uvwz.y) / Number(e1SCATTERING_TEXTURE_NU_SIZE),
      uvwz.z, uvwz.w);
  vec3 uvw1 = vec3((tex_x + 1.0 + uvwz.y) / Number(e1SCATTERING_TEXTURE_NU_SIZE),
      uvwz.z, uvwz.w);
#ifdef e1COMBINED_SCATTERING_TEXTURES
  vec4 combined_scattering =
      texture(e1Scattering_texture, uvw0) * (1.0 - lerp) +
      texture(e1Scattering_texture, uvw1) * lerp;
  IrradianceSpectrum scattering = IrradianceSpectrum(combined_scattering);
  single_mie_scattering =
      e1GetExtrapolatedSingleMieScattering(atmosphere, combined_scattering);
#else // e1COMBINED_SCATTERING_TEXTURES
  IrradianceSpectrum scattering = IrradianceSpectrum(
      texture(e1Scattering_texture, uvw0) * (1.0 - lerp) +
      texture(e1Scattering_texture, uvw1) * lerp);
  single_mie_scattering = IrradianceSpectrum(
      texture(e1Single_mie_scattering_texture, uvw0) * (1.0 - lerp) +
      texture(e1Single_mie_scattering_texture, uvw1) * lerp);
#endif // e1COMBINED_SCATTERING_TEXTURES
  return scattering;
}

// @shotamatsuda: Added for reading higher-order scattering texture.
#ifdef e1HAS_HIGHER_ORDER_SCATTERING_TEXTURE
IrradianceSpectrum e1GetScattering(
    const AtmosphereParameters atmosphere,
    const ReducedScatteringTexture e1Scattering_texture,
    const Length r, const Number mu, const Number mu_s, const Number nu,
    const bool ray_r_mu_intersects_ground) {
  vec4 uvwz = e1GetScatteringTextureUvwzFromRMuMuSNu(
      atmosphere, r, mu, mu_s, nu, ray_r_mu_intersects_ground);
  Number tex_coord_x = uvwz.x * Number(e1SCATTERING_TEXTURE_NU_SIZE - 1);
  Number tex_x = floor(tex_coord_x);
  Number lerp = tex_coord_x - tex_x;
  vec3 uvw0 = vec3((tex_x + uvwz.y) / Number(e1SCATTERING_TEXTURE_NU_SIZE),
      uvwz.z, uvwz.w);
  vec3 uvw1 = vec3((tex_x + 1.0 + uvwz.y) / Number(e1SCATTERING_TEXTURE_NU_SIZE),
      uvwz.z, uvwz.w);
  IrradianceSpectrum scattering = IrradianceSpectrum(
      texture(e1Scattering_texture, uvw0) * (1.0 - lerp) +
      texture(e1Scattering_texture, uvw1) * lerp);
  return scattering;
}
#endif // e1HAS_HIGHER_ORDER_SCATTERING_TEXTURE

RadianceSpectrum e1GetSkyRadiance(
    const AtmosphereParameters atmosphere,
    const TransmittanceTexture e1Transmittance_texture,
    const ReducedScatteringTexture e1Scattering_texture,
    const ReducedScatteringTexture e1Single_mie_scattering_texture,
    Position camera, const Direction view_ray, const Length shadow_length,
    const Direction sun_direction,
    out DimensionlessSpectrum transmittance) {
  // Compute the distance to the top atmosphere boundary along the view ray,
  // assuming the viewer is in space (or NaN if the view ray does not intersect
  // the atmosphere).
  Length r = length(camera);
  Length rmu = dot(camera, view_ray);
  // @shotamatsuda: Use e1SafeSqrt instead.
  // See: https://github.com/takram-design-engineering/three-geospatial/pull/26
  Length distance_to_top_atmosphere_boundary = -rmu -
      e1SafeSqrt(rmu * rmu - r * r +
          atmosphere.top_radius * atmosphere.top_radius);
  // If the viewer is in space and the view ray intersects the atmosphere, move
  // the viewer to the top atmosphere boundary (along the view ray):
  if (distance_to_top_atmosphere_boundary > 0.0 * m) {
    camera = camera + view_ray * distance_to_top_atmosphere_boundary;
    r = atmosphere.top_radius;
    rmu += distance_to_top_atmosphere_boundary;
  } else if (r > atmosphere.top_radius) {
    // If the view ray does not intersect the atmosphere, simply return 0.
    transmittance = DimensionlessSpectrum(1.0);
    return RadianceSpectrum(0.0 * watt_per_square_meter_per_sr_per_nm);
  }
  // Compute the r, mu, mu_s and nu parameters needed for the texture lookups.
  Number mu = rmu / r;
  Number mu_s = dot(camera, sun_direction) / r;
  Number nu = dot(view_ray, sun_direction);

  // @shotamatsuda: For rendering points below the bottom atmosphere.
  #ifdef e1GROUND
  bool ray_r_mu_intersects_ground = e1RayIntersectsGround(atmosphere, r, mu);
  #else // e1GROUND
  bool ray_r_mu_intersects_ground = false;
  #endif // e1GROUND

  transmittance = ray_r_mu_intersects_ground ? DimensionlessSpectrum(0.0) :
      e1GetTransmittanceToTopAtmosphereBoundary(
          atmosphere, e1Transmittance_texture, r, mu);
  IrradianceSpectrum single_mie_scattering;
  IrradianceSpectrum scattering;
  if (shadow_length == 0.0 * m) {
    scattering = e1GetCombinedScattering(
        atmosphere, e1Scattering_texture, e1Single_mie_scattering_texture,
        r, mu, mu_s, nu, ray_r_mu_intersects_ground,
        single_mie_scattering);
  } else {
    // Case of light shafts (shadow_length is the total length noted l in our
    // paper): we omit the scattering between the camera and the point at
    // distance l, by implementing Eq. (18) of the paper (shadow_transmittance
    // is the T(x,x_s) term, scattering is the S|x_s=x+lv term).
    Length d = shadow_length;
    Length r_p =
        e1ClampRadius(atmosphere, sqrt(d * d + 2.0 * r * mu * d + r * r));
    Number mu_p = (r * mu + d) / r_p;
    Number mu_s_p = (r * mu_s + d * nu) / r_p;

    scattering = e1GetCombinedScattering(
        atmosphere, e1Scattering_texture, e1Single_mie_scattering_texture,
        r_p, mu_p, mu_s_p, nu, ray_r_mu_intersects_ground,
        single_mie_scattering);
    DimensionlessSpectrum shadow_transmittance =
        e1GetTransmittance(atmosphere, e1Transmittance_texture,
            r, mu, shadow_length, ray_r_mu_intersects_ground);
    // @shotamatsuda: Occlude only single Rayleigh scattering by the shadow.
#ifdef e1HAS_HIGHER_ORDER_SCATTERING_TEXTURE
    IrradianceSpectrum higher_order_scattering = e1GetScattering(
        atmosphere, e1Higher_order_scattering_texture,
        r_p, mu_p, mu_s_p, nu, ray_r_mu_intersects_ground);
    IrradianceSpectrum single_scattering = scattering - higher_order_scattering;
    scattering = single_scattering * shadow_transmittance + higher_order_scattering;
#else // e1HAS_HIGHER_ORDER_SCATTERING_TEXTURE
    scattering = scattering * shadow_transmittance;
#endif // e1HAS_HIGHER_ORDER_SCATTERING_TEXTURE
    single_mie_scattering = single_mie_scattering * shadow_transmittance;
  }
  return scattering * e1RayleighPhaseFunction(nu) + single_mie_scattering *
      e1MiePhaseFunction(atmosphere.mie_phase_function_g, nu);
}

// @shotamatsuda: Returns the point on the ray closest to the origin.
vec3 e1ClosestPointOnRay(const Position camera, const Position point) {
  Position ray = point - camera;
  Number t = clamp(-dot(camera, ray) / dot(ray, ray), 0.0, 1.0);
  return camera + t * ray;
}

vec2 e1RaySphereIntersections(
    const Position camera, const Direction direction, const Length radius) {
  float b = 2.0 * dot(direction, camera);
  float c = dot(camera, camera) - radius * radius;
  float discriminant = b * b - 4.0 * c;
  float Q = sqrt(discriminant);
  return vec2(-b - Q, -b + Q) * 0.5;
}

// @shotamatsuda: Clip the view ray at the bottom atmosphere boundary.
bool e1ClipAtBottomAtmosphere(
    const AtmosphereParameters atmosphere,
    const Direction view_ray, inout Position camera, inout Position point) {
  const Length eps = 0.0;
  Length bottom_radius = atmosphere.bottom_radius + eps;
  Length r_camera = length(camera);
  Length r_point = length(point);
  bool camera_below = r_camera < bottom_radius;
  bool point_below = r_point < bottom_radius;

  vec2 t = e1RaySphereIntersections(camera, view_ray, bottom_radius);
  Position intersection = camera + view_ray * (camera_below ? t.y : t.x);
  camera = camera_below ? intersection : camera;
  point = point_below ? intersection : point;

  return camera_below && point_below;
}

RadianceSpectrum e1GetSkyRadianceToPoint(
    const AtmosphereParameters atmosphere,
    const TransmittanceTexture e1Transmittance_texture,
    const ReducedScatteringTexture e1Scattering_texture,
    const ReducedScatteringTexture e1Single_mie_scattering_texture,
    Position camera, Position point, const Length shadow_length,
    const Direction sun_direction, out DimensionlessSpectrum transmittance) {
  // @shotamatsuda: Avoid artifacts when the ray does not intersect the top
  // atmosphere boundary.
  if (length(e1ClosestPointOnRay(camera, point)) > atmosphere.top_radius) {
    transmittance = vec3(1.0);
    return vec3(0.0);
  }

  Direction view_ray = normalize(point - camera);
  if (e1ClipAtBottomAtmosphere(atmosphere, view_ray, camera, point)) {
    transmittance = vec3(1.0);
    return vec3(0.0);
  }

  // Compute the distance to the top atmosphere boundary along the view ray,
  // assuming the viewer is in space (or NaN if the view ray does not intersect
  // the atmosphere).
  Length r = length(camera);
  Length rmu = dot(camera, view_ray);
  // @shotamatsuda: Use e1SafeSqrt instead.
  // See: https://github.com/takram-design-engineering/three-geospatial/pull/26
  Length distance_to_top_atmosphere_boundary = -rmu -
      e1SafeSqrt(rmu * rmu - r * r +
          atmosphere.top_radius * atmosphere.top_radius);
  // If the viewer is in space and the view ray intersects the atmosphere, move
  // the viewer to the top atmosphere boundary (along the view ray):
  if (distance_to_top_atmosphere_boundary > 0.0 * m) {
    camera = camera + view_ray * distance_to_top_atmosphere_boundary;
    r = atmosphere.top_radius;
    rmu += distance_to_top_atmosphere_boundary;
  }

  // Compute the r, mu, mu_s and nu parameters for the first texture lookup.
  Number mu = rmu / r;
  Number mu_s = dot(camera, sun_direction) / r;
  Number nu = dot(view_ray, sun_direction);
  Length d = length(point - camera);
  bool ray_r_mu_intersects_ground = e1RayIntersectsGround(atmosphere, r, mu);

  // @shotamatsuda: Hack to avoid rendering artifacts near the horizon, due to
  // finite atmosphere texture resolution and finite floating point precision.
  // See: https://github.com/ebruneton/precomputed_atmospheric_scattering/pull/32
  if (!ray_r_mu_intersects_ground) {
    Number mu_horizon = -e1SafeSqrt(1.0 -
        (atmosphere.bottom_radius * atmosphere.bottom_radius) / (r * r));
    const Number eps = 0.004;
    mu = max(mu, mu_horizon + eps);
  }

  transmittance = e1GetTransmittance(atmosphere, e1Transmittance_texture,
      r, mu, d, ray_r_mu_intersects_ground);

  IrradianceSpectrum single_mie_scattering;
  IrradianceSpectrum scattering = e1GetCombinedScattering(
      atmosphere, e1Scattering_texture, e1Single_mie_scattering_texture,
      r, mu, mu_s, nu, ray_r_mu_intersects_ground,
      single_mie_scattering);

  // Compute the r, mu, mu_s and nu parameters for the second texture lookup.
  // If shadow_length is not 0 (case of light shafts), we want to ignore the
  // scattering along the last shadow_length meters of the view ray, which we
  // do by subtracting shadow_length from d (this way scattering_p is equal to
  // the S|x_s=x_0-lv term in Eq. (17) of our paper).
  d = max(d - shadow_length, 0.0 * m);
  Length r_p = e1ClampRadius(atmosphere, sqrt(d * d + 2.0 * r * mu * d + r * r));
  Number mu_p = (r * mu + d) / r_p;
  Number mu_s_p = (r * mu_s + d * nu) / r_p;

  IrradianceSpectrum single_mie_scattering_p;
  IrradianceSpectrum scattering_p = e1GetCombinedScattering(
      atmosphere, e1Scattering_texture, e1Single_mie_scattering_texture,
      r_p, mu_p, mu_s_p, nu, ray_r_mu_intersects_ground,
      single_mie_scattering_p);

  // Combine the lookup results to get the scattering between camera and point.
  DimensionlessSpectrum shadow_transmittance = transmittance;
  if (shadow_length > 0.0 * m) {
    // This is the T(x,x_s) term in Eq. (17) of our paper, for light shafts.
    shadow_transmittance = e1GetTransmittance(atmosphere, e1Transmittance_texture,
        r, mu, d, ray_r_mu_intersects_ground);
  }
  // @shotamatsuda: Occlude only single Rayleigh scattering by the shadow.
#ifdef e1HAS_HIGHER_ORDER_SCATTERING_TEXTURE
  IrradianceSpectrum higher_order_scattering = e1GetScattering(
      atmosphere, e1Higher_order_scattering_texture,
      r, mu, mu_s, nu, ray_r_mu_intersects_ground);
  IrradianceSpectrum single_scattering = scattering - higher_order_scattering;
  IrradianceSpectrum higher_order_scattering_p = e1GetScattering(
      atmosphere, e1Higher_order_scattering_texture,
      r_p, mu_p, mu_s_p, nu, ray_r_mu_intersects_ground);
  IrradianceSpectrum single_scattering_p =
      scattering_p - higher_order_scattering_p;
  scattering =
      single_scattering - shadow_transmittance * single_scattering_p +
      higher_order_scattering - transmittance * higher_order_scattering_p;
#else // e1HAS_HIGHER_ORDER_SCATTERING_TEXTURE
  scattering = scattering - shadow_transmittance * scattering_p;
#endif // e1HAS_HIGHER_ORDER_SCATTERING_TEXTURE

  single_mie_scattering =
      single_mie_scattering - shadow_transmittance * single_mie_scattering_p;
#ifdef e1COMBINED_SCATTERING_TEXTURES
  single_mie_scattering = e1GetExtrapolatedSingleMieScattering(
      atmosphere, vec4(scattering, single_mie_scattering.r));
#endif // e1COMBINED_SCATTERING_TEXTURES

  // Hack to avoid rendering artifacts when the sun is below the horizon.
  single_mie_scattering = single_mie_scattering *
      smoothstep(Number(0.0), Number(0.01), mu_s);

  return scattering * e1RayleighPhaseFunction(nu) + single_mie_scattering *
      e1MiePhaseFunction(atmosphere.mie_phase_function_g, nu);
}

IrradianceSpectrum e1GetSunAndSkyIrradiance(
    const AtmosphereParameters atmosphere,
    const TransmittanceTexture e1Transmittance_texture,
    const IrradianceTexture e1Irradiance_texture,
    const Position point, const Direction normal, const Direction sun_direction,
    out IrradianceSpectrum sky_irradiance) {
  Length r = length(point);
  Number mu_s = dot(point, sun_direction) / r;

  // Indirect irradiance (approximated if the surface is not horizontal).
  sky_irradiance = e1GetIrradiance(atmosphere, e1Irradiance_texture, r, mu_s) *
      (1.0 + dot(normal, point) / r) * 0.5;

  // Direct irradiance.
  return atmosphere.solar_irradiance *
      e1GetTransmittanceToSun(
          atmosphere, e1Transmittance_texture, r, mu_s) *
      max(dot(normal, sun_direction), 0.0);
}

// @shotamatsuda: Added for the clouds.
IrradianceSpectrum e1GetSunAndSkyScalarIrradiance(
    const AtmosphereParameters atmosphere,
    const TransmittanceTexture e1Transmittance_texture,
    const IrradianceTexture e1Irradiance_texture,
    const Position point, const Direction sun_direction,
    out IrradianceSpectrum sky_irradiance) {
  Length r = length(point);
  Number mu_s = dot(point, sun_direction) / r;

  // Indirect irradiance. Integral over sphere yields 2π.
  sky_irradiance = e1GetIrradiance(atmosphere, e1Irradiance_texture, r, mu_s) *
      2.0 * PI;

  // Direct irradiance. Omit the cosine term.
  return atmosphere.solar_irradiance *
      e1GetTransmittanceToSun(atmosphere, e1Transmittance_texture, r, mu_s);
}

Luminance3 e1GetSolarLuminance() {
  return e1ATMOSPHERE.solar_irradiance /
      (PI * e1ATMOSPHERE.sun_angular_radius * e1ATMOSPHERE.sun_angular_radius) *
      e1SUN_SPECTRAL_RADIANCE_TO_LUMINANCE;
}

Luminance3 e1GetSkyLuminance(
    const Position camera, Direction view_ray, const Length shadow_length,
    const Direction sun_direction, out DimensionlessSpectrum transmittance) {
  return e1GetSkyRadiance(e1ATMOSPHERE, e1Transmittance_texture,
      e1Scattering_texture, e1Single_mie_scattering_texture,
      camera, view_ray, shadow_length, sun_direction,
      transmittance) * e1SKY_SPECTRAL_RADIANCE_TO_LUMINANCE;
}

Luminance3 e1GetSkyLuminanceToPoint(
    const Position camera, const Position point, const Length shadow_length,
    const Direction sun_direction, out DimensionlessSpectrum transmittance) {
  return e1GetSkyRadianceToPoint(e1ATMOSPHERE, e1Transmittance_texture,
      e1Scattering_texture, e1Single_mie_scattering_texture,
      camera, point, shadow_length, sun_direction, transmittance) *
      e1SKY_SPECTRAL_RADIANCE_TO_LUMINANCE;
}

Illuminance3 e1GetSunAndSkyIlluminance(
    const Position p, const Direction normal, const Direction sun_direction,
    out IrradianceSpectrum sky_irradiance) {
  IrradianceSpectrum sun_irradiance = e1GetSunAndSkyIrradiance(
      e1ATMOSPHERE, e1Transmittance_texture, e1Irradiance_texture, p, normal,
      sun_direction, sky_irradiance);
  sky_irradiance *= e1SKY_SPECTRAL_RADIANCE_TO_LUMINANCE;
  return sun_irradiance * e1SUN_SPECTRAL_RADIANCE_TO_LUMINANCE;
}

// @shotamatsuda: Added for the clouds.
Illuminance3 e1GetSunAndSkyScalarIlluminance(
    const Position p, const Direction sun_direction,
    out IrradianceSpectrum sky_irradiance) {
  IrradianceSpectrum sun_irradiance = e1GetSunAndSkyScalarIrradiance(
      e1ATMOSPHERE, e1Transmittance_texture, e1Irradiance_texture, p,
      sun_direction, sky_irradiance);
  sky_irradiance *= e1SKY_SPECTRAL_RADIANCE_TO_LUMINANCE;
  return sun_irradiance * e1SUN_SPECTRAL_RADIANCE_TO_LUMINANCE;
}

#define GetSolarRadiance e1GetSolarLuminance
#define e1GetSkyRadiance e1GetSkyLuminance
#define e1GetSkyRadianceToPoint e1GetSkyLuminanceToPoint
#define e1GetSunAndSkyIrradiance e1GetSunAndSkyIlluminance
#define e1GetSunAndSkyScalarIrradiance e1GetSunAndSkyScalarIlluminance


uniform sampler2D e1NormalBuffer;

uniform mat4 e1ProjectionMatrix;
uniform mat4 e1ViewMatrix;
uniform mat4 e1InverseProjectionMatrix;
uniform mat4 e1InverseViewMatrix;
uniform float e1BottomRadius;
uniform mat4 e1WorldToECEFMatrix;
uniform float e1GeometricErrorCorrectionAmount;
uniform vec3 e1SunDirection;
uniform float e1CosSunAngularRadius;
uniform vec3 e1MoonDirection;
uniform float e1MoonAngularRadius;
uniform float e1LunarRadianceScale;
uniform float e1AlbedoScale;

vec3 e1GetLunarRadiance(const float e1MoonAngularRadius) {
  // Not a physical number but the order of 10^-6 relative to the sun may fit.
  vec3 radiance =
    e1ATMOSPHERE.solar_irradiance *
    0.000002 /
    (PI * e1MoonAngularRadius * e1MoonAngularRadius) *
    e1SUN_SPECTRAL_RADIANCE_TO_LUMINANCE;
  return radiance;
}

float e1IntersectSphere(const vec3 ray, const vec3 point, const float radius) {
  vec3 P = -point;
  float PoR = dot(P, ray);
  float D = dot(P, P) - radius * radius;
  return -PoR - sqrt(PoR * PoR - D);
}

float e1OrenNayarDiffuse(const vec3 L, const vec3 V, const vec3 N) {
  float NoL = dot(N, L);
  float NoV = dot(N, V);
  float s = dot(L, V) - NoL * NoV;
  float t = mix(1.0, max(NoL, NoV), step(0.0, s));
  return max(0.0, NoL) * (0.62406015 + 0.41284404 * s / t);
}

vec3 e1GetSkyRadiance(
  const vec3 e1CameraPosition,
  const vec3 rayDirection,
  const float shadowLength,
  const vec3 e1SunDirection,
  const vec3 e1MoonDirection,
  const float e1MoonAngularRadius,
  const float e1LunarRadianceScale,
  const float fragmentAngle
) {
  vec3 transmittance;
  vec3 radiance = e1GetSkyRadiance(
    e1CameraPosition,
    rayDirection,
    shadowLength,
    e1SunDirection,
    transmittance
  );

  // Rendering celestial objects without perspective doesn't make sense.
  #ifdef PERSPECTIVE_CAMERA

  #ifdef e1SUN
  float viewDotSun = dot(rayDirection, e1SunDirection);
  if (viewDotSun > e1CosSunAngularRadius) {
    float angle = acos(clamp(viewDotSun, -1.0, 1.0));
    float antialias = smoothstep(
      e1ATMOSPHERE.sun_angular_radius,
      e1ATMOSPHERE.sun_angular_radius - fragmentAngle,
      angle
    );
    radiance += transmittance * GetSolarRadiance() * antialias;
  }
  #endif // e1SUN

  #ifdef e1MOON
  float intersection = e1IntersectSphere(rayDirection, e1MoonDirection, e1MoonAngularRadius);
  if (intersection > 0.0) {
    vec3 normal = normalize(e1MoonDirection - rayDirection * intersection);
    float diffuse = e1OrenNayarDiffuse(-e1SunDirection, rayDirection, normal);
    float viewDotMoon = dot(rayDirection, e1MoonDirection);
    float angle = acos(clamp(viewDotMoon, -1.0, 1.0));
    float antialias = smoothstep(e1MoonAngularRadius, e1MoonAngularRadius - fragmentAngle, angle);
    radiance +=
      transmittance *
      e1GetLunarRadiance(e1MoonAngularRadius) *
      e1LunarRadianceScale *
      diffuse *
      antialias;
  }
  #endif // e1MOON

  #endif // PERSPECTIVE_CAMERA

  return radiance;
}


#ifdef HAS_LIGHTING_MASK
uniform sampler2D e1LightingMaskBuffer;
#endif // HAS_LIGHTING_MASK

// prettier-ignore
#define LIGHTING_MASK_CHANNEL_ LIGHTING_MASK_CHANNEL

#ifdef e1HAS_OVERLAY
uniform sampler2D e1OverlayBuffer;
#endif // e1HAS_OVERLAY

#ifdef e1HAS_SHADOW
uniform sampler2DArray e1ShadowBuffer;
uniform vec2 e1ShadowIntervals[e1SHADOW_CASCADE_COUNT];
uniform mat4 e1ShadowMatrices[e1SHADOW_CASCADE_COUNT];
uniform mat4 e1InverseShadowMatrices[e1SHADOW_CASCADE_COUNT];
uniform float e1ShadowFar;
uniform float e1ShadowTopHeight;
uniform float e1ShadowRadius;
uniform sampler3D e1StbnTexture;
uniform int e1Frame;
#endif // e1HAS_SHADOW

#ifdef e1HAS_SHADOW_LENGTH
uniform sampler2D e1ShadowLengthBuffer;
#endif // e1HAS_SHADOW_LENGTH

varying vec3 e1VCameraPosition;
varying vec3 e1VRayDirection;
varying vec3 e1VGeometryAltitudeCorrection;
varying vec3 e1VEllipsoidRadiiSquared;

vec3 e1ReadNormal(const vec2 uv, out bool degenerate) {
  vec3 normal = texture(e1NormalBuffer, uv).xyz;
  degenerate = normal == vec3(0.0);
  #ifdef OCT_ENCODED_NORMAL
  return e1UnpackVec2ToNormal(normal.xy);
  #else // OCT_ENCODED_NORMAL
  return 2.0 * normal - 1.0;
  #endif // OCT_ENCODED_NORMAL
}

void e1CorrectGeometricError(inout vec3 positionECEF, inout vec3 normalECEF) {
  // TODO: The error is pronounced at the edge of the ellipsoid due to the
  // large difference between the sphere position and the unprojected position
  // at the current fragment. Calculating the sphere position from the fragment
  // UV may resolve this.

  // Correct way is slerp, but this will be small-angle interpolation anyways.
  vec3 sphereNormal = normalize(positionECEF / e1VEllipsoidRadiiSquared);
  vec3 spherePosition = e1ATMOSPHERE.bottom_radius * sphereNormal;
  normalECEF = mix(normalECEF, sphereNormal, e1GeometricErrorCorrectionAmount);
  positionECEF = mix(positionECEF, spherePosition, e1GeometricErrorCorrectionAmount);
}

#if defined(e1SUN_LIGHT) || defined(e1SKY_LIGHT)

vec3 e1GetSunSkyIrradiance(
  const vec3 positionECEF,
  const vec3 normal,
  const vec3 inputColor,
  const float sunTransmittance
) {
  // Assume lambertian BRDF. If both e1SUN_LIGHT and e1SKY_LIGHT are not defined,
  // regard the inputColor as radiance at the texel.
  vec3 diffuse = inputColor * e1AlbedoScale * RECIPROCAL_PI;
  vec3 skyIrradiance;
  vec3 sunIrradiance = e1GetSunAndSkyIrradiance(positionECEF, normal, e1SunDirection, skyIrradiance);

  #ifdef e1HAS_SHADOW
  sunIrradiance *= sunTransmittance;
  #endif // e1HAS_SHADOW

  #if defined(e1SUN_LIGHT) && defined(e1SKY_LIGHT)
  return diffuse * (sunIrradiance + skyIrradiance);
  #elif defined(e1SUN_LIGHT)
  return diffuse * sunIrradiance;
  #elif defined(e1SKY_LIGHT)
  return diffuse * skyIrradiance;
  #endif // defined(e1SUN_LIGHT) && defined(e1SKY_LIGHT)
}

#endif // defined(e1SUN_LIGHT) || defined(e1SKY_LIGHT)

#if defined(e1TRANSMITTANCE) || defined(e1INSCATTER)

void e1ApplyTransmittanceInscatter(const vec3 positionECEF, float shadowLength, inout vec3 radiance) {
  vec3 transmittance;
  vec3 inscatter = e1GetSkyRadianceToPoint(
    e1VCameraPosition,
    positionECEF,
    shadowLength,
    e1SunDirection,
    transmittance
  );
  #ifdef e1TRANSMITTANCE
  radiance = radiance * transmittance;
  #endif // e1TRANSMITTANCE
  #ifdef e1INSCATTER
  radiance = radiance + inscatter;
  #endif // e1INSCATTER
}

#endif // defined(e1TRANSMITTANCE) || defined(e1INSCATTER)

#ifdef e1HAS_SHADOW

float e1GetSTBN() {
  ivec3 size = textureSize(e1StbnTexture, 0);
  vec3 scale = 1.0 / vec3(size);
  return texture(e1StbnTexture, vec3(gl_FragCoord.xy, float(e1Frame % size.z)) * scale).r;
}

vec2 e1GetShadowUv(const vec3 worldPosition, const int cascadeIndex) {
  vec4 clip = e1ShadowMatrices[cascadeIndex] * vec4(worldPosition, 1.0);
  clip /= clip.w;
  return clip.xy * 0.5 + 0.5;
}

float e1GetDistanceToShadowTop(const vec3 positionECEF) {
  // Distance to the top of the shadows along the sun direction, which matches
  // the ray origin of BSM.
  return e1RaySphereSecondIntersection(
    positionECEF / e1METER_TO_LENGTH_UNIT, // TODO: Make units consistent
    e1SunDirection,
    vec3(0.0),
    e1BottomRadius + e1ShadowTopHeight
  );
}

float e1ReadShadowOpticalDepth(const vec2 uv, const float distanceToTop, const int cascadeIndex) {
  // r: frontDepth, g: meanExtinction, b: maxOpticalDepth, a: maxOpticalDepthTail
  vec4 shadow = texture(e1ShadowBuffer, vec3(uv, float(cascadeIndex)));
  // Omit adding maxOpticalDepthTail to avoid pronounced aliasing. Ground
  // shadow will be attenuated by inscatter anyways.
  return min(shadow.b, shadow.g * max(0.0, distanceToTop - shadow.r));
}

float e1SampleShadowOpticalDepthPCF(
  const vec3 worldPosition,
  const float distanceToTop,
  const float radius,
  const int cascadeIndex
) {
  vec2 uv = e1GetShadowUv(worldPosition, cascadeIndex);
  if (uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0) {
    return 0.0;
  }

  vec2 texelSize = vec2(1.0) / vec2(textureSize(e1ShadowBuffer, 0).xy);
  float sum = 0.0;
  vec2 offset;
  
    #if 0 < e1SHADOW_SAMPLE_COUNT
    offset = e1VogelDisk(
      0,
      e1SHADOW_SAMPLE_COUNT,
      e1InterleavedGradientNoise(gl_FragCoord.xy) * PI2
    );
    sum += e1ReadShadowOpticalDepth(uv + offset * radius * texelSize, distanceToTop, cascadeIndex);
    #endif // 0 < e1SHADOW_SAMPLE_COUNT
  
    #if 1 < e1SHADOW_SAMPLE_COUNT
    offset = e1VogelDisk(
      1,
      e1SHADOW_SAMPLE_COUNT,
      e1InterleavedGradientNoise(gl_FragCoord.xy) * PI2
    );
    sum += e1ReadShadowOpticalDepth(uv + offset * radius * texelSize, distanceToTop, cascadeIndex);
    #endif // 1 < e1SHADOW_SAMPLE_COUNT
  
    #if 2 < e1SHADOW_SAMPLE_COUNT
    offset = e1VogelDisk(
      2,
      e1SHADOW_SAMPLE_COUNT,
      e1InterleavedGradientNoise(gl_FragCoord.xy) * PI2
    );
    sum += e1ReadShadowOpticalDepth(uv + offset * radius * texelSize, distanceToTop, cascadeIndex);
    #endif // 2 < e1SHADOW_SAMPLE_COUNT
  
    #if 3 < e1SHADOW_SAMPLE_COUNT
    offset = e1VogelDisk(
      3,
      e1SHADOW_SAMPLE_COUNT,
      e1InterleavedGradientNoise(gl_FragCoord.xy) * PI2
    );
    sum += e1ReadShadowOpticalDepth(uv + offset * radius * texelSize, distanceToTop, cascadeIndex);
    #endif // 3 < e1SHADOW_SAMPLE_COUNT
  
    #if 4 < e1SHADOW_SAMPLE_COUNT
    offset = e1VogelDisk(
      4,
      e1SHADOW_SAMPLE_COUNT,
      e1InterleavedGradientNoise(gl_FragCoord.xy) * PI2
    );
    sum += e1ReadShadowOpticalDepth(uv + offset * radius * texelSize, distanceToTop, cascadeIndex);
    #endif // 4 < e1SHADOW_SAMPLE_COUNT
  
    #if 5 < e1SHADOW_SAMPLE_COUNT
    offset = e1VogelDisk(
      5,
      e1SHADOW_SAMPLE_COUNT,
      e1InterleavedGradientNoise(gl_FragCoord.xy) * PI2
    );
    sum += e1ReadShadowOpticalDepth(uv + offset * radius * texelSize, distanceToTop, cascadeIndex);
    #endif // 5 < e1SHADOW_SAMPLE_COUNT
  
    #if 6 < e1SHADOW_SAMPLE_COUNT
    offset = e1VogelDisk(
      6,
      e1SHADOW_SAMPLE_COUNT,
      e1InterleavedGradientNoise(gl_FragCoord.xy) * PI2
    );
    sum += e1ReadShadowOpticalDepth(uv + offset * radius * texelSize, distanceToTop, cascadeIndex);
    #endif // 6 < e1SHADOW_SAMPLE_COUNT
  
    #if 7 < e1SHADOW_SAMPLE_COUNT
    offset = e1VogelDisk(
      7,
      e1SHADOW_SAMPLE_COUNT,
      e1InterleavedGradientNoise(gl_FragCoord.xy) * PI2
    );
    sum += e1ReadShadowOpticalDepth(uv + offset * radius * texelSize, distanceToTop, cascadeIndex);
    #endif // 7 < e1SHADOW_SAMPLE_COUNT
  
    #if 8 < e1SHADOW_SAMPLE_COUNT
    offset = e1VogelDisk(
      8,
      e1SHADOW_SAMPLE_COUNT,
      e1InterleavedGradientNoise(gl_FragCoord.xy) * PI2
    );
    sum += e1ReadShadowOpticalDepth(uv + offset * radius * texelSize, distanceToTop, cascadeIndex);
    #endif // 8 < e1SHADOW_SAMPLE_COUNT
  
    #if 9 < e1SHADOW_SAMPLE_COUNT
    offset = e1VogelDisk(
      9,
      e1SHADOW_SAMPLE_COUNT,
      e1InterleavedGradientNoise(gl_FragCoord.xy) * PI2
    );
    sum += e1ReadShadowOpticalDepth(uv + offset * radius * texelSize, distanceToTop, cascadeIndex);
    #endif // 9 < e1SHADOW_SAMPLE_COUNT
  
    #if 10 < e1SHADOW_SAMPLE_COUNT
    offset = e1VogelDisk(
      10,
      e1SHADOW_SAMPLE_COUNT,
      e1InterleavedGradientNoise(gl_FragCoord.xy) * PI2
    );
    sum += e1ReadShadowOpticalDepth(uv + offset * radius * texelSize, distanceToTop, cascadeIndex);
    #endif // 10 < e1SHADOW_SAMPLE_COUNT
  
    #if 11 < e1SHADOW_SAMPLE_COUNT
    offset = e1VogelDisk(
      11,
      e1SHADOW_SAMPLE_COUNT,
      e1InterleavedGradientNoise(gl_FragCoord.xy) * PI2
    );
    sum += e1ReadShadowOpticalDepth(uv + offset * radius * texelSize, distanceToTop, cascadeIndex);
    #endif // 11 < e1SHADOW_SAMPLE_COUNT
  
    #if 12 < e1SHADOW_SAMPLE_COUNT
    offset = e1VogelDisk(
      12,
      e1SHADOW_SAMPLE_COUNT,
      e1InterleavedGradientNoise(gl_FragCoord.xy) * PI2
    );
    sum += e1ReadShadowOpticalDepth(uv + offset * radius * texelSize, distanceToTop, cascadeIndex);
    #endif // 12 < e1SHADOW_SAMPLE_COUNT
  
    #if 13 < e1SHADOW_SAMPLE_COUNT
    offset = e1VogelDisk(
      13,
      e1SHADOW_SAMPLE_COUNT,
      e1InterleavedGradientNoise(gl_FragCoord.xy) * PI2
    );
    sum += e1ReadShadowOpticalDepth(uv + offset * radius * texelSize, distanceToTop, cascadeIndex);
    #endif // 13 < e1SHADOW_SAMPLE_COUNT
  
    #if 14 < e1SHADOW_SAMPLE_COUNT
    offset = e1VogelDisk(
      14,
      e1SHADOW_SAMPLE_COUNT,
      e1InterleavedGradientNoise(gl_FragCoord.xy) * PI2
    );
    sum += e1ReadShadowOpticalDepth(uv + offset * radius * texelSize, distanceToTop, cascadeIndex);
    #endif // 14 < e1SHADOW_SAMPLE_COUNT
  
    #if 15 < e1SHADOW_SAMPLE_COUNT
    offset = e1VogelDisk(
      15,
      e1SHADOW_SAMPLE_COUNT,
      e1InterleavedGradientNoise(gl_FragCoord.xy) * PI2
    );
    sum += e1ReadShadowOpticalDepth(uv + offset * radius * texelSize, distanceToTop, cascadeIndex);
    #endif // 15 < e1SHADOW_SAMPLE_COUNT
  
  return sum / float(e1SHADOW_SAMPLE_COUNT);
}

float e1SampleShadowOpticalDepth(
  const vec3 worldPosition,
  const vec3 positionECEF,
  const float radius,
  const float jitter
) {
  float distanceToTop = e1GetDistanceToShadowTop(positionECEF);
  if (distanceToTop <= 0.0) {
    return 0.0;
  }
  int cascadeIndex = getFadedCascadeIndex(
    e1ViewMatrix,
    worldPosition,
    e1ShadowIntervals,
    cameraNear,
    e1ShadowFar,
    jitter
  );
  return cascadeIndex >= 0
    ? e1SampleShadowOpticalDepthPCF(worldPosition, distanceToTop, radius, cascadeIndex)
    : 0.0;
}

float e1GetShadowRadius(const vec3 worldPosition) {
  vec4 clip = e1ShadowMatrices[0] * vec4(worldPosition, 1.0);
  clip /= clip.w;

  // Offset by 1px in each direction in shadow's clip coordinates.
  vec2 shadowSize = vec2(textureSize(e1ShadowBuffer, 0));
  vec3 offset = vec3(2.0 / shadowSize, 0.0);
  vec4 clipX = clip + offset.xzzz;
  vec4 clipY = clip + offset.zyzz;

  // Convert back to world space.
  vec4 worldX = e1InverseShadowMatrices[0] * clipX;
  vec4 worldY = e1InverseShadowMatrices[0] * clipY;

  // Project into the main camera's clip space.
  mat4 viewProjectionMatrix = e1ProjectionMatrix * e1ViewMatrix;
  vec4 projected = viewProjectionMatrix * vec4(worldPosition, 1.0);
  vec4 projectedX = viewProjectionMatrix * worldX;
  vec4 projectedY = viewProjectionMatrix * worldY;
  projected /= projected.w;
  projectedX /= projectedX.w;
  projectedY /= projectedY.w;

  // Take the mean of pixel sizes.
  vec2 center = (projected.xy * 0.5 + 0.5) * resolution;
  vec2 offsetX = (projectedX.xy * 0.5 + 0.5) * resolution;
  vec2 offsetY = (projectedY.xy * 0.5 + 0.5) * resolution;
  float size = max(length(offsetX - center), length(offsetY - center));

  return e1RemapClamped(size, 10.0, 50.0, 0.0, e1ShadowRadius);
}

#endif // e1HAS_SHADOW

void e1MainImage(const vec4 inputColor, const vec2 uv, out vec4 outputColor) {
  #if defined(HAS_LIGHTING_MASK) && defined(DEBUG_SHOW_LIGHTING_MASK)
  outputColor.rgb = vec3(texture(e1LightingMaskBuffer, uv).LIGHTING_MASK_CHANNEL_);
  outputColor.a = 1.0;
  return;
  #endif // defined(HAS_LIGHTING_MASK) && defined(DEBUG_SHOW_LIGHTING_MASK)

  float shadowLength = 0.0;
  #ifdef e1HAS_SHADOW_LENGTH
  shadowLength = texture(e1ShadowLengthBuffer, uv).r;
  #endif // e1HAS_SHADOW_LENGTH

  #ifdef e1HAS_OVERLAY
  vec4 overlay = texture(e1OverlayBuffer, uv);
  if (overlay.a == 1.0) {
    outputColor = overlay;
    return;
  }
  #endif // e1HAS_OVERLAY

  vec3 rayDirection = normalize(e1VRayDirection);
  vec3 dRDdx = dFdx(rayDirection);
  vec3 dRDdy = dFdy(rayDirection);
  float fragmentAngle = length(dRDdx + dRDdy) / length(rayDirection);

  float depth = e1ReadDepthValue(depthBuffer, uv);
  if (depth >= 1.0 - 1e-8) {
    #ifdef e1SKY
    outputColor.rgb = e1GetSkyRadiance(
      e1VCameraPosition,
      rayDirection,
      shadowLength,
      e1SunDirection,
      e1MoonDirection,
      e1MoonAngularRadius,
      e1LunarRadianceScale,
      fragmentAngle
    );
    outputColor.a = 1.0;
    #else // e1SKY
    outputColor = inputColor;
    #endif // e1SKY

    #ifdef e1HAS_OVERLAY
    outputColor.rgb = outputColor.rgb * (1.0 - overlay.a) + overlay.rgb;
    #endif // e1HAS_OVERLAY
    return;
  }
  depth = e1ReverseLogDepth(depth, cameraNear, cameraFar);

  // Reconstruct position and normal in world space.
  vec3 viewPosition = e1ScreenToView(
    uv,
    depth,
    getViewZ(depth),
    e1ProjectionMatrix,
    e1InverseProjectionMatrix
  );
  vec3 worldPosition = (e1InverseViewMatrix * vec4(viewPosition, 1.0)).xyz;
  vec3 positionECEF = (e1WorldToECEFMatrix * vec4(worldPosition, 1.0)).xyz;
  positionECEF = positionECEF * e1METER_TO_LENGTH_UNIT + e1VGeometryAltitudeCorrection;

  vec3 viewNormal;
  bool degenerateNormal = false;
  #ifdef RECONSTRUCT_NORMAL
  vec3 dVPdx = dFdx(viewPosition);
  vec3 dVPdy = dFdy(viewPosition);
  viewNormal = normalize(cross(dVPdx, dVPdy));
  #elif defined(e1HAS_NORMALS)
  viewNormal = e1ReadNormal(uv, degenerateNormal);
  #endif // defined(e1HAS_NORMALS)

  #if defined(RECONSTRUCT_NORMAL) || defined(e1HAS_NORMALS)
  vec3 worldNormal = (e1InverseViewMatrix * vec4(viewNormal, 0.0)).xyz;
  vec3 normalECEF = (e1WorldToECEFMatrix * vec4(worldNormal, 0.0)).xyz;
  #else // defined(RECONSTRUCT_NORMAL) || defined(e1HAS_NORMALS)
  vec3 normalECEF = normalize(positionECEF);
  #endif // defined(RECONSTRUCT_NORMAL) || defined(e1HAS_NORMALS)

  #ifdef e1CORRECT_GEOMETRIC_ERROR
  e1CorrectGeometricError(positionECEF, normalECEF);
  #endif // e1CORRECT_GEOMETRIC_ERROR

  #ifdef e1HAS_SHADOW
  float stbn = e1GetSTBN();
  float radius = e1GetShadowRadius(worldPosition);
  float opticalDepth = e1SampleShadowOpticalDepth(worldPosition, positionECEF, radius, stbn);
  float sunTransmittance = exp(-opticalDepth);
  #else // e1HAS_SHADOW
  float sunTransmittance = 1.0;
  #endif // e1HAS_SHADOW

  vec3 radiance;
  #if defined(e1SUN_LIGHT) || defined(e1SKY_LIGHT)
  // WORKAROUND: When both post-process lighting and sky options are enabled,
  // stars have degenerate normals. We use this to disable irradiance, which is
  // irrelevant for them.
  if (!degenerateNormal) {
    radiance = e1GetSunSkyIrradiance(positionECEF, normalECEF, inputColor.rgb, sunTransmittance);
  } else {
    radiance = inputColor.rgb;
  }
  #ifdef HAS_LIGHTING_MASK
  float lightingMask = texture(e1LightingMaskBuffer, uv).LIGHTING_MASK_CHANNEL_;
  radiance = mix(inputColor.rgb, radiance, lightingMask);
  #endif // HAS_LIGHTING_MASK
  #else // defined(e1SUN_LIGHT) || defined(e1SKY_LIGHT)
  radiance = inputColor.rgb;
  #endif // defined(e1SUN_LIGHT) || defined(e1SKY_LIGHT)

  #if defined(e1TRANSMITTANCE) || defined(e1INSCATTER)
  e1ApplyTransmittanceInscatter(positionECEF, shadowLength, radiance);
  #endif // defined(e1TRANSMITTANCE) || defined(e1INSCATTER)

  outputColor = vec4(radiance, inputColor.a);

  #ifdef e1HAS_OVERLAY
  outputColor.rgb = outputColor.rgb * (1.0 - overlay.a) + overlay.rgb;
  #endif // e1HAS_OVERLAY
}

vec4 blend23(const in vec4 dst,const in vec4 src,const in float opacity){return mix(dst,src,opacity);} void main(){ vec4 color0=texture2D(inputBuffer,UV);vec4 color1=vec4(0.0);e0MainImage(color0, UV, color1);
	color0 = blend23(color0, color1, e0BlendOpacity);

	e1MainImage(color0, UV, color1);
	color0 = blend23(color0, color1, e1BlendOpacity); color0.a=clamp(color0.a,0.0,1.0);gl_FragColor=color0;
#ifdef ENCODE_OUTPUT

gl_FragColor = linearToOutputTexel( gl_FragColor );

#endif

#ifdef DITHERING

	gl_FragColor.rgb = dithering( gl_FragColor.rgb );

#endif

}