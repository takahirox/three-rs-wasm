// Three.js r187 - Node System

// global
diagnostic( off, derivative_uniformity );


// directives


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>,
	@builtin( frag_depth ) depth : f32
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 1 ) var nodeUniform0 : texture_depth_2d;
@binding( 1 ) @group( 1 ) var nodeUniform1_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;
@binding( 4 ) @group( 1 ) var nodeUniform3_sampler : sampler;
@binding( 5 ) @group( 1 ) var nodeUniform3 : texture_2d<f32>;
@binding( 6 ) @group( 1 ) var nodeUniform4_sampler : sampler;
@binding( 7 ) @group( 1 ) var nodeUniform4 : texture_2d<f32>;
@binding( 8 ) @group( 1 ) var nodeUniform7_sampler : sampler;
@binding( 9 ) @group( 1 ) var nodeUniform7 : texture_2d<f32>;
@binding( 10 ) @group( 1 ) var nodeUniform40_sampler : sampler;
@binding( 11 ) @group( 1 ) var nodeUniform40 : texture_cube<f32>;

struct objectStruct {
	nodeUniform2 : f32,
	nodeUniform5 : vec3<f32>,
	nodeUniform6 : f32,
	nodeUniform41 : mat4x4<f32>,
	nodeUniform43 : f32,
	nodeUniform44 : f32
};
@binding( 3 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform9 : vec3<f32>,
	nodeUniform10 : f32,
	nodeUniform11 : f32,
	nodeUniform13 : vec3<f32>,
	nodeUniform14 : f32,
	nodeUniform15 : f32,
	nodeUniform17 : vec3<f32>,
	nodeUniform18 : f32,
	nodeUniform19 : f32,
	nodeUniform21 : vec3<f32>,
	nodeUniform22 : f32,
	nodeUniform23 : f32,
	nodeUniform25 : vec3<f32>,
	nodeUniform26 : f32,
	nodeUniform27 : f32,
	nodeUniform29 : vec3<f32>,
	nodeUniform30 : f32,
	nodeUniform31 : f32,
	nodeUniform33 : vec3<f32>,
	nodeUniform34 : f32,
	nodeUniform35 : f32,
	nodeUniform37 : vec3<f32>,
	nodeUniform38 : f32,
	nodeUniform39 : f32,
	nodeUniform8 : vec3<f32>,
	nodeUniform12 : vec3<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform32 : vec3<f32>,
	nodeUniform36 : vec3<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> nodeVar0 : f32;
var<private> nodeVar1 : vec2<u32>;
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> Metalness : f32;
var<private> nodeVar3 : vec4<f32>;
var<private> Roughness : f32;
var<private> nodeVar4 : vec4<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar5 : vec2<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar14 : vec4<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar15 : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}

