// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform7_sampler : sampler;
@binding( 2 ) @group( 1 ) var nodeUniform7 : texture_2d<f32>;
@binding( 3 ) @group( 1 ) var nodeUniform15_sampler : sampler_comparison;
@binding( 4 ) @group( 1 ) var nodeUniform15 : texture_depth_2d;
@binding( 5 ) @group( 1 ) var nodeUniform29_sampler : sampler;
@binding( 6 ) @group( 1 ) var nodeUniform29 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : vec3<f32>,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : f32,
	nodeUniform24 : f32,
	nodeUniform25 : mat4x4<f32>,
	nodeUniform27 : f32,
	nodeUniform28 : f32,
	nodeUniform30 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraPosition : vec3<f32>,
	nodeUniform10 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform13 : mat4x4<f32>,
	nodeUniform12 : vec4<f32>,
	nodeUniform19 : mat4x4<f32>,
	nodeUniform18 : vec4<f32>,
	nodeUniform11 : f32,
	nodeUniform14 : f32,
	nodeUniform16 : f32,
	nodeUniform17 : vec2<f32>,
	nodeUniform20 : f32,
	nodeUniform21 : f32,
	nodeUniform22 : vec2<f32>,
	nodeUniform23 : f32,
	cameraWorldMatrix : mat4x4<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : bool;
var<private> nodeVar2 : bool;
var<private> nodeVar3 : bool;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : u32;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> normalLocal : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<f32>;
var<private> nodeVar19 : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : u32;
var<private> nodeVar22 : u32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : u32;
var<private> nodeVar25 : u32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : u32;
var<private> nodeVar37 : u32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : u32;
var<private> nodeVar41 : u32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : vec3<f32>;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : u32;
var<private> nodeVar55 : u32;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : vec3<f32>;
var<private> nodeVar59 : vec3<f32>;
var<private> nodeVar60 : u32;
var<private> nodeVar61 : u32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : vec3<f32>;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : f32;
var<private> nodeVar70 : u32;
var<private> nodeVar71 : u32;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : vec3<f32>;
var<private> nodeVar76 : u32;
var<private> nodeVar77 : u32;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : u32;
var<private> nodeVar87 : u32;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : u32;
var<private> nodeVar93 : u32;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : f32;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : vec3<f32>;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : bool;
var<private> nodeVar111 : bool;
var<private> nodeVar112 : bool;
var<private> nodeVar113 : bool;
var<private> nodeVar114 : bool;
var<private> nodeVar115 : bool;
var<private> nodeVar116 : bool;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : f32;
var<private> nodeVar121 : u32;
var<private> nodeVar122 : u32;
var<private> nodeVar123 : u32;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : f32;
var<private> nodeVar130 : u32;
var<private> nodeVar131 : u32;
var<private> nodeVar132 : f32;
var<private> nodeVar133 : u32;
var<private> nodeVar134 : u32;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : u32;
var<private> nodeVar137 : u32;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : f32;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : bool;
var<private> nodeVar174 : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : f32;
var<private> nodeVar177 : u32;
var<private> nodeVar178 : u32;
var<private> nodeVar179 : u32;
var<private> nodeVar180 : f32;
var<private> nodeVar181 : vec3<f32>;
var<private> nodeVar182 : vec3<f32>;
var<private> nodeVar183 : vec3<f32>;
var<private> nodeVar184 : vec3<f32>;
var<private> nodeVar185 : f32;
var<private> nodeVar186 : u32;
var<private> nodeVar187 : u32;
var<private> nodeVar188 : f32;
var<private> nodeVar189 : u32;
var<private> nodeVar190 : u32;
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : u32;
var<private> nodeVar193 : u32;
var<private> nodeVar194 : f32;
var<private> nodeVar195 : vec3<f32>;
var<private> nodeVar196 : vec3<f32>;
var<private> nodeVar197 : vec3<f32>;
var<private> nodeVar198 : vec3<f32>;
var<private> nodeVar199 : f32;
var<private> nodeVar200 : vec3<f32>;
var<private> nodeVar201 : u32;
var<private> nodeVar202 : u32;
var<private> nodeVar203 : f32;
var<private> nodeVar204 : vec3<f32>;
var<private> nodeVar205 : vec3<f32>;
var<private> nodeVar206 : vec3<f32>;
var<private> nodeVar207 : vec3<f32>;
var<private> nodeVar208 : vec3<f32>;
var<private> nodeVar209 : vec3<f32>;
var<private> nodeVar210 : u32;
var<private> nodeVar211 : u32;
var<private> nodeVar212 : u32;
var<private> nodeVar213 : u32;
var<private> nodeVar214 : u32;
var<private> nodeVar215 : u32;
var<private> nodeVar216 : vec3<f32>;
var<private> nodeVar217 : vec3<f32>;
var<private> nodeVar218 : u32;
var<private> nodeVar219 : u32;
var<private> nodeVar220 : u32;
var<private> nodeVar221 : u32;
var<private> nodeVar222 : vec3<f32>;
var<private> nodeVar223 : vec3<f32>;
var<private> nodeVar224 : f32;
var<private> nodeVar225 : u32;
var<private> nodeVar226 : u32;
var<private> nodeVar227 : u32;
var<private> nodeVar228 : f32;
var<private> nodeVar229 : vec3<f32>;
var<private> nodeVar230 : vec3<f32>;
var<private> nodeVar231 : vec3<f32>;
var<private> nodeVar232 : vec3<f32>;
var<private> nodeVar233 : f32;
var<private> nodeVar234 : u32;
var<private> nodeVar235 : u32;
var<private> nodeVar236 : f32;
var<private> nodeVar237 : u32;
var<private> nodeVar238 : u32;
var<private> nodeVar239 : f32;
var<private> nodeVar240 : f32;
var<private> nodeVar241 : vec3<f32>;
var<private> nodeVar242 : u32;
var<private> nodeVar243 : u32;
var<private> nodeVar244 : u32;
var<private> nodeVar245 : u32;
var<private> nodeVar246 : u32;
var<private> nodeVar247 : u32;
var<private> nodeVar248 : f32;
var<private> nodeVar249 : f32;
var<private> nodeVar250 : f32;
var<private> nodeVar251 : f32;
var<private> nodeVar252 : f32;
var<private> nodeVar253 : f32;
var<private> nodeVar254 : f32;
var<private> nodeVar255 : vec3<f32>;
var<private> nodeVar256 : u32;
var<private> nodeVar257 : vec3<f32>;
var<private> nodeVar258 : vec3<f32>;
var<private> nodeVar259 : u32;
var<private> nodeVar260 : u32;
var<private> nodeVar261 : f32;
var<private> nodeVar262 : f32;
var<private> nodeVar263 : f32;
var<private> nodeVar264 : vec3<f32>;
var<private> nodeVar265 : vec3<f32>;
var<private> nodeVar266 : vec3<f32>;
var<private> nodeVar267 : vec3<f32>;
var<private> nodeVar268 : vec3<f32>;
var<private> nodeVar269 : vec3<f32>;
var<private> nodeVar270 : vec3<f32>;
var<private> nodeVar271 : f32;
var<private> nodeVar272 : vec3<f32>;
var<private> nodeVar273 : vec3<f32>;
var<private> nodeVar274 : vec3<f32>;
var<private> nodeVar275 : f32;
var<private> nodeVar276 : f32;
var<private> nodeVar277 : vec3<f32>;
var<private> nodeVar278 : u32;
var<private> nodeVar279 : u32;
var<private> nodeVar280 : f32;
var<private> nodeVar281 : f32;
var<private> nodeVar282 : f32;
var<private> nodeVar283 : vec3<f32>;
var<private> nodeVar284 : vec3<f32>;
var<private> nodeVar285 : vec3<f32>;
var<private> nodeVar286 : f32;
var<private> nodeVar287 : f32;
var<private> nodeVar288 : f32;
var<private> nodeVar289 : vec3<f32>;
var<private> nodeVar290 : vec3<f32>;
var<private> nodeVar291 : vec3<f32>;
var<private> nodeVar292 : vec3<f32>;
var<private> nodeVar293 : vec3<f32>;
var<private> nodeVar294 : vec3<f32>;
var<private> nodeVar295 : f32;
var<private> nodeVar296 : u32;
var<private> nodeVar297 : u32;
var<private> nodeVar298 : f32;
var<private> nodeVar299 : f32;
var<private> nodeVar300 : vec3<f32>;
var<private> nodeVar301 : vec3<f32>;
var<private> nodeVar302 : vec3<f32>;
var<private> nodeVar303 : vec3<f32>;
var<private> nodeVar304 : vec3<f32>;
var<private> nodeVar305 : vec3<f32>;
var<private> nodeVar306 : f32;
var<private> nodeVar307 : u32;
var<private> nodeVar308 : u32;
var<private> nodeVar309 : f32;
var<private> nodeVar310 : f32;
var<private> nodeVar311 : vec3<f32>;
var<private> nodeVar312 : vec3<f32>;
var<private> nodeVar313 : vec3<f32>;
var<private> nodeVar314 : vec3<f32>;
var<private> nodeVar315 : vec3<f32>;
var<private> nodeVar316 : vec3<f32>;
var<private> nodeVar317 : f32;
var<private> nodeVar318 : f32;
var<private> nodeVar319 : f32;
var<private> nodeVar320 : f32;
var<private> nodeVar321 : f32;
var<private> nodeVar322 : vec3<f32>;
var<private> nodeVar323 : vec3<f32>;
var<private> nodeVar324 : vec3<f32>;
var<private> nodeVar325 : vec3<f32>;
var<private> nodeVar326 : vec3<f32>;
var<private> nodeVar327 : vec3<f32>;
var<private> nodeVar328 : f32;
var<private> nodeVar329 : vec3<f32>;
var<private> nodeVar330 : f32;
var<private> nodeVar331 : bool;
var<private> nodeVar332 : bool;
var<private> nodeVar333 : bool;
var<private> nodeVar334 : bool;
var<private> nodeVar335 : bool;
var<private> nodeVar336 : bool;
var<private> nodeVar337 : vec3<f32>;
var<private> nodeVar338 : u32;
var<private> nodeVar339 : u32;
var<private> nodeVar340 : f32;
var<private> nodeVar341 : vec3<f32>;
var<private> nodeVar342 : vec3<f32>;
var<private> nodeVar343 : vec3<f32>;
var<private> nodeVar344 : vec3<f32>;
var<private> nodeVar345 : vec3<f32>;
var<private> nodeVar346 : vec3<f32>;
var<private> nodeVar347 : vec3<f32>;
var<private> nodeVar348 : u32;
var<private> nodeVar349 : u32;
var<private> nodeVar350 : f32;
var<private> nodeVar351 : vec3<f32>;
var<private> nodeVar352 : vec3<f32>;
var<private> nodeVar353 : vec3<f32>;
var<private> nodeVar354 : vec3<f32>;
var<private> nodeVar355 : vec3<f32>;
var<private> nodeVar356 : vec3<f32>;
var<private> nodeVar357 : f32;
var<private> nodeVar358 : vec3<f32>;
var<private> nodeVar359 : vec3<f32>;
var<private> nodeVar360 : f32;
var<private> nodeVar361 : vec3<f32>;
var<private> nodeVar362 : vec3<f32>;
var<private> nodeVar363 : f32;
var<private> nodeVar364 : vec3<f32>;
var<private> nodeVar365 : vec3<f32>;
var<private> nodeVar366 : f32;
var<private> nodeVar367 : vec3<f32>;
var<private> nodeVar368 : vec3<f32>;
var<private> nodeVar369 : vec3<f32>;
var<private> nodeVar370 : vec3<f32>;
var<private> nodeVar371 : bool;
var<private> nodeVar372 : vec3<f32>;
var<private> nodeVar373 : f32;
var<private> nodeVar374 : f32;
var<private> nodeVar375 : u32;
var<private> nodeVar376 : u32;
var<private> nodeVar377 : vec3<f32>;
var<private> nodeVar378 : vec3<f32>;
var<private> nodeVar379 : u32;
var<private> nodeVar380 : u32;
var<private> nodeVar381 : u32;
var<private> nodeVar382 : u32;
var<private> nodeVar383 : u32;
var<private> nodeVar384 : u32;
var<private> nodeVar385 : u32;
var<private> nodeVar386 : u32;
var<private> nodeVar387 : vec3<f32>;
var<private> nodeVar388 : vec3<f32>;
var<private> nodeVar389 : f32;
var<private> nodeVar390 : f32;
var<private> nodeVar391 : vec3<f32>;
var<private> nodeVar392 : u32;
var<private> nodeVar393 : u32;
var<private> nodeVar394 : u32;
var<private> nodeVar395 : u32;
var<private> nodeVar396 : u32;
var<private> nodeVar397 : u32;
var<private> nodeVar398 : u32;
var<private> nodeVar399 : u32;
var<private> nodeVar400 : f32;
var<private> nodeVar401 : f32;
var<private> nodeVar402 : f32;
var<private> nodeVar403 : f32;
var<private> nodeVar404 : f32;
var<private> nodeVar405 : f32;
var<private> nodeVar406 : f32;
var<private> nodeVar407 : vec4<f32>;
var<private> nodeVar408 : vec3<f32>;
var<private> nodeVar409 : vec3<f32>;
var<private> nodeVar410 : vec3<f32>;
var<private> nodeVar411 : f32;
var<private> nodeVar412 : vec3<f32>;
var<private> nodeVar413 : vec3<f32>;
var<private> nodeVar414 : vec3<f32>;
var<private> nodeVar415 : vec2<f32>;
var<private> nodeVar416 : u32;
var<private> nodeVar417 : u32;
var<private> nodeVar418 : f32;
var<private> nodeVar419 : vec3<f32>;
var<private> nodeVar420 : vec3<f32>;
var<private> nodeVar421 : vec3<f32>;
var<private> nodeVar422 : vec3<f32>;
var<private> nodeVar423 : vec3<f32>;
var<private> nodeVar424 : bool;
var<private> nodeVar426 : vec3<f32>;
var<private> nodeVar427 : f32;
var<private> normalWorldGeometry : vec3<f32>;
var<private> nodeVar429 : vec3<f32>;
var<private> nodeVar430 : f32;
var<private> nodeVar431 : f32;
var<private> nodeVar432 : f32;
var<private> nodeVar433 : f32;
var<private> nodeVar434 : f32;
var<private> nodeVar435 : u32;
var<private> nodeVar436 : u32;
var<private> nodeVar437 : u32;
var<private> nodeVar438 : u32;
var<private> nodeVar439 : u32;
var<private> nodeVar440 : f32;
var<private> nodeVar441 : vec3<f32>;
var<private> nodeVar442 : vec3<f32>;
var<private> nodeVar443 : f32;
var<private> nodeVar444 : f32;
var<private> nodeVar445 : f32;
var<private> nodeVar446 : f32;
var<private> nodeVar447 : f32;
var<private> nodeVar448 : f32;
var<private> nodeVar449 : f32;
var<private> nodeVar450 : vec3<f32>;
var<private> Metalness : f32;
var<private> Roughness : f32;
var<private> nodeVar451 : f32;
var<private> nodeVar452 : f32;
var<private> nodeVar453 : f32;
var<private> nodeVar454 : f32;
var<private> nodeVar455 : f32;
var<private> nodeVar456 : f32;
var<private> nodeVar457 : vec3<f32>;
var<private> nodeVar458 : f32;
var<private> nodeVar459 : f32;
var<private> nodeVar460 : f32;
var<private> nodeVar461 : f32;
var<private> nodeVar462 : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar463 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> nodeVar464 : vec3<f32>;
var<private> nodeVar465 : f32;
var<private> Output : vec4<f32>;
var<private> nodeVar466 : vec3<f32>;
var<private> nodeVar467 : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> NORMAL_nodeVar468 : vec3<f32>;
var<private> NORMAL_nodeVar469 : f32;
var<private> nodeVar470 : f32;
var<private> nodeVar471 : f32;
var<private> nodeVar472 : f32;
var<private> nodeVar473 : f32;
var<private> nodeVar474 : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar475 : f32;
var<private> nodeVar476 : vec2<f32>;
var<private> nodeVar477 : f32;
var<private> nodeVar478 : f32;
var<private> nodeVar479 : f32;
var<private> nodeVar480 : f32;
var<private> nodeVar481 : vec3<f32>;
var<private> nodeVar482 : vec3<f32>;
var<private> nodeVar483 : vec4<f32>;
var<private> nodeVar484 : vec4<f32>;
var<private> nodeVar485 : vec3<f32>;
var<private> nodeVar486 : vec3<f32>;
var<private> nodeVar487 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar488 : vec4<f32>;
var<private> nodeVar489 : f32;
var<private> shadowValue : f32;
var<private> nodeVar490 : f32;
var<private> nodeVar491 : vec4<f32>;
var<private> nodeVar492 : vec3<f32>;
var<private> nodeVar493 : vec3<f32>;
var<private> nodeVar494 : f32;
var<private> nodeVar495 : f32;
var<private> nodeVar496 : vec2<f32>;
var<private> nodeVar497 : f32;
var<private> nodeVar498 : vec2<f32>;
var<private> nodeVar499 : f32;
var<private> nodeVar500 : vec2<f32>;
var<private> nodeVar501 : f32;
var<private> nodeVar502 : vec2<f32>;
var<private> nodeVar503 : f32;
var<private> nodeVar504 : vec2<f32>;
var<private> nodeVar505 : f32;
var<private> nodeVar506 : f32;
var<private> nodeVar507 : vec4<f32>;
var<private> nodeVar508 : vec3<f32>;
var<private> nodeVar509 : vec3<f32>;
var<private> nodeVar510 : f32;
var<private> nodeVar511 : f32;
var<private> nodeVar512 : vec2<f32>;
var<private> nodeVar513 : f32;
var<private> nodeVar514 : vec2<f32>;
var<private> nodeVar515 : f32;
var<private> nodeVar516 : vec2<f32>;
var<private> nodeVar517 : f32;
var<private> nodeVar518 : vec2<f32>;
var<private> nodeVar519 : f32;
var<private> nodeVar520 : vec2<f32>;
var<private> nodeVar521 : f32;
var<private> nodeVar522 : f32;
var<private> nodeVar523 : vec3<f32>;
var<private> nodeVar524 : vec3<f32>;
var<private> nodeVar525 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar526 : vec3<f32>;
var<private> nodeVar527 : vec3<f32>;
var<private> nodeVar528 : vec3<f32>;
var<private> nodeVar529 : vec3<f32>;
var<private> nodeVar530 : f32;
var<private> nodeVar531 : f32;
var<private> nodeVar532 : f32;
var<private> nodeVar533 : vec3<f32>;
var<private> nodeVar534 : vec3<f32>;
var<private> nodeVar535 : vec3<f32>;
var<private> nodeVar536 : vec3<f32>;
var<private> nodeVar537 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar538 : vec3<f32>;
var<private> nodeVar539 : f32;
var<private> nodeVar540 : f32;
var<private> nodeVar541 : f32;
var<private> nodeVar542 : vec3<f32>;
var<private> nodeVar543 : vec3<f32>;
var<private> nodeVar544 : vec3<f32>;
var<private> nodeVar545 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar546 : f32;
var<private> nodeVar547 : f32;
var<private> nodeVar548 : f32;
var<private> nodeVar549 : vec3<f32>;
var<private> nodeVar550 : f32;
var<private> nodeVar551 : f32;
var<private> nodeVar552 : f32;
var<private> nodeVar553 : vec2<f32>;
var<private> nodeVar554 : vec4<f32>;
var<private> nodeVar555 : vec3<f32>;
var<private> nodeVar556 : f32;
var<private> nodeVar557 : f32;
var<private> nodeVar558 : f32;
var<private> nodeVar559 : f32;
var<private> nodeVar560 : f32;
var<private> nodeVar561 : vec2<f32>;
var<private> nodeVar562 : vec4<f32>;
var<private> nodeVar563 : vec3<f32>;
var<private> nodeVar564 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar565 : f32;
var<private> nodeVar566 : f32;
var<private> nodeVar567 : f32;
var<private> nodeVar568 : f32;
var<private> nodeVar569 : f32;
var<private> nodeVar570 : f32;
var<private> nodeVar571 : vec2<f32>;
var<private> nodeVar572 : vec4<f32>;
var<private> nodeVar573 : vec3<f32>;
var<private> nodeVar574 : f32;
var<private> nodeVar575 : f32;
var<private> nodeVar576 : f32;
var<private> nodeVar577 : f32;
var<private> nodeVar578 : f32;
var<private> nodeVar579 : vec2<f32>;
var<private> nodeVar580 : vec4<f32>;
var<private> nodeVar581 : vec3<f32>;
var<private> nodeVar582 : vec3<f32>;
var<private> nodeVar583 : vec3<f32>;
var<private> nodeVar584 : vec3<f32>;
var<private> nodeVar585 : vec3<f32>;
var<private> nodeVar586 : f32;
var<private> nodeVar587 : vec3<f32>;
var<private> nodeVar588 : vec3<f32>;
var<private> nodeVar589 : vec3<f32>;
var<private> nodeVar590 : vec3<f32>;
var<private> nodeVar591 : vec3<f32>;
var<private> nodeVar592 : vec3<f32>;
var<private> nodeVar593 : vec3<f32>;
var<private> nodeVar594 : f32;
var<private> nodeVar595 : f32;
var<private> nodeVar596 : f32;
var<private> nodeVar597 : vec3<f32>;
var<private> nodeVar598 : vec3<f32>;
var<private> nodeVar599 : vec3<f32>;
var<private> nodeVar600 : vec3<f32>;
var<private> nodeVar601 : vec3<f32>;
var<private> nodeVar602 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar603 : vec3<f32>;
var<private> nodeVar604 : vec3<f32>;
var<private> nodeVar605 : vec3<f32>;
var<private> nodeVar606 : vec3<f32>;
var<private> nodeVar607 : vec3<f32>;
var<private> nodeVar608 : vec3<f32>;
var<private> nodeVar609 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar610 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar611 : vec3<f32>;
var<private> nodeVar612 : f32;
var<private> nodeVar613 : vec3<f32>;
var<private> nodeVar614 : vec3<f32>;
var<private> nodeVar615 : vec3<f32>;
var<private> nodeVar616 : vec3<f32>;
var<private> nodeVar617 : vec3<f32>;
var<private> nodeVar618 : vec3<f32>;
var<private> nodeVar619 : vec3<f32>;
var<private> nodeVar620 : f32;
var<private> nodeVar621 : f32;
var<private> nodeVar622 : f32;
var<private> nodeVar623 : vec3<f32>;
var<private> nodeVar624 : vec3<f32>;
var<private> nodeVar625 : vec3<f32>;
var<private> nodeVar626 : vec3<f32>;
var<private> nodeVar627 : vec3<f32>;
var<private> nodeVar628 : vec3<f32>;
var<private> nodeVar629 : vec3<f32>;
var<private> nodeVar630 : f32;
var<private> nodeVar631 : vec3<f32>;
var<private> nodeVar632 : vec3<f32>;
var<private> nodeVar633 : vec3<f32>;
var<private> nodeVar634 : vec3<f32>;
var<private> nodeVar635 : vec3<f32>;
var<private> nodeVar636 : vec3<f32>;
var<private> nodeVar637 : vec3<f32>;
var<private> nodeVar638 : f32;
var<private> nodeVar639 : f32;
var<private> nodeVar640 : f32;
var<private> nodeVar641 : vec3<f32>;
var<private> nodeVar642 : vec3<f32>;
var<private> nodeVar643 : vec3<f32>;
var<private> nodeVar644 : vec3<f32>;
var<private> nodeVar645 : vec3<f32>;
var<private> nodeVar646 : vec3<f32>;
var<private> nodeVar647 : vec3<f32>;
var<private> nodeVar648 : vec3<f32>;
var<private> nodeVar649 : vec3<f32>;
var<private> nodeVar650 : vec3<f32>;
var<private> nodeVar651 : vec3<f32>;
var<private> nodeVar652 : vec3<f32>;
var<private> nodeVar653 : vec3<f32>;
var<private> nodeVar654 : vec3<f32>;
var<private> nodeVar655 : vec3<f32>;
var<private> nodeVar656 : vec3<f32>;
var<private> nodeVar657 : vec3<f32>;
var<private> nodeVar658 : vec3<f32>;
var<private> nodeVar659 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar660 : vec3<f32>;
var<private> nodeVar661 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar662 : vec3<f32>;
var<private> nodeVar663 : f32;
var<private> nodeVar664 : f32;
var<private> nodeVar665 : f32;
var<private> nodeVar666 : f32;
var<private> nodeVar667 : f32;
var<private> nodeVar668 : f32;
var<private> nodeVar669 : f32;
var<private> nodeVar670 : f32;
var<private> nodeVar671 : f32;
var<private> nodeVar672 : f32;
var<private> nodeVar673 : f32;
var<private> nodeVar674 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar675 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar676 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar677 : vec3<f32>;
var<private> nodeVar678 : vec4<f32>;

