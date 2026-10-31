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
@binding( 1 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform8 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform10_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform10 : texture_cube<f32>;

struct objectStruct {
	nodeUniform0 : f32,
	nodeUniform1 : f32,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform11 : mat4x4<f32>,
	nodeUniform13 : f32,
	nodeUniform14 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> IOR : f32;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> Iridescence : f32;
var<private> IridescenceIOR : f32;
var<private> IridescenceThickness : f32;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar0 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn127 ( outsideIOR : f32, eta2 : f32, cosTheta1 : f32, thinFilmThickness : f32, baseF0 : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : vec3<f32>;
	var nodeVar2 : f32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;

	let nodeConst0 = mix( outsideIOR, eta2, smoothstep( 0.0, 0.03, thinFilmThickness ) );
	let nodeConst1 = ( outsideIOR / nodeConst0 );
	let nodeConst2 = ( 1.0 - ( ( nodeConst1 * nodeConst1 ) * ( 1.0 - ( cosTheta1 * cosTheta1 ) ) ) );

	if ( ( nodeConst2 < 0.0 ) ) {

		return vec3<f32>( 1.0, 1.0, 1.0 );

	}

	let nodeConst3 = ( ( nodeConst0 - outsideIOR ) / ( nodeConst0 + outsideIOR ) );
	let nodeConst4 = exp2( ( ( ( cosTheta1 * -5.55473 ) - 6.98316 ) * cosTheta1 ) );
	let nodeConst5 = ( ( ( nodeConst3 * nodeConst3 ) * ( 1.0 - nodeConst4 ) ) + ( 1.0 * nodeConst4 ) );
	let nodeConst6 = ( 1.0 - nodeConst5 );
	let nodeConst7 = sqrt( clamp( baseF0, vec3<f32>( 0.0 ), vec3<f32>( 0.9999 ) ) );
	let nodeConst8 = ( ( vec3<f32>( 1.0, 1.0, 1.0 ) + nodeConst7 ) / ( vec3<f32>( 1.0, 1.0, 1.0 ) - nodeConst7 ) );
	let nodeConst9 = vec3<f32>( nodeConst0 );
	let nodeConst10 = ( ( nodeConst8 - nodeConst9 ) / ( nodeConst8 + nodeConst9 ) );
	let nodeConst11 = sqrt( nodeConst2 );
	let nodeConst12 = exp2( ( ( ( nodeConst11 * -5.55473 ) - 6.98316 ) * nodeConst11 ) );
	let nodeConst13 = ( ( ( nodeConst10 * nodeConst10 ) * vec3<f32>( ( 1.0 - nodeConst12 ) ) ) + vec3<f32>( ( 1.0 * nodeConst12 ) ) );
	let nodeConst14 = clamp( ( vec3<f32>( nodeConst5 ) * nodeConst13 ), vec3<f32>( 0.00001 ), vec3<f32>( 0.9999 ) );
	let nodeConst15 = ( ( vec3<f32>( ( nodeConst6 * nodeConst6 ) ) * nodeConst13 ) / ( vec3<f32>( 1.0, 1.0, 1.0 ) - nodeConst14 ) );
	nodeVar0 = ( vec3<f32>( nodeConst5 ) + nodeConst15 );
	nodeVar1 = ( nodeConst15 - vec3<f32>( nodeConst6 ) );

	for ( var m : i32 = 1; m <= 2; m ++ ) {

		nodeVar1 = ( nodeVar1 * sqrt( nodeConst14 ) );
		let nodeConst16 = ( ( f32( m ) * ( ( ( nodeConst0 * thinFilmThickness ) * nodeConst11 ) * 2.0 ) ) * 6.283185307179586e-9 );

		if ( ( nodeConst0 < outsideIOR ) ) {

			nodeVar2 = 3.141592653589793;

		} else {

			nodeVar2 = 0.0;

		}


		if ( ( nodeConst8.x < nodeConst0 ) ) {

			nodeVar3 = 3.141592653589793;

		} else {

			nodeVar3 = 0.0;

		}


		if ( ( nodeConst8.y < nodeConst0 ) ) {

			nodeVar4 = 3.141592653589793;

		} else {

			nodeVar4 = 0.0;

		}


		if ( ( nodeConst8.z < nodeConst0 ) ) {

			nodeVar5 = 3.141592653589793;

		} else {

			nodeVar5 = 0.0;

		}

		let nodeConst17 = ( vec3<f32>( f32( m ) ) * ( vec3<f32>( ( 3.141592653589793 - nodeVar2 ) ) + vec3<f32>( nodeVar3, nodeVar4, nodeVar5 ) ) );
		let nodeConst18 = ( ( ( vec3<f32>( 5.4856e-13, 4.4201e-13, 5.2481e-13 ) * sqrt( ( vec3<f32>( 4327800000.0, 9304600000.0, 6612100000.0 ) * vec3<f32>( 6.283185307179586 ) ) ) ) * cos( ( ( vec3<f32>( 1681000.0, 1795300.0, 2208400.0 ) * vec3<f32>( nodeConst16 ) ) + nodeConst17 ) ) ) * exp( ( vec3<f32>( ( - ( nodeConst16 * nodeConst16 ) ) ) * vec3<f32>( 4327800000.0, 9304600000.0, 6612100000.0 ) ) ) );
		nodeVar0 = ( nodeVar0 + ( nodeVar1 * ( ( mat3x3<f32>( 3.2404542, -0.969266, 0.0556434, -1.5371385, 1.8760108, -0.2040259, -0.4985314, 0.041556, 1.0572252 ) * ( vec3<f32>( ( nodeConst18.x + ( ( 1.6440828550896444e-8 * cos( ( ( nodeConst16 * 2239900.0 ) + nodeConst17.x ) ) ) * exp( ( ( nodeConst16 * nodeConst16 ) * -4528200000.0 ) ) ) ), nodeConst18.y, nodeConst18.z ) / vec3<f32>( 1.0685e-7 ) ) ) * vec3<f32>( 2.0 ) ) ) );

	}


	return max( nodeVar0, vec3<f32>( 0.0, 0.0, 0.0 ) );

}


fn fn116 ( f : vec3<f32>, f90 : f32, dotVH : f32 ) -> vec3<f32> {

	

	let nodeConst0 = clamp( ( 1.0 - dotVH ), 0.0, 1.0 );
	let nodeConst1 = ( nodeConst0 * nodeConst0 );
	let nodeConst2 = clamp( ( ( nodeConst0 * nodeConst1 ) * nodeConst1 ), 0.0, 0.9999 );

	return ( ( f - ( vec3<f32>( f90 ) * vec3<f32>( nodeConst2 ) ) ) / vec3<f32>( ( 1.0 - nodeConst2 ) ) );

}




@fragment
fn main( @location( 0 ) v_normalViewGeometry : vec3<f32>,
	@location( 1 ) v_positionViewDirection : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( ( vec3<f32>( 1.0 ) * vec3<f32>( 0.05, 0.05, 0.05 ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform0 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform1;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( 0.0, 0.045 ) + max( max( nodeConst0.x, nodeConst0.y ), nodeConst0.z ) ), 1.0 );
	IOR = object.nodeUniform4;
	let nodeConst1 = ( ( IOR - 1.0 ) / ( IOR + 1.0 ) );
	SpecularColor = ( min( ( vec3<f32>( ( nodeConst1 * nodeConst1 ) ) * vec3<f32>( 1.0, 1.0, 1.0 ) ), vec3<f32>( 1.0, 1.0, 1.0 ) ) * vec3<f32>( object.nodeUniform5 ) );
	SpecularColorBlended = mix( SpecularColor, DiffuseColor.xyz, Metalness );
	SpecularF90 = mix( object.nodeUniform5, 1.0, Metalness );
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform1 ) ) );
	Iridescence = 1.0;
	IridescenceIOR = clamp( 2.0, 1.0, 2.333 );
	IridescenceThickness = 100.0;
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar0 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar0;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst2 = clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 );
	let nodeConst3 = mix( SpecularColor, fn116( fn127( 1.0, IridescenceIOR, nodeConst2, IridescenceThickness, SpecularColor ), 1.0, nodeConst2 ), Iridescence );
	let nodeConst4 = ( ( nodeConst3 * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst4 );
	let nodeConst5 = ( nodeConst3 + ( ( vec3<f32>( 1.0 ) - nodeConst3 ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst6 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst4 * nodeConst5 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst6 ) * nodeConst5 ) ) ) * vec3<f32>( nodeConst6 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst7 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst8 = ( object.nodeUniform11 * vec4<f32>( nodeConst7, 1.0 ) );
	let nodeConst9 = clamp( Roughness, 0.0, 1.0 );
	nodeVar1 = textureSampleLevel( nodeUniform10, nodeUniform10_sampler, vec3<f32>( ( - nodeConst8.x ), nodeConst8.yz ), ( ( object.nodeUniform13 * nodeConst9 ) * ( 2.0 - nodeConst9 ) ) );
	radiance = ( radiance + ( nodeVar1.xyz * vec3<f32>( object.nodeUniform14 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst10 = ( object.nodeUniform11 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst11 = clamp( 1.0, 0.0, 1.0 );
	nodeVar2 = textureSampleLevel( nodeUniform10, nodeUniform10_sampler, vec3<f32>( ( - nodeConst10.x ), nodeConst10.yz ), ( ( object.nodeUniform13 * nodeConst11 ) * ( 2.0 - nodeConst11 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar2.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform14 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar3 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar3 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst12 = mix( DiffuseColor.xyz, fn116( fn127( 1.0, IridescenceIOR, nodeConst2, IridescenceThickness, DiffuseColor.xyz ), 1.0, nodeConst2 ), Iridescence );
	let nodeConst13 = ( ( nodeConst12 * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst13 );
	let nodeConst14 = ( nodeConst12 + ( ( vec3<f32>( 1.0 ) - nodeConst12 ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst15 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst13 * nodeConst14 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst15 ) * nodeConst14 ) ) ) * vec3<f32>( nodeConst15 ) ) );
	let nodeConst16 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar4 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst16 ) );
	nodeVar5 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst16 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar4 );
	indirectDiffuse = ( indirectDiffuse + nodeVar5 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst17 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst17;

	// result

	output.color = nodeConst17;

	return output;

}
