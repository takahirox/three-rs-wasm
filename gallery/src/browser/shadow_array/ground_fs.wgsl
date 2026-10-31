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
@binding( 2 ) @group( 1 ) var nodeUniform16 : texture_depth_2d_array;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : vec3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : f32,
	nodeUniform8 : mat3x3<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform18 : vec3<f32>,
	nodeUniform19 : f32,
	nodeUniform20 : f32,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform17 : f32
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
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar0 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> shadowValue : f32;
var<private> directSpecular : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar2 : vec4<f32>;

// codes
fn fn61 ( x : u32, k : i32 ) -> u32 {

	var nodeVar0 : i32;
	var nodeVar1 : u32;

	nodeVar0 = k;
	nodeVar1 = x;

	return ( ( nodeVar1 << u32( nodeVar0 ) ) | ( nodeVar1 >> u32( ( 32 - nodeVar0 ) ) ) );

}


fn fn63 ( a : u32, b : u32, c : u32 ) -> u32 {

	var nodeVar0 : u32;
	var nodeVar1 : u32;
	var nodeVar2 : u32;

	nodeVar0 = c;
	nodeVar1 = b;
	nodeVar2 = a;
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - fn61( nodeVar1, 14 ) );
	nodeVar2 = ( nodeVar2 ^ nodeVar0 );
	nodeVar2 = ( nodeVar2 - fn61( nodeVar0, 11 ) );
	nodeVar1 = ( nodeVar1 ^ nodeVar2 );
	nodeVar1 = ( nodeVar1 - fn61( nodeVar2, 25 ) );
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - fn61( nodeVar1, 16 ) );
	nodeVar2 = ( nodeVar2 ^ nodeVar0 );
	nodeVar2 = ( nodeVar2 - fn61( nodeVar0, 4 ) );
	nodeVar1 = ( nodeVar1 ^ nodeVar2 );
	nodeVar1 = ( nodeVar1 - fn61( nodeVar2, 14 ) );
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - fn61( nodeVar1, 24 ) );

	return nodeVar0;

}


fn fn68 ( x : i32, y : i32, z : i32 ) -> u32 {

	var nodeVar0 : i32;
	var nodeVar1 : i32;
	var nodeVar2 : i32;
	var nodeVar3 : u32;
	var nodeVar4 : u32;
	var nodeVar5 : u32;
	var nodeVar6 : u32;

	nodeVar0 = z;
	nodeVar1 = y;
	nodeVar2 = x;
	nodeVar3 = 3u;
	nodeVar4 = 0u;
	nodeVar5 = 0u;
	nodeVar6 = 0u;
	nodeVar6 = ( ( 3735928559u + ( nodeVar3 << 2u ) ) + 13u );
	nodeVar5 = nodeVar6;
	nodeVar4 = nodeVar5;
	nodeVar4 = ( nodeVar4 + u32( nodeVar2 ) );
	nodeVar5 = ( nodeVar5 + u32( nodeVar1 ) );
	nodeVar6 = ( nodeVar6 + u32( nodeVar0 ) );

	return fn63( nodeVar4, nodeVar5, nodeVar6 );

}


fn fn46 ( b : bool, t : f32, f : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : bool;
	var nodeVar3 : f32;

	nodeVar0 = f;
	nodeVar1 = t;
	nodeVar2 = b;

	return select( nodeVar0, nodeVar1, nodeVar2 );

}


fn fn47 ( val : f32, b : bool ) -> f32 {

	var nodeVar0 : bool;
	var nodeVar1 : f32;
	var nodeVar2 : f32;

	nodeVar0 = b;
	nodeVar1 = val;

	return select( nodeVar1, ( - nodeVar1 ), nodeVar0 );

}


