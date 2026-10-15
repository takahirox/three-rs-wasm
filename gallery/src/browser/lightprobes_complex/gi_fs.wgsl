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
@binding( 3 ) @group( 1 ) var nodeUniform18_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform18 : texture_depth_cube;
@binding( 5 ) @group( 1 ) var nodeUniform31_sampler : sampler_comparison;
@binding( 6 ) @group( 1 ) var nodeUniform31 : texture_depth_cube;
@binding( 7 ) @group( 1 ) var nodeUniform37_sampler : sampler;
@binding( 8 ) @group( 1 ) var nodeUniform37 : texture_3d<f32>;
@binding( 9 ) @group( 1 ) var nodeUniform43_sampler : sampler;
@binding( 10 ) @group( 1 ) var nodeUniform43 : texture_3d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform38 : vec3<f32>,
	nodeUniform39 : vec3<f32>,
	nodeUniform40 : vec3<f32>,
	nodeUniform41 : f32,
	nodeUniform42 : f32,
	nodeUniform44 : vec3<f32>,
	nodeUniform45 : vec3<f32>,
	nodeUniform46 : vec3<f32>,
	nodeUniform47 : f32,
	nodeUniform48 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform11 : vec3<f32>,
	nodeUniform22 : f32,
	nodeUniform23 : f32,
	nodeUniform25 : vec3<f32>,
	nodeUniform35 : f32,
	nodeUniform36 : f32,
	nodeUniform10 : vec3<f32>,
	nodeUniform24 : vec3<f32>,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform14 : f32,
	nodeUniform21 : f32,
	nodeUniform26 : mat4x4<f32>,
	nodeUniform27 : f32,
	nodeUniform34 : f32,
	nodeUniform16 : f32,
	nodeUniform15 : f32,
	nodeUniform17 : f32,
	nodeUniform19 : f32,
	nodeUniform20 : vec2<f32>,
	nodeUniform29 : f32,
	nodeUniform28 : f32,
	nodeUniform30 : f32,
	nodeUniform32 : f32,
	nodeUniform33 : vec2<f32>
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
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : vec2<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec2<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec2<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : vec2<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec2<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : vec3<f32>;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : vec2<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : vec2<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : vec2<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : vec2<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : vec2<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : f32;
var<private> nodeVar118 : f32;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : f32;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : f32;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec4<f32>;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec4<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec4<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : f32;
var<private> nodeVar157 : f32;
var<private> nodeVar158 : f32;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec4<f32>;
var<private> nodeVar161 : f32;
var<private> nodeVar162 : f32;
var<private> nodeVar163 : f32;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : vec4<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : f32;
var<private> nodeVar168 : f32;
var<private> nodeVar169 : f32;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : vec4<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : f32;
var<private> nodeVar174 : f32;
var<private> nodeVar175 : f32;
var<private> nodeVar176 : vec3<f32>;
var<private> nodeVar177 : vec4<f32>;
var<private> nodeVar178 : array< vec3<f32>, 9 >;
var<private> nodeVar179 : vec3<f32>;
var<private> nodeVar180 : vec3<f32>;
var<private> nodeVar181 : vec3<f32>;
var<private> nodeVar182 : vec3<f32>;
var<private> nodeVar183 : vec3<f32>;
var<private> nodeVar184 : vec3<f32>;
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> nodeVar187 : f32;
var<private> nodeVar188 : f32;
var<private> nodeVar189 : f32;
var<private> nodeVar190 : f32;
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : vec3<f32>;
var<private> nodeVar193 : vec3<f32>;
var<private> nodeVar194 : vec3<f32>;
var<private> nodeVar195 : vec3<f32>;
var<private> nodeVar196 : vec3<f32>;
var<private> nodeVar197 : vec3<f32>;
var<private> nodeVar198 : vec3<f32>;
var<private> nodeVar199 : vec3<f32>;
var<private> nodeVar200 : vec3<f32>;
var<private> nodeVar201 : vec3<f32>;
var<private> nodeVar202 : vec3<f32>;
var<private> nodeVar203 : vec3<f32>;
var<private> nodeVar204 : vec3<f32>;
var<private> nodeVar205 : f32;
var<private> nodeVar206 : f32;
var<private> nodeVar207 : f32;
var<private> nodeVar208 : f32;
var<private> nodeVar209 : f32;
var<private> nodeVar210 : f32;
var<private> nodeVar211 : f32;
var<private> nodeVar212 : vec3<f32>;
var<private> nodeVar213 : vec4<f32>;
var<private> nodeVar214 : f32;
var<private> nodeVar215 : f32;
var<private> nodeVar216 : f32;
var<private> nodeVar217 : vec3<f32>;
var<private> nodeVar218 : vec4<f32>;
var<private> nodeVar219 : vec3<f32>;
var<private> nodeVar220 : f32;
var<private> nodeVar221 : f32;
var<private> nodeVar222 : f32;
var<private> nodeVar223 : vec3<f32>;
var<private> nodeVar224 : vec4<f32>;
var<private> nodeVar225 : vec3<f32>;
var<private> nodeVar226 : f32;
var<private> nodeVar227 : f32;
var<private> nodeVar228 : f32;
var<private> nodeVar229 : vec3<f32>;
var<private> nodeVar230 : vec4<f32>;
var<private> nodeVar231 : f32;
var<private> nodeVar232 : f32;
var<private> nodeVar233 : f32;
var<private> nodeVar234 : vec3<f32>;
var<private> nodeVar235 : vec4<f32>;
var<private> nodeVar236 : vec3<f32>;
var<private> nodeVar237 : f32;
var<private> nodeVar238 : f32;
var<private> nodeVar239 : f32;
var<private> nodeVar240 : vec3<f32>;
var<private> nodeVar241 : vec4<f32>;
var<private> nodeVar242 : vec3<f32>;
var<private> nodeVar243 : f32;
var<private> nodeVar244 : f32;
var<private> nodeVar245 : f32;
var<private> nodeVar246 : vec3<f32>;
var<private> nodeVar247 : vec4<f32>;
var<private> nodeVar248 : array< vec3<f32>, 9 >;
var<private> nodeVar249 : vec3<f32>;
var<private> nodeVar250 : vec3<f32>;
var<private> nodeVar251 : vec3<f32>;
var<private> nodeVar252 : vec3<f32>;
var<private> nodeVar253 : vec3<f32>;
var<private> nodeVar254 : vec3<f32>;
var<private> nodeVar255 : vec3<f32>;
var<private> nodeVar256 : vec3<f32>;
var<private> nodeVar257 : f32;
var<private> nodeVar258 : f32;
var<private> nodeVar259 : f32;
var<private> nodeVar260 : f32;
var<private> nodeVar261 : vec3<f32>;
var<private> nodeVar262 : vec3<f32>;
var<private> nodeVar263 : vec3<f32>;
var<private> nodeVar264 : vec3<f32>;
var<private> nodeVar265 : vec3<f32>;
var<private> nodeVar266 : f32;
var<private> nodeVar267 : vec3<f32>;
var<private> nodeVar268 : vec3<f32>;
var<private> nodeVar269 : vec3<f32>;
var<private> nodeVar270 : vec3<f32>;
var<private> nodeVar271 : vec3<f32>;
var<private> nodeVar272 : vec3<f32>;
var<private> nodeVar273 : vec3<f32>;
var<private> nodeVar274 : f32;
var<private> nodeVar275 : f32;
var<private> nodeVar276 : f32;
var<private> nodeVar277 : vec3<f32>;
var<private> nodeVar278 : vec3<f32>;
var<private> nodeVar279 : vec3<f32>;
var<private> nodeVar280 : vec3<f32>;
var<private> nodeVar281 : vec3<f32>;
var<private> nodeVar282 : vec3<f32>;
var<private> nodeVar283 : vec3<f32>;
var<private> nodeVar284 : vec3<f32>;
var<private> nodeVar285 : vec3<f32>;
var<private> nodeVar286 : vec3<f32>;
var<private> nodeVar287 : vec3<f32>;
var<private> nodeVar288 : vec3<f32>;
var<private> nodeVar289 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar290 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar291 : vec3<f32>;
var<private> nodeVar292 : f32;
var<private> nodeVar293 : vec3<f32>;
var<private> nodeVar294 : vec3<f32>;
var<private> nodeVar295 : vec3<f32>;
var<private> nodeVar296 : vec3<f32>;
var<private> nodeVar297 : vec3<f32>;
var<private> nodeVar298 : vec3<f32>;
var<private> nodeVar299 : vec3<f32>;
var<private> nodeVar300 : f32;
var<private> nodeVar301 : f32;
var<private> nodeVar302 : f32;
var<private> nodeVar303 : vec3<f32>;
var<private> nodeVar304 : vec3<f32>;
var<private> nodeVar305 : vec3<f32>;
var<private> nodeVar306 : vec3<f32>;
var<private> nodeVar307 : vec3<f32>;
var<private> nodeVar308 : vec3<f32>;
var<private> nodeVar309 : vec3<f32>;
var<private> nodeVar310 : f32;
var<private> nodeVar311 : vec3<f32>;
var<private> nodeVar312 : vec3<f32>;
var<private> nodeVar313 : vec3<f32>;
var<private> nodeVar314 : vec3<f32>;
var<private> nodeVar315 : vec3<f32>;
var<private> nodeVar316 : vec3<f32>;
var<private> nodeVar317 : vec3<f32>;
var<private> nodeVar318 : f32;
var<private> nodeVar319 : f32;
var<private> nodeVar320 : f32;
var<private> nodeVar321 : vec3<f32>;
var<private> nodeVar322 : vec3<f32>;
var<private> nodeVar323 : vec3<f32>;
var<private> nodeVar324 : vec3<f32>;
var<private> nodeVar325 : vec3<f32>;
var<private> nodeVar326 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar327 : vec3<f32>;
var<private> nodeVar328 : vec3<f32>;
var<private> nodeVar329 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar330 : vec3<f32>;
var<private> nodeVar331 : vec3<f32>;
var<private> nodeVar332 : vec3<f32>;
var<private> nodeVar333 : vec3<f32>;
var<private> nodeVar334 : vec3<f32>;
var<private> nodeVar335 : vec3<f32>;
var<private> nodeVar336 : vec3<f32>;
var<private> nodeVar337 : vec3<f32>;
var<private> nodeVar338 : vec3<f32>;
var<private> nodeVar339 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar340 : vec3<f32>;
var<private> nodeVar341 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar342 : vec3<f32>;
var<private> nodeVar343 : f32;
var<private> nodeVar344 : f32;
var<private> nodeVar345 : f32;
var<private> nodeVar346 : f32;
var<private> nodeVar347 : f32;
var<private> nodeVar348 : f32;
var<private> nodeVar349 : f32;
var<private> nodeVar350 : f32;
var<private> nodeVar351 : f32;
var<private> nodeVar352 : f32;
var<private> nodeVar353 : f32;
var<private> nodeVar354 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar355 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar356 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar357 : vec3<f32>;
var<private> nodeVar358 : vec4<f32>;

