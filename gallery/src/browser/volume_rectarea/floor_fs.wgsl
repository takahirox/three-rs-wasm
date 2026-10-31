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
@binding( 1 ) @group( 1 ) var nodeUniform7_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform7 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform9_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform9 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform11_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform11 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform4 : mat3x3<f32>,
	nodeUniform5 : vec3<f32>,
	nodeUniform6 : f32,
	nodeUniform8 : mat4x4<f32>,
	nodeUniform10 : mat3x3<f32>,
	nodeUniform12 : mat3x3<f32>,
	nodeUniform18 : mat3x3<f32>,
	nodeUniform19 : mat3x3<f32>,
	nodeUniform24 : mat3x3<f32>,
	nodeUniform25 : mat3x3<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform17 : vec3<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform23 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform29 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform27 : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar0 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : mat3x3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar10 : vec4<f32>;
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : mat3x3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : vec4<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : mat3x3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn tsl_mod_float( x : f32, y : f32 ) -> f32 { return x - y * floor( x / y ); }
fn fn119 ( v1 : vec3<f32>, v2 : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;

	let nodeConst0 = dot( v1, v2 );
	nodeVar0 = abs( nodeConst0 );
	nodeVar1 = ( ( ( ( nodeVar0 * 0.0145206 ) + 0.4965155 ) * nodeVar0 ) + 0.8543985 );
	nodeVar2 = ( ( ( nodeVar0 + 4.1616724 ) * nodeVar0 ) + 3.417594 );

	if ( ( nodeConst0 > 0.0 ) ) {

		let nodeConst1 = ( nodeVar1 / nodeVar2 );
		nodeVar3 = nodeConst1;

	} else {

		let nodeConst1 = ( nodeVar1 / nodeVar2 );
		nodeVar3 = ( ( inverseSqrt( max( ( 1.0 - ( nodeConst0 * nodeConst0 ) ), 1e-7 ) ) * 0.5 ) - nodeConst1 );

	}


	return ( cross( v1, v2 ) * vec3<f32>( nodeVar3 ) );

}


fn fn118 ( f : vec3<f32> ) -> f32 {

	

	let nodeConst0 = length( f );

	return max( ( ( ( nodeConst0 * nodeConst0 ) + f.z ) / ( nodeConst0 + 1.0 ) ), 0.0 );

}


fn fn45 ( coord : vec2<f32> ) -> f32 {

	

	let nodeConst0 = ( coord * vec2<f32>( 2.0 ) );

	return sign( tsl_mod_float( ( floor( nodeConst0.x ) + floor( nodeConst0.y ) ), 2.0 ) );

}


fn fn117 ( N : vec3<f32>, V : vec3<f32>, roughness : f32 ) -> vec2<f32> {

	

	var nodeVar0 : vec2<f32> = vec2<f32>( roughness, sqrt( ( 1.0 - clamp( dot( N, V ), 0.0, 1.0 ) ) ) );
	nodeVar0 = ( ( nodeVar0 * vec2<f32>( 0.984375 ) ) + vec2<f32>( 0.0078125 ) );

	return nodeVar0;

}


fn fn120 ( N : vec3<f32>, V : vec3<f32>, P : vec3<f32>, mInv : mat3x3<f32>, p0 : vec3<f32>, p1 : vec3<f32>, p2 : vec3<f32>, p3 : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : vec3<f32>;
	var nodeVar2 : vec3<f32>;
	var nodeVar3 : mat3x3<f32>;
	var nodeVar4 : vec3<f32>;
	var nodeVar5 : vec3<f32>;
	var nodeVar6 : vec3<f32>;
	var nodeVar7 : vec3<f32>;
	var nodeVar8 : vec3<f32>;

	nodeVar0 = ( p1 - p0 );
	nodeVar1 = ( p3 - p0 );
	nodeVar2 = vec3<f32>( 0.0, 0.0, 0.0 );

	if ( ( dot( cross( nodeVar0, nodeVar1 ), ( P - p0 ) ) >= 0.0 ) ) {

		let nodeConst0 = normalize( ( V - ( N * vec3<f32>( dot( V, N ) ) ) ) );
		nodeVar3 = ( mInv * transpose( mat3x3<f32>( nodeConst0, ( - cross( N, nodeConst0 ) ), N ) ) );
		nodeVar4 = normalize( ( nodeVar3 * ( p0 - P ) ) );
		nodeVar5 = normalize( ( nodeVar3 * ( p1 - P ) ) );
		nodeVar6 = normalize( ( nodeVar3 * ( p2 - P ) ) );
		nodeVar7 = normalize( ( nodeVar3 * ( p3 - P ) ) );
		nodeVar8 = vec3<f32>( 0.0, 0.0, 0.0 );
		nodeVar8 = ( nodeVar8 + fn119( nodeVar4, nodeVar5 ) );
		nodeVar8 = ( nodeVar8 + fn119( nodeVar5, nodeVar6 ) );
		nodeVar8 = ( nodeVar8 + fn119( nodeVar6, nodeVar7 ) );
		nodeVar8 = ( nodeVar8 + fn119( nodeVar7, nodeVar4 ) );
		nodeVar2 = vec3<f32>( fn118( nodeVar8 ) );
		

	}


	return nodeVar2;

}