fn fn54 ( hash : u32, x : f32, y : f32, z : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : u32;
	var nodeVar4 : u32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;

	nodeVar0 = z;
	nodeVar1 = y;
	nodeVar2 = x;
	nodeVar3 = hash;
	nodeVar4 = ( nodeVar3 & 15u );
	nodeVar5 = fn46( ( nodeVar4 < 8u ), nodeVar2, nodeVar1 );
	nodeVar6 = fn46( ( nodeVar4 < 4u ), nodeVar1, fn46( ( ( nodeVar4 == 12u ) || ( nodeVar4 == 14u ) ), nodeVar2, nodeVar0 ) );

	return ( fn47( nodeVar5, bool( ( nodeVar4 & 1u ) ) ) + fn47( nodeVar6, bool( ( nodeVar4 & 2u ) ) ) );

}


fn fn48 ( x : f32 ) -> i32 {

	var nodeVar0 : f32;

	nodeVar0 = x;

	return i32( floor( nodeVar0 ) );

}


fn fn65 ( t : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = t;

	return ( ( ( nodeVar0 * nodeVar0 ) * nodeVar0 ) * ( ( nodeVar0 * ( ( nodeVar0 * 6.0 ) - 15.0 ) ) + 10.0 ) );

}


fn fn72 ( x : i32, y : i32, z : i32 ) -> vec3<u32> {

	var nodeVar0 : i32;
	var nodeVar1 : i32;
	var nodeVar2 : i32;
	var nodeVar3 : u32;
	var nodeVar4 : vec3<u32>;

	nodeVar0 = z;
	nodeVar1 = y;
	nodeVar2 = x;
	nodeVar3 = fn68( nodeVar2, nodeVar1, nodeVar0 );
	nodeVar4 = vec3<u32>( 0u, 0u, 0u );
	nodeVar4.x = ( nodeVar3 & 255u );
	nodeVar4.y = ( ( nodeVar3 >> 8u ) & 255u );
	nodeVar4.z = ( ( nodeVar3 >> 16u ) & 255u );

	return nodeVar4;

}


fn fn56 ( hash : vec3<u32>, x : f32, y : f32, z : f32 ) -> vec3<f32> {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : vec3<u32>;

	nodeVar0 = z;
	nodeVar1 = y;
	nodeVar2 = x;
	nodeVar3 = hash;

	return vec3<f32>( fn54( nodeVar3.x, nodeVar2, nodeVar1, nodeVar0 ), fn54( nodeVar3.y, nodeVar2, nodeVar1, nodeVar0 ), fn54( nodeVar3.z, nodeVar2, nodeVar1, nodeVar0 ) );

}


fn fn52 ( v0 : vec3<f32>, v1 : vec3<f32>, v2 : vec3<f32>, v3 : vec3<f32>, v4 : vec3<f32>, v5 : vec3<f32>, v6 : vec3<f32>, v7 : vec3<f32>, s : f32, t : f32, r : f32 ) -> vec3<f32> {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : vec3<f32>;
	var nodeVar4 : vec3<f32>;
	var nodeVar5 : vec3<f32>;
	var nodeVar6 : vec3<f32>;
	var nodeVar7 : vec3<f32>;
	var nodeVar8 : vec3<f32>;
	var nodeVar9 : vec3<f32>;
	var nodeVar10 : vec3<f32>;
	var nodeVar11 : f32;
	var nodeVar12 : f32;
	var nodeVar13 : f32;

	nodeVar0 = r;
	nodeVar1 = t;
	nodeVar2 = s;
	nodeVar3 = v7;
	nodeVar4 = v6;
	nodeVar5 = v5;
	nodeVar6 = v4;
	nodeVar7 = v3;
	nodeVar8 = v2;
	nodeVar9 = v1;
	nodeVar10 = v0;
	nodeVar11 = ( 1.0 - nodeVar2 );
	nodeVar12 = ( 1.0 - nodeVar1 );
	nodeVar13 = ( 1.0 - nodeVar0 );

	return ( ( vec3<f32>( nodeVar13 ) * ( ( vec3<f32>( nodeVar12 ) * ( ( nodeVar10 * vec3<f32>( nodeVar11 ) ) + ( nodeVar9 * vec3<f32>( nodeVar2 ) ) ) ) + ( vec3<f32>( nodeVar1 ) * ( ( nodeVar8 * vec3<f32>( nodeVar11 ) ) + ( nodeVar7 * vec3<f32>( nodeVar2 ) ) ) ) ) ) + ( vec3<f32>( nodeVar0 ) * ( ( vec3<f32>( nodeVar12 ) * ( ( nodeVar6 * vec3<f32>( nodeVar11 ) ) + ( nodeVar5 * vec3<f32>( nodeVar2 ) ) ) ) + ( vec3<f32>( nodeVar1 ) * ( ( nodeVar4 * vec3<f32>( nodeVar11 ) ) + ( nodeVar3 * vec3<f32>( nodeVar2 ) ) ) ) ) ) );

}


