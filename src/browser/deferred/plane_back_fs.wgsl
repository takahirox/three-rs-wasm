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
@binding( 3 ) @group( 1 ) var nodeUniform47_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform47 : texture_2d<f32>;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform3 : f32,
	nodeUniform5 : mat3x3<f32>,
	nodeUniform6 : vec3<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : mat4x4<f32>,
	nodeUniform42 : f32,
	nodeUniform43 : mat4x4<f32>,
	nodeUniform45 : f32,
	nodeUniform46 : f32,
	nodeUniform48 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform11 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform13 : f32,
	nodeUniform15 : vec3<f32>,
	nodeUniform16 : f32,
	nodeUniform17 : f32,
	nodeUniform19 : vec3<f32>,
	nodeUniform20 : f32,
	nodeUniform21 : f32,
	nodeUniform23 : vec3<f32>,
	nodeUniform24 : f32,
	nodeUniform25 : f32,
	nodeUniform27 : vec3<f32>,
	nodeUniform28 : f32,
	nodeUniform29 : f32,
	nodeUniform31 : vec3<f32>,
	nodeUniform32 : f32,
	nodeUniform33 : f32,
	nodeUniform35 : vec3<f32>,
	nodeUniform36 : f32,
	nodeUniform37 : f32,
	nodeUniform39 : vec3<f32>,
	nodeUniform40 : f32,
	nodeUniform41 : f32,
	nodeUniform10 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform18 : vec3<f32>,
	nodeUniform22 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform30 : vec3<f32>,
	nodeUniform34 : vec3<f32>,
	nodeUniform38 : vec3<f32>,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	cameraWorldMatrix : mat4x4<f32>
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
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : vec3<f32>;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : vec3<f32>;
var<private> nodeVar77 : vec3<f32>;
var<private> nodeVar78 : vec3<f32>;
var<private> nodeVar79 : vec3<f32>;
var<private> nodeVar80 : vec3<f32>;
var<private> nodeVar81 : vec3<f32>;
var<private> nodeVar82 : vec3<f32>;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : vec3<f32>;
var<private> nodeVar87 : vec3<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : vec3<f32>;
var<private> nodeVar91 : vec3<f32>;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : vec3<f32>;
var<private> nodeVar96 : vec3<f32>;
var<private> nodeVar97 : vec3<f32>;
var<private> nodeVar98 : vec3<f32>;
var<private> nodeVar99 : vec3<f32>;
var<private> nodeVar100 : vec3<f32>;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : f32;
var<private> nodeVar106 : vec3<f32>;
var<private> nodeVar107 : vec3<f32>;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : vec3<f32>;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> nodeVar122 : f32;
var<private> nodeVar123 : f32;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : vec3<f32>;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : f32;
var<private> nodeVar132 : f32;
var<private> nodeVar133 : f32;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : f32;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : vec3<f32>;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : vec3<f32>;
var<private> nodeVar147 : vec3<f32>;
var<private> nodeVar148 : vec3<f32>;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec3<f32>;
var<private> nodeVar151 : vec3<f32>;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : f32;
var<private> nodeVar155 : vec3<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : vec3<f32>;
var<private> nodeVar158 : vec3<f32>;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : f32;
var<private> nodeVar162 : f32;
var<private> nodeVar163 : f32;
var<private> nodeVar164 : f32;
var<private> nodeVar165 : f32;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec3<f32>;
var<private> nodeVar168 : vec3<f32>;
var<private> nodeVar169 : vec3<f32>;
var<private> nodeVar170 : vec3<f32>;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec3<f32>;
var<private> nodeVar173 : f32;
var<private> nodeVar174 : f32;
var<private> nodeVar175 : f32;
var<private> nodeVar176 : vec3<f32>;
var<private> nodeVar177 : vec3<f32>;
var<private> nodeVar178 : vec3<f32>;
var<private> nodeVar179 : vec3<f32>;
var<private> nodeVar180 : vec3<f32>;
var<private> nodeVar181 : vec3<f32>;
var<private> nodeVar182 : f32;
var<private> nodeVar183 : f32;
var<private> nodeVar184 : f32;
var<private> nodeVar185 : vec3<f32>;
var<private> nodeVar186 : vec3<f32>;
var<private> nodeVar187 : vec3<f32>;
var<private> nodeVar188 : vec3<f32>;
var<private> nodeVar189 : vec3<f32>;
var<private> nodeVar190 : vec3<f32>;
var<private> nodeVar191 : f32;
var<private> nodeVar192 : f32;
var<private> nodeVar193 : f32;
var<private> nodeVar194 : f32;
var<private> nodeVar195 : f32;
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
var<private> nodeVar206 : vec3<f32>;
var<private> nodeVar207 : vec3<f32>;
var<private> nodeVar208 : vec3<f32>;
var<private> nodeVar209 : vec3<f32>;
var<private> nodeVar210 : vec3<f32>;
var<private> nodeVar211 : vec3<f32>;
var<private> nodeVar212 : f32;
var<private> nodeVar213 : f32;
var<private> nodeVar214 : f32;
var<private> nodeVar215 : vec3<f32>;
var<private> nodeVar216 : vec3<f32>;
var<private> nodeVar217 : vec3<f32>;
var<private> nodeVar218 : vec3<f32>;
var<private> nodeVar219 : vec3<f32>;
var<private> nodeVar220 : vec3<f32>;
var<private> nodeVar221 : f32;
var<private> nodeVar222 : f32;
var<private> nodeVar223 : f32;
var<private> nodeVar224 : f32;
var<private> nodeVar225 : f32;
var<private> nodeVar226 : vec3<f32>;
var<private> nodeVar227 : vec3<f32>;
var<private> nodeVar228 : vec3<f32>;
var<private> nodeVar229 : vec3<f32>;
var<private> nodeVar230 : vec3<f32>;
var<private> nodeVar231 : vec3<f32>;
var<private> nodeVar232 : vec3<f32>;
var<private> nodeVar233 : f32;
var<private> nodeVar234 : f32;
var<private> nodeVar235 : f32;
var<private> nodeVar236 : vec3<f32>;
var<private> nodeVar237 : vec3<f32>;
var<private> nodeVar238 : vec3<f32>;
var<private> nodeVar239 : vec3<f32>;
var<private> nodeVar240 : vec3<f32>;
var<private> nodeVar241 : vec3<f32>;
var<private> nodeVar242 : f32;
var<private> nodeVar243 : f32;
var<private> nodeVar244 : f32;
var<private> nodeVar245 : vec3<f32>;
var<private> nodeVar246 : vec3<f32>;
var<private> nodeVar247 : vec3<f32>;
var<private> nodeVar248 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar249 : f32;
var<private> nodeVar250 : f32;
var<private> nodeVar251 : f32;
var<private> nodeVar252 : vec3<f32>;
var<private> nodeVar253 : f32;
var<private> nodeVar254 : f32;
var<private> nodeVar255 : f32;
var<private> nodeVar256 : vec2<f32>;
var<private> nodeVar257 : vec4<f32>;
var<private> nodeVar258 : vec3<f32>;
var<private> nodeVar259 : f32;
var<private> nodeVar260 : f32;
var<private> nodeVar261 : f32;
var<private> nodeVar262 : f32;
var<private> nodeVar263 : f32;
var<private> nodeVar264 : vec2<f32>;
var<private> nodeVar265 : vec4<f32>;
var<private> nodeVar266 : vec3<f32>;
var<private> nodeVar267 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar268 : f32;
var<private> nodeVar269 : f32;
var<private> nodeVar270 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar271 : f32;
var<private> nodeVar272 : f32;
var<private> nodeVar273 : f32;
var<private> nodeVar274 : vec2<f32>;
var<private> nodeVar275 : vec4<f32>;
var<private> nodeVar276 : vec3<f32>;
var<private> nodeVar277 : f32;
var<private> nodeVar278 : f32;
var<private> nodeVar279 : f32;
var<private> nodeVar280 : f32;
var<private> nodeVar281 : f32;
var<private> nodeVar282 : vec2<f32>;
var<private> nodeVar283 : vec4<f32>;
var<private> nodeVar284 : vec3<f32>;
var<private> nodeVar285 : vec3<f32>;
var<private> nodeVar286 : vec3<f32>;
var<private> nodeVar287 : vec3<f32>;
var<private> nodeVar288 : vec3<f32>;
var<private> nodeVar289 : f32;
var<private> nodeVar290 : vec3<f32>;
var<private> nodeVar291 : vec3<f32>;
var<private> nodeVar292 : vec3<f32>;
var<private> nodeVar293 : vec3<f32>;
var<private> nodeVar294 : vec3<f32>;
var<private> nodeVar295 : vec3<f32>;
var<private> nodeVar296 : vec3<f32>;
var<private> nodeVar297 : f32;
var<private> nodeVar298 : f32;
var<private> nodeVar299 : f32;
var<private> nodeVar300 : vec3<f32>;
var<private> nodeVar301 : vec3<f32>;
var<private> nodeVar302 : vec3<f32>;
var<private> nodeVar303 : vec3<f32>;
var<private> nodeVar304 : vec3<f32>;
var<private> nodeVar305 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar306 : vec3<f32>;
var<private> nodeVar307 : vec3<f32>;
var<private> nodeVar308 : vec3<f32>;
var<private> nodeVar309 : vec3<f32>;
var<private> nodeVar310 : vec3<f32>;
var<private> nodeVar311 : vec3<f32>;
var<private> nodeVar312 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar313 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
var<private> nodeVar314 : vec3<f32>;
var<private> nodeVar315 : f32;
var<private> nodeVar316 : vec3<f32>;
var<private> nodeVar317 : vec3<f32>;
var<private> nodeVar318 : vec3<f32>;
var<private> nodeVar319 : vec3<f32>;
var<private> nodeVar320 : vec3<f32>;
var<private> nodeVar321 : vec3<f32>;
var<private> nodeVar322 : vec3<f32>;
var<private> nodeVar323 : f32;
var<private> nodeVar324 : f32;
var<private> nodeVar325 : f32;
var<private> nodeVar326 : vec3<f32>;
var<private> nodeVar327 : vec3<f32>;
var<private> nodeVar328 : vec3<f32>;
var<private> nodeVar329 : vec3<f32>;
var<private> nodeVar330 : vec3<f32>;
var<private> nodeVar331 : vec3<f32>;
var<private> nodeVar332 : vec3<f32>;
var<private> nodeVar333 : f32;
var<private> nodeVar334 : vec3<f32>;
var<private> nodeVar335 : vec3<f32>;
var<private> nodeVar336 : vec3<f32>;
var<private> nodeVar337 : vec3<f32>;
var<private> nodeVar338 : vec3<f32>;
var<private> nodeVar339 : vec3<f32>;
var<private> nodeVar340 : vec3<f32>;
var<private> nodeVar341 : f32;
var<private> nodeVar342 : f32;
var<private> nodeVar343 : f32;
var<private> nodeVar344 : vec3<f32>;
var<private> nodeVar345 : vec3<f32>;
var<private> nodeVar346 : vec3<f32>;
var<private> nodeVar347 : vec3<f32>;
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
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar363 : vec3<f32>;
var<private> nodeVar364 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar365 : vec3<f32>;
var<private> nodeVar366 : f32;
var<private> nodeVar367 : f32;
var<private> nodeVar368 : f32;
var<private> nodeVar369 : f32;
var<private> nodeVar370 : f32;
var<private> nodeVar371 : f32;
var<private> nodeVar372 : f32;
var<private> nodeVar373 : f32;
var<private> nodeVar374 : f32;
var<private> nodeVar375 : f32;
var<private> nodeVar376 : f32;
var<private> nodeVar377 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar378 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar379 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar380 : vec3<f32>;
var<private> nodeVar381 : vec4<f32>;

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
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	Metalness = object.nodeUniform2;
	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar0 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( object.nodeUniform3, 0.0525 ) + max( max( nodeVar0.x, nodeVar0.y ), nodeVar0.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - object.nodeUniform2 ) ) );
	EmissiveColor = ( object.nodeUniform6 * vec3<f32>( object.nodeUniform7 ) );
	NORMAL_normalView = ( normalViewGeometry * vec3<f32>( -1.0 ) );
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
	nodeVar69 = ( render.nodeUniform18 - v_positionView );
	nodeVar70 = normalize( nodeVar69 );
	nodeVar71 = dot( normalView, nodeVar70 );

	if ( ( render.nodeUniform20 > 0.0 ) ) {

		nodeVar73 = length( nodeVar69 );
		nodeVar74 = ( nodeVar73 / render.nodeUniform20 );
		nodeVar75 = clamp( ( 1.0 - ( ( ( nodeVar74 * nodeVar74 ) * nodeVar74 ) * nodeVar74 ) ), 0.0, 1.0 );
		nodeVar72 = ( ( 1.0 / max( pow( nodeVar73, render.nodeUniform21 ), 0.01 ) ) * ( nodeVar75 * nodeVar75 ) );

	} else {

		nodeVar72 = ( 1.0 / max( pow( length( nodeVar69 ), render.nodeUniform21 ), 0.01 ) );

	}

	nodeVar76 = ( render.nodeUniform19 * vec3<f32>( nodeVar72 ) );
	nodeVar77 = ( vec3<f32>( clamp( nodeVar71, 0.0, 1.0 ) ) * nodeVar76 );
	nodeVar78 = nodeVar77;
	nodeVar79 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar80 = ( nodeVar78 * nodeVar79 );
	nodeVar81 = ( nodeVar70 + positionViewDirection );
	nodeVar82 = normalize( nodeVar81 );
	nodeVar83 = dot( positionViewDirection, nodeVar82 );
	nodeVar84 = clamp( nodeVar83, 0.0, 1.0 );
	nodeVar85 = exp2( ( ( ( nodeVar84 * -5.55473 ) - 6.98316 ) * nodeVar84 ) );
	nodeVar86 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar85 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar85 ) ) );
	nodeVar87 = ( vec3<f32>( 1.0 ) - nodeVar86 );
	nodeVar88 = nodeVar87;
	nodeVar89 = ( nodeVar80 * nodeVar88 );
	nodeVar90 = ( directDiffuse + nodeVar89 );
	directDiffuse = nodeVar90;
	nodeVar91 = normalize( ( nodeVar70 + positionViewDirection ) );
	nodeVar92 = clamp( dot( positionViewDirection, nodeVar91 ), 0.0, 1.0 );
	nodeVar93 = exp2( ( ( ( nodeVar92 * -5.55473 ) - 6.98316 ) * nodeVar92 ) );
	nodeVar94 = ( Roughness * Roughness );
	nodeVar95 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar93 ) ) ) + vec3<f32>( ( 1.0 * nodeVar93 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar94, clamp( dot( normalView, nodeVar70 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar94, clamp( dot( normalView, nodeVar91 ), 0.0, 1.0 ) ) ) );
	nodeVar96 = ( nodeVar78 * nodeVar95 );
	nodeVar97 = ( nodeVar96 * multiScatteringCompensation );
	nodeVar98 = ( directSpecular + nodeVar97 );
	directSpecular = nodeVar98;
	nodeVar99 = ( render.nodeUniform22 - v_positionView );
	nodeVar100 = normalize( nodeVar99 );
	nodeVar101 = dot( normalView, nodeVar100 );

	if ( ( render.nodeUniform24 > 0.0 ) ) {

		nodeVar103 = length( nodeVar99 );
		nodeVar104 = ( nodeVar103 / render.nodeUniform24 );
		nodeVar105 = clamp( ( 1.0 - ( ( ( nodeVar104 * nodeVar104 ) * nodeVar104 ) * nodeVar104 ) ), 0.0, 1.0 );
		nodeVar102 = ( ( 1.0 / max( pow( nodeVar103, render.nodeUniform25 ), 0.01 ) ) * ( nodeVar105 * nodeVar105 ) );

	} else {

		nodeVar102 = ( 1.0 / max( pow( length( nodeVar99 ), render.nodeUniform25 ), 0.01 ) );

	}

	nodeVar106 = ( render.nodeUniform23 * vec3<f32>( nodeVar102 ) );
	nodeVar107 = ( vec3<f32>( clamp( nodeVar101, 0.0, 1.0 ) ) * nodeVar106 );
	nodeVar108 = nodeVar107;
	nodeVar109 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar110 = ( nodeVar108 * nodeVar109 );
	nodeVar111 = ( nodeVar100 + positionViewDirection );
	nodeVar112 = normalize( nodeVar111 );
	nodeVar113 = dot( positionViewDirection, nodeVar112 );
	nodeVar114 = clamp( nodeVar113, 0.0, 1.0 );
	nodeVar115 = exp2( ( ( ( nodeVar114 * -5.55473 ) - 6.98316 ) * nodeVar114 ) );
	nodeVar116 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar115 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar115 ) ) );
	nodeVar117 = ( vec3<f32>( 1.0 ) - nodeVar116 );
	nodeVar118 = nodeVar117;
	nodeVar119 = ( nodeVar110 * nodeVar118 );
	nodeVar120 = ( directDiffuse + nodeVar119 );
	directDiffuse = nodeVar120;
	nodeVar121 = normalize( ( nodeVar100 + positionViewDirection ) );
	nodeVar122 = clamp( dot( positionViewDirection, nodeVar121 ), 0.0, 1.0 );
	nodeVar123 = exp2( ( ( ( nodeVar122 * -5.55473 ) - 6.98316 ) * nodeVar122 ) );
	nodeVar124 = ( Roughness * Roughness );
	nodeVar125 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar123 ) ) ) + vec3<f32>( ( 1.0 * nodeVar123 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar124, clamp( dot( normalView, nodeVar100 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar124, clamp( dot( normalView, nodeVar121 ), 0.0, 1.0 ) ) ) );
	nodeVar126 = ( nodeVar108 * nodeVar125 );
	nodeVar127 = ( nodeVar126 * multiScatteringCompensation );
	nodeVar128 = ( directSpecular + nodeVar127 );
	directSpecular = nodeVar128;
	nodeVar129 = ( render.nodeUniform26 - v_positionView );
	nodeVar130 = normalize( nodeVar129 );
	nodeVar131 = dot( normalView, nodeVar130 );

	if ( ( render.nodeUniform28 > 0.0 ) ) {

		nodeVar133 = length( nodeVar129 );
		nodeVar134 = ( nodeVar133 / render.nodeUniform28 );
		nodeVar135 = clamp( ( 1.0 - ( ( ( nodeVar134 * nodeVar134 ) * nodeVar134 ) * nodeVar134 ) ), 0.0, 1.0 );
		nodeVar132 = ( ( 1.0 / max( pow( nodeVar133, render.nodeUniform29 ), 0.01 ) ) * ( nodeVar135 * nodeVar135 ) );

	} else {

		nodeVar132 = ( 1.0 / max( pow( length( nodeVar129 ), render.nodeUniform29 ), 0.01 ) );

	}

	nodeVar136 = ( render.nodeUniform27 * vec3<f32>( nodeVar132 ) );
	nodeVar137 = ( vec3<f32>( clamp( nodeVar131, 0.0, 1.0 ) ) * nodeVar136 );
	nodeVar138 = nodeVar137;
	nodeVar139 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar140 = ( nodeVar138 * nodeVar139 );
	nodeVar141 = ( nodeVar130 + positionViewDirection );
	nodeVar142 = normalize( nodeVar141 );
	nodeVar143 = dot( positionViewDirection, nodeVar142 );
	nodeVar144 = clamp( nodeVar143, 0.0, 1.0 );
	nodeVar145 = exp2( ( ( ( nodeVar144 * -5.55473 ) - 6.98316 ) * nodeVar144 ) );
	nodeVar146 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar145 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar145 ) ) );
	nodeVar147 = ( vec3<f32>( 1.0 ) - nodeVar146 );
	nodeVar148 = nodeVar147;
	nodeVar149 = ( nodeVar140 * nodeVar148 );
	nodeVar150 = ( directDiffuse + nodeVar149 );
	directDiffuse = nodeVar150;
	nodeVar151 = normalize( ( nodeVar130 + positionViewDirection ) );
	nodeVar152 = clamp( dot( positionViewDirection, nodeVar151 ), 0.0, 1.0 );
	nodeVar153 = exp2( ( ( ( nodeVar152 * -5.55473 ) - 6.98316 ) * nodeVar152 ) );
	nodeVar154 = ( Roughness * Roughness );
	nodeVar155 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar153 ) ) ) + vec3<f32>( ( 1.0 * nodeVar153 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar154, clamp( dot( normalView, nodeVar130 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar154, clamp( dot( normalView, nodeVar151 ), 0.0, 1.0 ) ) ) );
	nodeVar156 = ( nodeVar138 * nodeVar155 );
	nodeVar157 = ( nodeVar156 * multiScatteringCompensation );
	nodeVar158 = ( directSpecular + nodeVar157 );
	directSpecular = nodeVar158;
	nodeVar159 = ( render.nodeUniform30 - v_positionView );
	nodeVar160 = normalize( nodeVar159 );
	nodeVar161 = dot( normalView, nodeVar160 );

	if ( ( render.nodeUniform32 > 0.0 ) ) {

		nodeVar163 = length( nodeVar159 );
		nodeVar164 = ( nodeVar163 / render.nodeUniform32 );
		nodeVar165 = clamp( ( 1.0 - ( ( ( nodeVar164 * nodeVar164 ) * nodeVar164 ) * nodeVar164 ) ), 0.0, 1.0 );
		nodeVar162 = ( ( 1.0 / max( pow( nodeVar163, render.nodeUniform33 ), 0.01 ) ) * ( nodeVar165 * nodeVar165 ) );

	} else {

		nodeVar162 = ( 1.0 / max( pow( length( nodeVar159 ), render.nodeUniform33 ), 0.01 ) );

	}

	nodeVar166 = ( render.nodeUniform31 * vec3<f32>( nodeVar162 ) );
	nodeVar167 = ( vec3<f32>( clamp( nodeVar161, 0.0, 1.0 ) ) * nodeVar166 );
	nodeVar168 = nodeVar167;
	nodeVar169 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar170 = ( nodeVar168 * nodeVar169 );
	nodeVar171 = ( nodeVar160 + positionViewDirection );
	nodeVar172 = normalize( nodeVar171 );
	nodeVar173 = dot( positionViewDirection, nodeVar172 );
	nodeVar174 = clamp( nodeVar173, 0.0, 1.0 );
	nodeVar175 = exp2( ( ( ( nodeVar174 * -5.55473 ) - 6.98316 ) * nodeVar174 ) );
	nodeVar176 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar175 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar175 ) ) );
	nodeVar177 = ( vec3<f32>( 1.0 ) - nodeVar176 );
	nodeVar178 = nodeVar177;
	nodeVar179 = ( nodeVar170 * nodeVar178 );
	nodeVar180 = ( directDiffuse + nodeVar179 );
	directDiffuse = nodeVar180;
	nodeVar181 = normalize( ( nodeVar160 + positionViewDirection ) );
	nodeVar182 = clamp( dot( positionViewDirection, nodeVar181 ), 0.0, 1.0 );
	nodeVar183 = exp2( ( ( ( nodeVar182 * -5.55473 ) - 6.98316 ) * nodeVar182 ) );
	nodeVar184 = ( Roughness * Roughness );
	nodeVar185 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar183 ) ) ) + vec3<f32>( ( 1.0 * nodeVar183 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar184, clamp( dot( normalView, nodeVar160 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar184, clamp( dot( normalView, nodeVar181 ), 0.0, 1.0 ) ) ) );
	nodeVar186 = ( nodeVar168 * nodeVar185 );
	nodeVar187 = ( nodeVar186 * multiScatteringCompensation );
	nodeVar188 = ( directSpecular + nodeVar187 );
	directSpecular = nodeVar188;
	nodeVar189 = ( render.nodeUniform34 - v_positionView );
	nodeVar190 = normalize( nodeVar189 );
	nodeVar191 = dot( normalView, nodeVar190 );

	if ( ( render.nodeUniform36 > 0.0 ) ) {

		nodeVar193 = length( nodeVar189 );
		nodeVar194 = ( nodeVar193 / render.nodeUniform36 );
		nodeVar195 = clamp( ( 1.0 - ( ( ( nodeVar194 * nodeVar194 ) * nodeVar194 ) * nodeVar194 ) ), 0.0, 1.0 );
		nodeVar192 = ( ( 1.0 / max( pow( nodeVar193, render.nodeUniform37 ), 0.01 ) ) * ( nodeVar195 * nodeVar195 ) );

	} else {

		nodeVar192 = ( 1.0 / max( pow( length( nodeVar189 ), render.nodeUniform37 ), 0.01 ) );

	}

	nodeVar196 = ( render.nodeUniform35 * vec3<f32>( nodeVar192 ) );
	nodeVar197 = ( vec3<f32>( clamp( nodeVar191, 0.0, 1.0 ) ) * nodeVar196 );
	nodeVar198 = nodeVar197;
	nodeVar199 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar200 = ( nodeVar198 * nodeVar199 );
	nodeVar201 = ( nodeVar190 + positionViewDirection );
	nodeVar202 = normalize( nodeVar201 );
	nodeVar203 = dot( positionViewDirection, nodeVar202 );
	nodeVar204 = clamp( nodeVar203, 0.0, 1.0 );
	nodeVar205 = exp2( ( ( ( nodeVar204 * -5.55473 ) - 6.98316 ) * nodeVar204 ) );
	nodeVar206 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar205 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar205 ) ) );
	nodeVar207 = ( vec3<f32>( 1.0 ) - nodeVar206 );
	nodeVar208 = nodeVar207;
	nodeVar209 = ( nodeVar200 * nodeVar208 );
	nodeVar210 = ( directDiffuse + nodeVar209 );
	directDiffuse = nodeVar210;
	nodeVar211 = normalize( ( nodeVar190 + positionViewDirection ) );
	nodeVar212 = clamp( dot( positionViewDirection, nodeVar211 ), 0.0, 1.0 );
	nodeVar213 = exp2( ( ( ( nodeVar212 * -5.55473 ) - 6.98316 ) * nodeVar212 ) );
	nodeVar214 = ( Roughness * Roughness );
	nodeVar215 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar213 ) ) ) + vec3<f32>( ( 1.0 * nodeVar213 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar214, clamp( dot( normalView, nodeVar190 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar214, clamp( dot( normalView, nodeVar211 ), 0.0, 1.0 ) ) ) );
	nodeVar216 = ( nodeVar198 * nodeVar215 );
	nodeVar217 = ( nodeVar216 * multiScatteringCompensation );
	nodeVar218 = ( directSpecular + nodeVar217 );
	directSpecular = nodeVar218;
	nodeVar219 = ( render.nodeUniform38 - v_positionView );
	nodeVar220 = normalize( nodeVar219 );
	nodeVar221 = dot( normalView, nodeVar220 );

	if ( ( render.nodeUniform40 > 0.0 ) ) {

		nodeVar223 = length( nodeVar219 );
		nodeVar224 = ( nodeVar223 / render.nodeUniform40 );
		nodeVar225 = clamp( ( 1.0 - ( ( ( nodeVar224 * nodeVar224 ) * nodeVar224 ) * nodeVar224 ) ), 0.0, 1.0 );
		nodeVar222 = ( ( 1.0 / max( pow( nodeVar223, render.nodeUniform41 ), 0.01 ) ) * ( nodeVar225 * nodeVar225 ) );

	} else {

		nodeVar222 = ( 1.0 / max( pow( length( nodeVar219 ), render.nodeUniform41 ), 0.01 ) );

	}

	nodeVar226 = ( render.nodeUniform39 * vec3<f32>( nodeVar222 ) );
	nodeVar227 = ( vec3<f32>( clamp( nodeVar221, 0.0, 1.0 ) ) * nodeVar226 );
	nodeVar228 = nodeVar227;
	nodeVar229 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar230 = ( nodeVar228 * nodeVar229 );
	nodeVar231 = ( nodeVar220 + positionViewDirection );
	nodeVar232 = normalize( nodeVar231 );
	nodeVar233 = dot( positionViewDirection, nodeVar232 );
	nodeVar234 = clamp( nodeVar233, 0.0, 1.0 );
	nodeVar235 = exp2( ( ( ( nodeVar234 * -5.55473 ) - 6.98316 ) * nodeVar234 ) );
	nodeVar236 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar235 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar235 ) ) );
	nodeVar237 = ( vec3<f32>( 1.0 ) - nodeVar236 );
	nodeVar238 = nodeVar237;
	nodeVar239 = ( nodeVar230 * nodeVar238 );
	nodeVar240 = ( directDiffuse + nodeVar239 );
	directDiffuse = nodeVar240;
	nodeVar241 = normalize( ( nodeVar220 + positionViewDirection ) );
	nodeVar242 = clamp( dot( positionViewDirection, nodeVar241 ), 0.0, 1.0 );
	nodeVar243 = exp2( ( ( ( nodeVar242 * -5.55473 ) - 6.98316 ) * nodeVar242 ) );
	nodeVar244 = ( Roughness * Roughness );
	nodeVar245 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar243 ) ) ) + vec3<f32>( ( 1.0 * nodeVar243 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar244, clamp( dot( normalView, nodeVar220 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar244, clamp( dot( normalView, nodeVar241 ), 0.0, 1.0 ) ) ) );
	nodeVar246 = ( nodeVar228 * nodeVar245 );
	nodeVar247 = ( nodeVar246 * multiScatteringCompensation );
	nodeVar248 = ( directSpecular + nodeVar247 );
	directSpecular = nodeVar248;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar249 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform42 );
	nodeVar250 = floor( nodeVar249 );
	nodeVar251 = nodeVar250;
	nodeVar252 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar253 = getFace( ( object.nodeUniform43 * vec4<f32>( vec3<f32>( nodeVar252.x, ( - nodeVar252.y ), nodeVar252.z ), 1.0 ) ).xyz );
	nodeVar254 = max( ( 4.0 - nodeVar251 ), 0.0 );
	nodeVar251 = max( nodeVar251, 4.0 );
	nodeVar255 = exp2( nodeVar251 );
	nodeVar256 = ( ( getUV( ( object.nodeUniform43 * vec4<f32>( vec3<f32>( nodeVar252.x, ( - nodeVar252.y ), nodeVar252.z ), 1.0 ) ).xyz, nodeVar253 ) * vec2<f32>( ( nodeVar255 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar253 > 2.0 ) ) {

		nodeVar256.y = ( nodeVar256.y + nodeVar255 );
		nodeVar253 = ( nodeVar253 - 3.0 );
		

	}

	nodeVar256.x = ( nodeVar256.x + ( nodeVar253 * nodeVar255 ) );
	nodeVar256.x = ( nodeVar256.x + ( nodeVar254 * ( 3.0 * 16.0 ) ) );
	nodeVar256.y = ( nodeVar256.y + ( 4.0 * ( exp2( object.nodeUniform42 ) - nodeVar255 ) ) );
	nodeVar256.x = ( nodeVar256.x * object.nodeUniform45 );
	nodeVar256.y = ( nodeVar256.y * object.nodeUniform46 );
	nodeVar257 = textureSampleGrad( nodeUniform47, nodeUniform47_sampler, nodeVar256, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar258 = nodeVar257.xyz;
	nodeVar259 = fract( nodeVar249 );

	if ( ( nodeVar259 != 0.0 ) ) {

		nodeVar260 = ( nodeVar250 + 1.0 );
		nodeVar261 = getFace( ( object.nodeUniform43 * vec4<f32>( vec3<f32>( nodeVar252.x, ( - nodeVar252.y ), nodeVar252.z ), 1.0 ) ).xyz );
		nodeVar262 = max( ( 4.0 - nodeVar260 ), 0.0 );
		nodeVar260 = max( nodeVar260, 4.0 );
		nodeVar263 = exp2( nodeVar260 );
		nodeVar264 = ( ( getUV( ( object.nodeUniform43 * vec4<f32>( vec3<f32>( nodeVar252.x, ( - nodeVar252.y ), nodeVar252.z ), 1.0 ) ).xyz, nodeVar261 ) * vec2<f32>( ( nodeVar263 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar261 > 2.0 ) ) {

			nodeVar264.y = ( nodeVar264.y + nodeVar263 );
			nodeVar261 = ( nodeVar261 - 3.0 );
			

		}

		nodeVar264.x = ( nodeVar264.x + ( nodeVar261 * nodeVar263 ) );
		nodeVar264.x = ( nodeVar264.x + ( nodeVar262 * ( 3.0 * 16.0 ) ) );
		nodeVar264.y = ( nodeVar264.y + ( 4.0 * ( exp2( object.nodeUniform42 ) - nodeVar263 ) ) );
		nodeVar264.x = ( nodeVar264.x * object.nodeUniform45 );
		nodeVar264.y = ( nodeVar264.y * object.nodeUniform46 );
		nodeVar265 = textureSampleGrad( nodeUniform47, nodeUniform47_sampler, nodeVar264, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar266 = nodeVar265.xyz;
		nodeVar258 = mix( nodeVar258, nodeVar266, nodeVar259 );
		

	}

	nodeVar267 = ( radiance + ( nodeVar258 * vec3<f32>( object.nodeUniform48 ) ) );
	radiance = nodeVar267;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar268 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform42 );
	nodeVar269 = floor( nodeVar268 );
	nodeVar270 = nodeVar269;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar271 = getFace( ( object.nodeUniform43 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar272 = max( ( 4.0 - nodeVar270 ), 0.0 );
	nodeVar270 = max( nodeVar270, 4.0 );
	nodeVar273 = exp2( nodeVar270 );
	nodeVar274 = ( ( getUV( ( object.nodeUniform43 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar271 ) * vec2<f32>( ( nodeVar273 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar271 > 2.0 ) ) {

		nodeVar274.y = ( nodeVar274.y + nodeVar273 );
		nodeVar271 = ( nodeVar271 - 3.0 );
		

	}

	nodeVar274.x = ( nodeVar274.x + ( nodeVar271 * nodeVar273 ) );
	nodeVar274.x = ( nodeVar274.x + ( nodeVar272 * ( 3.0 * 16.0 ) ) );
	nodeVar274.y = ( nodeVar274.y + ( 4.0 * ( exp2( object.nodeUniform42 ) - nodeVar273 ) ) );
	nodeVar274.x = ( nodeVar274.x * object.nodeUniform45 );
	nodeVar274.y = ( nodeVar274.y * object.nodeUniform46 );
	nodeVar275 = textureSampleGrad( nodeUniform47, nodeUniform47_sampler, nodeVar274, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar276 = nodeVar275.xyz;
	nodeVar277 = fract( nodeVar268 );

	if ( ( nodeVar277 != 0.0 ) ) {

		nodeVar278 = ( nodeVar269 + 1.0 );
		nodeVar279 = getFace( ( object.nodeUniform43 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar280 = max( ( 4.0 - nodeVar278 ), 0.0 );
		nodeVar278 = max( nodeVar278, 4.0 );
		nodeVar281 = exp2( nodeVar278 );
		nodeVar282 = ( ( getUV( ( object.nodeUniform43 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar279 ) * vec2<f32>( ( nodeVar281 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar279 > 2.0 ) ) {

			nodeVar282.y = ( nodeVar282.y + nodeVar281 );
			nodeVar279 = ( nodeVar279 - 3.0 );
			

		}

		nodeVar282.x = ( nodeVar282.x + ( nodeVar279 * nodeVar281 ) );
		nodeVar282.x = ( nodeVar282.x + ( nodeVar280 * ( 3.0 * 16.0 ) ) );
		nodeVar282.y = ( nodeVar282.y + ( 4.0 * ( exp2( object.nodeUniform42 ) - nodeVar281 ) ) );
		nodeVar282.x = ( nodeVar282.x * object.nodeUniform45 );
		nodeVar282.y = ( nodeVar282.y * object.nodeUniform46 );
		nodeVar283 = textureSampleGrad( nodeUniform47, nodeUniform47_sampler, nodeVar282, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar284 = nodeVar283.xyz;
		nodeVar276 = mix( nodeVar276, nodeVar284, nodeVar277 );
		

	}

	nodeVar285 = ( iblIrradiance + ( ( nodeVar276 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform48 ) ) );
	iblIrradiance = nodeVar285;
	nodeVar286 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar287 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar288 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar289 = ( SpecularF90 * dfg.y );
	nodeVar290 = ( nodeVar288 + vec3<f32>( nodeVar289 ) );
	nodeVar291 = ( nodeVar286 + nodeVar290 );
	nodeVar286 = nodeVar291;
	nodeVar292 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar293 = nodeVar292;
	nodeVar294 = ( nodeVar293 * vec3<f32>( 0.047619 ) );
	nodeVar295 = ( SpecularColor + nodeVar294 );
	nodeVar296 = ( nodeVar290 * nodeVar295 );
	nodeVar297 = ( dfg.x + dfg.y );
	nodeVar298 = ( 1.0 - nodeVar297 );
	nodeVar299 = nodeVar298;
	nodeVar300 = ( vec3<f32>( nodeVar299 ) * nodeVar295 );
	nodeVar301 = ( vec3<f32>( 1.0 ) - nodeVar300 );
	nodeVar302 = nodeVar301;
	nodeVar303 = ( nodeVar296 / nodeVar302 );
	nodeVar304 = ( nodeVar303 * vec3<f32>( nodeVar299 ) );
	nodeVar305 = ( nodeVar287 + nodeVar304 );
	nodeVar287 = nodeVar305;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar306 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar307 = ( irradiance * nodeVar306 );
	nodeVar308 = ( nodeVar286 + nodeVar287 );
	nodeVar309 = ( vec3<f32>( 1.0 ) - nodeVar308 );
	nodeVar310 = nodeVar309;
	nodeVar311 = ( nodeVar307 * nodeVar310 );
	nodeVar312 = nodeVar311;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar313 = ( indirectDiffuse + nodeVar312 );
	indirectDiffuse = nodeVar313;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar314 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar315 = ( SpecularF90 * dfg.y );
	nodeVar316 = ( nodeVar314 + vec3<f32>( nodeVar315 ) );
	nodeVar317 = ( singleScatteringDielectric + nodeVar316 );
	singleScatteringDielectric = nodeVar317;
	nodeVar318 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar319 = nodeVar318;
	nodeVar320 = ( nodeVar319 * vec3<f32>( 0.047619 ) );
	nodeVar321 = ( SpecularColor + nodeVar320 );
	nodeVar322 = ( nodeVar316 * nodeVar321 );
	nodeVar323 = ( dfg.x + dfg.y );
	nodeVar324 = ( 1.0 - nodeVar323 );
	nodeVar325 = nodeVar324;
	nodeVar326 = ( vec3<f32>( nodeVar325 ) * nodeVar321 );
	nodeVar327 = ( vec3<f32>( 1.0 ) - nodeVar326 );
	nodeVar328 = nodeVar327;
	nodeVar329 = ( nodeVar322 / nodeVar328 );
	nodeVar330 = ( nodeVar329 * vec3<f32>( nodeVar325 ) );
	nodeVar331 = ( multiScatteringDielectric + nodeVar330 );
	multiScatteringDielectric = nodeVar331;
	nodeVar332 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar333 = ( SpecularF90 * dfg.y );
	nodeVar334 = ( nodeVar332 + vec3<f32>( nodeVar333 ) );
	nodeVar335 = ( singleScatteringMetallic + nodeVar334 );
	singleScatteringMetallic = nodeVar335;
	nodeVar336 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar337 = nodeVar336;
	nodeVar338 = ( nodeVar337 * vec3<f32>( 0.047619 ) );
	nodeVar339 = ( DiffuseColor.xyz + nodeVar338 );
	nodeVar340 = ( nodeVar334 * nodeVar339 );
	nodeVar341 = ( dfg.x + dfg.y );
	nodeVar342 = ( 1.0 - nodeVar341 );
	nodeVar343 = nodeVar342;
	nodeVar344 = ( vec3<f32>( nodeVar343 ) * nodeVar339 );
	nodeVar345 = ( vec3<f32>( 1.0 ) - nodeVar344 );
	nodeVar346 = nodeVar345;
	nodeVar347 = ( nodeVar340 / nodeVar346 );
	nodeVar348 = ( nodeVar347 * vec3<f32>( nodeVar343 ) );
	nodeVar349 = ( multiScatteringMetallic + nodeVar348 );
	multiScatteringMetallic = nodeVar349;
	nodeVar350 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar351 = ( radiance * nodeVar350 );
	nodeVar352 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar353 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar354 = ( nodeVar352 * nodeVar353 );
	nodeVar355 = ( nodeVar351 + nodeVar354 );
	nodeVar356 = nodeVar355;
	nodeVar357 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar358 = ( vec3<f32>( 1.0 ) - nodeVar357 );
	nodeVar359 = nodeVar358;
	nodeVar360 = ( DiffuseContribution * nodeVar359 );
	nodeVar361 = ( nodeVar360 * nodeVar353 );
	nodeVar362 = nodeVar361;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar363 = ( indirectSpecular + nodeVar356 );
	indirectSpecular = nodeVar363;
	nodeVar364 = ( indirectDiffuse + nodeVar362 );
	indirectDiffuse = nodeVar364;
	ambientOcclusion = 1.0;
	nodeVar365 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar365;
	nodeVar366 = dot( normalView, positionViewDirection );
	nodeVar367 = ( clamp( nodeVar366, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar368 = ( Roughness * -16.0 );
	nodeVar369 = ( 1.0 - nodeVar368 );
	nodeVar370 = nodeVar369;
	nodeVar371 = ( - nodeVar370 );
	nodeVar372 = exp2( nodeVar371 );
	nodeVar373 = pow( nodeVar367, nodeVar372 );
	nodeVar374 = ( 1.0 - nodeVar373 );
	nodeVar375 = nodeVar374;
	nodeVar376 = ( ambientOcclusion - nodeVar375 );
	nodeVar377 = ( indirectSpecular * vec3<f32>( clamp( nodeVar376, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar377;
	nodeVar378 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar378;
	nodeVar379 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar379;
	nodeVar380 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar380;
	nodeVar381 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar381;

	// result

	output.color = nodeVar381;

	return output;

}
