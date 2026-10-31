// Three.js r187 - Node System

// global
diagnostic( off, derivative_uniformity );


// directives


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
var<private> normalWorldGeometry : vec3<f32>;
var<private> Output : vec4<f32>;

// codes
fn fn34 ( color : vec3<f32>, adjustment : f32 ) -> vec3<f32> {

	

	let nodeConst0 = cos( adjustment );

	return max( ( ( color * vec3<f32>( nodeConst0 ) ) + ( ( cross( vec3<f32>( 0.57735, 0.57735, 0.57735 ), color ) * vec3<f32>( sin( adjustment ) ) ) + ( vec3<f32>( 0.57735, 0.57735, 0.57735 ) * vec3<f32>( ( dot( vec3<f32>( 0.57735, 0.57735, 0.57735 ), color ) * ( 1.0 - nodeConst0 ) ) ) ) ) ), vec3<f32>( 0.0 ) );

}


fn fn130 ( suv : vec2<f32>, time : f32 ) -> vec3<f32> {

	

	let nodeConst0 = suv;
	let nodeConst1 = vec2<f32>( length( nodeConst0 ), atan2( nodeConst0.y, nodeConst0.x ) );

	return ( ( ( ( vec3<f32>( ( ( sin( ( ( nodeConst1.y * 150.0 ) + time ) ) * 0.5 ) + 0.5 ) ) * vec3<f32>( ( ( sin( ( ( nodeConst1.y * 80.0 ) - ( time * 0.6 ) ) ) * 0.5 ) + 0.5 ) ) ) * vec3<f32>( ( ( sin( ( ( nodeConst1.y * 45.0 ) + ( time * 0.8 ) ) ) * 0.5 ) + 0.5 ) ) ) * vec3<f32>( ( 1.0 - cos( ( nodeConst1.y + ( ( 22.0 * time ) - ( pow( ( nodeConst1.x + ( ( ( 0.1 * sin( ( ( nodeConst1.y * 10.0 ) - ( time * 0.6 ) ) ) ) * cos( ( ( nodeConst1.y * 48.0 ) + ( time * 0.3 ) ) ) ) * cos( ( ( nodeConst1.y * 3.7 ) + time ) ) ) ), 0.3 ) * 60.0 ) ) ) ) ) ) ) * vec3<f32>( ( nodeConst1.x * 2.0 ) ) );

}


fn fn27 ( base : vec3<f32>, blend : vec3<f32> ) -> vec3<f32> {

	


	return min( ( base / ( vec3<f32>( 1.0 ) - blend ) ), vec3<f32>( 1.0 ) );

}




@fragment
fn main( @location( 0 ) v_normalWorldGeometry : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	normalWorldGeometry = normalize( v_normalWorldGeometry );
	DiffuseColor = ( vec4<f32>( fn27( mix( fn34( vec3<f32>( 0.00030352698352941176, 0.17788841597328695, 0.4178850708380236 ), ( render.nodeUniform0 * 0.1 ) ), fn34( vec3<f32>( 0.0006070539670588235, 0.02028856305209031, 0.07818742179702069 ), ( render.nodeUniform0 * 0.5 ) ), distance( ( fragCoord.xy / render.nodeUniform1 ), vec2<f32>( 0.5 ) ) ), mix( vec3<f32>( 0.0 ), clamp( fn130( normalWorldGeometry.xy, render.nodeUniform0 ), vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) ), ( ( clamp( ( ( normalWorldGeometry.y - -0.1 ) / ( 1.0 - -0.1 ) ), 0.0, 1.0 ) * ( 1.0 - 0.0 ) ) + 0.0 ) ) ), 1.0 ) * vec4<f32>( render.nodeUniform4 ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform5 );
	DiffuseColor.w = 1.0;
	let nodeConst0 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst0;

	// result

	output.color = nodeConst0;

	return output;

}
