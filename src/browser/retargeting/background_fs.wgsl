// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms

struct renderStruct {
	nodeUniform0 : f32,
	nodeUniform4 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform1 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform3 : mat3x3<f32>,
	nodeUniform5 : f32,
	nodeUniform7 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> normalWorldGeometry : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;

// codes
fn lightSpeed ( suv : vec2<f32> ) -> vec3<f32> {

	var nodeVar0 : vec2<f32>;

	nodeVar0 = vec2<f32>( length( suv ), atan2( suv.y, suv.x ) );

	return ( ( ( ( vec3<f32>( ( ( sin( ( ( nodeVar0.y * 150.0 ) + render.nodeUniform0 ) ) * 0.5 ) + 0.5 ) ) * vec3<f32>( ( ( sin( ( ( nodeVar0.y * 80.0 ) - ( render.nodeUniform0 * 0.6 ) ) ) * 0.5 ) + 0.5 ) ) ) * vec3<f32>( ( ( sin( ( ( nodeVar0.y * 45.0 ) + ( render.nodeUniform0 * 0.8 ) ) ) * 0.5 ) + 0.5 ) ) ) * vec3<f32>( ( 1.0 - cos( ( nodeVar0.y + ( ( 22.0 * render.nodeUniform0 ) - ( pow( ( nodeVar0.x + ( ( ( 0.1 * sin( ( ( nodeVar0.y * 10.0 ) - ( render.nodeUniform0 * 0.6 ) ) ) ) * cos( ( ( nodeVar0.y * 48.0 ) + ( render.nodeUniform0 * 0.3 ) ) ) ) * cos( ( ( nodeVar0.y * 3.7 ) + render.nodeUniform0 ) ) ) ), 0.3 ) * 60.0 ) ) ) ) ) ) ) * vec3<f32>( ( nodeVar0.x * 2.0 ) ) );

}


fn blendDodge ( base : vec3<f32>, blend : vec3<f32> ) -> vec3<f32> {

	


	return min( ( base / ( vec3<f32>( 1.0 ) - blend ) ), vec3<f32>( 1.0 ) );

}




@fragment
fn main( @location( 0 ) v_normalWorldGeometry : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = ( render.nodeUniform0 * 0.1 );
	nodeVar1 = cos( nodeVar0 );
	nodeVar2 = ( render.nodeUniform0 * 0.5 );
	nodeVar3 = cos( nodeVar2 );
	normalWorldGeometry = normalize( v_normalWorldGeometry );
	DiffuseColor = ( vec4<f32>( blendDodge( mix( max( ( ( vec3<f32>( 0.00030352698352941176, 0.17788841597328695, 0.4178850708380236 ) * vec3<f32>( nodeVar1 ) ) + ( ( cross( vec3<f32>( 0.57735, 0.57735, 0.57735 ), vec3<f32>( 0.00030352698352941176, 0.17788841597328695, 0.4178850708380236 ) ) * vec3<f32>( sin( nodeVar0 ) ) ) + ( vec3<f32>( 0.57735, 0.57735, 0.57735 ) * vec3<f32>( ( dot( vec3<f32>( 0.57735, 0.57735, 0.57735 ), vec3<f32>( 0.00030352698352941176, 0.17788841597328695, 0.4178850708380236 ) ) * ( 1.0 - nodeVar1 ) ) ) ) ) ), vec3<f32>( 0.0 ) ), max( ( ( vec3<f32>( 0.0006070539670588235, 0.02028856305209031, 0.07818742179702069 ) * vec3<f32>( nodeVar3 ) ) + ( ( cross( vec3<f32>( 0.57735, 0.57735, 0.57735 ), vec3<f32>( 0.0006070539670588235, 0.02028856305209031, 0.07818742179702069 ) ) * vec3<f32>( sin( nodeVar2 ) ) ) + ( vec3<f32>( 0.57735, 0.57735, 0.57735 ) * vec3<f32>( ( dot( vec3<f32>( 0.57735, 0.57735, 0.57735 ), vec3<f32>( 0.0006070539670588235, 0.02028856305209031, 0.07818742179702069 ) ) * ( 1.0 - nodeVar3 ) ) ) ) ) ), vec3<f32>( 0.0 ) ), distance( ( fragCoord.xy / render.nodeUniform1 ), vec2<f32>( 0.5 ) ) ), mix( vec3<f32>( 0.0 ), clamp( lightSpeed( normalWorldGeometry.xy ), vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) ), ( ( clamp( ( ( normalWorldGeometry.y - -0.1 ) / ( 1.0 - -0.1 ) ), 0.0, 1.0 ) * ( 1.0 - 0.0 ) ) + 0.0 ) ) ), 1.0 ) * vec4<f32>( render.nodeUniform4 ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform5 );
	DiffuseColor.w = 1.0;
	nodeVar4 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar4;

	// result

	output.color = nodeVar4;

	return output;

}
