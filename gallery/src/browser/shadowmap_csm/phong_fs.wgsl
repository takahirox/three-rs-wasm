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
@binding( 1 ) @group( 1 ) var nodeUniform23_sampler : sampler_comparison;
@binding( 2 ) @group( 1 ) var nodeUniform23 : texture_depth_2d;
@binding( 3 ) @group( 1 ) var nodeUniform30_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform30 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform37_sampler : sampler_comparison;
@binding( 6 ) @group( 1 ) var nodeUniform37 : texture_depth_2d;
@binding( 7 ) @group( 1 ) var nodeUniform44_sampler : sampler_comparison;
@binding( 8 ) @group( 1 ) var nodeUniform44 : texture_depth_2d;

struct cascadesStruct {
	value : array< vec4<f32>, 4 >
};
@binding( 1 ) @group( 0 )
var<uniform> cascades : cascadesStruct;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform13 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	shadowFar : f32,
	nodeUniform18 : f32,
	nodeUniform20 : mat4x4<f32>,
	nodeUniform21 : f32,
	nodeUniform22 : f32,
	nodeUniform24 : f32,
	nodeUniform25 : vec2<f32>,
	nodeUniform26 : f32,
	nodeUniform27 : mat4x4<f32>,
	nodeUniform28 : f32,
	nodeUniform29 : f32,
	nodeUniform31 : f32,
	nodeUniform32 : vec2<f32>,
	nodeUniform33 : f32,
	nodeUniform34 : mat4x4<f32>,
	nodeUniform35 : f32,
	nodeUniform36 : f32,
	nodeUniform38 : f32,
	nodeUniform39 : vec2<f32>,
	nodeUniform40 : f32,
	nodeUniform41 : mat4x4<f32>,
	nodeUniform42 : f32,
	nodeUniform43 : f32,
	nodeUniform45 : f32,
	nodeUniform46 : vec2<f32>,
	nodeUniform47 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Shininess : f32;
var<private> SpecularColor : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> shadowValue : vec4<f32>;
var<private> cascade : vec2<f32>;
var<private> linearDepth : f32;
var<private> nodeVar0 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
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
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn24 ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn fn25 ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	

	let nodeConst0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeConst0 ), sin( nodeConst0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

}