// codes
fn mx_rotl32 ( x : u32, k : i32 ) -> u32 {

	var nodeVar0 : i32;
	var nodeVar1 : u32;

	nodeVar0 = k;
	nodeVar1 = x;

	return ( ( nodeVar1 << u32( nodeVar0 ) ) | ( nodeVar1 >> u32( ( 32 - nodeVar0 ) ) ) );

}


fn mx_bjfinal ( a : u32, b : u32, c : u32 ) -> u32 {

	var nodeVar0 : u32;
	var nodeVar1 : u32;
	var nodeVar2 : u32;

	nodeVar0 = c;
	nodeVar1 = b;
	nodeVar2 = a;
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - mx_rotl32( nodeVar1, 14 ) );
	nodeVar2 = ( nodeVar2 ^ nodeVar0 );
	nodeVar2 = ( nodeVar2 - mx_rotl32( nodeVar0, 11 ) );
	nodeVar1 = ( nodeVar1 ^ nodeVar2 );
	nodeVar1 = ( nodeVar1 - mx_rotl32( nodeVar2, 25 ) );
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - mx_rotl32( nodeVar1, 16 ) );
	nodeVar2 = ( nodeVar2 ^ nodeVar0 );
	nodeVar2 = ( nodeVar2 - mx_rotl32( nodeVar0, 4 ) );
	nodeVar1 = ( nodeVar1 ^ nodeVar2 );
	nodeVar1 = ( nodeVar1 - mx_rotl32( nodeVar2, 14 ) );
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - mx_rotl32( nodeVar1, 24 ) );

	return nodeVar0;

}


fn mx_select ( b : bool, t : f32, f : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : bool;
	var nodeVar3 : f32;

	nodeVar0 = f;
	nodeVar1 = t;
	nodeVar2 = b;

	return select( nodeVar0, nodeVar1, nodeVar2 );

}


fn mx_negate_if ( val : f32, b : bool ) -> f32 {

	var nodeVar0 : bool;
	var nodeVar1 : f32;
	var nodeVar2 : f32;

	nodeVar0 = b;
	nodeVar1 = val;

	return select( nodeVar1, ( - nodeVar1 ), nodeVar0 );

}


fn mx_floor ( x : f32 ) -> i32 {

	var nodeVar0 : f32;

	nodeVar0 = x;

	return i32( floor( nodeVar0 ) );

}


fn mx_fade ( t : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = t;

	return ( ( ( nodeVar0 * nodeVar0 ) * nodeVar0 ) * ( ( nodeVar0 * ( ( nodeVar0 * 6.0 ) - 15.0 ) ) + 10.0 ) );

}


fn mx_hash_int_2 ( x : i32, y : i32, z : i32 ) -> u32 {

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

	return mx_bjfinal( nodeVar4, nodeVar5, nodeVar6 );

}


fn mx_gradient_float_1 ( hash : u32, x : f32, y : f32, z : f32 ) -> f32 {

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
	nodeVar5 = mx_select( ( nodeVar4 < 8u ), nodeVar2, nodeVar1 );
	nodeVar6 = mx_select( ( nodeVar4 < 4u ), nodeVar1, mx_select( ( ( nodeVar4 == 12u ) || ( nodeVar4 == 14u ) ), nodeVar2, nodeVar0 ) );

	return ( mx_negate_if( nodeVar5, bool( ( nodeVar4 & 1u ) ) ) + mx_negate_if( nodeVar6, bool( ( nodeVar4 & 2u ) ) ) );

}


fn mx_trilerp_0 ( v0 : f32, v1 : f32, v2 : f32, v3 : f32, v4 : f32, v5 : f32, v6 : f32, v7 : f32, s : f32, t : f32, r : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;
	var nodeVar6 : f32;
	var nodeVar7 : f32;
	var nodeVar8 : f32;
	var nodeVar9 : f32;
	var nodeVar10 : f32;
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

	return ( ( nodeVar13 * ( ( nodeVar12 * ( ( nodeVar10 * nodeVar11 ) + ( nodeVar9 * nodeVar2 ) ) ) + ( nodeVar1 * ( ( nodeVar8 * nodeVar11 ) + ( nodeVar7 * nodeVar2 ) ) ) ) ) + ( nodeVar0 * ( ( nodeVar12 * ( ( nodeVar6 * nodeVar11 ) + ( nodeVar5 * nodeVar2 ) ) ) + ( nodeVar1 * ( ( nodeVar4 * nodeVar11 ) + ( nodeVar3 * nodeVar2 ) ) ) ) ) );

}


fn mx_gradient_scale3d_0 ( v : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = v;

	return ( 0.982 * nodeVar0 );

}


fn mx_perlin_noise_float_1 ( p : vec3<f32> ) -> f32 {

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
	var nodeVar13 : f32;

	nodeVar0 = p;
	nodeVar1 = 0;
	nodeVar2 = 0;
	nodeVar3 = 0;
	nodeVar4 = nodeVar0.x;
	nodeVar1 = mx_floor( nodeVar4 );
	nodeVar5 = ( nodeVar4 - f32( nodeVar1 ) );
	nodeVar6 = nodeVar0.y;
	nodeVar2 = mx_floor( nodeVar6 );
	nodeVar7 = ( nodeVar6 - f32( nodeVar2 ) );
	nodeVar8 = nodeVar0.z;
	nodeVar3 = mx_floor( nodeVar8 );
	nodeVar9 = ( nodeVar8 - f32( nodeVar3 ) );
	nodeVar10 = mx_fade( nodeVar5 );
	nodeVar11 = mx_fade( nodeVar7 );
	nodeVar12 = mx_fade( nodeVar9 );
	nodeVar13 = mx_trilerp_0( mx_gradient_float_1( mx_hash_int_2( nodeVar1, nodeVar2, nodeVar3 ), nodeVar5, nodeVar7, nodeVar9 ), mx_gradient_float_1( mx_hash_int_2( ( nodeVar1 + 1 ), nodeVar2, nodeVar3 ), ( nodeVar5 - 1.0 ), nodeVar7, nodeVar9 ), mx_gradient_float_1( mx_hash_int_2( nodeVar1, ( nodeVar2 + 1 ), nodeVar3 ), nodeVar5, ( nodeVar7 - 1.0 ), nodeVar9 ), mx_gradient_float_1( mx_hash_int_2( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ), nodeVar3 ), ( nodeVar5 - 1.0 ), ( nodeVar7 - 1.0 ), nodeVar9 ), mx_gradient_float_1( mx_hash_int_2( nodeVar1, nodeVar2, ( nodeVar3 + 1 ) ), nodeVar5, nodeVar7, ( nodeVar9 - 1.0 ) ), mx_gradient_float_1( mx_hash_int_2( ( nodeVar1 + 1 ), nodeVar2, ( nodeVar3 + 1 ) ), ( nodeVar5 - 1.0 ), nodeVar7, ( nodeVar9 - 1.0 ) ), mx_gradient_float_1( mx_hash_int_2( nodeVar1, ( nodeVar2 + 1 ), ( nodeVar3 + 1 ) ), nodeVar5, ( nodeVar7 - 1.0 ), ( nodeVar9 - 1.0 ) ), mx_gradient_float_1( mx_hash_int_2( ( nodeVar1 + 1 ), ( nodeVar2 + 1 ), ( nodeVar3 + 1 ) ), ( nodeVar5 - 1.0 ), ( nodeVar7 - 1.0 ), ( nodeVar9 - 1.0 ) ), nodeVar10, nodeVar11, nodeVar12 );

	return mx_gradient_scale3d_0( nodeVar13 );

}


