#version 300 es

#define attribute in
#define varying out
#define texture2D texture
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
#define THREE_REVISION 187
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
uniform mat4 modelMatrix;
uniform mat4 modelViewMatrix;
uniform mat4 projectionMatrix;
uniform mat4 viewMatrix;
uniform mat3 normalMatrix;
uniform vec3 cameraPosition;
uniform bool isOrthographic;
#ifdef USE_INSTANCING
	attribute mat4 instanceMatrix;
#endif
#ifdef USE_INSTANCING_COLOR
	attribute vec3 instanceColor;
#endif
#ifdef USE_INSTANCING_MORPH
	uniform sampler2D morphTexture;
#endif
attribute vec3 position;
attribute vec3 normal;
attribute vec2 uv;
#ifdef USE_UV1
	attribute vec2 uv1;
#endif
#ifdef USE_UV2
	attribute vec2 uv2;
#endif
#ifdef USE_UV3
	attribute vec2 uv3;
#endif
#ifdef USE_TANGENT
	attribute vec4 tangent;
#endif
#if defined( USE_COLOR_ALPHA )
	attribute vec4 color;
#elif defined( USE_COLOR )
	attribute vec3 color;
#endif
#ifdef USE_SKINNING
	attribute vec4 skinIndex;
	attribute vec4 skinWeight;
#endif

uniform vec2 resolution;uniform vec2 texelSize;uniform float cameraNear;uniform float cameraFar;uniform float aspect;uniform float time;varying vec2 vUv;uniform mat4 e1InverseViewMatrix;
uniform mat4 e1InverseProjectionMatrix;
uniform vec3 e1CameraPosition;
uniform mat4 e1WorldToECEFMatrix;
uniform vec3 e1AltitudeCorrection;
uniform float e1GeometricErrorCorrectionAmount;
uniform vec3 e1EllipsoidRadii;

varying vec3 e1VCameraPosition;
varying vec3 e1VRayDirection;
varying vec3 e1VGeometryAltitudeCorrection;
varying vec3 e1VEllipsoidRadiiSquared;

void e1GetCameraRay(out vec3 origin, out vec3 direction) {
  bool isPerspective = e1InverseProjectionMatrix[2][3] != 0.0; // 4th entry in the 3rd column

  if (isPerspective) {
    // Calculate the camera ray for a perspective camera.
    vec4 viewPosition = e1InverseProjectionMatrix * vec4(position, 1.0);
    vec4 worldDirection = e1InverseViewMatrix * vec4(viewPosition.xyz, 0.0);
    origin = e1CameraPosition;
    direction = worldDirection.xyz;
  } else {
    // Unprojected points to calculate direction.
    vec4 nearPoint = e1InverseProjectionMatrix * vec4(position.xy, -1.0, 1.0);
    vec4 farPoint = e1InverseProjectionMatrix * vec4(position.xy, -0.9, 1.0);
    nearPoint /= nearPoint.w;
    farPoint /= farPoint.w;

    // Calculate world values.
    vec4 worldDirection = e1InverseViewMatrix * vec4(farPoint.xyz - nearPoint.xyz, 0.0);
    vec4 worldOrigin = e1InverseViewMatrix * nearPoint;

    // Outputs
    direction = worldDirection.xyz;
    origin = worldOrigin.xyz;
  }
}

void e1MainSupport() {
  vec3 direction, origin;
  e1GetCameraRay(origin, direction);

  vec3 cameraPositionECEF = (e1WorldToECEFMatrix * vec4(origin, 1.0)).xyz;
  e1VCameraPosition = (cameraPositionECEF + e1AltitudeCorrection) * e1METER_TO_LENGTH_UNIT;
  e1VRayDirection = (e1WorldToECEFMatrix * vec4(direction, 0.0)).xyz;

  e1VGeometryAltitudeCorrection = e1AltitudeCorrection * e1METER_TO_LENGTH_UNIT;
  // Gradually turn off the altitude correction on geometries as the geometric
  // error correction takes effect, because that on the ideal sphere will be
  // over corrected.
  // See: https://github.com/takram-design-engineering/three-geospatial/pull/23#issuecomment-2542914656
  #ifdef e1CORRECT_GEOMETRIC_ERROR
  e1VGeometryAltitudeCorrection *= 1.0 - e1GeometricErrorCorrectionAmount;
  #endif // e1CORRECT_GEOMETRIC_ERROR

  vec3 radii = e1EllipsoidRadii * e1METER_TO_LENGTH_UNIT;
  e1VEllipsoidRadiiSquared = radii * radii;
} void main(){vUv=position.xy*0.5+0.5;e1MainSupport(); gl_Position=vec4(position.xy,1.0,1.0);}