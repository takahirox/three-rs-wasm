// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform18_sampler : sampler_comparison;
@binding( 2 ) @group( 1 ) var nodeUniform18 : texture_depth_2d_array;

struct objectStruct {
	nodeUniform1 : vec3<f32>,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : vec3<f32>,
	nodeUniform6 : f32,
	nodeUniform9 : mat3x3<f32>,
	nodeUniform14 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform7 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform15 : mat4x4<f32>,
	nodeUniform16 : f32,
	nodeUniform17 : f32,
	nodeUniform19 : f32,
	nodeUniform20 : mat4x4<f32>,
	nodeUniform21 : f32,
	nodeUniform22 : f32,
	nodeUniform23 : f32,
	nodeUniform24 : mat4x4<f32>,
	nodeUniform25 : f32,
	nodeUniform26 : f32,
	nodeUniform27 : f32,
	nodeUniform28 : mat4x4<f32>,
	nodeUniform29 : f32,
	nodeUniform30 : f32,
	nodeUniform31 : f32,
	nodeUniform32 : vec3<f32>,
	nodeUniform33 : f32,
	nodeUniform34 : f32
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
var<private> nodeVar0 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar7 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec4<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> shadowValue : f32;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar42 : vec4<f32>;
var<private> nodeVar43 : vec4<f32>;
var<private> nodeVar44 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar45 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_normalViewGeometry : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform1, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform2 );
	DiffuseColor.w = 1.0;
	Shininess = max( object.nodeUniform3, 0.0001 );
	SpecularColor = object.nodeUniform4;
	EmissiveColor = ( object.nodeUniform5 * vec3<f32>( object.nodeUniform6 ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar0 = ( irradiance + render.nodeUniform7 );
	irradiance = nodeVar0;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	nodeVar1 = ( render.nodeUniform11 - render.nodeUniform12 );
	nodeVar2 = vec4<f32>( nodeVar1, 0.0 );
	nodeVar3 = ( render.cameraViewMatrix * nodeVar2 );
	nodeVar4 = normalize( nodeVar3.xyz );
	nodeVar5 = nodeVar4;
	nodeVar6 = dot( normalView, nodeVar5 );
	shadowPositionWorld = v_positionWorld;
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar8 = ( render.nodeUniform15 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform16 ) ) ), 1.0 ) );
	nodeVar9 = ( nodeVar8.xyz / vec3<f32>( nodeVar8.w ) );
	nodeVar10 = vec3<f32>( nodeVar9.x, ( 1.0 - nodeVar9.y ), ( nodeVar9.z + render.nodeUniform17 ) );

	if ( ( ( ( ( ( nodeVar10.x >= 0.0 ) && ( nodeVar10.x <= 1.0 ) ) && ( nodeVar10.y >= 0.0 ) ) && ( nodeVar10.y <= 1.0 ) ) && ( nodeVar10.z <= 1.0 ) ) ) {

		nodeVar11 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeVar10.xy, 0, nodeVar10.z );
		nodeVar7 = nodeVar11;

	} else {

		nodeVar7 = 1.0;

	}

	nodeVar12 = mix( 1.0, nodeVar7, render.nodeUniform19 );
	shadowPositionWorld = v_positionWorld;
	nodeVar14 = ( render.nodeUniform20 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform21 ) ) ), 1.0 ) );
	nodeVar15 = ( nodeVar14.xyz / vec3<f32>( nodeVar14.w ) );
	nodeVar16 = vec3<f32>( nodeVar15.x, ( 1.0 - nodeVar15.y ), ( nodeVar15.z + render.nodeUniform22 ) );

	if ( ( ( ( ( ( nodeVar16.x >= 0.0 ) && ( nodeVar16.x <= 1.0 ) ) && ( nodeVar16.y >= 0.0 ) ) && ( nodeVar16.y <= 1.0 ) ) && ( nodeVar16.z <= 1.0 ) ) ) {

		nodeVar17 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeVar16.xy, 1, nodeVar16.z );
		nodeVar13 = nodeVar17;

	} else {

		nodeVar13 = 1.0;

	}

	nodeVar18 = mix( 1.0, nodeVar13, render.nodeUniform23 );
	shadowPositionWorld = v_positionWorld;
	nodeVar20 = ( render.nodeUniform24 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform25 ) ) ), 1.0 ) );
	nodeVar21 = ( nodeVar20.xyz / vec3<f32>( nodeVar20.w ) );
	nodeVar22 = vec3<f32>( nodeVar21.x, ( 1.0 - nodeVar21.y ), ( nodeVar21.z + render.nodeUniform26 ) );

	if ( ( ( ( ( ( nodeVar22.x >= 0.0 ) && ( nodeVar22.x <= 1.0 ) ) && ( nodeVar22.y >= 0.0 ) ) && ( nodeVar22.y <= 1.0 ) ) && ( nodeVar22.z <= 1.0 ) ) ) {

		nodeVar23 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeVar22.xy, 2, nodeVar22.z );
		nodeVar19 = nodeVar23;

	} else {

		nodeVar19 = 1.0;

	}

	nodeVar24 = mix( 1.0, nodeVar19, render.nodeUniform27 );
	shadowPositionWorld = v_positionWorld;
	nodeVar26 = ( render.nodeUniform28 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform29 ) ) ), 1.0 ) );
	nodeVar27 = ( nodeVar26.xyz / vec3<f32>( nodeVar26.w ) );
	nodeVar28 = vec3<f32>( nodeVar27.x, ( 1.0 - nodeVar27.y ), ( nodeVar27.z + render.nodeUniform30 ) );

	if ( ( ( ( ( ( nodeVar28.x >= 0.0 ) && ( nodeVar28.x <= 1.0 ) ) && ( nodeVar28.y >= 0.0 ) ) && ( nodeVar28.y <= 1.0 ) ) && ( nodeVar28.z <= 1.0 ) ) ) {

		nodeVar29 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, nodeVar28.xy, 3, nodeVar28.z );
		nodeVar25 = nodeVar29;

	} else {

		nodeVar25 = 1.0;

	}

	nodeVar30 = mix( 1.0, nodeVar25, render.nodeUniform31 );
	shadowValue = min( min( min( nodeVar12, nodeVar18 ), nodeVar24 ), nodeVar30 );
	nodeVar31 = ( vec3<f32>( clamp( nodeVar6, 0.0, 1.0 ) ) * ( render.nodeUniform13 * vec3<f32>( shadowValue ) ) );
	nodeVar32 = ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) );
	nodeVar33 = ( nodeVar31 * nodeVar32 );
	nodeVar34 = ( directDiffuse + nodeVar33 );
	directDiffuse = nodeVar34;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar35 = normalize( ( nodeVar5 + positionViewDirection ) );
	nodeVar36 = clamp( dot( positionViewDirection, nodeVar35 ), 0.0, 1.0 );
	nodeVar37 = exp2( ( ( ( nodeVar36 * -5.55473 ) - 6.98316 ) * nodeVar36 ) );
	nodeVar38 = ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar37 ) ) ) + vec3<f32>( ( 1.0 * nodeVar37 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeVar35 ), 0.0, 1.0 ), Shininess ) ) ) );
	nodeVar39 = ( nodeVar31 * nodeVar38 );
	nodeVar40 = ( nodeVar39 * vec3<f32>( 1.0 ) );
	nodeVar41 = ( directSpecular + nodeVar40 );
	directSpecular = nodeVar41;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar42 = ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) );
	nodeVar43 = ( vec4<f32>( irradiance, 1.0 ) * nodeVar42 );
	nodeVar44 = ( vec4<f32>( indirectDiffuse, 1.0 ) + nodeVar43 );
	indirectDiffuse = nodeVar44.xyz;
	ambientOcclusion = 1.0;
	nodeVar45 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar45;
	nodeVar46 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar46;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar47 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar47;
	nodeVar48 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar48;
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar49 = vec4<f32>( mix( Output.xyz, render.nodeUniform32, smoothstep( render.nodeUniform33, render.nodeUniform34, ( - v_positionView.z ) ) ), Output.w );
	Output = nodeVar49;

	// result

	output.color = nodeVar49;

	return output;

}
