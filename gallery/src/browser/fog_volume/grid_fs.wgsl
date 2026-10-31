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
@binding( 3 ) @group( 1 ) var nodeUniform19_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform19 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform16 : mat4x4<f32>,
	nodeUniform17 : f32,
	nodeUniform18 : f32,
	nodeUniform20 : f32,
	nodeUniform21 : vec2<f32>,
	nodeUniform22 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec3<f32>;
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
var<private> nodeVar1 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn tsl_mod_float( x : f32, y : f32 ) -> f32 { return x - y * floor( x / y ); }
fn fn24 ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn fn25 ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	

	let nodeConst0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeConst0 ), sin( nodeConst0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

}


fn fn110 ( alpha : f32, dotNL : f32, dotNV : f32 ) -> f32 {

	

	let nodeConst0 = ( alpha * alpha );

	return ( 0.5 / max( ( ( dotNL * sqrt( ( nodeConst0 + ( ( 1.0 - nodeConst0 ) * ( dotNV * dotNV ) ) ) ) ) + ( dotNV * sqrt( ( nodeConst0 + ( ( 1.0 - nodeConst0 ) * ( dotNL * dotNL ) ) ) ) ) ), 0.000001 ) );

}


fn fn112 ( alpha : f32, dotNH : f32 ) -> f32 {

	

	let nodeConst0 = ( alpha * alpha );
	let nodeConst1 = ( 1.0 - ( ( dotNH * dotNH ) * ( 1.0 - nodeConst0 ) ) );

	return ( ( nodeConst0 / ( nodeConst1 * nodeConst1 ) ) * 0.3183098861837907 );

}




