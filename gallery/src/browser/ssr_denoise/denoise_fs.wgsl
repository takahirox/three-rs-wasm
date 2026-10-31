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
@binding( 0 ) @group( 0 ) var nodeUniform0 : texture_depth_2d;
@binding( 1 ) @group( 0 ) var nodeUniform1_sampler : sampler;
@binding( 2 ) @group( 0 ) var nodeUniform1 : texture_2d<f32>;
@binding( 4 ) @group( 0 ) var nodeUniform3_sampler : sampler;
@binding( 5 ) @group( 0 ) var nodeUniform3 : texture_2d<f32>;
@binding( 6 ) @group( 0 ) var nodeUniform5_sampler : sampler;
@binding( 7 ) @group( 0 ) var nodeUniform5 : texture_2d<f32>;
@binding( 8 ) @group( 0 ) var nodeUniform8_sampler : sampler;
@binding( 9 ) @group( 0 ) var nodeUniform8 : texture_2d<f32>;

struct objectStruct {
	nodeUniform2 : mat4x4<f32>,
	nodeUniform4 : mat4x4<f32>,
	nodeUniform6 : vec2<f32>,
	nodeUniform7 : f32,
	nodeUniform9 : f32,
	nodeUniform10 : f32,
	nodeUniform11 : f32,
	nodeUniform12 : f32,
	nodeUniform13 : f32,
	nodeUniform14 : f32,
	nodeUniform15 : mat4x4<f32>,
	nodeUniform16 : f32,
	nodeUniform17 : f32,
	nodeUniform18 : f32,
	nodeUniform19 : f32,
	nodeUniform20 : u32,
	nodeUniform21 : f32
};
@binding( 3 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : f32;
var<private> nodeVar1 : vec2<u32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec4<f32>;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec4<f32>;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : bool;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : bool;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec4<f32>;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec3<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : vec3<f32>;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec3<f32>;
var<private> nodeVar36 : vec3<f32>;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec2<f32>;
var<private> nodeVar39 : vec2<f32>;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : vec2<f32>;
var<private> nodeVar42 : vec2<f32>;
var<private> nodeVar43 : vec4<f32>;
var<private> nodeVar44 : vec4<f32>;
var<private> nodeVar45 : vec4<f32>;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : vec4<f32>;
var<private> nodeVar50 : vec4<f32>;
var<private> nodeVar51 : vec4<f32>;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : bool;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : vec4<f32>;

// codes
fn fn150 ( viewZ : f32, tanHalfFovY : f32 ) -> f32 {

	


	return ( ( 2.0 * viewZ ) * tanHalfFovY );

}


fn fn148 ( x : f32, strength : f32 ) -> f32 {

	


	return ( 1.0 / pow( x, strength ) );

}


fn fn154 ( roughness : f32, percent : f32 ) -> f32 {

	


	return ( ( roughness * roughness ) * sqrt( ( percent / max( ( 1.0 - percent ), 0.000001 ) ) ) );

}


fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}

fn tsl_mod_vec2( x : vec2f, y : vec2f ) -> vec2f { return x - y * floor( x / y ); }
fn fn151 ( worldRayLength : f32, viewZ : f32, tanHalfFovY : f32 ) -> f32 {

	


	return clamp( ( worldRayLength / max( fn150( viewZ, tanHalfFovY ), 0.000001 ) ), 0.0, 1.0 );

}


fn fn149 ( frameNum : f32, strength : f32 ) -> f32 {

	


	return max( fn148( frameNum, strength ), 0.05 );

}


fn fn153 ( N : vec3<f32>, V : vec3<f32>, roughness : f32 ) -> vec3<f32> {

	


	return normalize( mix( N, reflect( ( - V ), N ), ( 1.0 - roughness ) ) );

}


fn fn155 ( roughness : f32, aggressivity : f32, invNormalPhi : f32 ) -> f32 {

	

	let nodeConst0 = ( 1.0 / max( atan( fn154( roughness, clamp( mix( ( invNormalPhi * invNormalPhi ), 0.0, sqrt( aggressivity ) ), 0.1, 0.99 ) ) ), 0.0058823529411764705 ) );

	return ( ( nodeConst0 * nodeConst0 ) * 8.0 );

}


