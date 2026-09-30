// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );

// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform16_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform16 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform31_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform31 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform10 : mat4x4<f32>,
	nodeUniform17 : mat3x3<f32>,
	nodeUniform32 : mat3x3<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform19 : f32,
	nodeUniform20 : f32,
	nodeUniform23 : f32,
	nodeUniform24 : f32,
	nodeUniform11 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform18 : f32,
	nodeUniform28 : mat4x4<f32>,
	nodeUniform29 : f32,
	nodeUniform30 : f32,
	nodeUniform33 : f32,
	nodeUniform34 : vec3<f32>,
	nodeUniform35 : f32,
	nodeUniform36 : f32
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
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar4 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec2<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : vec4<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec4<f32>;
var<private> nodeVar41 : vec4<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : vec4<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : vec2<f32>;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar67 : vec4<f32>;
var<private> nodeVar68 : vec4<f32>;
var<private> nodeVar69 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar70 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec4<f32>;

// codes

@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_normalViewGeometry : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Shininess = max( object.nodeUniform2, 0.0001 );
	SpecularColor = object.nodeUniform3;
	EmissiveColor = ( object.nodeUniform4 * vec3<f32>( object.nodeUniform5 ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar0 = ( irradiance + render.nodeUniform6 );
	irradiance = nodeVar0;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	nodeVar1 = ( render.nodeUniform9 - v_positionView );
	nodeVar2 = normalize( nodeVar1 );
	nodeVar3 = dot( normalView, nodeVar2 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar5 = ( render.nodeUniform12 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform14 ) ) ), 1.0 ) );
	nodeVar6 = ( nodeVar5.xyz / vec3<f32>( nodeVar5.w ) );
	nodeVar7 = vec3<f32>( nodeVar6.x, ( 1.0 - nodeVar6.y ), ( nodeVar6.z + render.nodeUniform15 ) );

	if ( ( ( ( ( ( nodeVar7.x >= 0.0 ) && ( nodeVar7.x <= 1.0 ) ) && ( nodeVar7.y >= 0.0 ) ) && ( nodeVar7.y <= 1.0 ) ) && ( nodeVar7.z <= 1.0 ) ) ) {

		nodeVar8 = 1.0;
		nodeVar9 = textureSample( nodeUniform16, nodeUniform16_sampler, ( object.nodeUniform17 * vec3<f32>( nodeVar7.xy, 1.0 ) ).xy ).xy;
		nodeVar10 = step( nodeVar7.z, nodeVar9.x );

		if ( ( nodeVar10 != 1.0 ) ) {

			nodeVar11 = max( 1e-7, ( nodeVar9.y * nodeVar9.y ) );
			nodeVar12 = ( nodeVar7.z - nodeVar9.x );
			nodeVar8 = max( nodeVar10, clamp( ( ( ( nodeVar11 / ( nodeVar11 + ( nodeVar12 * nodeVar12 ) ) ) - 0.3 ) / 0.65 ), 0.0, 1.0 ) );
			

		}

		nodeVar4 = nodeVar8;

	} else {

		nodeVar4 = 1.0;

	}

	nodeVar13 = mix( 1.0, nodeVar4, render.nodeUniform18 );
	nodeVar14 = ( render.nodeUniform11 * vec3<f32>( nodeVar13 ) );
	nodeVar15 = ( render.nodeUniform21 - render.nodeUniform22 );
	nodeVar16 = vec4<f32>( nodeVar15, 0.0 );
	nodeVar17 = ( render.cameraViewMatrix * nodeVar16 );
	nodeVar18 = normalize( nodeVar17.xyz );
	nodeVar19 = nodeVar18;
	nodeVar20 = dot( nodeVar2, nodeVar19 );
	nodeVar21 = smoothstep( render.nodeUniform19, render.nodeUniform20, nodeVar20 );
	nodeVar22 = ( nodeVar14 * vec3<f32>( nodeVar21 ) );

	if ( ( render.nodeUniform23 > 0.0 ) ) {

		nodeVar24 = length( nodeVar1 );
		nodeVar25 = ( nodeVar24 / render.nodeUniform23 );
		nodeVar26 = clamp( ( 1.0 - ( ( ( nodeVar25 * nodeVar25 ) * nodeVar25 ) * nodeVar25 ) ), 0.0, 1.0 );
		nodeVar23 = ( ( 1.0 / max( pow( nodeVar24, render.nodeUniform24 ), 0.01 ) ) * ( nodeVar26 * nodeVar26 ) );

	} else {

		nodeVar23 = ( 1.0 / max( pow( length( nodeVar1 ), render.nodeUniform24 ), 0.01 ) );

	}

	nodeVar27 = ( nodeVar22 * vec3<f32>( nodeVar23 ) );
	nodeVar28 = ( vec3<f32>( clamp( nodeVar3, 0.0, 1.0 ) ) * nodeVar27 );
	nodeVar29 = ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) );
	nodeVar30 = ( nodeVar28 * nodeVar29 );
	nodeVar31 = ( directDiffuse + nodeVar30 );
	directDiffuse = nodeVar31;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar32 = normalize( ( nodeVar2 + positionViewDirection ) );
	nodeVar33 = clamp( dot( positionViewDirection, nodeVar32 ), 0.0, 1.0 );
	nodeVar34 = exp2( ( ( ( nodeVar33 * -5.55473 ) - 6.98316 ) * nodeVar33 ) );
	nodeVar35 = ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar34 ) ) ) + vec3<f32>( ( 1.0 * nodeVar34 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeVar32 ), 0.0, 1.0 ), Shininess ) ) ) );
	nodeVar36 = ( nodeVar28 * nodeVar35 );
	nodeVar37 = ( nodeVar36 * vec3<f32>( 1.0 ) );
	nodeVar38 = ( directSpecular + nodeVar37 );
	directSpecular = nodeVar38;
	nodeVar39 = ( render.nodeUniform25 - render.nodeUniform26 );
	nodeVar40 = vec4<f32>( nodeVar39, 0.0 );
	nodeVar41 = ( render.cameraViewMatrix * nodeVar40 );
	nodeVar42 = normalize( nodeVar41.xyz );
	nodeVar43 = nodeVar42;
	nodeVar44 = dot( normalView, nodeVar43 );
	shadowPositionWorld = v_positionWorld;
	nodeVar46 = ( render.nodeUniform28 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform29 ) ) ), 1.0 ) );
	nodeVar47 = ( nodeVar46.xyz / vec3<f32>( nodeVar46.w ) );
	nodeVar48 = vec3<f32>( nodeVar47.x, ( 1.0 - nodeVar47.y ), ( nodeVar47.z + render.nodeUniform30 ) );

	if ( ( ( ( ( ( nodeVar48.x >= 0.0 ) && ( nodeVar48.x <= 1.0 ) ) && ( nodeVar48.y >= 0.0 ) ) && ( nodeVar48.y <= 1.0 ) ) && ( nodeVar48.z <= 1.0 ) ) ) {

		nodeVar49 = 1.0;
		nodeVar50 = textureSample( nodeUniform31, nodeUniform31_sampler, ( object.nodeUniform32 * vec3<f32>( nodeVar48.xy, 1.0 ) ).xy ).xy;
		nodeVar51 = step( nodeVar48.z, nodeVar50.x );

		if ( ( nodeVar51 != 1.0 ) ) {

			nodeVar52 = max( 1e-7, ( nodeVar50.y * nodeVar50.y ) );
			nodeVar53 = ( nodeVar48.z - nodeVar50.x );
			nodeVar49 = max( nodeVar51, clamp( ( ( ( nodeVar52 / ( nodeVar52 + ( nodeVar53 * nodeVar53 ) ) ) - 0.3 ) / 0.65 ), 0.0, 1.0 ) );
			

		}

		nodeVar45 = nodeVar49;

	} else {

		nodeVar45 = 1.0;

	}

	nodeVar54 = mix( 1.0, nodeVar45, render.nodeUniform33 );
	nodeVar55 = ( render.nodeUniform27 * vec3<f32>( nodeVar54 ) );
	nodeVar56 = ( vec3<f32>( clamp( nodeVar44, 0.0, 1.0 ) ) * nodeVar55 );
	nodeVar57 = ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) );
	nodeVar58 = ( nodeVar56 * nodeVar57 );
	nodeVar59 = ( directDiffuse + nodeVar58 );
	directDiffuse = nodeVar59;
	nodeVar60 = normalize( ( nodeVar43 + positionViewDirection ) );
	nodeVar61 = clamp( dot( positionViewDirection, nodeVar60 ), 0.0, 1.0 );
	nodeVar62 = exp2( ( ( ( nodeVar61 * -5.55473 ) - 6.98316 ) * nodeVar61 ) );
	nodeVar63 = ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar62 ) ) ) + vec3<f32>( ( 1.0 * nodeVar62 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeVar60 ), 0.0, 1.0 ), Shininess ) ) ) );
	nodeVar64 = ( nodeVar56 * nodeVar63 );
	nodeVar65 = ( nodeVar64 * vec3<f32>( 1.0 ) );
	nodeVar66 = ( directSpecular + nodeVar65 );
	directSpecular = nodeVar66;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar67 = ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) );
	nodeVar68 = ( vec4<f32>( irradiance, 1.0 ) * nodeVar67 );
	nodeVar69 = ( vec4<f32>( indirectDiffuse, 1.0 ) + nodeVar68 );
	indirectDiffuse = nodeVar69.xyz;
	ambientOcclusion = 1.0;
	nodeVar70 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar70;
	nodeVar71 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar71;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar72 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar72;
	nodeVar73 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar73;
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar74 = vec4<f32>( mix( Output.xyz, render.nodeUniform34, smoothstep( render.nodeUniform35, render.nodeUniform36, ( - v_positionView.z ) ) ), Output.w );
	Output = nodeVar74;

	// result

	output.color = nodeVar74;

	return output;

}