fn fn60 ( v : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;

	nodeVar0 = v;

	return ( vec3<f32>( 0.982 ) * nodeVar0 );

}


fn fn76 ( p : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : i32;
	var nodeVar2 : i32;
	var nodeVar3 : i32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;
	var nodeVar7 : f32;
	var nodeVar8 : f32;
	var nodeVar9 : f32;
	var nodeVar10 : f32;
	var nodeVar11 : f32;
	var nodeVar12 : f32;
	var nodeVar13 : vec3<f32>;

	nodeVar0 = p;
	nodeVar1 = 0;
	nodeVar2 = 0;
	nodeVar3 = 0;
	nodeVar4 = nodeVar0.x;
	nodeVar1 = fn48( nodeVar4 );
	nodeVar5 = ( nodeVar4 - f32( nodeVar1 ) );
	nodeVar6 = nodeVar0.y;
	nodeVar2 = fn48( nodeVar6 );
	nodeVar7 = ( nodeVar6 - f32( nodeVar2 ) );
	nodeVar8 = nodeVar0.z;
	nodeVar3 = fn48( nodeVar8 );
	nodeVar9 = ( nodeVar8 - f32( nodeVar3 ) );
	nodeVar10 = fn65( nodeVar5 );
	nodeVar11 = fn65( nodeVar7 );
	nodeVar12 = fn65( nodeVar9 );
	nodeVar13 = fn52( fn56( fn72( nodeVar1, nodeVar2, nodeVar3 ), nodeVar5, nodeVar7, nodeVar9 ), fn56( fn72( ( nodeVar1 + 1 ), nodeVar2, nodeVar3 ), ( nodeVar5 - 1.0 ), nodeVar7, nodeVar9 ), fn56( fn72( nodeVar1, ( nodeVar2 + 1 ), nodeVar3 ), nodeVar5, ( nodeVar7 - 1.0 ), nodeVar9 ), fn56( fn72( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ), nodeVar3 ), ( nodeVar5 - 1.0 ), ( nodeVar7 - 1.0 ), nodeVar9 ), fn56( fn72( nodeVar1, nodeVar2, ( nodeVar3 + 1 ) ), nodeVar5, nodeVar7, ( nodeVar9 - 1.0 ) ), fn56( fn72( ( nodeVar1 + 1 ), nodeVar2, ( nodeVar3 + 1 ) ), ( nodeVar5 - 1.0 ), nodeVar7, ( nodeVar9 - 1.0 ) ), fn56( fn72( nodeVar1, ( nodeVar2 + 1 ), ( nodeVar3 + 1 ) ), nodeVar5, ( nodeVar7 - 1.0 ), ( nodeVar9 - 1.0 ) ), fn56( fn72( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ), ( nodeVar3 + 1 ) ), ( nodeVar5 - 1.0 ), ( nodeVar7 - 1.0 ), ( nodeVar9 - 1.0 ) ), nodeVar10, nodeVar11, nodeVar12 );

	return fn60( nodeVar13 );

}


fn fn88 ( p : vec3<f32>, octaves : i32, lacunarity : f32, diminish : f32 ) -> vec3<f32> {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : vec3<f32>;
	var nodeVar3 : vec3<f32>;
	var nodeVar4 : f32;
	var nodeVar5 : i32;

	nodeVar0 = diminish;
	nodeVar1 = lacunarity;
	nodeVar2 = p;
	nodeVar3 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar4 = 1.0;
	nodeVar5 = octaves;

	for ( var i : i32 = 0; i < nodeVar5; i ++ ) {

		nodeVar3 = ( nodeVar3 + ( vec3<f32>( nodeVar4 ) * fn76( nodeVar2 ) ) );
		nodeVar4 = ( nodeVar4 * nodeVar0 );
		nodeVar2 = ( nodeVar2 * vec3<f32>( nodeVar1 ) );

	}


	return nodeVar3;

}




