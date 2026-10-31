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
@binding( 1 ) @group( 1 ) var nodeUniform1_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform10_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform10 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform19_sampler : sampler_comparison;
@binding( 6 ) @group( 1 ) var nodeUniform19 : texture_depth_2d;
@binding( 7 ) @group( 1 ) var nodeUniform30_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform30 : texture_3d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform2 : mat3x3<f32>,
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : f32,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform8 : vec3<f32>,
	nodeUniform9 : f32,
	nodeUniform11 : mat4x4<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform28 : vec3<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform31 : f32,
	nodeUniform29 : f32,
	nodeUniform13 : vec3<f32>,
	nodeUniform20 : f32,
	nodeUniform21 : vec2<f32>,
	nodeUniform17 : mat4x4<f32>,
	nodeUniform16 : vec4<f32>,
	nodeUniform23 : mat4x4<f32>,
	nodeUniform22 : vec4<f32>,
	nodeUniform15 : f32,
	nodeUniform18 : f32,
	nodeUniform24 : f32,
	nodeUniform25 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec4<f32>;
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
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : f32;
var<private> shadowValue : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> lightProbeIrradiance : vec3<f32>;
var<private> lightProbePriority : f32;
var<private> nodeVar18 : vec4<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : vec4<f32>;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : vec4<f32>;
var<private> nodeVar24 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn45 ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn fn46 ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	

	let nodeConst0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeConst0 ), sin( nodeConst0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

}


fn fn12 ( alpha : f32, dotNL : f32, dotNV : f32 ) -> f32 {

	

	let nodeConst0 = ( alpha * alpha );

	return ( 0.5 / max( ( ( dotNL * sqrt( ( nodeConst0 + ( ( 1.0 - nodeConst0 ) * ( dotNV * dotNV ) ) ) ) ) + ( dotNV * sqrt( ( nodeConst0 + ( ( 1.0 - nodeConst0 ) * ( dotNL * dotNL ) ) ) ) ) ), 0.000001 ) );

}


fn fn14 ( alpha : f32, dotNH : f32 ) -> f32 {

	

	let nodeConst0 = ( alpha * alpha );
	let nodeConst1 = ( 1.0 - ( ( dotNH * dotNH ) * ( 1.0 - nodeConst0 ) ) );

	return ( ( nodeConst0 / ( nodeConst1 * nodeConst1 ) ) * 0.3183098861837907 );

}




