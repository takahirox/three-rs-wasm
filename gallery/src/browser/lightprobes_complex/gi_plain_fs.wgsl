// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform8_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform8 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform18_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform18 : texture_3d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform25_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform25 : texture_3d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform20 : vec3<f32>,
	nodeUniform21 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform23 : f32,
	nodeUniform24 : f32,
	nodeUniform26 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform28 : vec3<f32>,
	nodeUniform29 : f32,
	nodeUniform30 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform13 : f32,
	nodeUniform15 : vec3<f32>,
	nodeUniform16 : f32,
	nodeUniform17 : f32,
	nodeUniform10 : vec3<f32>,
	nodeUniform14 : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : vec2<f32>;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : vec3<f32>;
var<private> nodeVar38 : vec3<f32>;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec4<f32>;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec4<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec4<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec4<f32>;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec4<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec4<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : f32;
var<private> nodeVar120 : f32;
var<private> nodeVar121 : f32;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec4<f32>;
var<private> nodeVar124 : array< vec3<f32>, 9 >;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : f32;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : f32;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : f32;
var<private> nodeVar155 : f32;
var<private> nodeVar156 : f32;
var<private> nodeVar157 : f32;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec4<f32>;
var<private> nodeVar160 : f32;
var<private> nodeVar161 : f32;
var<private> nodeVar162 : f32;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec4<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : f32;
var<private> nodeVar167 : f32;
var<private> nodeVar168 : f32;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec4<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : f32;
var<private> nodeVar173 : f32;
var<private> nodeVar174 : f32;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : vec4<f32>;
var<private> nodeVar177 : f32;
var<private> nodeVar178 : f32;
var<private> nodeVar179 : f32;
var<private> nodeVar180 : vec3<f32>;
var<private> nodeVar181 : vec4<f32>;
var<private> nodeVar182 : vec3<f32>;
var<private> nodeVar183 : f32;
var<private> nodeVar184 : f32;
var<private> nodeVar185 : f32;
var<private> nodeVar186 : vec3<f32>;
var<private> nodeVar187 : vec4<f32>;
var<private> nodeVar188 : vec3<f32>;
var<private> nodeVar189 : f32;
var<private> nodeVar190 : f32;
var<private> nodeVar191 : f32;
var<private> nodeVar192 : vec3<f32>;
var<private> nodeVar193 : vec4<f32>;
var<private> nodeVar194 : array< vec3<f32>, 9 >;
var<private> nodeVar195 : vec3<f32>;
var<private> nodeVar196 : vec3<f32>;
var<private> nodeVar197 : vec3<f32>;
var<private> nodeVar198 : vec3<f32>;
var<private> nodeVar199 : vec3<f32>;
var<private> nodeVar200 : vec3<f32>;
var<private> nodeVar201 : vec3<f32>;
var<private> nodeVar202 : vec3<f32>;
var<private> nodeVar203 : f32;
var<private> nodeVar204 : f32;
var<private> nodeVar205 : f32;
var<private> nodeVar206 : f32;
var<private> nodeVar207 : vec3<f32>;
var<private> nodeVar208 : vec3<f32>;
var<private> nodeVar209 : vec3<f32>;
var<private> nodeVar210 : vec3<f32>;
var<private> nodeVar211 : vec3<f32>;
var<private> nodeVar212 : f32;
var<private> nodeVar213 : vec3<f32>;
var<private> nodeVar214 : vec3<f32>;
var<private> nodeVar215 : vec3<f32>;
var<private> nodeVar216 : vec3<f32>;
var<private> nodeVar217 : vec3<f32>;
var<private> nodeVar218 : vec3<f32>;
var<private> nodeVar219 : vec3<f32>;
var<private> nodeVar220 : f32;
var<private> nodeVar221 : f32;
var<private> nodeVar222 : f32;
var<private> nodeVar223 : vec3<f32>;
var<private> nodeVar224 : vec3<f32>;
var<private> nodeVar225 : vec3<f32>;
var<private> nodeVar226 : vec3<f32>;
var<private> nodeVar227 : vec3<f32>;
var<private> nodeVar228 : vec3<f32>;
var<private> nodeVar229 : vec3<f32>;
var<private> nodeVar230 : vec3<f32>;
var<private> nodeVar231 : vec3<f32>;
var<private> nodeVar232 : vec3<f32>;
var<private> nodeVar233 : vec3<f32>;
var<private> nodeVar234 : vec3<f32>;
var<private> nodeVar235 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar236 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar237 : vec3<f32>;
var<private> nodeVar238 : f32;
var<private> nodeVar239 : vec3<f32>;
var<private> nodeVar240 : vec3<f32>;
var<private> nodeVar241 : vec3<f32>;
var<private> nodeVar242 : vec3<f32>;
var<private> nodeVar243 : vec3<f32>;
var<private> nodeVar244 : vec3<f32>;
var<private> nodeVar245 : vec3<f32>;
var<private> nodeVar246 : f32;
var<private> nodeVar247 : f32;
var<private> nodeVar248 : f32;
var<private> nodeVar249 : vec3<f32>;
var<private> nodeVar250 : vec3<f32>;
var<private> nodeVar251 : vec3<f32>;
var<private> nodeVar252 : vec3<f32>;
var<private> nodeVar253 : vec3<f32>;
var<private> nodeVar254 : vec3<f32>;
var<private> nodeVar255 : vec3<f32>;
var<private> nodeVar256 : f32;
var<private> nodeVar257 : vec3<f32>;
var<private> nodeVar258 : vec3<f32>;
var<private> nodeVar259 : vec3<f32>;
var<private> nodeVar260 : vec3<f32>;
var<private> nodeVar261 : vec3<f32>;
var<private> nodeVar262 : vec3<f32>;
var<private> nodeVar263 : vec3<f32>;
var<private> nodeVar264 : f32;
var<private> nodeVar265 : f32;
var<private> nodeVar266 : f32;
var<private> nodeVar267 : vec3<f32>;
var<private> nodeVar268 : vec3<f32>;
var<private> nodeVar269 : vec3<f32>;
var<private> nodeVar270 : vec3<f32>;
var<private> nodeVar271 : vec3<f32>;
var<private> nodeVar272 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar273 : vec3<f32>;
var<private> nodeVar274 : vec3<f32>;
var<private> nodeVar275 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar276 : vec3<f32>;
var<private> nodeVar277 : vec3<f32>;
var<private> nodeVar278 : vec3<f32>;
var<private> nodeVar279 : vec3<f32>;
var<private> nodeVar280 : vec3<f32>;
var<private> nodeVar281 : vec3<f32>;
var<private> nodeVar282 : vec3<f32>;
var<private> nodeVar283 : vec3<f32>;
var<private> nodeVar284 : vec3<f32>;
var<private> nodeVar285 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar286 : vec3<f32>;
var<private> nodeVar287 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar288 : vec3<f32>;
var<private> nodeVar289 : f32;
var<private> nodeVar290 : f32;
var<private> nodeVar291 : f32;
var<private> nodeVar292 : f32;
var<private> nodeVar293 : f32;
var<private> nodeVar294 : f32;
var<private> nodeVar295 : f32;
var<private> nodeVar296 : f32;
var<private> nodeVar297 : f32;
var<private> nodeVar298 : f32;
var<private> nodeVar299 : f32;
var<private> nodeVar300 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar301 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar302 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar303 : vec3<f32>;
var<private> nodeVar304 : vec4<f32>;