@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_normalViewGeometry : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( mix( vec3<f32>( 0.4, 0.7, 0.3 ), vec3<f32>( 0.6, 0.5, 0.3 ), clamp( ( fn88( ( v_positionWorld * vec3<f32>( 0.05 ) ), 3, 2.0, 0.5 ) * vec3<f32>( 1.0 ) ), vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) ).x ), 1.0 );
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
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst1 = ( render.nodeUniform13 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform14 ) ) ), 1.0 ) );
	let nodeConst2 = ( nodeConst1.xyz / vec3<f32>( nodeConst1.w ) );
	let nodeConst3 = vec3<f32>( nodeConst2.x, ( 1.0 - nodeConst2.y ), ( nodeConst2.z + render.nodeUniform15 ) );
	let nodeConst4 = ( nodeConst3.xy * vec2<f32>( 2.0, 2.0 ) );
	let nodeConst5 = clamp( floor( nodeConst4 ), vec2<f32>( 0.0, 0.0 ), vec2<f32>( 1.0, 1.0 ) );
	let nodeConst6 = vec3<f32>( ( nodeConst4 - nodeConst5 ), nodeConst3.z );

	if ( ( ( ( ( ( nodeConst6.x >= 0.0 ) && ( nodeConst6.x <= 1.0 ) ) && ( nodeConst6.y >= 0.0 ) ) && ( nodeConst6.y <= 1.0 ) ) && ( nodeConst6.z <= 1.0 ) ) ) {

		nodeVar1 = textureSampleCompare( nodeUniform16, nodeUniform16_sampler, nodeConst6.xy, i32( ( ( nodeConst5.y * 2.0 ) + nodeConst5.x ) ), nodeConst6.z );
		nodeVar0 = nodeVar1;

	} else {

		nodeVar0 = 1.0;

	}

	shadowValue = mix( 1.0, nodeVar0, render.nodeUniform17 );
	let nodeConst7 = ( vec3<f32>( clamp( dot( normalView, nodeConst0 ), 0.0, 1.0 ) ) * ( render.nodeUniform12 * vec3<f32>( shadowValue ) ) );
	directDiffuse = ( directDiffuse + ( nodeConst7 * ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	positionViewDirection = normalize( v_positionViewDirection );
	let nodeConst8 = normalize( ( nodeConst0 + positionViewDirection ) );
	let nodeConst9 = clamp( dot( positionViewDirection, nodeConst8 ), 0.0, 1.0 );
	let nodeConst10 = exp2( ( ( ( nodeConst9 * -5.55473 ) - 6.98316 ) * nodeConst9 ) );
	directSpecular = ( directSpecular + ( ( nodeConst7 * ( ( ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst10 ) ) ) + vec3<f32>( ( 1.0 * nodeConst10 ) ) ) * vec3<f32>( 0.25 ) ) * vec3<f32>( ( ( ( ( Shininess * 0.5 ) + 1.0 ) * 0.3183098861837907 ) * pow( clamp( dot( normalView, nodeConst8 ), 0.0, 1.0 ), Shininess ) ) ) ) ) * vec3<f32>( 1.0 ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( vec4<f32>( indirectDiffuse, 1.0 ) + ( vec4<f32>( irradiance, 1.0 ) * ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) ) ) ).xyz;
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	Output = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	nodeVar2 = vec4<f32>( mix( Output.xyz, render.nodeUniform18, smoothstep( render.nodeUniform19, render.nodeUniform20, ( - v_positionView.z ) ) ), Output.w );
	let nodeConst11 = nodeVar2;
	Output = nodeConst11;

	// result

	output.color = nodeConst11;

	return output;

}