fn fn146 ( i : f32, radius : f32 ) -> vec2<f32> {

	

	let nodeConst0 = ( ( i + 0.5 ) * 2.399827721492203 );

	return ( vec2<f32>( cos( nodeConst0 ), sin( nodeConst0 ) ) * vec2<f32>( ( radius * sqrt( ( ( i + 0.5 ) / 8.0 ) ) ) ) );

}


fn fn160 ( r : f32 ) -> mat2x2<f32> {

	

	let nodeConst0 = ( ( r * 2.0 ) * 3.141592653589793 );

	return mat2x2<f32>( cos( nodeConst0 ), ( - sin( nodeConst0 ) ), sin( nodeConst0 ), cos( nodeConst0 ) );

}


fn fn22 ( viewPosition : vec3<f32>, projectionMatrix : mat4x4<f32> ) -> vec2<f32> {

	var nodeVar0 : vec2<f32>;

	let nodeConst0 = ( projectionMatrix * vec4<f32>( viewPosition, 1.0 ) );
	nodeVar0 = ( ( ( nodeConst0.xy / vec2<f32>( nodeConst0.w ) ) * vec2<f32>( 0.5 ) ) + vec2<f32>( 0.5 ) );

	return vec2<f32>( nodeVar0.x, ( 1.0 - nodeVar0.y ) );

}


fn fn157 ( position : vec3<f32>, nPosition : vec3<f32>, normal : vec3<f32> ) -> f32 {

	


	return abs( dot( ( position - nPosition ), normal ) );

}


fn fn156 ( viewNormal : vec3<f32>, nNormalV : vec3<f32>, lobeFalloff : f32 ) -> f32 {

	


	return exp( ( ( dot( viewNormal, nNormalV ) - 1.0 ) * lobeFalloff ) );

}