fn fn44 ( lightDistance : f32, cutoffDistance : f32, decayExponent : f32 ) -> f32 {

	var nodeVar0 : f32;


	if ( ( cutoffDistance > 0.0 ) ) {

		let nodeConst0 = ( 1.0 / max( pow( lightDistance, decayExponent ), 0.01 ) );
		let nodeConst1 = ( lightDistance / cutoffDistance );
		let nodeConst2 = clamp( ( 1.0 - ( ( ( nodeConst1 * nodeConst1 ) * nodeConst1 ) * nodeConst1 ) ), 0.0, 1.0 );
		nodeVar0 = ( nodeConst0 * ( nodeConst2 * nodeConst2 ) );

	} else {

		let nodeConst0 = ( 1.0 / max( pow( lightDistance, decayExponent ), 0.01 ) );
		nodeVar0 = nodeConst0;

	}


	return nodeVar0;

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
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar1 = textureDimensions( nodeUniform0, u32( 0 ) );
	nodeVar0 = textureLoad( nodeUniform0, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVarying0 ) * vec2<f32>( nodeVar1 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar1 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst0 = nodeVar0;
	output.depth = nodeConst0;

	if ( ( nodeConst0 >= 1.0 ) ) {

		discard;
		

	}

	nodeVar2 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVarying0 );
	let nodeConst1 = nodeVar2;
	DiffuseColor = nodeConst1;
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform2 );
	DiffuseColor.w = 1.0;
	nodeVar3 = textureSample( nodeUniform3, nodeUniform3_sampler, nodeVarying0 );
	let nodeConst2 = nodeVar3;
	let nodeConst3 = nodeConst2.w;
	Metalness = nodeConst3;
	nodeVar4 = textureSample( nodeUniform4, nodeUniform4_sampler, nodeVarying0 );
	let nodeConst4 = nodeVar4;
	Roughness = min( ( max( nodeConst4.w, 0.045 ) + 0.0 ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - nodeConst3 ) ) );
	EmissiveColor = ( object.nodeUniform5 * vec3<f32>( object.nodeUniform6 ) );
	nodeVar5 = textureSample( nodeUniform7, nodeUniform7_sampler, vec2<f32>( Roughness, clamp( dot( nodeConst4.xyz, normalize( ( - nodeConst2.xyz ) ) ), 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar5;
	let multiScatteringCompensation = ( ( SpecularColorBlended * vec3<f32>( ( ( 1.0 / ( dfg.x + dfg.y ) ) - 1.0 ) ) ) + vec3<f32>( 1.0 ) );
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst5 = ( ( SpecularColor * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringDielectric = ( singleScatteringDielectric + nodeConst5 );
	let nodeConst6 = ( SpecularColor + ( ( vec3<f32>( 1.0 ) - SpecularColor ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst7 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringDielectric = ( multiScatteringDielectric + ( ( ( nodeConst5 * nodeConst6 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst7 ) * nodeConst6 ) ) ) * vec3<f32>( nodeConst7 ) ) );
	let nodeConst8 = ( render.nodeUniform8 - nodeConst2.xyz );
	let nodeConst9 = normalize( nodeConst8 );
	nodeVar6 = ( vec3<f32>( clamp( dot( nodeConst4.xyz, nodeConst9 ), 0.0, 1.0 ) ) * ( render.nodeUniform9 * vec3<f32>( fn44( length( nodeConst8 ), render.nodeUniform10, render.nodeUniform11 ) ) ) );
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst10 = clamp( dot( normalize( ( - nodeConst2.xyz ) ), normalize( ( nodeConst9 + normalize( ( - nodeConst2.xyz ) ) ) ) ), 0.0, 1.0 );
	let nodeConst11 = exp2( ( ( ( nodeConst10 * -5.55473 ) - 6.98316 ) * nodeConst10 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar6 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst11 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst11 ) ) ) ) ) );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst12 = normalize( ( nodeConst9 + normalize( ( - nodeConst2.xyz ) ) ) );
	let nodeConst13 = clamp( dot( normalize( ( - nodeConst2.xyz ) ), nodeConst12 ), 0.0, 1.0 );
	let nodeConst14 = exp2( ( ( ( nodeConst13 * -5.55473 ) - 6.98316 ) * nodeConst13 ) );
	let nodeConst15 = max( Roughness, 0.045 );
	let nodeConst16 = ( nodeConst15 * nodeConst15 );
	directSpecular = ( directSpecular + ( ( nodeVar6 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst14 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst14 ) ) ) * vec3<f32>( fn110( nodeConst16, clamp( dot( nodeConst4.xyz, nodeConst9 ), 0.0, 1.0 ), clamp( dot( nodeConst4.xyz, normalize( ( - nodeConst2.xyz ) ) ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst16, clamp( dot( nodeConst4.xyz, nodeConst12 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst17 = ( render.nodeUniform12 - nodeConst2.xyz );
	let nodeConst18 = normalize( nodeConst17 );
	nodeVar7 = ( vec3<f32>( clamp( dot( nodeConst4.xyz, nodeConst18 ), 0.0, 1.0 ) ) * ( render.nodeUniform13 * vec3<f32>( fn44( length( nodeConst17 ), render.nodeUniform14, render.nodeUniform15 ) ) ) );
	let nodeConst19 = clamp( dot( normalize( ( - nodeConst2.xyz ) ), normalize( ( nodeConst18 + normalize( ( - nodeConst2.xyz ) ) ) ) ), 0.0, 1.0 );
	let nodeConst20 = exp2( ( ( ( nodeConst19 * -5.55473 ) - 6.98316 ) * nodeConst19 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar7 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst20 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst20 ) ) ) ) ) );
	let nodeConst21 = normalize( ( nodeConst18 + normalize( ( - nodeConst2.xyz ) ) ) );
	let nodeConst22 = clamp( dot( normalize( ( - nodeConst2.xyz ) ), nodeConst21 ), 0.0, 1.0 );
	let nodeConst23 = exp2( ( ( ( nodeConst22 * -5.55473 ) - 6.98316 ) * nodeConst22 ) );
	let nodeConst24 = max( Roughness, 0.045 );
	let nodeConst25 = ( nodeConst24 * nodeConst24 );
	directSpecular = ( directSpecular + ( ( nodeVar7 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst23 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst23 ) ) ) * vec3<f32>( fn110( nodeConst25, clamp( dot( nodeConst4.xyz, nodeConst18 ), 0.0, 1.0 ), clamp( dot( nodeConst4.xyz, normalize( ( - nodeConst2.xyz ) ) ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst25, clamp( dot( nodeConst4.xyz, nodeConst21 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst26 = ( render.nodeUniform16 - nodeConst2.xyz );
	let nodeConst27 = normalize( nodeConst26 );
	nodeVar8 = ( vec3<f32>( clamp( dot( nodeConst4.xyz, nodeConst27 ), 0.0, 1.0 ) ) * ( render.nodeUniform17 * vec3<f32>( fn44( length( nodeConst26 ), render.nodeUniform18, render.nodeUniform19 ) ) ) );
	let nodeConst28 = clamp( dot( normalize( ( - nodeConst2.xyz ) ), normalize( ( nodeConst27 + normalize( ( - nodeConst2.xyz ) ) ) ) ), 0.0, 1.0 );
	let nodeConst29 = exp2( ( ( ( nodeConst28 * -5.55473 ) - 6.98316 ) * nodeConst28 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar8 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst29 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst29 ) ) ) ) ) );
	let nodeConst30 = normalize( ( nodeConst27 + normalize( ( - nodeConst2.xyz ) ) ) );
	let nodeConst31 = clamp( dot( normalize( ( - nodeConst2.xyz ) ), nodeConst30 ), 0.0, 1.0 );
	let nodeConst32 = exp2( ( ( ( nodeConst31 * -5.55473 ) - 6.98316 ) * nodeConst31 ) );
	let nodeConst33 = max( Roughness, 0.045 );
	let nodeConst34 = ( nodeConst33 * nodeConst33 );
	directSpecular = ( directSpecular + ( ( nodeVar8 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst32 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst32 ) ) ) * vec3<f32>( fn110( nodeConst34, clamp( dot( nodeConst4.xyz, nodeConst27 ), 0.0, 1.0 ), clamp( dot( nodeConst4.xyz, normalize( ( - nodeConst2.xyz ) ) ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst34, clamp( dot( nodeConst4.xyz, nodeConst30 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst35 = ( render.nodeUniform20 - nodeConst2.xyz );
	let nodeConst36 = normalize( nodeConst35 );
	nodeVar9 = ( vec3<f32>( clamp( dot( nodeConst4.xyz, nodeConst36 ), 0.0, 1.0 ) ) * ( render.nodeUniform21 * vec3<f32>( fn44( length( nodeConst35 ), render.nodeUniform22, render.nodeUniform23 ) ) ) );
	let nodeConst37 = clamp( dot( normalize( ( - nodeConst2.xyz ) ), normalize( ( nodeConst36 + normalize( ( - nodeConst2.xyz ) ) ) ) ), 0.0, 1.0 );
	let nodeConst38 = exp2( ( ( ( nodeConst37 * -5.55473 ) - 6.98316 ) * nodeConst37 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar9 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst38 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst38 ) ) ) ) ) );
	let nodeConst39 = normalize( ( nodeConst36 + normalize( ( - nodeConst2.xyz ) ) ) );
	let nodeConst40 = clamp( dot( normalize( ( - nodeConst2.xyz ) ), nodeConst39 ), 0.0, 1.0 );
	let nodeConst41 = exp2( ( ( ( nodeConst40 * -5.55473 ) - 6.98316 ) * nodeConst40 ) );
	let nodeConst42 = max( Roughness, 0.045 );
	let nodeConst43 = ( nodeConst42 * nodeConst42 );
	directSpecular = ( directSpecular + ( ( nodeVar9 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst41 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst41 ) ) ) * vec3<f32>( fn110( nodeConst43, clamp( dot( nodeConst4.xyz, nodeConst36 ), 0.0, 1.0 ), clamp( dot( nodeConst4.xyz, normalize( ( - nodeConst2.xyz ) ) ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst43, clamp( dot( nodeConst4.xyz, nodeConst39 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst44 = ( render.nodeUniform24 - nodeConst2.xyz );
	let nodeConst45 = normalize( nodeConst44 );
	nodeVar10 = ( vec3<f32>( clamp( dot( nodeConst4.xyz, nodeConst45 ), 0.0, 1.0 ) ) * ( render.nodeUniform25 * vec3<f32>( fn44( length( nodeConst44 ), render.nodeUniform26, render.nodeUniform27 ) ) ) );
	let nodeConst46 = clamp( dot( normalize( ( - nodeConst2.xyz ) ), normalize( ( nodeConst45 + normalize( ( - nodeConst2.xyz ) ) ) ) ), 0.0, 1.0 );
	let nodeConst47 = exp2( ( ( ( nodeConst46 * -5.55473 ) - 6.98316 ) * nodeConst46 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar10 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst47 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst47 ) ) ) ) ) );
	let nodeConst48 = normalize( ( nodeConst45 + normalize( ( - nodeConst2.xyz ) ) ) );
	let nodeConst49 = clamp( dot( normalize( ( - nodeConst2.xyz ) ), nodeConst48 ), 0.0, 1.0 );
	let nodeConst50 = exp2( ( ( ( nodeConst49 * -5.55473 ) - 6.98316 ) * nodeConst49 ) );
	let nodeConst51 = max( Roughness, 0.045 );
	let nodeConst52 = ( nodeConst51 * nodeConst51 );
	directSpecular = ( directSpecular + ( ( nodeVar10 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst50 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst50 ) ) ) * vec3<f32>( fn110( nodeConst52, clamp( dot( nodeConst4.xyz, nodeConst45 ), 0.0, 1.0 ), clamp( dot( nodeConst4.xyz, normalize( ( - nodeConst2.xyz ) ) ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst52, clamp( dot( nodeConst4.xyz, nodeConst48 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst53 = ( render.nodeUniform28 - nodeConst2.xyz );
	let nodeConst54 = normalize( nodeConst53 );
	nodeVar11 = ( vec3<f32>( clamp( dot( nodeConst4.xyz, nodeConst54 ), 0.0, 1.0 ) ) * ( render.nodeUniform29 * vec3<f32>( fn44( length( nodeConst53 ), render.nodeUniform30, render.nodeUniform31 ) ) ) );
	let nodeConst55 = clamp( dot( normalize( ( - nodeConst2.xyz ) ), normalize( ( nodeConst54 + normalize( ( - nodeConst2.xyz ) ) ) ) ), 0.0, 1.0 );
	let nodeConst56 = exp2( ( ( ( nodeConst55 * -5.55473 ) - 6.98316 ) * nodeConst55 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar11 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst56 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst56 ) ) ) ) ) );
	let nodeConst57 = normalize( ( nodeConst54 + normalize( ( - nodeConst2.xyz ) ) ) );
	let nodeConst58 = clamp( dot( normalize( ( - nodeConst2.xyz ) ), nodeConst57 ), 0.0, 1.0 );
	let nodeConst59 = exp2( ( ( ( nodeConst58 * -5.55473 ) - 6.98316 ) * nodeConst58 ) );
	let nodeConst60 = max( Roughness, 0.045 );
	let nodeConst61 = ( nodeConst60 * nodeConst60 );
	directSpecular = ( directSpecular + ( ( nodeVar11 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst59 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst59 ) ) ) * vec3<f32>( fn110( nodeConst61, clamp( dot( nodeConst4.xyz, nodeConst54 ), 0.0, 1.0 ), clamp( dot( nodeConst4.xyz, normalize( ( - nodeConst2.xyz ) ) ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst61, clamp( dot( nodeConst4.xyz, nodeConst57 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst62 = ( render.nodeUniform32 - nodeConst2.xyz );
	let nodeConst63 = normalize( nodeConst62 );
	nodeVar12 = ( vec3<f32>( clamp( dot( nodeConst4.xyz, nodeConst63 ), 0.0, 1.0 ) ) * ( render.nodeUniform33 * vec3<f32>( fn44( length( nodeConst62 ), render.nodeUniform34, render.nodeUniform35 ) ) ) );
	let nodeConst64 = clamp( dot( normalize( ( - nodeConst2.xyz ) ), normalize( ( nodeConst63 + normalize( ( - nodeConst2.xyz ) ) ) ) ), 0.0, 1.0 );
	let nodeConst65 = exp2( ( ( ( nodeConst64 * -5.55473 ) - 6.98316 ) * nodeConst64 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar12 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst65 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst65 ) ) ) ) ) );
	let nodeConst66 = normalize( ( nodeConst63 + normalize( ( - nodeConst2.xyz ) ) ) );
	let nodeConst67 = clamp( dot( normalize( ( - nodeConst2.xyz ) ), nodeConst66 ), 0.0, 1.0 );
	let nodeConst68 = exp2( ( ( ( nodeConst67 * -5.55473 ) - 6.98316 ) * nodeConst67 ) );
	let nodeConst69 = max( Roughness, 0.045 );
	let nodeConst70 = ( nodeConst69 * nodeConst69 );
	directSpecular = ( directSpecular + ( ( nodeVar12 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst68 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst68 ) ) ) * vec3<f32>( fn110( nodeConst70, clamp( dot( nodeConst4.xyz, nodeConst63 ), 0.0, 1.0 ), clamp( dot( nodeConst4.xyz, normalize( ( - nodeConst2.xyz ) ) ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst70, clamp( dot( nodeConst4.xyz, nodeConst66 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	let nodeConst71 = ( render.nodeUniform36 - nodeConst2.xyz );
	let nodeConst72 = normalize( nodeConst71 );
	nodeVar13 = ( vec3<f32>( clamp( dot( nodeConst4.xyz, nodeConst72 ), 0.0, 1.0 ) ) * ( render.nodeUniform37 * vec3<f32>( fn44( length( nodeConst71 ), render.nodeUniform38, render.nodeUniform39 ) ) ) );
	let nodeConst73 = clamp( dot( normalize( ( - nodeConst2.xyz ) ), normalize( ( nodeConst72 + normalize( ( - nodeConst2.xyz ) ) ) ) ), 0.0, 1.0 );
	let nodeConst74 = exp2( ( ( ( nodeConst73 * -5.55473 ) - 6.98316 ) * nodeConst73 ) );
	directDiffuse = ( directDiffuse + ( ( nodeVar13 * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeConst74 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst74 ) ) ) ) ) );
	let nodeConst75 = normalize( ( nodeConst72 + normalize( ( - nodeConst2.xyz ) ) ) );
	let nodeConst76 = clamp( dot( normalize( ( - nodeConst2.xyz ) ), nodeConst75 ), 0.0, 1.0 );
	let nodeConst77 = exp2( ( ( ( nodeConst76 * -5.55473 ) - 6.98316 ) * nodeConst76 ) );
	let nodeConst78 = max( Roughness, 0.045 );
	let nodeConst79 = ( nodeConst78 * nodeConst78 );
	directSpecular = ( directSpecular + ( ( nodeVar13 * ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeConst77 ) ) ) + vec3<f32>( ( SpecularF90 * nodeConst77 ) ) ) * vec3<f32>( fn110( nodeConst79, clamp( dot( nodeConst4.xyz, nodeConst72 ), 0.0, 1.0 ), clamp( dot( nodeConst4.xyz, normalize( ( - nodeConst2.xyz ) ) ), 0.0, 1.0 ) ) ) ) * vec3<f32>( fn112( nodeConst79, clamp( dot( nodeConst4.xyz, nodeConst75 ), 0.0, 1.0 ) ) ) ) ) * multiScatteringCompensation ) );
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst80 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - normalize( ( - nodeConst2.xyz ) ) ), nodeConst4.xyz ), nodeConst4.xyz, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	let nodeConst81 = ( object.nodeUniform41 * vec4<f32>( nodeConst80, 1.0 ) );
	let nodeConst82 = clamp( Roughness, 0.0, 1.0 );
	nodeVar14 = textureSampleLevel( nodeUniform40, nodeUniform40_sampler, vec3<f32>( ( - nodeConst81.x ), nodeConst81.yz ), ( ( object.nodeUniform43 * nodeConst82 ) * ( 2.0 - nodeConst82 ) ) );
	radiance = ( radiance + ( nodeVar14.xyz * vec3<f32>( object.nodeUniform44 ) ) );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( nodeConst4.xyz, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst83 = ( object.nodeUniform41 * vec4<f32>( normalWorld, 1.0 ) );
	let nodeConst84 = clamp( 1.0, 0.0, 1.0 );
	nodeVar15 = textureSampleLevel( nodeUniform40, nodeUniform40_sampler, vec3<f32>( ( - nodeConst83.x ), nodeConst83.yz ), ( ( object.nodeUniform43 * nodeConst84 ) * ( 2.0 - nodeConst84 ) ) );
	iblIrradiance = ( iblIrradiance + ( ( nodeVar15.xyz * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform44 ) ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar16 = ( ( irradiance * ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) ) ) * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( indirectDiffuse + nodeVar16 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst85 = ( ( DiffuseColor.xyz * vec3<f32>( dfg.x ) ) + vec3<f32>( ( SpecularF90 * dfg.y ) ) );
	singleScatteringMetallic = ( singleScatteringMetallic + nodeConst85 );
	let nodeConst86 = ( DiffuseColor.xyz + ( ( vec3<f32>( 1.0 ) - DiffuseColor.xyz ) * vec3<f32>( 0.047619 ) ) );
	let nodeConst87 = ( 1.0 - ( dfg.x + dfg.y ) );
	multiScatteringMetallic = ( multiScatteringMetallic + ( ( ( nodeConst85 * nodeConst86 ) / ( vec3<f32>( 1.0 ) - ( vec3<f32>( nodeConst87 ) * nodeConst86 ) ) ) * vec3<f32>( nodeConst87 ) ) );
	let nodeConst88 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar17 = ( ( radiance * mix( singleScatteringDielectric, singleScatteringMetallic, Metalness ) ) + ( mix( multiScatteringDielectric, multiScatteringMetallic, Metalness ) * nodeConst88 ) );
	nodeVar18 = ( ( DiffuseContribution * ( vec3<f32>( 1.0 ) - ( singleScatteringDielectric + multiScatteringDielectric ) ) ) * nodeConst88 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = ( indirectSpecular + nodeVar17 );
	indirectDiffuse = ( indirectDiffuse + nodeVar18 );
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectSpecular = ( indirectSpecular * vec3<f32>( clamp( ( ambientOcclusion - ( 1.0 - pow( ( clamp( dot( nodeConst4.xyz, normalize( ( - nodeConst2.xyz ) ) ), 0.0, 1.0 ) + ambientOcclusion ), exp2( ( - ( 1.0 - ( Roughness * -16.0 ) ) ) ) ) ) ), 0.0, 1.0 ) ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst89 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst89;

	// result

	output.color = nodeConst89;

	return output;

}
