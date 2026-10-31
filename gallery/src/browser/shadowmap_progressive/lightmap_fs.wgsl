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
@binding( 1 ) @group( 1 ) var nodeUniform16_sampler : sampler_comparison;
@binding( 2 ) @group( 1 ) var nodeUniform16 : texture_depth_2d;
@binding( 3 ) @group( 1 ) var nodeUniform26_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform26 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform36_sampler : sampler_comparison;
@binding( 6 ) @group( 1 ) var nodeUniform36 : texture_depth_2d;
@binding( 7 ) @group( 1 ) var nodeUniform46_sampler : sampler_comparison;
@binding( 8 ) @group( 1 ) var nodeUniform46 : texture_depth_2d;
@binding( 9 ) @group( 1 ) var nodeUniform50_sampler : sampler;
@binding( 10 ) @group( 1 ) var nodeUniform50 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform51 : mat3x3<f32>,
	nodeUniform52 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform11 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform32 : vec3<f32>,
	nodeUniform42 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform30 : vec3<f32>,
	nodeUniform31 : vec3<f32>,
	nodeUniform40 : vec3<f32>,
	nodeUniform41 : vec3<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform17 : f32,
	nodeUniform18 : vec2<f32>,
	nodeUniform19 : f32,
	nodeUniform23 : mat4x4<f32>,
	nodeUniform24 : f32,
	nodeUniform25 : f32,
	nodeUniform27 : f32,
	nodeUniform28 : vec2<f32>,
	nodeUniform29 : f32,
	nodeUniform33 : mat4x4<f32>,
	nodeUniform34 : f32,
	nodeUniform35 : f32,
	nodeUniform37 : f32,
	nodeUniform38 : vec2<f32>,
	nodeUniform39 : f32,
	nodeUniform43 : mat4x4<f32>,
	nodeUniform44 : f32,
	nodeUniform45 : f32,
	nodeUniform47 : f32,
	nodeUniform48 : vec2<f32>,
	nodeUniform49 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Shininess : f32;
var<private> SpecularColor : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar0 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> directSpecular : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
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
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> indirectDiffuse : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar28 : vec4<f32>;

// codes
fn fn24 ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn fn25 ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	

	let nodeConst0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeConst0 ), sin( nodeConst0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

}