@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar1 = textureDimensions( nodeUniform0, u32( 0 ) );
	nodeVar0 = textureLoad( nodeUniform0, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVarying0 ) * vec2<f32>( nodeVar1 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar1 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst0 = nodeVar0;

	if ( ( nodeConst0 >= 1.0 ) ) {

		discard;
		

	} else {

		nodeVar2 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVarying0 );
		let nodeConst1 = ( ( nodeVar2.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) );
		let nodeConst2 = normalize( ( vec4<f32>( nodeConst1, 0.0 ) * object.nodeUniform2 ).xyz );
		nodeVar3 = textureSample( nodeUniform3, nodeUniform3_sampler, nodeVarying0 );
		let nodeConst3 = max( max( nodeVar3, vec4<f32>( 0.0 ) ), vec4<f32>( 0.0 ) );
		let nodeConst4 = ( object.nodeUniform4 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVarying0.x, ( 1.0 - nodeVarying0.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeConst0 ), 1.0 ) );
		let nodeConst5 = ( nodeConst4.xyz / vec3<f32>( nodeConst4.w ) );
		nodeVar4 = textureSample( nodeUniform5, nodeUniform5_sampler, nodeVarying0 );
		nodeVar5 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVarying0 );
		let nodeConst6 = vec2<f32>( nodeVar4.w, nodeVar5.w );
		nodeVar6 = vec4<f32>( 0.0, 0.0, 0.0, 1.0 );
		let nodeConst7 = ( i32( object.nodeUniform7 ) + 83 );
		let nodeConst8 = tsl_mod_vec2( ( floor( ( nodeVarying0 * object.nodeUniform6 ) ) + floor( ( fract( vec2<f32>( ( f32( nodeConst7 ) * 0.7548776662 ), ( f32( nodeConst7 ) * 0.569840291 ) ) ) * vec2<f32>( 32.0 ) ) ) ), vec2<f32>( 32.0 ) );
		let nodeConst9 = ( ( ( nodeConst8.x * 0.7548776662466927 ) + ( nodeConst8.y * 0.5698402909980532 ) ) + 83.0 );
		nodeVar6 = vec4<f32>( fract( ( ( nodeConst9 * 1.324717957244746 ) * 0.7548776662466927 ) ), fract( ( ( nodeConst9 * 2.649435914489492 ) * 0.5698402909980532 ) ), fract( ( ( nodeConst9 * 3.974153871734238 ) * 0.419875421 ) ), fract( ( ( nodeConst9 * 5.298871828978984 ) * 0.43015970900194667 ) ) );
		var nodeVar7 : vec4<f32> = nodeVar6;
		nodeVar8 = textureSample( nodeUniform8, nodeUniform8_sampler, nodeVarying0 );
		let nodeConst10 = max( nodeVar8, vec4<f32>( 0.0 ) );
		nodeVar9 = 1.0;
		nodeVar10 = 0.0;
		nodeVar11 = 0.0;
		nodeVar12 = false;
		nodeVar13 = 0.0;
		nodeVar14 = 0.0;
		nodeVar15 = 0.0;
		nodeVar16 = 0.0;
		nodeVar17 = 0.0;
		nodeVar18 = false;
		nodeVar19 = nodeConst10.w;

		if ( ( nodeVar19 > 1000.0 ) ) {

			nodeVar19 = 0.25;
			nodeVar18 = true;
			

		}

		let nodeConst11 = ( 1.0 / ( nodeVar19 + 0.001 ) );
		nodeVar13 = ( nodeVar13 + ( nodeVar19 * nodeConst11 ) );
		nodeVar14 = ( nodeVar14 + nodeConst11 );

		if ( ( object.nodeUniform9 > 0.0 ) ) {

			nodeVar17 = ( nodeVar17 + 1.0 );
			let nodeConst12 = dot( nodeConst10.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) );
			let nodeConst13 = ( nodeConst12 - nodeVar15 );
			nodeVar15 = ( nodeVar15 + ( nodeConst13 / nodeVar17 ) );
			nodeVar16 = ( nodeVar16 + ( nodeConst13 * ( nodeConst12 - nodeVar15 ) ) );
			

		}

		nodeVar20 = textureSample( nodeUniform8, nodeUniform8_sampler, ( nodeVarying0 + ( vec2<f32>( -1.0, 0.0 ) / object.nodeUniform6 ) ) );
		let nodeConst14 = max( nodeVar20, vec4<f32>( 0.0 ) );
		nodeVar21 = nodeConst14.w;

		if ( ( nodeVar21 > 1000.0 ) ) {

			nodeVar21 = 0.25;
			nodeVar18 = true;
			

		}

		let nodeConst15 = ( 1.0 / ( nodeVar21 + 0.001 ) );
		nodeVar13 = ( nodeVar13 + ( nodeVar21 * nodeConst15 ) );
		nodeVar14 = ( nodeVar14 + nodeConst15 );

		if ( ( object.nodeUniform9 > 0.0 ) ) {

			nodeVar17 = ( nodeVar17 + 1.0 );
			let nodeConst16 = dot( nodeConst14.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) );
			let nodeConst17 = ( nodeConst16 - nodeVar15 );
			nodeVar15 = ( nodeVar15 + ( nodeConst17 / nodeVar17 ) );
			nodeVar16 = ( nodeVar16 + ( nodeConst17 * ( nodeConst16 - nodeVar15 ) ) );
			

		}

		nodeVar22 = textureSample( nodeUniform8, nodeUniform8_sampler, ( nodeVarying0 + ( vec2<f32>( 1.0, 0.0 ) / object.nodeUniform6 ) ) );
		let nodeConst18 = max( nodeVar22, vec4<f32>( 0.0 ) );
		nodeVar23 = nodeConst18.w;

		if ( ( nodeVar23 > 1000.0 ) ) {

			nodeVar23 = 0.25;
			nodeVar18 = true;
			

		}

		let nodeConst19 = ( 1.0 / ( nodeVar23 + 0.001 ) );
		nodeVar13 = ( nodeVar13 + ( nodeVar23 * nodeConst19 ) );
		nodeVar14 = ( nodeVar14 + nodeConst19 );

		if ( ( object.nodeUniform9 > 0.0 ) ) {

			nodeVar17 = ( nodeVar17 + 1.0 );
			let nodeConst20 = dot( nodeConst18.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) );
			let nodeConst21 = ( nodeConst20 - nodeVar15 );
			nodeVar15 = ( nodeVar15 + ( nodeConst21 / nodeVar17 ) );
			nodeVar16 = ( nodeVar16 + ( nodeConst21 * ( nodeConst20 - nodeVar15 ) ) );
			

		}

		nodeVar24 = textureSample( nodeUniform8, nodeUniform8_sampler, ( nodeVarying0 + ( vec2<f32>( 0.0, -1.0 ) / object.nodeUniform6 ) ) );
		let nodeConst22 = max( nodeVar24, vec4<f32>( 0.0 ) );
		nodeVar25 = nodeConst22.w;

		if ( ( nodeVar25 > 1000.0 ) ) {

			nodeVar25 = 0.25;
			nodeVar18 = true;
			

		}

		let nodeConst23 = ( 1.0 / ( nodeVar25 + 0.001 ) );
		nodeVar13 = ( nodeVar13 + ( nodeVar25 * nodeConst23 ) );
		nodeVar14 = ( nodeVar14 + nodeConst23 );

		if ( ( object.nodeUniform9 > 0.0 ) ) {

			nodeVar17 = ( nodeVar17 + 1.0 );
			let nodeConst24 = dot( nodeConst22.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) );
			let nodeConst25 = ( nodeConst24 - nodeVar15 );
			nodeVar15 = ( nodeVar15 + ( nodeConst25 / nodeVar17 ) );
			nodeVar16 = ( nodeVar16 + ( nodeConst25 * ( nodeConst24 - nodeVar15 ) ) );
			

		}

		nodeVar26 = textureSample( nodeUniform8, nodeUniform8_sampler, ( nodeVarying0 + ( vec2<f32>( 0.0, 1.0 ) / object.nodeUniform6 ) ) );
		let nodeConst26 = max( nodeVar26, vec4<f32>( 0.0 ) );
		nodeVar27 = nodeConst26.w;

		if ( ( nodeVar27 > 1000.0 ) ) {

			nodeVar27 = 0.25;
			nodeVar18 = true;
			

		}

		let nodeConst27 = ( 1.0 / ( nodeVar27 + 0.001 ) );
		nodeVar13 = ( nodeVar13 + ( nodeVar27 * nodeConst27 ) );
		nodeVar14 = ( nodeVar14 + nodeConst27 );

		if ( ( object.nodeUniform9 > 0.0 ) ) {

			nodeVar17 = ( nodeVar17 + 1.0 );
			let nodeConst28 = dot( nodeConst26.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) );
			let nodeConst29 = ( nodeConst28 - nodeVar15 );
			nodeVar15 = ( nodeVar15 + ( nodeConst29 / nodeVar17 ) );
			nodeVar16 = ( nodeVar16 + ( nodeConst29 * ( nodeConst28 - nodeVar15 ) ) );
			

		}

		let nodeConst30 = vec4<f32>( ( nodeVar13 / nodeVar14 ), nodeVar15, sqrt( ( nodeVar16 / max( nodeVar17, 1.0 ) ) ), f32( nodeVar18 ) );
		nodeVar9 = nodeConst30.x;
		nodeVar10 = nodeConst30.y;
		nodeVar11 = nodeConst30.z;
		nodeVar12 = ( nodeConst30.w > 0.5 );
		let nodeConst31 = tan( ( object.nodeUniform10 * 0.5 ) );
		let nodeConst32 = fn151( nodeVar9, abs( nodeConst5.z ), nodeConst31 );
		nodeVar28 = nodeConst3.xyz;
		nodeVar29 = 1.0;
		let nodeConst33 = ( 1.0 / nodeConst3.w );
		nodeVar30 = nodeConst33;
		nodeVar31 = 1.0;
		nodeVar32 = nodeConst10.xyz;
		nodeVar33 = 1.0;

		if ( ( length( nodeConst10.xyz ) < 0.0001 ) ) {

			nodeVar32 = vec3<f32>( 0.0, 0.0, 0.0 );
			nodeVar33 = 0.0;
			

		}

		nodeVar34 = ( object.nodeUniform11 * 0.1 );
		nodeVar34 = ( nodeVar34 * ( nodeVar9 * abs( nodeConst5.z ) ) );
		nodeVar34 = ( nodeVar34 * max( sqrt( nodeConst6.y ), 0.01 ) );
		let nodeConst34 = ( 1.0 - fn149( nodeConst33, ( 1.0 - object.nodeUniform12 ) ) );
		nodeVar34 = ( nodeVar34 * mix( 1.0, 0.001, nodeConst34 ) );
		nodeVar35 = vec3<f32>( 0.0, 0.0, 0.0 );
		nodeVar36 = vec3<f32>( 0.0, 0.0, 0.0 );
		let nodeConst35 = reflect( ( - fn153( nodeConst1, ( - normalize( nodeConst5 ) ), nodeConst6.y ) ), nodeConst1 );
		let nodeConst36 = normalize( cross( nodeConst1, nodeConst35 ) );
		nodeVar35 = ( nodeConst36 * vec3<f32>( mix( 1.0, nodeConst6.y, clamp( ( acos( abs( nodeConst1.z ) ) / 1.5707963267948966 ), 0.0, 1.0 ) ) ) );
		nodeVar36 = cross( nodeConst35, nodeConst36 );
		nodeVar35 = ( nodeVar35 * vec3<f32>( nodeVar34 ) );
		nodeVar36 = ( nodeVar36 * vec3<f32>( nodeVar34 ) );
		const nodeConst37 = vec3<f32>( 0.0, 0.0, 0.0 );
		nodeVar37 = 1.0;
		nodeVar38 = vec2<f32>( 0.0, 0.0 );
		let nodeConst38 = fn155( nodeConst6.y, nodeConst34, ( 1.0 - object.nodeUniform13 ) );

		for ( var i : i32 = 0; i < 8; i ++ ) {

			nodeVar39 = fn146( f32( i ), 1.0 );
			let nodeConst39 = normalize( nodeVar39 );

			if ( ( dot( nodeVar38, nodeVar38 ) > 0.001 ) ) {

				nodeVar40 = 1.0;

			} else {

				nodeVar40 = 0.0;

			}

			nodeVar41 = ( fn160( nodeVar7.x ) * ( mix( nodeConst39, normalize( max( nodeVar38, vec2<f32>( 0.000001 ) ) ), ( ( object.nodeUniform14 * nodeConst34 ) * nodeVar40 ) ) * vec2<f32>( ( length( nodeVar39 ) * nodeVar37 ) ) ) );
			nodeVar42 = fn22( ( nodeConst5 + ( ( nodeVar36 * vec3<f32>( nodeVar41.x ) ) + ( nodeVar35 * vec3<f32>( nodeVar41.y ) ) ) ), object.nodeUniform15 );
			nodeVar42 = clamp( ( vec2<f32>( 1.0 ) - abs( ( vec2<f32>( 1.0 ) - abs( nodeVar42 ) ) ) ), vec2<f32>( 0.0 ), vec2<f32>( 1.0 ) );
			nodeVar43 = textureSample( nodeUniform3, nodeUniform3_sampler, nodeVar42 );
			let nodeConst40 = max( max( nodeVar43, vec4<f32>( 0.0 ) ), vec4<f32>( 0.0 ) );
			nodeVar44 = textureSample( nodeUniform8, nodeUniform8_sampler, nodeVar42 );
			nodeVar45 = max( max( nodeVar44, vec4<f32>( 0.0 ) ), vec4<f32>( 0.0 ) );
			nodeVar46 = textureLoad( nodeUniform0, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar42 ) * vec2<f32>( nodeVar1 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar1 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
			let nodeConst41 = ( object.nodeUniform4 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar42.x, ( 1.0 - nodeVar42.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar46 ), 1.0 ) );
			let nodeConst42 = ( nodeConst41.xyz / vec3<f32>( nodeConst41.w ) );
			let nodeConst43 = abs( nodeConst42.z );
			nodeVar47 = 0.0;
			nodeVar47 = ( nodeVar47 + ( ( abs( ( dot( nodeVar45.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - dot( nodeConst10.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) ) ) * object.nodeUniform16 ) * 10.0 ) );

			if ( ( ( nodeVar45.w > 1000.0 ) && nodeVar12 ) ) {

				nodeVar48 = 1.0;

			} else {

				nodeVar48 = ( ( abs( ( nodeConst32 - fn151( nodeVar45.w, nodeConst43, nodeConst31 ) ) ) * object.nodeUniform17 ) / abs( nodeConst5.z ) );

			}

			nodeVar47 = ( nodeVar47 + nodeVar48 );
			nodeVar49 = textureSample( nodeUniform5, nodeUniform5_sampler, nodeVar42 );
			nodeVar50 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar42 );
			nodeVar47 = ( nodeVar47 + ( abs( ( nodeConst6.y - vec2<f32>( nodeVar49.w, nodeVar50.w ).y ) ) * object.nodeUniform18 ) );
			nodeVar51 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar42 );
			nodeVar52 = ( exp( ( - ( ( nodeVar47 * nodeConst34 ) + ( fn157( nodeConst5, nodeConst42, nodeConst1 ) * ( ( ( object.nodeUniform19 * 500.0 ) * abs( nodeConst1.z ) ) / abs( nodeConst5.z ) ) ) ) ) ) * fn156( nodeConst2, normalize( ( vec4<f32>( ( ( nodeVar51.xyz * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ), 0.0 ) * object.nodeUniform2 ).xyz ), nodeConst38 ) );
			nodeVar37 = mix( nodeVar37, nodeVar52, object.nodeUniform14 );
			nodeVar38 = mix( nodeVar38, ( nodeConst39 * vec2<f32>( ( nodeVar52 - 0.5 ) ) ), 0.5 );
			nodeVar52 = ( nodeVar52 * mix( ( 1.0 / ( pow( dot( nodeVar45.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ), 2.0 ) + 0.01 ) ), 1.0, min( ( nodeConst33 / 5.0 ), 1.0 ) ) );
			nodeVar32 = ( nodeVar32 + ( nodeVar45.xyz * vec3<f32>( nodeVar52 ) ) );
			nodeVar33 = ( nodeVar33 + nodeVar52 );
			nodeVar28 = ( nodeVar28 + ( nodeConst40.xyz * vec3<f32>( nodeVar52 ) ) );
			nodeVar29 = ( nodeVar29 + nodeVar52 );
			nodeVar53 = bool( object.nodeUniform20 );

			if ( nodeVar53 ) {


				if ( ( nodeConst40.w > nodeConst3.w ) ) {

					nodeVar54 = ( nodeVar52 * 0.33 );

				} else {

					nodeVar54 = 0.0;

				}

				nodeVar30 = ( nodeVar30 + ( ( 1.0 / nodeConst40.w ) * nodeVar54 ) );
				nodeVar31 = ( nodeVar31 + nodeVar54 );
				

			}


		}

		nodeVar28 = ( nodeVar28 / vec3<f32>( max( nodeVar29, 0.000001 ) ) );
		nodeVar28 = max( nodeVar28, vec3<f32>( 0.000001 ) );
		nodeVar32 = ( nodeVar32 / vec3<f32>( max( nodeVar33, 0.000001 ) ) );
		let nodeConst44 = ( 1.0 / max( ( nodeVar30 / max( nodeVar31, 0.000001 ) ), 0.000001 ) );
		let nodeConst45 = ( nodeVar11 / max( nodeVar10, 0.0001 ) );
		let nodeConst46 = ( nodeConst44 * ( 1.0 - clamp( ( ( nodeConst45 * object.nodeUniform9 ) * ( 1.0 - nodeConst44 ) ), 0.0, 0.9 ) ) );
		let nodeConst47 = ( object.nodeUniform21 * mix( ( 1.0 - object.nodeUniform9 ), 1.0, smoothstep( 0.1, 2.0, nodeConst45 ) ) );
		let nodeConst48 = ( ( 1.0 - nodeConst46 ) / ( ( ( dot( nodeVar28, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * nodeConst47 ) * 10.0 ) + 1.0 ) );
		let nodeConst49 = ( nodeConst46 / ( ( ( dot( nodeVar32, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) * nodeConst47 ) * 10.0 ) + 1.0 ) );
		nodeVar55 = vec4<f32>( ( ( ( nodeVar28 * vec3<f32>( nodeConst48 ) ) + ( nodeVar32 * vec3<f32>( nodeConst49 ) ) ) / vec3<f32>( max( ( nodeConst48 + nodeConst49 ), 0.000001 ) ) ), nodeConst44 );
		

	}


	// result

	output.color = nodeVar55;

	return output;

}
