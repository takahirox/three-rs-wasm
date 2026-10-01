// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


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
@binding( 10 ) @group( 1 ) var nodeUniform45_sampler : sampler;
@binding( 11 ) @group( 1 ) var nodeUniform45 : texture_2d<f32>;

struct objectStruct {
	nodeUniform2 : f32,
	nodeUniform5 : vec3<f32>,
	nodeUniform6 : f32,
	nodeUniform40 : f32,
	nodeUniform41 : mat4x4<f32>,
	nodeUniform43 : f32,
	nodeUniform44 : f32,
	nodeUniform46 : f32
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
var<private> nodeVar5 : f32;
var<private> nodeVar6 : vec2<f32>;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : vec3<f32>;
var<private> nodeVar12 : vec3<f32>;
var<private> nodeVar13 : vec3<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : vec3<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : vec3<f32>;
var<private> nodeVar25 : vec3<f32>;
var<private> nodeVar26 : vec3<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec3<f32>;
var<private> nodeVar31 : vec3<f32>;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : vec3<f32>;
var<private> nodeVar34 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : vec3<f32>;
var<private> nodeVar40 : vec3<f32>;
var<private> nodeVar41 : vec3<f32>;
var<private> nodeVar42 : vec3<f32>;
var<private> nodeVar43 : vec3<f32>;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : vec3<f32>;
var<private> nodeVar51 : vec3<f32>;
var<private> nodeVar52 : vec3<f32>;
var<private> nodeVar53 : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : vec3<f32>;
var<private> nodeVar61 : vec3<f32>;
var<private> nodeVar62 : vec3<f32>;
var<private> nodeVar63 : vec3<f32>;
var<private> nodeVar64 : vec3<f32>;
var<private> nodeVar65 : vec3<f32>;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : vec3<f32>;
var<private> nodeVar72 : vec3<f32>;
var<private> nodeVar73 : vec3<f32>;
var<private> nodeVar74 : vec3<f32>;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : f32;
var<private> nodeVar88 : f32;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : vec3<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : vec3<f32>;
var<private> nodeVar102 : vec3<f32>;
var<private> nodeVar103 : vec3<f32>;
var<private> nodeVar104 : vec3<f32>;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : vec3<f32>;
var<private> nodeVar115 : vec3<f32>;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : f32;
var<private> nodeVar118 : f32;
var<private> nodeVar119 : f32;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : vec3<f32>;
var<private> nodeVar124 : vec3<f32>;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : f32;
var<private> nodeVar127 : f32;
var<private> nodeVar128 : f32;
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
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : vec3<f32>;
var<private> nodeVar144 : vec3<f32>;
var<private> nodeVar145 : vec3<f32>;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : vec3<f32>;
var<private> nodeVar153 : vec3<f32>;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : f32;
var<private> nodeVar157 : f32;
var<private> nodeVar158 : f32;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec3<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : vec3<f32>;
var<private> nodeVar164 : vec3<f32>;
var<private> nodeVar165 : f32;
var<private> nodeVar166 : f32;
var<private> nodeVar167 : f32;
var<private> nodeVar168 : f32;
var<private> nodeVar169 : f32;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : vec3<f32>;
var<private> nodeVar175 : vec3<f32>;
var<private> nodeVar176 : vec3<f32>;
var<private> nodeVar177 : f32;
var<private> nodeVar178 : f32;
var<private> nodeVar179 : f32;
var<private> nodeVar180 : vec3<f32>;
var<private> nodeVar181 : vec3<f32>;
var<private> nodeVar182 : vec3<f32>;
var<private> nodeVar183 : vec3<f32>;
var<private> nodeVar184 : vec3<f32>;
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : f32;
var<private> nodeVar187 : f32;
var<private> nodeVar188 : f32;
var<private> nodeVar189 : vec3<f32>;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : vec3<f32>;
var<private> nodeVar192 : vec3<f32>;
var<private> nodeVar193 : vec3<f32>;
var<private> nodeVar194 : vec3<f32>;
var<private> nodeVar195 : f32;
var<private> nodeVar196 : f32;
var<private> nodeVar197 : f32;
var<private> nodeVar198 : f32;
var<private> nodeVar199 : f32;
var<private> nodeVar200 : vec3<f32>;
var<private> nodeVar201 : vec3<f32>;
var<private> nodeVar202 : vec3<f32>;
var<private> nodeVar203 : vec3<f32>;
var<private> nodeVar204 : vec3<f32>;
var<private> nodeVar205 : vec3<f32>;
var<private> nodeVar206 : vec3<f32>;
var<private> nodeVar207 : f32;
var<private> nodeVar208 : f32;
var<private> nodeVar209 : f32;
var<private> nodeVar210 : vec3<f32>;
var<private> nodeVar211 : vec3<f32>;
var<private> nodeVar212 : vec3<f32>;
var<private> nodeVar213 : vec3<f32>;
var<private> nodeVar214 : vec3<f32>;
var<private> nodeVar215 : vec3<f32>;
var<private> nodeVar216 : f32;
var<private> nodeVar217 : f32;
var<private> nodeVar218 : f32;
var<private> nodeVar219 : vec3<f32>;
var<private> nodeVar220 : vec3<f32>;
var<private> nodeVar221 : vec3<f32>;
var<private> nodeVar222 : vec3<f32>;
var<private> nodeVar223 : vec3<f32>;
var<private> nodeVar224 : vec3<f32>;
var<private> nodeVar225 : f32;
var<private> nodeVar226 : f32;
var<private> nodeVar227 : f32;
var<private> nodeVar228 : f32;
var<private> nodeVar229 : f32;
var<private> nodeVar230 : vec3<f32>;
var<private> nodeVar231 : vec3<f32>;
var<private> nodeVar232 : vec3<f32>;
var<private> nodeVar233 : vec3<f32>;
var<private> nodeVar234 : vec3<f32>;
var<private> nodeVar235 : vec3<f32>;
var<private> nodeVar236 : vec3<f32>;
var<private> nodeVar237 : f32;
var<private> nodeVar238 : f32;
var<private> nodeVar239 : f32;
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
var<private> radiance : vec3<f32>;
var<private> nodeVar253 : f32;
var<private> nodeVar254 : f32;
var<private> nodeVar255 : f32;
var<private> nodeVar256 : vec3<f32>;
var<private> nodeVar257 : f32;
var<private> nodeVar258 : f32;
var<private> nodeVar259 : f32;
var<private> nodeVar260 : vec2<f32>;
var<private> nodeVar261 : vec4<f32>;
var<private> nodeVar262 : vec3<f32>;
var<private> nodeVar263 : f32;
var<private> nodeVar264 : f32;
var<private> nodeVar265 : f32;
var<private> nodeVar266 : f32;
var<private> nodeVar267 : f32;
var<private> nodeVar268 : vec2<f32>;
var<private> nodeVar269 : vec4<f32>;
var<private> nodeVar270 : vec3<f32>;
var<private> nodeVar271 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar272 : f32;
var<private> nodeVar273 : f32;
var<private> nodeVar274 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar275 : f32;
var<private> nodeVar276 : f32;
var<private> nodeVar277 : f32;
var<private> nodeVar278 : vec2<f32>;
var<private> nodeVar279 : vec4<f32>;
var<private> nodeVar280 : vec3<f32>;
var<private> nodeVar281 : f32;
var<private> nodeVar282 : f32;
var<private> nodeVar283 : f32;
var<private> nodeVar284 : f32;
var<private> nodeVar285 : f32;
var<private> nodeVar286 : vec2<f32>;
var<private> nodeVar287 : vec4<f32>;
var<private> nodeVar288 : vec3<f32>;
var<private> nodeVar289 : vec3<f32>;
var<private> nodeVar290 : vec3<f32>;
var<private> nodeVar291 : vec3<f32>;
var<private> nodeVar292 : vec3<f32>;
var<private> nodeVar293 : f32;
var<private> nodeVar294 : vec3<f32>;
var<private> nodeVar295 : vec3<f32>;
var<private> nodeVar296 : vec3<f32>;
var<private> nodeVar297 : vec3<f32>;
var<private> nodeVar298 : vec3<f32>;
var<private> nodeVar299 : vec3<f32>;
var<private> nodeVar300 : vec3<f32>;
var<private> nodeVar301 : f32;
var<private> nodeVar302 : f32;
var<private> nodeVar303 : f32;
var<private> nodeVar304 : vec3<f32>;
var<private> nodeVar305 : vec3<f32>;
var<private> nodeVar306 : vec3<f32>;
var<private> nodeVar307 : vec3<f32>;
var<private> nodeVar308 : vec3<f32>;
var<private> nodeVar309 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar310 : vec3<f32>;
var<private> nodeVar311 : vec3<f32>;
var<private> nodeVar312 : vec3<f32>;
var<private> nodeVar313 : vec3<f32>;
var<private> nodeVar314 : vec3<f32>;
var<private> nodeVar315 : vec3<f32>;
var<private> nodeVar316 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar317 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar318 : vec3<f32>;
var<private> nodeVar319 : f32;
var<private> nodeVar320 : vec3<f32>;
var<private> nodeVar321 : vec3<f32>;
var<private> nodeVar322 : vec3<f32>;
var<private> nodeVar323 : vec3<f32>;
var<private> nodeVar324 : vec3<f32>;
var<private> nodeVar325 : vec3<f32>;
var<private> nodeVar326 : vec3<f32>;
var<private> nodeVar327 : f32;
var<private> nodeVar328 : f32;
var<private> nodeVar329 : f32;
var<private> nodeVar330 : vec3<f32>;
var<private> nodeVar331 : vec3<f32>;
var<private> nodeVar332 : vec3<f32>;
var<private> nodeVar333 : vec3<f32>;
var<private> nodeVar334 : vec3<f32>;
var<private> nodeVar335 : vec3<f32>;
var<private> nodeVar336 : vec3<f32>;
var<private> nodeVar337 : f32;
var<private> nodeVar338 : vec3<f32>;
var<private> nodeVar339 : vec3<f32>;
var<private> nodeVar340 : vec3<f32>;
var<private> nodeVar341 : vec3<f32>;
var<private> nodeVar342 : vec3<f32>;
var<private> nodeVar343 : vec3<f32>;
var<private> nodeVar344 : vec3<f32>;
var<private> nodeVar345 : f32;
var<private> nodeVar346 : f32;
var<private> nodeVar347 : f32;
var<private> nodeVar348 : vec3<f32>;
var<private> nodeVar349 : vec3<f32>;
var<private> nodeVar350 : vec3<f32>;
var<private> nodeVar351 : vec3<f32>;
var<private> nodeVar352 : vec3<f32>;
var<private> nodeVar353 : vec3<f32>;
var<private> nodeVar354 : vec3<f32>;
var<private> nodeVar355 : vec3<f32>;
var<private> nodeVar356 : vec3<f32>;
var<private> nodeVar357 : vec3<f32>;
var<private> nodeVar358 : vec3<f32>;
var<private> nodeVar359 : vec3<f32>;
var<private> nodeVar360 : vec3<f32>;
var<private> nodeVar361 : vec3<f32>;
var<private> nodeVar362 : vec3<f32>;
var<private> nodeVar363 : vec3<f32>;
var<private> nodeVar364 : vec3<f32>;
var<private> nodeVar365 : vec3<f32>;
var<private> nodeVar366 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar367 : vec3<f32>;
var<private> nodeVar368 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar369 : vec3<f32>;
var<private> nodeVar370 : f32;
var<private> nodeVar371 : f32;
var<private> nodeVar372 : f32;
var<private> nodeVar373 : f32;
var<private> nodeVar374 : f32;
var<private> nodeVar375 : f32;
var<private> nodeVar376 : f32;
var<private> nodeVar377 : f32;
var<private> nodeVar378 : f32;
var<private> nodeVar379 : f32;
var<private> nodeVar380 : f32;
var<private> nodeVar381 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar382 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar383 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar384 : vec3<f32>;
var<private> nodeVar385 : vec4<f32>;

// codes
fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

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
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar1 = textureDimensions( nodeUniform0, u32( 0 ) );
	nodeVar0 = textureLoad( nodeUniform0, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVarying0 ) * vec2<f32>( nodeVar1 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar1 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	output.depth = nodeVar0;

	if ( ( nodeVar0 >= 1.0 ) ) {

		discard;
		

	}

	nodeVar2 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVarying0 );
	DiffuseColor = nodeVar2;
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform2 );
	DiffuseColor.w = 1.0;
	nodeVar3 = textureSample( nodeUniform3, nodeUniform3_sampler, nodeVarying0 );
	Metalness = nodeVar3.w;
	nodeVar4 = textureSample( nodeUniform4, nodeUniform4_sampler, nodeVarying0 );
	Roughness = min( ( max( nodeVar4.w, 0.0525 ) + 0.0 ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - nodeVar3.w ) ) );
	EmissiveColor = ( object.nodeUniform5 * vec3<f32>( object.nodeUniform6 ) );
	nodeVar5 = dot( nodeVar4.xyz, normalize( ( - nodeVar3.xyz ) ) );
	nodeVar6 = textureSample( nodeUniform7, nodeUniform7_sampler, vec2<f32>( Roughness, clamp( nodeVar5, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar6;
	nodeVar7 = ( dfg.x + dfg.y );
	nodeVar8 = ( 1.0 / nodeVar7 );
	nodeVar9 = nodeVar8;
	nodeVar10 = ( nodeVar9 - 1.0 );
	nodeVar11 = ( SpecularColorBlended * vec3<f32>( nodeVar10 ) );
	nodeVar12 = ( nodeVar11 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar12;
	nodeVar13 = ( render.nodeUniform8 - nodeVar3.xyz );
	nodeVar14 = normalize( nodeVar13 );
	nodeVar15 = dot( nodeVar4.xyz, nodeVar14 );

	if ( ( render.nodeUniform10 > 0.0 ) ) {

		nodeVar17 = length( nodeVar13 );
		nodeVar18 = ( nodeVar17 / render.nodeUniform10 );
		nodeVar19 = clamp( ( 1.0 - ( ( ( nodeVar18 * nodeVar18 ) * nodeVar18 ) * nodeVar18 ) ), 0.0, 1.0 );
		nodeVar16 = ( ( 1.0 / max( pow( nodeVar17, render.nodeUniform11 ), 0.01 ) ) * ( nodeVar19 * nodeVar19 ) );

	} else {

		nodeVar16 = ( 1.0 / max( pow( length( nodeVar13 ), render.nodeUniform11 ), 0.01 ) );

	}

	nodeVar20 = ( render.nodeUniform9 * vec3<f32>( nodeVar16 ) );
	nodeVar21 = ( vec3<f32>( clamp( nodeVar15, 0.0, 1.0 ) ) * nodeVar20 );
	nodeVar22 = nodeVar21;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar23 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar24 = ( nodeVar22 * nodeVar23 );
	nodeVar25 = ( nodeVar14 + normalize( ( - nodeVar3.xyz ) ) );
	nodeVar26 = normalize( nodeVar25 );
	nodeVar27 = dot( normalize( ( - nodeVar3.xyz ) ), nodeVar26 );
	nodeVar28 = clamp( nodeVar27, 0.0, 1.0 );
	nodeVar29 = exp2( ( ( ( nodeVar28 * -5.55473 ) - 6.98316 ) * nodeVar28 ) );
	nodeVar30 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar29 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar29 ) ) );
	nodeVar31 = ( vec3<f32>( 1.0 ) - nodeVar30 );
	nodeVar32 = nodeVar31;
	nodeVar33 = ( nodeVar24 * nodeVar32 );
	nodeVar34 = ( directDiffuse + nodeVar33 );
	directDiffuse = nodeVar34;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar35 = normalize( ( nodeVar14 + normalize( ( - nodeVar3.xyz ) ) ) );
	nodeVar36 = clamp( dot( normalize( ( - nodeVar3.xyz ) ), nodeVar35 ), 0.0, 1.0 );
	nodeVar37 = exp2( ( ( ( nodeVar36 * -5.55473 ) - 6.98316 ) * nodeVar36 ) );
	nodeVar38 = ( Roughness * Roughness );
	nodeVar39 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar37 ) ) ) + vec3<f32>( ( 1.0 * nodeVar37 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar38, clamp( dot( nodeVar4.xyz, nodeVar14 ), 0.0, 1.0 ), clamp( dot( nodeVar4.xyz, normalize( ( - nodeVar3.xyz ) ) ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar38, clamp( dot( nodeVar4.xyz, nodeVar35 ), 0.0, 1.0 ) ) ) );
	nodeVar40 = ( nodeVar22 * nodeVar39 );
	nodeVar41 = ( nodeVar40 * multiScatteringCompensation );
	nodeVar42 = ( directSpecular + nodeVar41 );
	directSpecular = nodeVar42;
	nodeVar43 = ( render.nodeUniform12 - nodeVar3.xyz );
	nodeVar44 = normalize( nodeVar43 );
	nodeVar45 = dot( nodeVar4.xyz, nodeVar44 );

	if ( ( render.nodeUniform14 > 0.0 ) ) {

		nodeVar47 = length( nodeVar43 );
		nodeVar48 = ( nodeVar47 / render.nodeUniform14 );
		nodeVar49 = clamp( ( 1.0 - ( ( ( nodeVar48 * nodeVar48 ) * nodeVar48 ) * nodeVar48 ) ), 0.0, 1.0 );
		nodeVar46 = ( ( 1.0 / max( pow( nodeVar47, render.nodeUniform15 ), 0.01 ) ) * ( nodeVar49 * nodeVar49 ) );

	} else {

		nodeVar46 = ( 1.0 / max( pow( length( nodeVar43 ), render.nodeUniform15 ), 0.01 ) );

	}

	nodeVar50 = ( render.nodeUniform13 * vec3<f32>( nodeVar46 ) );
	nodeVar51 = ( vec3<f32>( clamp( nodeVar45, 0.0, 1.0 ) ) * nodeVar50 );
	nodeVar52 = nodeVar51;
	nodeVar53 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar54 = ( nodeVar52 * nodeVar53 );
	nodeVar55 = ( nodeVar44 + normalize( ( - nodeVar3.xyz ) ) );
	nodeVar56 = normalize( nodeVar55 );
	nodeVar57 = dot( normalize( ( - nodeVar3.xyz ) ), nodeVar56 );
	nodeVar58 = clamp( nodeVar57, 0.0, 1.0 );
	nodeVar59 = exp2( ( ( ( nodeVar58 * -5.55473 ) - 6.98316 ) * nodeVar58 ) );
	nodeVar60 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar59 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar59 ) ) );
	nodeVar61 = ( vec3<f32>( 1.0 ) - nodeVar60 );
	nodeVar62 = nodeVar61;
	nodeVar63 = ( nodeVar54 * nodeVar62 );
	nodeVar64 = ( directDiffuse + nodeVar63 );
	directDiffuse = nodeVar64;
	nodeVar65 = normalize( ( nodeVar44 + normalize( ( - nodeVar3.xyz ) ) ) );
	nodeVar66 = clamp( dot( normalize( ( - nodeVar3.xyz ) ), nodeVar65 ), 0.0, 1.0 );
	nodeVar67 = exp2( ( ( ( nodeVar66 * -5.55473 ) - 6.98316 ) * nodeVar66 ) );
	nodeVar68 = ( Roughness * Roughness );
	nodeVar69 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar67 ) ) ) + vec3<f32>( ( 1.0 * nodeVar67 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar68, clamp( dot( nodeVar4.xyz, nodeVar44 ), 0.0, 1.0 ), clamp( dot( nodeVar4.xyz, normalize( ( - nodeVar3.xyz ) ) ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar68, clamp( dot( nodeVar4.xyz, nodeVar65 ), 0.0, 1.0 ) ) ) );
	nodeVar70 = ( nodeVar52 * nodeVar69 );
	nodeVar71 = ( nodeVar70 * multiScatteringCompensation );
	nodeVar72 = ( directSpecular + nodeVar71 );
	directSpecular = nodeVar72;
	nodeVar73 = ( render.nodeUniform16 - nodeVar3.xyz );
	nodeVar74 = normalize( nodeVar73 );
	nodeVar75 = dot( nodeVar4.xyz, nodeVar74 );

	if ( ( render.nodeUniform18 > 0.0 ) ) {

		nodeVar77 = length( nodeVar73 );
		nodeVar78 = ( nodeVar77 / render.nodeUniform18 );
		nodeVar79 = clamp( ( 1.0 - ( ( ( nodeVar78 * nodeVar78 ) * nodeVar78 ) * nodeVar78 ) ), 0.0, 1.0 );
		nodeVar76 = ( ( 1.0 / max( pow( nodeVar77, render.nodeUniform19 ), 0.01 ) ) * ( nodeVar79 * nodeVar79 ) );

	} else {

		nodeVar76 = ( 1.0 / max( pow( length( nodeVar73 ), render.nodeUniform19 ), 0.01 ) );

	}

	nodeVar80 = ( render.nodeUniform17 * vec3<f32>( nodeVar76 ) );
	nodeVar81 = ( vec3<f32>( clamp( nodeVar75, 0.0, 1.0 ) ) * nodeVar80 );
	nodeVar82 = nodeVar81;
	nodeVar83 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar84 = ( nodeVar82 * nodeVar83 );
	nodeVar85 = ( nodeVar74 + normalize( ( - nodeVar3.xyz ) ) );
	nodeVar86 = normalize( nodeVar85 );
	nodeVar87 = dot( normalize( ( - nodeVar3.xyz ) ), nodeVar86 );
	nodeVar88 = clamp( nodeVar87, 0.0, 1.0 );
	nodeVar89 = exp2( ( ( ( nodeVar88 * -5.55473 ) - 6.98316 ) * nodeVar88 ) );
	nodeVar90 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar89 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar89 ) ) );
	nodeVar91 = ( vec3<f32>( 1.0 ) - nodeVar90 );
	nodeVar92 = nodeVar91;
	nodeVar93 = ( nodeVar84 * nodeVar92 );
	nodeVar94 = ( directDiffuse + nodeVar93 );
	directDiffuse = nodeVar94;
	nodeVar95 = normalize( ( nodeVar74 + normalize( ( - nodeVar3.xyz ) ) ) );
	nodeVar96 = clamp( dot( normalize( ( - nodeVar3.xyz ) ), nodeVar95 ), 0.0, 1.0 );
	nodeVar97 = exp2( ( ( ( nodeVar96 * -5.55473 ) - 6.98316 ) * nodeVar96 ) );
	nodeVar98 = ( Roughness * Roughness );
	nodeVar99 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar97 ) ) ) + vec3<f32>( ( 1.0 * nodeVar97 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar98, clamp( dot( nodeVar4.xyz, nodeVar74 ), 0.0, 1.0 ), clamp( dot( nodeVar4.xyz, normalize( ( - nodeVar3.xyz ) ) ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar98, clamp( dot( nodeVar4.xyz, nodeVar95 ), 0.0, 1.0 ) ) ) );
	nodeVar100 = ( nodeVar82 * nodeVar99 );
	nodeVar101 = ( nodeVar100 * multiScatteringCompensation );
	nodeVar102 = ( directSpecular + nodeVar101 );
	directSpecular = nodeVar102;
	nodeVar103 = ( render.nodeUniform20 - nodeVar3.xyz );
	nodeVar104 = normalize( nodeVar103 );
	nodeVar105 = dot( nodeVar4.xyz, nodeVar104 );

	if ( ( render.nodeUniform22 > 0.0 ) ) {

		nodeVar107 = length( nodeVar103 );
		nodeVar108 = ( nodeVar107 / render.nodeUniform22 );
		nodeVar109 = clamp( ( 1.0 - ( ( ( nodeVar108 * nodeVar108 ) * nodeVar108 ) * nodeVar108 ) ), 0.0, 1.0 );
		nodeVar106 = ( ( 1.0 / max( pow( nodeVar107, render.nodeUniform23 ), 0.01 ) ) * ( nodeVar109 * nodeVar109 ) );

	} else {

		nodeVar106 = ( 1.0 / max( pow( length( nodeVar103 ), render.nodeUniform23 ), 0.01 ) );

	}

	nodeVar110 = ( render.nodeUniform21 * vec3<f32>( nodeVar106 ) );
	nodeVar111 = ( vec3<f32>( clamp( nodeVar105, 0.0, 1.0 ) ) * nodeVar110 );
	nodeVar112 = nodeVar111;
	nodeVar113 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar114 = ( nodeVar112 * nodeVar113 );
	nodeVar115 = ( nodeVar104 + normalize( ( - nodeVar3.xyz ) ) );
	nodeVar116 = normalize( nodeVar115 );
	nodeVar117 = dot( normalize( ( - nodeVar3.xyz ) ), nodeVar116 );
	nodeVar118 = clamp( nodeVar117, 0.0, 1.0 );
	nodeVar119 = exp2( ( ( ( nodeVar118 * -5.55473 ) - 6.98316 ) * nodeVar118 ) );
	nodeVar120 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar119 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar119 ) ) );
	nodeVar121 = ( vec3<f32>( 1.0 ) - nodeVar120 );
	nodeVar122 = nodeVar121;
	nodeVar123 = ( nodeVar114 * nodeVar122 );
	nodeVar124 = ( directDiffuse + nodeVar123 );
	directDiffuse = nodeVar124;
	nodeVar125 = normalize( ( nodeVar104 + normalize( ( - nodeVar3.xyz ) ) ) );
	nodeVar126 = clamp( dot( normalize( ( - nodeVar3.xyz ) ), nodeVar125 ), 0.0, 1.0 );
	nodeVar127 = exp2( ( ( ( nodeVar126 * -5.55473 ) - 6.98316 ) * nodeVar126 ) );
	nodeVar128 = ( Roughness * Roughness );
	nodeVar129 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar127 ) ) ) + vec3<f32>( ( 1.0 * nodeVar127 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar128, clamp( dot( nodeVar4.xyz, nodeVar104 ), 0.0, 1.0 ), clamp( dot( nodeVar4.xyz, normalize( ( - nodeVar3.xyz ) ) ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar128, clamp( dot( nodeVar4.xyz, nodeVar125 ), 0.0, 1.0 ) ) ) );
	nodeVar130 = ( nodeVar112 * nodeVar129 );
	nodeVar131 = ( nodeVar130 * multiScatteringCompensation );
	nodeVar132 = ( directSpecular + nodeVar131 );
	directSpecular = nodeVar132;
	nodeVar133 = ( render.nodeUniform24 - nodeVar3.xyz );
	nodeVar134 = normalize( nodeVar133 );
	nodeVar135 = dot( nodeVar4.xyz, nodeVar134 );

	if ( ( render.nodeUniform26 > 0.0 ) ) {

		nodeVar137 = length( nodeVar133 );
		nodeVar138 = ( nodeVar137 / render.nodeUniform26 );
		nodeVar139 = clamp( ( 1.0 - ( ( ( nodeVar138 * nodeVar138 ) * nodeVar138 ) * nodeVar138 ) ), 0.0, 1.0 );
		nodeVar136 = ( ( 1.0 / max( pow( nodeVar137, render.nodeUniform27 ), 0.01 ) ) * ( nodeVar139 * nodeVar139 ) );

	} else {

		nodeVar136 = ( 1.0 / max( pow( length( nodeVar133 ), render.nodeUniform27 ), 0.01 ) );

	}

	nodeVar140 = ( render.nodeUniform25 * vec3<f32>( nodeVar136 ) );
	nodeVar141 = ( vec3<f32>( clamp( nodeVar135, 0.0, 1.0 ) ) * nodeVar140 );
	nodeVar142 = nodeVar141;
	nodeVar143 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar144 = ( nodeVar142 * nodeVar143 );
	nodeVar145 = ( nodeVar134 + normalize( ( - nodeVar3.xyz ) ) );
	nodeVar146 = normalize( nodeVar145 );
	nodeVar147 = dot( normalize( ( - nodeVar3.xyz ) ), nodeVar146 );
	nodeVar148 = clamp( nodeVar147, 0.0, 1.0 );
	nodeVar149 = exp2( ( ( ( nodeVar148 * -5.55473 ) - 6.98316 ) * nodeVar148 ) );
	nodeVar150 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar149 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar149 ) ) );
	nodeVar151 = ( vec3<f32>( 1.0 ) - nodeVar150 );
	nodeVar152 = nodeVar151;
	nodeVar153 = ( nodeVar144 * nodeVar152 );
	nodeVar154 = ( directDiffuse + nodeVar153 );
	directDiffuse = nodeVar154;
	nodeVar155 = normalize( ( nodeVar134 + normalize( ( - nodeVar3.xyz ) ) ) );
	nodeVar156 = clamp( dot( normalize( ( - nodeVar3.xyz ) ), nodeVar155 ), 0.0, 1.0 );
	nodeVar157 = exp2( ( ( ( nodeVar156 * -5.55473 ) - 6.98316 ) * nodeVar156 ) );
	nodeVar158 = ( Roughness * Roughness );
	nodeVar159 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar157 ) ) ) + vec3<f32>( ( 1.0 * nodeVar157 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar158, clamp( dot( nodeVar4.xyz, nodeVar134 ), 0.0, 1.0 ), clamp( dot( nodeVar4.xyz, normalize( ( - nodeVar3.xyz ) ) ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar158, clamp( dot( nodeVar4.xyz, nodeVar155 ), 0.0, 1.0 ) ) ) );
	nodeVar160 = ( nodeVar142 * nodeVar159 );
	nodeVar161 = ( nodeVar160 * multiScatteringCompensation );
	nodeVar162 = ( directSpecular + nodeVar161 );
	directSpecular = nodeVar162;
	nodeVar163 = ( render.nodeUniform28 - nodeVar3.xyz );
	nodeVar164 = normalize( nodeVar163 );
	nodeVar165 = dot( nodeVar4.xyz, nodeVar164 );

	if ( ( render.nodeUniform30 > 0.0 ) ) {

		nodeVar167 = length( nodeVar163 );
		nodeVar168 = ( nodeVar167 / render.nodeUniform30 );
		nodeVar169 = clamp( ( 1.0 - ( ( ( nodeVar168 * nodeVar168 ) * nodeVar168 ) * nodeVar168 ) ), 0.0, 1.0 );
		nodeVar166 = ( ( 1.0 / max( pow( nodeVar167, render.nodeUniform31 ), 0.01 ) ) * ( nodeVar169 * nodeVar169 ) );

	} else {

		nodeVar166 = ( 1.0 / max( pow( length( nodeVar163 ), render.nodeUniform31 ), 0.01 ) );

	}

	nodeVar170 = ( render.nodeUniform29 * vec3<f32>( nodeVar166 ) );
	nodeVar171 = ( vec3<f32>( clamp( nodeVar165, 0.0, 1.0 ) ) * nodeVar170 );
	nodeVar172 = nodeVar171;
	nodeVar173 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar174 = ( nodeVar172 * nodeVar173 );
	nodeVar175 = ( nodeVar164 + normalize( ( - nodeVar3.xyz ) ) );
	nodeVar176 = normalize( nodeVar175 );
	nodeVar177 = dot( normalize( ( - nodeVar3.xyz ) ), nodeVar176 );
	nodeVar178 = clamp( nodeVar177, 0.0, 1.0 );
	nodeVar179 = exp2( ( ( ( nodeVar178 * -5.55473 ) - 6.98316 ) * nodeVar178 ) );
	nodeVar180 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar179 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar179 ) ) );
	nodeVar181 = ( vec3<f32>( 1.0 ) - nodeVar180 );
	nodeVar182 = nodeVar181;
	nodeVar183 = ( nodeVar174 * nodeVar182 );
	nodeVar184 = ( directDiffuse + nodeVar183 );
	directDiffuse = nodeVar184;
	nodeVar185 = normalize( ( nodeVar164 + normalize( ( - nodeVar3.xyz ) ) ) );
	nodeVar186 = clamp( dot( normalize( ( - nodeVar3.xyz ) ), nodeVar185 ), 0.0, 1.0 );
	nodeVar187 = exp2( ( ( ( nodeVar186 * -5.55473 ) - 6.98316 ) * nodeVar186 ) );
	nodeVar188 = ( Roughness * Roughness );
	nodeVar189 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar187 ) ) ) + vec3<f32>( ( 1.0 * nodeVar187 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar188, clamp( dot( nodeVar4.xyz, nodeVar164 ), 0.0, 1.0 ), clamp( dot( nodeVar4.xyz, normalize( ( - nodeVar3.xyz ) ) ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar188, clamp( dot( nodeVar4.xyz, nodeVar185 ), 0.0, 1.0 ) ) ) );
	nodeVar190 = ( nodeVar172 * nodeVar189 );
	nodeVar191 = ( nodeVar190 * multiScatteringCompensation );
	nodeVar192 = ( directSpecular + nodeVar191 );
	directSpecular = nodeVar192;
	nodeVar193 = ( render.nodeUniform32 - nodeVar3.xyz );
	nodeVar194 = normalize( nodeVar193 );
	nodeVar195 = dot( nodeVar4.xyz, nodeVar194 );

	if ( ( render.nodeUniform34 > 0.0 ) ) {

		nodeVar197 = length( nodeVar193 );
		nodeVar198 = ( nodeVar197 / render.nodeUniform34 );
		nodeVar199 = clamp( ( 1.0 - ( ( ( nodeVar198 * nodeVar198 ) * nodeVar198 ) * nodeVar198 ) ), 0.0, 1.0 );
		nodeVar196 = ( ( 1.0 / max( pow( nodeVar197, render.nodeUniform35 ), 0.01 ) ) * ( nodeVar199 * nodeVar199 ) );

	} else {

		nodeVar196 = ( 1.0 / max( pow( length( nodeVar193 ), render.nodeUniform35 ), 0.01 ) );

	}

	nodeVar200 = ( render.nodeUniform33 * vec3<f32>( nodeVar196 ) );
	nodeVar201 = ( vec3<f32>( clamp( nodeVar195, 0.0, 1.0 ) ) * nodeVar200 );
	nodeVar202 = nodeVar201;
	nodeVar203 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar204 = ( nodeVar202 * nodeVar203 );
	nodeVar205 = ( nodeVar194 + normalize( ( - nodeVar3.xyz ) ) );
	nodeVar206 = normalize( nodeVar205 );
	nodeVar207 = dot( normalize( ( - nodeVar3.xyz ) ), nodeVar206 );
	nodeVar208 = clamp( nodeVar207, 0.0, 1.0 );
	nodeVar209 = exp2( ( ( ( nodeVar208 * -5.55473 ) - 6.98316 ) * nodeVar208 ) );
	nodeVar210 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar209 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar209 ) ) );
	nodeVar211 = ( vec3<f32>( 1.0 ) - nodeVar210 );
	nodeVar212 = nodeVar211;
	nodeVar213 = ( nodeVar204 * nodeVar212 );
	nodeVar214 = ( directDiffuse + nodeVar213 );
	directDiffuse = nodeVar214;
	nodeVar215 = normalize( ( nodeVar194 + normalize( ( - nodeVar3.xyz ) ) ) );
	nodeVar216 = clamp( dot( normalize( ( - nodeVar3.xyz ) ), nodeVar215 ), 0.0, 1.0 );
	nodeVar217 = exp2( ( ( ( nodeVar216 * -5.55473 ) - 6.98316 ) * nodeVar216 ) );
	nodeVar218 = ( Roughness * Roughness );
	nodeVar219 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar217 ) ) ) + vec3<f32>( ( 1.0 * nodeVar217 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar218, clamp( dot( nodeVar4.xyz, nodeVar194 ), 0.0, 1.0 ), clamp( dot( nodeVar4.xyz, normalize( ( - nodeVar3.xyz ) ) ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar218, clamp( dot( nodeVar4.xyz, nodeVar215 ), 0.0, 1.0 ) ) ) );
	nodeVar220 = ( nodeVar202 * nodeVar219 );
	nodeVar221 = ( nodeVar220 * multiScatteringCompensation );
	nodeVar222 = ( directSpecular + nodeVar221 );
	directSpecular = nodeVar222;
	nodeVar223 = ( render.nodeUniform36 - nodeVar3.xyz );
	nodeVar224 = normalize( nodeVar223 );
	nodeVar225 = dot( nodeVar4.xyz, nodeVar224 );

	if ( ( render.nodeUniform38 > 0.0 ) ) {

		nodeVar227 = length( nodeVar223 );
		nodeVar228 = ( nodeVar227 / render.nodeUniform38 );
		nodeVar229 = clamp( ( 1.0 - ( ( ( nodeVar228 * nodeVar228 ) * nodeVar228 ) * nodeVar228 ) ), 0.0, 1.0 );
		nodeVar226 = ( ( 1.0 / max( pow( nodeVar227, render.nodeUniform39 ), 0.01 ) ) * ( nodeVar229 * nodeVar229 ) );

	} else {

		nodeVar226 = ( 1.0 / max( pow( length( nodeVar223 ), render.nodeUniform39 ), 0.01 ) );

	}

	nodeVar230 = ( render.nodeUniform37 * vec3<f32>( nodeVar226 ) );
	nodeVar231 = ( vec3<f32>( clamp( nodeVar225, 0.0, 1.0 ) ) * nodeVar230 );
	nodeVar232 = nodeVar231;
	nodeVar233 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar234 = ( nodeVar232 * nodeVar233 );
	nodeVar235 = ( nodeVar224 + normalize( ( - nodeVar3.xyz ) ) );
	nodeVar236 = normalize( nodeVar235 );
	nodeVar237 = dot( normalize( ( - nodeVar3.xyz ) ), nodeVar236 );
	nodeVar238 = clamp( nodeVar237, 0.0, 1.0 );
	nodeVar239 = exp2( ( ( ( nodeVar238 * -5.55473 ) - 6.98316 ) * nodeVar238 ) );
	nodeVar240 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar239 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar239 ) ) );
	nodeVar241 = ( vec3<f32>( 1.0 ) - nodeVar240 );
	nodeVar242 = nodeVar241;
	nodeVar243 = ( nodeVar234 * nodeVar242 );
	nodeVar244 = ( directDiffuse + nodeVar243 );
	directDiffuse = nodeVar244;
	nodeVar245 = normalize( ( nodeVar224 + normalize( ( - nodeVar3.xyz ) ) ) );
	nodeVar246 = clamp( dot( normalize( ( - nodeVar3.xyz ) ), nodeVar245 ), 0.0, 1.0 );
	nodeVar247 = exp2( ( ( ( nodeVar246 * -5.55473 ) - 6.98316 ) * nodeVar246 ) );
	nodeVar248 = ( Roughness * Roughness );
	nodeVar249 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar247 ) ) ) + vec3<f32>( ( 1.0 * nodeVar247 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar248, clamp( dot( nodeVar4.xyz, nodeVar224 ), 0.0, 1.0 ), clamp( dot( nodeVar4.xyz, normalize( ( - nodeVar3.xyz ) ) ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar248, clamp( dot( nodeVar4.xyz, nodeVar245 ), 0.0, 1.0 ) ) ) );
	nodeVar250 = ( nodeVar232 * nodeVar249 );
	nodeVar251 = ( nodeVar250 * multiScatteringCompensation );
	nodeVar252 = ( directSpecular + nodeVar251 );
	directSpecular = nodeVar252;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar253 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform40 );
	nodeVar254 = floor( nodeVar253 );
	nodeVar255 = nodeVar254;
	nodeVar256 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - normalize( ( - nodeVar3.xyz ) ) ), nodeVar4.xyz ), nodeVar4.xyz, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar257 = getFace( ( object.nodeUniform41 * vec4<f32>( vec3<f32>( nodeVar256.x, ( - nodeVar256.y ), nodeVar256.z ), 1.0 ) ).xyz );
	nodeVar258 = max( ( 4.0 - nodeVar255 ), 0.0 );
	nodeVar255 = max( nodeVar255, 4.0 );
	nodeVar259 = exp2( nodeVar255 );
	nodeVar260 = ( ( getUV( ( object.nodeUniform41 * vec4<f32>( vec3<f32>( nodeVar256.x, ( - nodeVar256.y ), nodeVar256.z ), 1.0 ) ).xyz, nodeVar257 ) * vec2<f32>( ( nodeVar259 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar257 > 2.0 ) ) {

		nodeVar260.y = ( nodeVar260.y + nodeVar259 );
		nodeVar257 = ( nodeVar257 - 3.0 );
		

	}

	nodeVar260.x = ( nodeVar260.x + ( nodeVar257 * nodeVar259 ) );
	nodeVar260.x = ( nodeVar260.x + ( nodeVar258 * ( 3.0 * 16.0 ) ) );
	nodeVar260.y = ( nodeVar260.y + ( 4.0 * ( exp2( object.nodeUniform40 ) - nodeVar259 ) ) );
	nodeVar260.x = ( nodeVar260.x * object.nodeUniform43 );
	nodeVar260.y = ( nodeVar260.y * object.nodeUniform44 );
	nodeVar261 = textureSampleGrad( nodeUniform45, nodeUniform45_sampler, nodeVar260, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar262 = nodeVar261.xyz;
	nodeVar263 = fract( nodeVar253 );

	if ( ( nodeVar263 != 0.0 ) ) {

		nodeVar264 = ( nodeVar254 + 1.0 );
		nodeVar265 = getFace( ( object.nodeUniform41 * vec4<f32>( vec3<f32>( nodeVar256.x, ( - nodeVar256.y ), nodeVar256.z ), 1.0 ) ).xyz );
		nodeVar266 = max( ( 4.0 - nodeVar264 ), 0.0 );
		nodeVar264 = max( nodeVar264, 4.0 );
		nodeVar267 = exp2( nodeVar264 );
		nodeVar268 = ( ( getUV( ( object.nodeUniform41 * vec4<f32>( vec3<f32>( nodeVar256.x, ( - nodeVar256.y ), nodeVar256.z ), 1.0 ) ).xyz, nodeVar265 ) * vec2<f32>( ( nodeVar267 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar265 > 2.0 ) ) {

			nodeVar268.y = ( nodeVar268.y + nodeVar267 );
			nodeVar265 = ( nodeVar265 - 3.0 );
			

		}

		nodeVar268.x = ( nodeVar268.x + ( nodeVar265 * nodeVar267 ) );
		nodeVar268.x = ( nodeVar268.x + ( nodeVar266 * ( 3.0 * 16.0 ) ) );
		nodeVar268.y = ( nodeVar268.y + ( 4.0 * ( exp2( object.nodeUniform40 ) - nodeVar267 ) ) );
		nodeVar268.x = ( nodeVar268.x * object.nodeUniform43 );
		nodeVar268.y = ( nodeVar268.y * object.nodeUniform44 );
		nodeVar269 = textureSampleGrad( nodeUniform45, nodeUniform45_sampler, nodeVar268, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar270 = nodeVar269.xyz;
		nodeVar262 = mix( nodeVar262, nodeVar270, nodeVar263 );
		

	}

	nodeVar271 = ( radiance + ( nodeVar262 * vec3<f32>( object.nodeUniform46 ) ) );
	radiance = nodeVar271;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar272 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform40 );
	nodeVar273 = floor( nodeVar272 );
	nodeVar274 = nodeVar273;
	normalWorld = normalize( ( vec4<f32>( nodeVar4.xyz, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar275 = getFace( ( object.nodeUniform41 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar276 = max( ( 4.0 - nodeVar274 ), 0.0 );
	nodeVar274 = max( nodeVar274, 4.0 );
	nodeVar277 = exp2( nodeVar274 );
	nodeVar278 = ( ( getUV( ( object.nodeUniform41 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar275 ) * vec2<f32>( ( nodeVar277 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar275 > 2.0 ) ) {

		nodeVar278.y = ( nodeVar278.y + nodeVar277 );
		nodeVar275 = ( nodeVar275 - 3.0 );
		

	}

	nodeVar278.x = ( nodeVar278.x + ( nodeVar275 * nodeVar277 ) );
	nodeVar278.x = ( nodeVar278.x + ( nodeVar276 * ( 3.0 * 16.0 ) ) );
	nodeVar278.y = ( nodeVar278.y + ( 4.0 * ( exp2( object.nodeUniform40 ) - nodeVar277 ) ) );
	nodeVar278.x = ( nodeVar278.x * object.nodeUniform43 );
	nodeVar278.y = ( nodeVar278.y * object.nodeUniform44 );
	nodeVar279 = textureSampleGrad( nodeUniform45, nodeUniform45_sampler, nodeVar278, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar280 = nodeVar279.xyz;
	nodeVar281 = fract( nodeVar272 );

	if ( ( nodeVar281 != 0.0 ) ) {

		nodeVar282 = ( nodeVar273 + 1.0 );
		nodeVar283 = getFace( ( object.nodeUniform41 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar284 = max( ( 4.0 - nodeVar282 ), 0.0 );
		nodeVar282 = max( nodeVar282, 4.0 );
		nodeVar285 = exp2( nodeVar282 );
		nodeVar286 = ( ( getUV( ( object.nodeUniform41 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar283 ) * vec2<f32>( ( nodeVar285 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar283 > 2.0 ) ) {

			nodeVar286.y = ( nodeVar286.y + nodeVar285 );
			nodeVar283 = ( nodeVar283 - 3.0 );
			

		}

		nodeVar286.x = ( nodeVar286.x + ( nodeVar283 * nodeVar285 ) );
		nodeVar286.x = ( nodeVar286.x + ( nodeVar284 * ( 3.0 * 16.0 ) ) );
		nodeVar286.y = ( nodeVar286.y + ( 4.0 * ( exp2( object.nodeUniform40 ) - nodeVar285 ) ) );
		nodeVar286.x = ( nodeVar286.x * object.nodeUniform43 );
		nodeVar286.y = ( nodeVar286.y * object.nodeUniform44 );
		nodeVar287 = textureSampleGrad( nodeUniform45, nodeUniform45_sampler, nodeVar286, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar288 = nodeVar287.xyz;
		nodeVar280 = mix( nodeVar280, nodeVar288, nodeVar281 );
		

	}

	nodeVar289 = ( iblIrradiance + ( ( nodeVar280 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform46 ) ) );
	iblIrradiance = nodeVar289;
	nodeVar290 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar291 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar292 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar293 = ( SpecularF90 * dfg.y );
	nodeVar294 = ( nodeVar292 + vec3<f32>( nodeVar293 ) );
	nodeVar295 = ( nodeVar290 + nodeVar294 );
	nodeVar290 = nodeVar295;
	nodeVar296 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar297 = nodeVar296;
	nodeVar298 = ( nodeVar297 * vec3<f32>( 0.047619 ) );
	nodeVar299 = ( SpecularColor + nodeVar298 );
	nodeVar300 = ( nodeVar294 * nodeVar299 );
	nodeVar301 = ( dfg.x + dfg.y );
	nodeVar302 = ( 1.0 - nodeVar301 );
	nodeVar303 = nodeVar302;
	nodeVar304 = ( vec3<f32>( nodeVar303 ) * nodeVar299 );
	nodeVar305 = ( vec3<f32>( 1.0 ) - nodeVar304 );
	nodeVar306 = nodeVar305;
	nodeVar307 = ( nodeVar300 / nodeVar306 );
	nodeVar308 = ( nodeVar307 * vec3<f32>( nodeVar303 ) );
	nodeVar309 = ( nodeVar291 + nodeVar308 );
	nodeVar291 = nodeVar309;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar310 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar311 = ( irradiance * nodeVar310 );
	nodeVar312 = ( nodeVar290 + nodeVar291 );
	nodeVar313 = ( vec3<f32>( 1.0 ) - nodeVar312 );
	nodeVar314 = nodeVar313;
	nodeVar315 = ( nodeVar311 * nodeVar314 );
	nodeVar316 = nodeVar315;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar317 = ( indirectDiffuse + nodeVar316 );
	indirectDiffuse = nodeVar317;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar318 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar319 = ( SpecularF90 * dfg.y );
	nodeVar320 = ( nodeVar318 + vec3<f32>( nodeVar319 ) );
	nodeVar321 = ( singleScatteringDielectric + nodeVar320 );
	singleScatteringDielectric = nodeVar321;
	nodeVar322 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar323 = nodeVar322;
	nodeVar324 = ( nodeVar323 * vec3<f32>( 0.047619 ) );
	nodeVar325 = ( SpecularColor + nodeVar324 );
	nodeVar326 = ( nodeVar320 * nodeVar325 );
	nodeVar327 = ( dfg.x + dfg.y );
	nodeVar328 = ( 1.0 - nodeVar327 );
	nodeVar329 = nodeVar328;
	nodeVar330 = ( vec3<f32>( nodeVar329 ) * nodeVar325 );
	nodeVar331 = ( vec3<f32>( 1.0 ) - nodeVar330 );
	nodeVar332 = nodeVar331;
	nodeVar333 = ( nodeVar326 / nodeVar332 );
	nodeVar334 = ( nodeVar333 * vec3<f32>( nodeVar329 ) );
	nodeVar335 = ( multiScatteringDielectric + nodeVar334 );
	multiScatteringDielectric = nodeVar335;
	nodeVar336 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar337 = ( SpecularF90 * dfg.y );
	nodeVar338 = ( nodeVar336 + vec3<f32>( nodeVar337 ) );
	nodeVar339 = ( singleScatteringMetallic + nodeVar338 );
	singleScatteringMetallic = nodeVar339;
	nodeVar340 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar341 = nodeVar340;
	nodeVar342 = ( nodeVar341 * vec3<f32>( 0.047619 ) );
	nodeVar343 = ( DiffuseColor.xyz + nodeVar342 );
	nodeVar344 = ( nodeVar338 * nodeVar343 );
	nodeVar345 = ( dfg.x + dfg.y );
	nodeVar346 = ( 1.0 - nodeVar345 );
	nodeVar347 = nodeVar346;
	nodeVar348 = ( vec3<f32>( nodeVar347 ) * nodeVar343 );
	nodeVar349 = ( vec3<f32>( 1.0 ) - nodeVar348 );
	nodeVar350 = nodeVar349;
	nodeVar351 = ( nodeVar344 / nodeVar350 );
	nodeVar352 = ( nodeVar351 * vec3<f32>( nodeVar347 ) );
	nodeVar353 = ( multiScatteringMetallic + nodeVar352 );
	multiScatteringMetallic = nodeVar353;
	nodeVar354 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar355 = ( radiance * nodeVar354 );
	nodeVar356 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar357 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar358 = ( nodeVar356 * nodeVar357 );
	nodeVar359 = ( nodeVar355 + nodeVar358 );
	nodeVar360 = nodeVar359;
	nodeVar361 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar362 = ( vec3<f32>( 1.0 ) - nodeVar361 );
	nodeVar363 = nodeVar362;
	nodeVar364 = ( DiffuseContribution * nodeVar363 );
	nodeVar365 = ( nodeVar364 * nodeVar357 );
	nodeVar366 = nodeVar365;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar367 = ( indirectSpecular + nodeVar360 );
	indirectSpecular = nodeVar367;
	nodeVar368 = ( indirectDiffuse + nodeVar366 );
	indirectDiffuse = nodeVar368;
	ambientOcclusion = 1.0;
	nodeVar369 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar369;
	nodeVar370 = dot( nodeVar4.xyz, normalize( ( - nodeVar3.xyz ) ) );
	nodeVar371 = ( clamp( nodeVar370, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar372 = ( Roughness * -16.0 );
	nodeVar373 = ( 1.0 - nodeVar372 );
	nodeVar374 = nodeVar373;
	nodeVar375 = ( - nodeVar374 );
	nodeVar376 = exp2( nodeVar375 );
	nodeVar377 = pow( nodeVar371, nodeVar376 );
	nodeVar378 = ( 1.0 - nodeVar377 );
	nodeVar379 = nodeVar378;
	nodeVar380 = ( ambientOcclusion - nodeVar379 );
	nodeVar381 = ( indirectSpecular * vec3<f32>( clamp( nodeVar380, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar381;
	nodeVar382 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar382;
	nodeVar383 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar383;
	nodeVar384 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar384;
	nodeVar385 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar385;

	// result

	output.color = nodeVar385;

	return output;

}
