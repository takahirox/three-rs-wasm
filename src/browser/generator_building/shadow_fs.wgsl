// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms

struct objectStruct {
	nodeUniform0 : mat4x4<f32>,
	nodeUniform2 : mat4x4<f32>,
	nodeUniform3 : vec3<f32>,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraPosition : vec3<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : bool;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : u32;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> normalLocal : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : u32;
var<private> nodeVar20 : u32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : u32;
var<private> nodeVar23 : u32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : vec3<f32>;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : vec3<f32>;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : u32;
var<private> nodeVar35 : u32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : u32;
var<private> nodeVar39 : u32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : vec3<f32>;
var<private> nodeVar46 : vec3<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec3<f32>;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : u32;
var<private> nodeVar53 : u32;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec3<f32>;
var<private> nodeVar58 : u32;
var<private> nodeVar59 : u32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : u32;
var<private> nodeVar69 : u32;
var<private> nodeVar70 : f32;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : u32;
var<private> nodeVar75 : u32;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : u32;
var<private> nodeVar85 : u32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : u32;
var<private> nodeVar91 : u32;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : bool;
var<private> nodeVar109 : bool;
var<private> nodeVar110 : bool;
var<private> nodeVar111 : bool;
var<private> nodeVar112 : bool;
var<private> nodeVar113 : bool;
var<private> nodeVar114 : bool;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : f32;
var<private> nodeVar119 : u32;
var<private> nodeVar120 : u32;
var<private> nodeVar121 : u32;
var<private> nodeVar122 : f32;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : f32;
var<private> nodeVar128 : u32;
var<private> nodeVar129 : u32;
var<private> nodeVar130 : f32;
var<private> nodeVar131 : u32;
var<private> nodeVar132 : u32;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : u32;
var<private> nodeVar135 : u32;
var<private> nodeVar136 : f32;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : f32;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : vec3<f32>;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : bool;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : f32;
var<private> nodeVar175 : u32;
var<private> nodeVar176 : u32;
var<private> nodeVar177 : u32;
var<private> nodeVar178 : f32;
var<private> nodeVar179 : vec3<f32>;
var<private> nodeVar180 : vec3<f32>;
var<private> nodeVar181 : vec3<f32>;
var<private> nodeVar182 : vec3<f32>;
var<private> nodeVar183 : f32;
var<private> nodeVar184 : u32;
var<private> nodeVar185 : u32;
var<private> nodeVar186 : f32;
var<private> nodeVar187 : u32;
var<private> nodeVar188 : u32;
var<private> nodeVar189 : vec3<f32>;
var<private> nodeVar190 : u32;
var<private> nodeVar191 : u32;
var<private> nodeVar192 : f32;
var<private> nodeVar193 : vec3<f32>;
var<private> nodeVar194 : vec3<f32>;
var<private> nodeVar195 : vec3<f32>;
var<private> nodeVar196 : vec3<f32>;
var<private> nodeVar197 : f32;
var<private> nodeVar198 : vec3<f32>;
var<private> nodeVar199 : u32;
var<private> nodeVar200 : u32;
var<private> nodeVar201 : f32;
var<private> nodeVar202 : vec3<f32>;
var<private> nodeVar203 : vec3<f32>;
var<private> nodeVar204 : vec3<f32>;
var<private> nodeVar205 : vec3<f32>;
var<private> nodeVar206 : vec3<f32>;
var<private> nodeVar207 : vec3<f32>;
var<private> nodeVar208 : u32;
var<private> nodeVar209 : u32;
var<private> nodeVar210 : u32;
var<private> nodeVar211 : u32;
var<private> nodeVar212 : u32;
var<private> nodeVar213 : u32;
var<private> nodeVar214 : vec3<f32>;
var<private> nodeVar215 : vec3<f32>;
var<private> nodeVar216 : u32;
var<private> nodeVar217 : u32;
var<private> nodeVar218 : u32;
var<private> nodeVar219 : u32;
var<private> nodeVar220 : vec3<f32>;
var<private> nodeVar221 : vec3<f32>;
var<private> nodeVar222 : f32;
var<private> nodeVar223 : u32;
var<private> nodeVar224 : u32;
var<private> nodeVar225 : u32;
var<private> nodeVar226 : f32;
var<private> nodeVar227 : vec3<f32>;
var<private> nodeVar228 : vec3<f32>;
var<private> nodeVar229 : vec3<f32>;
var<private> nodeVar230 : vec3<f32>;
var<private> nodeVar231 : f32;
var<private> nodeVar232 : u32;
var<private> nodeVar233 : u32;
var<private> nodeVar234 : f32;
var<private> nodeVar235 : u32;
var<private> nodeVar236 : u32;
var<private> nodeVar237 : f32;
var<private> nodeVar238 : f32;
var<private> nodeVar239 : vec3<f32>;
var<private> nodeVar240 : u32;
var<private> nodeVar241 : u32;
var<private> nodeVar242 : u32;
var<private> nodeVar243 : u32;
var<private> nodeVar244 : u32;
var<private> nodeVar245 : u32;
var<private> nodeVar246 : f32;
var<private> nodeVar247 : f32;
var<private> nodeVar248 : f32;
var<private> nodeVar249 : f32;
var<private> nodeVar250 : f32;
var<private> nodeVar251 : f32;
var<private> nodeVar252 : f32;
var<private> nodeVar253 : vec3<f32>;
var<private> nodeVar254 : u32;
var<private> nodeVar255 : vec3<f32>;
var<private> nodeVar256 : vec3<f32>;
var<private> nodeVar257 : u32;
var<private> nodeVar258 : u32;
var<private> nodeVar259 : f32;
var<private> nodeVar260 : f32;
var<private> nodeVar261 : f32;
var<private> nodeVar262 : vec3<f32>;
var<private> nodeVar263 : vec3<f32>;
var<private> nodeVar264 : vec3<f32>;
var<private> nodeVar265 : vec3<f32>;
var<private> nodeVar266 : vec3<f32>;
var<private> nodeVar267 : vec3<f32>;
var<private> nodeVar268 : vec3<f32>;
var<private> nodeVar269 : f32;
var<private> nodeVar270 : vec3<f32>;
var<private> nodeVar271 : vec3<f32>;
var<private> nodeVar272 : vec3<f32>;
var<private> nodeVar273 : f32;
var<private> nodeVar274 : f32;
var<private> nodeVar275 : vec3<f32>;
var<private> nodeVar276 : u32;
var<private> nodeVar277 : u32;
var<private> nodeVar278 : f32;
var<private> nodeVar279 : f32;
var<private> nodeVar280 : f32;
var<private> nodeVar281 : vec3<f32>;
var<private> nodeVar282 : vec3<f32>;
var<private> nodeVar283 : vec3<f32>;
var<private> nodeVar284 : f32;
var<private> nodeVar285 : f32;
var<private> nodeVar286 : f32;
var<private> nodeVar287 : vec3<f32>;
var<private> nodeVar288 : vec3<f32>;
var<private> nodeVar289 : vec3<f32>;
var<private> nodeVar290 : vec3<f32>;
var<private> nodeVar291 : vec3<f32>;
var<private> nodeVar292 : vec3<f32>;
var<private> nodeVar293 : f32;
var<private> nodeVar294 : u32;
var<private> nodeVar295 : u32;
var<private> nodeVar296 : f32;
var<private> nodeVar297 : f32;
var<private> nodeVar298 : vec3<f32>;
var<private> nodeVar299 : vec3<f32>;
var<private> nodeVar300 : vec3<f32>;
var<private> nodeVar301 : vec3<f32>;
var<private> nodeVar302 : vec3<f32>;
var<private> nodeVar303 : vec3<f32>;
var<private> nodeVar304 : f32;
var<private> nodeVar305 : u32;
var<private> nodeVar306 : u32;
var<private> nodeVar307 : f32;
var<private> nodeVar308 : f32;
var<private> nodeVar309 : vec3<f32>;
var<private> nodeVar310 : vec3<f32>;
var<private> nodeVar311 : vec3<f32>;
var<private> nodeVar312 : vec3<f32>;
var<private> nodeVar313 : vec3<f32>;
var<private> nodeVar314 : vec3<f32>;
var<private> nodeVar315 : f32;
var<private> nodeVar316 : f32;
var<private> nodeVar317 : f32;
var<private> nodeVar318 : f32;
var<private> nodeVar319 : f32;
var<private> nodeVar320 : vec3<f32>;
var<private> nodeVar321 : vec3<f32>;
var<private> nodeVar322 : vec3<f32>;
var<private> nodeVar323 : vec3<f32>;
var<private> nodeVar324 : vec3<f32>;
var<private> nodeVar325 : vec3<f32>;
var<private> nodeVar326 : f32;
var<private> nodeVar327 : vec3<f32>;
var<private> nodeVar328 : f32;
var<private> nodeVar329 : bool;
var<private> nodeVar330 : bool;
var<private> nodeVar331 : bool;
var<private> nodeVar332 : bool;
var<private> nodeVar333 : bool;
var<private> nodeVar334 : bool;
var<private> nodeVar335 : vec3<f32>;
var<private> nodeVar336 : u32;
var<private> nodeVar337 : u32;
var<private> nodeVar338 : f32;
var<private> nodeVar339 : vec3<f32>;
var<private> nodeVar340 : vec3<f32>;
var<private> nodeVar341 : vec3<f32>;
var<private> nodeVar342 : vec3<f32>;
var<private> nodeVar343 : vec3<f32>;
var<private> nodeVar344 : vec3<f32>;
var<private> nodeVar345 : vec3<f32>;
var<private> nodeVar346 : u32;
var<private> nodeVar347 : u32;
var<private> nodeVar348 : f32;
var<private> nodeVar349 : vec3<f32>;
var<private> nodeVar350 : vec3<f32>;
var<private> nodeVar351 : vec3<f32>;
var<private> nodeVar352 : vec3<f32>;
var<private> nodeVar353 : vec3<f32>;
var<private> nodeVar354 : vec3<f32>;
var<private> nodeVar355 : f32;
var<private> nodeVar356 : vec3<f32>;
var<private> nodeVar357 : vec3<f32>;
var<private> nodeVar358 : f32;
var<private> nodeVar359 : vec3<f32>;
var<private> nodeVar360 : vec3<f32>;
var<private> nodeVar361 : f32;
var<private> nodeVar362 : vec3<f32>;
var<private> nodeVar363 : vec3<f32>;
var<private> nodeVar364 : f32;
var<private> nodeVar365 : vec3<f32>;
var<private> nodeVar366 : vec3<f32>;
var<private> nodeVar367 : vec3<f32>;
var<private> nodeVar368 : vec3<f32>;
var<private> nodeVar369 : bool;
var<private> nodeVar370 : vec3<f32>;
var<private> nodeVar371 : f32;
var<private> nodeVar372 : f32;
var<private> nodeVar373 : u32;
var<private> nodeVar374 : u32;
var<private> nodeVar375 : vec3<f32>;
var<private> nodeVar376 : vec3<f32>;
var<private> nodeVar377 : u32;
var<private> nodeVar378 : u32;
var<private> nodeVar379 : u32;
var<private> nodeVar380 : u32;
var<private> nodeVar381 : u32;
var<private> nodeVar382 : u32;
var<private> nodeVar383 : u32;
var<private> nodeVar384 : u32;
var<private> nodeVar385 : vec3<f32>;
var<private> nodeVar386 : vec3<f32>;
var<private> nodeVar387 : f32;
var<private> nodeVar388 : f32;
var<private> nodeVar389 : vec3<f32>;
var<private> nodeVar390 : u32;
var<private> nodeVar391 : u32;
var<private> nodeVar392 : u32;
var<private> nodeVar393 : u32;
var<private> nodeVar394 : u32;
var<private> nodeVar395 : u32;
var<private> nodeVar396 : u32;
var<private> nodeVar397 : u32;
var<private> nodeVar398 : f32;
var<private> nodeVar399 : f32;
var<private> nodeVar400 : f32;
var<private> nodeVar401 : f32;
var<private> nodeVar402 : f32;
var<private> nodeVar403 : f32;
var<private> nodeVar404 : f32;
var<private> nodeVar405 : vec4<f32>;
var<private> nodeVar406 : vec3<f32>;
var<private> nodeVar407 : vec3<f32>;
var<private> nodeVar408 : vec3<f32>;
var<private> nodeVar409 : f32;
var<private> nodeVar410 : vec3<f32>;
var<private> nodeVar411 : vec3<f32>;
var<private> nodeVar412 : vec3<f32>;
var<private> nodeVar413 : vec2<f32>;
var<private> nodeVar414 : u32;
var<private> nodeVar415 : u32;
var<private> nodeVar416 : f32;
var<private> nodeVar417 : vec3<f32>;
var<private> nodeVar418 : vec3<f32>;
var<private> nodeVar419 : vec3<f32>;
var<private> nodeVar420 : vec3<f32>;
var<private> nodeVar421 : vec3<f32>;
var<private> nodeVar422 : bool;
var<private> nodeVar424 : vec3<f32>;
var<private> nodeVar425 : f32;
var<private> normalWorldGeometry : vec3<f32>;
var<private> nodeVar426 : vec3<f32>;
var<private> nodeVar427 : f32;
var<private> nodeVar428 : f32;
var<private> nodeVar429 : f32;
var<private> nodeVar430 : f32;
var<private> nodeVar431 : f32;
var<private> nodeVar432 : u32;
var<private> nodeVar433 : u32;
var<private> nodeVar434 : u32;
var<private> nodeVar435 : u32;
var<private> nodeVar436 : u32;
var<private> nodeVar437 : f32;
var<private> nodeVar438 : vec3<f32>;
var<private> nodeVar439 : vec3<f32>;
var<private> nodeVar440 : f32;
var<private> nodeVar441 : f32;
var<private> nodeVar442 : f32;
var<private> nodeVar443 : f32;
var<private> nodeVar444 : f32;
var<private> nodeVar445 : f32;
var<private> nodeVar446 : f32;
var<private> nodeVar447 : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> nodeVar448 : vec4<f32>;

// codes
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




@fragment
fn main( @location( 0 ) positionLocal : vec3<f32>,
	@location( 1 ) @interpolate( flat, either ) nodeVarying3 : f32,
	@location( 2 ) @interpolate( flat, either ) nodeVarying4 : vec3<f32>,
	@location( 3 ) @interpolate( flat, either ) nodeVarying5 : vec3<f32>,
	@location( 4 ) v_positionWorld : vec3<f32>,
	@location( 5 ) nodeVarying7 : f32,
	@location( 6 ) v_normalWorldGeometry : vec3<f32>,
	@location( 7 ) nodeVarying11 : vec2<f32>,
	@location( 8 ) nodeVarying12 : vec3<f32>,
	@location( 9 ) nodeVarying13 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar1 = ( nodeVarying3 == 6.0 );

	if ( ( ( nodeVarying3 == 4.0 ) || nodeVar1 ) ) {


		if ( nodeVar1 ) {

			nodeVar3 = floor( ( nodeVarying4 * vec3<f32>( 2.0 ) ) );
			nodeVar4 = ( ( ( u32( ( nodeVar3.x + 2097152.0 ) ) * 73856093u ) ^ ( u32( ( nodeVar3.y + 2097152.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeVar3.z + 2097152.0 ) ) * 83492791u ) );
			nodeVar6 = ( nodeVarying11.y * 2.4 );
			nodeVar7 = vec3<f32>( ( nodeVarying11.x * 0.5 ), ( nodeVarying11.y * 0.5 ), ( 0.1 + nodeVar6 ) );
			nodeVar8 = vec3<f32>( ( - nodeVar7.x ), ( - nodeVar7.y ), 0.1 );
			nodeVar9 = ( positionLocal - nodeVarying4 );
			normalLocal = nodeVarying12;
			nodeVar10 = normalize( cross( vec3<f32>( 0.0, 1.0, 0.0 ), normalLocal ) );
			nodeVar11 = vec3<f32>( dot( nodeVar9, nodeVar10 ), nodeVar9.y, 0.0 );
			nodeVar12 = normalize( ( positionLocal - ( object.nodeUniform0 * vec4<f32>( render.cameraPosition, 1.0 ) ).xyz ) );
			nodeVar13 = vec3<f32>( dot( nodeVar12, nodeVar10 ), nodeVar12.y, ( - dot( nodeVar12, normalLocal ) ) );
			nodeVar14 = ( ( vec3<f32>( ( - nodeVar7.x ), nodeVar8.y, 0.1 ) - nodeVar11 ) / nodeVar13 );
			nodeVar15 = ( ( vec3<f32>( nodeVar7.x, nodeVar7.y, ( 0.1 + 0.05 ) ) - nodeVar11 ) / nodeVar13 );
			nodeVar16 = max( nodeVar14, nodeVar15 );
			nodeVar17 = min( nodeVar14, nodeVar15 );
			nodeVar18 = max( max( nodeVar17.x, nodeVar17.y ), nodeVar17.z );
			nodeVar19 = ( ( ( nodeVar4 + 79599u ) * 747796405u ) + 2891336453u );
			nodeVar20 = ( ( ( nodeVar19 >> ( ( nodeVar19 >> 28u ) + 4u ) ) ^ nodeVar19 ) * 277803737u );
			nodeVar21 = step( 0.18, ( f32( ( ( nodeVar20 >> 22u ) ^ nodeVar20 ) ) * 2.3283064365386963e-10 ) );
			nodeVar22 = ( ( ( nodeVar4 + 39650u ) * 747796405u ) + 2891336453u );
			nodeVar23 = ( ( ( nodeVar22 >> ( ( nodeVar22 >> 28u ) + 4u ) ) ^ nodeVar22 ) * 277803737u );
			nodeVar25 = ( 0.1 + 1.2 );
			nodeVar26 = vec3<f32>( -0.45, nodeVar8.y, nodeVar25 );
			nodeVar27 = ( ( nodeVar26 - nodeVar11 ) / nodeVar13 );
			nodeVar28 = ( nodeVar7.z - 1.2 );
			nodeVar29 = vec3<f32>( 0.45, ( nodeVar8.y + 1.5 ), nodeVar28 );
			nodeVar30 = ( ( nodeVar29 - nodeVar11 ) / nodeVar13 );
			nodeVar31 = max( nodeVar27, nodeVar30 );
			nodeVar32 = min( nodeVar27, nodeVar30 );
			nodeVar33 = max( max( nodeVar32.x, nodeVar32.y ), nodeVar32.z );
			nodeVar34 = ( ( ( nodeVar4 + 34690u ) * 747796405u ) + 2891336453u );
			nodeVar35 = ( ( ( nodeVar34 >> ( ( nodeVar34 >> 28u ) + 4u ) ) ^ nodeVar34 ) * 277803737u );
			nodeVar36 = step( 0.45, ( f32( ( ( nodeVar35 >> 22u ) ^ nodeVar35 ) ) * 2.3283064365386963e-10 ) );
			nodeVar38 = ( ( ( nodeVar4 + 105097u ) * 747796405u ) + 2891336453u );
			nodeVar39 = ( ( ( nodeVar38 >> ( ( nodeVar38 >> 28u ) + 4u ) ) ^ nodeVar38 ) * 277803737u );
			nodeVar40 = ( f32( ( ( nodeVar39 >> 22u ) ^ nodeVar39 ) ) * 2.3283064365386963e-10 );
			nodeVar41 = mix( ( ( - nodeVar7.x ) + 0.7 ), ( nodeVar7.x - 0.7 ), step( 0.5, nodeVar40 ) );
			nodeVar42 = ( 0.1 + ( nodeVar6 * 0.5 ) );
			nodeVar43 = vec3<f32>( ( nodeVar41 - 0.75 ), nodeVar8.y, nodeVar42 );
			nodeVar44 = ( ( nodeVar43 - nodeVar11 ) / nodeVar13 );
			nodeVar45 = vec3<f32>( ( nodeVar41 + 0.75 ), ( nodeVar8.y + 1.0 ), ( nodeVar42 + 0.65 ) );
			nodeVar46 = ( ( nodeVar45 - nodeVar11 ) / nodeVar13 );
			nodeVar47 = max( nodeVar44, nodeVar46 );
			nodeVar48 = min( nodeVar44, nodeVar46 );
			nodeVar49 = max( max( nodeVar48.x, nodeVar48.y ), nodeVar48.z );
			nodeVar51 = ( nodeVar7.x * 0.62 );
			nodeVar52 = ( ( ( nodeVar4 + 40310u ) * 747796405u ) + 2891336453u );
			nodeVar53 = ( ( ( nodeVar52 >> ( ( nodeVar52 >> 28u ) + 4u ) ) ^ nodeVar52 ) * 277803737u );
			nodeVar54 = ( f32( ( ( nodeVar53 >> 22u ) ^ nodeVar53 ) ) * 2.3283064365386963e-10 );
			nodeVar55 = mix( 0.16, 0.3, nodeVar54 );
			nodeVar56 = vec3<f32>( ( nodeVar51 - nodeVar55 ), ( nodeVar8.y + 0.4 ), ( 0.1 + 0.25 ) );
			nodeVar57 = ( ( nodeVar56 - nodeVar11 ) / nodeVar13 );
			nodeVar58 = ( ( ( nodeVar4 + 66130u ) * 747796405u ) + 2891336453u );
			nodeVar59 = ( ( ( nodeVar58 >> ( ( nodeVar58 >> 28u ) + 4u ) ) ^ nodeVar58 ) * 277803737u );
			nodeVar60 = ( f32( ( ( nodeVar59 >> 22u ) ^ nodeVar59 ) ) * 2.3283064365386963e-10 );
			nodeVar61 = vec3<f32>( ( nodeVar51 + nodeVar55 ), ( ( nodeVar8.y + 0.4 ) + mix( 0.5, 1.2, nodeVar60 ) ), ( ( 0.1 + 0.25 ) + ( nodeVar55 * 2.0 ) ) );
			nodeVar62 = ( ( nodeVar61 - nodeVar11 ) / nodeVar13 );
			nodeVar63 = max( nodeVar57, nodeVar62 );
			nodeVar64 = min( nodeVar57, nodeVar62 );
			nodeVar65 = max( max( nodeVar64.x, nodeVar64.y ), nodeVar64.z );
			nodeVar67 = ( nodeVar7.x * 0.0 );
			nodeVar68 = ( ( ( nodeVar4 + 39610u ) * 747796405u ) + 2891336453u );
			nodeVar69 = ( ( ( nodeVar68 >> ( ( nodeVar68 >> 28u ) + 4u ) ) ^ nodeVar68 ) * 277803737u );
			nodeVar70 = ( f32( ( ( nodeVar69 >> 22u ) ^ nodeVar69 ) ) * 2.3283064365386963e-10 );
			nodeVar71 = mix( 0.16, 0.3, nodeVar70 );
			nodeVar72 = vec3<f32>( ( nodeVar67 - nodeVar71 ), ( nodeVar8.y + 0.4 ), ( 0.1 + 0.25 ) );
			nodeVar73 = ( ( nodeVar72 - nodeVar11 ) / nodeVar13 );
			nodeVar74 = ( ( ( nodeVar4 + 66030u ) * 747796405u ) + 2891336453u );
			nodeVar75 = ( ( ( nodeVar74 >> ( ( nodeVar74 >> 28u ) + 4u ) ) ^ nodeVar74 ) * 277803737u );
			nodeVar76 = ( f32( ( ( nodeVar75 >> 22u ) ^ nodeVar75 ) ) * 2.3283064365386963e-10 );
			nodeVar77 = vec3<f32>( ( nodeVar67 + nodeVar71 ), ( ( nodeVar8.y + 0.4 ) + mix( 0.5, 1.2, nodeVar76 ) ), ( ( 0.1 + 0.25 ) + ( nodeVar71 * 2.0 ) ) );
			nodeVar78 = ( ( nodeVar77 - nodeVar11 ) / nodeVar13 );
			nodeVar79 = max( nodeVar73, nodeVar78 );
			nodeVar80 = min( nodeVar73, nodeVar78 );
			nodeVar81 = max( max( nodeVar80.x, nodeVar80.y ), nodeVar80.z );
			nodeVar83 = ( nodeVar7.x * -0.62 );
			nodeVar84 = ( ( ( nodeVar4 + 38910u ) * 747796405u ) + 2891336453u );
			nodeVar85 = ( ( ( nodeVar84 >> ( ( nodeVar84 >> 28u ) + 4u ) ) ^ nodeVar84 ) * 277803737u );
			nodeVar86 = ( f32( ( ( nodeVar85 >> 22u ) ^ nodeVar85 ) ) * 2.3283064365386963e-10 );
			nodeVar87 = mix( 0.16, 0.3, nodeVar86 );
			nodeVar88 = vec3<f32>( ( nodeVar83 - nodeVar87 ), ( nodeVar8.y + 0.4 ), ( 0.1 + 0.25 ) );
			nodeVar89 = ( ( nodeVar88 - nodeVar11 ) / nodeVar13 );
			nodeVar90 = ( ( ( nodeVar4 + 65930u ) * 747796405u ) + 2891336453u );
			nodeVar91 = ( ( ( nodeVar90 >> ( ( nodeVar90 >> 28u ) + 4u ) ) ^ nodeVar90 ) * 277803737u );
			nodeVar92 = ( f32( ( ( nodeVar91 >> 22u ) ^ nodeVar91 ) ) * 2.3283064365386963e-10 );
			nodeVar93 = vec3<f32>( ( nodeVar83 + nodeVar87 ), ( ( nodeVar8.y + 0.4 ) + mix( 0.5, 1.2, nodeVar92 ) ), ( ( 0.1 + 0.25 ) + ( nodeVar87 * 2.0 ) ) );
			nodeVar94 = ( ( nodeVar93 - nodeVar11 ) / nodeVar13 );
			nodeVar95 = max( nodeVar89, nodeVar94 );
			nodeVar96 = min( nodeVar89, nodeVar94 );
			nodeVar97 = max( max( nodeVar96.x, nodeVar96.y ), nodeVar96.z );
			nodeVar99 = vec3<f32>( ( ( - nodeVar7.x ) + 0.15 ), nodeVar8.y, ( 0.1 + 0.06 ) );
			nodeVar100 = ( ( nodeVar99 - nodeVar11 ) / nodeVar13 );
			nodeVar101 = vec3<f32>( ( nodeVar7.x - 0.15 ), ( nodeVar8.y + 0.4 ), ( 0.1 + 0.95 ) );
			nodeVar102 = ( ( nodeVar101 - nodeVar11 ) / nodeVar13 );
			nodeVar103 = max( nodeVar100, nodeVar102 );
			nodeVar104 = min( nodeVar100, nodeVar102 );
			nodeVar105 = max( max( nodeVar104.x, nodeVar104.y ), nodeVar104.z );
			nodeVar106 = max( ( ( nodeVar8 - nodeVar11 ) / nodeVar13 ), ( ( nodeVar7 - nodeVar11 ) / nodeVar13 ) );
			nodeVar107 = min( min( nodeVar106.x, nodeVar106.y ), nodeVar106.z );
			nodeVar108 = ( ( ( min( min( nodeVar103.x, nodeVar103.y ), nodeVar103.z ) > nodeVar105 ) && ( nodeVar105 > 0.0 ) ) && ( nodeVar105 < nodeVar107 ) );

			if ( nodeVar108 ) {

				nodeVar98 = nodeVar105;

			} else {

				nodeVar98 = nodeVar107;

			}

			nodeVar109 = ( ( ( ( min( min( nodeVar95.x, nodeVar95.y ), nodeVar95.z ) > nodeVar97 ) && ( nodeVar97 > 0.0 ) ) && ( nodeVar86 > 0.15 ) ) && ( nodeVar97 < nodeVar98 ) );

			if ( nodeVar109 ) {

				nodeVar82 = nodeVar97;

			} else {

				nodeVar82 = nodeVar98;

			}

			nodeVar110 = ( ( ( ( min( min( nodeVar79.x, nodeVar79.y ), nodeVar79.z ) > nodeVar81 ) && ( nodeVar81 > 0.0 ) ) && ( nodeVar70 > 0.15 ) ) && ( nodeVar81 < nodeVar82 ) );

			if ( nodeVar110 ) {

				nodeVar66 = nodeVar81;

			} else {

				nodeVar66 = nodeVar82;

			}

			nodeVar111 = ( ( ( ( min( min( nodeVar63.x, nodeVar63.y ), nodeVar63.z ) > nodeVar65 ) && ( nodeVar65 > 0.0 ) ) && ( nodeVar54 > 0.15 ) ) && ( nodeVar65 < nodeVar66 ) );

			if ( nodeVar111 ) {

				nodeVar50 = nodeVar65;

			} else {

				nodeVar50 = nodeVar66;

			}

			nodeVar112 = ( ( ( min( min( nodeVar47.x, nodeVar47.y ), nodeVar47.z ) > nodeVar49 ) && ( nodeVar49 > 0.0 ) ) && ( nodeVar49 < nodeVar50 ) );

			if ( nodeVar112 ) {

				nodeVar37 = nodeVar49;

			} else {

				nodeVar37 = nodeVar50;

			}

			nodeVar113 = ( ( ( ( min( min( nodeVar31.x, nodeVar31.y ), nodeVar31.z ) > nodeVar33 ) && ( nodeVar33 > 0.0 ) ) && ( nodeVar36 > 0.5 ) ) && ( nodeVar33 < nodeVar37 ) );

			if ( nodeVar113 ) {

				nodeVar24 = nodeVar33;

			} else {

				nodeVar24 = nodeVar37;

			}

			nodeVar114 = ( ( ( ( min( min( nodeVar16.x, nodeVar16.y ), nodeVar16.z ) > nodeVar18 ) && ( nodeVar18 > 0.0 ) ) && ( ( ( 1.0 - nodeVar21 ) * step( 0.45, ( f32( ( ( nodeVar23 >> 22u ) ^ nodeVar23 ) ) * 2.3283064365386963e-10 ) ) ) > 0.5 ) ) && ( nodeVar18 < nodeVar24 ) );

			if ( nodeVar114 ) {

				nodeVar5 = ( mix( vec3<f32>( 0.0722718506743852, 0.08228270712149792, 0.09758734713304495 ), vec3<f32>( 0.00972121731707524, 0.010960094003125918, 0.012983032338510335 ), step( 0.62, fract( ( ( nodeVar11 + ( nodeVar13 * vec3<f32>( nodeVar18 ) ) ).y * 7.0 ) ) ) ) * vec3<f32>( 0.8 ) );

			} else {


				if ( nodeVar113 ) {

					nodeVar117 = ( nodeVar11 + ( nodeVar13 * vec3<f32>( nodeVar33 ) ) );
					nodeVar118 = ( ( nodeVar117.z - nodeVar25 ) / ( nodeVar28 - nodeVar25 ) );
					nodeVar119 = ( ( ( nodeVar4 + 91u ) + ( u32( floor( ( nodeVar118 * 8.0 ) ) ) * 197u ) ) + ( u32( floor( ( ( ( nodeVar117 - nodeVar26 ) / ( nodeVar29 - nodeVar26 ) ).y * 4.0 ) ) ) * 4099u ) );
					nodeVar120 = ( ( nodeVar119 * 747796405u ) + 2891336453u );
					nodeVar121 = ( ( ( nodeVar120 >> ( ( nodeVar120 >> 28u ) + 4u ) ) ^ nodeVar120 ) * 277803737u );
					nodeVar122 = ( ( f32( ( ( nodeVar121 >> 22u ) ^ nodeVar121 ) ) * 2.3283064365386963e-10 ) * 6.0 );

					if ( ( nodeVar122 > 5.0 ) ) {

						nodeVar116 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

					} else {


						if ( ( nodeVar122 > 4.0 ) ) {

							nodeVar123 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

						} else {


							if ( ( nodeVar122 > 3.0 ) ) {

								nodeVar124 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

							} else {


								if ( ( nodeVar122 > 2.0 ) ) {

									nodeVar125 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

								} else {


									if ( ( nodeVar122 > 1.0 ) ) {

										nodeVar126 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

									} else {

										nodeVar126 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

									}

									nodeVar125 = nodeVar126;

								}

								nodeVar124 = nodeVar125;

							}

							nodeVar123 = nodeVar124;

						}

						nodeVar116 = nodeVar123;

					}

					nodeVar127 = fract( ( ( ( nodeVar117 - nodeVar26 ) / ( nodeVar29 - nodeVar26 ) ).y * 4.0 ) );
					nodeVar128 = ( ( ( nodeVar119 + 1u ) * 747796405u ) + 2891336453u );
					nodeVar129 = ( ( ( nodeVar128 >> ( ( nodeVar128 >> 28u ) + 4u ) ) ^ nodeVar128 ) * 277803737u );
					nodeVar130 = ( f32( ( ( nodeVar129 >> 22u ) ^ nodeVar129 ) ) * 2.3283064365386963e-10 );
					nodeVar131 = ( ( ( nodeVar4 + 119831u ) * 747796405u ) + 2891336453u );
					nodeVar132 = ( ( ( nodeVar131 >> ( ( nodeVar131 >> 28u ) + 4u ) ) ^ nodeVar131 ) * 277803737u );
					nodeVar115 = ( mix( mix( vec3<f32>( 0.015208514418949472, 0.012983032338510335, 0.010960094003125918 ), nodeVar116, ( ( ( step( 0.08, nodeVar127 ) * step( nodeVar127, mix( 0.4, 0.85, fract( ( nodeVar130 * 5.0 ) ) ) ) ) * step( abs( ( fract( ( nodeVar118 * 8.0 ) ) - 0.5 ) ), mix( 0.26, 0.46, fract( ( nodeVar130 * 13.0 ) ) ) ) ) * step( 0.15, nodeVar130 ) ) ), mix( vec3<f32>( 0.14702726648767014, 0.10224173307914941, 0.05612849004241121 ), vec3<f32>( 0.623960391667596, 0.6038273388475408, 0.55201140150344 ), step( 0.5, ( f32( ( ( nodeVar132 >> 22u ) ^ nodeVar132 ) ) * 2.3283064365386963e-10 ) ) ), step( nodeVar127, 0.08 ) ) * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeVar21 ), clamp( ( ( nodeVar117.z - 0.1 ) / nodeVar6 ), 0.0, 1.0 ) ) ) );

				} else {


					if ( nodeVar112 ) {

						nodeVar134 = ( ( ( nodeVar4 + 119831u ) * 747796405u ) + 2891336453u );
						nodeVar135 = ( ( ( nodeVar134 >> ( ( nodeVar134 >> 28u ) + 4u ) ) ^ nodeVar134 ) * 277803737u );
						nodeVar137 = ( nodeVar11 + ( nodeVar13 * vec3<f32>( nodeVar49 ) ) );

						if ( ( ( ( nodeVar137 - nodeVar43 ) / ( nodeVar45 - nodeVar43 ) ).y > 0.92 ) ) {

							nodeVar136 = 1.3;

						} else {

							nodeVar136 = 0.85;

						}

						nodeVar133 = ( ( mix( vec3<f32>( 0.06847816983662762, 0.05126945836711539, 0.036889450395083165 ), vec3<f32>( 0.025186859622305935, 0.033104766565152086, 0.04231141061442144 ), ( f32( ( ( nodeVar135 >> 22u ) ^ nodeVar135 ) ) * 2.3283064365386963e-10 ) ) * vec3<f32>( nodeVar136 ) ) * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeVar21 ), clamp( ( ( nodeVar137.z - 0.1 ) / nodeVar6 ), 0.0, 1.0 ) ) ) );

					} else {


						if ( nodeVar111 ) {

							nodeVar140 = ( nodeVar11 + ( nodeVar13 * vec3<f32>( nodeVar65 ) ) );

							if ( ( ( ( nodeVar140 - nodeVar56 ) / ( nodeVar61 - nodeVar56 ) ).y > 0.6 ) ) {

								nodeVar142 = ( nodeVar60 * 6.0 );

								if ( ( nodeVar142 > 5.0 ) ) {

									nodeVar141 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

								} else {


									if ( ( nodeVar142 > 4.0 ) ) {

										nodeVar143 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

									} else {


										if ( ( nodeVar142 > 3.0 ) ) {

											nodeVar144 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

										} else {


											if ( ( nodeVar142 > 2.0 ) ) {

												nodeVar145 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

											} else {


												if ( ( nodeVar142 > 1.0 ) ) {

													nodeVar146 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

												} else {

													nodeVar146 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

												}

												nodeVar145 = nodeVar146;

											}

											nodeVar144 = nodeVar145;

										}

										nodeVar143 = nodeVar144;

									}

									nodeVar141 = nodeVar143;

								}

								nodeVar139 = nodeVar141;

							} else {

								nodeVar139 = vec3<f32>( 0.7011018919268015, 0.6653872982754769, 0.5972017883558645 );

							}

							nodeVar138 = ( nodeVar139 * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeVar21 ), clamp( ( ( nodeVar140.z - 0.1 ) / nodeVar6 ), 0.0, 1.0 ) ) ) );

						} else {


							if ( nodeVar110 ) {

								nodeVar149 = ( nodeVar11 + ( nodeVar13 * vec3<f32>( nodeVar81 ) ) );

								if ( ( ( ( nodeVar149 - nodeVar72 ) / ( nodeVar77 - nodeVar72 ) ).y > 0.6 ) ) {

									nodeVar151 = ( nodeVar76 * 6.0 );

									if ( ( nodeVar151 > 5.0 ) ) {

										nodeVar150 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

									} else {


										if ( ( nodeVar151 > 4.0 ) ) {

											nodeVar152 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

										} else {


											if ( ( nodeVar151 > 3.0 ) ) {

												nodeVar153 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

											} else {


												if ( ( nodeVar151 > 2.0 ) ) {

													nodeVar154 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

												} else {


													if ( ( nodeVar151 > 1.0 ) ) {

														nodeVar155 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

													} else {

														nodeVar155 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

													}

													nodeVar154 = nodeVar155;

												}

												nodeVar153 = nodeVar154;

											}

											nodeVar152 = nodeVar153;

										}

										nodeVar150 = nodeVar152;

									}

									nodeVar148 = nodeVar150;

								} else {

									nodeVar148 = vec3<f32>( 0.7011018919268015, 0.6653872982754769, 0.5972017883558645 );

								}

								nodeVar147 = ( nodeVar148 * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeVar21 ), clamp( ( ( nodeVar149.z - 0.1 ) / nodeVar6 ), 0.0, 1.0 ) ) ) );

							} else {


								if ( nodeVar109 ) {

									nodeVar158 = ( nodeVar11 + ( nodeVar13 * vec3<f32>( nodeVar97 ) ) );

									if ( ( ( ( nodeVar158 - nodeVar88 ) / ( nodeVar93 - nodeVar88 ) ).y > 0.6 ) ) {

										nodeVar160 = ( nodeVar92 * 6.0 );

										if ( ( nodeVar160 > 5.0 ) ) {

											nodeVar159 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

										} else {


											if ( ( nodeVar160 > 4.0 ) ) {

												nodeVar161 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

											} else {


												if ( ( nodeVar160 > 3.0 ) ) {

													nodeVar162 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

												} else {


													if ( ( nodeVar160 > 2.0 ) ) {

														nodeVar163 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

													} else {


														if ( ( nodeVar160 > 1.0 ) ) {

															nodeVar164 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

														} else {

															nodeVar164 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

														}

														nodeVar163 = nodeVar164;

													}

													nodeVar162 = nodeVar163;

												}

												nodeVar161 = nodeVar162;

											}

											nodeVar159 = nodeVar161;

										}

										nodeVar157 = nodeVar159;

									} else {

										nodeVar157 = vec3<f32>( 0.7011018919268015, 0.6653872982754769, 0.5972017883558645 );

									}

									nodeVar156 = ( nodeVar157 * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeVar21 ), clamp( ( ( nodeVar158.z - 0.1 ) / nodeVar6 ), 0.0, 1.0 ) ) ) );

								} else {


									if ( nodeVar108 ) {

										nodeVar167 = ( nodeVar11 + ( nodeVar13 * vec3<f32>( nodeVar105 ) ) );

										if ( ( ( ( nodeVar167 - nodeVar99 ) / ( nodeVar101 - nodeVar99 ) ).y > 0.93 ) ) {

											nodeVar166 = vec3<f32>( 0.6938717612856897, 0.6583748172725346, 0.5775804404214573 );

										} else {

											nodeVar166 = vec3<f32>( 0.035601314869097636, 0.02955683443236377, 0.023153366173251363 );

										}

										nodeVar165 = ( nodeVar166 * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeVar21 ), clamp( ( ( nodeVar167.z - 0.1 ) / nodeVar6 ), 0.0, 1.0 ) ) ) );

									} else {

										nodeVar169 = ( nodeVar11 + ( nodeVar13 * vec3<f32>( nodeVar107 ) ) );
										nodeVar170 = ( ( nodeVar169 - nodeVar8 ) / ( nodeVar7 - nodeVar8 ) );
										nodeVar171 = ( nodeVar170.z > 0.998 );

										if ( nodeVar171 ) {


											if ( bool( nodeVar36 ) ) {

												nodeVar174 = ( nodeVarying11.x * 2.5 );
												nodeVar175 = ( ( ( nodeVar4 + 31u ) + ( u32( floor( ( nodeVar170.x * nodeVar174 ) ) ) * 197u ) ) + ( u32( floor( ( nodeVar170.y * 4.0 ) ) ) * 4099u ) );
												nodeVar176 = ( ( nodeVar175 * 747796405u ) + 2891336453u );
												nodeVar177 = ( ( ( nodeVar176 >> ( ( nodeVar176 >> 28u ) + 4u ) ) ^ nodeVar176 ) * 277803737u );
												nodeVar178 = ( ( f32( ( ( nodeVar177 >> 22u ) ^ nodeVar177 ) ) * 2.3283064365386963e-10 ) * 6.0 );

												if ( ( nodeVar178 > 5.0 ) ) {

													nodeVar173 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

												} else {


													if ( ( nodeVar178 > 4.0 ) ) {

														nodeVar179 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

													} else {


														if ( ( nodeVar178 > 3.0 ) ) {

															nodeVar180 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

														} else {


															if ( ( nodeVar178 > 2.0 ) ) {

																nodeVar181 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

															} else {


																if ( ( nodeVar178 > 1.0 ) ) {

																	nodeVar182 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

																} else {

																	nodeVar182 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

																}

																nodeVar181 = nodeVar182;

															}

															nodeVar180 = nodeVar181;

														}

														nodeVar179 = nodeVar180;

													}

													nodeVar173 = nodeVar179;

												}

												nodeVar183 = fract( ( nodeVar170.y * 4.0 ) );
												nodeVar184 = ( ( ( nodeVar175 + 1u ) * 747796405u ) + 2891336453u );
												nodeVar185 = ( ( ( nodeVar184 >> ( ( nodeVar184 >> 28u ) + 4u ) ) ^ nodeVar184 ) * 277803737u );
												nodeVar186 = ( f32( ( ( nodeVar185 >> 22u ) ^ nodeVar185 ) ) * 2.3283064365386963e-10 );
												nodeVar187 = ( ( ( nodeVar4 + 119831u ) * 747796405u ) + 2891336453u );
												nodeVar188 = ( ( ( nodeVar187 >> ( ( nodeVar187 >> 28u ) + 4u ) ) ^ nodeVar187 ) * 277803737u );
												nodeVar172 = mix( mix( vec3<f32>( 0.015208514418949472, 0.012983032338510335, 0.010960094003125918 ), nodeVar173, ( ( ( step( 0.08, nodeVar183 ) * step( nodeVar183, mix( 0.4, 0.85, fract( ( nodeVar186 * 5.0 ) ) ) ) ) * step( abs( ( fract( ( nodeVar170.x * nodeVar174 ) ) - 0.5 ) ), mix( 0.26, 0.46, fract( ( nodeVar186 * 13.0 ) ) ) ) ) * step( 0.15, nodeVar186 ) ) ), mix( vec3<f32>( 0.14702726648767014, 0.10224173307914941, 0.05612849004241121 ), vec3<f32>( 0.623960391667596, 0.6038273388475408, 0.55201140150344 ), step( 0.5, ( f32( ( ( nodeVar188 >> 22u ) ^ nodeVar188 ) ) * 2.3283064365386963e-10 ) ) ), step( nodeVar183, 0.08 ) );

											} else {

												nodeVar190 = ( ( ( nodeVar4 + 119831u ) * 747796405u ) + 2891336453u );
												nodeVar191 = ( ( ( nodeVar190 >> ( ( nodeVar190 >> 28u ) + 4u ) ) ^ nodeVar190 ) * 277803737u );
												nodeVar192 = ( ( f32( ( ( nodeVar191 >> 22u ) ^ nodeVar191 ) ) * 2.3283064365386963e-10 ) * 6.0 );

												if ( ( nodeVar192 > 5.0 ) ) {

													nodeVar189 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

												} else {


													if ( ( nodeVar192 > 4.0 ) ) {

														nodeVar193 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

													} else {


														if ( ( nodeVar192 > 3.0 ) ) {

															nodeVar194 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

														} else {


															if ( ( nodeVar192 > 2.0 ) ) {

																nodeVar195 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

															} else {


																if ( ( nodeVar192 > 1.0 ) ) {

																	nodeVar196 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

																} else {

																	nodeVar196 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

																}

																nodeVar195 = nodeVar196;

															}

															nodeVar194 = nodeVar195;

														}

														nodeVar193 = nodeVar194;

													}

													nodeVar189 = nodeVar193;

												}

												nodeVar197 = mix( 0.3, 0.7, nodeVar40 );
												nodeVar199 = ( ( ( nodeVar4 + 41260u ) * 747796405u ) + 2891336453u );
												nodeVar200 = ( ( ( nodeVar199 >> ( ( nodeVar199 >> 28u ) + 4u ) ) ^ nodeVar199 ) * 277803737u );
												nodeVar201 = ( ( f32( ( ( nodeVar200 >> 22u ) ^ nodeVar200 ) ) * 2.3283064365386963e-10 ) * 6.0 );

												if ( ( nodeVar201 > 5.0 ) ) {

													nodeVar198 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

												} else {


													if ( ( nodeVar201 > 4.0 ) ) {

														nodeVar202 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

													} else {


														if ( ( nodeVar201 > 3.0 ) ) {

															nodeVar203 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

														} else {


															if ( ( nodeVar201 > 2.0 ) ) {

																nodeVar204 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

															} else {


																if ( ( nodeVar201 > 1.0 ) ) {

																	nodeVar205 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

																} else {

																	nodeVar205 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

																}

																nodeVar204 = nodeVar205;

															}

															nodeVar203 = nodeVar204;

														}

														nodeVar202 = nodeVar203;

													}

													nodeVar198 = nodeVar202;

												}

												nodeVar172 = mix( mix( mix( nodeVar189, mix( vec3<f32>( 0.6866853124288864, 0.6444796819634361, 0.5647115056965487 ), vec3<f32>( 0.5209955731953768, 0.48514994004665124, 0.4232676699760063 ), nodeVar40 ), 0.45 ), vec3<f32>( 0.8069522576650873, 0.775822218312646, 0.7011018919268015 ), ( smoothstep( 0.10600000000000001, 0.094, abs( ( nodeVar170.x - nodeVar197 ) ) ) * smoothstep( 0.14600000000000002, 0.134, abs( ( nodeVar170.y - 0.62 ) ) ) ) ), nodeVar198, ( ( smoothstep( 0.081, 0.06899999999999999, abs( ( nodeVar170.x - nodeVar197 ) ) ) * smoothstep( 0.10600000000000001, 0.094, abs( ( nodeVar170.y - 0.62 ) ) ) ) * 0.85 ) );

											}

											nodeVar168 = nodeVar172;

										} else {


											if ( ( nodeVar170.y > 0.998 ) ) {

												nodeVar208 = ( ( ( nodeVar4 + 25910u ) * 747796405u ) + 2891336453u );
												nodeVar209 = ( ( ( nodeVar208 >> ( ( nodeVar208 >> 28u ) + 4u ) ) ^ nodeVar208 ) * 277803737u );

												if ( ( ( f32( ( ( nodeVar209 >> 22u ) ^ nodeVar209 ) ) * 2.3283064365386963e-10 ) > 0.65 ) ) {

													nodeVar210 = ( ( ( nodeVar4 + 86350u ) * 747796405u ) + 2891336453u );
													nodeVar211 = ( ( ( nodeVar210 >> ( ( nodeVar210 >> 28u ) + 4u ) ) ^ nodeVar210 ) * 277803737u );
													nodeVar207 = mix( vec3<f32>( 1.0, 0.6938717612856897, 0.3515325994898463 ), vec3<f32>( 1.0, 0.8227857543924378, 0.5972017883558645 ), ( f32( ( ( nodeVar211 >> 22u ) ^ nodeVar211 ) ) * 2.3283064365386963e-10 ) );

												} else {

													nodeVar212 = ( ( ( nodeVar4 + 59550u ) * 747796405u ) + 2891336453u );
													nodeVar213 = ( ( ( nodeVar212 >> ( ( nodeVar212 >> 28u ) + 4u ) ) ^ nodeVar212 ) * 277803737u );
													nodeVar207 = mix( vec3<f32>( 0.85499260812105, 0.8879231178794776, 0.9215818562755338 ), vec3<f32>( 0.7379104087672317, 0.8069522576650873, 1.0 ), ( f32( ( ( nodeVar213 >> 22u ) ^ nodeVar213 ) ) * 2.3283064365386963e-10 ) );

												}

												nodeVar206 = mix( vec3<f32>( 0.8148465722120952, 0.7912979403281551, 0.7379104087672317 ), ( nodeVar207 * vec3<f32>( mix( 0.5, 6.0, nodeVar21 ) ) ), ( step( abs( ( fract( ( nodeVar170.x * 3.0 ) ) - 0.5 ) ), 0.3 ) * step( abs( ( fract( ( nodeVar170.z * 4.0 ) ) - 0.5 ) ), 0.15 ) ) );

											} else {


												if ( ( nodeVar170.y < 0.002 ) ) {

													nodeVar216 = ( ( ( nodeVar4 + 27510u ) * 747796405u ) + 2891336453u );
													nodeVar217 = ( ( ( nodeVar216 >> ( ( nodeVar216 >> 28u ) + 4u ) ) ^ nodeVar216 ) * 277803737u );

													if ( ( ( f32( ( ( nodeVar217 >> 22u ) ^ nodeVar217 ) ) * 2.3283064365386963e-10 ) > 0.75 ) ) {

														nodeVar215 = mix( vec3<f32>( 0.01764195448412081, 0.019382360952473074, 0.023153366173251363 ), vec3<f32>( 0.623960391667596, 0.6038273388475408, 0.5394794890033748 ), tsl_mod_float( ( floor( ( nodeVar170.x * 8.0 ) ) + floor( ( nodeVar170.z * 10.0 ) ) ), 2.0 ) );

													} else {

														nodeVar218 = ( ( ( nodeVar4 + 119831u ) * 747796405u ) + 2891336453u );
														nodeVar219 = ( ( ( nodeVar218 >> ( ( nodeVar218 >> 28u ) + 4u ) ) ^ nodeVar218 ) * 277803737u );
														nodeVar215 = ( mix( vec3<f32>( 0.46207699964472876, 0.4286904966038916, 0.36130677977297226 ), vec3<f32>( 0.3005437944049895, 0.2746773120495699, 0.22696587349938613 ), ( f32( ( ( nodeVar219 >> 22u ) ^ nodeVar219 ) ) * 2.3283064365386963e-10 ) ) * vec3<f32>( ( 1.0 - ( max( step( 0.93, fract( ( nodeVar170.x * 8.0 ) ) ), step( 0.93, fract( ( nodeVar170.z * 10.0 ) ) ) ) * 0.35 ) ) ) );

													}

													nodeVar214 = nodeVar215;

												} else {


													if ( bool( nodeVar36 ) ) {

														nodeVar222 = ( nodeVar6 * 2.5 );
														nodeVar223 = ( ( ( nodeVar4 + 57u ) + ( u32( floor( ( nodeVar170.z * nodeVar222 ) ) ) * 197u ) ) + ( u32( floor( ( nodeVar170.y * 4.0 ) ) ) * 4099u ) );
														nodeVar224 = ( ( nodeVar223 * 747796405u ) + 2891336453u );
														nodeVar225 = ( ( ( nodeVar224 >> ( ( nodeVar224 >> 28u ) + 4u ) ) ^ nodeVar224 ) * 277803737u );
														nodeVar226 = ( ( f32( ( ( nodeVar225 >> 22u ) ^ nodeVar225 ) ) * 2.3283064365386963e-10 ) * 6.0 );

														if ( ( nodeVar226 > 5.0 ) ) {

															nodeVar221 = vec3<f32>( 0.024157632443547246, 0.02121901037134225, 0.02955683443236377 );

														} else {


															if ( ( nodeVar226 > 4.0 ) ) {

																nodeVar227 = vec3<f32>( 0.6724431569510133, 0.6307571363387763, 0.55201140150344 );

															} else {


																if ( ( nodeVar226 > 3.0 ) ) {

																	nodeVar228 = vec3<f32>( 0.035601314869097636, 0.09530746662221588, 0.2541520943200296 );

																} else {


																	if ( ( nodeVar226 > 2.0 ) ) {

																		nodeVar229 = vec3<f32>( 0.04666508633021928, 0.14702726648767014, 0.07818742179702069 );

																	} else {


																		if ( ( nodeVar226 > 1.0 ) ) {

																			nodeVar230 = vec3<f32>( 0.4452011945063733, 0.21586050010324417, 0.028426039499072558 );

																		} else {

																			nodeVar230 = vec3<f32>( 0.34191442489801843, 0.04666508633021928, 0.033104766565152086 );

																		}

																		nodeVar229 = nodeVar230;

																	}

																	nodeVar228 = nodeVar229;

																}

																nodeVar227 = nodeVar228;

															}

															nodeVar221 = nodeVar227;

														}

														nodeVar231 = fract( ( nodeVar170.y * 4.0 ) );
														nodeVar232 = ( ( ( nodeVar223 + 1u ) * 747796405u ) + 2891336453u );
														nodeVar233 = ( ( ( nodeVar232 >> ( ( nodeVar232 >> 28u ) + 4u ) ) ^ nodeVar232 ) * 277803737u );
														nodeVar234 = ( f32( ( ( nodeVar233 >> 22u ) ^ nodeVar233 ) ) * 2.3283064365386963e-10 );
														nodeVar235 = ( ( ( nodeVar4 + 119831u ) * 747796405u ) + 2891336453u );
														nodeVar236 = ( ( ( nodeVar235 >> ( ( nodeVar235 >> 28u ) + 4u ) ) ^ nodeVar235 ) * 277803737u );
														nodeVar220 = mix( mix( vec3<f32>( 0.015208514418949472, 0.012983032338510335, 0.010960094003125918 ), nodeVar221, ( ( ( step( 0.08, nodeVar231 ) * step( nodeVar231, mix( 0.4, 0.85, fract( ( nodeVar234 * 5.0 ) ) ) ) ) * step( abs( ( fract( ( nodeVar170.z * nodeVar222 ) ) - 0.5 ) ), mix( 0.26, 0.46, fract( ( nodeVar234 * 13.0 ) ) ) ) ) * step( 0.15, nodeVar234 ) ) ), mix( vec3<f32>( 0.14702726648767014, 0.10224173307914941, 0.05612849004241121 ), vec3<f32>( 0.623960391667596, 0.6038273388475408, 0.55201140150344 ), step( 0.5, ( f32( ( ( nodeVar236 >> 22u ) ^ nodeVar236 ) ) * 2.3283064365386963e-10 ) ) ), step( nodeVar231, 0.08 ) );

													} else {

														nodeVar220 = mix( vec3<f32>( 0.6866853124288864, 0.6444796819634361, 0.5647115056965487 ), vec3<f32>( 0.5209955731953768, 0.48514994004665124, 0.4232676699760063 ), nodeVar40 );

													}

													nodeVar214 = nodeVar220;

												}

												nodeVar206 = nodeVar214;

											}

											nodeVar168 = nodeVar206;

										}


										if ( nodeVar171 ) {

											nodeVar237 = ( ( smoothstep( 0.0, 0.15, nodeVar170.x ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar170.x ) ) ) * ( smoothstep( 0.0, 0.15, nodeVar170.y ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar170.y ) ) ) );

										} else {


											if ( ( ( nodeVar170.y < 0.002 ) || ( nodeVar170.y > 0.998 ) ) ) {

												nodeVar238 = ( ( smoothstep( 0.0, 0.15, nodeVar170.x ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar170.x ) ) ) * ( smoothstep( 0.0, 0.15, nodeVar170.z ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar170.z ) ) ) );

											} else {

												nodeVar238 = ( ( smoothstep( 0.0, 0.15, nodeVar170.y ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar170.y ) ) ) * ( smoothstep( 0.0, 0.15, nodeVar170.z ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar170.z ) ) ) );

											}

											nodeVar237 = nodeVar238;

										}

										nodeVar165 = ( ( nodeVar168 * vec3<f32>( mix( 0.72, 1.0, nodeVar237 ) ) ) * vec3<f32>( mix( 1.0, mix( 0.25, 0.6, nodeVar21 ), clamp( ( ( nodeVar169.z - 0.1 ) / nodeVar6 ), 0.0, 1.0 ) ) ) );

									}

									nodeVar156 = nodeVar165;

								}

								nodeVar147 = nodeVar156;

							}

							nodeVar138 = nodeVar147;

						}

						nodeVar133 = nodeVar138;

					}

					nodeVar115 = nodeVar133;

				}

				nodeVar5 = nodeVar115;

			}

			nodeVar240 = ( ( ( nodeVar4 + 25910u ) * 747796405u ) + 2891336453u );
			nodeVar241 = ( ( ( nodeVar240 >> ( ( nodeVar240 >> 28u ) + 4u ) ) ^ nodeVar240 ) * 277803737u );

			if ( ( ( f32( ( ( nodeVar241 >> 22u ) ^ nodeVar241 ) ) * 2.3283064365386963e-10 ) > 0.65 ) ) {

				nodeVar242 = ( ( ( nodeVar4 + 86350u ) * 747796405u ) + 2891336453u );
				nodeVar243 = ( ( ( nodeVar242 >> ( ( nodeVar242 >> 28u ) + 4u ) ) ^ nodeVar242 ) * 277803737u );
				nodeVar239 = mix( vec3<f32>( 1.0, 0.6938717612856897, 0.3515325994898463 ), vec3<f32>( 1.0, 0.8227857543924378, 0.5972017883558645 ), ( f32( ( ( nodeVar243 >> 22u ) ^ nodeVar243 ) ) * 2.3283064365386963e-10 ) );

			} else {

				nodeVar244 = ( ( ( nodeVar4 + 59550u ) * 747796405u ) + 2891336453u );
				nodeVar245 = ( ( ( nodeVar244 >> ( ( nodeVar244 >> 28u ) + 4u ) ) ^ nodeVar244 ) * 277803737u );
				nodeVar239 = mix( vec3<f32>( 0.85499260812105, 0.8879231178794776, 0.9215818562755338 ), vec3<f32>( 0.7379104087672317, 0.8069522576650873, 1.0 ), ( f32( ( ( nodeVar245 >> 22u ) ^ nodeVar245 ) ) * 2.3283064365386963e-10 ) );

			}


			if ( nodeVar114 ) {

				nodeVar246 = 0.0;

			} else {


				if ( nodeVar113 ) {

					nodeVar247 = 1.0;

				} else {


					if ( nodeVar112 ) {

						nodeVar248 = 1.0;

					} else {


						if ( nodeVar111 ) {

							nodeVar249 = 1.0;

						} else {


							if ( nodeVar110 ) {

								nodeVar250 = 1.0;

							} else {


								if ( nodeVar109 ) {

									nodeVar251 = 1.0;

								} else {


									if ( nodeVar108 ) {

										nodeVar252 = 1.0;

									} else {

										nodeVar252 = 1.0;

									}

									nodeVar251 = nodeVar252;

								}

								nodeVar250 = nodeVar251;

							}

							nodeVar249 = nodeVar250;

						}

						nodeVar248 = nodeVar249;

					}

					nodeVar247 = nodeVar248;

				}

				nodeVar246 = nodeVar247;

			}

			nodeVar2 = vec4<f32>( ( ( nodeVar5 * mix( vec3<f32>( 1.0, 1.0, 1.0 ), nodeVar239, ( nodeVar21 * 0.6 ) ) ) * vec3<f32>( mix( 0.45, 1.5, nodeVar21 ) ) ), ( nodeVar21 * nodeVar246 ) );

		} else {

			nodeVar253 = floor( ( nodeVarying5 * vec3<f32>( 2.0 ) ) );
			nodeVar254 = ( ( ( u32( ( nodeVar253.x + 2097152.0 ) ) * 73856093u ) ^ ( u32( ( nodeVar253.y + 2097152.0 ) ) * 19349663u ) ) ^ ( u32( ( nodeVar253.z + 2097152.0 ) ) * 83492791u ) );
			nodeVar256 = vec3<f32>( ( nodeVarying11.x * 0.5 ), ( nodeVarying11.y * 0.5 ), ( 0.1 + ( nodeVarying11.y * 1.55 ) ) );
			nodeVar257 = ( ( ( nodeVar254 + 119831u ) * 747796405u ) + 2891336453u );
			nodeVar258 = ( ( ( nodeVar257 >> ( ( nodeVar257 >> 28u ) + 4u ) ) ^ nodeVar257 ) * 277803737u );
			nodeVar259 = ( f32( ( ( nodeVar258 >> 22u ) ^ nodeVar258 ) ) * 2.3283064365386963e-10 );
			nodeVar260 = smoothstep( 0.3, 1.0, nodeVar259 );
			nodeVar261 = ( nodeVar256.x * ( nodeVar260 * nodeVar260 ) );
			nodeVar262 = vec3<f32>( ( - nodeVar256.x ), ( - nodeVar256.y ), 0.1 );
			nodeVar263 = ( positionLocal - nodeVarying5 );
			normalLocal = nodeVarying12;
			nodeVar264 = normalize( cross( vec3<f32>( 0.0, 1.0, 0.0 ), normalLocal ) );
			nodeVar265 = vec3<f32>( dot( nodeVar263, nodeVar264 ), nodeVar263.y, 0.0 );
			nodeVar266 = normalize( ( positionLocal - ( object.nodeUniform0 * vec4<f32>( render.cameraPosition, 1.0 ) ).xyz ) );
			normalLocal = nodeVarying12;
			nodeVar267 = vec3<f32>( dot( nodeVar266, nodeVar264 ), nodeVar266.y, ( - dot( nodeVar266, normalLocal ) ) );
			nodeVar268 = ( ( vec3<f32>( ( nodeVar256.x - nodeVar261 ), nodeVar262.y, 0.1 ) - nodeVar265 ) / nodeVar267 );
			nodeVar269 = ( 0.1 + 0.12 );
			nodeVar270 = ( ( vec3<f32>( nodeVar256.x, nodeVar256.y, nodeVar269 ) - nodeVar265 ) / nodeVar267 );
			nodeVar271 = max( nodeVar268, nodeVar270 );
			nodeVar272 = min( nodeVar268, nodeVar270 );
			nodeVar273 = max( max( nodeVar272.x, nodeVar272.y ), nodeVar272.z );
			nodeVar275 = ( ( vec3<f32>( ( - nodeVar256.x ), nodeVar262.y, 0.1 ) - nodeVar265 ) / nodeVar267 );
			nodeVar276 = ( ( ( nodeVar254 + 105097u ) * 747796405u ) + 2891336453u );
			nodeVar277 = ( ( ( nodeVar276 >> ( ( nodeVar276 >> 28u ) + 4u ) ) ^ nodeVar276 ) * 277803737u );
			nodeVar278 = ( f32( ( ( nodeVar277 >> 22u ) ^ nodeVar277 ) ) * 2.3283064365386963e-10 );
			nodeVar279 = smoothstep( 0.3, 1.0, nodeVar278 );
			nodeVar280 = ( nodeVar256.x * ( nodeVar279 * nodeVar279 ) );
			nodeVar281 = ( ( vec3<f32>( ( ( - nodeVar256.x ) + nodeVar280 ), nodeVar256.y, nodeVar269 ) - nodeVar265 ) / nodeVar267 );
			nodeVar282 = max( nodeVar275, nodeVar281 );
			nodeVar283 = min( nodeVar275, nodeVar281 );
			nodeVar284 = max( max( nodeVar283.x, nodeVar283.y ), nodeVar283.z );
			nodeVar286 = ( nodeVar256.x * 0.82 );
			nodeVar287 = vec3<f32>( ( nodeVar286 - 0.5 ), nodeVar262.y, ( nodeVar256.z - 0.7 ) );
			nodeVar288 = ( ( nodeVar287 - nodeVar265 ) / nodeVar267 );
			nodeVar289 = vec3<f32>( ( nodeVar286 + 0.5 ), ( nodeVar262.y + mix( 1.7, 2.3, nodeVar259 ) ), ( nodeVar256.z - 0.1 ) );
			nodeVar290 = ( ( nodeVar289 - nodeVar265 ) / nodeVar267 );
			nodeVar291 = max( nodeVar288, nodeVar290 );
			nodeVar292 = min( nodeVar288, nodeVar290 );
			nodeVar293 = max( max( nodeVar292.x, nodeVar292.y ), nodeVar292.z );
			nodeVar294 = ( ( ( nodeVar254 + 8200u ) * 747796405u ) + 2891336453u );
			nodeVar295 = ( ( ( nodeVar294 >> ( ( nodeVar294 >> 28u ) + 4u ) ) ^ nodeVar294 ) * 277803737u );
			nodeVar297 = ( nodeVar256.x * -0.82 );
			nodeVar298 = vec3<f32>( ( nodeVar297 - 0.5 ), nodeVar262.y, ( nodeVar256.z - 0.7 ) );
			nodeVar299 = ( ( nodeVar298 - nodeVar265 ) / nodeVar267 );
			nodeVar300 = vec3<f32>( ( nodeVar297 + 0.5 ), ( nodeVar262.y + mix( 1.7, 2.3, nodeVar278 ) ), ( nodeVar256.z - 0.1 ) );
			nodeVar301 = ( ( nodeVar300 - nodeVar265 ) / nodeVar267 );
			nodeVar302 = max( nodeVar299, nodeVar301 );
			nodeVar303 = min( nodeVar299, nodeVar301 );
			nodeVar304 = max( max( nodeVar303.x, nodeVar303.y ), nodeVar303.z );
			nodeVar305 = ( ( ( nodeVar254 + 15070u ) * 747796405u ) + 2891336453u );
			nodeVar306 = ( ( ( nodeVar305 >> ( ( nodeVar305 >> 28u ) + 4u ) ) ^ nodeVar305 ) * 277803737u );
			nodeVar308 = mix( ( nodeVar256.x * -0.3 ), ( nodeVar256.x * 0.3 ), nodeVar259 );
			nodeVar309 = vec3<f32>( ( nodeVar308 - 1.1 ), nodeVar262.y, ( nodeVar256.z - 0.95 ) );
			nodeVar310 = ( ( nodeVar309 - nodeVar265 ) / nodeVar267 );
			nodeVar311 = vec3<f32>( ( nodeVar308 + 1.1 ), ( nodeVar262.y + mix( 0.8, 0.9, nodeVar278 ) ), ( nodeVar256.z - 0.1 ) );
			nodeVar312 = ( ( nodeVar311 - nodeVar265 ) / nodeVar267 );
			nodeVar313 = max( nodeVar310, nodeVar312 );
			nodeVar314 = min( nodeVar310, nodeVar312 );
			nodeVar315 = max( max( nodeVar314.x, nodeVar314.y ), nodeVar314.z );
			nodeVar317 = mix( -0.6, 0.6, nodeVar278 );
			nodeVar318 = ( nodeVarying11.y * 1.55 );
			nodeVar319 = ( ( 0.1 + ( nodeVar318 * 0.5 ) ) + mix( -0.4, 0.5, nodeVar259 ) );
			nodeVar320 = vec3<f32>( ( nodeVar317 - 0.6 ), nodeVar262.y, ( nodeVar319 - 0.35 ) );
			nodeVar321 = ( ( nodeVar320 - nodeVar265 ) / nodeVar267 );
			nodeVar322 = vec3<f32>( ( nodeVar317 + 0.6 ), ( nodeVar262.y + 0.42 ), ( nodeVar319 + 0.35 ) );
			nodeVar323 = ( ( nodeVar322 - nodeVar265 ) / nodeVar267 );
			nodeVar324 = max( nodeVar321, nodeVar323 );
			nodeVar325 = min( nodeVar321, nodeVar323 );
			nodeVar326 = max( max( nodeVar325.x, nodeVar325.y ), nodeVar325.z );
			nodeVar327 = max( ( ( nodeVar262 - nodeVar265 ) / nodeVar267 ), ( ( nodeVar256 - nodeVar265 ) / nodeVar267 ) );
			nodeVar328 = min( min( nodeVar327.x, nodeVar327.y ), nodeVar327.z );
			nodeVar329 = ( ( ( min( min( nodeVar324.x, nodeVar324.y ), nodeVar324.z ) > nodeVar326 ) && ( nodeVar326 > 0.0 ) ) && ( nodeVar326 < nodeVar328 ) );

			if ( nodeVar329 ) {

				nodeVar316 = nodeVar326;

			} else {

				nodeVar316 = nodeVar328;

			}

			nodeVar330 = ( ( ( min( min( nodeVar313.x, nodeVar313.y ), nodeVar313.z ) > nodeVar315 ) && ( nodeVar315 > 0.0 ) ) && ( nodeVar315 < nodeVar316 ) );

			if ( nodeVar330 ) {

				nodeVar307 = nodeVar315;

			} else {

				nodeVar307 = nodeVar316;

			}

			nodeVar331 = ( ( ( ( min( min( nodeVar302.x, nodeVar302.y ), nodeVar302.z ) > nodeVar304 ) && ( nodeVar304 > 0.0 ) ) && ( ( f32( ( ( nodeVar306 >> 22u ) ^ nodeVar306 ) ) * 2.3283064365386963e-10 ) > 0.4 ) ) && ( nodeVar304 < nodeVar307 ) );

			if ( nodeVar331 ) {

				nodeVar296 = nodeVar304;

			} else {

				nodeVar296 = nodeVar307;

			}

			nodeVar332 = ( ( ( ( min( min( nodeVar291.x, nodeVar291.y ), nodeVar291.z ) > nodeVar293 ) && ( nodeVar293 > 0.0 ) ) && ( ( f32( ( ( nodeVar295 >> 22u ) ^ nodeVar295 ) ) * 2.3283064365386963e-10 ) > 0.4 ) ) && ( nodeVar293 < nodeVar296 ) );

			if ( nodeVar332 ) {

				nodeVar285 = nodeVar293;

			} else {

				nodeVar285 = nodeVar296;

			}

			nodeVar333 = ( ( ( ( min( min( nodeVar282.x, nodeVar282.y ), nodeVar282.z ) > nodeVar284 ) && ( nodeVar284 > 0.0 ) ) && ( nodeVar280 > 0.05 ) ) && ( nodeVar284 < nodeVar285 ) );

			if ( nodeVar333 ) {

				nodeVar274 = nodeVar284;

			} else {

				nodeVar274 = nodeVar285;

			}

			nodeVar334 = ( ( ( ( min( min( nodeVar271.x, nodeVar271.y ), nodeVar271.z ) > nodeVar273 ) && ( nodeVar273 > 0.0 ) ) && ( nodeVar261 > 0.05 ) ) && ( nodeVar273 < nodeVar274 ) );

			if ( nodeVar334 ) {

				nodeVar336 = ( ( ( nodeVar254 + 125490u ) * 747796405u ) + 2891336453u );
				nodeVar337 = ( ( ( nodeVar336 >> ( ( nodeVar336 >> 28u ) + 4u ) ) ^ nodeVar336 ) * 277803737u );
				nodeVar338 = ( ( f32( ( ( nodeVar337 >> 22u ) ^ nodeVar337 ) ) * 2.3283064365386963e-10 ) * 6.0 );

				if ( ( nodeVar338 > 5.0 ) ) {

					nodeVar335 = mix( vec3<f32>( 0.26225065751888765, 0.10224173307914941, 0.057805430183792694 ), vec3<f32>( 0.3231432091022285, 0.14412847084818123, 0.08437621153575764 ), nodeVar259 );

				} else {


					if ( ( nodeVar338 > 4.0 ) ) {

						nodeVar339 = mix( vec3<f32>( 0.1499597898006365, 0.17788841597328695, 0.09758734713304495 ), vec3<f32>( 0.1912016827303171, 0.22696587349938613, 0.11443537381770343 ), nodeVar259 );

					} else {


						if ( ( nodeVar338 > 3.0 ) ) {

							nodeVar340 = mix( vec3<f32>( 0.11443537381770343, 0.16202937562896222, 0.1912016827303171 ), vec3<f32>( 0.158960835050774, 0.21952619971859377, 0.25818285291079235 ), nodeVar259 );

						} else {


							if ( ( nodeVar338 > 2.0 ) ) {

								nodeVar341 = mix( vec3<f32>( 0.16202937562896222, 0.14412847084818123, 0.12743768042608497 ), vec3<f32>( 0.22696587349938613, 0.20507873637973145, 0.181164244239483 ), nodeVar259 );

							} else {


								if ( ( nodeVar338 > 1.0 ) ) {

									nodeVar342 = mix( vec3<f32>( 0.2541520943200296, 0.19461783043107173, 0.12743768042608497 ), vec3<f32>( 0.3277780980458375, 0.26225065751888765, 0.16826940017946088 ), nodeVar259 );

								} else {

									nodeVar342 = mix( vec3<f32>( 0.5906188409113381, 0.5209955731953768, 0.3813260114221238 ), vec3<f32>( 0.6866853124288864, 0.6104955708001716, 0.4793201830913402 ), nodeVar259 );

								}

								nodeVar341 = nodeVar342;

							}

							nodeVar340 = nodeVar341;

						}

						nodeVar339 = nodeVar340;

					}

					nodeVar335 = nodeVar339;

				}

				nodeVar343 = ( nodeVar265 + ( nodeVar267 * vec3<f32>( nodeVar273 ) ) );
				nodeVar255 = ( ( nodeVar335 * vec3<f32>( mix( 0.78, 1.12, fract( ( nodeVar343.x * 2.5 ) ) ) ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeVar343.z - 0.1 ) / nodeVar318 ), 0.0, 1.0 ) ) ) );

			} else {


				if ( nodeVar333 ) {

					nodeVar346 = ( ( ( nodeVar254 + 125490u ) * 747796405u ) + 2891336453u );
					nodeVar347 = ( ( ( nodeVar346 >> ( ( nodeVar346 >> 28u ) + 4u ) ) ^ nodeVar346 ) * 277803737u );
					nodeVar348 = ( ( f32( ( ( nodeVar347 >> 22u ) ^ nodeVar347 ) ) * 2.3283064365386963e-10 ) * 6.0 );

					if ( ( nodeVar348 > 5.0 ) ) {

						nodeVar345 = mix( vec3<f32>( 0.26225065751888765, 0.10224173307914941, 0.057805430183792694 ), vec3<f32>( 0.3231432091022285, 0.14412847084818123, 0.08437621153575764 ), nodeVar259 );

					} else {


						if ( ( nodeVar348 > 4.0 ) ) {

							nodeVar349 = mix( vec3<f32>( 0.1499597898006365, 0.17788841597328695, 0.09758734713304495 ), vec3<f32>( 0.1912016827303171, 0.22696587349938613, 0.11443537381770343 ), nodeVar259 );

						} else {


							if ( ( nodeVar348 > 3.0 ) ) {

								nodeVar350 = mix( vec3<f32>( 0.11443537381770343, 0.16202937562896222, 0.1912016827303171 ), vec3<f32>( 0.158960835050774, 0.21952619971859377, 0.25818285291079235 ), nodeVar259 );

							} else {


								if ( ( nodeVar348 > 2.0 ) ) {

									nodeVar351 = mix( vec3<f32>( 0.16202937562896222, 0.14412847084818123, 0.12743768042608497 ), vec3<f32>( 0.22696587349938613, 0.20507873637973145, 0.181164244239483 ), nodeVar259 );

								} else {


									if ( ( nodeVar348 > 1.0 ) ) {

										nodeVar352 = mix( vec3<f32>( 0.2541520943200296, 0.19461783043107173, 0.12743768042608497 ), vec3<f32>( 0.3277780980458375, 0.26225065751888765, 0.16826940017946088 ), nodeVar259 );

									} else {

										nodeVar352 = mix( vec3<f32>( 0.5906188409113381, 0.5209955731953768, 0.3813260114221238 ), vec3<f32>( 0.6866853124288864, 0.6104955708001716, 0.4793201830913402 ), nodeVar259 );

									}

									nodeVar351 = nodeVar352;

								}

								nodeVar350 = nodeVar351;

							}

							nodeVar349 = nodeVar350;

						}

						nodeVar345 = nodeVar349;

					}

					nodeVar353 = ( nodeVar265 + ( nodeVar267 * vec3<f32>( nodeVar284 ) ) );
					nodeVar344 = ( ( nodeVar345 * vec3<f32>( mix( 0.78, 1.12, fract( ( nodeVar353.x * 2.5 ) ) ) ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeVar353.z - 0.1 ) / nodeVar318 ), 0.0, 1.0 ) ) ) );

				} else {


					if ( nodeVar332 ) {

						nodeVar356 = ( nodeVar265 + ( nodeVar267 * vec3<f32>( nodeVar293 ) ) );

						if ( ( ( ( nodeVar356 - nodeVar287 ) / ( nodeVar289 - nodeVar287 ) ).y > 0.94 ) ) {

							nodeVar355 = 1.2;

						} else {

							nodeVar355 = 0.82;

						}

						nodeVar354 = ( ( mix( vec3<f32>( 0.04231141061442144, 0.025186859622305935, 0.015996293361446288 ), vec3<f32>( 0.09084171117479915, 0.06301001764564068, 0.04231141061442144 ), nodeVar278 ) * vec3<f32>( nodeVar355 ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeVar356.z - 0.1 ) / nodeVar318 ), 0.0, 1.0 ) ) ) );

					} else {


						if ( nodeVar331 ) {

							nodeVar359 = ( nodeVar265 + ( nodeVar267 * vec3<f32>( nodeVar304 ) ) );

							if ( ( ( ( nodeVar359 - nodeVar298 ) / ( nodeVar300 - nodeVar298 ) ).y > 0.94 ) ) {

								nodeVar358 = 1.2;

							} else {

								nodeVar358 = 0.82;

							}

							nodeVar357 = ( ( mix( vec3<f32>( 0.04231141061442144, 0.025186859622305935, 0.015996293361446288 ), vec3<f32>( 0.09084171117479915, 0.06301001764564068, 0.04231141061442144 ), nodeVar278 ) * vec3<f32>( nodeVar358 ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeVar359.z - 0.1 ) / nodeVar318 ), 0.0, 1.0 ) ) ) );

						} else {


							if ( nodeVar330 ) {

								nodeVar362 = ( nodeVar265 + ( nodeVar267 * vec3<f32>( nodeVar315 ) ) );

								if ( ( ( ( nodeVar362 - nodeVar309 ) / ( nodeVar311 - nodeVar309 ) ).y > 0.9 ) ) {

									nodeVar361 = 1.12;

								} else {

									nodeVar361 = 0.85;

								}

								nodeVar360 = ( ( mix( vec3<f32>( 0.10224173307914941, 0.06847816983662762, 0.04231141061442144 ), vec3<f32>( 0.054480276435339814, 0.09305896283800832, 0.14412847084818123 ), nodeVar278 ) * vec3<f32>( nodeVar361 ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeVar362.z - 0.1 ) / nodeVar318 ), 0.0, 1.0 ) ) ) );

							} else {


								if ( nodeVar329 ) {

									nodeVar365 = ( nodeVar265 + ( nodeVar267 * vec3<f32>( nodeVar326 ) ) );

									if ( ( ( ( nodeVar365 - nodeVar320 ) / ( nodeVar322 - nodeVar320 ) ).y > 0.94 ) ) {

										nodeVar364 = 1.25;

									} else {

										nodeVar364 = 0.8;

									}

									nodeVar363 = ( ( mix( vec3<f32>( 0.06847816983662762, 0.035601314869097636, 0.019382360952473074 ), vec3<f32>( 0.14702726648767014, 0.06847816983662762, 0.02955683443236377 ), nodeVar259 ) * vec3<f32>( nodeVar364 ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeVar365.z - 0.1 ) / nodeVar318 ), 0.0, 1.0 ) ) ) );

								} else {

									nodeVar367 = ( nodeVar265 + ( nodeVar267 * vec3<f32>( nodeVar328 ) ) );
									nodeVar368 = ( ( nodeVar367 - nodeVar262 ) / ( nodeVar256 - nodeVar262 ) );
									nodeVar369 = ( nodeVar368.z > 0.998 );

									if ( nodeVar369 ) {

										nodeVar370 = mix( mix( vec3<f32>( 0.3231432091022285, 0.25818285291079235, 0.171441100722554 ), vec3<f32>( 0.158960835050774, 0.19461783043107173, 0.22322795730611386 ), nodeVar278 ), vec3<f32>( 0.48514994004665124, 0.4178850708380236, 0.3094689228067428 ), ( nodeVar259 * 0.6 ) );
										nodeVar371 = mix( 0.22, 0.78, nodeVar278 );

										if ( ( nodeVar371 < 0.5 ) ) {

											nodeVar372 = mix( 0.68, 0.82, nodeVar259 );

										} else {

											nodeVar372 = mix( 0.18, 0.32, nodeVar259 );

										}

										nodeVar373 = ( ( ( nodeVar254 + 11240u ) * 747796405u ) + 2891336453u );
										nodeVar374 = ( ( ( nodeVar373 >> ( ( nodeVar373 >> 28u ) + 4u ) ) ^ nodeVar373 ) * 277803737u );
										nodeVar366 = mix( mix( mix( mix( nodeVar370, ( nodeVar370 * vec3<f32>( 0.5 ) ), smoothstep( 0.05, 0.04, nodeVar368.y ) ), mix( vec3<f32>( 0.10224173307914941, 0.061246054224174035, 0.030713443727452196 ), vec3<f32>( 0.040915196900556984, 0.03954623527052923, 0.04518620437910499 ), step( 0.5, nodeVar259 ) ), ( smoothstep( 0.09100000000000001, 0.079, abs( ( nodeVar368.x - nodeVar371 ) ) ) * smoothstep( 0.356, 0.344, abs( ( nodeVar368.y - 0.33 ) ) ) ) ), vec3<f32>( 0.0069954101845983935, 0.006048833020386069, 0.005181516700061659 ), ( smoothstep( 0.081, 0.06899999999999999, abs( ( nodeVar368.x - nodeVar372 ) ) ) * smoothstep( 0.09100000000000001, 0.079, abs( ( nodeVar368.y - 0.56 ) ) ) ) ), mix( vec3<f32>( 0.025186859622305935, 0.04231141061442144, 0.06847816983662762 ), vec3<f32>( 0.19461783043107173, 0.10224173307914941, 0.04231141061442144 ), ( f32( ( ( nodeVar374 >> 22u ) ^ nodeVar374 ) ) * 2.3283064365386963e-10 ) ), ( smoothstep( 0.061, 0.049, abs( ( nodeVar368.x - nodeVar372 ) ) ) * smoothstep( 0.07100000000000001, 0.059000000000000004, abs( ( nodeVar368.y - 0.56 ) ) ) ) );

									} else {


										if ( ( nodeVar368.y > 0.998 ) ) {

											nodeVar377 = ( ( ( nodeVar254 + 25910u ) * 747796405u ) + 2891336453u );
											nodeVar378 = ( ( ( nodeVar377 >> ( ( nodeVar377 >> 28u ) + 4u ) ) ^ nodeVar377 ) * 277803737u );

											if ( ( ( f32( ( ( nodeVar378 >> 22u ) ^ nodeVar378 ) ) * 2.3283064365386963e-10 ) > 0.88 ) ) {

												nodeVar379 = ( ( ( nodeVar254 + 59550u ) * 747796405u ) + 2891336453u );
												nodeVar380 = ( ( ( nodeVar379 >> ( ( nodeVar379 >> 28u ) + 4u ) ) ^ nodeVar379 ) * 277803737u );
												nodeVar376 = mix( vec3<f32>( 0.7379104087672317, 0.8069522576650873, 1.0 ), vec3<f32>( 0.34670405634441115, 0.46778379610254284, 1.0 ), ( f32( ( ( nodeVar380 >> 22u ) ^ nodeVar380 ) ) * 2.3283064365386963e-10 ) );

											} else {

												nodeVar381 = ( ( ( nodeVar254 + 86350u ) * 747796405u ) + 2891336453u );
												nodeVar382 = ( ( ( nodeVar381 >> ( ( nodeVar381 >> 28u ) + 4u ) ) ^ nodeVar381 ) * 277803737u );
												nodeVar376 = mix( vec3<f32>( 1.0, 0.4793201830913402, 0.059511238155621766 ), vec3<f32>( 1.0, 0.775822218312646, 0.33245153633549385 ), ( f32( ( ( nodeVar382 >> 22u ) ^ nodeVar382 ) ) * 2.3283064365386963e-10 ) );

											}

											nodeVar383 = ( ( ( nodeVar254 + 79599u ) * 747796405u ) + 2891336453u );
											nodeVar384 = ( ( ( nodeVar383 >> ( ( nodeVar383 >> 28u ) + 4u ) ) ^ nodeVar383 ) * 277803737u );
											nodeVar375 = mix( mix( mix( mix( vec3<f32>( 0.3231432091022285, 0.25818285291079235, 0.171441100722554 ), vec3<f32>( 0.158960835050774, 0.19461783043107173, 0.22322795730611386 ), nodeVar278 ), vec3<f32>( 0.48514994004665124, 0.4178850708380236, 0.3094689228067428 ), ( nodeVar259 * 0.6 ) ), vec3<f32>( 1.0, 1.0, 1.0 ), 0.5 ), ( nodeVar376 * vec3<f32>( mix( 1.0, 4.5, step( 0.8, ( f32( ( ( nodeVar384 >> 22u ) ^ nodeVar384 ) ) * 2.3283064365386963e-10 ) ) ) ) ), smoothstep( 0.16, 0.13, length( vec2<f32>( ( nodeVar368.x - 0.5 ), ( nodeVar368.z - 0.5 ) ) ) ) );

										} else {


											if ( ( nodeVar368.y < 0.002 ) ) {

												nodeVar385 = mix( ( mix( vec3<f32>( 0.06847816983662762, 0.033104766565152086, 0.014443843592229466 ), vec3<f32>( 0.14412847084818123, 0.0722718506743852, 0.02955683443236377 ), nodeVar278 ) * vec3<f32>( ( 1.0 - ( step( 0.94, fract( ( nodeVar368.x * 6.0 ) ) ) * 0.3 ) ) ) ), mix( vec3<f32>( 0.19461783043107173, 0.04373502925049377, 0.031896033067374104 ), vec3<f32>( 0.04231141061442144, 0.09530746662221588, 0.11697066774917994 ), nodeVar259 ), ( ( smoothstep( 0.306, 0.294, abs( ( nodeVar368.x - 0.5 ) ) ) * smoothstep( 0.266, 0.254, abs( ( nodeVar368.z - 0.62 ) ) ) ) * 0.9 ) );

											} else {

												nodeVar386 = mix( mix( vec3<f32>( 0.3231432091022285, 0.25818285291079235, 0.171441100722554 ), vec3<f32>( 0.158960835050774, 0.19461783043107173, 0.22322795730611386 ), nodeVar278 ), vec3<f32>( 0.48514994004665124, 0.4178850708380236, 0.3094689228067428 ), ( nodeVar259 * 0.6 ) );
												nodeVar385 = mix( nodeVar386, ( nodeVar386 * vec3<f32>( 0.5 ) ), smoothstep( 0.05, 0.04, nodeVar368.y ) );

											}

											nodeVar375 = nodeVar385;

										}

										nodeVar366 = nodeVar375;

									}


									if ( nodeVar369 ) {

										nodeVar387 = ( ( smoothstep( 0.0, 0.15, nodeVar368.x ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar368.x ) ) ) * ( smoothstep( 0.0, 0.15, nodeVar368.y ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar368.y ) ) ) );

									} else {


										if ( ( ( nodeVar368.y < 0.002 ) || ( nodeVar368.y > 0.998 ) ) ) {

											nodeVar388 = ( ( smoothstep( 0.0, 0.15, nodeVar368.x ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar368.x ) ) ) * ( smoothstep( 0.0, 0.15, nodeVar368.z ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar368.z ) ) ) );

										} else {

											nodeVar388 = ( ( smoothstep( 0.0, 0.15, nodeVar368.y ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar368.y ) ) ) * ( smoothstep( 0.0, 0.15, nodeVar368.z ) * smoothstep( 0.0, 0.15, ( 1.0 - nodeVar368.z ) ) ) );

										}

										nodeVar387 = nodeVar388;

									}

									nodeVar363 = ( ( nodeVar366 * vec3<f32>( mix( 0.72, 1.0, nodeVar387 ) ) ) * vec3<f32>( mix( 1.0, 0.42, clamp( ( ( nodeVar367.z - 0.1 ) / nodeVar318 ), 0.0, 1.0 ) ) ) );

								}

								nodeVar360 = nodeVar363;

							}

							nodeVar357 = nodeVar360;

						}

						nodeVar354 = nodeVar357;

					}

					nodeVar344 = nodeVar354;

				}

				nodeVar255 = nodeVar344;

			}

			nodeVar390 = ( ( ( nodeVar254 + 25910u ) * 747796405u ) + 2891336453u );
			nodeVar391 = ( ( ( nodeVar390 >> ( ( nodeVar390 >> 28u ) + 4u ) ) ^ nodeVar390 ) * 277803737u );

			if ( ( ( f32( ( ( nodeVar391 >> 22u ) ^ nodeVar391 ) ) * 2.3283064365386963e-10 ) > 0.88 ) ) {

				nodeVar392 = ( ( ( nodeVar254 + 59550u ) * 747796405u ) + 2891336453u );
				nodeVar393 = ( ( ( nodeVar392 >> ( ( nodeVar392 >> 28u ) + 4u ) ) ^ nodeVar392 ) * 277803737u );
				nodeVar389 = mix( vec3<f32>( 0.7379104087672317, 0.8069522576650873, 1.0 ), vec3<f32>( 0.34670405634441115, 0.46778379610254284, 1.0 ), ( f32( ( ( nodeVar393 >> 22u ) ^ nodeVar393 ) ) * 2.3283064365386963e-10 ) );

			} else {

				nodeVar394 = ( ( ( nodeVar254 + 86350u ) * 747796405u ) + 2891336453u );
				nodeVar395 = ( ( ( nodeVar394 >> ( ( nodeVar394 >> 28u ) + 4u ) ) ^ nodeVar394 ) * 277803737u );
				nodeVar389 = mix( vec3<f32>( 1.0, 0.4793201830913402, 0.059511238155621766 ), vec3<f32>( 1.0, 0.775822218312646, 0.33245153633549385 ), ( f32( ( ( nodeVar395 >> 22u ) ^ nodeVar395 ) ) * 2.3283064365386963e-10 ) );

			}

			nodeVar396 = ( ( ( nodeVar254 + 79599u ) * 747796405u ) + 2891336453u );
			nodeVar397 = ( ( ( nodeVar396 >> ( ( nodeVar396 >> 28u ) + 4u ) ) ^ nodeVar396 ) * 277803737u );
			nodeVar398 = step( 0.8, ( f32( ( ( nodeVar397 >> 22u ) ^ nodeVar397 ) ) * 2.3283064365386963e-10 ) );

			if ( nodeVar334 ) {

				nodeVar399 = 0.2;

			} else {


				if ( nodeVar333 ) {

					nodeVar400 = 0.2;

				} else {


					if ( nodeVar332 ) {

						nodeVar401 = 1.0;

					} else {


						if ( nodeVar331 ) {

							nodeVar402 = 1.0;

						} else {


							if ( nodeVar330 ) {

								nodeVar403 = 1.0;

							} else {


								if ( nodeVar329 ) {

									nodeVar404 = 1.0;

								} else {

									nodeVar404 = 1.0;

								}

								nodeVar403 = nodeVar404;

							}

							nodeVar402 = nodeVar403;

						}

						nodeVar401 = nodeVar402;

					}

					nodeVar400 = nodeVar401;

				}

				nodeVar399 = nodeVar400;

			}

			nodeVar2 = vec4<f32>( ( ( nodeVar255 * mix( vec3<f32>( 1.0, 1.0, 1.0 ), nodeVar389, ( nodeVar398 * 0.85 ) ) ) * vec3<f32>( mix( 1.0, 1.3, nodeVar398 ) ) ), ( nodeVar398 * nodeVar399 ) );

		}

		nodeVar405 = nodeVar2;

		if ( nodeVar1 ) {

			nodeVar406 = vec3<f32>( 0.6038273388475408, 0.6583748172725346, 0.623960391667596 );

		} else {

			nodeVar406 = vec3<f32>( 0.46778379610254284, 0.5647115056965487, 0.5209955731953768 );

		}


		if ( nodeVar1 ) {

			nodeVar407 = vec3<f32>( 0.008023192982520563, 0.010329823026364548, 0.012983032338510335 );

		} else {

			nodeVar408 = ( v_positionWorld * vec3<f32>( 0.3 ) );
			nodeVar407 = mix( vec3<f32>( 0.006512090790025684, 0.008023192982520563, 0.010329823026364548 ), vec3<f32>( 0.01680737574872402, 0.024157632443547246, 0.030713443727452196 ), ( ( ( valueNoise( nodeVar408 ) + ( valueNoise( ( nodeVar408 * vec3<f32>( 2.0 ) ) ) * 0.5 ) ) * 0.5 ) + 0.5 ) );

		}


		if ( nodeVar1 ) {

			nodeVar409 = ( 0.14 + ( ( smoothstep( -0.15, 0.5, ( mx_fractal_noise_float( vec3<f32>( ( v_positionWorld.x * 1.3 ), ( v_positionWorld.y * 0.06 ), ( v_positionWorld.z * 1.3 ) ), 2, 2.0, 0.5 ) * 1.0 ) ) * 0.45 ) * 0.22 ) );

		} else {

			nodeVar409 = clamp( ( ( 0.64 + ( smoothstep( -0.15, 0.5, ( mx_fractal_noise_float( vec3<f32>( ( v_positionWorld.x * 1.3 ), ( v_positionWorld.y * 0.06 ), ( v_positionWorld.z * 1.3 ) ), 2, 2.0, 0.5 ) * 1.0 ) ) * 0.45 ) ) + ( smoothstep( 0.32, 0.0, nodeVarying13.y ) * 0.4 ) ), 0.0, 0.95 );

		}

		nodeVar0 = mix( ( nodeVar405.xyz * nodeVar406 ), nodeVar407, nodeVar409 );

	} else {


		if ( ( nodeVarying3 == 7.0 ) ) {

			nodeVar410 = mix( ( object.nodeUniform3 * vec3<f32>( 0.3 ) ), vec3<f32>( 0.01764195448412081, 0.014443843592229466, 0.011612245176281512 ), 0.5 );

		} else {


			if ( ( nodeVarying3 == 8.0 ) ) {

				nodeVar413 = floor( ( vec2<f32>( v_positionWorld.x, v_positionWorld.z ) * vec2<f32>( 0.2 ) ) );
				nodeVar414 = ( ( ( ( u32( ( nodeVar413.x + 65536.0 ) ) * 73856093u ) ^ ( u32( ( nodeVar413.y + 65536.0 ) ) * 19349663u ) ) * 747796405u ) + 2891336453u );
				nodeVar415 = ( ( ( nodeVar414 >> ( ( nodeVar414 >> 28u ) + 4u ) ) ^ nodeVar414 ) * 277803737u );
				nodeVar416 = ( ( f32( ( ( nodeVar415 >> 22u ) ^ nodeVar415 ) ) * 2.3283064365386963e-10 ) * 5.0 );

				if ( ( nodeVar416 > 4.0 ) ) {

					nodeVar412 = vec3<f32>( 0.014443843592229466, 0.013702083043526807, 0.015996293361446288 );

				} else {


					if ( ( nodeVar416 > 3.0 ) ) {

						nodeVar417 = vec3<f32>( 0.10224173307914941, 0.11193242782769693, 0.09758734713304495 );

					} else {


						if ( ( nodeVar416 > 2.0 ) ) {

							nodeVar418 = vec3<f32>( 0.015996293361446288, 0.03954623527052923, 0.07818742179702069 );

						} else {


							if ( ( nodeVar416 > 1.0 ) ) {

								nodeVar419 = vec3<f32>( 0.01764195448412081, 0.06847816983662762, 0.031896033067374104 );

							} else {

								nodeVar419 = vec3<f32>( 0.14412847084818123, 0.028426039499072558, 0.028426039499072558 );

							}

							nodeVar418 = nodeVar419;

						}

						nodeVar417 = nodeVar418;

					}

					nodeVar412 = nodeVar417;

				}

				nodeVar411 = nodeVar412;

			} else {


				if ( ( nodeVarying3 == 2.0 ) ) {

					nodeVar420 = ( object.nodeUniform3 * vec3<f32>( 0.55 ) );

				} else {

					nodeVar422 = ( nodeVarying3 == 3.0 );

					if ( nodeVar422 ) {

						nodeVar421 = ( mix( object.nodeUniform3, vec3<f32>( 1.0, 1.0, 1.0 ), 0.22 ) * vec3<f32>( ( 1.0 + ( nodeVarying7 * 0.18 ) ) ) );

					} else {


						if ( ( nodeVarying3 == 5.0 ) ) {

							nodeVar425 = ( ( valueNoise( ( v_positionWorld * vec3<f32>( 0.4 ) ) ) * 0.5 ) + 0.5 );
							normalWorldGeometry = normalize( v_normalWorldGeometry );
							nodeVar426 = vec3<f32>( ( v_positionWorld.x * 6.0 ), ( v_positionWorld.y * 0.5 ), ( v_positionWorld.z * 6.0 ) );
							nodeVar424 = mix( ( ( mix( vec3<f32>( 0.8879231178794776, 0.8796223968851662, 0.8387990117372213 ), vec3<f32>( 0.623960391667596, 0.6038273388475408, 0.5394794890033748 ), nodeVar425 ) + vec3<f32>( ( valueNoise( ( v_positionWorld * vec3<f32>( 5.0 ) ) ) * 0.04 ) ) ) * vec3<f32>( mix( 1.0, ( mix( 0.82, 1.04, fract( ( nodeVarying13.y * 6.0 ) ) ) * 0.42 ), ( ( ( ( smoothstep( 0.06, 0.14, nodeVarying13.x ) * smoothstep( 0.94, 0.86, nodeVarying13.x ) ) * smoothstep( 0.12, 0.2, nodeVarying13.y ) ) * smoothstep( 0.96, 0.88, nodeVarying13.y ) ) * ( smoothstep( 0.65, 0.4, abs( normalWorldGeometry.y ) ) * smoothstep( 0.08, 0.015, length( fwidth( v_positionWorld ) ) ) ) ) ) ) ), vec3<f32>( 0.158960835050774, 0.13843161502267545, 0.10224173307914941 ), ( ( ( smoothstep( 0.4, 0.0, nodeVarying13.y ) * ( ( ( ( valueNoise( nodeVar426 ) + ( valueNoise( ( nodeVar426 * vec3<f32>( 2.0 ) ) ) * 0.5 ) ) + ( valueNoise( ( nodeVar426 * vec3<f32>( 4.0 ) ) ) * 0.25 ) ) * 0.5 ) + 0.5 ) ) * ( nodeVar425 + 0.3 ) ) * 0.5 ) );

						} else {


							if ( ( nodeVarying3 == 1.0 ) ) {

								nodeVar427 = 0.12;

							} else {


								if ( nodeVar422 ) {

									nodeVar428 = 0.2;

								} else {

									nodeVar428 = 0.0;

								}

								nodeVar427 = nodeVar428;

							}

							nodeVar429 = ( positionLocal.y / 0.3 );
							nodeVar430 = floor( nodeVar429 );
							normalWorldGeometry = normalize( v_normalWorldGeometry );
							normalWorldGeometry = normalize( v_normalWorldGeometry );
							nodeVar431 = ( ( ( ( positionLocal.x * normalWorldGeometry.z ) - ( positionLocal.z * normalWorldGeometry.x ) ) / 0.6 ) + ( tsl_mod_float( nodeVar430, 2.0 ) * 0.5 ) );
							nodeVar432 = ( ( u32( ( nodeVar430 + 65536.0 ) ) * 73856093u ) ^ ( u32( ( floor( nodeVar431 ) + 65536.0 ) ) * 19349663u ) );
							nodeVar433 = ( ( nodeVar432 * 747796405u ) + 2891336453u );
							nodeVar434 = ( ( ( nodeVar433 >> ( ( nodeVar433 >> 28u ) + 4u ) ) ^ nodeVar433 ) * 277803737u );
							nodeVar435 = ( ( ( nodeVar432 + 1u ) * 747796405u ) + 2891336453u );
							nodeVar436 = ( ( ( nodeVar435 >> ( ( nodeVar435 >> 28u ) + 4u ) ) ^ nodeVar435 ) * 277803737u );
							nodeVar437 = ( ( ( f32( ( ( nodeVar436 >> 22u ) ^ nodeVar436 ) ) * 2.3283064365386963e-10 ) - 0.5 ) * 0.14 );
							nodeVar438 = ( ( mix( object.nodeUniform3, vec3<f32>( 1.0, 1.0, 1.0 ), nodeVar427 ) * vec3<f32>( ( ( ( 1.0 + ( nodeVarying7 * 0.18 ) ) + ( valueNoise( ( v_positionWorld * vec3<f32>( 0.7 ) ) ) * 0.06 ) ) + ( ( ( f32( ( ( nodeVar434 >> 22u ) ^ nodeVar434 ) ) * 2.3283064365386963e-10 ) - 0.5 ) * 0.14 ) ) ) ) * vec3<f32>( ( 1.0 + nodeVar437 ), 1.0, ( 1.0 - nodeVar437 ) ) );
							normalWorldGeometry = normalize( v_normalWorldGeometry );
							nodeVar439 = abs( normalWorldGeometry );
							nodeVar440 = clamp( ( ( ( nodeVar439.z * fwidth( v_positionWorld.x ) ) + ( nodeVar439.x * fwidth( v_positionWorld.z ) ) ) / 0.6 ), 0.000001, 0.5 );
							nodeVar441 = max( nodeVar440, 0.008333333333333333 );
							nodeVar442 = clamp( fwidth( nodeVar429 ), 0.000001, 0.5 );
							nodeVar443 = max( nodeVar442, 0.016666666666666666 );
							nodeVar444 = smoothstep( 0.7, 0.45, nodeVar439.y );
							nodeVar446 = ( 1.0 - nodeVar444 );

							if ( ( nodeVar446 > 0.0 ) ) {

								nodeVar447 = ( v_positionWorld * vec3<f32>( 0.025 ) );
								nodeVar445 = ( smoothstep( 0.0, 0.55, ( ( valueNoise( nodeVar447 ) + ( valueNoise( ( nodeVar447 * vec3<f32>( 2.0 ) ) ) * 0.5 ) ) + ( valueNoise( ( nodeVar447 * vec3<f32>( 4.0 ) ) ) * 0.25 ) ) ) * 0.22 );

							} else {

								nodeVar445 = 0.0;

							}

							nodeVar424 = mix( mix( nodeVar438, ( nodeVar438 * vec3<f32>( 0.6 ) ), ( max( ( smoothstep( ( nodeVar441 + nodeVar440 ), ( nodeVar441 - nodeVar440 ), ( 0.5 - abs( ( fract( nodeVar431 ) - 0.5 ) ) ) ) * min( ( 0.008333333333333333 / nodeVar441 ), 1.0 ) ), ( smoothstep( ( nodeVar443 + nodeVar442 ), ( nodeVar443 - nodeVar442 ), ( 0.5 - abs( ( fract( nodeVar429 ) - 0.5 ) ) ) ) * min( ( 0.016666666666666666 / nodeVar443 ), 1.0 ) ) ) * nodeVar444 ) ), vec3<f32>( 0.06847816983662762, 0.054480276435339814, 0.036889450395083165 ), mix( ( ( smoothstep( -0.1, 0.45, ( mx_fractal_noise_float( vec3<f32>( ( v_positionWorld.x * 1.5 ), ( v_positionWorld.y * 0.04 ), ( v_positionWorld.z * 1.5 ) ), 2, 2.0, 0.5 ) * 1.0 ) ) * smoothstep( 210.0, 0.0, v_positionWorld.y ) ) * 0.6 ), nodeVar445, nodeVar446 ) );

						}

						nodeVar421 = nodeVar424;

					}

					nodeVar420 = nodeVar421;

				}

				nodeVar411 = nodeVar420;

			}

			nodeVar410 = nodeVar411;

		}

		nodeVar0 = nodeVar410;

	}

	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), ( 1.0 * vec4<f32>( nodeVar0, 1.0 ).w ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform6 );
	nodeVar448 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar448;

	// result

	output.color = nodeVar448;

	return output;

}
