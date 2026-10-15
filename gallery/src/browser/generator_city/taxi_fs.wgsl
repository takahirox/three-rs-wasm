// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 2 ) @group( 1 ) var nodeUniform11_sampler : sampler;
@binding( 3 ) @group( 1 ) var nodeUniform11 : texture_2d<f32>;
@binding( 4 ) @group( 1 ) var nodeUniform20_sampler : sampler_comparison;
@binding( 5 ) @group( 1 ) var nodeUniform20 : texture_depth_2d;
@binding( 6 ) @group( 1 ) var nodeUniform24_sampler : sampler;
@binding( 7 ) @group( 1 ) var nodeUniform24 : texture_3d<f32>;
@binding( 8 ) @group( 1 ) var nodeUniform34_sampler : sampler;
@binding( 9 ) @group( 1 ) var nodeUniform34 : texture_2d<f32>;

struct NodeBuffer_10726Struct {
	value : array< vec4<f32>, 2 >
};
@binding( 1 ) @group( 1 )
var<uniform> NodeBuffer_10726 : NodeBuffer_10726Struct;

struct objectStruct {
	nodeUniform1 : f32,
	nodeUniform2 : f32,
	nodeUniform4 : vec3<f32>,
	nodeUniform5 : vec2<f32>,
	nodeUniform6 : f32,
	nodeUniform7 : vec2<f32>,
	nodeUniform8 : f32,
	nodeUniform10 : mat3x3<f32>,
	nodeUniform12 : mat4x4<f32>,
	nodeUniform25 : vec3<f32>,
	nodeUniform26 : vec3<f32>,
	nodeUniform27 : vec3<f32>,
	nodeUniform28 : f32,
	nodeUniform29 : f32,
	nodeUniform30 : mat4x4<f32>,
	nodeUniform32 : f32,
	nodeUniform33 : f32,
	nodeUniform35 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform16 : vec3<f32>,
	nodeUniform14 : vec3<f32>,
	nodeUniform15 : vec3<f32>,
	nodeUniform17 : mat4x4<f32>,
	nodeUniform18 : f32,
	nodeUniform19 : f32,
	nodeUniform23 : f32,
	cameraWorldMatrix : mat4x4<f32>,
	nodeUniform21 : f32,
	nodeUniform22 : vec2<f32>
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : vec3<f32>;
var<private> nodeVar1 : bool;
var<private> nodeVar2 : vec3<f32>;
var<private> nodeVar3 : vec2<f32>;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : vec3<f32>;
var<private> nodeVar6 : vec3<f32>;
var<private> nodeVar7 : vec3<f32>;
var<private> nodeVar8 : vec2<f32>;
var<private> nodeVar9 : vec2<f32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : bool;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec2<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec3<f32>;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec2<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : vec2<f32>;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec2<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : bool;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec2<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : bool;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec2<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : vec3<f32>;
var<private> nodeVar45 : vec2<f32>;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> Metalness : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : vec2<f32>;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : bool;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> Roughness : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : vec2<f32>;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : f32;
var<private> nodeVar65 : bool;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> normalViewGeometry : vec3<f32>;
var<private> nodeVar68 : vec3<f32>;
var<private> SpecularColor : vec3<f32>;
var<private> SpecularColorBlended : vec3<f32>;
var<private> SpecularF90 : f32;
var<private> DiffuseContribution : vec3<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> nodeVar69 : vec3<f32>;
var<private> nodeVar70 : bool;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : vec2<f32>;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> positionViewDirection : vec3<f32>;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : vec2<f32>;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : f32;
var<private> nodeVar83 : vec3<f32>;
var<private> nodeVar84 : vec3<f32>;
var<private> nodeVar85 : vec3<f32>;
var<private> nodeVar86 : vec4<f32>;
var<private> nodeVar87 : vec4<f32>;
var<private> nodeVar88 : vec3<f32>;
var<private> nodeVar89 : vec3<f32>;
var<private> nodeVar90 : f32;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar91 : f32;
var<private> normalWorld : vec3<f32>;
var<private> nodeVar92 : vec4<f32>;
var<private> nodeVar93 : vec3<f32>;
var<private> nodeVar94 : vec3<f32>;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : vec2<f32>;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : vec2<f32>;
var<private> nodeVar100 : f32;
var<private> nodeVar101 : vec2<f32>;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : vec2<f32>;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : vec2<f32>;
var<private> nodeVar106 : f32;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : vec3<f32>;
var<private> nodeVar109 : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar110 : vec3<f32>;
var<private> nodeVar111 : vec3<f32>;
var<private> nodeVar112 : vec3<f32>;
var<private> nodeVar113 : vec3<f32>;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : vec3<f32>;
var<private> nodeVar118 : vec3<f32>;
var<private> nodeVar119 : vec3<f32>;
var<private> nodeVar120 : vec3<f32>;
var<private> nodeVar121 : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> nodeVar122 : vec3<f32>;
var<private> nodeVar123 : f32;
var<private> nodeVar124 : f32;
var<private> nodeVar125 : f32;
var<private> nodeVar126 : vec3<f32>;
var<private> nodeVar127 : vec3<f32>;
var<private> nodeVar128 : vec3<f32>;
var<private> nodeVar129 : vec3<f32>;
var<private> irradiance : vec3<f32>;
var<private> nodeVar130 : vec3<f32>;
var<private> nodeVar131 : vec3<f32>;
var<private> nodeVar132 : vec3<f32>;
var<private> nodeVar133 : vec3<f32>;
var<private> nodeVar134 : vec3<f32>;
var<private> nodeVar135 : vec3<f32>;
var<private> nodeVar136 : vec3<f32>;
var<private> nodeVar137 : vec3<f32>;
var<private> nodeVar138 : vec3<f32>;
var<private> nodeVar139 : vec3<f32>;
var<private> nodeVar140 : vec3<f32>;
var<private> nodeVar141 : vec3<f32>;
var<private> nodeVar142 : f32;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : f32;
var<private> nodeVar148 : f32;
var<private> nodeVar149 : vec3<f32>;
var<private> nodeVar150 : vec4<f32>;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : f32;
var<private> nodeVar154 : vec3<f32>;
var<private> nodeVar155 : vec4<f32>;
var<private> nodeVar156 : vec3<f32>;
var<private> nodeVar157 : f32;
var<private> nodeVar158 : f32;
var<private> nodeVar159 : f32;
var<private> nodeVar160 : vec3<f32>;
var<private> nodeVar161 : vec4<f32>;
var<private> nodeVar162 : vec3<f32>;
var<private> nodeVar163 : f32;
var<private> nodeVar164 : f32;
var<private> nodeVar165 : f32;
var<private> nodeVar166 : vec3<f32>;
var<private> nodeVar167 : vec4<f32>;
var<private> nodeVar168 : f32;
var<private> nodeVar169 : f32;
var<private> nodeVar170 : f32;
var<private> nodeVar171 : vec3<f32>;
var<private> nodeVar172 : vec4<f32>;
var<private> nodeVar173 : vec3<f32>;
var<private> nodeVar174 : f32;
var<private> nodeVar175 : f32;
var<private> nodeVar176 : f32;
var<private> nodeVar177 : vec3<f32>;
var<private> nodeVar178 : vec4<f32>;
var<private> nodeVar179 : vec3<f32>;
var<private> nodeVar180 : f32;
var<private> nodeVar181 : f32;
var<private> nodeVar182 : f32;
var<private> nodeVar183 : vec3<f32>;
var<private> nodeVar184 : vec4<f32>;
var<private> nodeVar185 : array< vec3<f32>, 9 >;
var<private> nodeVar186 : vec3<f32>;
var<private> nodeVar187 : vec3<f32>;
var<private> nodeVar188 : vec3<f32>;
var<private> nodeVar189 : vec3<f32>;
var<private> radiance : vec3<f32>;
var<private> nodeVar190 : f32;
var<private> nodeVar191 : f32;
var<private> nodeVar192 : f32;
var<private> nodeVar193 : vec3<f32>;
var<private> nodeVar194 : f32;
var<private> nodeVar195 : f32;
var<private> nodeVar196 : f32;
var<private> nodeVar197 : vec2<f32>;
var<private> nodeVar198 : vec4<f32>;
var<private> nodeVar199 : vec3<f32>;
var<private> nodeVar200 : f32;
var<private> nodeVar201 : f32;
var<private> nodeVar202 : f32;
var<private> nodeVar203 : f32;
var<private> nodeVar204 : f32;
var<private> nodeVar205 : vec2<f32>;
var<private> nodeVar206 : vec4<f32>;
var<private> nodeVar207 : vec3<f32>;
var<private> nodeVar208 : vec3<f32>;
var<private> iblIrradiance : vec3<f32>;
var<private> nodeVar209 : f32;
var<private> nodeVar210 : f32;
var<private> nodeVar211 : f32;
var<private> nodeVar212 : f32;
var<private> nodeVar213 : f32;
var<private> nodeVar214 : f32;
var<private> nodeVar215 : vec2<f32>;
var<private> nodeVar216 : vec4<f32>;
var<private> nodeVar217 : vec3<f32>;
var<private> nodeVar218 : f32;
var<private> nodeVar219 : f32;
var<private> nodeVar220 : f32;
var<private> nodeVar221 : f32;
var<private> nodeVar222 : f32;
var<private> nodeVar223 : vec2<f32>;
var<private> nodeVar224 : vec4<f32>;
var<private> nodeVar225 : vec3<f32>;
var<private> nodeVar226 : vec3<f32>;
var<private> nodeVar227 : vec3<f32>;
var<private> nodeVar228 : vec3<f32>;
var<private> nodeVar229 : vec3<f32>;
var<private> nodeVar230 : f32;
var<private> nodeVar231 : vec3<f32>;
var<private> nodeVar232 : vec3<f32>;
var<private> nodeVar233 : vec3<f32>;
var<private> nodeVar234 : vec3<f32>;
var<private> nodeVar235 : vec3<f32>;
var<private> nodeVar236 : vec3<f32>;
var<private> nodeVar237 : vec3<f32>;
var<private> nodeVar238 : f32;
var<private> nodeVar239 : f32;
var<private> nodeVar240 : f32;
var<private> nodeVar241 : vec3<f32>;
var<private> nodeVar242 : vec3<f32>;
var<private> nodeVar243 : vec3<f32>;
var<private> nodeVar244 : vec3<f32>;
var<private> nodeVar245 : vec3<f32>;
var<private> nodeVar246 : vec3<f32>;
var<private> nodeVar247 : vec3<f32>;
var<private> nodeVar248 : vec3<f32>;
var<private> nodeVar249 : vec3<f32>;
var<private> nodeVar250 : vec3<f32>;
var<private> nodeVar251 : vec3<f32>;
var<private> nodeVar252 : vec3<f32>;
var<private> nodeVar253 : vec3<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar254 : vec3<f32>;
var<private> singleScatteringDielectric : vec3<f32>;
var<private> multiScatteringDielectric : vec3<f32>;
var<private> singleScatteringMetallic : vec3<f32>;
var<private> multiScatteringMetallic : vec3<f32>;
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
var<private> nodeVar273 : vec3<f32>;
var<private> nodeVar274 : f32;
var<private> nodeVar275 : vec3<f32>;
var<private> nodeVar276 : vec3<f32>;
var<private> nodeVar277 : vec3<f32>;
var<private> nodeVar278 : vec3<f32>;
var<private> nodeVar279 : vec3<f32>;
var<private> nodeVar280 : vec3<f32>;
var<private> nodeVar281 : vec3<f32>;
var<private> nodeVar282 : f32;
var<private> nodeVar283 : f32;
var<private> nodeVar284 : f32;
var<private> nodeVar285 : vec3<f32>;
var<private> nodeVar286 : vec3<f32>;
var<private> nodeVar287 : vec3<f32>;
var<private> nodeVar288 : vec3<f32>;
var<private> nodeVar289 : vec3<f32>;
var<private> nodeVar290 : vec3<f32>;
var<private> nodeVar291 : vec3<f32>;
var<private> nodeVar292 : vec3<f32>;
var<private> nodeVar293 : vec3<f32>;
var<private> nodeVar294 : vec3<f32>;
var<private> nodeVar295 : vec3<f32>;
var<private> nodeVar296 : vec3<f32>;
var<private> nodeVar297 : vec3<f32>;
var<private> nodeVar298 : vec3<f32>;
var<private> nodeVar299 : vec3<f32>;
var<private> nodeVar300 : vec3<f32>;
var<private> nodeVar301 : vec3<f32>;
var<private> nodeVar302 : vec3<f32>;
var<private> nodeVar303 : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar304 : vec3<f32>;
var<private> nodeVar305 : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar306 : vec3<f32>;
var<private> nodeVar307 : f32;
var<private> nodeVar308 : f32;
var<private> nodeVar309 : f32;
var<private> nodeVar310 : f32;
var<private> nodeVar311 : f32;
var<private> nodeVar312 : f32;
var<private> nodeVar313 : f32;
var<private> nodeVar314 : f32;
var<private> nodeVar315 : f32;
var<private> nodeVar316 : f32;
var<private> nodeVar317 : f32;
var<private> nodeVar318 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar319 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> nodeVar320 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar321 : vec3<f32>;
var<private> nodeVar322 : vec4<f32>;

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
fn main( @location( 0 ) @interpolate( flat, either ) nodeVarying3 : f32,
	@location( 1 ) v_normalViewGeometry : vec3<f32>,
	@location( 2 ) v_positionViewDirection : vec3<f32>,
	@location( 3 ) v_positionWorld : vec3<f32>,
	@location( 4 ) nodeVarying8 : vec3<f32>,
	@location( 5 ) nodeVarying9 : vec3<f32>,
	@location( 6 ) nodeVarying10 : vec2<f32>,
	@location( 7 ) nodeVarying11 : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar1 = ( nodeVarying3 == 2.0 );

	if ( nodeVar1 ) {

		nodeVar0 = ( vec3<f32>( 0.009134058699157796, 0.00972121731707524, 0.010960094003125918 ) * vec3<f32>( ( ( smoothstep( ( object.nodeUniform1 * 0.85 ), ( object.nodeUniform1 * 0.94 ), length( vec2<f32>( ( abs( nodeVarying8.z ) - object.nodeUniform2 ), ( nodeVarying8.y - object.nodeUniform1 ) ) ) ) * 0.2 ) + 0.8 ) ) );

	} else {


		if ( ( nodeVarying3 == 3.0 ) ) {

			nodeVar3 = vec2<f32>( ( abs( nodeVarying8.z ) - object.nodeUniform2 ), ( nodeVarying8.y - object.nodeUniform1 ) );
			nodeVar4 = length( nodeVar3 );
			nodeVar2 = mix( vec3<f32>( 0.008568125615105716, 0.010960094003125918, 0.014443843592229466 ), vec3<f32>( 0.4286904966038916, 0.46778379610254284, 0.4910208498384856 ), max( max( smoothstep( 0.62, 0.42, ( abs( ( fract( ( atan2( nodeVar3.y, nodeVar3.x ) * 0.7957747154594768 ) ) - 0.5 ) ) * 2.0 ) ), smoothstep( ( object.nodeUniform1 * 0.53 ), ( object.nodeUniform1 * 0.59 ), nodeVar4 ) ), smoothstep( 0.06, 0.035, nodeVar4 ) ) );

		} else {


			if ( ( nodeVarying3 == 4.0 ) ) {

				nodeVar5 = vec3<f32>( 0.014443843592229466, 0.01680737574872402, 0.019382360952473074 );

			} else {


				if ( ( nodeVarying3 == 6.0 ) ) {

					nodeVar6 = vec3<f32>( 1.0, 0.6724431569510133, 0.14412847084818123 );

				} else {


					if ( ( nodeVarying3 == 1.0 ) ) {

						nodeVar8 = ( nodeVarying10 - vec2<f32>( 0.5 ) );
						nodeVar9 = ( ( abs( nodeVar8 ) - vec2<f32>( 0.47, 0.43 ) ) + vec2<f32>( 0.045 ) );
						nodeVar10 = ( ( length( max( nodeVar9, vec2<f32>( 0.0 ) ) ) + min( max( nodeVar9.x, nodeVar9.y ), 0.0 ) ) - 0.045 );
						nodeVar11 = max( fwidth( nodeVar10 ), 0.001 );
						nodeVar13 = ( abs( nodeVarying11.x ) > 0.5 );

						if ( nodeVar13 ) {

							nodeVar12 = smoothstep( 0.026, 0.035, abs( ( nodeVarying8.z - NodeBuffer_10726.value[ 0u ].x ) ) );

						} else {

							nodeVar12 = 1.0;

						}


						if ( nodeVar13 ) {

							nodeVar14 = smoothstep( 0.026, 0.035, abs( ( nodeVarying8.z - NodeBuffer_10726.value[ 1u ].x ) ) );

						} else {

							nodeVar14 = 1.0;

						}

						nodeVar15 = ( ( abs( nodeVar8 ) - vec2<f32>( 0.455, 0.405 ) ) + vec2<f32>( 0.035 ) );
						nodeVar16 = ( ( length( max( nodeVar15, vec2<f32>( 0.0 ) ) ) + min( max( nodeVar15.x, nodeVar15.y ), 0.0 ) ) - 0.035 );
						nodeVar17 = max( fwidth( nodeVar16 ), 0.001 );

						if ( nodeVar13 ) {

							nodeVar18 = smoothstep( 0.043, 0.053, abs( ( nodeVarying8.z - NodeBuffer_10726.value[ 0u ].x ) ) );

						} else {

							nodeVar18 = 1.0;

						}


						if ( nodeVar13 ) {

							nodeVar19 = smoothstep( 0.043, 0.053, abs( ( nodeVarying8.z - NodeBuffer_10726.value[ 1u ].x ) ) );

						} else {

							nodeVar19 = 1.0;

						}

						nodeVar7 = mix( mix( nodeVarying9, vec3<f32>( 0.006512090790025684, 0.00972121731707524, 0.011612245176281512 ), ( ( smoothstep( nodeVar11, ( - nodeVar11 ), nodeVar10 ) * nodeVar12 ) * nodeVar14 ) ), vec3<f32>( 0.012286488353353374, 0.024157632443547246, 0.035601314869097636 ), ( ( smoothstep( nodeVar17, ( - nodeVar17 ), nodeVar16 ) * nodeVar18 ) * nodeVar19 ) );

					} else {


						if ( ( ( nodeVarying3 == 5.0 ) && ( nodeVarying11.z < -0.5 ) ) ) {

							nodeVar20 = vec3<f32>( 0.16202937562896222, 0.21586050010324417, 0.2541520943200296 );

						} else {

							nodeVar21 = smoothstep( 0.65, 0.85, abs( nodeVarying8.x ) );
							nodeVar22 = ( ( abs( vec2<f32>( ( nodeVarying8.z - object.nodeUniform5.x ), ( nodeVarying8.y - ( object.nodeUniform6 - 0.1 ) ) ) ) - vec2<f32>( 0.06, 0.011 ) ) + vec2<f32>( 0.006 ) );
							nodeVar23 = ( ( length( max( nodeVar22, vec2<f32>( 0.0 ) ) ) + min( max( nodeVar22.x, nodeVar22.y ), 0.0 ) ) - 0.006 );
							nodeVar24 = max( fwidth( nodeVar23 ), 0.001 );
							nodeVar25 = ( ( abs( vec2<f32>( ( nodeVarying8.z - object.nodeUniform5.y ), ( nodeVarying8.y - ( object.nodeUniform6 - 0.1 ) ) ) ) - vec2<f32>( 0.06, 0.011 ) ) + vec2<f32>( 0.006 ) );
							nodeVar26 = ( ( length( max( nodeVar25, vec2<f32>( 0.0 ) ) ) + min( max( nodeVar25.x, nodeVar25.y ), 0.0 ) ) - 0.006 );
							nodeVar27 = max( fwidth( nodeVar26 ), 0.001 );
							nodeVar28 = ( ( abs( vec2<f32>( nodeVarying8.x, ( nodeVarying8.y - ( object.nodeUniform7.x - 0.025 ) ) ) ) - vec2<f32>( 0.3, 0.075 ) ) + vec2<f32>( 0.025 ) );
							nodeVar29 = ( ( length( max( nodeVar28, vec2<f32>( 0.0 ) ) ) + min( max( nodeVar28.x, nodeVar28.y ), 0.0 ) ) - 0.025 );
							nodeVar30 = max( fwidth( nodeVar29 ), 0.001 );
							nodeVar32 = ( nodeVarying3 == 7.0 );

							if ( nodeVar32 ) {

								nodeVar31 = 1.0;

							} else {

								nodeVar31 = 0.0;

							}


							if ( nodeVar32 ) {

								nodeVar33 = object.nodeUniform7.x;

							} else {

								nodeVar33 = object.nodeUniform7.y;

							}

							nodeVar34 = ( ( abs( vec2<f32>( nodeVarying8.x, ( nodeVarying8.y - ( nodeVar33 - 0.29 ) ) ) ) - vec2<f32>( 0.66, 0.045 ) ) + vec2<f32>( 0.03 ) );
							nodeVar35 = ( ( length( max( nodeVar34, vec2<f32>( 0.0 ) ) ) + min( max( nodeVar34.x, nodeVar34.y ), 0.0 ) ) - 0.03 );
							nodeVar36 = max( fwidth( nodeVar35 ), 0.001 );
							nodeVar38 = ( nodeVar32 || ( nodeVarying3 == 8.0 ) );

							if ( nodeVar38 ) {

								nodeVar37 = 1.0;

							} else {

								nodeVar37 = 0.0;

							}


							if ( nodeVar32 ) {

								nodeVar39 = ( object.nodeUniform7.x - 0.2 );

							} else {

								nodeVar39 = ( object.nodeUniform7.y - 0.17 );

							}

							nodeVar40 = ( ( abs( vec2<f32>( nodeVarying8.x, ( nodeVarying8.y - nodeVar39 ) ) ) - vec2<f32>( 0.155, 0.055 ) ) + vec2<f32>( 0.008 ) );
							nodeVar41 = ( ( length( max( nodeVar40, vec2<f32>( 0.0 ) ) ) + min( max( nodeVar40.x, nodeVar40.y ), 0.0 ) ) - 0.008 );
							nodeVar42 = max( fwidth( nodeVar41 ), 0.001 );

							if ( nodeVar38 ) {

								nodeVar43 = 1.0;

							} else {

								nodeVar43 = 0.0;

							}


							if ( nodeVar32 ) {

								nodeVar44 = vec3<f32>( 0.6375968739867731, 0.775822218312646, 0.8307698767709715 );

							} else {

								nodeVar44 = vec3<f32>( 0.19806931954941637, 0.005181516700061659, 0.007499032040460618 );

							}

							nodeVar45 = ( ( abs( vec2<f32>( ( abs( nodeVarying8.x ) - 0.61 ), ( nodeVarying8.y - nodeVar33 ) ) ) - vec2<f32>( 0.19, 0.055 ) ) + vec2<f32>( 0.018 ) );
							nodeVar46 = ( ( length( max( nodeVar45, vec2<f32>( 0.0 ) ) ) + min( max( nodeVar45.x, nodeVar45.y ), 0.0 ) ) - 0.018 );
							nodeVar47 = max( fwidth( nodeVar46 ), 0.001 );

							if ( nodeVar38 ) {

								nodeVar48 = 1.0;

							} else {

								nodeVar48 = 0.0;

							}

							nodeVar20 = mix( mix( mix( ( ( nodeVarying9 * vec3<f32>( ( mix( 1.0, ( ( smoothstep( ( object.nodeUniform1 + 0.025 ), ( object.nodeUniform1 + 0.095 ), length( vec2<f32>( ( abs( nodeVarying8.z ) - object.nodeUniform2 ), ( nodeVarying8.y - object.nodeUniform1 ) ) ) ) * 0.28 ) + 0.72 ), nodeVar21 ) * ( ( smoothstep( 0.25, 0.65, nodeVarying8.y ) * 0.3 ) + 0.7 ) ) ) ) * vec3<f32>( ( 1.0 - ( ( ( ( max( max( max( max( max( 0.0, smoothstep( 0.012, 0.004, abs( ( nodeVarying8.z - object.nodeUniform4.x ) ) ) ), smoothstep( 0.012, 0.004, abs( ( nodeVarying8.z - object.nodeUniform4.y ) ) ) ), smoothstep( 0.012, 0.004, abs( ( nodeVarying8.z - object.nodeUniform4.z ) ) ) ), smoothstep( nodeVar24, ( - nodeVar24 ), nodeVar23 ) ), smoothstep( nodeVar27, ( - nodeVar27 ), nodeVar26 ) ) * nodeVar21 ) * smoothstep( 0.36, 0.43, nodeVarying8.y ) ) * smoothstep( object.nodeUniform6, ( object.nodeUniform6 - 0.025 ), nodeVarying8.y ) ) * 0.5 ) ) ) ), ( vec3<f32>( 0.008023192982520563, 0.010329823026364548, 0.011612245176281512 ) * vec3<f32>( ( ( step( 0.5, fract( ( nodeVarying8.y * 65.0 ) ) ) * 0.35 ) + 0.65 ) ) ), max( ( smoothstep( nodeVar30, ( - nodeVar30 ), nodeVar29 ) * nodeVar31 ), ( smoothstep( nodeVar36, ( - nodeVar36 ), nodeVar35 ) * nodeVar37 ) ) ), vec3<f32>( 0.6866853124288864, 0.6938717612856897, 0.6514056374127929 ), ( smoothstep( nodeVar42, ( - nodeVar42 ), nodeVar41 ) * nodeVar43 ) ), nodeVar44, ( smoothstep( nodeVar47, ( - nodeVar47 ), nodeVar46 ) * nodeVar48 ) );

						}

						nodeVar7 = nodeVar20;

					}

					nodeVar6 = nodeVar7;

				}

				nodeVar5 = nodeVar6;

			}

			nodeVar2 = nodeVar5;

		}

		nodeVar0 = nodeVar2;

	}

	DiffuseColor = vec4<f32>( nodeVar0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform8 );
	DiffuseColor.w = 1.0;

	if ( ( nodeVarying3 == 3.0 ) ) {

		nodeVar49 = 0.8;

	} else {


		if ( ( ( nodeVar1 || ( nodeVarying3 == 4.0 ) ) || ( nodeVarying3 == 6.0 ) ) ) {

			nodeVar50 = 0.0;

		} else {


			if ( ( nodeVarying3 == 1.0 ) ) {

				nodeVar52 = ( ( abs( ( nodeVarying10 - vec2<f32>( 0.5 ) ) ) - vec2<f32>( 0.455, 0.405 ) ) + vec2<f32>( 0.035 ) );
				nodeVar53 = ( ( length( max( nodeVar52, vec2<f32>( 0.0 ) ) ) + min( max( nodeVar52.x, nodeVar52.y ), 0.0 ) ) - 0.035 );
				nodeVar54 = max( fwidth( nodeVar53 ), 0.001 );
				nodeVar56 = ( abs( nodeVarying11.x ) > 0.5 );

				if ( nodeVar56 ) {

					nodeVar55 = smoothstep( 0.043, 0.053, abs( ( nodeVarying8.z - NodeBuffer_10726.value[ 0u ].x ) ) );

				} else {

					nodeVar55 = 1.0;

				}


				if ( nodeVar56 ) {

					nodeVar57 = smoothstep( 0.043, 0.053, abs( ( nodeVarying8.z - NodeBuffer_10726.value[ 1u ].x ) ) );

				} else {

					nodeVar57 = 1.0;

				}

				nodeVar51 = ( ( smoothstep( nodeVar54, ( - nodeVar54 ), nodeVar53 ) * nodeVar55 ) * nodeVar57 );

			} else {


				if ( ( ( nodeVarying3 == 5.0 ) && ( nodeVarying11.z < -0.5 ) ) ) {

					nodeVar58 = 1.0;

				} else {

					nodeVar58 = 0.0;

				}

				nodeVar51 = nodeVar58;

			}

			nodeVar50 = mix( 0.25, 0.85, nodeVar51 );

		}

		nodeVar49 = nodeVar50;

	}

	Metalness = nodeVar49;

	if ( ( nodeVar1 || ( nodeVarying3 == 4.0 ) ) ) {

		nodeVar59 = 0.85;

	} else {


		if ( ( nodeVarying3 == 1.0 ) ) {

			nodeVar61 = ( ( abs( ( nodeVarying10 - vec2<f32>( 0.5 ) ) ) - vec2<f32>( 0.455, 0.405 ) ) + vec2<f32>( 0.035 ) );
			nodeVar62 = ( ( length( max( nodeVar61, vec2<f32>( 0.0 ) ) ) + min( max( nodeVar61.x, nodeVar61.y ), 0.0 ) ) - 0.035 );
			nodeVar63 = max( fwidth( nodeVar62 ), 0.001 );
			nodeVar65 = ( abs( nodeVarying11.x ) > 0.5 );

			if ( nodeVar65 ) {

				nodeVar64 = smoothstep( 0.043, 0.053, abs( ( nodeVarying8.z - NodeBuffer_10726.value[ 0u ].x ) ) );

			} else {

				nodeVar64 = 1.0;

			}


			if ( nodeVar65 ) {

				nodeVar66 = smoothstep( 0.043, 0.053, abs( ( nodeVarying8.z - NodeBuffer_10726.value[ 1u ].x ) ) );

			} else {

				nodeVar66 = 1.0;

			}

			nodeVar60 = ( ( smoothstep( nodeVar63, ( - nodeVar63 ), nodeVar62 ) * nodeVar64 ) * nodeVar66 );

		} else {


			if ( ( ( nodeVarying3 == 5.0 ) && ( nodeVarying11.z < -0.5 ) ) ) {

				nodeVar67 = 1.0;

			} else {

				nodeVar67 = 0.0;

			}

			nodeVar60 = nodeVar67;

		}

		nodeVar59 = mix( 0.32, 0.055, nodeVar60 );

	}

	normalViewGeometry = normalize( v_normalViewGeometry );
	nodeVar68 = max( abs( dpdx( normalViewGeometry ) ), abs( - dpdy( normalViewGeometry ) ) );
	Roughness = min( ( max( nodeVar59, 0.0525 ) + max( max( nodeVar68.x, nodeVar68.y ), nodeVar68.z ) ), 1.0 );
	SpecularColor = vec3<f32>( 0.04, 0.04, 0.04 );
	SpecularColorBlended = mix( vec3<f32>( 0.04, 0.04, 0.04 ), DiffuseColor.xyz, Metalness );
	SpecularF90 = 1.0;
	DiffuseContribution = ( DiffuseColor.xyz * vec3<f32>( ( 1.0 - nodeVar49 ) ) );
	nodeVar70 = ( nodeVarying3 == 7.0 );

	if ( nodeVar70 ) {

		nodeVar69 = ( vec3<f32>( 0.6938717612856897, 0.8631572134510892, 1.0 ) * vec3<f32>( 4.0 ) );

	} else {

		nodeVar69 = ( vec3<f32>( 0.8713671191959567, 0.0, 0.002428215868235294 ) * vec3<f32>( 1.5 ) );

	}


	if ( nodeVar70 ) {

		nodeVar71 = object.nodeUniform7.x;

	} else {

		nodeVar71 = object.nodeUniform7.y;

	}

	nodeVar72 = ( ( abs( vec2<f32>( ( abs( nodeVarying8.x ) - 0.61 ), ( nodeVarying8.y - nodeVar71 ) ) ) - vec2<f32>( 0.19, 0.055 ) ) + vec2<f32>( 0.018 ) );
	nodeVar73 = ( ( length( max( nodeVar72, vec2<f32>( 0.0 ) ) ) + min( max( nodeVar72.x, nodeVar72.y ), 0.0 ) ) - 0.018 );
	nodeVar74 = max( fwidth( nodeVar73 ), 0.001 );

	if ( ( nodeVar70 || ( nodeVarying3 == 8.0 ) ) ) {

		nodeVar75 = 1.0;

	} else {

		nodeVar75 = 0.0;

	}


	if ( ( nodeVarying3 == 6.0 ) ) {

		nodeVar76 = ( vec3<f32>( 1.0, 0.6795424696265424, 0.19806931954941637 ) * vec3<f32>( 2.0 ) );

	} else {

		nodeVar76 = vec3<f32>( 0.0, 0.0, 0.0 );

	}

	EmissiveColor = ( ( nodeVar69 * vec3<f32>( ( smoothstep( nodeVar74, ( - nodeVar74 ), nodeVar73 ) * nodeVar75 ) ) ) + nodeVar76 );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	positionViewDirection = normalize( v_positionViewDirection );
	nodeVar77 = dot( normalView, positionViewDirection );
	nodeVar78 = textureSample( nodeUniform11, nodeUniform11_sampler, vec2<f32>( Roughness, clamp( nodeVar77, 0.0, 1.0 ) ) ).xy;
	let dfg = nodeVar78;
	nodeVar79 = ( dfg.x + dfg.y );
	nodeVar80 = ( 1.0 / nodeVar79 );
	nodeVar81 = nodeVar80;
	nodeVar82 = ( nodeVar81 - 1.0 );
	nodeVar83 = ( SpecularColorBlended * vec3<f32>( nodeVar82 ) );
	nodeVar84 = ( nodeVar83 + vec3<f32>( 1.0 ) );
	let multiScatteringCompensation = nodeVar84;
	nodeVar85 = ( render.nodeUniform14 - render.nodeUniform15 );
	nodeVar86 = vec4<f32>( nodeVar85, 0.0 );
	nodeVar87 = ( render.cameraViewMatrix * nodeVar86 );
	nodeVar88 = normalize( nodeVar87.xyz );
	nodeVar89 = nodeVar88;
	nodeVar90 = dot( normalView, nodeVar89 );
	shadowPositionWorld = v_positionWorld;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	nodeVar92 = ( render.nodeUniform17 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform18 ) ) ), 1.0 ) );
	nodeVar93 = ( nodeVar92.xyz / vec3<f32>( nodeVar92.w ) );
	nodeVar94 = vec3<f32>( nodeVar93.x, ( 1.0 - nodeVar93.y ), ( nodeVar93.z + render.nodeUniform19 ) );

	if ( ( ( ( ( ( nodeVar94.x >= 0.0 ) && ( nodeVar94.x <= 1.0 ) ) && ( nodeVar94.y >= 0.0 ) ) && ( nodeVar94.y <= 1.0 ) ) && ( nodeVar94.z <= 1.0 ) ) ) {

		nodeVar95 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
		nodeVar96 = ( render.nodeUniform21 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform22 ).x );
		nodeVar97 = ( nodeVar94.xy + ( vogelDiskSample( 0, 5, nodeVar95 ) * vec2<f32>( nodeVar96 ) ) );
		nodeVar98 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, nodeVar97, nodeVar94.z );
		nodeVar99 = ( nodeVar94.xy + ( vogelDiskSample( 1, 5, nodeVar95 ) * vec2<f32>( nodeVar96 ) ) );
		nodeVar100 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, nodeVar99, nodeVar94.z );
		nodeVar101 = ( nodeVar94.xy + ( vogelDiskSample( 2, 5, nodeVar95 ) * vec2<f32>( nodeVar96 ) ) );
		nodeVar102 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, nodeVar101, nodeVar94.z );
		nodeVar103 = ( nodeVar94.xy + ( vogelDiskSample( 3, 5, nodeVar95 ) * vec2<f32>( nodeVar96 ) ) );
		nodeVar104 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, nodeVar103, nodeVar94.z );
		nodeVar105 = ( nodeVar94.xy + ( vogelDiskSample( 4, 5, nodeVar95 ) * vec2<f32>( nodeVar96 ) ) );
		nodeVar106 = textureSampleCompare( nodeUniform20, nodeUniform20_sampler, nodeVar105, nodeVar94.z );
		nodeVar91 = ( ( ( ( ( nodeVar98 + nodeVar100 ) + nodeVar102 ) + nodeVar104 ) + nodeVar106 ) * 0.2 );

	} else {

		nodeVar91 = 1.0;

	}

	nodeVar107 = mix( 1.0, nodeVar91, render.nodeUniform23 );
	nodeVar108 = ( vec3<f32>( clamp( nodeVar90, 0.0, 1.0 ) ) * ( render.nodeUniform16 * vec3<f32>( nodeVar107 ) ) );
	nodeVar109 = nodeVar108;
	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar110 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar111 = ( nodeVar109 * nodeVar110 );
	nodeVar112 = ( nodeVar89 + positionViewDirection );
	nodeVar113 = normalize( nodeVar112 );
	nodeVar114 = dot( positionViewDirection, nodeVar113 );
	nodeVar115 = clamp( nodeVar114, 0.0, 1.0 );
	nodeVar116 = exp2( ( ( ( nodeVar115 * -5.55473 ) - 6.98316 ) * nodeVar115 ) );
	nodeVar117 = ( ( SpecularColor * vec3<f32>( ( 1.0 - nodeVar116 ) ) ) + vec3<f32>( ( SpecularF90 * nodeVar116 ) ) );
	nodeVar118 = ( vec3<f32>( 1.0 ) - nodeVar117 );
	nodeVar119 = nodeVar118;
	nodeVar120 = ( nodeVar111 * nodeVar119 );
	nodeVar121 = ( directDiffuse + nodeVar120 );
	directDiffuse = nodeVar121;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar122 = normalize( ( nodeVar89 + positionViewDirection ) );
	nodeVar123 = clamp( dot( positionViewDirection, nodeVar122 ), 0.0, 1.0 );
	nodeVar124 = exp2( ( ( ( nodeVar123 * -5.55473 ) - 6.98316 ) * nodeVar123 ) );
	nodeVar125 = ( Roughness * Roughness );
	nodeVar126 = ( ( ( ( SpecularColorBlended * vec3<f32>( ( 1.0 - nodeVar124 ) ) ) + vec3<f32>( ( 1.0 * nodeVar124 ) ) ) * vec3<f32>( V_GGX_SmithCorrelated( nodeVar125, clamp( dot( normalView, nodeVar89 ), 0.0, 1.0 ), clamp( dot( normalView, positionViewDirection ), 0.0, 1.0 ) ) ) ) * vec3<f32>( D_GGX( nodeVar125, clamp( dot( normalView, nodeVar122 ), 0.0, 1.0 ) ) ) );
	nodeVar127 = ( nodeVar109 * nodeVar126 );
	nodeVar128 = ( nodeVar127 * multiScatteringCompensation );
	nodeVar129 = ( directSpecular + nodeVar128 );
	directSpecular = nodeVar129;
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar130 = ( object.nodeUniform25 - object.nodeUniform26 );
	nodeVar131 = ( object.nodeUniform27 - vec3<f32>( 1.0 ) );
	nodeVar132 = ( nodeVar130 / nodeVar131 );
	nodeVar133 = ( normalWorld * nodeVar132 );
	nodeVar134 = ( nodeVar133 * vec3<f32>( 0.5 ) );
	nodeVar135 = ( v_positionWorld + nodeVar134 );
	nodeVar136 = ( nodeVar135 - object.nodeUniform26 );
	nodeVar137 = ( nodeVar136 / nodeVar130 );
	nodeVar138 = ( clamp( nodeVar137, vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) ) * nodeVar131 );
	nodeVar139 = ( nodeVar138 / object.nodeUniform27 );
	nodeVar140 = ( vec3<f32>( 0.5, 0.5, 0.5 ) / object.nodeUniform27 );
	nodeVar141 = ( nodeVar139 + nodeVar140 );
	nodeVar142 = ( nodeVar141.z * object.nodeUniform27.z );
	nodeVar143 = ( nodeVar142 + 1.0 );
	nodeVar144 = ( object.nodeUniform27.z + 2.0 );
	nodeVar145 = ( nodeVar144 * 0.0 );
	nodeVar146 = ( nodeVar143 + nodeVar145 );
	nodeVar147 = ( nodeVar144 * 7.0 );
	nodeVar148 = ( nodeVar146 / nodeVar147 );
	nodeVar149 = vec3<f32>( nodeVar141.xy, nodeVar148 );
	nodeVar150 = textureSample( nodeUniform24, nodeUniform24_sampler, nodeVar149 );
	nodeVar151 = ( nodeVar144 * 1.0 );
	nodeVar152 = ( nodeVar143 + nodeVar151 );
	nodeVar153 = ( nodeVar152 / nodeVar147 );
	nodeVar154 = vec3<f32>( nodeVar141.xy, nodeVar153 );
	nodeVar155 = textureSample( nodeUniform24, nodeUniform24_sampler, nodeVar154 );
	nodeVar156 = vec3<f32>( nodeVar150.w, nodeVar155.xy );
	nodeVar157 = ( nodeVar144 * 2.0 );
	nodeVar158 = ( nodeVar143 + nodeVar157 );
	nodeVar159 = ( nodeVar158 / nodeVar147 );
	nodeVar160 = vec3<f32>( nodeVar141.xy, nodeVar159 );
	nodeVar161 = textureSample( nodeUniform24, nodeUniform24_sampler, nodeVar160 );
	nodeVar162 = vec3<f32>( nodeVar155.zw, nodeVar161.x );
	nodeVar163 = ( nodeVar144 * 3.0 );
	nodeVar164 = ( nodeVar143 + nodeVar163 );
	nodeVar165 = ( nodeVar164 / nodeVar147 );
	nodeVar166 = vec3<f32>( nodeVar141.xy, nodeVar165 );
	nodeVar167 = textureSample( nodeUniform24, nodeUniform24_sampler, nodeVar166 );
	nodeVar168 = ( nodeVar144 * 4.0 );
	nodeVar169 = ( nodeVar143 + nodeVar168 );
	nodeVar170 = ( nodeVar169 / nodeVar147 );
	nodeVar171 = vec3<f32>( nodeVar141.xy, nodeVar170 );
	nodeVar172 = textureSample( nodeUniform24, nodeUniform24_sampler, nodeVar171 );
	nodeVar173 = vec3<f32>( nodeVar167.w, nodeVar172.xy );
	nodeVar174 = ( nodeVar144 * 5.0 );
	nodeVar175 = ( nodeVar143 + nodeVar174 );
	nodeVar176 = ( nodeVar175 / nodeVar147 );
	nodeVar177 = vec3<f32>( nodeVar141.xy, nodeVar176 );
	nodeVar178 = textureSample( nodeUniform24, nodeUniform24_sampler, nodeVar177 );
	nodeVar179 = vec3<f32>( nodeVar172.zw, nodeVar178.x );
	nodeVar180 = ( nodeVar144 * 6.0 );
	nodeVar181 = ( nodeVar143 + nodeVar180 );
	nodeVar182 = ( nodeVar181 / nodeVar147 );
	nodeVar183 = vec3<f32>( nodeVar141.xy, nodeVar182 );
	nodeVar184 = textureSample( nodeUniform24, nodeUniform24_sampler, nodeVar183 );
	nodeVar185 = array< vec3<f32>, 9 >( nodeVar150.xyz, nodeVar156, nodeVar162, nodeVar161.yzw, nodeVar167.xyz, nodeVar173, nodeVar179, nodeVar178.yzw, nodeVar184.xyz );
	nodeVar186 = ( ( ( ( ( ( ( ( ( nodeVar185[ 0u ] * vec3<f32>( 0.886227 ) ) + ( ( nodeVar185[ 1u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( nodeVar185[ 2u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar185[ 3u ] * vec3<f32>( 1.023328 ) ) * vec3<f32>( normalWorld.x ) ) ) + ( ( ( nodeVar185[ 4u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.y ) ) ) + ( ( ( nodeVar185[ 5u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.y ) ) * vec3<f32>( normalWorld.z ) ) ) + ( nodeVar185[ 6u ] * vec3<f32>( ( ( ( normalWorld.z * normalWorld.z ) * 0.743125 ) - 0.247708 ) ) ) ) + ( ( ( nodeVar185[ 7u ] * vec3<f32>( 0.858086 ) ) * vec3<f32>( normalWorld.x ) ) * vec3<f32>( normalWorld.z ) ) ) + ( ( nodeVar185[ 8u ] * vec3<f32>( 0.429043 ) ) * vec3<f32>( ( ( normalWorld.x * normalWorld.x ) - ( normalWorld.y * normalWorld.y ) ) ) ) );
	nodeVar187 = max( nodeVar186, vec3<f32>( 0.0, 0.0, 0.0 ) );
	nodeVar188 = ( nodeVar187 * vec3<f32>( object.nodeUniform28 ) );
	nodeVar189 = ( irradiance + nodeVar188 );
	irradiance = nodeVar189;
	radiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar190 = clamp( roughnessToMip( Roughness ), -2.0, object.nodeUniform29 );
	nodeVar191 = floor( nodeVar190 );
	nodeVar192 = nodeVar191;
	nodeVar193 = normalize( ( render.cameraWorldMatrix * vec4<f32>( normalize( mix( reflect( ( - positionViewDirection ), normalView ), normalView, ( ( ( Roughness * Roughness ) * Roughness ) * Roughness ) ) ), 0.0 ) ).xyz );
	nodeVar194 = getFace( ( object.nodeUniform30 * vec4<f32>( vec3<f32>( nodeVar193.x, ( - nodeVar193.y ), nodeVar193.z ), 1.0 ) ).xyz );
	nodeVar195 = max( ( 4.0 - nodeVar192 ), 0.0 );
	nodeVar192 = max( nodeVar192, 4.0 );
	nodeVar196 = exp2( nodeVar192 );
	nodeVar197 = ( ( getUV( ( object.nodeUniform30 * vec4<f32>( vec3<f32>( nodeVar193.x, ( - nodeVar193.y ), nodeVar193.z ), 1.0 ) ).xyz, nodeVar194 ) * vec2<f32>( ( nodeVar196 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar194 > 2.0 ) ) {

		nodeVar197.y = ( nodeVar197.y + nodeVar196 );
		nodeVar194 = ( nodeVar194 - 3.0 );
		

	}

	nodeVar197.x = ( nodeVar197.x + ( nodeVar194 * nodeVar196 ) );
	nodeVar197.x = ( nodeVar197.x + ( nodeVar195 * ( 3.0 * 16.0 ) ) );
	nodeVar197.y = ( nodeVar197.y + ( 4.0 * ( exp2( object.nodeUniform29 ) - nodeVar196 ) ) );
	nodeVar197.x = ( nodeVar197.x * object.nodeUniform32 );
	nodeVar197.y = ( nodeVar197.y * object.nodeUniform33 );
	nodeVar198 = textureSampleGrad( nodeUniform34, nodeUniform34_sampler, nodeVar197, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar199 = nodeVar198.xyz;
	nodeVar200 = fract( nodeVar190 );

	if ( ( nodeVar200 != 0.0 ) ) {

		nodeVar201 = ( nodeVar191 + 1.0 );
		nodeVar202 = getFace( ( object.nodeUniform30 * vec4<f32>( vec3<f32>( nodeVar193.x, ( - nodeVar193.y ), nodeVar193.z ), 1.0 ) ).xyz );
		nodeVar203 = max( ( 4.0 - nodeVar201 ), 0.0 );
		nodeVar201 = max( nodeVar201, 4.0 );
		nodeVar204 = exp2( nodeVar201 );
		nodeVar205 = ( ( getUV( ( object.nodeUniform30 * vec4<f32>( vec3<f32>( nodeVar193.x, ( - nodeVar193.y ), nodeVar193.z ), 1.0 ) ).xyz, nodeVar202 ) * vec2<f32>( ( nodeVar204 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar202 > 2.0 ) ) {

			nodeVar205.y = ( nodeVar205.y + nodeVar204 );
			nodeVar202 = ( nodeVar202 - 3.0 );
			

		}

		nodeVar205.x = ( nodeVar205.x + ( nodeVar202 * nodeVar204 ) );
		nodeVar205.x = ( nodeVar205.x + ( nodeVar203 * ( 3.0 * 16.0 ) ) );
		nodeVar205.y = ( nodeVar205.y + ( 4.0 * ( exp2( object.nodeUniform29 ) - nodeVar204 ) ) );
		nodeVar205.x = ( nodeVar205.x * object.nodeUniform32 );
		nodeVar205.y = ( nodeVar205.y * object.nodeUniform33 );
		nodeVar206 = textureSampleGrad( nodeUniform34, nodeUniform34_sampler, nodeVar205, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar207 = nodeVar206.xyz;
		nodeVar199 = mix( nodeVar199, nodeVar207, nodeVar200 );
		

	}

	nodeVar208 = ( radiance + ( nodeVar199 * vec3<f32>( object.nodeUniform35 ) ) );
	radiance = nodeVar208;
	iblIrradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar209 = clamp( roughnessToMip( 1.0 ), -2.0, object.nodeUniform29 );
	nodeVar210 = floor( nodeVar209 );
	nodeVar211 = nodeVar210;
	nodeVar212 = getFace( ( object.nodeUniform30 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
	nodeVar213 = max( ( 4.0 - nodeVar211 ), 0.0 );
	nodeVar211 = max( nodeVar211, 4.0 );
	nodeVar214 = exp2( nodeVar211 );
	nodeVar215 = ( ( getUV( ( object.nodeUniform30 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar212 ) * vec2<f32>( ( nodeVar214 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

	if ( ( nodeVar212 > 2.0 ) ) {

		nodeVar215.y = ( nodeVar215.y + nodeVar214 );
		nodeVar212 = ( nodeVar212 - 3.0 );
		

	}

	nodeVar215.x = ( nodeVar215.x + ( nodeVar212 * nodeVar214 ) );
	nodeVar215.x = ( nodeVar215.x + ( nodeVar213 * ( 3.0 * 16.0 ) ) );
	nodeVar215.y = ( nodeVar215.y + ( 4.0 * ( exp2( object.nodeUniform29 ) - nodeVar214 ) ) );
	nodeVar215.x = ( nodeVar215.x * object.nodeUniform32 );
	nodeVar215.y = ( nodeVar215.y * object.nodeUniform33 );
	nodeVar216 = textureSampleGrad( nodeUniform34, nodeUniform34_sampler, nodeVar215, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
	nodeVar217 = nodeVar216.xyz;
	nodeVar218 = fract( nodeVar209 );

	if ( ( nodeVar218 != 0.0 ) ) {

		nodeVar219 = ( nodeVar210 + 1.0 );
		nodeVar220 = getFace( ( object.nodeUniform30 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz );
		nodeVar221 = max( ( 4.0 - nodeVar219 ), 0.0 );
		nodeVar219 = max( nodeVar219, 4.0 );
		nodeVar222 = exp2( nodeVar219 );
		nodeVar223 = ( ( getUV( ( object.nodeUniform30 * vec4<f32>( vec3<f32>( normalWorld.x, ( - normalWorld.y ), normalWorld.z ), 1.0 ) ).xyz, nodeVar220 ) * vec2<f32>( ( nodeVar222 - 2.0 ) ) ) + vec2<f32>( 1.0 ) );

		if ( ( nodeVar220 > 2.0 ) ) {

			nodeVar223.y = ( nodeVar223.y + nodeVar222 );
			nodeVar220 = ( nodeVar220 - 3.0 );
			

		}

		nodeVar223.x = ( nodeVar223.x + ( nodeVar220 * nodeVar222 ) );
		nodeVar223.x = ( nodeVar223.x + ( nodeVar221 * ( 3.0 * 16.0 ) ) );
		nodeVar223.y = ( nodeVar223.y + ( 4.0 * ( exp2( object.nodeUniform29 ) - nodeVar222 ) ) );
		nodeVar223.x = ( nodeVar223.x * object.nodeUniform32 );
		nodeVar223.y = ( nodeVar223.y * object.nodeUniform33 );
		nodeVar224 = textureSampleGrad( nodeUniform34, nodeUniform34_sampler, nodeVar223, vec2<f32>( 0.0, 0.0 ), vec2<f32>( 0.0, 0.0 ) );
		nodeVar225 = nodeVar224.xyz;
		nodeVar217 = mix( nodeVar217, nodeVar225, nodeVar218 );
		

	}

	nodeVar226 = ( iblIrradiance + ( ( nodeVar217 * vec3<f32>( 3.141592653589793 ) ) * vec3<f32>( object.nodeUniform35 ) ) );
	iblIrradiance = nodeVar226;
	nodeVar227 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar228 = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar229 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar230 = ( SpecularF90 * dfg.y );
	nodeVar231 = ( nodeVar229 + vec3<f32>( nodeVar230 ) );
	nodeVar232 = ( nodeVar227 + nodeVar231 );
	nodeVar227 = nodeVar232;
	nodeVar233 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar234 = nodeVar233;
	nodeVar235 = ( nodeVar234 * vec3<f32>( 0.047619 ) );
	nodeVar236 = ( SpecularColor + nodeVar235 );
	nodeVar237 = ( nodeVar231 * nodeVar236 );
	nodeVar238 = ( dfg.x + dfg.y );
	nodeVar239 = ( 1.0 - nodeVar238 );
	nodeVar240 = nodeVar239;
	nodeVar241 = ( vec3<f32>( nodeVar240 ) * nodeVar236 );
	nodeVar242 = ( vec3<f32>( 1.0 ) - nodeVar241 );
	nodeVar243 = nodeVar242;
	nodeVar244 = ( nodeVar237 / nodeVar243 );
	nodeVar245 = ( nodeVar244 * vec3<f32>( nodeVar240 ) );
	nodeVar246 = ( nodeVar228 + nodeVar245 );
	nodeVar228 = nodeVar246;
	nodeVar247 = ( DiffuseContribution * vec3<f32>( 0.3183098861837907 ) );
	nodeVar248 = ( irradiance * nodeVar247 );
	nodeVar249 = ( nodeVar227 + nodeVar228 );
	nodeVar250 = ( vec3<f32>( 1.0 ) - nodeVar249 );
	nodeVar251 = nodeVar250;
	nodeVar252 = ( nodeVar248 * nodeVar251 );
	nodeVar253 = nodeVar252;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar254 = ( indirectDiffuse + nodeVar253 );
	indirectDiffuse = nodeVar254;
	singleScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringDielectric = vec3<f32>( 0.0, 0.0, 0.0 );
	singleScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	multiScatteringMetallic = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar255 = ( SpecularColor * vec3<f32>( dfg.x ) );
	nodeVar256 = ( SpecularF90 * dfg.y );
	nodeVar257 = ( nodeVar255 + vec3<f32>( nodeVar256 ) );
	nodeVar258 = ( singleScatteringDielectric + nodeVar257 );
	singleScatteringDielectric = nodeVar258;
	nodeVar259 = ( vec3<f32>( 1.0 ) - SpecularColor );
	nodeVar260 = nodeVar259;
	nodeVar261 = ( nodeVar260 * vec3<f32>( 0.047619 ) );
	nodeVar262 = ( SpecularColor + nodeVar261 );
	nodeVar263 = ( nodeVar257 * nodeVar262 );
	nodeVar264 = ( dfg.x + dfg.y );
	nodeVar265 = ( 1.0 - nodeVar264 );
	nodeVar266 = nodeVar265;
	nodeVar267 = ( vec3<f32>( nodeVar266 ) * nodeVar262 );
	nodeVar268 = ( vec3<f32>( 1.0 ) - nodeVar267 );
	nodeVar269 = nodeVar268;
	nodeVar270 = ( nodeVar263 / nodeVar269 );
	nodeVar271 = ( nodeVar270 * vec3<f32>( nodeVar266 ) );
	nodeVar272 = ( multiScatteringDielectric + nodeVar271 );
	multiScatteringDielectric = nodeVar272;
	nodeVar273 = ( DiffuseColor.xyz * vec3<f32>( dfg.x ) );
	nodeVar274 = ( SpecularF90 * dfg.y );
	nodeVar275 = ( nodeVar273 + vec3<f32>( nodeVar274 ) );
	nodeVar276 = ( singleScatteringMetallic + nodeVar275 );
	singleScatteringMetallic = nodeVar276;
	nodeVar277 = ( vec3<f32>( 1.0 ) - DiffuseColor.xyz );
	nodeVar278 = nodeVar277;
	nodeVar279 = ( nodeVar278 * vec3<f32>( 0.047619 ) );
	nodeVar280 = ( DiffuseColor.xyz + nodeVar279 );
	nodeVar281 = ( nodeVar275 * nodeVar280 );
	nodeVar282 = ( dfg.x + dfg.y );
	nodeVar283 = ( 1.0 - nodeVar282 );
	nodeVar284 = nodeVar283;
	nodeVar285 = ( vec3<f32>( nodeVar284 ) * nodeVar280 );
	nodeVar286 = ( vec3<f32>( 1.0 ) - nodeVar285 );
	nodeVar287 = nodeVar286;
	nodeVar288 = ( nodeVar281 / nodeVar287 );
	nodeVar289 = ( nodeVar288 * vec3<f32>( nodeVar284 ) );
	nodeVar290 = ( multiScatteringMetallic + nodeVar289 );
	multiScatteringMetallic = nodeVar290;
	nodeVar291 = mix( singleScatteringDielectric, singleScatteringMetallic, Metalness );
	nodeVar292 = ( radiance * nodeVar291 );
	nodeVar293 = mix( multiScatteringDielectric, multiScatteringMetallic, Metalness );
	nodeVar294 = ( iblIrradiance * vec3<f32>( 0.3183098861837907 ) );
	nodeVar295 = ( nodeVar293 * nodeVar294 );
	nodeVar296 = ( nodeVar292 + nodeVar295 );
	nodeVar297 = nodeVar296;
	nodeVar298 = ( singleScatteringDielectric + multiScatteringDielectric );
	nodeVar299 = ( vec3<f32>( 1.0 ) - nodeVar298 );
	nodeVar300 = nodeVar299;
	nodeVar301 = ( DiffuseContribution * nodeVar300 );
	nodeVar302 = ( nodeVar301 * nodeVar294 );
	nodeVar303 = nodeVar302;
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar304 = ( indirectSpecular + nodeVar297 );
	indirectSpecular = nodeVar304;
	nodeVar305 = ( indirectDiffuse + nodeVar303 );
	indirectDiffuse = nodeVar305;
	ambientOcclusion = 1.0;
	nodeVar306 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar306;
	nodeVar307 = dot( normalView, positionViewDirection );
	nodeVar308 = ( clamp( nodeVar307, 0.0, 1.0 ) + ambientOcclusion );
	nodeVar309 = ( Roughness * -16.0 );
	nodeVar310 = ( 1.0 - nodeVar309 );
	nodeVar311 = nodeVar310;
	nodeVar312 = ( - nodeVar311 );
	nodeVar313 = exp2( nodeVar312 );
	nodeVar314 = pow( nodeVar308, nodeVar313 );
	nodeVar315 = ( 1.0 - nodeVar314 );
	nodeVar316 = nodeVar315;
	nodeVar317 = ( ambientOcclusion - nodeVar316 );
	nodeVar318 = ( indirectSpecular * vec3<f32>( clamp( nodeVar317, 0.0, 1.0 ) ) );
	indirectSpecular = nodeVar318;
	nodeVar319 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar319;
	nodeVar320 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar320;
	nodeVar321 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar321;
	nodeVar322 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar322;

	// result

	output.color = nodeVar322;

	return output;

}