// codes
fn V_GGX_SmithCorrelated ( alpha : f32, dotNL : f32, dotNV : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = ( alpha * alpha );

	return ( 0.5 / max( ( ( dotNL * sqrt( ( nodeVar0 + ( ( 1.0 - nodeVar0 ) * ( dotNV * dotNV ) ) ) ) ) + ( dotNV * sqrt( ( nodeVar0 + ( ( 1.0 - nodeVar0 ) * ( dotNL * dotNL ) ) ) ) ) ), 0.000001 ) );

}


fn D_GGX ( alpha : f32, dotNH : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;

	nodeVar0 = ( alpha * alpha );
	nodeVar1 = ( 1.0 - ( ( dotNH * dotNH ) * ( 1.0 - nodeVar0 ) ) );

	return ( ( nodeVar0 / ( nodeVar1 * nodeVar1 ) ) * 0.3183098861837907 );

}




@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	Metalness = object.nodeUniform2;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform3, 0.0525 ) + max( max( nodeVar0.x, nodeVar0.y ), nodeVar0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar1 = dot( normalView, positionViewDirection );
	nodeVar2 = textureSample( nodeUniform8, nodeUniform8_sampler, vec2<f32>( Roughness, clamp( nodeVar1, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar2;
	nodeVar3 = ( dfg.x + dfg.y );
	nodeVar4 = ( 1.0 / nodeVar3 );
	nodeVar5 = nodeVar4;
	nodeVar6 = ( nodeVar5 - 1.0 );
	nodeVar7 = ( SpecularColorBlended * vec3<f32>( nodeVar6 ) );
	nodeVar8 = ( nodeVar7 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar8;
	nodeVar9 = ( render.nodeUniform10 - v_positionView );
	nodeVar10 = normalize( nodeVar9 );
	nodeVar11 = dot( normalView, nodeVar10 );

	if ( ( render.nodeUniform12 > 0.0 ) ) {

		nodeVar13 = length( nodeVar9 );
		nodeVar14 = ( nodeVar13 / render.nodeUniform12 );
		nodeVar15 = clamp( ( 1.0 - ( ( ( nodeVar14 * nodeVar14 ) * nodeVar14 ) * nodeVar14 ) ), 0.0, 1.0 );
		nodeVar12 = ( ( 1.0 / max( pow( nodeVar13, render.nodeUniform13 ), 0.01 ) ) * ( nodeVar15 * nodeVar15 ) );

	} else {

		nodeVar12 = ( 1.0 / max( pow( length( nodeVar9 ), render.nodeUniform13 ), 0.01 ) );

	}

	nodeVar16 = ( render.nodeUniform11 * vec3<f32>( nodeVar12 ) );
	nodeVar17 = ( vec3<f32>( clamp( nodeVar11, 0.0, 1.0 ) ) * nodeVar16 );
	nodeVar18 = nodeVar17;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar19 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar20 = ( nodeVar18 * nodeVar19 );
	nodeVar21 = ( nodeVar10 + positionViewDirection );
	nodeVar22 = normalize( nodeVar21 );
	nodeVar23 = dot( positionViewDirection, nodeVar22 );
	nodeVar24 = clamp( nodeVar23, 0.0, 1.0 );
	nodeVar25 = exp2( ( ( ( nodeVar24 * -5.55473 ) - 6.98316 ) * nodeVar24 ) );
	nodeVar26 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar25 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar25 ) ) );
	nodeVar27 = ( vec3<f32>( 1.0 ) - nodeVar26 );
	nodeVar28 = nodeVar27;
	nodeVar29 = ( nodeVar20 * nodeVar28 );
	nodeVar30 = ( directDiffuse + nodeVar29 );
	directDiffuse = nodeVar30;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar31 = normalize( ( nodeVar10 + positionViewDirection ) );
	nodeVar32 = clamp( dot( positionViewDirection, nodeVar31 ), 0.0, 1.0 );
	nodeVar33 = exp2( ( ( ( nodeVar32 * -5.55473 ) - 6.98316 ) * nodeVar32 ) );
	nodeVar34 = ( Roughness * Roughness );
	nodeVar35 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar33 ) ) ) + vec3<f32>( ( 1.0 * nodeVar33 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar34, clamp( dot( normalView, nodeVar10 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar34, clamp( dot( normalView, nodeVar31 ), 0.0, 1.0 ) ) ) );
	nodeVar36 = ( nodeVar18 * nodeVar35 );
	nodeVar37 = ( nodeVar36 * multiScatteringCompensation );
	nodeVar38 = ( directSpecular + nodeVar37 );
	directSpecular = nodeVar38;
	nodeVar39 = ( render.nodeUniform14 - v_positionView );
	nodeVar40 = normalize( nodeVar39 );
	nodeVar41 = dot( normalView, nodeVar40 );

	if ( ( render.nodeUniform16 > 0.0 ) ) {

		nodeVar43 = length( nodeVar39 );
		nodeVar44 = ( nodeVar43 / render.nodeUniform16 );
		nodeVar45 = clamp( ( 1.0 - ( ( ( nodeVar44 * nodeVar44 ) * nodeVar44 ) * nodeVar44 ) ), 0.0, 1.0 );
		nodeVar42 = ( ( 1.0 / max( pow( nodeVar43, render.nodeUniform17 ), 0.01 ) ) * ( nodeVar45 * nodeVar45 ) );

	} else {

		nodeVar42 = ( 1.0 / max( pow( length( nodeVar39 ), render.nodeUniform17 ), 0.01 ) );

	}

	nodeVar46 = ( render.nodeUniform15 * vec3<f32>( nodeVar42 ) );
	nodeVar47 = ( vec3<f32>( clamp( nodeVar41, 0.0, 1.0 ) ) * nodeVar46 );
	nodeVar48 = nodeVar47;
	nodeVar49 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar50 = ( nodeVar48 * nodeVar49 );
	nodeVar51 = ( nodeVar40 + positionViewDirection );
	nodeVar52 = normalize( nodeVar51 );
	nodeVar53 = dot( positionViewDirection, nodeVar52 );
	nodeVar54 = clamp( nodeVar53, 0.0, 1.0 );
	nodeVar55 = exp2( ( ( ( nodeVar54 * -5.55473 ) - 6.98316 ) * nodeVar54 ) );
	nodeVar56 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar55 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar55 ) ) );
	nodeVar57 = ( vec3<f32>( 1.0 ) - nodeVar56 );
	nodeVar58 = nodeVar57;
	nodeVar59 = ( nodeVar50 * nodeVar58 );
	nodeVar60 = ( directDiffuse + nodeVar59 );
	directDiffuse = nodeVar60;
	nodeVar61 = normalize( ( nodeVar40 + positionViewDirection ) );
	nodeVar62 = clamp( dot( positionViewDirection, nodeVar61 ), 0.0, 1.0 );
	nodeVar63 = exp2( ( ( ( nodeVar62 * -5.55473 ) - 6.98316 ) * nodeVar62 ) );
	nodeVar64 = ( Roughness * Roughness );
	nodeVar65 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar63 ) ) ) + vec3<f32>( ( 1.0 * nodeVar63 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar64, clamp( dot( normalView, nodeVar40 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar64, clamp( dot( normalView, nodeVar61 ), 0.0, 1.0 ) ) ) );
	nodeVar66 = ( nodeVar48 * nodeVar65 );
	nodeVar67 = ( nodeVar66 * multiScatteringCompensation );
	nodeVar68 = ( directSpecular + nodeVar67 );
	directSpecular = nodeVar68;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar69 = ( object.nodeUniform20 - object.nodeUniform21 );
	nodeVar70 = ( object.nodeUniform22 - vec3<f32>( 1.0 ) );
	nodeVar71 = ( nodeVar69 / nodeVar70 );
	nodeVar72 = ( normalWorld * nodeVar71 );
	nodeVar73 = ( nodeVar72 * vec3<f32>( 0.5 ) );
	nodeVar74 = ( v_positionWorld + nodeVar73 );
	nodeVar75 = ( nodeVar74 - object.nodeUniform21 );
	nodeVar76 = ( nodeVar75 / nodeVar69 );
	nodeVar77 = ( clamp( nodeVar76, vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) ) * nodeVar70 );
	nodeVar78 = ( nodeVar77 / object.nodeUniform22 );
	nodeVar79 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform22 );
	nodeVar80 = ( nodeVar78 + nodeVar79 );
	nodeVar81 = ( nodeVar80.z * object.nodeUniform22.z );
	nodeVar82 = ( nodeVar81 + 1.0 );
	nodeVar83 = ( object.nodeUniform22.z + 2.0 );
	nodeVar84 = ( nodeVar83 * 0.0 );
	nodeVar85 = ( nodeVar82 + nodeVar84 );
	nodeVar86 = ( nodeVar83 * 7.0 );
	nodeVar87 = ( nodeVar85 / nodeVar86 );
	nodeVar88 = vec3<f32>( nodeVar80.xy, nodeVar87 );
	nodeVar89 = textureSample( nodeUniform18, nodeUniform18_sampler, nodeVar88 );
	nodeVar90 = ( nodeVar83 * 1.0 );
	nodeVar91 = ( nodeVar82 + nodeVar90 );
	nodeVar92 = ( nodeVar91 / nodeVar86 );
	nodeVar93 = vec3<f32>( nodeVar80.xy, nodeVar92 );
	nodeVar94 = textureSample( nodeUniform18, nodeUniform18_sampler, nodeVar93 );
	nodeVar95 = vec3<f32>( nodeVar89.w, nodeVar94.xy );
	nodeVar96 = ( nodeVar83 * 2.0 );
	nodeVar97 = ( nodeVar82 + nodeVar96 );
	nodeVar98 = ( nodeVar97 / nodeVar86 );
	nodeVar99 = vec3<f32>( nodeVar80.xy, nodeVar98 );
	nodeVar100 = textureSample( nodeUniform18, nodeUniform18_sampler, nodeVar99 );
	nodeVar101 = vec3<f32>( nodeVar94.zw, nodeVar100.x );
	nodeVar102 = ( nodeVar83 * 3.0 );
	nodeVar103 = ( nodeVar82 + nodeVar102 );
	nodeVar104 = ( nodeVar103 / nodeVar86 );
	nodeVar105 = vec3<f32>( nodeVar80.xy, nodeVar104 );
	nodeVar106 = textureSample( nodeUniform18, nodeUniform18_sampler, nodeVar105 );
	nodeVar107 = ( nodeVar83 * 4.0 );
	nodeVar108 = ( nodeVar82 + nodeVar107 );
	nodeVar109 = ( nodeVar108 / nodeVar86 );
	nodeVar110 = vec3<f32>( nodeVar80.xy, nodeVar109 );
	nodeVar111 = textureSample( nodeUniform18, nodeUniform18_sampler, nodeVar110 );
	nodeVar112 = vec3<f32>( nodeVar106.w, nodeVar111.xy );
	nodeVar113 = ( nodeVar83 * 5.0 );
	nodeVar114 = ( nodeVar82 + nodeVar113 );
	nodeVar115 = ( nodeVar114 / nodeVar86 );
	nodeVar116 = vec3<f32>( nodeVar80.xy, nodeVar115 );
	nodeVar117 = textureSample( nodeUniform18, nodeUniform18_sampler, nodeVar116 );
	nodeVar118 = vec3<f32>( nodeVar111.zw, nodeVar117.x );
	nodeVar119 = ( nodeVar83 * 6.0 );
	nodeVar120 = ( nodeVar82 + nodeVar119 );
	nodeVar121 = ( nodeVar120 / nodeVar86 );
	nodeVar122 = vec3<f32>( nodeVar80.xy, nodeVar121 );
	nodeVar123 = textureSample( nodeUniform18, nodeUniform18_sampler, nodeVar122 );
	nodeVar124 = array< vec3<f32>, 9 >( nodeVar89.xyz, nodeVar95, nodeVar101, nodeVar100.yzw, nodeVar106.xyz, nodeVar112, nodeVar118, nodeVar117.yzw, nodeVar123.xyz );
	nodeVar125 = ( ( ( ( ( ( ( ( ( nodeVar124[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeVar124[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeVar124[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar124[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeVar124[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeVar124[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeVar124[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeVar124[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar124[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) );
	nodeVar126 = max( nodeVar125, vec3<f32>( 0.0, 0.0, 0.0 ) );
	nodeVar127 = ( nodeVar126 * vec3<f32>( object.nodeUniform23 ) );
	nodeVar128 = ( object.nodeUniform21 - v_positionWorld );
	nodeVar129 = max( nodeVar128, vec3<f32>( 0.0 ) );
	nodeVar130 = ( v_positionWorld - object.nodeUniform20 );
	nodeVar131 = max( nodeVar130, vec3<f32>( 0.0 ) );
	nodeVar132 = ( nodeVar129 + nodeVar131 );
	nodeVar133 = length( nodeVar132 );
	nodeVar134 = smoothstep( 0.0, object.nodeUniform24, nodeVar133 );
	nodeVar135 = ( 1.0 - nodeVar134 );
	nodeVar136 = nodeVar135;
	nodeVar137 = ( nodeVar127 * vec3<f32>( nodeVar136 ) );
	nodeVar138 = ( irradiance + nodeVar137 );
	irradiance = nodeVar138;
	nodeVar139 = ( object.nodeUniform26 - object.nodeUniform27 );
	nodeVar140 = ( object.nodeUniform28 - vec3<f32>( 1.0 ) );
	nodeVar141 = ( nodeVar139 / nodeVar140 );
	nodeVar142 = ( normalWorld * nodeVar141 );
	nodeVar143 = ( nodeVar142 * vec3<f32>( 0.5 ) );
	nodeVar144 = ( v_positionWorld + nodeVar143 );
	nodeVar145 = ( nodeVar144 - object.nodeUniform27 );
	nodeVar146 = ( nodeVar145 / nodeVar139 );
	nodeVar147 = ( clamp( nodeVar146, vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) ) * nodeVar140 );
	nodeVar148 = ( nodeVar147 / object.nodeUniform28 );
	nodeVar149 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform28 );
	nodeVar150 = ( nodeVar148 + nodeVar149 );
	nodeVar151 = ( nodeVar150.z * object.nodeUniform28.z );
	nodeVar152 = ( nodeVar151 + 1.0 );
	nodeVar153 = ( object.nodeUniform28.z + 2.0 );
	nodeVar154 = ( nodeVar153 * 0.0 );
	nodeVar155 = ( nodeVar152 + nodeVar154 );
	nodeVar156 = ( nodeVar153 * 7.0 );
	nodeVar157 = ( nodeVar155 / nodeVar156 );
	nodeVar158 = vec3<f32>( nodeVar150.xy, nodeVar157 );
	nodeVar159 = textureSample( nodeUniform25, nodeUniform25_sampler, nodeVar158 );
	nodeVar160 = ( nodeVar153 * 1.0 );
	nodeVar161 = ( nodeVar152 + nodeVar160 );
	nodeVar162 = ( nodeVar161 / nodeVar156 );
	nodeVar163 = vec3<f32>( nodeVar150.xy, nodeVar162 );
	nodeVar164 = textureSample( nodeUniform25, nodeUniform25_sampler, nodeVar163 );
	nodeVar165 = vec3<f32>( nodeVar159.w, nodeVar164.xy );
	nodeVar166 = ( nodeVar153 * 2.0 );
	nodeVar167 = ( nodeVar152 + nodeVar166 );
	nodeVar168 = ( nodeVar167 / nodeVar156 );
	nodeVar169 = vec3<f32>( nodeVar150.xy, nodeVar168 );
	nodeVar170 = textureSample( nodeUniform25, nodeUniform25_sampler, nodeVar169 );
	nodeVar171 = vec3<f32>( nodeVar164.zw, nodeVar170.x );
	nodeVar172 = ( nodeVar153 * 3.0 );
	nodeVar173 = ( nodeVar152 + nodeVar172 );
	nodeVar174 = ( nodeVar173 / nodeVar156 );
	nodeVar175 = vec3<f32>( nodeVar150.xy, nodeVar174 );
	nodeVar176 = textureSample( nodeUniform25, nodeUniform25_sampler, nodeVar175 );
	nodeVar177 = ( nodeVar153 * 4.0 );
	nodeVar178 = ( nodeVar152 + nodeVar177 );
	nodeVar179 = ( nodeVar178 / nodeVar156 );
	nodeVar180 = vec3<f32>( nodeVar150.xy, nodeVar179 );
	nodeVar181 = textureSample( nodeUniform25, nodeUniform25_sampler, nodeVar180 );
	nodeVar182 = vec3<f32>( nodeVar176.w, nodeVar181.xy );
	nodeVar183 = ( nodeVar153 * 5.0 );
	nodeVar184 = ( nodeVar152 + nodeVar183 );
	nodeVar185 = ( nodeVar184 / nodeVar156 );
	nodeVar186 = vec3<f32>( nodeVar150.xy, nodeVar185 );
	nodeVar187 = textureSample( nodeUniform25, nodeUniform25_sampler, nodeVar186 );
	nodeVar188 = vec3<f32>( nodeVar181.zw, nodeVar187.x );
	nodeVar189 = ( nodeVar153 * 6.0 );
	nodeVar190 = ( nodeVar152 + nodeVar189 );
	nodeVar191 = ( nodeVar190 / nodeVar156 );
	nodeVar192 = vec3<f32>( nodeVar150.xy, nodeVar191 );
	nodeVar193 = textureSample( nodeUniform25, nodeUniform25_sampler, nodeVar192 );
	nodeVar194 = array< vec3<f32>, 9 >( nodeVar159.xyz, nodeVar165, nodeVar171, nodeVar170.yzw, nodeVar176.xyz, nodeVar182, nodeVar188, nodeVar187.yzw, nodeVar193.xyz );
	nodeVar195 = ( ( ( ( ( ( ( ( ( nodeVar194[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeVar194[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeVar194[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar194[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeVar194[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeVar194[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeVar194[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeVar194[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar194[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) );
	nodeVar196 = max( nodeVar195, vec3<f32>( 0.0, 0.0, 0.0 ) );
	nodeVar197 = ( nodeVar196 * vec3<f32>( object.nodeUniform29 ) );
	nodeVar198 = ( object.nodeUniform27 - v_positionWorld );
	nodeVar199 = max( nodeVar198, vec3<f32>( 0.0 ) );
	nodeVar200 = ( v_positionWorld - object.nodeUniform26 );
	nodeVar201 = max( nodeVar200, vec3<f32>( 0.0 ) );
	nodeVar202 = ( nodeVar199 + nodeVar201 );
	nodeVar203 = length( nodeVar202 );
	nodeVar204 = smoothstep( 0.0, object.nodeUniform30, nodeVar203 );
	nodeVar205 = ( 1.0 - nodeVar204 );
	nodeVar206 = nodeVar205;
	nodeVar207 = ( nodeVar197 * vec3<f32>( nodeVar206 ) );
	nodeVar208 = ( irradiance + nodeVar207 );
	irradiance = nodeVar208;
	nodeVar209 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar210 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar211 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar212 = ( SpecularF90 * dfg.y );
	nodeVar213 = ( nodeVar211 + vec3<f32>( nodeVar212 ) );
	nodeVar214 = ( nodeVar209 + nodeVar213 );
	nodeVar209 = nodeVar214;
	nodeVar215 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar216 = nodeVar215;
	nodeVar217 = ( nodeVar216 * vec3<f32>( 0.047619 ) );
	nodeVar218 = ( SpecularColor + nodeVar217 );
	nodeVar219 = ( nodeVar213 * nodeVar218 );
	nodeVar220 = ( dfg.x + dfg.y );
	nodeVar221 = ( 1.0 - nodeVar220 );
	nodeVar222 = nodeVar221;
	nodeVar223 = ( vec3<f32>( nodeVar222 ) * nodeVar218 );
	nodeVar224 = ( vec3<f32>( 1.0 ) - nodeVar223 );
	nodeVar225 = nodeVar224;
	nodeVar226 = ( nodeVar219 / nodeVar225 );
	nodeVar227 = ( nodeVar226 * vec3<f32>( nodeVar222 ) );
	nodeVar228 = ( nodeVar210 + nodeVar227 );
	nodeVar210 = nodeVar228;
	nodeVar229 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar230 = ( irradiance * nodeVar229 );
	nodeVar231 = ( nodeVar209 + nodeVar210 );
	nodeVar232 = ( vec3<f32>( 1.0 ) - nodeVar231 );
	nodeVar233 = nodeVar232;
	nodeVar234 = ( nodeVar230 * nodeVar233 );
	nodeVar235 = nodeVar234;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar236 = ( indirectDiffuse + nodeVar235 );
	indirectDiffuse = nodeVar236;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar237 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar238 = ( SpecularF90 * dfg.y );
	nodeVar239 = ( nodeVar237 + vec3<f32>( nodeVar238 ) );
	nodeVar240 = ( singleScatteringDielectric + nodeVar239 );
	singleScatteringDielectric = nodeVar240;
	nodeVar241 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar242 = nodeVar241;
	nodeVar243 = ( nodeVar242 * vec3<f32>( 0.047619 ) );
	nodeVar244 = ( SpecularColor + nodeVar243 );
	nodeVar245 = ( nodeVar239 * nodeVar244 );
	nodeVar246 = ( dfg.x + dfg.y );
	nodeVar247 = ( 1.0 - nodeVar246 );
	nodeVar248 = nodeVar247;
	nodeVar249 = ( vec3<f32>( nodeVar248 ) * nodeVar244 );
	nodeVar250 = ( vec3<f32>( 1.0 ) - nodeVar249 );
	nodeVar251 = nodeVar250;
	nodeVar252 = ( nodeVar245 / nodeVar251 );
	nodeVar253 = ( nodeVar252 * vec3<f32>( nodeVar248 ) );
	nodeVar254 = ( multiScatteringDielectric + nodeVar253 );
	multiScatteringDielectric = nodeVar254;
	nodeVar255 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar256 = ( SpecularF90 * dfg.y );
	nodeVar257 = ( nodeVar255 + vec3<f32>( nodeVar256 ) );
	nodeVar258 = ( singleScatteringMetallic + nodeVar257 );
	singleScatteringMetallic = nodeVar258;
	nodeVar259 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar260 = nodeVar259;
	nodeVar261 = ( nodeVar260 * vec3<f32>( 0.047619 ) );
	nodeVar262 = ( DiffuseColor.xyz + nodeVar261 );
	nodeVar263 = ( nodeVar257 * nodeVar262 );
	nodeVar264 = ( dfg.x + dfg.y );
	nodeVar265 = ( 1.0 - nodeVar264 );
	nodeVar266 = nodeVar265;
	nodeVar267 = ( vec3<f32>( nodeVar266 ) * nodeVar262 );
	nodeVar268 = ( vec3<f32>( 1.0 ) - nodeVar267 );
	nodeVar269 = nodeVar268;
	nodeVar270 = ( nodeVar263 / nodeVar269 );
	nodeVar271 = ( nodeVar270 * vec3<f32>( nodeVar266 ) );
	nodeVar272 = ( multiScatteringMetallic + nodeVar271 );
	multiScatteringMetallic = nodeVar272;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar273 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar274 = ( radiance * nodeVar273 );
	nodeVar275 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar276 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar277 = ( nodeVar275 * nodeVar276 );
	nodeVar278 = ( nodeVar274 + nodeVar277 );
	nodeVar279 = nodeVar278;
	nodeVar280 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar281 = ( vec3<f32>( 1.0 ) - nodeVar280 );
	nodeVar282 = nodeVar281;
	nodeVar283 = ( DiffuseContribution * nodeVar282 );
	nodeVar284 = ( nodeVar283 * nodeVar276 );
	nodeVar285 = nodeVar284;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar286 = ( indirectSpecular + nodeVar279 );
	indirectSpecular = nodeVar286;
	nodeVar287 = ( indirectDiffuse + nodeVar285 );
	indirectDiffuse = nodeVar287;
	ambientOcclusion = 1.0;
	nodeVar288 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar288;
	nodeVar289 = dot( normalView, positionViewDirection );
	nodeVar290 = ( clamp( nodeVar289, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar291 = ( Roughness * -16.0 );
	nodeVar292 = ( 1.0 - nodeVar291 );
	nodeVar293 = nodeVar292;
	nodeVar294 = ( - nodeVar293 );
	nodeVar295 = exp2( nodeVar294 );
	nodeVar296 = pow( nodeVar290, nodeVar295 );
	nodeVar297 = ( 1.0 - nodeVar296 );
	nodeVar298 = nodeVar297;
	nodeVar299 = ( ambientOcclusion - nodeVar298 );
	nodeVar300 = ( indirectSpecular * vec3<f32>( clamp( nodeVar299, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar300;
	nodeVar301 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar301;
	nodeVar302 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar302;
	nodeVar303 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar303;
	nodeVar304 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar304;

	// result

	output.color = nodeVar304;

	return output;

}