@fragment
fn main( @location( 0 ) v_positionWorld : vec3<f32>,
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@location( 3 ) nodeVarying5 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Shininess = max( object.nodeUniform2, 0.0001 );
	SpecularColor = object.nodeUniform3;
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	let nodeConst0 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform9 - render.nodeUniform10 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst1 = ( render.nodeUniform13 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform14 ) ) ), 1.0 ) );
	let nodeConst2 = ( nodeConst1.xyz / vec3<f32>( nodeConst1.w ) );
	let nodeConst3 = vec3<f32>( nodeConst2.x, ( 1.0 - nodeConst2.y ), ( nodeConst2.z + render.nodeUniform15 ) );

	if ( ( ( ( ( ( nodeConst3.x >= 0.0 ) && ( nodeConst3.x <= 1.0 ) ) && ( nodeConst3.y >= 0.0 ) ) && ( nodeConst3.y <= 1.0 ) ) && ( nodeConst3.z <= 1.0 ) ) ) {

		let nodeConst4 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst5 = ( vec2<f32>( render.nodeUniform17 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform18 ) );
		let nodeConst6 = ( nodeConst3.xy + ( fn25( 0, 5, nodeConst4 ) * nodeConst5 ) );
		nodeVar1 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeConst6, nodeConst3.z );
		let nodeConst7 = ( nodeConst3.xy + ( fn25( 1, 5, nodeConst4 ) * nodeConst5 ) );
		nodeVar2 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeConst7, nodeConst3.z );
		let nodeConst8 = ( nodeConst3.xy + ( fn25( 2, 5, nodeConst4 ) * nodeConst5 ) );
		nodeVar3 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeConst8, nodeConst3.z );
		let nodeConst9 = ( nodeConst3.xy + ( fn25( 3, 5, nodeConst4 ) * nodeConst5 ) );
		nodeVar4 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeConst9, nodeConst3.z );
		let nodeConst10 = ( nodeConst3.xy + ( fn25( 4, 5, nodeConst4 ) * nodeConst5 ) );
		nodeVar5 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeConst10, nodeConst3.z );
		nodeVar0 = ( ( ( ( ( nodeVar1 + nodeVar2 ) + nodeVar3 ) + nodeVar4 ) + nodeVar5 ) * 0.2 );

	} else {

		nodeVar0 = 1.0;

	}

	nodeVar6 = mix( 1.0, nodeVar0, render.nodeUniform19 );
	let nodeConst11 = ( vec3<f32>( clamp( dot( normalView, nodeConst0 ), 0.0, 1.0 ) ) * ( render.nodeUniform11 * vec3<f32>( nodeVar6 ) ) );
	directDiffuse = ( directDiffuse + ( nodeConst11 * ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	positionViewDirection = normalize( v_positionViewDirection );
	let nodeConst12 = normalize( ( nodeConst0 + positionViewDirection ) );
	let nodeConst13 = clamp( dot( positionViewDirection, nodeConst12 ), 0.0, 1.0 );
	let nodeConst14 = exp2( ( ( ( nodeConst13 * -5.55473 ) - 6.98316 ) * nodeConst13 ) );
	directSpecular = ( directSpecular + ( ( nodeConst11 * ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst14 ) ) ) + vec3<f32>( ( 1.0 * nodeConst14 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeConst12 ), 0.0, 1.0 ), Shininess ) ) ) ) ) * vec3<f32>( 1.0 ) ) );
	let nodeConst15 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform20 - render.nodeUniform21 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	let nodeConst16 = ( render.nodeUniform23 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform24 ) ) ), 1.0 ) );
	let nodeConst17 = ( nodeConst16.xyz / vec3<f32>( nodeConst16.w ) );
	let nodeConst18 = vec3<f32>( nodeConst17.x, ( 1.0 - nodeConst17.y ), ( nodeConst17.z + render.nodeUniform25 ) );

	if ( ( ( ( ( ( nodeConst18.x >= 0.0 ) && ( nodeConst18.x <= 1.0 ) ) && ( nodeConst18.y >= 0.0 ) ) && ( nodeConst18.y <= 1.0 ) ) && ( nodeConst18.z <= 1.0 ) ) ) {

		let nodeConst19 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst20 = ( vec2<f32>( render.nodeUniform27 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform28 ) );
		let nodeConst21 = ( nodeConst18.xy + ( fn25( 0, 5, nodeConst19 ) * nodeConst20 ) );
		nodeVar8 = textureSampleCompare( nodeUniform26, nodeUniform26_sampler, nodeConst21, nodeConst18.z );
		let nodeConst22 = ( nodeConst18.xy + ( fn25( 1, 5, nodeConst19 ) * nodeConst20 ) );
		nodeVar9 = textureSampleCompare( nodeUniform26, nodeUniform26_sampler, nodeConst22, nodeConst18.z );
		let nodeConst23 = ( nodeConst18.xy + ( fn25( 2, 5, nodeConst19 ) * nodeConst20 ) );
		nodeVar10 = textureSampleCompare( nodeUniform26, nodeUniform26_sampler, nodeConst23, nodeConst18.z );
		let nodeConst24 = ( nodeConst18.xy + ( fn25( 3, 5, nodeConst19 ) * nodeConst20 ) );
		nodeVar11 = textureSampleCompare( nodeUniform26, nodeUniform26_sampler, nodeConst24, nodeConst18.z );
		let nodeConst25 = ( nodeConst18.xy + ( fn25( 4, 5, nodeConst19 ) * nodeConst20 ) );
		nodeVar12 = textureSampleCompare( nodeUniform26, nodeUniform26_sampler, nodeConst25, nodeConst18.z );
		nodeVar7 = ( ( ( ( ( nodeVar8 + nodeVar9 ) + nodeVar10 ) + nodeVar11 ) + nodeVar12 ) * 0.2 );

	} else {

		nodeVar7 = 1.0;

	}

	nodeVar13 = mix( 1.0, nodeVar7, render.nodeUniform29 );
	let nodeConst26 = ( vec3<f32>( clamp( dot( normalView, nodeConst15 ), 0.0, 1.0 ) ) * ( render.nodeUniform22 * vec3<f32>( nodeVar13 ) ) );
	directDiffuse = ( directDiffuse + ( nodeConst26 * ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) ) ) );
	let nodeConst27 = normalize( ( nodeConst15 + positionViewDirection ) );
	let nodeConst28 = clamp( dot( positionViewDirection, nodeConst27 ), 0.0, 1.0 );
	let nodeConst29 = exp2( ( ( ( nodeConst28 * -5.55473 ) - 6.98316 ) * nodeConst28 ) );
	directSpecular = ( directSpecular + ( ( nodeConst26 * ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst29 ) ) ) + vec3<f32>( ( 1.0 * nodeConst29 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeConst27 ), 0.0, 1.0 ), Shininess ) ) ) ) ) * vec3<f32>( 1.0 ) ) );
	let nodeConst30 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform30 - render.nodeUniform31 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	let nodeConst31 = ( render.nodeUniform33 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform34 ) ) ), 1.0 ) );
	let nodeConst32 = ( nodeConst31.xyz / vec3<f32>( nodeConst31.w ) );
	let nodeConst33 = vec3<f32>( nodeConst32.x, ( 1.0 - nodeConst32.y ), ( nodeConst32.z + render.nodeUniform35 ) );

	if ( ( ( ( ( ( nodeConst33.x >= 0.0 ) && ( nodeConst33.x <= 1.0 ) ) && ( nodeConst33.y >= 0.0 ) ) && ( nodeConst33.y <= 1.0 ) ) && ( nodeConst33.z <= 1.0 ) ) ) {

		let nodeConst34 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst35 = ( vec2<f32>( render.nodeUniform37 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform38 ) );
		let nodeConst36 = ( nodeConst33.xy + ( fn25( 0, 5, nodeConst34 ) * nodeConst35 ) );
		nodeVar15 = textureSampleCompare( nodeUniform36, nodeUniform36_sampler, nodeConst36, nodeConst33.z );
		let nodeConst37 = ( nodeConst33.xy + ( fn25( 1, 5, nodeConst34 ) * nodeConst35 ) );
		nodeVar16 = textureSampleCompare( nodeUniform36, nodeUniform36_sampler, nodeConst37, nodeConst33.z );
		let nodeConst38 = ( nodeConst33.xy + ( fn25( 2, 5, nodeConst34 ) * nodeConst35 ) );
		nodeVar17 = textureSampleCompare( nodeUniform36, nodeUniform36_sampler, nodeConst38, nodeConst33.z );
		let nodeConst39 = ( nodeConst33.xy + ( fn25( 3, 5, nodeConst34 ) * nodeConst35 ) );
		nodeVar18 = textureSampleCompare( nodeUniform36, nodeUniform36_sampler, nodeConst39, nodeConst33.z );
		let nodeConst40 = ( nodeConst33.xy + ( fn25( 4, 5, nodeConst34 ) * nodeConst35 ) );
		nodeVar19 = textureSampleCompare( nodeUniform36, nodeUniform36_sampler, nodeConst40, nodeConst33.z );
		nodeVar14 = ( ( ( ( ( nodeVar15 + nodeVar16 ) + nodeVar17 ) + nodeVar18 ) + nodeVar19 ) * 0.2 );

	} else {

		nodeVar14 = 1.0;

	}

	nodeVar20 = mix( 1.0, nodeVar14, render.nodeUniform39 );
	let nodeConst41 = ( vec3<f32>( clamp( dot( normalView, nodeConst30 ), 0.0, 1.0 ) ) * ( render.nodeUniform32 * vec3<f32>( nodeVar20 ) ) );
	directDiffuse = ( directDiffuse + ( nodeConst41 * ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) ) ) );
	let nodeConst42 = normalize( ( nodeConst30 + positionViewDirection ) );
	let nodeConst43 = clamp( dot( positionViewDirection, nodeConst42 ), 0.0, 1.0 );
	let nodeConst44 = exp2( ( ( ( nodeConst43 * -5.55473 ) - 6.98316 ) * nodeConst43 ) );
	directSpecular = ( directSpecular + ( ( nodeConst41 * ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst44 ) ) ) + vec3<f32>( ( 1.0 * nodeConst44 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeConst42 ), 0.0, 1.0 ), Shininess ) ) ) ) ) * vec3<f32>( 1.0 ) ) );
	let nodeConst45 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform40 - render.nodeUniform41 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	let nodeConst46 = ( render.nodeUniform43 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform44 ) ) ), 1.0 ) );
	let nodeConst47 = ( nodeConst46.xyz / vec3<f32>( nodeConst46.w ) );
	let nodeConst48 = vec3<f32>( nodeConst47.x, ( 1.0 - nodeConst47.y ), ( nodeConst47.z + render.nodeUniform45 ) );

	if ( ( ( ( ( ( nodeConst48.x >= 0.0 ) && ( nodeConst48.x <= 1.0 ) ) && ( nodeConst48.y >= 0.0 ) ) && ( nodeConst48.y <= 1.0 ) ) && ( nodeConst48.z <= 1.0 ) ) ) {

		let nodeConst49 = ( fn24( fragCoord.xy ) * 6.28318530718 );
		let nodeConst50 = ( vec2<f32>( render.nodeUniform47 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform48 ) );
		let nodeConst51 = ( nodeConst48.xy + ( fn25( 0, 5, nodeConst49 ) * nodeConst50 ) );
		nodeVar22 = textureSampleCompare( nodeUniform46, nodeUniform46_sampler, nodeConst51, nodeConst48.z );
		let nodeConst52 = ( nodeConst48.xy + ( fn25( 1, 5, nodeConst49 ) * nodeConst50 ) );
		nodeVar23 = textureSampleCompare( nodeUniform46, nodeUniform46_sampler, nodeConst52, nodeConst48.z );
		let nodeConst53 = ( nodeConst48.xy + ( fn25( 2, 5, nodeConst49 ) * nodeConst50 ) );
		nodeVar24 = textureSampleCompare( nodeUniform46, nodeUniform46_sampler, nodeConst53, nodeConst48.z );
		let nodeConst54 = ( nodeConst48.xy + ( fn25( 3, 5, nodeConst49 ) * nodeConst50 ) );
		nodeVar25 = textureSampleCompare( nodeUniform46, nodeUniform46_sampler, nodeConst54, nodeConst48.z );
		let nodeConst55 = ( nodeConst48.xy + ( fn25( 4, 5, nodeConst49 ) * nodeConst50 ) );
		nodeVar26 = textureSampleCompare( nodeUniform46, nodeUniform46_sampler, nodeConst55, nodeConst48.z );
		nodeVar21 = ( ( ( ( ( nodeVar22 + nodeVar23 ) + nodeVar24 ) + nodeVar25 ) + nodeVar26 ) * 0.2 );

	} else {

		nodeVar21 = 1.0;

	}

	nodeVar27 = mix( 1.0, nodeVar21, render.nodeUniform49 );
	let nodeConst56 = ( vec3<f32>( clamp( dot( normalView, nodeConst45 ), 0.0, 1.0 ) ) * ( render.nodeUniform42 * vec3<f32>( nodeVar27 ) ) );
	directDiffuse = ( directDiffuse + ( nodeConst56 * ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) ) ) );
	let nodeConst57 = normalize( ( nodeConst45 + positionViewDirection ) );
	let nodeConst58 = clamp( dot( positionViewDirection, nodeConst57 ), 0.0, 1.0 );
	let nodeConst59 = exp2( ( ( ( nodeConst58 * -5.55473 ) - 6.98316 ) * nodeConst58 ) );
	directSpecular = ( directSpecular + ( ( nodeConst56 * ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst59 ) ) ) + vec3<f32>( ( 1.0 * nodeConst59 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeConst57 ), 0.0, 1.0 ), Shininess ) ) ) ) ) * vec3<f32>( 1.0 ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( vec4<f32>( indirectDiffuse, 1.0 ) + ( vec4<f32>( irradiance, 1.0 ) * ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) ) ) ).xyz;
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar28 = textureSample( nodeUniform50, nodeUniform50_sampler, ( object.nodeUniform51 * vec3<f32>( nodeVarying5, 1.0 ) ).xy );

	// result

	output.color = mix( nodeVar28, Output, ( 1.0 / object.nodeUniform52 ) );

	return output;

}
