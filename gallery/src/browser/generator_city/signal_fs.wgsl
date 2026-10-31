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
@binding( 3 ) @group( 1 ) var nodeUniform16_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform16 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform24_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform24 : texture_3d<f32>;
@binding( 7 ) @group( 1 ) var nodeUniform26_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform26 : texture_cube<f32>;

struct objectStruct {
	nodeUniform4 : f32,
	nodeUniform6 : mat3x3<f32>,
	nodeUniform8 : mat4x4<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform27 : mat4x4<f32>,
	nodeUniform29 : f32,
	nodeUniform30 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform17 : f32,
	nodeUniform18 : vec2<f32>,
	nodeUniform19 : f32,
	nodeUniform25 : f32,
	nodeUniform23 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec3<f32>;
var<private> Metalness : f32;
var<private> nodeVar3 : f32;
var<private> Roughness : f32;
var<private> nodeVar4 : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar6 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar7 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> lightProbeIrradiance : vec3<f32>;
var<private> lightProbePriority : f32;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : vec4<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : vec4<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar22 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar23 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
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
fn main( @location( 0 ) @interpolate( flat, either ) nodeVarying3 : f32,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code


	if ( ( nodeVarying3 == 1.0 ) ) {

		nodeVar0 = vec3<f32>( 0.04231141061442144, 0.0030352698352941175, 0.0018211619011764706 );

	} else {


		if ( ( nodeVarying3 == 2.0 ) ) {

			nodeVar1 = vec3<f32>( 0.05126945836711539, 0.019382360952473074, 0.002428215868235294 );

		} else {

			let nodeConst1 = ( nodeVarying3 == 3.0 );

			if ( nodeConst1 ) {

				nodeVar2 = vec3<f32>( 0.024157632443547246, 0.6444796819634361, 0.07036009568874305 );

			} else {

				nodeVar2 = vec3<f32>( 0.01680737574872402, 0.02955683443236377, 0.02217388478862708 );

			}

			nodeVar1 = nodeVar2;

		}

		nodeVar0 = nodeVar1;

	}

	DiffuseColor = vec4<f32>( nodeVar0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform4 );
	DiffuseColor.w = 1.0;
	let nodeConst2 = ( nodeVarying3 > 0.5 );

	if ( nodeConst2 ) {

		nodeVar3 = 0.0;

	} else {

		nodeVar3 = 0.7;

	}

	let nodeConst3 = nodeVar3;
	Metalness = nodeConst3;

	if ( nodeConst2 ) {

		nodeVar4 = 0.3;

	} else {

		nodeVar4 = 0.5;

	}

	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst4 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( nodeVar4, 0.045 ) + max( max( nodeConst4.x, nodeConst4.y ), nodeConst4.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - nodeConst3 ) ) );
	let nodeConst1 = ( nodeVarying3 == 3.0 );

	if ( nodeConst1 ) {

		nodeVar5 = ( vec3<f32>( 0.024157632443547246, 0.6444796819634361, 0.07036009568874305 ) * vec3<f32>( 18.0 ) );

	} else {

		nodeVar5 = vec3<f32>( 0.0, 0.0, 0.0 );

	}

	EmissiveColor = nodeVar5;
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar6 = textureSample( nodeUniform7, nodeUniform7_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar6;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst5 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst5 );
	let nodeConst6 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst7 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst5 * nodeConst6 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst7 ) * nodeConst6 ) ) ) * vec3<f32>( nodeConst7 ) ) );
	let nodeConst8 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform10 - render.nodeUniform11 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst9 = ( render.nodeUniform13 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform14 ) ) ), 1.0 ) );
	let nodeConst10 = ( nodeConst9.xyz / vec3<f32>( nodeConst9.w ) );
	let nodeConst11 = vec3<f32>( nodeConst10.x, ( 1.0 - nodeConst10.y ), ( nodeConst10.z + render.nodeUniform15 ) );

	if ( ( ( ( ( ( nodeConst11.x >= 0.0 ) && ( nodeConst11.x <= 1.0 ) ) && ( nodeConst11.y >= 0.0 ) ) && ( nodeConst11.y <= 1.0 ) ) && ( nodeConst11.z <= 1.0 ) ) ) {

		let nodeConst12 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst13 = ( vec2<f32>( render.nodeUniform17 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform18 ) );
		let nodeConst14 = ( nodeConst11.xy + ( fn25( 0, 5, nodeConst12 ) * nodeConst13 ) );
		nodeVar8 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeConst14, nodeConst11.z );
		let nodeConst15 = ( nodeConst11.xy + ( fn25( 1, 5, nodeConst12 ) * nodeConst13 ) );
		nodeVar9 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeConst15, nodeConst11.z );
		let nodeConst16 = ( nodeConst11.xy + ( fn25( 2, 5, nodeConst12 ) * nodeConst13 ) );
		nodeVar10 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeConst16, nodeConst11.z );
		let nodeConst17 = ( nodeConst11.xy + ( fn25( 3, 5, nodeConst12 ) * nodeConst13 ) );
		nodeVar11 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeConst17, nodeConst11.z );
		let nodeConst18 = ( nodeConst11.xy + ( fn25( 4, 5, nodeConst12 ) * nodeConst13 ) );
		nodeVar12 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeConst18, nodeConst11.z );
		nodeVar7 = ( ( ( ( ( nodeVar8 + nodeVar9 ) + nodeVar10 ) + nodeVar11 ) + nodeVar12 ) * 0.2 );

	} else {

		nodeVar7 = 1.0;

	}

	nodeVar13 = mix( 1.0, nodeVar7, render.nodeUniform19 );
	nodeVar14 = ( vec3<f32>( clamp( dot( normalView, nodeConst8 ), 0.0, 1.0 ) ) * ( render.nodeUniform12 * vec3<f32>( nodeVar13 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst19 = clamp( dot( positionViewDirection, normalize( ( nodeConst8 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst20 = exp2( ( ( ( nodeConst19 * -5.55473 ) - 6.98316 ) * nodeConst19 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar14 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst20 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst20 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst21 = normalize( ( nodeConst8 + positionViewDirection ) );
	let nodeConst22 = clamp( dot( positionViewDirection, nodeConst21 ), 0.0, 1.0 );
	let nodeConst23 = exp2( ( ( ( nodeConst22 * -5.55473 ) - 6.98316 ) * nodeConst22 ) );
	let nodeConst24 = max( Roughness, 0.045 );
	let nodeConst25 = ( nodeConst24 * nodeConst24 );
	directSpecular = ( directSpecular + ( ( nodeVar14 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst23 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst23 ) ) ) * vec3<f32>( fn110( nodeConst25, clamp( dot( normalView, nodeConst8 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst25, clamp( dot( normalView, nodeConst21 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	lightProbeIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	lightProbePriority = ( - 1000000.0 );
	let nodeConst26 = ( ( object.nodeUniform20 - object.nodeUniform21 ) / object.nodeUniform22 );
	let nodeConst27 = ( v_positionWorld + ( ( normalWorld * nodeConst26 ) * vec3<f32>( 0.5 ) ) );
	let nodeConst28 = ( nodeConst26 * vec3<f32>( 0.01 ) );

	if ( ( ( all( ( nodeConst27 >= ( object.nodeUniform21 - nodeConst28 ) ) ) && all( ( nodeConst27 <= ( object.nodeUniform20 + nodeConst28 ) ) ) ) && ( render.nodeUniform23 >= lightProbePriority ) ) ) {

		let nodeConst29 = ( object.nodeUniform20 - object.nodeUniform21 );
		let nodeConst30 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform22 );
		let nodeConst31 = clamp( ( ( ( v_positionWorld + ( ( normalWorld * ( nodeConst29 / object.nodeUniform22 ) ) * vec3<f32>( 0.5 ) ) ) - object.nodeUniform21 ) / nodeConst29 ), nodeConst30, ( vec3<f32>( 1.0 ) - nodeConst30 ) );
		let nodeConst32 = ( ( nodeConst31.z * object.nodeUniform22.z ) + 1.0 );
		let nodeConst33 = ( object.nodeUniform22.z + 2.0 );
		let nodeConst34 = ( nodeConst33 * 7.0 );
		nodeVar15 = textureSample( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeConst31.xy, ( ( nodeConst32 + ( nodeConst33 * 0.0 ) ) / nodeConst34 ) ) );
		nodeVar16 = textureSample( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeConst31.xy, ( ( nodeConst32 + ( nodeConst33 * 1.0 ) ) / nodeConst34 ) ) );
		nodeVar17 = textureSample( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeConst31.xy, ( ( nodeConst32 + ( nodeConst33 * 2.0 ) ) / nodeConst34 ) ) );
		nodeVar18 = textureSample( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeConst31.xy, ( ( nodeConst32 + ( nodeConst33 * 3.0 ) ) / nodeConst34 ) ) );
		nodeVar19 = textureSample( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeConst31.xy, ( ( nodeConst32 + ( nodeConst33 * 4.0 ) ) / nodeConst34 ) ) );
		nodeVar20 = textureSample( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeConst31.xy, ( ( nodeConst32 + ( nodeConst33 * 5.0 ) ) / nodeConst34 ) ) );
		nodeVar21 = textureSample( nodeUniform24, nodeUniform24_sampler, vec3<f32>( nodeConst31.xy, ( ( nodeConst32 + ( nodeConst33 * 6.0 ) ) / nodeConst34 ) ) );
		let nodeConst35 = array< vec3<f32>, 9 >( nodeVar15.xyz, vec3<f32>( nodeVar15.w, nodeVar16.xy ), vec3<f32>( nodeVar16.zw, nodeVar17.x ), nodeVar17.yzw, nodeVar18.xyz, vec3<f32>( nodeVar18.w, nodeVar19.xy ), vec3<f32>( nodeVar19.zw, nodeVar20.x ), nodeVar20.yzw, nodeVar21.xyz );
		let nodeConst36 = ( max( ( ( ( ( ( ( ( ( ( nodeConst35[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeConst35[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeConst35[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst35[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeConst35[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeConst35[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeConst35[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeConst35[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst35[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ) * vec3<f32>( render.nodeUniform25 ) );
		lightProbeIrradiance = nodeConst36;
		lightProbePriority = render.nodeUniform23;
		

	}

	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst37 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst38 = ( object.nodeUniform27 * vec4<f32>( nodeConst37, 1.0 ) );
	let nodeConst39 = clamp( Roughness, 0.0, 1.0 );
	nodeVar22 = textureSampleLevel( nodeUniform26, nodeUniform26_sampler, vec3<f32>( ( - nodeConst38.x ), nodeConst38.yz ), ( ( object.nodeUniform29 * nodeConst39 ) * ( 2.0 - nodeConst39 ) ) );
	radiance = ( radiance + ( nodeVar22.xyz * vec3<f32>( object.nodeUniform30 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst40 = ( object.nodeUniform27 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst41 = clamp( 1.0, 0.0, 1.0 );
	nodeVar23 = textureSampleLevel( nodeUniform26, nodeUniform26_sampler, vec3<f32>( ( - nodeConst40.x ), nodeConst40.yz ), ( ( object.nodeUniform29 * nodeConst41 ) * ( 2.0 - nodeConst41 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar23.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform30 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + lightProbeIrradiance );
	nodeVar24 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar24 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst42 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst42 );
	let nodeConst43 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst44 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst42 * nodeConst43 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst44 ) * nodeConst43 ) ) ) * vec3<f32>( nodeConst44 ) ) );
	let nodeConst45 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar25 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst45 ) );
	nodeVar26 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst45 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar25 );
	indirectDiffuse = ( indirectDiffuse + nodeVar26 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst46 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst46;

	// result

	output.color = nodeConst46;

	return output;

}