@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) nodeVarying6 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform2;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( fn45( ( nodeVarying6 * vec2<f32>( 400.0 ) ) ), 0.045 ) + max( max( nodeConst0.x, nodeConst0.y ), nodeConst0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	EmissiveColor = ( object.nodeUniform5 * vec3<f32>( object.nodeUniform6 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar0 = textureSample( nodeUniform7, nodeUniform7_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar0;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst1 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst1 );
	let nodeConst2 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst3 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst1 * nodeConst2 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst3 ) * nodeConst2 ) ) ) * vec3<f32>( nodeConst3 ) ) );
	nodeVar1 = v_positionView;
	let nodeConst4 = fn117( normalView, positionViewDirection, Roughness );
	nodeVar2 = textureSample( nodeUniform9, nodeUniform9_sampler, ( object.nodeUniform10 * vec3<f32>( nodeConst4, 1.0 ) ).xy );
	nodeVar3 = nodeVar2;
	nodeVar4 = textureSample( nodeUniform11, nodeUniform11_sampler, ( object.nodeUniform12 * vec3<f32>( nodeConst4, 1.0 ) ).xy );
	nodeVar5 = nodeVar4;
	nodeVar6 = mat3x3<f32>( vec3<f32>( nodeVar3.x, 0.0, nodeVar3.y ), vec3<f32>( 0.0, 1.0, 0.0 ), vec3<f32>( nodeVar3.z, 0.0, nodeVar3.w ) );
	nodeVar7 = ( ( SpecularColorBlended * vec3<f32>( nodeVar5.x ) ) + ( ( vec3<f32>( SpecularF90 ) - SpecularColorBlended ) * vec3<f32>( nodeVar5.y ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst5 = ( ( render.nodeUniform14 + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform16, 0.0 ) ).xyz ) - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform17, 0.0 ) ).xyz );
	let nodeConst6 = ( ( render.nodeUniform14 - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform16, 0.0 ) ).xyz ) - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform17, 0.0 ) ).xyz );
	let nodeConst7 = ( ( render.nodeUniform14 - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform16, 0.0 ) ).xyz ) + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform17, 0.0 ) ).xyz );
	let nodeConst8 = ( ( render.nodeUniform14 + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform16, 0.0 ) ).xyz ) + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform17, 0.0 ) ).xyz );
	directSpecular = ( directSpecular + ( ( render.nodeUniform13 * nodeVar7 ) * fn120( normalView, positionViewDirection, nodeVar1, nodeVar6, nodeConst5, nodeConst6, nodeConst7, nodeConst8 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	directDiffuse = ( directDiffuse + ( ( render.nodeUniform13 * DiffuseContribution ) * fn120( normalView, positionViewDirection, nodeVar1, mat3x3<f32>( 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0 ), nodeConst5, nodeConst6, nodeConst7, nodeConst8 ) ) );
	nodeVar8 = v_positionView;
	let nodeConst9 = fn117( normalView, positionViewDirection, Roughness );
	nodeVar9 = textureSample( nodeUniform9, nodeUniform9_sampler, ( object.nodeUniform18 * vec3<f32>( nodeConst9, 1.0 ) ).xy );
	nodeVar10 = nodeVar9;
	nodeVar11 = textureSample( nodeUniform11, nodeUniform11_sampler, ( object.nodeUniform19 * vec3<f32>( nodeConst9, 1.0 ) ).xy );
	nodeVar12 = nodeVar11;
	nodeVar13 = mat3x3<f32>( vec3<f32>( nodeVar10.x, 0.0, nodeVar10.y ), vec3<f32>( 0.0, 1.0, 0.0 ), vec3<f32>( nodeVar10.z, 0.0, nodeVar10.w ) );
	nodeVar14 = ( ( SpecularColorBlended * vec3<f32>( nodeVar12.x ) ) + ( ( vec3<f32>( SpecularF90 ) - SpecularColorBlended ) * vec3<f32>( nodeVar12.y ) ) );
	let nodeConst10 = ( ( render.nodeUniform21 + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform22, 0.0 ) ).xyz ) - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform23, 0.0 ) ).xyz );
	let nodeConst11 = ( ( render.nodeUniform21 - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform22, 0.0 ) ).xyz ) - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform23, 0.0 ) ).xyz );
	let nodeConst12 = ( ( render.nodeUniform21 - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform22, 0.0 ) ).xyz ) + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform23, 0.0 ) ).xyz );
	let nodeConst13 = ( ( render.nodeUniform21 + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform22, 0.0 ) ).xyz ) + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform23, 0.0 ) ).xyz );
	directSpecular = ( directSpecular + ( ( render.nodeUniform20 * nodeVar14 ) * fn120( normalView, positionViewDirection, nodeVar8, nodeVar13, nodeConst10, nodeConst11, nodeConst12, nodeConst13 ) ) );
	directDiffuse = ( directDiffuse + ( ( render.nodeUniform20 * DiffuseContribution ) * fn120( normalView, positionViewDirection, nodeVar8, mat3x3<f32>( 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0 ), nodeConst10, nodeConst11, nodeConst12, nodeConst13 ) ) );
	nodeVar15 = v_positionView;
	let nodeConst14 = fn117( normalView, positionViewDirection, Roughness );
	nodeVar16 = textureSample( nodeUniform9, nodeUniform9_sampler, ( object.nodeUniform24 * vec3<f32>( nodeConst14, 1.0 ) ).xy );
	nodeVar17 = nodeVar16;
	nodeVar18 = textureSample( nodeUniform11, nodeUniform11_sampler, ( object.nodeUniform25 * vec3<f32>( nodeConst14, 1.0 ) ).xy );
	nodeVar19 = nodeVar18;
	nodeVar20 = mat3x3<f32>( vec3<f32>( nodeVar17.x, 0.0, nodeVar17.y ), vec3<f32>( 0.0, 1.0, 0.0 ), vec3<f32>( nodeVar17.z, 0.0, nodeVar17.w ) );
	nodeVar21 = ( ( SpecularColorBlended * vec3<f32>( nodeVar19.x ) ) + ( ( vec3<f32>( SpecularF90 ) - SpecularColorBlended ) * vec3<f32>( nodeVar19.y ) ) );
	let nodeConst15 = ( ( render.nodeUniform27 + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform28, 0.0 ) ).xyz ) - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform29, 0.0 ) ).xyz );
	let nodeConst16 = ( ( render.nodeUniform27 - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform28, 0.0 ) ).xyz ) - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform29, 0.0 ) ).xyz );
	let nodeConst17 = ( ( render.nodeUniform27 - ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform28, 0.0 ) ).xyz ) + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform29, 0.0 ) ).xyz );
	let nodeConst18 = ( ( render.nodeUniform27 + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform28, 0.0 ) ).xyz ) + ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform29, 0.0 ) ).xyz );
	directSpecular = ( directSpecular + ( ( render.nodeUniform26 * nodeVar21 ) * fn120( normalView, positionViewDirection, nodeVar15, nodeVar20, nodeConst15, nodeConst16, nodeConst17, nodeConst18 ) ) );
	directDiffuse = ( directDiffuse + ( ( render.nodeUniform26 * DiffuseContribution ) * fn120( normalView, positionViewDirection, nodeVar15, mat3x3<f32>( 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0 ), nodeConst15, nodeConst16, nodeConst17, nodeConst18 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar22 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar22 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst19 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst19 );
	let nodeConst20 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst21 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst19 * nodeConst20 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst21 ) * nodeConst20 ) ) ) * vec3<f32>( nodeConst21 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst22 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar23 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst22 ) );
	nodeVar24 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst22 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar23 );
	indirectDiffuse = ( indirectDiffuse + nodeVar24 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst23 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst23;

	// result

	output.color = nodeConst23;

	return output;

}