// codes
fn interleavedGradientNoise ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn vogelDiskSample ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	var nodeVar0 : f32;

	nodeVar0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeVar0 ), sin( nodeVar0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

}


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
	@location( 3 ) v_positionWorld : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

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
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	let nodeConst0 = ( render.nodeUniform12 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform14 ) ) ), 1.0 ) ).xyz;
	let nodeConst1 = abs( nodeConst0 );
	nodeVar12 = 1.0;
	nodeVar13 = max( max( nodeConst1.x, nodeConst1.y ), nodeConst1.z );

	if ( ( ( ( nodeVar13 - render.nodeUniform15 ) <= 0.0 ) && ( ( nodeVar13 - render.nodeUniform16 ) >= 0.0 ) ) ) {

		nodeVar14 = ( - nodeVar13 );
		nodeVar15 = ( ( ( render.nodeUniform16 + nodeVar14 ) * render.nodeUniform15 ) / ( ( render.nodeUniform15 - render.nodeUniform16 ) * nodeVar14 ) );
		nodeVar15 = ( nodeVar15 + render.nodeUniform17 );
		nodeVar16 = normalize( nodeConst0 );
		nodeVar18 = abs( nodeVar16 );

		if ( ( nodeVar18.x > nodeVar18.z ) ) {

			nodeVar17 = vec3<f32>( 0.0, 1.0, 0.0 );

		} else {

			nodeVar17 = vec3<f32>( 1.0, 0.0, 0.0 );

		}

		nodeVar19 = normalize( cross( nodeVar16, nodeVar17 ) );
		nodeVar20 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar21 = vogelDiskSample( 0, 5, nodeVar20 );
		nodeVar22 = cross( nodeVar16, nodeVar19 );
		nodeVar23 = ( render.nodeUniform19 / render.nodeUniform20.x );
		nodeVar24 = ( nodeVar16 + ( ( ( nodeVar19 * vec3<f32>( nodeVar21.x ) ) + ( nodeVar22 * vec3<f32>( nodeVar21.y ) ) ) * vec3<f32>( nodeVar23 ) ) );
		nodeVar25 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, vec3<f32>( nodeVar24.x, ( - nodeVar24.y ), nodeVar24.z ), nodeVar15 );
		nodeVar26 = vogelDiskSample( 1, 5, nodeVar20 );
		nodeVar27 = ( nodeVar16 + ( ( ( nodeVar19 * vec3<f32>( nodeVar26.x ) ) + ( nodeVar22 * vec3<f32>( nodeVar26.y ) ) ) * vec3<f32>( nodeVar23 ) ) );
		nodeVar28 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, vec3<f32>( nodeVar27.x, ( - nodeVar27.y ), nodeVar27.z ), nodeVar15 );
		nodeVar29 = vogelDiskSample( 2, 5, nodeVar20 );
		nodeVar30 = ( nodeVar16 + ( ( ( nodeVar19 * vec3<f32>( nodeVar29.x ) ) + ( nodeVar22 * vec3<f32>( nodeVar29.y ) ) ) * vec3<f32>( nodeVar23 ) ) );
		nodeVar31 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, vec3<f32>( nodeVar30.x, ( - nodeVar30.y ), nodeVar30.z ), nodeVar15 );
		nodeVar32 = vogelDiskSample( 3, 5, nodeVar20 );
		nodeVar33 = ( nodeVar16 + ( ( ( nodeVar19 * vec3<f32>( nodeVar32.x ) ) + ( nodeVar22 * vec3<f32>( nodeVar32.y ) ) ) * vec3<f32>( nodeVar23 ) ) );
		nodeVar34 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, vec3<f32>( nodeVar33.x, ( - nodeVar33.y ), nodeVar33.z ), nodeVar15 );
		nodeVar35 = vogelDiskSample( 4, 5, nodeVar20 );
		nodeVar36 = ( nodeVar16 + ( ( ( nodeVar19 * vec3<f32>( nodeVar35.x ) ) + ( nodeVar22 * vec3<f32>( nodeVar35.y ) ) ) * vec3<f32>( nodeVar23 ) ) );
		nodeVar37 = textureSampleCompare( nodeUniform18, nodeUniform18_sampler, vec3<f32>( nodeVar36.x, ( - nodeVar36.y ), nodeVar36.z ), nodeVar15 );
		nodeVar12 = ( ( ( ( ( nodeVar25 + nodeVar28 ) + nodeVar31 ) + nodeVar34 ) + nodeVar37 ) * 0.2 );
		

	}

	nodeVar38 = mix( 1.0, nodeVar12, render.nodeUniform21 );

	if ( ( render.nodeUniform22 > 0.0 ) ) {

		nodeVar40 = length( nodeVar9 );
		nodeVar41 = ( nodeVar40 / render.nodeUniform22 );
		nodeVar42 = clamp( ( 1.0 - ( ( ( nodeVar41 * nodeVar41 ) * nodeVar41 ) * nodeVar41 ) ), 0.0, 1.0 );
		nodeVar39 = ( ( 1.0 / max( pow( nodeVar40, render.nodeUniform23 ), 0.01 ) ) * ( nodeVar42 * nodeVar42 ) );

	} else {

		nodeVar39 = ( 1.0 / max( pow( length( nodeVar9 ), render.nodeUniform23 ), 0.01 ) );

	}

	nodeVar43 = ( ( render.nodeUniform11 * vec3<f32>( nodeVar38 ) ) * vec3<f32>( nodeVar39 ) );
	nodeVar44 = ( vec3<f32>( clamp( nodeVar11, 0.0, 1.0 ) ) * nodeVar43 );
	nodeVar45 = nodeVar44;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar46 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar47 = ( nodeVar45 * nodeVar46 );
	nodeVar48 = ( nodeVar10 + positionViewDirection );
	nodeVar49 = normalize( nodeVar48 );
	nodeVar50 = dot( positionViewDirection, nodeVar49 );
	nodeVar51 = clamp( nodeVar50, 0.0, 1.0 );
	nodeVar52 = exp2( ( ( ( nodeVar51 * -5.55473 ) - 6.98316 ) * nodeVar51 ) );
	nodeVar53 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar52 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar52 ) ) );
	nodeVar54 = ( vec3<f32>( 1.0 ) - nodeVar53 );
	nodeVar55 = nodeVar54;
	nodeVar56 = ( nodeVar47 * nodeVar55 );
	nodeVar57 = ( directDiffuse + nodeVar56 );
	directDiffuse = nodeVar57;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar58 = normalize( ( nodeVar10 + positionViewDirection ) );
	nodeVar59 = clamp( dot( positionViewDirection, nodeVar58 ), 0.0, 1.0 );
	nodeVar60 = exp2( ( ( ( nodeVar59 * -5.55473 ) - 6.98316 ) * nodeVar59 ) );
	nodeVar61 = ( Roughness * Roughness );
	nodeVar62 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar60 ) ) ) + vec3<f32>( ( 1.0 * nodeVar60 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar61, clamp( dot( normalView, nodeVar10 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar61, clamp( dot( normalView, nodeVar58 ), 0.0, 1.0 ) ) ) );
	nodeVar63 = ( nodeVar45 * nodeVar62 );
	nodeVar64 = ( nodeVar63 * multiScatteringCompensation );
	nodeVar65 = ( directSpecular + nodeVar64 );
	directSpecular = nodeVar65;
	nodeVar66 = ( render.nodeUniform24 - v_positionView );
	nodeVar67 = normalize( nodeVar66 );
	nodeVar68 = dot( normalView, nodeVar67 );
	shadowPositionWorld = v_positionWorld;
	let nodeConst2 = ( render.nodeUniform26 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform27 ) ) ), 1.0 ) ).xyz;
	let nodeConst3 = abs( nodeConst2 );
	nodeVar69 = 1.0;
	nodeVar70 = max( max( nodeConst3.x, nodeConst3.y ), nodeConst3.z );

	if ( ( ( ( nodeVar70 - render.nodeUniform28 ) <= 0.0 ) && ( ( nodeVar70 - render.nodeUniform29 ) >= 0.0 ) ) ) {

		nodeVar71 = ( - nodeVar70 );
		nodeVar72 = ( ( ( render.nodeUniform29 + nodeVar71 ) * render.nodeUniform28 ) / ( ( render.nodeUniform28 - render.nodeUniform29 ) * nodeVar71 ) );
		nodeVar72 = ( nodeVar72 + render.nodeUniform30 );
		nodeVar73 = normalize( nodeConst2 );
		nodeVar75 = abs( nodeVar73 );

		if ( ( nodeVar75.x > nodeVar75.z ) ) {

			nodeVar74 = vec3<f32>( 0.0, 1.0, 0.0 );

		} else {

			nodeVar74 = vec3<f32>( 1.0, 0.0, 0.0 );

		}

		nodeVar76 = normalize( cross( nodeVar73, nodeVar74 ) );
		nodeVar77 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar78 = vogelDiskSample( 0, 5, nodeVar77 );
		nodeVar79 = cross( nodeVar73, nodeVar76 );
		nodeVar80 = ( render.nodeUniform32 / render.nodeUniform33.x );
		nodeVar81 = ( nodeVar73 + ( ( ( nodeVar76 * vec3<f32>( nodeVar78.x ) ) + ( nodeVar79 * vec3<f32>( nodeVar78.y ) ) ) * vec3<f32>( nodeVar80 ) ) );
		nodeVar82 = textureSampleCompare( nodeUniform31, nodeUniform31_sampler, vec3<f32>( nodeVar81.x, ( - nodeVar81.y ), nodeVar81.z ), nodeVar72 );
		nodeVar83 = vogelDiskSample( 1, 5, nodeVar77 );
		nodeVar84 = ( nodeVar73 + ( ( ( nodeVar76 * vec3<f32>( nodeVar83.x ) ) + ( nodeVar79 * vec3<f32>( nodeVar83.y ) ) ) * vec3<f32>( nodeVar80 ) ) );
		nodeVar85 = textureSampleCompare( nodeUniform31, nodeUniform31_sampler, vec3<f32>( nodeVar84.x, ( - nodeVar84.y ), nodeVar84.z ), nodeVar72 );
		nodeVar86 = vogelDiskSample( 2, 5, nodeVar77 );
		nodeVar87 = ( nodeVar73 + ( ( ( nodeVar76 * vec3<f32>( nodeVar86.x ) ) + ( nodeVar79 * vec3<f32>( nodeVar86.y ) ) ) * vec3<f32>( nodeVar80 ) ) );
		nodeVar88 = textureSampleCompare( nodeUniform31, nodeUniform31_sampler, vec3<f32>( nodeVar87.x, ( - nodeVar87.y ), nodeVar87.z ), nodeVar72 );
		nodeVar89 = vogelDiskSample( 3, 5, nodeVar77 );
		nodeVar90 = ( nodeVar73 + ( ( ( nodeVar76 * vec3<f32>( nodeVar89.x ) ) + ( nodeVar79 * vec3<f32>( nodeVar89.y ) ) ) * vec3<f32>( nodeVar80 ) ) );
		nodeVar91 = textureSampleCompare( nodeUniform31, nodeUniform31_sampler, vec3<f32>( nodeVar90.x, ( - nodeVar90.y ), nodeVar90.z ), nodeVar72 );
		nodeVar92 = vogelDiskSample( 4, 5, nodeVar77 );
		nodeVar93 = ( nodeVar73 + ( ( ( nodeVar76 * vec3<f32>( nodeVar92.x ) ) + ( nodeVar79 * vec3<f32>( nodeVar92.y ) ) ) * vec3<f32>( nodeVar80 ) ) );
		nodeVar94 = textureSampleCompare( nodeUniform31, nodeUniform31_sampler, vec3<f32>( nodeVar93.x, ( - nodeVar93.y ), nodeVar93.z ), nodeVar72 );
		nodeVar69 = ( ( ( ( ( nodeVar82 + nodeVar85 ) + nodeVar88 ) + nodeVar91 ) + nodeVar94 ) * 0.2 );
		

	}

	nodeVar95 = mix( 1.0, nodeVar69, render.nodeUniform34 );

	if ( ( render.nodeUniform35 > 0.0 ) ) {

		nodeVar97 = length( nodeVar66 );
		nodeVar98 = ( nodeVar97 / render.nodeUniform35 );
		nodeVar99 = clamp( ( 1.0 - ( ( ( nodeVar98 * nodeVar98 ) * nodeVar98 ) * nodeVar98 ) ), 0.0, 1.0 );
		nodeVar96 = ( ( 1.0 / max( pow( nodeVar97, render.nodeUniform36 ), 0.01 ) ) * ( nodeVar99 * nodeVar99 ) );

	} else {

		nodeVar96 = ( 1.0 / max( pow( length( nodeVar66 ), render.nodeUniform36 ), 0.01 ) );

	}

	nodeVar100 = ( ( render.nodeUniform25 * vec3<f32>( nodeVar95 ) ) * vec3<f32>( nodeVar96 ) );
	nodeVar101 = ( vec3<f32>( clamp( nodeVar68, 0.0, 1.0 ) ) * nodeVar100 );
	nodeVar102 = nodeVar101;
	nodeVar103 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar104 = ( nodeVar102 * nodeVar103 );
	nodeVar105 = ( nodeVar67 + positionViewDirection );
	nodeVar106 = normalize( nodeVar105 );
	nodeVar107 = dot( positionViewDirection, nodeVar106 );
	nodeVar108 = clamp( nodeVar107, 0.0, 1.0 );
	nodeVar109 = exp2( ( ( ( nodeVar108 * -5.55473 ) - 6.98316 ) * nodeVar108 ) );
	nodeVar110 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar109 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar109 ) ) );
	nodeVar111 = ( vec3<f32>( 1.0 ) - nodeVar110 );
	nodeVar112 = nodeVar111;
	nodeVar113 = ( nodeVar104 * nodeVar112 );
	nodeVar114 = ( directDiffuse + nodeVar113 );
	directDiffuse = nodeVar114;
	nodeVar115 = normalize( ( nodeVar67 + positionViewDirection ) );
	nodeVar116 = clamp( dot( positionViewDirection, nodeVar115 ), 0.0, 1.0 );
	nodeVar117 = exp2( ( ( ( nodeVar116 * -5.55473 ) - 6.98316 ) * nodeVar116 ) );
	nodeVar118 = ( Roughness * Roughness );
	nodeVar119 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar117 ) ) ) + vec3<f32>( ( 1.0 * nodeVar117 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar118, clamp( dot( normalView, nodeVar67 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar118, clamp( dot( normalView, nodeVar115 ), 0.0, 1.0 ) ) ) );
	nodeVar120 = ( nodeVar102 * nodeVar119 );
	nodeVar121 = ( nodeVar120 * multiScatteringCompensation );
	nodeVar122 = ( directSpecular + nodeVar121 );
	directSpecular = nodeVar122;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar123 = ( object.nodeUniform38 - object.nodeUniform39 );
	nodeVar124 = ( object.nodeUniform40 - vec3<f32>( 1.0 ) );
	nodeVar125 = ( nodeVar123 / nodeVar124 );
	nodeVar126 = ( normalWorld * nodeVar125 );
	nodeVar127 = ( nodeVar126 * vec3<f32>( 0.5 ) );
	nodeVar128 = ( v_positionWorld + nodeVar127 );
	nodeVar129 = ( nodeVar128 - object.nodeUniform39 );
	nodeVar130 = ( nodeVar129 / nodeVar123 );
	nodeVar131 = ( clamp( nodeVar130, vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) ) * nodeVar124 );
	nodeVar132 = ( nodeVar131 / object.nodeUniform40 );
	nodeVar133 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform40 );
	nodeVar134 = ( nodeVar132 + nodeVar133 );
	nodeVar135 = ( nodeVar134.z * object.nodeUniform40.z );
	nodeVar136 = ( nodeVar135 + 1.0 );
	nodeVar137 = ( object.nodeUniform40.z + 2.0 );
	nodeVar138 = ( nodeVar137 * 0.0 );
	nodeVar139 = ( nodeVar136 + nodeVar138 );
	nodeVar140 = ( nodeVar137 * 7.0 );
	nodeVar141 = ( nodeVar139 / nodeVar140 );
	nodeVar142 = vec3<f32>( nodeVar134.xy, nodeVar141 );
	nodeVar143 = textureSample( nodeUniform37, nodeUniform37_sampler, nodeVar142 );
	nodeVar144 = ( nodeVar137 * 1.0 );
	nodeVar145 = ( nodeVar136 + nodeVar144 );
	nodeVar146 = ( nodeVar145 / nodeVar140 );
	nodeVar147 = vec3<f32>( nodeVar134.xy, nodeVar146 );
	nodeVar148 = textureSample( nodeUniform37, nodeUniform37_sampler, nodeVar147 );
	nodeVar149 = vec3<f32>( nodeVar143.w, nodeVar148.xy );
	nodeVar150 = ( nodeVar137 * 2.0 );
	nodeVar151 = ( nodeVar136 + nodeVar150 );
	nodeVar152 = ( nodeVar151 / nodeVar140 );
	nodeVar153 = vec3<f32>( nodeVar134.xy, nodeVar152 );
	nodeVar154 = textureSample( nodeUniform37, nodeUniform37_sampler, nodeVar153 );
	nodeVar155 = vec3<f32>( nodeVar148.zw, nodeVar154.x );
	nodeVar156 = ( nodeVar137 * 3.0 );
	nodeVar157 = ( nodeVar136 + nodeVar156 );
	nodeVar158 = ( nodeVar157 / nodeVar140 );
	nodeVar159 = vec3<f32>( nodeVar134.xy, nodeVar158 );
	nodeVar160 = textureSample( nodeUniform37, nodeUniform37_sampler, nodeVar159 );
	nodeVar161 = ( nodeVar137 * 4.0 );
	nodeVar162 = ( nodeVar136 + nodeVar161 );
	nodeVar163 = ( nodeVar162 / nodeVar140 );
	nodeVar164 = vec3<f32>( nodeVar134.xy, nodeVar163 );
	nodeVar165 = textureSample( nodeUniform37, nodeUniform37_sampler, nodeVar164 );
	nodeVar166 = vec3<f32>( nodeVar160.w, nodeVar165.xy );
	nodeVar167 = ( nodeVar137 * 5.0 );
	nodeVar168 = ( nodeVar136 + nodeVar167 );
	nodeVar169 = ( nodeVar168 / nodeVar140 );
	nodeVar170 = vec3<f32>( nodeVar134.xy, nodeVar169 );
	nodeVar171 = textureSample( nodeUniform37, nodeUniform37_sampler, nodeVar170 );
	nodeVar172 = vec3<f32>( nodeVar165.zw, nodeVar171.x );
	nodeVar173 = ( nodeVar137 * 6.0 );
	nodeVar174 = ( nodeVar136 + nodeVar173 );
	nodeVar175 = ( nodeVar174 / nodeVar140 );
	nodeVar176 = vec3<f32>( nodeVar134.xy, nodeVar175 );
	nodeVar177 = textureSample( nodeUniform37, nodeUniform37_sampler, nodeVar176 );
	nodeVar178 = array< vec3<f32>, 9 >( nodeVar143.xyz, nodeVar149, nodeVar155, nodeVar154.yzw, nodeVar160.xyz, nodeVar166, nodeVar172, nodeVar171.yzw, nodeVar177.xyz );
	nodeVar179 = ( ( ( ( ( ( ( ( ( nodeVar178[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeVar178[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeVar178[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar178[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeVar178[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeVar178[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeVar178[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeVar178[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar178[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) );
	nodeVar180 = max( nodeVar179, vec3<f32>( 0.0, 0.0, 0.0 ) );
	nodeVar181 = ( nodeVar180 * vec3<f32>( object.nodeUniform41 ) );
	nodeVar182 = ( object.nodeUniform39 - v_positionWorld );
	nodeVar183 = max( nodeVar182, vec3<f32>( 0.0 ) );
	nodeVar184 = ( v_positionWorld - object.nodeUniform38 );
	nodeVar185 = max( nodeVar184, vec3<f32>( 0.0 ) );
	nodeVar186 = ( nodeVar183 + nodeVar185 );
	nodeVar187 = length( nodeVar186 );
	nodeVar188 = smoothstep( 0.0, object.nodeUniform42, nodeVar187 );
	nodeVar189 = ( 1.0 - nodeVar188 );
	nodeVar190 = nodeVar189;
	nodeVar191 = ( nodeVar181 * vec3<f32>( nodeVar190 ) );
	nodeVar192 = ( irradiance + nodeVar191 );
	irradiance = nodeVar192;
	nodeVar193 = ( object.nodeUniform44 - object.nodeUniform45 );
	nodeVar194 = ( object.nodeUniform46 - vec3<f32>( 1.0 ) );
	nodeVar195 = ( nodeVar193 / nodeVar194 );
	nodeVar196 = ( normalWorld * nodeVar195 );
	nodeVar197 = ( nodeVar196 * vec3<f32>( 0.5 ) );
	nodeVar198 = ( v_positionWorld + nodeVar197 );
	nodeVar199 = ( nodeVar198 - object.nodeUniform45 );
	nodeVar200 = ( nodeVar199 / nodeVar193 );
	nodeVar201 = ( clamp( nodeVar200, vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) ) * nodeVar194 );
	nodeVar202 = ( nodeVar201 / object.nodeUniform46 );
	nodeVar203 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform46 );
	nodeVar204 = ( nodeVar202 + nodeVar203 );
	nodeVar205 = ( nodeVar204.z * object.nodeUniform46.z );
	nodeVar206 = ( nodeVar205 + 1.0 );
	nodeVar207 = ( object.nodeUniform46.z + 2.0 );
	nodeVar208 = ( nodeVar207 * 0.0 );
	nodeVar209 = ( nodeVar206 + nodeVar208 );
	nodeVar210 = ( nodeVar207 * 7.0 );
	nodeVar211 = ( nodeVar209 / nodeVar210 );
	nodeVar212 = vec3<f32>( nodeVar204.xy, nodeVar211 );
	nodeVar213 = textureSample( nodeUniform43, nodeUniform43_sampler, nodeVar212 );
	nodeVar214 = ( nodeVar207 * 1.0 );
	nodeVar215 = ( nodeVar206 + nodeVar214 );
	nodeVar216 = ( nodeVar215 / nodeVar210 );
	nodeVar217 = vec3<f32>( nodeVar204.xy, nodeVar216 );
	nodeVar218 = textureSample( nodeUniform43, nodeUniform43_sampler, nodeVar217 );
	nodeVar219 = vec3<f32>( nodeVar213.w, nodeVar218.xy );
	nodeVar220 = ( nodeVar207 * 2.0 );
	nodeVar221 = ( nodeVar206 + nodeVar220 );
	nodeVar222 = ( nodeVar221 / nodeVar210 );
	nodeVar223 = vec3<f32>( nodeVar204.xy, nodeVar222 );
	nodeVar224 = textureSample( nodeUniform43, nodeUniform43_sampler, nodeVar223 );
	nodeVar225 = vec3<f32>( nodeVar218.zw, nodeVar224.x );
	nodeVar226 = ( nodeVar207 * 3.0 );
	nodeVar227 = ( nodeVar206 + nodeVar226 );
	nodeVar228 = ( nodeVar227 / nodeVar210 );
	nodeVar229 = vec3<f32>( nodeVar204.xy, nodeVar228 );
	nodeVar230 = textureSample( nodeUniform43, nodeUniform43_sampler, nodeVar229 );
	nodeVar231 = ( nodeVar207 * 4.0 );
	nodeVar232 = ( nodeVar206 + nodeVar231 );
	nodeVar233 = ( nodeVar232 / nodeVar210 );
	nodeVar234 = vec3<f32>( nodeVar204.xy, nodeVar233 );
	nodeVar235 = textureSample( nodeUniform43, nodeUniform43_sampler, nodeVar234 );
	nodeVar236 = vec3<f32>( nodeVar230.w, nodeVar235.xy );
	nodeVar237 = ( nodeVar207 * 5.0 );
	nodeVar238 = ( nodeVar206 + nodeVar237 );
	nodeVar239 = ( nodeVar238 / nodeVar210 );
	nodeVar240 = vec3<f32>( nodeVar204.xy, nodeVar239 );
	nodeVar241 = textureSample( nodeUniform43, nodeUniform43_sampler, nodeVar240 );
	nodeVar242 = vec3<f32>( nodeVar235.zw, nodeVar241.x );
	nodeVar243 = ( nodeVar207 * 6.0 );
	nodeVar244 = ( nodeVar206 + nodeVar243 );
	nodeVar245 = ( nodeVar244 / nodeVar210 );
	nodeVar246 = vec3<f32>( nodeVar204.xy, nodeVar245 );
	nodeVar247 = textureSample( nodeUniform43, nodeUniform43_sampler, nodeVar246 );
	nodeVar248 = array< vec3<f32>, 9 >( nodeVar213.xyz, nodeVar219, nodeVar225, nodeVar224.yzw, nodeVar230.xyz, nodeVar236, nodeVar242, nodeVar241.yzw, nodeVar247.xyz );
	nodeVar249 = ( ( ( ( ( ( ( ( ( nodeVar248[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeVar248[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeVar248[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar248[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeVar248[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeVar248[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeVar248[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeVar248[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar248[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) );
	nodeVar250 = max( nodeVar249, vec3<f32>( 0.0, 0.0, 0.0 ) );
	nodeVar251 = ( nodeVar250 * vec3<f32>( object.nodeUniform47 ) );
	nodeVar252 = ( object.nodeUniform45 - v_positionWorld );
	nodeVar253 = max( nodeVar252, vec3<f32>( 0.0 ) );
	nodeVar254 = ( v_positionWorld - object.nodeUniform44 );
	nodeVar255 = max( nodeVar254, vec3<f32>( 0.0 ) );
	nodeVar256 = ( nodeVar253 + nodeVar255 );
	nodeVar257 = length( nodeVar256 );
	nodeVar258 = smoothstep( 0.0, object.nodeUniform48, nodeVar257 );
	nodeVar259 = ( 1.0 - nodeVar258 );
	nodeVar260 = nodeVar259;
	nodeVar261 = ( nodeVar251 * vec3<f32>( nodeVar260 ) );
	nodeVar262 = ( irradiance + nodeVar261 );
	irradiance = nodeVar262;
	nodeVar263 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar264 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar265 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar266 = ( SpecularF90 * dfg.y );
	nodeVar267 = ( nodeVar265 + vec3<f32>( nodeVar266 ) );
	nodeVar268 = ( nodeVar263 + nodeVar267 );
	nodeVar263 = nodeVar268;
	nodeVar269 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar270 = nodeVar269;
	nodeVar271 = ( nodeVar270 * vec3<f32>( 0.047619 ) );
	nodeVar272 = ( SpecularColor + nodeVar271 );
	nodeVar273 = ( nodeVar267 * nodeVar272 );
	nodeVar274 = ( dfg.x + dfg.y );
	nodeVar275 = ( 1.0 - nodeVar274 );
	nodeVar276 = nodeVar275;
	nodeVar277 = ( vec3<f32>( nodeVar276 ) * nodeVar272 );
	nodeVar278 = ( vec3<f32>( 1.0 ) - nodeVar277 );
	nodeVar279 = nodeVar278;
	nodeVar280 = ( nodeVar273 / nodeVar279 );
	nodeVar281 = ( nodeVar280 * vec3<f32>( nodeVar276 ) );
	nodeVar282 = ( nodeVar264 + nodeVar281 );
	nodeVar264 = nodeVar282;
	nodeVar283 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar284 = ( irradiance * nodeVar283 );
	nodeVar285 = ( nodeVar263 + nodeVar264 );
	nodeVar286 = ( vec3<f32>( 1.0 ) - nodeVar285 );
	nodeVar287 = nodeVar286;
	nodeVar288 = ( nodeVar284 * nodeVar287 );
	nodeVar289 = nodeVar288;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar290 = ( indirectDiffuse + nodeVar289 );
	indirectDiffuse = nodeVar290;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar291 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar292 = ( SpecularF90 * dfg.y );
	nodeVar293 = ( nodeVar291 + vec3<f32>( nodeVar292 ) );
	nodeVar294 = ( singleScatteringDielectric + nodeVar293 );
	singleScatteringDielectric = nodeVar294;
	nodeVar295 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar296 = nodeVar295;
	nodeVar297 = ( nodeVar296 * vec3<f32>( 0.047619 ) );
	nodeVar298 = ( SpecularColor + nodeVar297 );
	nodeVar299 = ( nodeVar293 * nodeVar298 );
	nodeVar300 = ( dfg.x + dfg.y );
	nodeVar301 = ( 1.0 - nodeVar300 );
	nodeVar302 = nodeVar301;
	nodeVar303 = ( vec3<f32>( nodeVar302 ) * nodeVar298 );
	nodeVar304 = ( vec3<f32>( 1.0 ) - nodeVar303 );
	nodeVar305 = nodeVar304;
	nodeVar306 = ( nodeVar299 / nodeVar305 );
	nodeVar307 = ( nodeVar306 * vec3<f32>( nodeVar302 ) );
	nodeVar308 = ( multiScatteringDielectric + nodeVar307 );
	multiScatteringDielectric = nodeVar308;
	nodeVar309 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar310 = ( SpecularF90 * dfg.y );
	nodeVar311 = ( nodeVar309 + vec3<f32>( nodeVar310 ) );
	nodeVar312 = ( singleScatteringMetallic + nodeVar311 );
	singleScatteringMetallic = nodeVar312;
	nodeVar313 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar314 = nodeVar313;
	nodeVar315 = ( nodeVar314 * vec3<f32>( 0.047619 ) );
	nodeVar316 = ( DiffuseColor.xyz + nodeVar315 );
	nodeVar317 = ( nodeVar311 * nodeVar316 );
	nodeVar318 = ( dfg.x + dfg.y );
	nodeVar319 = ( 1.0 - nodeVar318 );
	nodeVar320 = nodeVar319;
	nodeVar321 = ( vec3<f32>( nodeVar320 ) * nodeVar316 );
	nodeVar322 = ( vec3<f32>( 1.0 ) - nodeVar321 );
	nodeVar323 = nodeVar322;
	nodeVar324 = ( nodeVar317 / nodeVar323 );
	nodeVar325 = ( nodeVar324 * vec3<f32>( nodeVar320 ) );
	nodeVar326 = ( multiScatteringMetallic + nodeVar325 );
	multiScatteringMetallic = nodeVar326;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar327 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar328 = ( radiance * nodeVar327 );
	nodeVar329 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar330 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar331 = ( nodeVar329 * nodeVar330 );
	nodeVar332 = ( nodeVar328 + nodeVar331 );
	nodeVar333 = nodeVar332;
	nodeVar334 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar335 = ( vec3<f32>( 1.0 ) - nodeVar334 );
	nodeVar336 = nodeVar335;
	nodeVar337 = ( DiffuseContribution * nodeVar336 );
	nodeVar338 = ( nodeVar337 * nodeVar330 );
	nodeVar339 = nodeVar338;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar340 = ( indirectSpecular + nodeVar333 );
	indirectSpecular = nodeVar340;
	nodeVar341 = ( indirectDiffuse + nodeVar339 );
	indirectDiffuse = nodeVar341;
	ambientOcclusion = 1.0;
	nodeVar342 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar342;
	nodeVar343 = dot( normalView, positionViewDirection );
	nodeVar344 = ( clamp( nodeVar343, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar345 = ( Roughness * -16.0 );
	nodeVar346 = ( 1.0 - nodeVar345 );
	nodeVar347 = nodeVar346;
	nodeVar348 = ( - nodeVar347 );
	nodeVar349 = exp2( nodeVar348 );
	nodeVar350 = pow( nodeVar344, nodeVar349 );
	nodeVar351 = ( 1.0 - nodeVar350 );
	nodeVar352 = nodeVar351;
	nodeVar353 = ( ambientOcclusion - nodeVar352 );
	nodeVar354 = ( indirectSpecular * vec3<f32>( clamp( nodeVar353, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar354;
	nodeVar355 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar355;
	nodeVar356 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar356;
	nodeVar357 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar357;
	nodeVar358 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar358;

	// result

	output.color = nodeVar358;

	return output;

}