fn tsl_mod_float( x : f32, y : f32 ) -> f32 { return x - y * floor( x / y ); }
fn valueNoise ( p : vec3<f32> ) -> f32 {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : vec3<f32>;
	var nodeVar2 : u32;
	var nodeVar3 : u32;
	var nodeVar4 : vec3<f32>;
	var nodeVar5 : u32;
	var nodeVar6 : u32;
	var nodeVar7 : vec3<f32>;
	var nodeVar8 : vec3<f32>;
	var nodeVar9 : vec3<f32>;
	var nodeVar10 : u32;
	var nodeVar11 : u32;
	var nodeVar12 : vec3<f32>;
	var nodeVar13 : u32;
	var nodeVar14 : u32;
	var nodeVar15 : vec3<f32>;
	var nodeVar16 : u32;
	var nodeVar17 : u32;
	var nodeVar18 : vec3<f32>;
	var nodeVar19 : u32;
	var nodeVar20 : u32;
	var nodeVar21 : vec3<f32>;
	var nodeVar22 : u32;
	var nodeVar23 : u32;
	var nodeVar24 : vec3<f32>;
	var nodeVar25 : u32;
	var nodeVar26 : u32;

	nodeVar0 = floor( p );
	nodeVar1 = ( nodeVar0 + vec3<f32>( 0.0, 0.0, 0.0 ) );
	nodeVar2 = ( ( ( ( ( u32( ( nodeVar1.x + 1048576.0 ) ) * 73856093u ) ^ ( u32( ( nodeVar1.y + 1048576.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeVar1.z + 1048576.0 ) ) * 83492791u ) ) * 747796405u ) + 2891336453u );
	nodeVar3 = ( ( ( nodeVar2 >> ( ( nodeVar2 >> 28u ) + 4u ) ) ^ nodeVar2 ) * 277803737u );
	nodeVar4 = ( nodeVar0 + vec3<f32>( 1.0, 0.0, 0.0 ) );
	nodeVar5 = ( ( ( ( ( u32( ( nodeVar4.x + 1048576.0 ) ) * 73856093u ) ^ ( u32( ( nodeVar4.y + 1048576.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeVar4.z + 1048576.0 ) ) * 83492791u ) ) * 747796405u ) + 2891336453u );
	nodeVar6 = ( ( ( nodeVar5 >> ( ( nodeVar5 >> 28u ) + 4u ) ) ^ nodeVar5 ) * 277803737u );
	nodeVar7 = fract( p );
	nodeVar8 = ( ( nodeVar7 * nodeVar7 ) * ( ( nodeVar7 * vec3<f32>( -2.0 ) ) + vec3<f32>( 3.0 ) ) );
	nodeVar9 = ( nodeVar0 + vec3<f32>( 0.0, 1.0, 0.0 ) );
	nodeVar10 = ( ( ( ( ( u32( ( nodeVar9.x + 1048576.0 ) ) * 73856093u ) ^ ( u32( ( nodeVar9.y + 1048576.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeVar9.z + 1048576.0 ) ) * 83492791u ) ) * 747796405u ) + 2891336453u );
	nodeVar11 = ( ( ( nodeVar10 >> ( ( nodeVar10 >> 28u ) + 4u ) ) ^ nodeVar10 ) * 277803737u );
	nodeVar12 = ( nodeVar0 + vec3<f32>( 1.0, 1.0, 0.0 ) );
	nodeVar13 = ( ( ( ( ( u32( ( nodeVar12.x + 1048576.0 ) ) * 73856093u ) ^ ( u32( ( nodeVar12.y + 1048576.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeVar12.z + 1048576.0 ) ) * 83492791u ) ) * 747796405u ) + 2891336453u );
	nodeVar14 = ( ( ( nodeVar13 >> ( ( nodeVar13 >> 28u ) + 4u ) ) ^ nodeVar13 ) * 277803737u );
	nodeVar15 = ( nodeVar0 + vec3<f32>( 0.0, 0.0, 1.0 ) );
	nodeVar16 = ( ( ( ( ( u32( ( nodeVar15.x + 1048576.0 ) ) * 73856093u ) ^ ( u32( ( nodeVar15.y + 1048576.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeVar15.z + 1048576.0 ) ) * 83492791u ) ) * 747796405u ) + 2891336453u );
	nodeVar17 = ( ( ( nodeVar16 >> ( ( nodeVar16 >> 28u ) + 4u ) ) ^ nodeVar16 ) * 277803737u );
	nodeVar18 = ( nodeVar0 + vec3<f32>( 1.0, 0.0, 1.0 ) );
	nodeVar19 = ( ( ( ( ( u32( ( nodeVar18.x + 1048576.0 ) ) * 73856093u ) ^ ( u32( ( nodeVar18.y + 1048576.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeVar18.z + 1048576.0 ) ) * 83492791u ) ) * 747796405u ) + 2891336453u );
	nodeVar20 = ( ( ( nodeVar19 >> ( ( nodeVar19 >> 28u ) + 4u ) ) ^ nodeVar19 ) * 277803737u );
	nodeVar21 = ( nodeVar0 + vec3<f32>( 0.0, 1.0, 1.0 ) );
	nodeVar22 = ( ( ( ( ( u32( ( nodeVar21.x + 1048576.0 ) ) * 73856093u ) ^ ( u32( ( nodeVar21.y + 1048576.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeVar21.z + 1048576.0 ) ) * 83492791u ) ) * 747796405u ) + 2891336453u );
	nodeVar23 = ( ( ( nodeVar22 >> ( ( nodeVar22 >> 28u ) + 4u ) ) ^ nodeVar22 ) * 277803737u );
	nodeVar24 = ( nodeVar0 + vec3<f32>( 1.0, 1.0, 1.0 ) );
	nodeVar25 = ( ( ( ( ( u32( ( nodeVar24.x + 1048576.0 ) ) * 73856093u ) ^ ( u32( ( nodeVar24.y + 1048576.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeVar24.z + 1048576.0 ) ) * 83492791u ) ) * 747796405u ) + 2891336453u );
	nodeVar26 = ( ( ( nodeVar25 >> ( ( nodeVar25 >> 28u ) + 4u ) ) ^ nodeVar25 ) * 277803737u );

	return ( ( mix( mix( mix( ( f32( ( ( nodeVar3 >> 22u ) ^ nodeVar3 ) ) * 2.3283064365386963e-10 ), ( f32( ( ( nodeVar6 >> 22u ) ^ nodeVar6 ) ) * 2.3283064365386963e-10 ), nodeVar8.x ), mix( ( f32( ( ( nodeVar11 >> 22u ) ^ nodeVar11 ) ) * 2.3283064365386963e-10 ), ( f32( ( ( nodeVar14 >> 22u ) ^ nodeVar14 ) ) * 2.3283064365386963e-10 ), nodeVar8.x ), nodeVar8.y ), mix( mix( ( f32( ( ( nodeVar17 >> 22u ) ^ nodeVar17 ) ) * 2.3283064365386963e-10 ), ( f32( ( ( nodeVar20 >> 22u ) ^ nodeVar20 ) ) * 2.3283064365386963e-10 ), nodeVar8.x ), mix( ( f32( ( ( nodeVar23 >> 22u ) ^ nodeVar23 ) ) * 2.3283064365386963e-10 ), ( f32( ( ( nodeVar26 >> 22u ) ^ nodeVar26 ) ) * 2.3283064365386963e-10 ), nodeVar8.x ), nodeVar8.y ), nodeVar8.z ) * 2.0 ) - 1.0 );

}


fn mx_fractal_noise_float ( p : vec3<f32>, octaves : i32, lacunarity : f32, diminish : f32 ) -> f32 {

	var nodeVar0 : f32;
	var nodeVar1 : f32;
	var nodeVar2 : vec3<f32>;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : i32;

	nodeVar0 = diminish;
	nodeVar1 = lacunarity;
	nodeVar2 = p;
	nodeVar3 = 0.0;
	nodeVar4 = 1.0;
	nodeVar5 = octaves;

	for ( var i : i32 = 0; i < nodeVar5; i ++ ) {

		nodeVar3 = ( nodeVar3 + ( nodeVar4 * mx_perlin_noise_float_1( nodeVar2 ) ) );
		nodeVar4 = ( nodeVar4 * nodeVar0 );
		nodeVar2 = ( nodeVar2 * vec3<f32>( nodeVar1 ) );

	}


	return nodeVar3;

}


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


fn roughnessToMip ( roughness : f32 ) -> f32 {

	var nodeVar0 : f32;

	nodeVar0 = 0.0;

	if ( ( roughness >= 0.8 ) ) {

		nodeVar0 = ( ( ( ( 1.0 - roughness ) * ( -1.0 - -2.0 ) ) / ( 1.0 - 0.8 ) ) + -2.0 );
		

	} else {


		if ( ( roughness >= 0.4 ) ) {

			nodeVar0 = ( ( ( ( 0.8 - roughness ) * ( 2.0 - -1.0 ) ) / ( 0.8 - 0.4 ) ) + -1.0 );
			

		} else {


			if ( ( roughness >= 0.305 ) ) {

				nodeVar0 = ( ( ( ( 0.4 - roughness ) * ( 3.0 - 2.0 ) ) / ( 0.4 - 0.305 ) ) + 2.0 );
				

			} else {


				if ( ( roughness >= 0.21 ) ) {

					nodeVar0 = ( ( ( ( 0.305 - roughness ) * ( 4.0 - 3.0 ) ) / ( 0.305 - 0.21 ) ) + 3.0 );
					

				} else {

					nodeVar0 = ( -2.0 * log2( ( 1.16 * roughness ) ) );
					

				}

				

			}

			

		}

		

	}


	return nodeVar0;

}


fn getFace ( direction : vec3<f32> ) -> f32 {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : f32;
	var nodeVar2 : f32;
	var nodeVar3 : f32;
	var nodeVar4 : f32;
	var nodeVar5 : f32;

	nodeVar0 = abs( direction );
	nodeVar1 = -1.0;

	if ( ( nodeVar0.x > nodeVar0.z ) ) {


		if ( ( nodeVar0.x > nodeVar0.y ) ) {


			if ( ( direction.x > 0.0 ) ) {

				nodeVar2 = 0.0;

			} else {

				nodeVar2 = 3.0;

			}

			nodeVar1 = nodeVar2;
			

		} else {


			if ( ( direction.y > 0.0 ) ) {

				nodeVar3 = 1.0;

			} else {

				nodeVar3 = 4.0;

			}

			nodeVar1 = nodeVar3;
			

		}

		

	} else {


		if ( ( nodeVar0.z > nodeVar0.y ) ) {


			if ( ( direction.z > 0.0 ) ) {

				nodeVar4 = 2.0;

			} else {

				nodeVar4 = 5.0;

			}

			nodeVar1 = nodeVar4;
			

		} else {


			if ( ( direction.y > 0.0 ) ) {

				nodeVar5 = 1.0;

			} else {

				nodeVar5 = 4.0;

			}

			nodeVar1 = nodeVar5;
			

		}

		

	}


	return nodeVar1;

}


fn getUV ( direction : vec3<f32>, face : f32 ) -> vec2<f32> {

	var nodeVar0 : vec2<f32>;

	nodeVar0 = vec2<f32>( 0.0, 0.0 );

	if ( ( face == 0.0 ) ) {

		nodeVar0 = ( vec2<f32>( direction.z, direction.y ) / vec2<f32>( abs( direction.x ) ) );
		

	} else {


		if ( ( face == 1.0 ) ) {

			nodeVar0 = ( vec2<f32>( ( - direction.x ), ( - direction.z ) ) / vec2<f32>( abs( direction.y ) ) );
			

		} else {


			if ( ( face == 2.0 ) ) {

				nodeVar0 = ( vec2<f32>( ( - direction.x ), direction.y ) / vec2<f32>( abs( direction.z ) ) );
				

			} else {


				if ( ( face == 3.0 ) ) {

					nodeVar0 = ( vec2<f32>( ( - direction.z ), direction.y ) / vec2<f32>( abs( direction.x ) ) );
					

				} else {


					if ( ( face == 4.0 ) ) {

						nodeVar0 = ( vec2<f32>( ( - direction.x ), direction.z ) / vec2<f32>( abs( direction.y ) ) );
						

					} else {

						nodeVar0 = ( vec2<f32>( direction.x, direction.y ) / vec2<f32>( abs( direction.z ) ) );
						

					}

					

				}

				

			}

			

		}

		

	}


	return ( vec2<f32>( 0.5 ) * ( nodeVar0 + vec2<f32>( 1.0 ) ) );

}




@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) positionLocal : vec3<f32>,
	@location( 2 ) @interpolate( flat, either ) nodeVarying3 : f32,
	@location( 3 ) @interpolate( flat, either ) nodeVarying4 : vec3<f32>,
	@location( 4 ) @interpolate( flat, either ) nodeVarying5 : vec3<f32>,
	@location( 5 ) v_positionWorld : vec3<f32>,
	@location( 6 ) nodeVarying7 : f32,
	@location( 7 ) v_normalWorldGeometry : vec3<f32>,
	@location( 8 ) v_normalViewGeometry : vec3<f32>,
	@location( 9 ) v_positionViewDirection : vec3<f32>,
	@location( 10 ) nodeVarying12 : vec2<f32>,
	@location( 11 ) nodeVarying13 : vec3<f32>,
	@location( 12 ) nodeVarying14 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar1 = ( nodeVarying3 == 4.0 );
	nodeVar2 = ( nodeVarying3 == 6.0 );
	nodeVar3 = ( nodeVar1 || nodeVar2 );

	if ( nodeVar3 ) {


		if ( nodeVar2 ) {

			nodeVar5 = floor( ( nodeVarying4 * vec3<f32>( 2.0 ) ) );
			nodeVar6 = ( ( ( u32( ( nodeVar5.x + 2097152.0 ) ) * 73856093u ) ^ ( u32( ( nodeVar5.y + 2097152.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeVar5.z + 2097152.0 ) ) * 83492791u ) );
			nodeVar8 = ( nodeVarying12.y * 2.4 );
			nodeVar9 = vec3<f32>( ( nodeVarying12.x * 0.5 ), ( nodeVarying12.y * 0.5 ), ( 0.1 + nodeVar8 ) );
			nodeVar10 = vec3<f32>( ( - nodeVar9.x ), ( - nodeVar9.y ), 0.1 );
			nodeVar11 = ( positionLocal - nodeVarying4 );
			normalLocal = nodeVarying13;
			nodeVar12 = normalize( cross( vec3<f32>( 0.0, 1.0, 0.0 ), normalLocal ) );
			nodeVar13 = vec3<f32>( dot( nodeVar11, nodeVar12 ), nodeVar11.y, 0.0 );
			nodeVar14 = normalize( ( positionLocal - ( object.nodeUniform0 * vec4<f32>( render.cameraPosition, 1.0 ) ).xyz ) );
			nodeVar15 = vec3<f32>( dot( nodeVar14, nodeVar12 ), nodeVar14.y, ( - dot( nodeVar14, normalLocal ) ) );
			nodeVar16 = ( ( vec3<f32>( ( - nodeVar9.x ), nodeVar10.y, 0.1 ) - nodeVar13 ) / nodeVar15 );
			nodeVar17 = ( ( vec3<f32>( nodeVar9.x, nodeVar9.y, ( 0.1 + 0.05 ) ) - nodeVar13 ) / nodeVar15 );
			nodeVar18 = max( nodeVar16, nodeVar17 );
			nodeVar19 = min( nodeVar16, nodeVar17 );
			nodeVar20 = max( max( nodeVar19.x, nodeVar19.y ), nodeVar19.z );
			nodeVar21 = ( ( ( nodeVar6 + 79599u ) * 747796405u ) + 2891336453u );
			nodeVar22 = ( ( ( nodeVar21 >> ( ( nodeVar21 >> 28u ) + 4u ) ) ^ nodeVar21 ) * 277803737u );
			nodeVar23 = step( 0.18, ( f32( ( ( nodeVar22 >> 22u ) ^ nodeVar22 ) ) * 2.3283064365386963e-10 ) );
			nodeVar24 = ( ( ( nodeVar6 + 39650u ) * 747796405u ) + 2891336453u );
			nodeVar25 = ( ( ( nodeVar24 >> ( ( nodeVar24 >> 28u ) + 4u ) ) ^ nodeVar24 ) * 277803737u );
			nodeVar27 = ( 0.1 + 1.2 );
			nodeVar28 = vec3<f32>( -0.45, nodeVar10.y, nodeVar27 );
			nodeVar29 = ( ( nodeVar28 - nodeVar13 ) / nodeVar15 );
			nodeVar30 = ( nodeVar9.z - 1.2 );
			nodeVar31 = vec3<f32>( 0.45, ( nodeVar10.y + 1.5 ), nodeVar30 );
			nodeVar32 = ( ( nodeVar31 - nodeVar13 ) / nodeVar15 );
			nodeVar33 = max( nodeVar29, nodeVar32 );
			nodeVar34 = min( nodeVar29, nodeVar32 );
			nodeVar35 = max( max( nodeVar34.x, nodeVar34.y ), nodeVar34.z );
			nodeVar36 = ( ( ( nodeVar6 + 34690u ) * 747796405u ) + 2891336453u );
			nodeVar37 = ( ( ( nodeVar36 >> ( ( nodeVar36 >> 28u ) + 4u ) ) ^ nodeVar36 ) * 277803737u );
			nodeVar38 = step( 0.45, ( f32( ( ( nodeVar37 >> 22u ) ^ nodeVar37 ) ) * 2.3283064365386963e-10 ) );
			nodeVar40 = ( ( ( nodeVar6 + 105097u ) * 747796405u ) + 2891336453u );
			nodeVar41 = ( ( ( nodeVar40 >> ( ( nodeVar40 >> 28u ) + 4u ) ) ^ nodeVar40 ) * 277803737u );
			nodeVar42 = ( f32( ( ( nodeVar41 >> 22u ) ^ nodeVar41 ) ) * 2.3283064365386963e-10 );
			nodeVar43 = mix( ( ( - nodeVar9.x ) + 0.7 ), ( nodeVar9.x - 0.7 ), step( 0.5, nodeVar42 ) );
			nodeVar44 = ( 0.1 + ( nodeVar8 * 0.5 ) );
			nodeVar45 = vec3<f32>( ( nodeVar43 - 0.75 ), nodeVar10.y, nodeVar44 );
			nodeVar46 = ( ( nodeVar45 - nodeVar13 ) / nodeVar15 );
			nodeVar47 = vec3<f32>( ( nodeVar43 + 0.75 ), ( nodeVar10.y + 1.0 ), ( nodeVar44 + 0.65 ) );
			nodeVar48 = ( ( nodeVar47 - nodeVar13 ) / nodeVar15 );
			nodeVar49 = max( nodeVar46, nodeVar48 );
			nodeVar50 = min( nodeVar46, nodeVar48 );
			nodeVar51 = max( max( nodeVar50.x, nodeVar50.y ), nodeVar50.z );
			nodeVar53 = ( nodeVar9.x * 0.62 );
			nodeVar54 = ( ( ( nodeVar6 + 40310u ) * 747796405u ) + 2891336453u );
			nodeVar55 = ( ( ( nodeVar54 >> ( ( nodeVar54 >> 28u ) + 4u ) ) ^ nodeVar54 ) * 277803737u );
			nodeVar56 = ( f32( ( ( nodeVar55 >> 22u ) ^ nodeVar55 ) ) * 2.3283064365386963e-10 );
			nodeVar57 = mix( 0.16, 0.3, nodeVar56 );
			nodeVar58 = vec3<f32>( ( nodeVar53 - nodeVar57 ), ( nodeVar10.y + 0.4 ), ( 0.1 + 0.25 ) );
			nodeVar59 = ( ( nodeVar58 - nodeVar13 ) / nodeVar15 );
			nodeVar60 = ( ( ( nodeVar6 + 66130u ) * 747796405u ) + 2891336453u );
			nodeVar61 = ( ( ( nodeVar60 >> ( ( nodeVar60 >> 28u ) + 4u ) ) ^ nodeVar60 ) * 277803737u );
			nodeVar62 = ( f32( ( ( nodeVar61 >> 22u ) ^ nodeVar61 ) ) * 2.3283064365386963e-10 );
			nodeVar63 = vec3<f32>( ( nodeVar53 + nodeVar57 ), ( ( nodeVar10.y + 0.4 ) + mix( 0.5, 1.2, nodeVar62 ) ), ( ( 0.1 + 0.25 ) + ( nodeVar57 * 2.0 ) ) );
			nodeVar64 = ( ( nodeVar63 - nodeVar13 ) / nodeVar15 );
			nodeVar65 = max( nodeVar59, nodeVar64 );
			nodeVar66 = min( nodeVar59, nodeVar64 );
			nodeVar67 = max( max( nodeVar66.x, nodeVar66.y ), nodeVar66.z );
			nodeVar69 = ( nodeVar9.x * 0.0 );
			nodeVar70 = ( ( ( nodeVar6 + 39610u ) * 747796405u ) + 2891336453u );
			nodeVar71 = ( ( ( nodeVar70 >> ( ( nodeVar70 >> 28u ) + 4u ) ) ^ nodeVar70 ) * 277803737u );
			nodeVar72 = ( f32( ( ( nodeVar71 >> 22u ) ^ nodeVar71 ) ) * 2.3283064365386963e-10 );
			nodeVar73 = mix( 0.16, 0.3, nodeVar72 );
			nodeVar74 = vec3<f32>( ( nodeVar69 - nodeVar73 ), ( nodeVar10.y + 0.4 ), ( 0.1 + 0.25 ) );
			nodeVar75 = ( ( nodeVar74 - nodeVar13 ) / nodeVar15 );
			nodeVar76 = ( ( ( nodeVar6 + 66030u ) * 747796405u ) + 2891336453u );
			nodeVar77 = ( ( ( nodeVar76 >> ( ( nodeVar76 >> 28u ) + 4u ) ) ^ nodeVar76 ) * 277803737u );
			nodeVar78 = ( f32( ( ( nodeVar77 >> 22u ) ^ nodeVar77 ) ) * 2.3283064365386963e-10 );
			nodeVar79 = vec3<f32>( ( nodeVar69 + nodeVar73 ), ( ( nodeVar10.y + 0.4 ) + mix( 0.5, 1.2, nodeVar78 ) ), ( ( 0.1 + 0.25 ) + ( nodeVar73 * 2.0 ) ) );
			nodeVar80 = ( ( nodeVar79 - nodeVar13 ) / nodeVar15 );
			nodeVar81 = max( nodeVar75, nodeVar80 );
			nodeVar82 = min( nodeVar75, nodeVar80 );
			nodeVar83 = max( max( nodeVar82.x, nodeVar82.y ), nodeVar82.z );
			nodeVar85 = ( nodeVar9.x * -0.62 );
			nodeVar86 = ( ( ( nodeVar6 + 38910u ) * 747796405u ) + 2891336453u );
			nodeVar87 = ( ( ( nodeVar86 >> ( ( nodeVar86 >> 28u ) + 4u ) ) ^ nodeVar86 ) * 277803737u );
			nodeVar88 = ( f32( ( ( nodeVar87 >> 22u ) ^ nodeVar87 ) ) * 2.3283064365386963e-10 );
			nodeVar89 = mix( 0.16, 0.3, nodeVar88 );
			nodeVar90 = vec3<f32>( ( nodeVar85 - nodeVar89 ), ( nodeVar10.y + 0.4 ), ( 0.1 + 0.25 ) );
			nodeVar91 = ( ( nodeVar90 - nodeVar13 ) / nodeVar15 );
			nodeVar92 = ( ( ( nodeVar6 + 65930u ) * 747796405u ) + 2891336453u );
			nodeVar93 = ( ( ( nodeVar92 >> ( ( nodeVar92 >> 28u ) + 4u ) ) ^ nodeVar92 ) * 277803737u );
			nodeVar94 = ( f32( ( ( nodeVar93 >> 22u ) ^ nodeVar93 ) ) * 2.3283064365386963e-10 );
			nodeVar95 = vec3<f32>( ( nodeVar85 + nodeVar89 ), ( ( nodeVar10.y + 0.4 ) + mix( 0.5, 1.2, nodeVar94 ) ), ( ( 0.1 + 0.25 ) + ( nodeVar89 * 2.0 ) ) );
			nodeVar96 = ( ( nodeVar95 - nodeVar13 ) / nodeVar15 );
			nodeVar97 = max( nodeVar91, nodeVar96 );
			nodeVar98 = min( nodeVar91, nodeVar96 );
			nodeVar99 = max( max( nodeVar98.x, nodeVar98.y ), nodeVar98.z );
			nodeVar101 = vec3<f32>( ( ( - nodeVar9.x ) + 0.15 ), nodeVar10.y, ( 0.1 + 0.06 ) );
			nodeVar102 = ( ( nodeVar101 - nodeVar13 ) / nodeVar15 );
			nodeVar103 = vec3<f32>( ( nodeVar9.x - 0.15 ), ( nodeVar10.y + 0.4 ), ( 0.1 + 0.95 ) );
			nodeVar104 = ( ( nodeVar103 - nodeVar13 ) / nodeVar15 );
			nodeVar105 = max( nodeVar102, nodeVar104 );
			nodeVar106 = min( nodeVar102, nodeVar104 );
			nodeVar107 = max( max( nodeVar106.x, nodeVar106.y ), nodeVar106.z );
			nodeVar108 = max( ( ( nodeVar10 - nodeVar13 ) / nodeVar15 ), ( ( nodeVar9 - nodeVar13 ) / nodeVar15 ) );
			nodeVar109 = min( min( nodeVar108.x, nodeVar108.y ), nodeVar108.z );
			nodeVar110 = ( ( ( min( min( nodeVar105.x, nodeVar105.y ), nodeVar105.z ) > nodeVar107 ) && ( nodeVar107 > 0.0 ) ) && ( nodeVar107 < nodeVar109 ) );

			if ( nodeVar110 ) {

				nodeVar100 = nodeVar107;

			} else {

				nodeVar100 = nodeVar109;

			}

			nodeVar111 = ( ( ( ( min( min( nodeVar97.x, nodeVar97.y ), nodeVar97.z ) > nodeVar99 ) && ( nodeVar99 > 0.0 ) ) && ( nodeVar88 > 0.15 ) ) && ( nodeVar99 < nodeVar100 ) );

			if ( nodeVar111 ) {

				nodeVar84 = nodeVar99;

			} else {

				nodeVar84 = nodeVar100;

			}

			nodeVar112 = ( ( ( ( min( min( nodeVar81.x, nodeVar81.y ), nodeVar81.z ) > nodeVar83 ) && ( nodeVar83 > 0.0 ) ) && ( nodeVar72 > 0.15 ) ) && ( nodeVar83 < nodeVar84 ) );

			if ( nodeVar112 ) {

				nodeVar68 = nodeVar83;

			} else {

				nodeVar68 = nodeVar84;

			}

			nodeVar113 = ( ( ( ( min( min( nodeVar65.x, nodeVar65.y ), nodeVar65.z ) > nodeVar67 ) && ( nodeVar67 > 0.0 ) ) && ( nodeVar56 > 0.15 ) ) && ( nodeVar67 < nodeVar68 ) );

			if ( nodeVar113 ) {

				nodeVar52 = nodeVar67;

			} else {

				nodeVar52 = nodeVar68;

			}

			nodeVar114 = ( ( ( min( min( nodeVar49.x, nodeVar49.y ), nodeVar49.z ) > nodeVar51 ) && ( nodeVar51 > 0.0 ) ) && ( nodeVar51 < nodeVar52 ) );

			if ( nodeVar114 ) {

				nodeVar39 = nodeVar51;

			} else {

				nodeVar39 = nodeVar52;

			}

			nodeVar115 = ( ( ( ( min( min( nodeVar33.x, nodeVar33.y ), nodeVar33.z ) > nodeVar35 ) && ( nodeVar35 > 0.0 ) ) && ( nodeVar38 > 0.5 ) ) && ( nodeVar35 < nodeVar39 ) );

			if ( nodeVar115 ) {

				nodeVar26 = nodeVar35;

			} else {

				nodeVar26 = nodeVar39;

			}

			nodeVar116 = ( ( ( ( min( min( nodeVar18.x, nodeVar18.y ), nodeVar18.z ) > nodeVar20 ) && ( nodeVar20 > 0.0 ) ) && ( ( ( 1.0 - nodeVar23 ) * step( 0.45, ( f32( ( ( nodeVar25 >> 22u ) ^ nodeVar25 ) ) * 2.3283064365386963e-10 ) ) ) > 0.5 ) ) && ( nodeVar20 < nodeVar26 ) );

			if ( nodeVar116 ) {

				nodeVar7 = ( mix( vec3<f32>( 0.0722718506743852, 0.08228270712149792, 0.09758734713304495 ), vec3<f32>( 0.00972121731707524, 0.010960094003125918, 0.012983032338510335 ), step( 0.62, fract( ( ( nodeVar13 + ( nodeVar15 * vec3<f32>( nodeVar20 ) ) ).y * 7.0 ) ) ) ) * vec3<f32>( 0.8 ) );

			} else {


				if ( nodeVar115 ) {

					nodeVar119 = ( nodeVar13 + ( nodeVar15 * vec3<f32>( nodeVar35 ) ) );
					nodeVar120 = ( ( nodeVar119.z - nodeVar27 ) / ( nodeVar30 - nodeVar27 ) );
					nodeVar121 = ( ( ( nodeVar6 + 91u ) + ( u32( floor( ( nodeVar120 * 8.0 ) ) ) * 197u ) ) + ( u32( floor( ( ( ( nodeVar119 - nodeVar28 ) / ( nodeVar31 - nodeVar28 ) ).y * 4.0 ) ) ) * 4099u ) );
					nodeVar122 = ( ( nodeVar121 * 747796405u ) + 2891336453u );
					nodeVar123 = ( ( ( nodeVar122 >> ( ( nodeVar122 >> 28u ) + 4u ) ) ^ nodeVar122 ) * 277803737u );
					nodeVar124 = ( ( f32( ( ( nodeVar123 >> 22u ) ^ nodeVar123 ) ) * 2.3283064365386963e-10 ) * 6.0 );

					if ( ( nodeVar124 > 5.0 ) ) {

						nodeVar118 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

					} else {


						if ( ( nodeVar124 > 4.0 ) ) {

							nodeVar125 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

						} else {


							if ( ( nodeVar124 > 3.0 ) ) {

								nodeVar126 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

							} else {


								if ( ( nodeVar124 > 2.0 ) ) {

									nodeVar127 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

								} else {


									if ( ( nodeVar124 > 1.0 ) ) {

										nodeVar128 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

									} else {

										nodeVar128 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

									}

									nodeVar127 = nodeVar128;

								}

								nodeVar126 = nodeVar127;

							}

							nodeVar125 = nodeVar126;

						}

						nodeVar118 = nodeVar125;

					}

					nodeVar129 = fract( ( ( ( nodeVar119 - nodeVar28 ) / ( nodeVar31 - nodeVar28 ) ).y * 4.0 ) );
					nodeVar130 = ( ( ( nodeVar121 + 1u ) * 747796405u ) + 2891336453u );
					nodeVar131 = ( ( ( nodeVar130 >> ( ( nodeVar130 >> 28u ) + 4u ) ) ^ nodeVar130 ) * 277803737u );
					nodeVar132 = ( f32( ( ( nodeVar131 >> 22u ) ^ nodeVar131 ) ) * 2.3283064365386963e-10 );
					nodeVar133 = ( ( ( nodeVar6 + 119831u ) * 747796405u ) + 2891336453u );
					nodeVar134 = ( ( ( nodeVar133 >> ( ( nodeVar133 >> 28u ) + 4u ) ) ^ nodeVar133 ) * 277803737u );
					nodeVar117 = ( mix( mix( vec3<f32>( 0.015208514418949472, 0.012983032338510335, 0.010960094003125918 ), nodeVar118, ( ( ( step( 0.08, nodeVar129 ) * step( nodeVar129, mix( 0.4, 0.85, fract( ( nodeVar132 * 5.0 ) ) ) ) ) * step( abs( ( fract( ( nodeVar120 * 8.0 ) ) - 0.5 ) ), mix( 0.26, 0.46, fract( ( nodeVar132 * 13.0 ) ) ) ) ) * step( 0.15, nodeVar132 ) ) ), mix( vec3<f32>( 0.14702726648767014, 0.10224173307914941, 0.05612849004241121 ), vec3<f32>( 0.623960391667596, 0.6038273388475408, 0.55201140150344 ), step( 0.5, ( f32( ( ( nodeVar134 >> 22u ) ^ nodeVar134 ) ) * 2.3283064365386963e-10 ) ) ), step( nodeVar129, 0.08 ) ) * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeVar23 ), clamp( ( ( nodeVar119.z - 0.1 ) / nodeVar8 ), 0.0, 1.0 ) ) ) );

				} else {


					if ( nodeVar114 ) {

						nodeVar136 = ( ( ( nodeVar6 + 119831u ) * 747796405u ) + 2891336453u );
						nodeVar137 = ( ( ( nodeVar136 >> ( ( nodeVar136 >> 28u ) + 4u ) ) ^ nodeVar136 ) * 277803737u );
						nodeVar139 = ( nodeVar13 + ( nodeVar15 * vec3<f32>( nodeVar51 ) ) );

						if ( ( ( ( nodeVar139 - nodeVar45 ) / ( nodeVar47 - nodeVar45 ) ).y > 0.92 ) ) {

							nodeVar138 = 1.3;

						} else {

							nodeVar138 = 0.85;

						}

						nodeVar135 = ( ( mix( vec3<f32>( 0.06847816983662762, 0.05126945836711539, 0.036889450395083165 ), vec3<f32>( 0.025186859622305935, 0.033104766565152086, 0.04231141061442144 ), ( f32( ( ( nodeVar137 >> 22u ) ^ nodeVar137 ) ) * 2.3283064365386963e-10 ) ) * vec3<f32>( nodeVar138 ) ) * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeVar23 ), clamp( ( ( nodeVar139.z - 0.1 ) / nodeVar8 ), 0.0, 1.0 ) ) ) );

					} else {


						if ( nodeVar113 ) {

							nodeVar142 = ( nodeVar13 + ( nodeVar15 * vec3<f32>( nodeVar67 ) ) );

							if ( ( ( ( nodeVar142 - nodeVar58 ) / ( nodeVar63 - nodeVar58 ) ).y > 0.6 ) ) {

								nodeVar144 = ( nodeVar62 * 6.0 );

								if ( ( nodeVar144 > 5.0 ) ) {

									nodeVar143 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

								} else {


									if ( ( nodeVar144 > 4.0 ) ) {

										nodeVar145 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

									} else {


										if ( ( nodeVar144 > 3.0 ) ) {

											nodeVar146 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

										} else {


											if ( ( nodeVar144 > 2.0 ) ) {

												nodeVar147 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

											} else {


												if ( ( nodeVar144 > 1.0 ) ) {

													nodeVar148 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

												} else {

													nodeVar148 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

												}

												nodeVar147 = nodeVar148;

											}

											nodeVar146 = nodeVar147;

										}

										nodeVar145 = nodeVar146;

									}

									nodeVar143 = nodeVar145;

								}

								nodeVar141 = nodeVar143;

							} else {

								nodeVar141 = vec3<f32>( 0.7011018919268015, 0.6653872982754769, 0.5972017883558645 );

							}

							nodeVar140 = ( nodeVar141 * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeVar23 ), clamp( ( ( nodeVar142.z - 0.1 ) / nodeVar8 ), 0.0, 1.0 ) ) ) );

						} else {


							if ( nodeVar112 ) {

								nodeVar151 = ( nodeVar13 + ( nodeVar15 * vec3<f32>( nodeVar83 ) ) );

								if ( ( ( ( nodeVar151 - nodeVar74 ) / ( nodeVar79 - nodeVar74 ) ).y > 0.6 ) ) {

									nodeVar153 = ( nodeVar78 * 6.0 );

									if ( ( nodeVar153 > 5.0 ) ) {

										nodeVar152 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

									} else {


										if ( ( nodeVar153 > 4.0 ) ) {

											nodeVar154 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

										} else {


											if ( ( nodeVar153 > 3.0 ) ) {

												nodeVar155 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

											} else {


												if ( ( nodeVar153 > 2.0 ) ) {

													nodeVar156 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

												} else {


													if ( ( nodeVar153 > 1.0 ) ) {

														nodeVar157 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

													} else {

														nodeVar157 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

													}

													nodeVar156 = nodeVar157;

												}

												nodeVar155 = nodeVar156;

											}

											nodeVar154 = nodeVar155;

										}

										nodeVar152 = nodeVar154;

									}

									nodeVar150 = nodeVar152;

								} else {

									nodeVar150 = vec3<f32>( 0.7011018919268015, 0.6653872982754769, 0.5972017883558645 );

								}

								nodeVar149 = ( nodeVar150 * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeVar23 ), clamp( ( ( nodeVar151.z - 0.1 ) / nodeVar8 ), 0.0, 1.0 ) ) ) );

							} else {


								if ( nodeVar111 ) {

									nodeVar160 = ( nodeVar13 + ( nodeVar15 * vec3<f32>( nodeVar99 ) ) );

									if ( ( ( ( nodeVar160 - nodeVar90 ) / ( nodeVar95 - nodeVar90 ) ).y > 0.6 ) ) {

										nodeVar162 = ( nodeVar94 * 6.0 );

										if ( ( nodeVar162 > 5.0 ) ) {

											nodeVar161 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

										} else {


											if ( ( nodeVar162 > 4.0 ) ) {

												nodeVar163 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

											} else {


												if ( ( nodeVar162 > 3.0 ) ) {

													nodeVar164 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

												} else {


													if ( ( nodeVar162 > 2.0 ) ) {

														nodeVar165 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

													} else {


														if ( ( nodeVar162 > 1.0 ) ) {

															nodeVar166 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

														} else {

															nodeVar166 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

														}

														nodeVar165 = nodeVar166;

													}

													nodeVar164 = nodeVar165;

												}

												nodeVar163 = nodeVar164;

											}

											nodeVar161 = nodeVar163;

										}

										nodeVar159 = nodeVar161;

									} else {

										nodeVar159 = vec3<f32>( 0.7011018919268015, 0.6653872982754769, 0.5972017883558645 );

									}

									nodeVar158 = ( nodeVar159 * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeVar23 ), clamp( ( ( nodeVar160.z - 0.1 ) / nodeVar8 ), 0.0, 1.0 ) ) ) );

								} else {


									if ( nodeVar110 ) {

										nodeVar169 = ( nodeVar13 + ( nodeVar15 * vec3<f32>( nodeVar107 ) ) );

										if ( ( ( ( nodeVar169 - nodeVar101 ) / ( nodeVar103 - nodeVar101 ) ).y > 0.93 ) ) {

											nodeVar168 = vec3<f32>( 0.6938717612856897, 0.6583748172725346, 0.5775804404214573 );

										} else {

											nodeVar168 = vec3<f32>( 0.035601314869097636, 0.02955683443236377, 0.023153366173251363 );

										}

										nodeVar167 = ( nodeVar168 * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeVar23 ), clamp( ( ( nodeVar169.z - 0.1 ) / nodeVar8 ), 0.0, 1.0 ) ) ) );

									} else {

										nodeVar171 = ( nodeVar13 + ( nodeVar15 * vec3<f32>( nodeVar109 ) ) );
										nodeVar172 = ( ( nodeVar171 - nodeVar10 ) / ( nodeVar9 - nodeVar10 ) );
										nodeVar173 = ( nodeVar172.z > 0.998 );

										if ( nodeVar173 ) {


											if ( bool( nodeVar38 ) ) {

												nodeVar176 = ( nodeVarying12.x * 2.5 );
												nodeVar177 = ( ( ( nodeVar6 + 31u ) + ( u32( floor( ( nodeVar172.x * nodeVar176 ) ) ) * 197u ) ) + ( u32( floor( ( nodeVar172.y * 4.0 ) ) ) * 4099u ) );
												nodeVar178 = ( ( nodeVar177 * 747796405u ) + 2891336453u );
												nodeVar179 = ( ( ( nodeVar178 >> ( ( nodeVar178 >> 28u ) + 4u ) ) ^ nodeVar178 ) * 277803737u );
												nodeVar180 = ( ( f32( ( ( nodeVar179 >> 22u ) ^ nodeVar179 ) ) * 2.3283064365386963e-10 ) * 6.0 );

												if ( ( nodeVar180 > 5.0 ) ) {

													nodeVar175 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

												} else {


													if ( ( nodeVar180 > 4.0 ) ) {

														nodeVar181 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

													} else {


														if ( ( nodeVar180 > 3.0 ) ) {

															nodeVar182 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

														} else {


															if ( ( nodeVar180 > 2.0 ) ) {

																nodeVar183 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

															} else {


																if ( ( nodeVar180 > 1.0 ) ) {

																	nodeVar184 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

																} else {

																	nodeVar184 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

																}

																nodeVar183 = nodeVar184;

															}

															nodeVar182 = nodeVar183;

														}

														nodeVar181 = nodeVar182;

													}

													nodeVar175 = nodeVar181;

												}

												nodeVar185 = fract( ( nodeVar172.y * 4.0 ) );
												nodeVar186 = ( ( ( nodeVar177 + 1u ) * 747796405u ) + 2891336453u );
												nodeVar187 = ( ( ( nodeVar186 >> ( ( nodeVar186 >> 28u ) + 4u ) ) ^ nodeVar186 ) * 277803737u );
												nodeVar188 = ( f32( ( ( nodeVar187 >> 22u ) ^ nodeVar187 ) ) * 2.3283064365386963e-10 );
												nodeVar189 = ( ( ( nodeVar6 + 119831u ) * 747796405u ) + 2891336453u );
												nodeVar190 = ( ( ( nodeVar189 >> ( ( nodeVar189 >> 28u ) + 4u ) ) ^ nodeVar189 ) * 277803737u );
												nodeVar174 = mix( mix( vec3<f32>( 0.015208514418949472, 0.012983032338510335, 0.010960094003125918 ), nodeVar175, ( ( ( step( 0.08, nodeVar185 ) * step( nodeVar185, mix( 0.4, 0.85, fract( ( nodeVar188 * 5.0 ) ) ) ) ) * step( abs( ( fract( ( nodeVar172.x * nodeVar176 ) ) - 0.5 ) ), mix( 0.26, 0.46, fract( ( nodeVar188 * 13.0 ) ) ) ) ) * step( 0.15, nodeVar188 ) ) ), mix( vec3<f32>( 0.14702726648767014, 0.10224173307914941, 0.05612849004241121 ), vec3<f32>( 0.623960391667596, 0.6038273388475408, 0.55201140150344 ), step( 0.5, ( f32( ( ( nodeVar190 >> 22u ) ^ nodeVar190 ) ) * 2.3283064365386963e-10 ) ) ), step( nodeVar185, 0.08 ) );

											} else {

												nodeVar192 = ( ( ( nodeVar6 + 119831u ) * 747796405u ) + 2891336453u );
												nodeVar193 = ( ( ( nodeVar192 >> ( ( nodeVar192 >> 28u ) + 4u ) ) ^ nodeVar192 ) * 277803737u );
												nodeVar194 = ( ( f32( ( ( nodeVar193 >> 22u ) ^ nodeVar193 ) ) * 2.3283064365386963e-10 ) * 6.0 );

												if ( ( nodeVar194 > 5.0 ) ) {

													nodeVar191 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

												} else {


													if ( ( nodeVar194 > 4.0 ) ) {

														nodeVar195 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

													} else {


														if ( ( nodeVar194 > 3.0 ) ) {

															nodeVar196 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

														} else {


															if ( ( nodeVar194 > 2.0 ) ) {

																nodeVar197 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

															} else {


																if ( ( nodeVar194 > 1.0 ) ) {

																	nodeVar198 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

																} else {

																	nodeVar198 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

																}

																nodeVar197 = nodeVar198;

															}

															nodeVar196 = nodeVar197;

														}

														nodeVar195 = nodeVar196;

													}

													nodeVar191 = nodeVar195;

												}

												nodeVar199 = mix( 0.3, 0.7, nodeVar42 );
												nodeVar201 = ( ( ( nodeVar6 + 41260u ) * 747796405u ) + 2891336453u );
												nodeVar202 = ( ( ( nodeVar201 >> ( ( nodeVar201 >> 28u ) + 4u ) ) ^ nodeVar201 ) * 277803737u );
												nodeVar203 = ( ( f32( ( ( nodeVar202 >> 22u ) ^ nodeVar202 ) ) * 2.3283064365386963e-10 ) * 6.0 );

												if ( ( nodeVar203 > 5.0 ) ) {

													nodeVar200 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

												} else {


													if ( ( nodeVar203 > 4.0 ) ) {

														nodeVar204 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

													} else {


														if ( ( nodeVar203 > 3.0 ) ) {

															nodeVar205 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

														} else {


															if ( ( nodeVar203 > 2.0 ) ) {

																nodeVar206 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

															} else {


																if ( ( nodeVar203 > 1.0 ) ) {

																	nodeVar207 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

																} else {

																	nodeVar207 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

																}

																nodeVar206 = nodeVar207;

															}

															nodeVar205 = nodeVar206;

														}

														nodeVar204 = nodeVar205;

													}

													nodeVar200 = nodeVar204;

												}

												nodeVar174 = mix( mix( mix( nodeVar191, mix( vec3<f32>( 0.6866853124288864, 0.6444796819634361, 0.5647115056965487 ), vec3<f32>( 0.5209955731953768, 0.48514994004665124, 0.4232676699760063 ), nodeVar42 ), 0.45 ), vec3<f32>( 0.8069522576650873, 0.775822218312646, 0.7011018919268015 ), ( smoothstep( 0.10600000000000001, 0.094, abs( ( nodeVar172.x - nodeVar199 ) ) ) * smoothstep( 0.14600000000000002, 0.134, abs( ( nodeVar172.y - 0.62 ) ) ) ) ), nodeVar200, ( ( smoothstep( 0.081, 0.06899999999999999, abs( ( nodeVar172.x - nodeVar199 ) ) ) * smoothstep( 0.10600000000000001, 0.094, abs( ( nodeVar172.y - 0.62 ) ) ) ) * 0.85 ) );

											}

											nodeVar170 = nodeVar174;

										} else {


											if ( ( nodeVar172.y > 0.998 ) ) {

												nodeVar210 = ( ( ( nodeVar6 + 25910u ) * 747796405u ) + 2891336453u );
												nodeVar211 = ( ( ( nodeVar210 >> ( ( nodeVar210 >> 28u ) + 4u ) ) ^ nodeVar210 ) * 277803737u );

												if ( ( ( f32( ( ( nodeVar211 >> 22u ) ^ nodeVar211 ) ) * 2.3283064365386963e-10 ) > 0.65 ) ) {

													nodeVar212 = ( ( ( nodeVar6 + 86350u ) * 747796405u ) + 2891336453u );
													nodeVar213 = ( ( ( nodeVar212 >> ( ( nodeVar212 >> 28u ) + 4u ) ) ^ nodeVar212 ) * 277803737u );
													nodeVar209 = mix( vec3<f32>( 1.0, 0.6938717612856897, 0.3515325994898463 ), vec3<f32>( 1.0, 0.8227857543924378, 0.5972017883558645 ), ( f32( ( ( nodeVar213 >> 22u ) ^ nodeVar213 ) ) * 2.3283064365386963e-10 ) );

												} else {

													nodeVar214 = ( ( ( nodeVar6 + 59550u ) * 747796405u ) + 2891336453u );
													nodeVar215 = ( ( ( nodeVar214 >> ( ( nodeVar214 >> 28u ) + 4u ) ) ^ nodeVar214 ) * 277803737u );
													nodeVar209 = mix( vec3<f32>( 0.85499260812105, 0.8879231178794776, 0.9215818562755338 ), vec3<f32>( 0.7379104087672317, 0.8069522576650873, 1.0 ), ( f32( ( ( nodeVar215 >> 22u ) ^ nodeVar215 ) ) * 2.3283064365386963e-10 ) );

												}

												nodeVar208 = mix( vec3<f32>( 0.8148465722120952, 0.7912979403281551, 0.7379104087672317 ), ( nodeVar209 * vec3<f32>( mix( 0.5, 6.0, nodeVar23 ) ) ), ( step( abs( ( fract( ( nodeVar172.x * 3.0 ) ) - 0.5 ) ), 0.3 ) * step( abs( ( fract( ( nodeVar172.z * 4.0 ) ) - 0.5 ) ), 0.15 ) ) );

											} else {


												if ( ( nodeVar172.y < 0.002 ) ) {

													nodeVar218 = ( ( ( nodeVar6 + 27510u ) * 747796405u ) + 2891336453u );
													nodeVar219 = ( ( ( nodeVar218 >> ( ( nodeVar218 >> 28u ) + 4u ) ) ^ nodeVar218 ) * 277803737u );

													if ( ( ( f32( ( ( nodeVar219 >> 22u ) ^ nodeVar219 ) ) * 2.3283064365386963e-10 ) > 0.75 ) ) {

														nodeVar217 = mix( vec3<f32>( 0.01764195448412081, 0.019382360952473074, 0.023153366173251363 ), vec3<f32>( 0.623960391667596, 0.6038273388475408, 0.5394794890033748 ), tsl_mod_float( ( floor( ( nodeVar172.x * 8.0 ) ) + floor( ( nodeVar172.z * 10.0 ) ) ), 2.0 ) );

													} else {

														nodeVar220 = ( ( ( nodeVar6 + 119831u ) * 747796405u ) + 2891336453u );
														nodeVar221 = ( ( ( nodeVar220 >> ( ( nodeVar220 >> 28u ) + 4u ) ) ^ nodeVar220 ) * 277803737u );
														nodeVar217 = ( mix( vec3<f32>( 0.46207699964472876, 0.4286904966038916, 0.36130677977297226 ), vec3<f32>( 0.3005437944049895, 0.2746773120495699, 0.22696587349938613 ), ( f32( ( ( nodeVar221 >> 22u ) ^ nodeVar221 ) ) * 2.3283064365386963e-10 ) ) * vec3<f32>( ( 1.0 - ( max( step( 0.93, fract( ( nodeVar172.x * 8.0 ) ) ), step( 0.93, fract( ( nodeVar172.z * 10.0 ) ) ) ) * 0.35 ) ) ) );

													}

													nodeVar216 = nodeVar217;

												} else {


													if ( bool( nodeVar38 ) ) {

														nodeVar224 = ( nodeVar8 * 2.5 );
														nodeVar225 = ( ( ( nodeVar6 + 57u ) + ( u32( floor( ( nodeVar172.z * nodeVar224 ) ) ) * 197u ) ) + ( u32( floor( ( nodeVar172.y * 4.0 ) ) ) * 4099u ) );
														nodeVar226 = ( ( nodeVar225 * 747796405u ) + 2891336453u );
														nodeVar227 = ( ( ( nodeVar226 >> ( ( nodeVar226 >> 28u ) + 4u ) ) ^ nodeVar226 ) * 277803737u );
														nodeVar228 = ( ( f32( ( ( nodeVar227 >> 22u ) ^ nodeVar227 ) ) * 2.3283064365386963e-10 ) * 6.0 );

														if ( ( nodeVar228 > 5.0 ) ) {

															nodeVar223 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

														} else {


															if ( ( nodeVar228 > 4.0 ) ) {

																nodeVar229 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

															} else {


																if ( ( nodeVar228 > 3.0 ) ) {

																	nodeVar230 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

																} else {


																	if ( ( nodeVar228 > 2.0 ) ) {

																		nodeVar231 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

																	} else {


																		if ( ( nodeVar228 > 1.0 ) ) {

																			nodeVar232 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

																		} else {

																			nodeVar232 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

																		}

																		nodeVar231 = nodeVar232;

																	}

																	nodeVar230 = nodeVar231;

																}

																nodeVar229 = nodeVar230;

															}

															nodeVar223 = nodeVar229;

														}

														nodeVar233 = fract( ( nodeVar172.y * 4.0 ) );
														nodeVar234 = ( ( ( nodeVar225 + 1u ) * 747796405u ) + 2891336453u );
														nodeVar235 = ( ( ( nodeVar234 >> ( ( nodeVar234 >> 28u ) + 4u ) ) ^ nodeVar234 ) * 277803737u );
														nodeVar236 = ( f32( ( ( nodeVar235 >> 22u ) ^ nodeVar235 ) ) * 2.3283064365386963e-10 );
														nodeVar237 = ( ( ( nodeVar6 + 119831u ) * 747796405u ) + 2891336453u );
														nodeVar238 = ( ( ( nodeVar237 >> ( ( nodeVar237 >> 28u ) + 4u ) ) ^ nodeVar237 ) * 277803737u );
														nodeVar222 = mix( mix( vec3<f32>( 0.015208514418949472, 0.012983032338510335, 0.010960094003125918 ), nodeVar223, ( ( ( step( 0.08, nodeVar233 ) * step( nodeVar233, mix( 0.4, 0.85, fract( ( nodeVar236 * 5.0 ) ) ) ) ) * step( abs( ( fract( ( nodeVar172.z * nodeVar224 ) ) - 0.5 ) ), mix( 0.26, 0.46, fract( ( nodeVar236 * 13.0 ) ) ) ) ) * step( 0.15, nodeVar236 ) ) ), mix( vec3<f32>( 0.14702726648767014, 0.10224173307914941, 0.05612849004241121 ), vec3<f32>( 0.623960391667596, 0.6038273388475408, 0.55201140150344 ), step( 0.5, ( f32( ( ( nodeVar238 >> 22u ) ^ nodeVar238 ) ) * 2.3283064365386963e-10 ) ) ), step( nodeVar233, 0.08 ) );

													} else {

														nodeVar222 = mix( vec3<f32>( 0.6866853124288864, 0.6444796819634361, 0.5647115056965487 ), vec3<f32>( 0.5209955731953768, 0.48514994004665124, 0.4232676699760063 ), nodeVar42 );

													}

													nodeVar216 = nodeVar222;

												}

												nodeVar208 = nodeVar216;

											}

											nodeVar170 = nodeVar208;

										}


										if ( nodeVar173 ) {

											nodeVar239 = ( ( smoothstep( 0.0, 0.15, nodeVar172.x ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar172.x ) ) ) * ( smoothstep( 0.0, 0.15, nodeVar172.y ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar172.y ) ) ) );

										} else {


											if ( ( ( nodeVar172.y < 0.002 ) || ( nodeVar172.y > 0.998 ) ) ) {

												nodeVar240 = ( ( smoothstep( 0.0, 0.15, nodeVar172.x ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar172.x ) ) ) * ( smoothstep( 0.0, 0.15, nodeVar172.z ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar172.z ) ) ) );

											} else {

												nodeVar240 = ( ( smoothstep( 0.0, 0.15, nodeVar172.y ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar172.y ) ) ) * ( smoothstep( 0.0, 0.15, nodeVar172.z ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar172.z ) ) ) );

											}

											nodeVar239 = nodeVar240;

										}

										nodeVar167 = ( ( nodeVar170 * vec3<f32>( mix( 0.72, 1.0, nodeVar239 ) ) ) * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeVar23 ), clamp( ( ( nodeVar171.z - 0.1 ) / nodeVar8 ), 0.0, 1.0 ) ) ) );

									}

									nodeVar158 = nodeVar167;

								}

								nodeVar149 = nodeVar158;

							}

							nodeVar140 = nodeVar149;

						}

						nodeVar135 = nodeVar140;

					}

					nodeVar117 = nodeVar135;

				}

				nodeVar7 = nodeVar117;

			}

			nodeVar242 = ( ( ( nodeVar6 + 25910u ) * 747796405u ) + 2891336453u );
			nodeVar243 = ( ( ( nodeVar242 >> ( ( nodeVar242 >> 28u ) + 4u ) ) ^ nodeVar242 ) * 277803737u );

			if ( ( ( f32( ( ( nodeVar243 >> 22u ) ^ nodeVar243 ) ) * 2.3283064365386963e-10 ) > 0.65 ) ) {

				nodeVar244 = ( ( ( nodeVar6 + 86350u ) * 747796405u ) + 2891336453u );
				nodeVar245 = ( ( ( nodeVar244 >> ( ( nodeVar244 >> 28u ) + 4u ) ) ^ nodeVar244 ) * 277803737u );
				nodeVar241 = mix( vec3<f32>( 1.0, 0.6938717612856897, 0.3515325994898463 ), vec3<f32>( 1.0, 0.8227857543924378, 0.5972017883558645 ), ( f32( ( ( nodeVar245 >> 22u ) ^ nodeVar245 ) ) * 2.3283064365386963e-10 ) );

			} else {

				nodeVar246 = ( ( ( nodeVar6 + 59550u ) * 747796405u ) + 2891336453u );
				nodeVar247 = ( ( ( nodeVar246 >> ( ( nodeVar246 >> 28u ) + 4u ) ) ^ nodeVar246 ) * 277803737u );
				nodeVar241 = mix( vec3<f32>( 0.85499260812105, 0.8879231178794776, 0.9215818562755338 ), vec3<f32>( 0.7379104087672317, 0.8069522576650873, 1.0 ), ( f32( ( ( nodeVar247 >> 22u ) ^ nodeVar247 ) ) * 2.3283064365386963e-10 ) );

			}


			if ( nodeVar116 ) {

				nodeVar248 = 0.0;

			} else {


				if ( nodeVar115 ) {

					nodeVar249 = 1.0;

				} else {


					if ( nodeVar114 ) {

						nodeVar250 = 1.0;

					} else {


						if ( nodeVar113 ) {

							nodeVar251 = 1.0;

						} else {


							if ( nodeVar112 ) {

								nodeVar252 = 1.0;

							} else {


								if ( nodeVar111 ) {

									nodeVar253 = 1.0;

								} else {


									if ( nodeVar110 ) {

										nodeVar254 = 1.0;

									} else {

										nodeVar254 = 1.0;

									}

									nodeVar253 = nodeVar254;

								}

								nodeVar252 = nodeVar253;

							}

							nodeVar251 = nodeVar252;

						}

						nodeVar250 = nodeVar251;

					}

					nodeVar249 = nodeVar250;

				}

				nodeVar248 = nodeVar249;

			}

			nodeVar4 = vec4<f32>( ( ( nodeVar7 * mix( vec3<f32>( 1.0, 1.0, 1.0 ), nodeVar241, ( nodeVar23 * 0.6 ) ) ) * vec3<f32>( mix( 0.45, 1.5, nodeVar23 ) ) ), ( nodeVar23 * nodeVar248 ) );

		} else {

			nodeVar255 = floor( ( nodeVarying5 * vec3<f32>( 2.0 ) ) );
			nodeVar256 = ( ( ( u32( ( nodeVar255.x + 2097152.0 ) ) * 73856093u ) ^ ( u32( ( nodeVar255.y + 2097152.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeVar255.z + 2097152.0 ) ) * 83492791u ) );
			nodeVar258 = vec3<f32>( ( nodeVarying12.x * 0.5 ), ( nodeVarying12.y * 0.5 ), ( 0.1 + ( nodeVarying12.y * 1.55 ) ) );
			nodeVar259 = ( ( ( nodeVar256 + 119831u ) * 747796405u ) + 2891336453u );
			nodeVar260 = ( ( ( nodeVar259 >> ( ( nodeVar259 >> 28u ) + 4u ) ) ^ nodeVar259 ) * 277803737u );
			nodeVar261 = ( f32( ( ( nodeVar260 >> 22u ) ^ nodeVar260 ) ) * 2.3283064365386963e-10 );
			nodeVar262 = smoothstep( 0.3, 1.0, nodeVar261 );
			nodeVar263 = ( nodeVar258.x * ( nodeVar262 * nodeVar262 ) );
			nodeVar264 = vec3<f32>( ( - nodeVar258.x ), ( - nodeVar258.y ), 0.1 );
			nodeVar265 = ( positionLocal - nodeVarying5 );
			normalLocal = nodeVarying13;
			nodeVar266 = normalize( cross( vec3<f32>( 0.0, 1.0, 0.0 ), normalLocal ) );
			nodeVar267 = vec3<f32>( dot( nodeVar265, nodeVar266 ), nodeVar265.y, 0.0 );
			nodeVar268 = normalize( ( positionLocal - ( object.nodeUniform0 * vec4<f32>( render.cameraPosition, 1.0 ) ).xyz ) );
			normalLocal = nodeVarying13;
			nodeVar269 = vec3<f32>( dot( nodeVar268, nodeVar266 ), nodeVar268.y, ( - dot( nodeVar268, normalLocal ) ) );
			nodeVar270 = ( ( vec3<f32>( ( nodeVar258.x - nodeVar263 ), nodeVar264.y, 0.1 ) - nodeVar267 ) / nodeVar269 );
			nodeVar271 = ( 0.1 + 0.12 );
			nodeVar272 = ( ( vec3<f32>( nodeVar258.x, nodeVar258.y, nodeVar271 ) - nodeVar267 ) / nodeVar269 );
			nodeVar273 = max( nodeVar270, nodeVar272 );
			nodeVar274 = min( nodeVar270, nodeVar272 );
			nodeVar275 = max( max( nodeVar274.x, nodeVar274.y ), nodeVar274.z );
			nodeVar277 = ( ( vec3<f32>( ( - nodeVar258.x ), nodeVar264.y, 0.1 ) - nodeVar267 ) / nodeVar269 );
			nodeVar278 = ( ( ( nodeVar256 + 105097u ) * 747796405u ) + 2891336453u );
			nodeVar279 = ( ( ( nodeVar278 >> ( ( nodeVar278 >> 28u ) + 4u ) ) ^ nodeVar278 ) * 277803737u );
			nodeVar280 = ( f32( ( ( nodeVar279 >> 22u ) ^ nodeVar279 ) ) * 2.3283064365386963e-10 );
			nodeVar281 = smoothstep( 0.3, 1.0, nodeVar280 );
			nodeVar282 = ( nodeVar258.x * ( nodeVar281 * nodeVar281 ) );
			nodeVar283 = ( ( vec3<f32>( ( ( - nodeVar258.x ) + nodeVar282 ), nodeVar258.y, nodeVar271 ) - nodeVar267 ) / nodeVar269 );
			nodeVar284 = max( nodeVar277, nodeVar283 );
			nodeVar285 = min( nodeVar277, nodeVar283 );
			nodeVar286 = max( max( nodeVar285.x, nodeVar285.y ), nodeVar285.z );
			nodeVar288 = ( nodeVar258.x * 0.82 );
			nodeVar289 = vec3<f32>( ( nodeVar288 - 0.5 ), nodeVar264.y, ( nodeVar258.z - 0.7 ) );
			nodeVar290 = ( ( nodeVar289 - nodeVar267 ) / nodeVar269 );
			nodeVar291 = vec3<f32>( ( nodeVar288 + 0.5 ), ( nodeVar264.y + mix( 1.7, 2.3, nodeVar261 ) ), ( nodeVar258.z - 0.1 ) );
			nodeVar292 = ( ( nodeVar291 - nodeVar267 ) / nodeVar269 );
			nodeVar293 = max( nodeVar290, nodeVar292 );
			nodeVar294 = min( nodeVar290, nodeVar292 );
			nodeVar295 = max( max( nodeVar294.x, nodeVar294.y ), nodeVar294.z );
			nodeVar296 = ( ( ( nodeVar256 + 8200u ) * 747796405u ) + 2891336453u );
			nodeVar297 = ( ( ( nodeVar296 >> ( ( nodeVar296 >> 28u ) + 4u ) ) ^ nodeVar296 ) * 277803737u );
			nodeVar299 = ( nodeVar258.x * -0.82 );
			nodeVar300 = vec3<f32>( ( nodeVar299 - 0.5 ), nodeVar264.y, ( nodeVar258.z - 0.7 ) );
			nodeVar301 = ( ( nodeVar300 - nodeVar267 ) / nodeVar269 );
			nodeVar302 = vec3<f32>( ( nodeVar299 + 0.5 ), ( nodeVar264.y + mix( 1.7, 2.3, nodeVar280 ) ), ( nodeVar258.z - 0.1 ) );
			nodeVar303 = ( ( nodeVar302 - nodeVar267 ) / nodeVar269 );
			nodeVar304 = max( nodeVar301, nodeVar303 );
			nodeVar305 = min( nodeVar301, nodeVar303 );
			nodeVar306 = max( max( nodeVar305.x, nodeVar305.y ), nodeVar305.z );
			nodeVar307 = ( ( ( nodeVar256 + 15070u ) * 747796405u ) + 2891336453u );
			nodeVar308 = ( ( ( nodeVar307 >> ( ( nodeVar307 >> 28u ) + 4u ) ) ^ nodeVar307 ) * 277803737u );
			nodeVar310 = mix( ( nodeVar258.x * -0.3 ), ( nodeVar258.x * 0.3 ), nodeVar261 );
			nodeVar311 = vec3<f32>( ( nodeVar310 - 1.1 ), nodeVar264.y, ( nodeVar258.z - 0.95 ) );
			nodeVar312 = ( ( nodeVar311 - nodeVar267 ) / nodeVar269 );
			nodeVar313 = vec3<f32>( ( nodeVar310 + 1.1 ), ( nodeVar264.y + mix( 0.8, 0.9, nodeVar280 ) ), ( nodeVar258.z - 0.1 ) );
			nodeVar314 = ( ( nodeVar313 - nodeVar267 ) / nodeVar269 );
			nodeVar315 = max( nodeVar312, nodeVar314 );
			nodeVar316 = min( nodeVar312, nodeVar314 );
			nodeVar317 = max( max( nodeVar316.x, nodeVar316.y ), nodeVar316.z );
			nodeVar319 = mix( -0.6, 0.6, nodeVar280 );
			nodeVar320 = ( nodeVarying12.y * 1.55 );
			nodeVar321 = ( ( 0.1 + ( nodeVar320 * 0.5 ) ) + mix( -0.4, 0.5, nodeVar261 ) );
			nodeVar322 = vec3<f32>( ( nodeVar319 - 0.6 ), nodeVar264.y, ( nodeVar321 - 0.35 ) );
			nodeVar323 = ( ( nodeVar322 - nodeVar267 ) / nodeVar269 );
			nodeVar324 = vec3<f32>( ( nodeVar319 + 0.6 ), ( nodeVar264.y + 0.42 ), ( nodeVar321 + 0.35 ) );
			nodeVar325 = ( ( nodeVar324 - nodeVar267 ) / nodeVar269 );
			nodeVar326 = max( nodeVar323, nodeVar325 );
			nodeVar327 = min( nodeVar323, nodeVar325 );
			nodeVar328 = max( max( nodeVar327.x, nodeVar327.y ), nodeVar327.z );
			nodeVar329 = max( ( ( nodeVar264 - nodeVar267 ) / nodeVar269 ), ( ( nodeVar258 - nodeVar267 ) / nodeVar269 ) );
			nodeVar330 = min( min( nodeVar329.x, nodeVar329.y ), nodeVar329.z );
			nodeVar331 = ( ( ( min( min( nodeVar326.x, nodeVar326.y ), nodeVar326.z ) > nodeVar328 ) && ( nodeVar328 > 0.0 ) ) && ( nodeVar328 < nodeVar330 ) );

			if ( nodeVar331 ) {

				nodeVar318 = nodeVar328;

			} else {

				nodeVar318 = nodeVar330;

			}

			nodeVar332 = ( ( ( min( min( nodeVar315.x, nodeVar315.y ), nodeVar315.z ) > nodeVar317 ) && ( nodeVar317 > 0.0 ) ) && ( nodeVar317 < nodeVar318 ) );

			if ( nodeVar332 ) {

				nodeVar309 = nodeVar317;

			} else {

				nodeVar309 = nodeVar318;

			}

			nodeVar333 = ( ( ( ( min( min( nodeVar304.x, nodeVar304.y ), nodeVar304.z ) > nodeVar306 ) && ( nodeVar306 > 0.0 ) ) && ( ( f32( ( ( nodeVar308 >> 22u ) ^ nodeVar308 ) ) * 2.3283064365386963e-10 ) > 0.4 ) ) && ( nodeVar306 < nodeVar309 ) );

			if ( nodeVar333 ) {

				nodeVar298 = nodeVar306;

			} else {

				nodeVar298 = nodeVar309;

			}

			nodeVar334 = ( ( ( ( min( min( nodeVar293.x, nodeVar293.y ), nodeVar293.z ) > nodeVar295 ) && ( nodeVar295 > 0.0 ) ) && ( ( f32( ( ( nodeVar297 >> 22u ) ^ nodeVar297 ) ) * 2.3283064365386963e-10 ) > 0.4 ) ) && ( nodeVar295 < nodeVar298 ) );

			if ( nodeVar334 ) {

				nodeVar287 = nodeVar295;

			} else {

				nodeVar287 = nodeVar298;

			}

			nodeVar335 = ( ( ( ( min( min( nodeVar284.x, nodeVar284.y ), nodeVar284.z ) > nodeVar286 ) && ( nodeVar286 > 0.0 ) ) && ( nodeVar282 > 0.05 ) ) && ( nodeVar286 < nodeVar287 ) );

			if ( nodeVar335 ) {

				nodeVar276 = nodeVar286;

			} else {

				nodeVar276 = nodeVar287;

			}

			nodeVar336 = ( ( ( ( min( min( nodeVar273.x, nodeVar273.y ), nodeVar273.z ) > nodeVar275 ) && ( nodeVar275 > 0.0 ) ) && ( nodeVar263 > 0.05 ) ) && ( nodeVar275 < nodeVar276 ) );

			if ( nodeVar336 ) {

				nodeVar338 = ( ( ( nodeVar256 + 125490u ) * 747796405u ) + 2891336453u );
				nodeVar339 = ( ( ( nodeVar338 >> ( ( nodeVar338 >> 28u ) + 4u ) ) ^ nodeVar338 ) * 277803737u );
				nodeVar340 = ( ( f32( ( ( nodeVar339 >> 22u ) ^ nodeVar339 ) ) * 2.3283064365386963e-10 ) * 6.0 );

				if ( ( nodeVar340 > 5.0 ) ) {

					nodeVar337 = mix( vec3<f32>( 0.26225065751888765, 0.10224173307914941, 0.057805430183792694 ), vec3<f32>( 0.3231432091022285, 0.14412847084818123, 0.08437621153575764 ), nodeVar261 );

				} else {


					if ( ( nodeVar340 > 4.0 ) ) {

						nodeVar341 = mix( vec3<f32>( 0.1499597898006365, 0.17788841597328695, 0.09758734713304495 ), vec3<f32>( 0.1912016827303171, 0.22696587349938613, 0.11443537381770343 ), nodeVar261 );

					} else {


						if ( ( nodeVar340 > 3.0 ) ) {

							nodeVar342 = mix( vec3<f32>( 0.11443537381770343, 0.16202937562896222, 0.1912016827303171 ), vec3<f32>( 0.158960835050774, 0.21952619971859377, 0.25818285291079235 ), nodeVar261 );

						} else {


							if ( ( nodeVar340 > 2.0 ) ) {

								nodeVar343 = mix( vec3<f32>( 0.16202937562896222, 0.14412847084818123, 0.12743768042608497 ), vec3<f32>( 0.22696587349938613, 0.20507873637973145, 0.181164244239483 ), nodeVar261 );

							} else {


								if ( ( nodeVar340 > 1.0 ) ) {

									nodeVar344 = mix( vec3<f32>( 0.2541520943200296, 0.19461783043107173, 0.12743768042608497 ), vec3<f32>( 0.3277780980458375, 0.26225065751888765, 0.16826940017946088 ), nodeVar261 );

								} else {

									nodeVar344 = mix( vec3<f32>( 0.5906188409113381, 0.5209955731953768, 0.3813260114221238 ), vec3<f32>( 0.6866853124288864, 0.6104955708001716, 0.4793201830913402 ), nodeVar261 );

								}

								nodeVar343 = nodeVar344;

							}

							nodeVar342 = nodeVar343;

						}

						nodeVar341 = nodeVar342;

					}

					nodeVar337 = nodeVar341;

				}

				nodeVar345 = ( nodeVar267 + ( nodeVar269 * vec3<f32>( nodeVar275 ) ) );
				nodeVar257 = ( ( nodeVar337 * vec3<f32>( mix( 0.78, 1.12, fract( ( nodeVar345.x * 2.5 ) ) ) ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeVar345.z - 0.1 ) / nodeVar320 ), 0.0, 1.0 ) ) ) );

			} else {


				if ( nodeVar335 ) {

					nodeVar348 = ( ( ( nodeVar256 + 125490u ) * 747796405u ) + 2891336453u );
					nodeVar349 = ( ( ( nodeVar348 >> ( ( nodeVar348 >> 28u ) + 4u ) ) ^ nodeVar348 ) * 277803737u );
					nodeVar350 = ( ( f32( ( ( nodeVar349 >> 22u ) ^ nodeVar349 ) ) * 2.3283064365386963e-10 ) * 6.0 );

					if ( ( nodeVar350 > 5.0 ) ) {

						nodeVar347 = mix( vec3<f32>( 0.26225065751888765, 0.10224173307914941, 0.057805430183792694 ), vec3<f32>( 0.3231432091022285, 0.14412847084818123, 0.08437621153575764 ), nodeVar261 );

					} else {


						if ( ( nodeVar350 > 4.0 ) ) {

							nodeVar351 = mix( vec3<f32>( 0.1499597898006365, 0.17788841597328695, 0.09758734713304495 ), vec3<f32>( 0.1912016827303171, 0.22696587349938613, 0.11443537381770343 ), nodeVar261 );

						} else {


							if ( ( nodeVar350 > 3.0 ) ) {

								nodeVar352 = mix( vec3<f32>( 0.11443537381770343, 0.16202937562896222, 0.1912016827303171 ), vec3<f32>( 0.158960835050774, 0.21952619971859377, 0.25818285291079235 ), nodeVar261 );

							} else {


								if ( ( nodeVar350 > 2.0 ) ) {

									nodeVar353 = mix( vec3<f32>( 0.16202937562896222, 0.14412847084818123, 0.12743768042608497 ), vec3<f32>( 0.22696587349938613, 0.20507873637973145, 0.181164244239483 ), nodeVar261 );

								} else {


									if ( ( nodeVar350 > 1.0 ) ) {

										nodeVar354 = mix( vec3<f32>( 0.2541520943200296, 0.19461783043107173, 0.12743768042608497 ), vec3<f32>( 0.3277780980458375, 0.26225065751888765, 0.16826940017946088 ), nodeVar261 );

									} else {

										nodeVar354 = mix( vec3<f32>( 0.5906188409113381, 0.5209955731953768, 0.3813260114221238 ), vec3<f32>( 0.6866853124288864, 0.6104955708001716, 0.4793201830913402 ), nodeVar261 );

									}

									nodeVar353 = nodeVar354;

								}

								nodeVar352 = nodeVar353;

							}

							nodeVar351 = nodeVar352;

						}

						nodeVar347 = nodeVar351;

					}

					nodeVar355 = ( nodeVar267 + ( nodeVar269 * vec3<f32>( nodeVar286 ) ) );
					nodeVar346 = ( ( nodeVar347 * vec3<f32>( mix( 0.78, 1.12, fract( ( nodeVar355.x * 2.5 ) ) ) ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeVar355.z - 0.1 ) / nodeVar320 ), 0.0, 1.0 ) ) ) );

				} else {


					if ( nodeVar334 ) {

						nodeVar358 = ( nodeVar267 + ( nodeVar269 * vec3<f32>( nodeVar295 ) ) );

						if ( ( ( ( nodeVar358 - nodeVar289 ) / ( nodeVar291 - nodeVar289 ) ).y > 0.94 ) ) {

							nodeVar357 = 1.2;

						} else {

							nodeVar357 = 0.82;

						}

						nodeVar356 = ( ( mix( vec3<f32>( 0.04231141061442144, 0.025186859622305935, 0.015996293361446288 ), vec3<f32>( 0.09084171117479915, 0.06301001764564068, 0.04231141061442144 ), nodeVar280 ) * vec3<f32>( nodeVar357 ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeVar358.z - 0.1 ) / nodeVar320 ), 0.0, 1.0 ) ) ) );

					} else {


						if ( nodeVar333 ) {

							nodeVar361 = ( nodeVar267 + ( nodeVar269 * vec3<f32>( nodeVar306 ) ) );

							if ( ( ( ( nodeVar361 - nodeVar300 ) / ( nodeVar302 - nodeVar300 ) ).y > 0.94 ) ) {

								nodeVar360 = 1.2;

							} else {

								nodeVar360 = 0.82;

							}

							nodeVar359 = ( ( mix( vec3<f32>( 0.04231141061442144, 0.025186859622305935, 0.015996293361446288 ), vec3<f32>( 0.09084171117479915, 0.06301001764564068, 0.04231141061442144 ), nodeVar280 ) * vec3<f32>( nodeVar360 ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeVar361.z - 0.1 ) / nodeVar320 ), 0.0, 1.0 ) ) ) );

						} else {


							if ( nodeVar332 ) {

								nodeVar364 = ( nodeVar267 + ( nodeVar269 * vec3<f32>( nodeVar317 ) ) );

								if ( ( ( ( nodeVar364 - nodeVar311 ) / ( nodeVar313 - nodeVar311 ) ).y > 0.9 ) ) {

									nodeVar363 = 1.12;

								} else {

									nodeVar363 = 0.85;

								}

								nodeVar362 = ( ( mix( vec3<f32>( 0.10224173307914941, 0.06847816983662762, 0.04231141061442144 ), vec3<f32>( 0.054480276435339814, 0.09305896283800832, 0.14412847084818123 ), nodeVar280 ) * vec3<f32>( nodeVar363 ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeVar364.z - 0.1 ) / nodeVar320 ), 0.0, 1.0 ) ) ) );

							} else {


								if ( nodeVar331 ) {

									nodeVar367 = ( nodeVar267 + ( nodeVar269 * vec3<f32>( nodeVar328 ) ) );

									if ( ( ( ( nodeVar367 - nodeVar322 ) / ( nodeVar324 - nodeVar322 ) ).y > 0.94 ) ) {

										nodeVar366 = 1.25;

									} else {

										nodeVar366 = 0.8;

									}

									nodeVar365 = ( ( mix( vec3<f32>( 0.06847816983662762, 0.035601314869097636, 0.019382360952473074 ), vec3<f32>( 0.14702726648767014, 0.06847816983662762, 0.02955683443236377 ), nodeVar261 ) * vec3<f32>( nodeVar366 ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeVar367.z - 0.1 ) / nodeVar320 ), 0.0, 1.0 ) ) ) );

								} else {

									nodeVar369 = ( nodeVar267 + ( nodeVar269 * vec3<f32>( nodeVar330 ) ) );
									nodeVar370 = ( ( nodeVar369 - nodeVar264 ) / ( nodeVar258 - nodeVar264 ) );
									nodeVar371 = ( nodeVar370.z > 0.998 );

									if ( nodeVar371 ) {

										nodeVar372 = mix( mix( vec3<f32>( 0.3231432091022285, 0.25818285291079235, 0.171441100722554 ), vec3<f32>( 0.158960835050774, 0.19461783043107173, 0.22322795730611386 ), nodeVar280 ), vec3<f32>( 0.48514994004665124, 0.4178850708380236, 0.3094689228067428 ), ( nodeVar261 * 0.6 ) );
										nodeVar373 = mix( 0.22, 0.78, nodeVar280 );

										if ( ( nodeVar373 < 0.5 ) ) {

											nodeVar374 = mix( 0.68, 0.82, nodeVar261 );

										} else {

											nodeVar374 = mix( 0.18, 0.32, nodeVar261 );

										}

										nodeVar375 = ( ( ( nodeVar256 + 11240u ) * 747796405u ) + 2891336453u );
										nodeVar376 = ( ( ( nodeVar375 >> ( ( nodeVar375 >> 28u ) + 4u ) ) ^ nodeVar375 ) * 277803737u );
										nodeVar368 = mix( mix( mix( mix( nodeVar372, ( nodeVar372 * vec3<f32>( 0.5 ) ), smoothstep( 0.05, 0.04, nodeVar370.y ) ), mix( vec3<f32>( 0.10224173307914941, 0.061246054224174035, 0.030713443727452196 ), vec3<f32>( 0.040915196900556984, 0.03954623527052923, 0.04518620437910499 ), step( 0.5, nodeVar261 ) ), ( smoothstep( 0.09100000000000001, 0.079, abs( ( nodeVar370.x - nodeVar373 ) ) ) * smoothstep( 0.356, 0.344, abs( ( nodeVar370.y - 0.33 ) ) ) ) ), vec3<f32>( 0.0069954101845983935, 0.006048833020386069, 0.005181516700061659 ), ( smoothstep( 0.081, 0.06899999999999999, abs( ( nodeVar370.x - nodeVar374 ) ) ) * smoothstep( 0.09100000000000001, 0.079, abs( ( nodeVar370.y - 0.56 ) ) ) ) ), mix( vec3<f32>( 0.025186859622305935, 0.04231141061442144, 0.06847816983662762 ), vec3<f32>( 0.19461783043107173, 0.10224173307914941, 0.04231141061442144 ), ( f32( ( ( nodeVar376 >> 22u ) ^ nodeVar376 ) ) * 2.3283064365386963e-10 ) ), ( smoothstep( 0.061, 0.049, abs( ( nodeVar370.x - nodeVar374 ) ) ) * smoothstep( 0.07100000000000001, 0.059000000000000004, abs( ( nodeVar370.y - 0.56 ) ) ) ) );

									} else {


										if ( ( nodeVar370.y > 0.998 ) ) {

											nodeVar379 = ( ( ( nodeVar256 + 25910u ) * 747796405u ) + 2891336453u );
											nodeVar380 = ( ( ( nodeVar379 >> ( ( nodeVar379 >> 28u ) + 4u ) ) ^ nodeVar379 ) * 277803737u );

											if ( ( ( f32( ( ( nodeVar380 >> 22u ) ^ nodeVar380 ) ) * 2.3283064365386963e-10 ) > 0.88 ) ) {

												nodeVar381 = ( ( ( nodeVar256 + 59550u ) * 747796405u ) + 2891336453u );
												nodeVar382 = ( ( ( nodeVar381 >> ( ( nodeVar381 >> 28u ) + 4u ) ) ^ nodeVar381 ) * 277803737u );
												nodeVar378 = mix( vec3<f32>( 0.7379104087672317, 0.8069522576650873, 1.0 ), vec3<f32>( 0.34670405634441115, 0.46778379610254284, 1.0 ), ( f32( ( ( nodeVar382 >> 22u ) ^ nodeVar382 ) ) * 2.3283064365386963e-10 ) );

											} else {

												nodeVar383 = ( ( ( nodeVar256 + 86350u ) * 747796405u ) + 2891336453u );
												nodeVar384 = ( ( ( nodeVar383 >> ( ( nodeVar383 >> 28u ) + 4u ) ) ^ nodeVar383 ) * 277803737u );
												nodeVar378 = mix( vec3<f32>( 1.0, 0.4793201830913402, 0.059511238155621766 ), vec3<f32>( 1.0, 0.775822218312646, 0.33245153633549385 ), ( f32( ( ( nodeVar384 >> 22u ) ^ nodeVar384 ) ) * 2.3283064365386963e-10 ) );

											}

											nodeVar385 = ( ( ( nodeVar256 + 79599u ) * 747796405u ) + 2891336453u );
											nodeVar386 = ( ( ( nodeVar385 >> ( ( nodeVar385 >> 28u ) + 4u ) ) ^ nodeVar385 ) * 277803737u );
											nodeVar377 = mix( mix( mix( mix( vec3<f32>( 0.3231432091022285, 0.25818285291079235, 0.171441100722554 ), vec3<f32>( 0.158960835050774, 0.19461783043107173, 0.22322795730611386 ), nodeVar280 ), vec3<f32>( 0.48514994004665124, 0.4178850708380236, 0.3094689228067428 ), ( nodeVar261 * 0.6 ) ), vec3<f32>( 1.0, 1.0, 1.0 ), 0.5 ), ( nodeVar378 * vec3<f32>( mix( 1.0, 4.5, step( 0.8, ( f32( ( ( nodeVar386 >> 22u ) ^ nodeVar386 ) ) * 2.3283064365386963e-10 ) ) ) ) ), smoothstep( 0.16, 0.13, length( vec2<f32>( ( nodeVar370.x - 0.5 ), ( nodeVar370.z - 0.5 ) ) ) ) );

										} else {


											if ( ( nodeVar370.y < 0.002 ) ) {

												nodeVar387 = mix( ( mix( vec3<f32>( 0.06847816983662762, 0.033104766565152086, 0.014443843592229466 ), vec3<f32>( 0.14412847084818123, 0.0722718506743852, 0.02955683443236377 ), nodeVar280 ) * vec3<f32>( ( 1.0 - ( step( 0.94, fract( ( nodeVar370.x * 6.0 ) ) ) * 0.3 ) ) ) ), mix( vec3<f32>( 0.19461783043107173, 0.04373502925049377, 0.031896033067374104 ), vec3<f32>( 0.04231141061442144, 0.09530746662221588, 0.11697066774917994 ), nodeVar261 ), ( ( smoothstep( 0.306, 0.294, abs( ( nodeVar370.x - 0.5 ) ) ) * smoothstep( 0.266, 0.254, abs( ( nodeVar370.z - 0.62 ) ) ) ) * 0.9 ) );

											} else {

												nodeVar388 = mix( mix( vec3<f32>( 0.3231432091022285, 0.25818285291079235, 0.171441100722554 ), vec3<f32>( 0.158960835050774, 0.19461783043107173, 0.22322795730611386 ), nodeVar280 ), vec3<f32>( 0.48514994004665124, 0.4178850708380236, 0.3094689228067428 ), ( nodeVar261 * 0.6 ) );
												nodeVar387 = mix( nodeVar388, ( nodeVar388 * vec3<f32>( 0.5 ) ), smoothstep( 0.05, 0.04, nodeVar370.y ) );

											}

											nodeVar377 = nodeVar387;

										}

										nodeVar368 = nodeVar377;

									}


									if ( nodeVar371 ) {

										nodeVar389 = ( ( smoothstep( 0.0, 0.15, nodeVar370.x ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar370.x ) ) ) * ( smoothstep( 0.0, 0.15, nodeVar370.y ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar370.y ) ) ) );

									} else {


										if ( ( ( nodeVar370.y < 0.002 ) || ( nodeVar370.y > 0.998 ) ) ) {

											nodeVar390 = ( ( smoothstep( 0.0, 0.15, nodeVar370.x ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar370.x ) ) ) * ( smoothstep( 0.0, 0.15, nodeVar370.z ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar370.z ) ) ) );

										} else {

											nodeVar390 = ( ( smoothstep( 0.0, 0.15, nodeVar370.y ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar370.y ) ) ) * ( smoothstep( 0.0, 0.15, nodeVar370.z ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar370.z ) ) ) );

										}

										nodeVar389 = nodeVar390;

									}

									nodeVar365 = ( ( nodeVar368 * vec3<f32>( mix( 0.72, 1.0, nodeVar389 ) ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeVar369.z - 0.1 ) / nodeVar320 ), 0.0, 1.0 ) ) ) );

								}

								nodeVar362 = nodeVar365;

							}

							nodeVar359 = nodeVar362;

						}

						nodeVar356 = nodeVar359;

					}

					nodeVar346 = nodeVar356;

				}

				nodeVar257 = nodeVar346;

			}

			nodeVar392 = ( ( ( nodeVar256 + 25910u ) * 747796405u ) + 2891336453u );
			nodeVar393 = ( ( ( nodeVar392 >> ( ( nodeVar392 >> 28u ) + 4u ) ) ^ nodeVar392 ) * 277803737u );

			if ( ( ( f32( ( ( nodeVar393 >> 22u ) ^ nodeVar393 ) ) * 2.3283064365386963e-10 ) > 0.88 ) ) {

				nodeVar394 = ( ( ( nodeVar256 + 59550u ) * 747796405u ) + 2891336453u );
				nodeVar395 = ( ( ( nodeVar394 >> ( ( nodeVar394 >> 28u ) + 4u ) ) ^ nodeVar394 ) * 277803737u );
				nodeVar391 = mix( vec3<f32>( 0.7379104087672317, 0.8069522576650873, 1.0 ), vec3<f32>( 0.34670405634441115, 0.46778379610254284, 1.0 ), ( f32( ( ( nodeVar395 >> 22u ) ^ nodeVar395 ) ) * 2.3283064365386963e-10 ) );

			} else {

				nodeVar396 = ( ( ( nodeVar256 + 86350u ) * 747796405u ) + 2891336453u );
				nodeVar397 = ( ( ( nodeVar396 >> ( ( nodeVar396 >> 28u ) + 4u ) ) ^ nodeVar396 ) * 277803737u );
				nodeVar391 = mix( vec3<f32>( 1.0, 0.4793201830913402, 0.059511238155621766 ), vec3<f32>( 1.0, 0.775822218312646, 0.33245153633549385 ), ( f32( ( ( nodeVar397 >> 22u ) ^ nodeVar397 ) ) * 2.3283064365386963e-10 ) );

			}

			nodeVar398 = ( ( ( nodeVar256 + 79599u ) * 747796405u ) + 2891336453u );
			nodeVar399 = ( ( ( nodeVar398 >> ( ( nodeVar398 >> 28u ) + 4u ) ) ^ nodeVar398 ) * 277803737u );
			nodeVar400 = step( 0.8, ( f32( ( ( nodeVar399 >> 22u ) ^ nodeVar399 ) ) * 2.3283064365386963e-10 ) );

			if ( nodeVar336 ) {

				nodeVar401 = 0.2;

			} else {


				if ( nodeVar335 ) {

					nodeVar402 = 0.2;

				} else {


					if ( nodeVar334 ) {

						nodeVar403 = 1.0;

					} else {


						if ( nodeVar333 ) {

							nodeVar404 = 1.0;

						} else {


							if ( nodeVar332 ) {

								nodeVar405 = 1.0;

							} else {


								if ( nodeVar331 ) {

									nodeVar406 = 1.0;

								} else {

									nodeVar406 = 1.0;

								}

								nodeVar405 = nodeVar406;

							}

							nodeVar404 = nodeVar405;

						}

						nodeVar403 = nodeVar404;

					}

					nodeVar402 = nodeVar403;

				}

				nodeVar401 = nodeVar402;

			}

			nodeVar4 = vec4<f32>( ( ( nodeVar257 * mix( vec3<f32>( 1.0, 1.0, 1.0 ), nodeVar391, ( nodeVar400 * 0.85 ) ) ) * vec3<f32>( mix( 1.0, 1.3, nodeVar400 ) ) ), ( nodeVar400 * nodeVar401 ) );

		}

		nodeVar407 = nodeVar4;

		if ( nodeVar2 ) {

			nodeVar408 = vec3<f32>( 0.6038273388475408, 0.6583748172725346, 0.623960391667596 );

		} else {

			nodeVar408 = vec3<f32>( 0.46778379610254284, 0.5647115056965487, 0.5209955731953768 );

		}


		if ( nodeVar2 ) {

			nodeVar409 = vec3<f32>( 0.008023192982520563, 0.010329823026364548, 0.012983032338510335 );

		} else {

			nodeVar410 = ( v_positionWorld * vec3<f32>( 0.3 ) );
			nodeVar409 = mix( vec3<f32>( 0.006512090790025684, 0.008023192982520563, 0.010329823026364548 ), vec3<f32>( 0.01680737574872402, 0.024157632443547246, 0.030713443727452196 ), ( ( ( valueNoise( nodeVar410 ) + ( valueNoise( ( nodeVar410 * vec3<f32>( 2.0 ) ) ) * 0.5 ) ) * 0.5 ) + 0.5 ) );

		}


		if ( nodeVar2 ) {

			nodeVar411 = ( 0.14 + ( ( smoothstep( -0.15, 0.5, ( mx_fractal_noise_float( vec3<f32>( ( v_positionWorld.x * 1.3 ), ( v_positionWorld.y * 0.06 ), ( v_positionWorld.z * 1.3 ) ), 2, 2.0, 0.5 ) * 1.0 ) ) * 0.45 ) * 0.22 ) );

		} else {

			nodeVar411 = clamp( ( ( 0.64 + ( smoothstep( -0.15, 0.5, ( mx_fractal_noise_float( vec3<f32>( ( v_positionWorld.x * 1.3 ), ( v_positionWorld.y * 0.06 ), ( v_positionWorld.z * 1.3 ) ), 2, 2.0, 0.5 ) * 1.0 ) ) * 0.45 ) ) + ( smoothstep( 0.32, 0.0, nodeVarying14.y ) * 0.4 ) ), 0.0, 0.95 );

		}

		nodeVar0 = mix( ( nodeVar407.xyz * nodeVar408 ), nodeVar409, nodeVar411 );

	} else {


		if ( ( nodeVarying3 == 7.0 ) ) {

			nodeVar412 = mix( ( object.nodeUniform3 * vec3<f32>( 0.3 ) ), vec3<f32>( 0.01764195448412081, 0.014443843592229466, 0.011612245176281512 ), 0.5 );

		} else {


			if ( ( nodeVarying3 == 8.0 ) ) {

				nodeVar415 = floor( ( vec2<f32>( v_positionWorld.x, v_positionWorld.z ) * vec2<f32>( 0.2 ) ) );
				nodeVar416 = ( ( ( ( u32( ( nodeVar415.x + 65536.0 ) ) * 73856093u ) ^ ( u32( ( nodeVar415.y + 65536.0 ) ) * 19349663u ) ) * 747796405u ) + 2891336453u );
				nodeVar417 = ( ( ( nodeVar416 >> ( ( nodeVar416 >> 28u ) + 4u ) ) ^ nodeVar416 ) * 277803737u );
				nodeVar418 = ( ( f32( ( ( nodeVar417 >> 22u ) ^ nodeVar417 ) ) * 2.3283064365386963e-10 ) * 5.0 );

				if ( ( nodeVar418 > 4.0 ) ) {

					nodeVar414 = vec3<f32>( 0.014443843592229466, 0.013702083043526807, 0.015996293361446288 );

				} else {


					if ( ( nodeVar418 > 3.0 ) ) {

						nodeVar419 = vec3<f32>( 0.10224173307914941, 0.11193242782769693, 0.09758734713304495 );

					} else {


						if ( ( nodeVar418 > 2.0 ) ) {

							nodeVar420 = vec3<f32>( 0.015996293361446288, 0.03954623527052923, 0.07818742179702069 );

						} else {


							if ( ( nodeVar418 > 1.0 ) ) {

								nodeVar421 = vec3<f32>( 0.01764195448412081, 0.06847816983662762, 0.031896033067374104 );

							} else {

								nodeVar421 = vec3<f32>( 0.14412847084818123, 0.028426039499072558, 0.028426039499072558 );

							}

							nodeVar420 = nodeVar421;

						}

						nodeVar419 = nodeVar420;

					}

					nodeVar414 = nodeVar419;

				}

				nodeVar413 = nodeVar414;

			} else {


				if ( ( nodeVarying3 == 2.0 ) ) {

					nodeVar422 = ( object.nodeUniform3 * vec3<f32>( 0.55 ) );

				} else {

					nodeVar424 = ( nodeVarying3 == 3.0 );

					if ( nodeVar424 ) {

						nodeVar423 = ( mix( object.nodeUniform3, vec3<f32>( 1.0, 1.0, 1.0 ), 0.22 ) * vec3<f32>( ( 1.0 + ( nodeVarying7 * 0.18 ) ) ) );

					} else {


						if ( ( nodeVarying3 == 5.0 ) ) {

							nodeVar427 = ( ( valueNoise( ( v_positionWorld * vec3<f32>( 0.4 ) ) ) * 0.5 ) + 0.5 );
							normalWorldGeometry = normalize( v_normalWorldGeometry );
							nodeVar429 = vec3<f32>( ( v_positionWorld.x * 6.0 ), ( v_positionWorld.y * 0.5 ), ( v_positionWorld.z * 6.0 ) );
							nodeVar426 = mix( ( ( mix( vec3<f32>( 0.8879231178794776, 0.8796223968851662, 0.8387990117372213 ), vec3<f32>( 0.623960391667596, 0.6038273388475408, 0.5394794890033748 ), nodeVar427 ) + vec3<f32>( ( valueNoise( ( v_positionWorld * vec3<f32>( 5.0 ) ) ) * 0.04 ) ) ) * vec3<f32>( mix( 1.0, ( mix( 0.82, 1.04, fract( ( nodeVarying14.y * 6.0 ) ) ) * 0.42 ), ( ( ( ( smoothstep( 0.06, 0.14, nodeVarying14.x ) * smoothstep( 0.94, 0.86, nodeVarying14.x ) ) * smoothstep( 0.12, 0.2, nodeVarying14.y ) ) * smoothstep( 0.96, 0.88, nodeVarying14.y ) ) * ( smoothstep( 0.65, 0.4, abs( normalWorldGeometry.y ) ) * smoothstep( 0.08, 0.015, length( fwidth( v_positionWorld ) ) ) ) ) ) ) ), vec3<f32>( 0.158960835050774, 0.13843161502267545, 0.10224173307914941 ), ( ( ( smoothstep( 0.4, 0.0, nodeVarying14.y ) * ( ( ( ( valueNoise( nodeVar429 ) + ( valueNoise( ( nodeVar429 * vec3<f32>( 2.0 ) ) ) * 0.5 ) ) + ( valueNoise( ( nodeVar429 * vec3<f32>( 4.0 ) ) ) * 0.25 ) ) * 0.5 ) + 0.5 ) ) * ( nodeVar427 + 0.3 ) ) * 0.5 ) );

						} else {


							if ( ( nodeVarying3 == 1.0 ) ) {

								nodeVar430 = 0.12;

							} else {


								if ( nodeVar424 ) {

									nodeVar431 = 0.2;

								} else {

									nodeVar431 = 0.0;

								}

								nodeVar430 = nodeVar431;

							}

							nodeVar432 = ( positionLocal.y / 0.3 );
							nodeVar433 = floor( nodeVar432 );
							normalWorldGeometry = normalize( v_normalWorldGeometry );
							normalWorldGeometry = normalize( v_normalWorldGeometry );
							nodeVar434 = ( ( ( ( positionLocal.x * normalWorldGeometry.z ) - ( positionLocal.z * normalWorldGeometry.x ) ) / 0.6 ) + ( tsl_mod_float( nodeVar433, 2.0 ) * 0.5 ) );
							nodeVar435 = ( ( u32( ( nodeVar433 + 65536.0 ) ) * 73856093u ) ^ ( u32( ( floor( nodeVar434 ) + 65536.0 ) ) * 19349663u ) );
							nodeVar436 = ( ( nodeVar435 * 747796405u ) + 2891336453u );
							nodeVar437 = ( ( ( nodeVar436 >> ( ( nodeVar436 >> 28u ) + 4u ) ) ^ nodeVar436 ) * 277803737u );
							nodeVar438 = ( ( ( nodeVar435 + 1u ) * 747796405u ) + 2891336453u );
							nodeVar439 = ( ( ( nodeVar438 >> ( ( nodeVar438 >> 28u ) + 4u ) ) ^ nodeVar438 ) * 277803737u );
							nodeVar440 = ( ( ( f32( ( ( nodeVar439 >> 22u ) ^ nodeVar439 ) ) * 2.3283064365386963e-10 ) - 0.5 ) * 0.14 );
							nodeVar441 = ( ( mix( object.nodeUniform3, vec3<f32>( 1.0, 1.0, 1.0 ), nodeVar430 ) * vec3<f32>( ( ( ( 1.0 + ( nodeVarying7 * 0.18 ) ) + ( valueNoise( ( v_positionWorld * vec3<f32>( 0.7 ) ) ) * 0.06 ) ) + ( ( ( f32( ( ( nodeVar437 >> 22u ) ^ nodeVar437 ) ) * 2.3283064365386963e-10 ) - 0.5 ) * 0.14 ) ) ) ) * vec3<f32>( ( 1.0 + nodeVar440 ), 1.0, ( 1.0 - nodeVar440 ) ) );
							normalWorldGeometry = normalize( v_normalWorldGeometry );
							nodeVar442 = abs( normalWorldGeometry );
							nodeVar443 = clamp( ( ( ( nodeVar442.z * fwidth( v_positionWorld.x ) ) + ( nodeVar442.x * fwidth( v_positionWorld.z ) ) ) / 0.6 ), 0.000001, 0.5 );
							nodeVar444 = max( nodeVar443, 0.008333333333333333 );
							nodeVar445 = clamp( fwidth( nodeVar432 ), 0.000001, 0.5 );
							nodeVar446 = max( nodeVar445, 0.016666666666666666 );
							nodeVar447 = smoothstep( 0.7, 0.45, nodeVar442.y );
							nodeVar449 = ( 1.0 - nodeVar447 );

							if ( ( nodeVar449 > 0.0 ) ) {

								nodeVar450 = ( v_positionWorld * vec3<f32>( 0.025 ) );
								nodeVar448 = ( smoothstep( 0.0, 0.55, ( ( valueNoise( nodeVar450 ) + ( valueNoise( ( nodeVar450 * vec3<f32>( 2.0 ) ) ) * 0.5 ) ) + ( valueNoise( ( nodeVar450 * vec3<f32>( 4.0 ) ) ) * 0.25 ) ) ) * 0.22 );

							} else {

								nodeVar448 = 0.0;

							}

							nodeVar426 = mix( mix( nodeVar441, ( nodeVar441 * vec3<f32>( 0.6 ) ), ( max( ( smoothstep( ( nodeVar444 + nodeVar443 ), ( nodeVar444 - nodeVar443 ), ( 0.5 - abs( ( fract( nodeVar434 ) - 0.5 ) ) ) ) * min( ( 0.008333333333333333 / nodeVar444 ), 1.0 ) ), ( smoothstep( ( nodeVar446 + nodeVar445 ), ( nodeVar446 - nodeVar445 ), ( 0.5 - abs( ( fract( nodeVar432 ) - 0.5 ) ) ) ) * min( ( 0.016666666666666666 / nodeVar446 ), 1.0 ) ) ) * nodeVar447 ) ), vec3<f32>( 0.06847816983662762, 0.054480276435339814, 0.036889450395083165 ), mix( ( ( smoothstep( -0.1, 0.45, ( mx_fractal_noise_float( vec3<f32>( ( v_positionWorld.x * 1.5 ), ( v_positionWorld.y * 0.04 ), ( v_positionWorld.z * 1.5 ) ), 2, 2.0, 0.5 ) * 1.0 ) ) * smoothstep( 210.0, 0.0, v_positionWorld.y ) ) * 0.6 ), nodeVar448, nodeVar449 ) );

						}

						nodeVar423 = nodeVar426;

					}

					nodeVar422 = nodeVar423;

				}

				nodeVar413 = nodeVar422;

			}

			nodeVar412 = nodeVar413;

		}

		nodeVar0 = nodeVar412;

	}

	DiffuseColor = vec4<f32>( nodeVar0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform6 );
	DiffuseColor.w = 1.0;
	Metalness = 0.0;

	if ( nodeVar2 ) {

		nodeVar451 = 0.14;

	} else {


		if ( ( nodeVarying3 == 8.0 ) ) {

			nodeVar452 = 0.85;

		} else {


			if ( ( nodeVarying3 == 7.0 ) ) {

				nodeVar453 = 0.6;

			} else {


				if ( nodeVar1 ) {

					nodeVar454 = ( ( 0.16 + ( ( smoothstep( 0.32, 0.0, nodeVarying14.y ) * 0.4 ) * 0.45 ) ) + ( ( smoothstep( -0.15, 0.5, ( mx_fractal_noise_float( vec3<f32>( ( v_positionWorld.x * 1.3 ), ( v_positionWorld.y * 0.06 ), ( v_positionWorld.z * 1.3 ) ), 2, 2.0, 0.5 ) * 1.0 ) ) * 0.45 ) * 0.2 ) );

				} else {


					if ( ( nodeVarying3 == 3.0 ) ) {

						nodeVar455 = 0.8;

					} else {


						if ( ( nodeVarying3 == 5.0 ) ) {

							normalWorldGeometry = normalize( v_normalWorldGeometry );
							nodeVar456 = ( 0.52 + ( ( ( ( ( smoothstep( 0.06, 0.14, nodeVarying14.x ) * smoothstep( 0.94, 0.86, nodeVarying14.x ) ) * smoothstep( 0.12, 0.2, nodeVarying14.y ) ) * smoothstep( 0.96, 0.88, nodeVarying14.y ) ) * ( smoothstep( 0.65, 0.4, abs( normalWorldGeometry.y ) ) * smoothstep( 0.08, 0.015, length( fwidth( v_positionWorld ) ) ) ) ) * 0.08 ) );

						} else {

							normalWorldGeometry = normalize( v_normalWorldGeometry );
							nodeVar457 = abs( normalWorldGeometry );
							nodeVar458 = clamp( ( ( ( nodeVar457.z * fwidth( v_positionWorld.x ) ) + ( nodeVar457.x * fwidth( v_positionWorld.z ) ) ) / 0.6 ), 0.000001, 0.5 );
							nodeVar459 = max( nodeVar458, 0.008333333333333333 );
							normalWorldGeometry = normalize( v_normalWorldGeometry );
							normalWorldGeometry = normalize( v_normalWorldGeometry );
							nodeVar460 = ( positionLocal.y / 0.3 );
							nodeVar461 = clamp( fwidth( nodeVar460 ), 0.000001, 0.5 );
							nodeVar462 = max( nodeVar461, 0.016666666666666666 );
							nodeVar456 = ( ( ( valueNoise( ( v_positionWorld * vec3<f32>( 0.5 ) ) ) * 0.08 ) + 0.82 ) + ( ( max( ( smoothstep( ( nodeVar459 + nodeVar458 ), ( nodeVar459 - nodeVar458 ), ( 0.5 - abs( ( fract( ( ( ( ( positionLocal.x * normalWorldGeometry.z ) - ( positionLocal.z * normalWorldGeometry.x ) ) / 0.6 ) + ( tsl_mod_float( floor( nodeVar460 ), 2.0 ) * 0.5 ) ) ) - 0.5 ) ) ) ) * min( ( 0.008333333333333333 / nodeVar459 ), 1.0 ) ), ( smoothstep( ( nodeVar462 + nodeVar461 ), ( nodeVar462 - nodeVar461 ), ( 0.5 - abs( ( fract( nodeVar460 ) - 0.5 ) ) ) ) * min( ( 0.016666666666666666 / nodeVar462 ), 1.0 ) ) ) * smoothstep( 0.7, 0.45, nodeVar457.y ) ) * 0.12 ) );

						}

						nodeVar455 = nodeVar456;

					}

					nodeVar454 = nodeVar455;

				}

				nodeVar453 = nodeVar454;

			}

			nodeVar452 = nodeVar453;

		}

		nodeVar451 = nodeVar452;

	}

	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar463 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( nodeVar451, 0.0525 ) + max( max( nodeVar463.x, nodeVar463.y ), nodeVar463.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - 0.0 ) ) );

	if ( nodeVar3 ) {

		nodeVar407 = nodeVar4;
		nodeVar407 = nodeVar4;

		if ( nodeVar2 ) {

			nodeVar465 = 2.0;

		} else {

			nodeVar465 = ( 2.2 * ( 1.0 - ( clamp( ( ( 0.64 + ( smoothstep( -0.15, 0.5, ( mx_fractal_noise_float( vec3<f32>( ( v_positionWorld.x * 1.3 ), ( v_positionWorld.y * 0.06 ), ( v_positionWorld.z * 1.3 ) ), 2, 2.0, 0.5 ) * 1.0 ) ) * 0.45 ) ) + ( smoothstep( 0.32, 0.0, nodeVarying14.y ) * 0.4 ) ), 0.0, 0.95 ) * 0.6 ) ) );

		}

		nodeVar464 = ( ( nodeVar407.xyz * vec3<f32>( nodeVar407.w ) ) * vec3<f32>( nodeVar465 ) );

	} else {

		nodeVar464 = vec3<f32>( 0.0, 0.0, 0.0 );

	}

	EmissiveColor = nodeVar464;
	nodeVar466 = dpdx( v_positionView );
	nodeVar467 = - dpdy( v_positionView );
	NORMAL_normalView = normalViewGeometry;
	NORMAL_nodeVar468 = cross( nodeVar467, NORMAL_normalView );
	NORMAL_nodeVar469 = dot( nodeVar466, NORMAL_nodeVar468 );

	if ( ( ( ( ( ( nodeVar1 || ( nodeVarying3 == 2.0 ) ) || ( nodeVarying3 == 3.0 ) ) || nodeVar2 ) || ( nodeVarying3 == 7.0 ) ) || ( nodeVarying3 == 8.0 ) ) ) {

		nodeVar470 = 0.0;

	} else {


		if ( ( nodeVarying3 == 5.0 ) ) {

			normalWorldGeometry = normalize( v_normalWorldGeometry );
			nodeVar471 = ( ( ( ( ( smoothstep( 0.06, 0.14, nodeVarying14.x ) * smoothstep( 0.94, 0.86, nodeVarying14.x ) ) * smoothstep( 0.12, 0.2, nodeVarying14.y ) ) * smoothstep( 0.96, 0.88, nodeVarying14.y ) ) * ( smoothstep( 0.65, 0.4, abs( normalWorldGeometry.y ) ) * smoothstep( 0.08, 0.015, length( fwidth( v_positionWorld ) ) ) ) ) * ( ( fract( ( nodeVarying14.y * 6.0 ) ) * 0.012 ) - 0.01 ) );

		} else {

			nodeVar472 = max( ( length( fwidth( v_positionWorld ) ) * 1.5 ), 0.008 );
			normalWorldGeometry = normalize( v_normalWorldGeometry );
			normalWorldGeometry = normalize( v_normalWorldGeometry );
			nodeVar473 = ( positionLocal.y / 0.3 );
			normalWorldGeometry = normalize( v_normalWorldGeometry );
			nodeVar471 = ( ( ( smoothstep( 0.0, nodeVar472, ( ( 0.5 - abs( ( fract( ( ( ( ( positionLocal.x * normalWorldGeometry.z ) - ( positionLocal.z * normalWorldGeometry.x ) ) / 0.6 ) + ( tsl_mod_float( floor( nodeVar473 ), 2.0 ) * 0.5 ) ) ) - 0.5 ) ) ) * 0.6 ) ) * smoothstep( 0.0, nodeVar472, ( ( 0.5 - abs( ( fract( nodeVar473 ) - 0.5 ) ) ) * 0.3 ) ) ) * smoothstep( 0.7, 0.45, abs( normalWorldGeometry ).y ) ) * 0.003 );

		}

		nodeVar470 = nodeVar471;

	}

	nodeVar474 = ( vec3<f32>( sign( NORMAL_nodeVar469 ) ) * ( ( vec3<f32>( dpdx( nodeVar470 ) ) * NORMAL_nodeVar468 ) + ( vec3<f32>( - dpdy( nodeVar470 ) ) * cross( NORMAL_normalView, nodeVar466 ) ) ) );
	normalView = normalize( ( ( vec3<f32>( abs( NORMAL_nodeVar469 ) ) * NORMAL_normalView ) - nodeVar474 ) );
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar475 = dot( normalView, positionViewDirection );
	nodeVar476 = textureSample( nodeUniform7, nodeUniform7_sampler, vec2<f32>( Roughness, clamp( nodeVar475, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar476;
	nodeVar477 = ( dfg.x + dfg.y );
	nodeVar478 = ( 1.0 / nodeVar477 );
	nodeVar479 = nodeVar478;
	nodeVar480 = ( nodeVar479 - 1.0 );
	nodeVar481 = ( SpecularColorBlended * vec3<f32>( nodeVar480 ) );
	nodeVar482 = ( nodeVar481 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar482;
	nodeVar483 = vec4<f32>( render.nodeUniform9, 0.0 );
	nodeVar484 = ( render.cameraViewMatrix * nodeVar483 );
	nodeVar485 = normalize( nodeVar484.xyz );
	nodeVar486 = nodeVar485;
	nodeVar487 = dot( normalView, nodeVar486 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar488 = vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform11 ) ) ), 1.0 );
	nodeVar489 = ( - v_positionView.z );
	shadowValue = 1.0;

	if ( ( ( nodeVar489 >= render.nodeUniform12.x ) && ( nodeVar489 < render.nodeUniform12.y ) ) ) {

		nodeVar491 = ( render.nodeUniform13 * nodeVar488 );
		nodeVar492 = ( nodeVar491.xyz / vec3<f32>( nodeVar491.w ) );
		nodeVar493 = vec3<f32>( nodeVar492.x, ( 1.0 - nodeVar492.y ), ( nodeVar492.z + render.nodeUniform14 ) );

		if ( ( ( ( ( ( nodeVar493.x >= 0.0 ) && ( nodeVar493.x <= 1.0 ) ) && ( nodeVar493.y >= 0.0 ) ) && ( nodeVar493.y <= 1.0 ) ) && ( nodeVar493.z <= 1.0 ) ) ) {

			nodeVar494 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar495 = ( render.nodeUniform16 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform17 ).x );
			nodeVar496 = ( nodeVar493.xy + ( vogelDiskSample( 0, 5, nodeVar494 ) * vec2<f32>( nodeVar495 ) ) );
			nodeVar497 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar496, nodeVar493.z );
			nodeVar498 = ( nodeVar493.xy + ( vogelDiskSample( 1, 5, nodeVar494 ) * vec2<f32>( nodeVar495 ) ) );
			nodeVar499 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar498, nodeVar493.z );
			nodeVar500 = ( nodeVar493.xy + ( vogelDiskSample( 2, 5, nodeVar494 ) * vec2<f32>( nodeVar495 ) ) );
			nodeVar501 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar500, nodeVar493.z );
			nodeVar502 = ( nodeVar493.xy + ( vogelDiskSample( 3, 5, nodeVar494 ) * vec2<f32>( nodeVar495 ) ) );
			nodeVar503 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar502, nodeVar493.z );
			nodeVar504 = ( nodeVar493.xy + ( vogelDiskSample( 4, 5, nodeVar494 ) * vec2<f32>( nodeVar495 ) ) );
			nodeVar505 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar504, nodeVar493.z );
			nodeVar490 = ( ( ( ( ( nodeVar497 + nodeVar499 ) + nodeVar501 ) + nodeVar503 ) + nodeVar505 ) * 0.2 );

		} else {

			nodeVar490 = 1.0;

		}

		shadowValue = mix( nodeVar490, shadowValue, smoothstep( render.nodeUniform12.z, render.nodeUniform12.y, nodeVar489 ) );
		

	}


	if ( ( ( nodeVar489 >= render.nodeUniform18.x ) && ( nodeVar489 < render.nodeUniform18.y ) ) ) {

		nodeVar507 = ( render.nodeUniform19 * nodeVar488 );
		nodeVar508 = ( nodeVar507.xyz / vec3<f32>( nodeVar507.w ) );
		nodeVar509 = vec3<f32>( nodeVar508.x, ( 1.0 - nodeVar508.y ), ( nodeVar508.z + render.nodeUniform20 ) );

		if ( ( ( ( ( ( nodeVar509.x >= 0.0 ) && ( nodeVar509.x <= 1.0 ) ) && ( nodeVar509.y >= 0.0 ) ) && ( nodeVar509.y <= 1.0 ) ) && ( nodeVar509.z <= 1.0 ) ) ) {

			nodeVar510 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar511 = ( render.nodeUniform21 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform22 ).x );
			nodeVar512 = ( nodeVar509.xy + ( vogelDiskSample( 0, 5, nodeVar510 ) * vec2<f32>( nodeVar511 ) ) );
			nodeVar513 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar512, nodeVar509.z );
			nodeVar514 = ( nodeVar509.xy + ( vogelDiskSample( 1, 5, nodeVar510 ) * vec2<f32>( nodeVar511 ) ) );
			nodeVar515 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar514, nodeVar509.z );
			nodeVar516 = ( nodeVar509.xy + ( vogelDiskSample( 2, 5, nodeVar510 ) * vec2<f32>( nodeVar511 ) ) );
			nodeVar517 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar516, nodeVar509.z );
			nodeVar518 = ( nodeVar509.xy + ( vogelDiskSample( 3, 5, nodeVar510 ) * vec2<f32>( nodeVar511 ) ) );
			nodeVar519 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar518, nodeVar509.z );
			nodeVar520 = ( nodeVar509.xy + ( vogelDiskSample( 4, 5, nodeVar510 ) * vec2<f32>( nodeVar511 ) ) );
			nodeVar521 = textureSampleCompare( nodeUniform15, nodeUniform15_sampler, nodeVar520, nodeVar509.z );
			nodeVar506 = ( ( ( ( ( nodeVar513 + nodeVar515 ) + nodeVar517 ) + nodeVar519 ) + nodeVar521 ) * 0.2 );

		} else {

			nodeVar506 = 1.0;

		}

		shadowValue = mix( nodeVar506, shadowValue, smoothstep( render.nodeUniform18.z, render.nodeUniform18.y, nodeVar489 ) );
		

	}

	nodeVar522 = mix( 1.0, shadowValue, render.nodeUniform23 );
	nodeVar523 = ( render.nodeUniform10 * vec3<f32>( nodeVar522 ) );
	nodeVar524 = ( vec3<f32>( clamp( nodeVar487, 0.0, 1.0 ) ) * nodeVar523 );
	nodeVar525 = nodeVar524;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar526 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar527 = ( nodeVar525 * nodeVar526 );
	nodeVar528 = ( nodeVar486 + positionViewDirection );
	nodeVar529 = normalize( nodeVar528 );
	nodeVar530 = dot( positionViewDirection, nodeVar529 );
	nodeVar531 = clamp( nodeVar530, 0.0, 1.0 );
	nodeVar532 = exp2( ( ( ( nodeVar531 * -5.55473 ) - 6.98316 ) * nodeVar531 ) );
	nodeVar533 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar532 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar532 ) ) );
	nodeVar534 = ( vec3<f32>( 1.0 ) - nodeVar533 );
	nodeVar535 = nodeVar534;
	nodeVar536 = ( nodeVar527 * nodeVar535 );
	nodeVar537 = ( directDiffuse + nodeVar536 );
	directDiffuse = nodeVar537;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar538 = normalize( ( nodeVar486 + positionViewDirection ) );
	nodeVar539 = clamp( dot( positionViewDirection, nodeVar538 ), 0.0, 1.0 );
	nodeVar540 = exp2( ( ( ( nodeVar539 * -5.55473 ) - 6.98316 ) * nodeVar539 ) );
	nodeVar541 = ( Roughness * Roughness );
	nodeVar542 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar540 ) ) ) + vec3<f32>( ( 1.0 * nodeVar540 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar541, clamp( dot( normalView, nodeVar486 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar541, clamp( dot( normalView, nodeVar538 ), 0.0, 1.0 ) ) ) );
	nodeVar543 = ( nodeVar525 * nodeVar542 );
	nodeVar544 = ( nodeVar543 * multiScatteringCompensation );
	nodeVar545 = ( directSpecular + nodeVar544 );
	directSpecular = nodeVar545;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar546 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform24 );
	nodeVar547 = floor( nodeVar546 );
	nodeVar548 = nodeVar547;
	nodeVar549 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar550 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar549.x, ( - nodeVar549.y ), nodeVar549.z ), 1.0 ) ).xyz );
	nodeVar551 = max( ( 4.0 - nodeVar548 ), 0.0 );
	nodeVar548 = max( nodeVar548, 4.0 );
	nodeVar552 = exp2( nodeVar548 );
	nodeVar553 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar549.x, ( - nodeVar549.y ), nodeVar549.z ), 1.0 ) ).xyz, nodeVar550 ) * vec2<f32>( ( nodeVar552 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar550 > 2.0 ) ) {

		nodeVar553.y = ( nodeVar553.y + nodeVar552 );
		nodeVar550 = ( nodeVar550 - 3.0 );
		

	}

	nodeVar553.x = ( nodeVar553.x + ( nodeVar550 * nodeVar552 ) );
	nodeVar553.x = ( nodeVar553.x + ( nodeVar551 * ( 3.0 * 16.0 ) ) );
	nodeVar553.y = ( nodeVar553.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar552 ) ) );
	nodeVar553.x = ( nodeVar553.x * object.nodeUniform27 );
	nodeVar553.y = ( nodeVar553.y * object.nodeUniform28 );
	nodeVar554 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar553, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar555 = nodeVar554.xyz;
	nodeVar556 = fract( nodeVar546 );

	if ( ( nodeVar556 != 0.0 ) ) {

		nodeVar557 = ( nodeVar547 + 1.0 );
		nodeVar558 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar549.x, ( - nodeVar549.y ), nodeVar549.z ), 1.0 ) ).xyz );
		nodeVar559 = max( ( 4.0 - nodeVar557 ), 0.0 );
		nodeVar557 = max( nodeVar557, 4.0 );
		nodeVar560 = exp2( nodeVar557 );
		nodeVar561 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( nodeVar549.x, ( - nodeVar549.y ), nodeVar549.z ), 1.0 ) ).xyz, nodeVar558 ) * vec2<f32>( ( nodeVar560 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar558 > 2.0 ) ) {

			nodeVar561.y = ( nodeVar561.y + nodeVar560 );
			nodeVar558 = ( nodeVar558 - 3.0 );
			

		}

		nodeVar561.x = ( nodeVar561.x + ( nodeVar558 * nodeVar560 ) );
		nodeVar561.x = ( nodeVar561.x + ( nodeVar559 * ( 3.0 * 16.0 ) ) );
		nodeVar561.y = ( nodeVar561.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar560 ) ) );
		nodeVar561.x = ( nodeVar561.x * object.nodeUniform27 );
		nodeVar561.y = ( nodeVar561.y * object.nodeUniform28 );
		nodeVar562 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar561, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar563 = nodeVar562.xyz;
		nodeVar555 = mix( nodeVar555, nodeVar563, nodeVar556 );
		

	}

	nodeVar564 = ( radiance + ( nodeVar555 * vec3<f32>( object.nodeUniform30 ) ) );
	radiance = nodeVar564;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar565 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform24 );
	nodeVar566 = floor( nodeVar565 );
	nodeVar567 = nodeVar566;
	nodeVar568 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar569 = max( ( 4.0 - nodeVar567 ), 0.0 );
	nodeVar567 = max( nodeVar567, 4.0 );
	nodeVar570 = exp2( nodeVar567 );
	nodeVar571 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar568 ) * vec2<f32>( ( nodeVar570 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar568 > 2.0 ) ) {

		nodeVar571.y = ( nodeVar571.y + nodeVar570 );
		nodeVar568 = ( nodeVar568 - 3.0 );
		

	}

	nodeVar571.x = ( nodeVar571.x + ( nodeVar568 * nodeVar570 ) );
	nodeVar571.x = ( nodeVar571.x + ( nodeVar569 * ( 3.0 * 16.0 ) ) );
	nodeVar571.y = ( nodeVar571.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar570 ) ) );
	nodeVar571.x = ( nodeVar571.x * object.nodeUniform27 );
	nodeVar571.y = ( nodeVar571.y * object.nodeUniform28 );
	nodeVar572 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar571, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar573 = nodeVar572.xyz;
	nodeVar574 = fract( nodeVar565 );

	if ( ( nodeVar574 != 0.0 ) ) {

		nodeVar575 = ( nodeVar566 + 1.0 );
		nodeVar576 = getFace( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar577 = max( ( 4.0 - nodeVar575 ), 0.0 );
		nodeVar575 = max( nodeVar575, 4.0 );
		nodeVar578 = exp2( nodeVar575 );
		nodeVar579 = ( ( getUV( ( object.nodeUniform25 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar576 ) * vec2<f32>( ( nodeVar578 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar576 > 2.0 ) ) {

			nodeVar579.y = ( nodeVar579.y + nodeVar578 );
			nodeVar576 = ( nodeVar576 - 3.0 );
			

		}

		nodeVar579.x = ( nodeVar579.x + ( nodeVar576 * nodeVar578 ) );
		nodeVar579.x = ( nodeVar579.x + ( nodeVar577 * ( 3.0 * 16.0 ) ) );
		nodeVar579.y = ( nodeVar579.y + ( 4.0 * ( exp2( object.nodeUniform24 ) - nodeVar578 ) ) );
		nodeVar579.x = ( nodeVar579.x * object.nodeUniform27 );
		nodeVar579.y = ( nodeVar579.y * object.nodeUniform28 );
		nodeVar580 = textureSampleGrad( nodeUniform29, nodeUniform29_sampler, nodeVar579, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar581 = nodeVar580.xyz;
		nodeVar573 = mix( nodeVar573, nodeVar581, nodeVar574 );
		

	}

	nodeVar582 = ( iblIrradiance + ( ( nodeVar573 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform30 ) ) );
	iblIrradiance = nodeVar582;
	nodeVar583 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar584 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar585 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar586 = ( SpecularF90 * dfg.y );
	nodeVar587 = ( nodeVar585 + vec3<f32>( nodeVar586 ) );
	nodeVar588 = ( nodeVar583 + nodeVar587 );
	nodeVar583 = nodeVar588;
	nodeVar589 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar590 = nodeVar589;
	nodeVar591 = ( nodeVar590 * vec3<f32>( 0.047619 ) );
	nodeVar592 = ( SpecularColor + nodeVar591 );
	nodeVar593 = ( nodeVar587 * nodeVar592 );
	nodeVar594 = ( dfg.x + dfg.y );
	nodeVar595 = ( 1.0 - nodeVar594 );
	nodeVar596 = nodeVar595;
	nodeVar597 = ( vec3<f32>( nodeVar596 ) * nodeVar592 );
	nodeVar598 = ( vec3<f32>( 1.0 ) - nodeVar597 );
	nodeVar599 = nodeVar598;
	nodeVar600 = ( nodeVar593 / nodeVar599 );
	nodeVar601 = ( nodeVar600 * vec3<f32>( nodeVar596 ) );
	nodeVar602 = ( nodeVar584 + nodeVar601 );
	nodeVar584 = nodeVar602;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar603 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar604 = ( irradiance * nodeVar603 );
	nodeVar605 = ( nodeVar583 + nodeVar584 );
	nodeVar606 = ( vec3<f32>( 1.0 ) - nodeVar605 );
	nodeVar607 = nodeVar606;
	nodeVar608 = ( nodeVar604 * nodeVar607 );
	nodeVar609 = nodeVar608;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar610 = ( indirectDiffuse + nodeVar609 );
	indirectDiffuse = nodeVar610;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar611 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar612 = ( SpecularF90 * dfg.y );
	nodeVar613 = ( nodeVar611 + vec3<f32>( nodeVar612 ) );
	nodeVar614 = ( singleScatteringDielectric + nodeVar613 );
	singleScatteringDielectric = nodeVar614;
	nodeVar615 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar616 = nodeVar615;
	nodeVar617 = ( nodeVar616 * vec3<f32>( 0.047619 ) );
	nodeVar618 = ( SpecularColor + nodeVar617 );
	nodeVar619 = ( nodeVar613 * nodeVar618 );
	nodeVar620 = ( dfg.x + dfg.y );
	nodeVar621 = ( 1.0 - nodeVar620 );
	nodeVar622 = nodeVar621;
	nodeVar623 = ( vec3<f32>( nodeVar622 ) * nodeVar618 );
	nodeVar624 = ( vec3<f32>( 1.0 ) - nodeVar623 );
	nodeVar625 = nodeVar624;
	nodeVar626 = ( nodeVar619 / nodeVar625 );
	nodeVar627 = ( nodeVar626 * vec3<f32>( nodeVar622 ) );
	nodeVar628 = ( multiScatteringDielectric + nodeVar627 );
	multiScatteringDielectric = nodeVar628;
	nodeVar629 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar630 = ( SpecularF90 * dfg.y );
	nodeVar631 = ( nodeVar629 + vec3<f32>( nodeVar630 ) );
	nodeVar632 = ( singleScatteringMetallic + nodeVar631 );
	singleScatteringMetallic = nodeVar632;
	nodeVar633 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar634 = nodeVar633;
	nodeVar635 = ( nodeVar634 * vec3<f32>( 0.047619 ) );
	nodeVar636 = ( DiffuseColor.xyz + nodeVar635 );
	nodeVar637 = ( nodeVar631 * nodeVar636 );
	nodeVar638 = ( dfg.x + dfg.y );
	nodeVar639 = ( 1.0 - nodeVar638 );
	nodeVar640 = nodeVar639;
	nodeVar641 = ( vec3<f32>( nodeVar640 ) * nodeVar636 );
	nodeVar642 = ( vec3<f32>( 1.0 ) - nodeVar641 );
	nodeVar643 = nodeVar642;
	nodeVar644 = ( nodeVar637 / nodeVar643 );
	nodeVar645 = ( nodeVar644 * vec3<f32>( nodeVar640 ) );
	nodeVar646 = ( multiScatteringMetallic + nodeVar645 );
	multiScatteringMetallic = nodeVar646;
	nodeVar647 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar648 = ( radiance * nodeVar647 );
	nodeVar649 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar650 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar651 = ( nodeVar649 * nodeVar650 );
	nodeVar652 = ( nodeVar648 + nodeVar651 );
	nodeVar653 = nodeVar652;
	nodeVar654 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar655 = ( vec3<f32>( 1.0 ) - nodeVar654 );
	nodeVar656 = nodeVar655;
	nodeVar657 = ( DiffuseContribution * nodeVar656 );
	nodeVar658 = ( nodeVar657 * nodeVar650 );
	nodeVar659 = nodeVar658;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar660 = ( indirectSpecular + nodeVar653 );
	indirectSpecular = nodeVar660;
	nodeVar661 = ( indirectDiffuse + nodeVar659 );
	indirectDiffuse = nodeVar661;
	ambientOcclusion = 1.0;
	nodeVar662 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar662;
	nodeVar663 = dot( normalView, positionViewDirection );
	nodeVar664 = ( clamp( nodeVar663, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar665 = ( Roughness * -16.0 );
	nodeVar666 = ( 1.0 - nodeVar665 );
	nodeVar667 = nodeVar666;
	nodeVar668 = ( - nodeVar667 );
	nodeVar669 = exp2( nodeVar668 );
	nodeVar670 = pow( nodeVar664, nodeVar669 );
	nodeVar671 = ( 1.0 - nodeVar670 );
	nodeVar672 = nodeVar671;
	nodeVar673 = ( ambientOcclusion - nodeVar672 );
	nodeVar674 = ( indirectSpecular * vec3<f32>( clamp( nodeVar673, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar674;
	nodeVar675 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar675;
	nodeVar676 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar676;
	nodeVar677 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar677;
	nodeVar678 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar678;

	// result

	output.color = nodeVar678;

	return output;

}