@fragment
fn main( @location( 0 ) v_positionWorld : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	let nodeConst0 = floor( v_positionWorld.xz );

	if ( ( abs( tsl_mod_float( ( nodeConst0.x + nodeConst0.y ), 2.0 ) ) > 0.5 ) ) {

		nodeVar0 = vec3<f32>( 0.04373502925049377, 0.04817182422013895, 0.061246054224174035 );

	} else {

		nodeVar0 = vec3<f32>( 0.033104766565152086, 0.036889450395083165, 0.04666508633021928 );

	}

	let nodeConst1 = ( v_positionWorld.xz * vec2<f32>( 5.0 ) );
	let nodeConst2 = ( ( vec2<f32>( 0.5 ) - abs( ( fract( nodeConst1 ) - vec2<f32>( 0.5 ) ) ) ) / max( fwidth( nodeConst1 ), vec2<f32>( 0.0001 ) ) );
	let nodeConst3 = ( ( vec2<f32>( 0.5 ) - abs( ( fract( v_positionWorld.xz ) - vec2<f32>( 0.5 ) ) ) ) / max( fwidth( v_positionWorld.xz ), vec2<f32>( 0.0001 ) ) );
	let nodeConst4 = abs( ( v_positionWorld.xz - round( v_positionWorld.xz ) ) );
	let nodeConst5 = max( fwidth( v_positionWorld.xz ), vec2<f32>( 0.0001 ) );
	DiffuseColor = vec4<f32>( mix( mix( mix( nodeVar0, vec3<f32>( 0.07421356837213867, 0.08228270712149792, 0.10702310296918527 ), max( smoothstep( 0.9, 0.0, nodeConst2.x ), smoothstep( 0.9, 0.0, nodeConst2.y ) ) ), vec3<f32>( 0.7304607400847158, 0.7304607400847158, 0.7304607400847158 ), max( smoothstep( 1.4, 0.0, nodeConst3.x ), smoothstep( 1.4, 0.0, nodeConst3.y ) ) ), vec3<f32>( 1.0, 1.0, 1.0 ), max( ( smoothstep( 0.08, 0.07, nodeConst4.x ) * smoothstep( ( nodeConst5.y * 1.5 ), 0.0, nodeConst4.y ) ), ( smoothstep( 0.08, 0.07, nodeConst4.y ) * smoothstep( ( nodeConst5.x * 1.5 ), 0.0, nodeConst4.x ) ) ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform2;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst6 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform3, 0.045 ) + max( max( nodeConst6.x, nodeConst6.y ), nodeConst6.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar1 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar1;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst7 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst7 );
	let nodeConst8 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst9 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst7 * nodeConst8 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst9 ) * nodeConst8 ) ) ) * vec3<f32>( nodeConst9 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	irradiance = ( irradiance + mix( render.nodeUniform9, render.nodeUniform10, ( ( dot( normalWorld, normalize( render.nodeUniform12 ) ) * 0.5 ) + 0.5 ) ) );
	let nodeConst10 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform13 - render.nodeUniform14 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	let nodeConst11 = ( render.nodeUniform16 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform17 ) ) ), 1.0 ) );
	let nodeConst12 = ( nodeConst11.xyz / vec3<f32>( nodeConst11.w ) );
	let nodeConst13 = vec3<f32>( nodeConst12.x, ( 1.0 - nodeConst12.y ), ( nodeConst12.z + render.nodeUniform18 ) );

	if ( ( ( ( ( ( nodeConst13.x >= 0.0 ) && ( nodeConst13.x <= 1.0 ) ) && ( nodeConst13.y >= 0.0 ) ) && ( nodeConst13.y <= 1.0 ) ) && ( nodeConst13.z <= 1.0 ) ) ) {

		let nodeConst14 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst15 = ( vec2<f32>( render.nodeUniform20 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform21 ) );
		let nodeConst16 = ( nodeConst13.xy + ( fn25( 0, 5, nodeConst14 ) * nodeConst15 ) );
		nodeVar3 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst16, nodeConst13.z );
		let nodeConst17 = ( nodeConst13.xy + ( fn25( 1, 5, nodeConst14 ) * nodeConst15 ) );
		nodeVar4 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst17, nodeConst13.z );
		let nodeConst18 = ( nodeConst13.xy + ( fn25( 2, 5, nodeConst14 ) * nodeConst15 ) );
		nodeVar5 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst18, nodeConst13.z );
		let nodeConst19 = ( nodeConst13.xy + ( fn25( 3, 5, nodeConst14 ) * nodeConst15 ) );
		nodeVar6 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst19, nodeConst13.z );
		let nodeConst20 = ( nodeConst13.xy + ( fn25( 4, 5, nodeConst14 ) * nodeConst15 ) );
		nodeVar7 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst20, nodeConst13.z );
		nodeVar2 = ( ( ( ( ( nodeVar3 + nodeVar4 ) + nodeVar5 ) + nodeVar6 ) + nodeVar7 ) * 0.2 );

	} else {

		nodeVar2 = 1.0;

	}

	nodeVar8 = mix( 1.0, nodeVar2, render.nodeUniform22 );
	nodeVar9 = ( vec3<f32>( clamp( dot( normalView, nodeConst10 ), 0.0, 1.0 ) ) * ( render.nodeUniform15 * vec3<f32>( nodeVar8 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst21 = clamp( dot( positionViewDirection, normalize( ( nodeConst10 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst22 = exp2( ( ( ( nodeConst21 * -5.55473 ) - 6.98316 ) * nodeConst21 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar9 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst22 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst22 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst23 = normalize( ( nodeConst10 + positionViewDirection ) );
	let nodeConst24 = clamp( dot( positionViewDirection, nodeConst23 ), 0.0, 1.0 );
	let nodeConst25 = exp2( ( ( ( nodeConst24 * -5.55473 ) - 6.98316 ) * nodeConst24 ) );
	let nodeConst26 = max( Roughness, 0.045 );
	let nodeConst27 = ( nodeConst26 * nodeConst26 );
	directSpecular = ( directSpecular + ( ( nodeVar9 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst25 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst25 ) ) ) * vec3<f32>( fn110( nodeConst27, clamp( dot( normalView, nodeConst10 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst27, clamp( dot( normalView, nodeConst23 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	nodeVar10 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar10 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst28 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst28 );
	let nodeConst29 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst30 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst28 * nodeConst29 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst30 ) * nodeConst29 ) ) ) * vec3<f32>( nodeConst30 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst31 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar11 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst31 ) );
	nodeVar12 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst31 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar11 );
	indirectDiffuse = ( indirectDiffuse + nodeVar12 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst32 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst32;

	// result

	output.color = nodeConst32;

	return output;

}