@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32>,
	@location( 4 ) nodeVarying7 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform1, nodeUniform1_sampler, ( object.nodeUniform2 * vec3<f32>( nodeVarying7, 1.0 ) ).xy );
	DiffuseColor = ( vec4<f32>( object.nodeUniform0, 1.0 ) * nodeVar0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform4;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform5, 0.045 ) + max( max( nodeConst0.x, nodeConst0.y ), nodeConst0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform4 ) ) );
	EmissiveColor = ( object.nodeUniform8 * vec3<f32>( object.nodeUniform9 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar1 = textureSample( nodeUniform10, nodeUniform10_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar1;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst1 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst1 );
	let nodeConst2 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst3 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst1 * nodeConst2 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst3 ) * nodeConst2 ) ) ) * vec3<f32>( nodeConst3 ) ) );
	let nodeConst4 = normalize( ( render.cameraViewMatrix * vec4<f32>( render.nodeUniform13, 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar2 = vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform15 ) ) ), 1.0 );
	nodeVar3 = ( - v_positionView.z );
	shadowValue = 1.0;

	if ( ( ( nodeVar3 >= render.nodeUniform16.x ) && ( nodeVar3 < render.nodeUniform16.y ) ) ) {

		let nodeConst5 = ( render.nodeUniform17 * nodeVar2 );
		let nodeConst6 = ( nodeConst5.xyz / vec3<f32>( nodeConst5.w ) );
		let nodeConst7 = vec3<f32>( nodeConst6.x, ( 1.0 - nodeConst6.y ), ( nodeConst6.z + render.nodeUniform18 ) );

		if ( ( ( ( ( ( nodeConst7.x >= 0.0 ) && ( nodeConst7.x <= 1.0 ) ) && ( nodeConst7.y >= 0.0 ) ) && ( nodeConst7.y <= 1.0 ) ) && ( nodeConst7.z <= 1.0 ) ) ) {

			let nodeConst8 = ( fn45( fragCoord.xy ) * 6.28318530718 );
			let nodeConst9 = ( vec2<f32>( render.nodeUniform20 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform21 ) );
			let nodeConst10 = ( nodeConst7.xy + ( fn46( 0, 5, nodeConst8 ) * nodeConst9 ) );
			nodeVar5 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst10, nodeConst7.z );
			let nodeConst11 = ( nodeConst7.xy + ( fn46( 1, 5, nodeConst8 ) * nodeConst9 ) );
			nodeVar6 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst11, nodeConst7.z );
			let nodeConst12 = ( nodeConst7.xy + ( fn46( 2, 5, nodeConst8 ) * nodeConst9 ) );
			nodeVar7 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst12, nodeConst7.z );
			let nodeConst13 = ( nodeConst7.xy + ( fn46( 3, 5, nodeConst8 ) * nodeConst9 ) );
			nodeVar8 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst13, nodeConst7.z );
			let nodeConst14 = ( nodeConst7.xy + ( fn46( 4, 5, nodeConst8 ) * nodeConst9 ) );
			nodeVar9 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst14, nodeConst7.z );
			nodeVar4 = ( ( ( ( ( nodeVar5 + nodeVar6 ) + nodeVar7 ) + nodeVar8 ) + nodeVar9 ) * 0.2 );

		} else {

			nodeVar4 = 1.0;

		}

		shadowValue = mix( nodeVar4, shadowValue, smoothstep( render.nodeUniform16.z, render.nodeUniform16.y, nodeVar3 ) );
		

	}


	if ( ( ( nodeVar3 >= render.nodeUniform22.x ) && ( nodeVar3 < render.nodeUniform22.y ) ) ) {

		let nodeConst15 = ( render.nodeUniform23 * nodeVar2 );
		let nodeConst16 = ( nodeConst15.xyz / vec3<f32>( nodeConst15.w ) );
		let nodeConst17 = vec3<f32>( nodeConst16.x, ( 1.0 - nodeConst16.y ), ( nodeConst16.z + render.nodeUniform24 ) );

		if ( ( ( ( ( ( nodeConst17.x >= 0.0 ) && ( nodeConst17.x <= 1.0 ) ) && ( nodeConst17.y >= 0.0 ) ) && ( nodeConst17.y <= 1.0 ) ) && ( nodeConst17.z <= 1.0 ) ) ) {

			let nodeConst18 = ( fn45( fragCoord.xy ) * 6.28318530718 );
			let nodeConst19 = ( vec2<f32>( render.nodeUniform20 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform21 ) );
			let nodeConst20 = ( nodeConst17.xy + ( fn46( 0, 5, nodeConst18 ) * nodeConst19 ) );
			nodeVar11 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst20, nodeConst17.z );
			let nodeConst21 = ( nodeConst17.xy + ( fn46( 1, 5, nodeConst18 ) * nodeConst19 ) );
			nodeVar12 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst21, nodeConst17.z );
			let nodeConst22 = ( nodeConst17.xy + ( fn46( 2, 5, nodeConst18 ) * nodeConst19 ) );
			nodeVar13 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst22, nodeConst17.z );
			let nodeConst23 = ( nodeConst17.xy + ( fn46( 3, 5, nodeConst18 ) * nodeConst19 ) );
			nodeVar14 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst23, nodeConst17.z );
			let nodeConst24 = ( nodeConst17.xy + ( fn46( 4, 5, nodeConst18 ) * nodeConst19 ) );
			nodeVar15 = textureSampleCompare( nodeUniform19, nodeUniform19_sampler, nodeConst24, nodeConst17.z );
			nodeVar10 = ( ( ( ( ( nodeVar11 + nodeVar12 ) + nodeVar13 ) + nodeVar14 ) + nodeVar15 ) * 0.2 );

		} else {

			nodeVar10 = 1.0;

		}

		shadowValue = mix( nodeVar10, shadowValue, smoothstep( render.nodeUniform22.z, render.nodeUniform22.y, nodeVar3 ) );
		

	}

	nodeVar16 = mix( 1.0, shadowValue, render.nodeUniform25 );
	nodeVar17 = ( vec3<f32>( clamp( dot( normalView, nodeConst4 ), 0.0, 1.0 ) ) * ( render.nodeUniform14 * vec3<f32>( nodeVar16 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst25 = clamp( dot( positionViewDirection, normalize( ( nodeConst4 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst26 = exp2( ( ( ( nodeConst25 * -5.55473 ) - 6.98316 ) * nodeConst25 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar17 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst26 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst26 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst27 = normalize( ( nodeConst4 + positionViewDirection ) );
	let nodeConst28 = clamp( dot( positionViewDirection, nodeConst27 ), 0.0, 1.0 );
	let nodeConst29 = exp2( ( ( ( nodeConst28 * -5.55473 ) - 6.98316 ) * nodeConst28 ) );
	let nodeConst30 = max( Roughness, 0.045 );
	let nodeConst31 = ( nodeConst30 * nodeConst30 );
	directSpecular = ( directSpecular + ( ( nodeVar17 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst29 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst29 ) ) ) * vec3<f32>( fn12( nodeConst31, clamp( dot( normalView, nodeConst4 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn14( nodeConst31, clamp( dot( normalView, nodeConst27 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	lightProbeIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	lightProbePriority = ( - 1000000.0 );
	let nodeConst32 = ( ( object.nodeUniform26 - object.nodeUniform27 ) / object.nodeUniform28 );
	let nodeConst33 = ( v_positionWorld + ( ( normalWorld * nodeConst32 ) * vec3<f32>( 0.5 ) ) );
	let nodeConst34 = ( nodeConst32 * vec3<f32>( 0.01 ) );

	if ( ( ( all( ( nodeConst33 >= ( object.nodeUniform27 - nodeConst34 ) ) ) && all( ( nodeConst33 <= ( object.nodeUniform26 + nodeConst34 ) ) ) ) && ( render.nodeUniform29 >= lightProbePriority ) ) ) {

		let nodeConst35 = ( object.nodeUniform26 - object.nodeUniform27 );
		let nodeConst36 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform28 );
		let nodeConst37 = clamp( ( ( ( v_positionWorld + ( ( normalWorld * ( nodeConst35 / object.nodeUniform28 ) ) * vec3<f32>( 0.5 ) ) ) - object.nodeUniform27 ) / nodeConst35 ), nodeConst36, ( vec3<f32>( 1.0 ) - nodeConst36 ) );
		let nodeConst38 = ( ( nodeConst37.z * object.nodeUniform28.z ) + 1.0 );
		let nodeConst39 = ( object.nodeUniform28.z + 2.0 );
		let nodeConst40 = ( nodeConst39 * 7.0 );
		nodeVar18 = textureSample( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeConst37.xy, ( ( nodeConst38 + ( nodeConst39 * 0.0 ) ) / nodeConst40 ) ) );
		nodeVar19 = textureSample( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeConst37.xy, ( ( nodeConst38 + ( nodeConst39 * 1.0 ) ) / nodeConst40 ) ) );
		nodeVar20 = textureSample( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeConst37.xy, ( ( nodeConst38 + ( nodeConst39 * 2.0 ) ) / nodeConst40 ) ) );
		nodeVar21 = textureSample( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeConst37.xy, ( ( nodeConst38 + ( nodeConst39 * 3.0 ) ) / nodeConst40 ) ) );
		nodeVar22 = textureSample( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeConst37.xy, ( ( nodeConst38 + ( nodeConst39 * 4.0 ) ) / nodeConst40 ) ) );
		nodeVar23 = textureSample( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeConst37.xy, ( ( nodeConst38 + ( nodeConst39 * 5.0 ) ) / nodeConst40 ) ) );
		nodeVar24 = textureSample( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeConst37.xy, ( ( nodeConst38 + ( nodeConst39 * 6.0 ) ) / nodeConst40 ) ) );
		let nodeConst41 = array< vec3<f32>, 9 >( nodeVar18.xyz, vec3<f32>( nodeVar18.w, nodeVar19.xy ), vec3<f32>( nodeVar19.zw, nodeVar20.x ), nodeVar20.yzw, nodeVar21.xyz, vec3<f32>( nodeVar21.w, nodeVar22.xy ), vec3<f32>( nodeVar22.zw, nodeVar23.x ), nodeVar23.yzw, nodeVar24.xyz );
		let nodeConst42 = ( max( ( ( ( ( ( ( ( ( ( nodeConst41[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeConst41[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeConst41[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst41[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeConst41[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeConst41[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeConst41[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeConst41[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst41[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ) * vec3<f32>( render.nodeUniform31 ) );
		lightProbeIrradiance = nodeConst42;
		lightProbePriority = render.nodeUniform29;
		

	}

	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + lightProbeIrradiance );
	nodeVar25 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar25 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst43 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst43 );
	let nodeConst44 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst45 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst43 * nodeConst44 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst45 ) * nodeConst44 ) ) ) * vec3<f32>( nodeConst45 ) ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst46 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar26 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst46 ) );
	nodeVar27 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst46 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar26 );
	indirectDiffuse = ( indirectDiffuse + nodeVar27 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst47 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst47;

	// result

	output.color = nodeConst47;

	return output;

}