@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_positionViewDirection : vec3<f32>,
	@location( 2 ) v_positionWorld : vec3<f32>,
	@location( 3 ) v_normalViewGeometry : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Shininess = max( object.nodeUniform2, 0.0001 );
	SpecularColor = object.nodeUniform3;
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	irradiance = ( irradiance + render.nodeUniform6 );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	let nodeConst0 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform10 - render.nodeUniform11 ), 0.0 ) ).xyz );
	let nodeConst1 = ( vec3<f32>( clamp( dot( normalView, nodeConst0 ), 0.0, 1.0 ) ) * render.nodeUniform12 );
	directDiffuse = ( directDiffuse + ( nodeConst1 * ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	positionViewDirection = normalize( v_positionViewDirection );
	let nodeConst2 = normalize( ( nodeConst0 + positionViewDirection ) );
	let nodeConst3 = clamp( dot( positionViewDirection, nodeConst2 ), 0.0, 1.0 );
	let nodeConst4 = exp2( ( ( ( nodeConst3 * -5.55473 ) - 6.98316 ) * nodeConst3 ) );
	directSpecular = ( directSpecular + ( ( nodeConst1 * ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst4 ) ) ) + vec3<f32>( ( 1.0 * nodeConst4 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeConst2 ), 0.0, 1.0 ), Shininess ) ) ) ) ) * vec3<f32>( 1.0 ) ) );
	let nodeConst5 = normalize( ( render.cameraViewMatrix * vec4<f32>( ( render.nodeUniform14 - render.nodeUniform15 ), 0.0 ) ).xyz );
	shadowPositionWorld = v_positionWorld;
	shadowValue = vec4<f32>( 1.0, 1.0, 1.0, 1.0 );
	cascade = vec2<f32>( 0.0, 0.0 );
	cascade = cascades.value[ 0u ].xy;
	linearDepth = ( ( v_positionView.z + render.nodeUniform18 ) / ( render.nodeUniform18 - render.shadowFar ) );

	if ( ( ( linearDepth >= cascade.x ) && ( linearDepth <= cascade.y ) ) ) {

		shadowPositionWorld = v_positionWorld;
		normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
		let nodeConst6 = ( render.nodeUniform20 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform21 ) ) ), 1.0 ) );
		let nodeConst7 = ( nodeConst6.xyz / vec3<f32>( nodeConst6.w ) );
		let nodeConst8 = vec3<f32>( nodeConst7.x, ( 1.0 - nodeConst7.y ), ( nodeConst7.z + render.nodeUniform22 ) );

		if ( ( ( ( ( ( nodeConst8.x >= 0.0 ) && ( nodeConst8.x <= 1.0 ) ) && ( nodeConst8.y >= 0.0 ) ) && ( nodeConst8.y <= 1.0 ) ) && ( nodeConst8.z <= 1.0 ) ) ) {

			let nodeConst9 = ( fn24( fragCoord.xy ) * 6.28318530718 );
			let nodeConst10 = ( vec2<f32>( render.nodeUniform24 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform25 ) );
			let nodeConst11 = ( nodeConst8.xy + ( fn25( 0, 5, nodeConst9 ) * nodeConst10 ) );
			nodeVar1 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeConst11, nodeConst8.z );
			let nodeConst12 = ( nodeConst8.xy + ( fn25( 1, 5, nodeConst9 ) * nodeConst10 ) );
			nodeVar2 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeConst12, nodeConst8.z );
			let nodeConst13 = ( nodeConst8.xy + ( fn25( 2, 5, nodeConst9 ) * nodeConst10 ) );
			nodeVar3 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeConst13, nodeConst8.z );
			let nodeConst14 = ( nodeConst8.xy + ( fn25( 3, 5, nodeConst9 ) * nodeConst10 ) );
			nodeVar4 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeConst14, nodeConst8.z );
			let nodeConst15 = ( nodeConst8.xy + ( fn25( 4, 5, nodeConst9 ) * nodeConst10 ) );
			nodeVar5 = textureSampleCompare( nodeUniform23, nodeUniform23_sampler, nodeConst15, nodeConst8.z );
			nodeVar0 = ( ( ( ( ( nodeVar1 + nodeVar2 ) + nodeVar3 ) + nodeVar4 ) + nodeVar5 ) * 0.2 );

		} else {

			nodeVar0 = 1.0;

		}

		nodeVar6 = mix( 1.0, nodeVar0, render.nodeUniform26 );
		shadowValue = vec4<f32>( nodeVar6 );
		

	}

	cascade = cascades.value[ 1u ].xy;

	if ( ( ( linearDepth >= cascade.x ) && ( linearDepth <= cascade.y ) ) ) {

		shadowPositionWorld = v_positionWorld;
		normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
		let nodeConst16 = ( render.nodeUniform27 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform28 ) ) ), 1.0 ) );
		let nodeConst17 = ( nodeConst16.xyz / vec3<f32>( nodeConst16.w ) );
		let nodeConst18 = vec3<f32>( nodeConst17.x, ( 1.0 - nodeConst17.y ), ( nodeConst17.z + render.nodeUniform29 ) );

		if ( ( ( ( ( ( nodeConst18.x >= 0.0 ) && ( nodeConst18.x <= 1.0 ) ) && ( nodeConst18.y >= 0.0 ) ) && ( nodeConst18.y <= 1.0 ) ) && ( nodeConst18.z <= 1.0 ) ) ) {

			let nodeConst19 = ( fn24( fragCoord.xy ) * 6.28318530718 );
			let nodeConst20 = ( vec2<f32>( render.nodeUniform31 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform32 ) );
			let nodeConst21 = ( nodeConst18.xy + ( fn25( 0, 5, nodeConst19 ) * nodeConst20 ) );
			nodeVar8 = textureSampleCompare( nodeUniform30, nodeUniform30_sampler, nodeConst21, nodeConst18.z );
			let nodeConst22 = ( nodeConst18.xy + ( fn25( 1, 5, nodeConst19 ) * nodeConst20 ) );
			nodeVar9 = textureSampleCompare( nodeUniform30, nodeUniform30_sampler, nodeConst22, nodeConst18.z );
			let nodeConst23 = ( nodeConst18.xy + ( fn25( 2, 5, nodeConst19 ) * nodeConst20 ) );
			nodeVar10 = textureSampleCompare( nodeUniform30, nodeUniform30_sampler, nodeConst23, nodeConst18.z );
			let nodeConst24 = ( nodeConst18.xy + ( fn25( 3, 5, nodeConst19 ) * nodeConst20 ) );
			nodeVar11 = textureSampleCompare( nodeUniform30, nodeUniform30_sampler, nodeConst24, nodeConst18.z );
			let nodeConst25 = ( nodeConst18.xy + ( fn25( 4, 5, nodeConst19 ) * nodeConst20 ) );
			nodeVar12 = textureSampleCompare( nodeUniform30, nodeUniform30_sampler, nodeConst25, nodeConst18.z );
			nodeVar7 = ( ( ( ( ( nodeVar8 + nodeVar9 ) + nodeVar10 ) + nodeVar11 ) + nodeVar12 ) * 0.2 );

		} else {

			nodeVar7 = 1.0;

		}

		nodeVar13 = mix( 1.0, nodeVar7, render.nodeUniform33 );
		shadowValue = vec4<f32>( nodeVar13 );
		

	}

	cascade = cascades.value[ 2u ].xy;

	if ( ( ( linearDepth >= cascade.x ) && ( linearDepth <= cascade.y ) ) ) {

		shadowPositionWorld = v_positionWorld;
		normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
		let nodeConst26 = ( render.nodeUniform34 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform35 ) ) ), 1.0 ) );
		let nodeConst27 = ( nodeConst26.xyz / vec3<f32>( nodeConst26.w ) );
		let nodeConst28 = vec3<f32>( nodeConst27.x, ( 1.0 - nodeConst27.y ), ( nodeConst27.z + render.nodeUniform36 ) );

		if ( ( ( ( ( ( nodeConst28.x >= 0.0 ) && ( nodeConst28.x <= 1.0 ) ) && ( nodeConst28.y >= 0.0 ) ) && ( nodeConst28.y <= 1.0 ) ) && ( nodeConst28.z <= 1.0 ) ) ) {

			let nodeConst29 = ( fn24( fragCoord.xy ) * 6.28318530718 );
			let nodeConst30 = ( vec2<f32>( render.nodeUniform38 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform39 ) );
			let nodeConst31 = ( nodeConst28.xy + ( fn25( 0, 5, nodeConst29 ) * nodeConst30 ) );
			nodeVar15 = textureSampleCompare( nodeUniform37, nodeUniform37_sampler, nodeConst31, nodeConst28.z );
			let nodeConst32 = ( nodeConst28.xy + ( fn25( 1, 5, nodeConst29 ) * nodeConst30 ) );
			nodeVar16 = textureSampleCompare( nodeUniform37, nodeUniform37_sampler, nodeConst32, nodeConst28.z );
			let nodeConst33 = ( nodeConst28.xy + ( fn25( 2, 5, nodeConst29 ) * nodeConst30 ) );
			nodeVar17 = textureSampleCompare( nodeUniform37, nodeUniform37_sampler, nodeConst33, nodeConst28.z );
			let nodeConst34 = ( nodeConst28.xy + ( fn25( 3, 5, nodeConst29 ) * nodeConst30 ) );
			nodeVar18 = textureSampleCompare( nodeUniform37, nodeUniform37_sampler, nodeConst34, nodeConst28.z );
			let nodeConst35 = ( nodeConst28.xy + ( fn25( 4, 5, nodeConst29 ) * nodeConst30 ) );
			nodeVar19 = textureSampleCompare( nodeUniform37, nodeUniform37_sampler, nodeConst35, nodeConst28.z );
			nodeVar14 = ( ( ( ( ( nodeVar15 + nodeVar16 ) + nodeVar17 ) + nodeVar18 ) + nodeVar19 ) * 0.2 );

		} else {

			nodeVar14 = 1.0;

		}

		nodeVar20 = mix( 1.0, nodeVar14, render.nodeUniform40 );
		shadowValue = vec4<f32>( nodeVar20 );
		

	}

	cascade = cascades.value[ 3u ].xy;

	if ( ( ( linearDepth >= cascade.x ) && ( linearDepth <= cascade.y ) ) ) {

		shadowPositionWorld = v_positionWorld;
		normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
		let nodeConst36 = ( render.nodeUniform41 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform42 ) ) ), 1.0 ) );
		let nodeConst37 = ( nodeConst36.xyz / vec3<f32>( nodeConst36.w ) );
		let nodeConst38 = vec3<f32>( nodeConst37.x, ( 1.0 - nodeConst37.y ), ( nodeConst37.z + render.nodeUniform43 ) );

		if ( ( ( ( ( ( nodeConst38.x >= 0.0 ) && ( nodeConst38.x <= 1.0 ) ) && ( nodeConst38.y >= 0.0 ) ) && ( nodeConst38.y <= 1.0 ) ) && ( nodeConst38.z <= 1.0 ) ) ) {

			let nodeConst39 = ( fn24( fragCoord.xy ) * 6.28318530718 );
			let nodeConst40 = ( vec2<f32>( render.nodeUniform45 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform46 ) );
			let nodeConst41 = ( nodeConst38.xy + ( fn25( 0, 5, nodeConst39 ) * nodeConst40 ) );
			nodeVar22 = textureSampleCompare( nodeUniform44, nodeUniform44_sampler, nodeConst41, nodeConst38.z );
			let nodeConst42 = ( nodeConst38.xy + ( fn25( 1, 5, nodeConst39 ) * nodeConst40 ) );
			nodeVar23 = textureSampleCompare( nodeUniform44, nodeUniform44_sampler, nodeConst42, nodeConst38.z );
			let nodeConst43 = ( nodeConst38.xy + ( fn25( 2, 5, nodeConst39 ) * nodeConst40 ) );
			nodeVar24 = textureSampleCompare( nodeUniform44, nodeUniform44_sampler, nodeConst43, nodeConst38.z );
			let nodeConst44 = ( nodeConst38.xy + ( fn25( 3, 5, nodeConst39 ) * nodeConst40 ) );
			nodeVar25 = textureSampleCompare( nodeUniform44, nodeUniform44_sampler, nodeConst44, nodeConst38.z );
			let nodeConst45 = ( nodeConst38.xy + ( fn25( 4, 5, nodeConst39 ) * nodeConst40 ) );
			nodeVar26 = textureSampleCompare( nodeUniform44, nodeUniform44_sampler, nodeConst45, nodeConst38.z );
			nodeVar21 = ( ( ( ( ( nodeVar22 + nodeVar23 ) + nodeVar24 ) + nodeVar25 ) + nodeVar26 ) * 0.2 );

		} else {

			nodeVar21 = 1.0;

		}

		nodeVar27 = mix( 1.0, nodeVar21, render.nodeUniform47 );
		shadowValue = vec4<f32>( nodeVar27 );
		

	}

	let nodeConst46 = ( vec4<f32>( clamp( dot( normalView, nodeConst5 ), 0.0, 1.0 ) ) * ( vec4<f32>( render.nodeUniform16, 1.0 ) * shadowValue ) );
	directDiffuse = ( vec4<f32>( directDiffuse, 1.0 ) + ( nodeConst46 * vec4<f32>( ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) ), 1.0 ) ) ).xyz;
	let nodeConst47 = normalize( ( nodeConst5 + positionViewDirection ) );
	let nodeConst48 = clamp( dot( positionViewDirection, nodeConst47 ), 0.0, 1.0 );
	let nodeConst49 = exp2( ( ( ( nodeConst48 * -5.55473 ) - 6.98316 ) * nodeConst48 ) );
	directSpecular = ( vec4<f32>( directSpecular, 1.0 ) + ( ( nodeConst46 * vec4<f32>( ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst49 ) ) ) + vec3<f32>( ( 1.0 * nodeConst49 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeConst47 ), 0.0, 1.0 ), Shininess ) ) ) ), 1.0 ) ) * vec4<f32>( 1.0 ) ) ).xyz;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( vec4<f32>( indirectDiffuse, 1.0 ) + ( vec4<f32>( irradiance, 1.0 ) * ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) ) ) ).xyz;
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst50 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst50;

	// result

	output.color = nodeConst50;

	return output;

}
