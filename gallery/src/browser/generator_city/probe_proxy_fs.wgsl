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
@binding( 1 ) @group( 2 ) var nodeUniform14_sampler : sampler;
@binding( 2 ) @group( 2 ) var nodeUniform14 : texture_2d<f32>;
@binding( 3 ) @group( 2 ) var nodeUniform22_sampler : sampler_comparison;
@binding( 4 ) @group( 2 ) var nodeUniform22 : texture_depth_2d;
@binding( 5 ) @group( 2 ) var nodeUniform30_sampler : sampler;
@binding( 6 ) @group( 2 ) var nodeUniform30 : texture_3d<f32>;
@binding( 7 ) @group( 2 ) var nodeUniform32_sampler : sampler;
@binding( 8 ) @group( 2 ) var nodeUniform32 : texture_cube<f32>;

struct cameraViewMatricesStruct {
	value : array< mat4x4<f32>, 48 >
};
@binding( 0 ) @group( 0 )
var<uniform> cameraViewMatrices : cameraViewMatricesStruct;

struct cameraWorldMatricesStruct {
	value : array< mat4x4<f32>, 48 >
};
@binding( 2 ) @group( 0 )
var<uniform> cameraWorldMatrices : cameraWorldMatricesStruct;

struct objectStruct {
	nodeUniform4 : mat4x4<f32>,
	nodeUniform5 : u32,
	nodeUniform6 : f32,
	nodeUniform7 : f32,
	nodeUniform8 : f32,
	nodeUniform11 : mat3x3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform13 : f32,
	nodeUniform26 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform33 : mat4x4<f32>,
	nodeUniform35 : f32,
	nodeUniform36 : f32
};
@binding( 0 ) @group( 2 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform18 : vec3<f32>,
	nodeUniform31 : f32,
	nodeUniform29 : f32,
	nodeUniform16 : vec3<f32>,
	nodeUniform17 : vec3<f32>,
	nodeUniform19 : mat4x4<f32>,
	nodeUniform20 : f32,
	nodeUniform21 : f32,
	nodeUniform23 : f32,
	nodeUniform24 : vec2<f32>,
	nodeUniform25 : f32
};
@binding( 1 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : u32;
var<private> nodeVar1 : u32;
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
var<private> nodeVar2 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> lightProbeIrradiance : vec3<f32>;
var<private> lightProbePriority : f32;
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar18 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar19 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn5 ( seed : u32 ) -> f32 {

	

	let nodeConst0 = ( ( seed * 747796405u ) + 2891336453u );
	let nodeConst1 = ( ( ( nodeConst0 >> ( ( nodeConst0 >> 28u ) + 4u ) ) ^ nodeConst0 ) * 277803737u );

	return ( f32( ( ( nodeConst1 >> 22u ) ^ nodeConst1 ) ) * 2.3283064365386963e-10 );

}


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
fn main( @location( 0 ) @interpolate(flat, either) v_cameraIndex : u32,
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) v_positionViewDirection : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	let nodeConst1 = ( v_positionWorld.x + 101.0 );
	let nodeConst2 = floor( ( nodeConst1 / 112.0 ) );
	let nodeConst3 = ( v_positionWorld.z + 71.0 );
	let nodeConst4 = floor( ( nodeConst3 / 82.0 ) );
	nodeVar0 = object.nodeUniform5;
	nodeVar1 = ( ( ( u32( ( ( ( nodeConst2 * 3.0 ) + clamp( floor( ( ( ( nodeConst1 - ( nodeConst2 * 112.0 ) ) - 5.0 ) / 26.666666666666668 ) ), 0.0, 2.0 ) ) + 4096.0 ) ) * 73856093u ) ^ ( u32( ( ( ( nodeConst4 * 2.0 ) + clamp( floor( ( ( ( nodeConst3 - ( nodeConst4 * 82.0 ) ) - 5.0 ) / 25.0 ) ), 0.0, 1.0 ) ) + 4096.0 ) ) * 19349663u ) ) ^ ( nodeVar0 * 2654435761u ) );
	let nodeConst5 = fn5( ( nodeVar1 + 230900u ) );
	DiffuseColor = vec4<f32>( ( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( vec3<f32>( 0.3915724777393922, 0.09084171117479915, 0.04518620437910499 ), vec3<f32>( 0.33245153633549385, 0.06847816983662762, 0.0343398068028541 ), step( 0.058823529411764705, nodeConst5 ) ), vec3<f32>( 0.2541520943200296, 0.14412847084818123, 0.08437621153575764 ), step( 0.11764705882352941, nodeConst5 ) ), vec3<f32>( 0.20507873637973145, 0.12743768042608497, 0.08021982030622662 ), step( 0.17647058823529413, nodeConst5 ) ), vec3<f32>( 0.55201140150344, 0.36625259558833256, 0.16202937562896222 ), step( 0.23529411764705882, nodeConst5 ) ), vec3<f32>( 0.4793201830913402, 0.3231432091022285, 0.158960835050774 ), step( 0.29411764705882354, nodeConst5 ) ), vec3<f32>( 0.5394794890033748, 0.4396571738310091, 0.22696587349938613 ), step( 0.35294117647058826, nodeConst5 ) ), vec3<f32>( 0.5647115056965487, 0.5271151256969157, 0.4452011945063733 ), step( 0.4117647058823529, nodeConst5 ) ), vec3<f32>( 0.5647115056965487, 0.5271151256969157, 0.4452011945063733 ), step( 0.47058823529411764, nodeConst5 ) ), vec3<f32>( 0.508881320845802, 0.4735314961384573, 0.3915724777393922 ), step( 0.5294117647058824, nodeConst5 ) ), vec3<f32>( 0.6375968739867731, 0.6038273388475408, 0.514917665367466 ), step( 0.5882352941176471, nodeConst5 ) ), vec3<f32>( 0.45641102317066595, 0.4286904966038916, 0.35640014413537763 ), step( 0.6470588235294118, nodeConst5 ) ), vec3<f32>( 0.3231432091022285, 0.3139887133649649, 0.2746773120495699 ), step( 0.7058823529411765, nodeConst5 ) ), vec3<f32>( 0.25818285291079235, 0.2501582847191642, 0.22696587349938613 ), step( 0.7647058823529411, nodeConst5 ) ), vec3<f32>( 0.37626212298046485, 0.36625259558833256, 0.3231432091022285 ), step( 0.8235294117647058, nodeConst5 ) ), vec3<f32>( 0.7083757798856457, 0.6724431569510133, 0.5972017883558645 ), step( 0.8823529411764706, nodeConst5 ) ), vec3<f32>( 0.20155625378383743, 0.23839757380151394, 0.2663556047920505 ), step( 0.9411764705882353, nodeConst5 ) ) * vec3<f32>( ( ( fn5( ( nodeVar1 + 155260u ) ) * 0.12 ) + 0.94 ) ) ), 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform6 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform7;
	normalViewGeometry = normalize( v_normalViewGeometry );
	let nodeConst6 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform8, 0.045 ) + max( max( nodeConst6.x, nodeConst6.y ), nodeConst6.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform7 ) ) );
	EmissiveColor = ( object.nodeUniform12 * vec3<f32>( object.nodeUniform13 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar2 = textureSample( nodeUniform14, nodeUniform14_sampler, vec2<f32>( Roughness, clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar2;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst7 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst7 );
	let nodeConst8 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst9 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst7 * nodeConst8 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst9 ) * nodeConst8 ) ) ) * vec3<f32>( nodeConst9 ) ) );
	let nodeConst10 = normalize( ( cameraViewMatrices.value[ v_cameraIndex ] * vec4<f32>( ( render.nodeUniform16 - render.nodeUniform17 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * cameraViewMatrices.value[ v_cameraIndex ] ).xyz );
	let nodeConst11 = ( render.nodeUniform19 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform20 ) ) ), 1.0 ) );
	let nodeConst12 = ( nodeConst11.xyz / vec3<f32>( nodeConst11.w ) );
	let nodeConst13 = vec3<f32>( nodeConst12.x, ( 1.0 - nodeConst12.y ), ( nodeConst12.z + render.nodeUniform21 ) );

	if ( ( ( ( ( ( nodeConst13.x >= 0.0 ) && ( nodeConst13.x <= 1.0 ) ) && ( nodeConst13.y >= 0.0 ) ) && ( nodeConst13.y <= 1.0 ) ) && ( nodeConst13.z <= 1.0 ) ) ) {

		let nodeConst14 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst15 = ( vec2<f32>( render.nodeUniform23 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform24 ) );
		let nodeConst16 = ( nodeConst13.xy + ( fn25( 0, 5, nodeConst14 ) * nodeConst15 ) );
		nodeVar4 = textureSampleCompare( nodeUniform22, nodeUniform22_sampler, nodeConst16, nodeConst13.z );
		let nodeConst17 = ( nodeConst13.xy + ( fn25( 1, 5, nodeConst14 ) * nodeConst15 ) );
		nodeVar5 = textureSampleCompare( nodeUniform22, nodeUniform22_sampler, nodeConst17, nodeConst13.z );
		let nodeConst18 = ( nodeConst13.xy + ( fn25( 2, 5, nodeConst14 ) * nodeConst15 ) );
		nodeVar6 = textureSampleCompare( nodeUniform22, nodeUniform22_sampler, nodeConst18, nodeConst13.z );
		let nodeConst19 = ( nodeConst13.xy + ( fn25( 3, 5, nodeConst14 ) * nodeConst15 ) );
		nodeVar7 = textureSampleCompare( nodeUniform22, nodeUniform22_sampler, nodeConst19, nodeConst13.z );
		let nodeConst20 = ( nodeConst13.xy + ( fn25( 4, 5, nodeConst14 ) * nodeConst15 ) );
		nodeVar8 = textureSampleCompare( nodeUniform22, nodeUniform22_sampler, nodeConst20, nodeConst13.z );
		nodeVar3 = ( ( ( ( ( nodeVar4 + nodeVar5 ) + nodeVar6 ) + nodeVar7 ) + nodeVar8 ) * 0.2 );

	} else {

		nodeVar3 = 1.0;

	}

	nodeVar9 = mix( 1.0, nodeVar3, render.nodeUniform25 );
	nodeVar10 = ( vec3<f32>( clamp( dot( normalView, nodeConst10 ), 0.0, 1.0 ) ) * ( render.nodeUniform18 * vec3<f32>( nodeVar9 ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst21 = clamp( dot( positionViewDirection, normalize( ( nodeConst10 + positionViewDirection ) ) ), 0.0, 1.0 );
	let nodeConst22 = exp2( ( ( ( nodeConst21 * -5.55473 ) - 6.98316 ) * nodeConst21 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar10 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst22 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst22 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst23 = normalize( ( nodeConst10 + positionViewDirection ) );
	let nodeConst24 = clamp( dot( positionViewDirection, nodeConst23 ), 0.0, 1.0 );
	let nodeConst25 = exp2( ( ( ( nodeConst24 * -5.55473 ) - 6.98316 ) * nodeConst24 ) );
	let nodeConst26 = max( Roughness, 0.045 );
	let nodeConst27 = ( nodeConst26 * nodeConst26 );
	directSpecular = ( directSpecular + ( ( nodeVar10 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst25 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst25 ) ) ) * vec3<f32>( fn110( nodeConst27, clamp( dot( normalView, nodeConst10 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst27, clamp( dot( normalView, nodeConst23 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	lightProbeIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	lightProbePriority = ( - 1000000.0 );
	let nodeConst28 = ( ( object.nodeUniform26 - object.nodeUniform27 ) / object.nodeUniform28 );
	let nodeConst29 = ( v_positionWorld + ( ( normalWorld * nodeConst28 ) * vec3<f32>( 0.5 ) ) );
	let nodeConst30 = ( nodeConst28 * vec3<f32>( 0.01 ) );

	if ( ( ( all( ( nodeConst29 >= ( object.nodeUniform27 - nodeConst30 ) ) ) && all( ( nodeConst29 <= ( object.nodeUniform26 + nodeConst30 ) ) ) ) && ( render.nodeUniform29 >= lightProbePriority ) ) ) {

		let nodeConst31 = ( object.nodeUniform26 - object.nodeUniform27 );
		let nodeConst32 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform28 );
		let nodeConst33 = clamp( ( ( ( v_positionWorld + ( ( normalWorld * ( nodeConst31 / object.nodeUniform28 ) ) * vec3<f32>( 0.5 ) ) ) - object.nodeUniform27 ) / nodeConst31 ), nodeConst32, ( vec3<f32>( 1.0 ) - nodeConst32 ) );
		let nodeConst34 = ( ( nodeConst33.z * object.nodeUniform28.z ) + 1.0 );
		let nodeConst35 = ( object.nodeUniform28.z + 2.0 );
		let nodeConst36 = ( nodeConst35 * 7.0 );
		nodeVar11 = textureSample( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeConst33.xy, ( ( nodeConst34 + ( nodeConst35 * 0.0 ) ) / nodeConst36 ) ) );
		nodeVar12 = textureSample( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeConst33.xy, ( ( nodeConst34 + ( nodeConst35 * 1.0 ) ) / nodeConst36 ) ) );
		nodeVar13 = textureSample( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeConst33.xy, ( ( nodeConst34 + ( nodeConst35 * 2.0 ) ) / nodeConst36 ) ) );
		nodeVar14 = textureSample( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeConst33.xy, ( ( nodeConst34 + ( nodeConst35 * 3.0 ) ) / nodeConst36 ) ) );
		nodeVar15 = textureSample( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeConst33.xy, ( ( nodeConst34 + ( nodeConst35 * 4.0 ) ) / nodeConst36 ) ) );
		nodeVar16 = textureSample( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeConst33.xy, ( ( nodeConst34 + ( nodeConst35 * 5.0 ) ) / nodeConst36 ) ) );
		nodeVar17 = textureSample( nodeUniform30, nodeUniform30_sampler, vec3<f32>( nodeConst33.xy, ( ( nodeConst34 + ( nodeConst35 * 6.0 ) ) / nodeConst36 ) ) );
		let nodeConst37 = array< vec3<f32>, 9 >( nodeVar11.xyz, vec3<f32>( nodeVar11.w, nodeVar12.xy ), vec3<f32>( nodeVar12.zw, nodeVar13.x ), nodeVar13.yzw, nodeVar14.xyz, vec3<f32>( nodeVar14.w, nodeVar15.xy ), vec3<f32>( nodeVar15.zw, nodeVar16.x ), nodeVar16.yzw, nodeVar17.xyz );
		let nodeConst38 = ( max( ( ( ( ( ( ( ( ( ( nodeConst37[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeConst37[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeConst37[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst37[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeConst37[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeConst37[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeConst37[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeConst37[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeConst37[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) ), vec3<f32>( 0.0, 0.0, 0.0 ) ) * vec3<f32>( render.nodeUniform31 ) );
		lightProbeIrradiance = nodeConst38;
		lightProbePriority = render.nodeUniform29;
		

	}

	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst39 = normalize( ( cameraWorldMatrices.value[ v_cameraIndex ] * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst40 = ( object.nodeUniform33 * vec4<f32>( nodeConst39, 1.0 ) );
	let nodeConst41 = clamp( Roughness, 0.0, 1.0 );
	nodeVar18 = textureSampleLevel( nodeUniform32, nodeUniform32_sampler, vec3<f32>( ( - nodeConst40.x ), nodeConst40.yz ), ( ( object.nodeUniform35 * nodeConst41 ) * ( 2.0 - nodeConst41 ) ) );
	radiance = ( radiance + ( nodeVar18.xyz * vec3<f32>( object.nodeUniform36 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst42 = ( object.nodeUniform33 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst43 = clamp( 1.0, 0.0, 1.0 );
	nodeVar19 = textureSampleLevel( nodeUniform32, nodeUniform32_sampler, vec3<f32>( ( - nodeConst42.x ), nodeConst42.yz ), ( ( object.nodeUniform35 * nodeConst43 ) * ( 2.0 - nodeConst43 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar19.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform36 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + lightProbeIrradiance );
	nodeVar20 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar20 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst44 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst44 );
	let nodeConst45 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst46 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst44 * nodeConst45 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst46 ) * nodeConst45 ) ) ) * vec3<f32>( nodeConst46 ) ) );
	let nodeConst47 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar21 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst47 ) );
	nodeVar22 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst47 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar21 );
	indirectDiffuse = ( indirectDiffuse + nodeVar22 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst48 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst48;

	// result

	output.color = nodeConst48;

	return output;

}
