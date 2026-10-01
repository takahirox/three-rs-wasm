// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 0 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 0 ) var nodeUniform0 : texture_2d<f32>;
@binding( 3 ) @group( 0 ) var nodeUniform5 : texture_2d<f32>;
@binding( 4 ) @group( 0 ) var nodeUniform11_sampler : sampler;
@binding( 5 ) @group( 0 ) var nodeUniform11 : texture_2d<f32>;

struct objectStruct {
	nodeUniform1 : mat3x3<f32>,
	nodeUniform2 : vec2<f32>,
	nodeUniform3 : mat3x3<f32>,
	nodeUniform4 : vec2<f32>,
	nodeUniform6 : mat3x3<f32>,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform8 : mat3x3<f32>,
	nodeUniform9 : mat3x3<f32>,
	nodeUniform10 : mat3x3<f32>,
	nodeUniform12 : mat3x3<f32>,
	nodeUniform13 : mat3x3<f32>,
	nodeUniform14 : mat3x3<f32>,
	nodeUniform15 : mat3x3<f32>,
	nodeUniform16 : mat3x3<f32>,
	nodeUniform17 : mat3x3<f32>,
	nodeUniform18 : mat3x3<f32>,
	nodeUniform19 : mat3x3<f32>
};
@binding( 2 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : vec4<f32>;
var<private> nodeVar3 : vec2<f32>;
var<private> nodeVar4 : vec2<f32>;
var<private> nodeVar5 : vec2<f32>;
var<private> nodeVar6 : vec2<f32>;
var<private> nodeVar8 : vec2<f32>;
var<private> nodeVar9 : vec4<f32>;
var<private> nodeVar12 : vec2<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec2<u32>;
var<private> nodeVar15 : vec4<f32>;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : vec2<f32>;
var<private> nodeVar18 : vec2<f32>;
var<private> nodeVar19 : vec2<f32>;
var<private> nodeVar20 : vec4<f32>;
var<private> nodeVar21 : vec2<f32>;
var<private> nodeVar22 : vec4<f32>;
var<private> nodeVar23 : vec4<f32>;
var<private> nodeVar24 : f32;
var<private> nodeVar26 : vec2<f32>;
var<private> nodeVar27 : vec4<f32>;
var<private> nodeVar28 : vec2<f32>;
var<private> nodeVar29 : vec2<f32>;
var<private> nodeVar30 : vec2<f32>;
var<private> nodeVar31 : vec2<f32>;
var<private> nodeVar32 : vec2<f32>;
var<private> nodeVar33 : vec4<f32>;
var<private> nodeVar34 : vec2<f32>;
var<private> nodeVar35 : vec4<f32>;
var<private> nodeVar36 : vec2<u32>;
var<private> nodeVar37 : vec4<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : vec2<f32>;
var<private> nodeVar40 : vec2<f32>;
var<private> nodeVar41 : vec2<f32>;
var<private> nodeVar42 : vec4<f32>;
var<private> nodeVar43 : vec2<f32>;
var<private> nodeVar44 : vec4<f32>;
var<private> nodeVar45 : vec4<f32>;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : vec2<f32>;
var<private> nodeVar48 : vec4<f32>;
var<private> nodeVar49 : vec2<f32>;

// codes
fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}



@fragment
fn main( @location( 0 ) nodeVarying0 : vec4<f32>,
	@location( 1 ) nodeVarying1 : vec4<f32>,
	@location( 2 ) nodeVarying2 : vec4<f32>,
	@location( 3 ) nodeVarying3 : vec2<f32>,
	@location( 4 ) nodeVarying4 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = vec4<f32>( 0.0, 0.0, 0.0, 0.0 );
	nodeVar1 = vec4<f32>( 0.0, 0.0, 0.0, 0.0 );
	nodeVar2 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform1 * vec3<f32>( nodeVarying4, 1.0 ) ).xy );
	nodeVar3 = nodeVar2.xy;

	if ( ( nodeVar3.y > 0.0 ) ) {

		nodeVar4 = vec2<f32>( 0.0, 0.0 );
		nodeVar5 = vec2<f32>( 0.0, 0.0 );
		nodeVar6 = vec2<f32>( 0.0, 1.0 );
		nodeVar8 = nodeVarying0.xy;

		for ( var i : i32 = 0; i < 8; i ++ ) {

			nodeVar9 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform3 * vec3<f32>( nodeVar8, 1.0 ) ).xy );
			nodeVar6 = nodeVar9.xy;
			nodeVar8 = ( nodeVar8 - ( vec2<f32>( 2.0, 0.0 ) * object.nodeUniform4 ) );

			if ( ( ( nodeVar8.x <= nodeVarying1.x ) || ( ( nodeVar6.y <= 0.8281 ) || ( nodeVar6.x != 0.0 ) ) ) ) {

				break;
				

			}


		}

		nodeVar8.x = ( nodeVar8.x + ( 0.25 * object.nodeUniform4.x ) );
		nodeVar8.x = ( nodeVar8.x + object.nodeUniform4.x );
		nodeVar8.x = ( nodeVar8.x + ( 2.0 * object.nodeUniform4.x ) );
		nodeVar12 = nodeVar6;
		nodeVar12.x = ( 0.0 + ( nodeVar12.x * 0.5 ) );
		nodeVar14 = textureDimensions( nodeUniform5, u32( 0 ) );
		nodeVar13 = textureLoad( nodeUniform5, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( object.nodeUniform6 * vec3<f32>( nodeVar12, 1.0 ) ).xy ) * vec2<f32>( nodeVar14 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar14 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		nodeVar8.x = ( nodeVar8.x - ( object.nodeUniform4.x * ( vec4<f32>( 255.0 ) * nodeVar13 ).x ) );
		nodeVar5.x = nodeVar8.x;
		nodeVar5.y = nodeVarying2.y;
		nodeVar4.x = nodeVar5.x;
		nodeVar15 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform7 * vec3<f32>( nodeVar5, 1.0 ) ).xy );
		nodeVar16 = nodeVar15.x;
		nodeVar17 = vec2<f32>( 0.0, 0.0 );
		nodeVar18 = vec2<f32>( 0.0, 1.0 );
		nodeVar19 = nodeVarying0.zw;

		for ( var i : i32 = 0; i < 8; i ++ ) {

			nodeVar20 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform8 * vec3<f32>( nodeVar19, 1.0 ) ).xy );
			nodeVar18 = nodeVar20.xy;
			nodeVar19 = ( nodeVar19 + ( vec2<f32>( 2.0, 0.0 ) * object.nodeUniform4 ) );

			if ( ( ( nodeVar19.x >= nodeVarying1.y ) || ( ( nodeVar18.y <= 0.8281 ) || ( nodeVar18.x != 0.0 ) ) ) ) {

				break;
				

			}


		}

		nodeVar19.x = ( nodeVar19.x - ( 0.25 * object.nodeUniform4.x ) );
		nodeVar19.x = ( nodeVar19.x - object.nodeUniform4.x );
		nodeVar19.x = ( nodeVar19.x - ( 2.0 * object.nodeUniform4.x ) );
		nodeVar21 = nodeVar18;
		nodeVar21.x = ( 0.5 + ( nodeVar21.x * 0.5 ) );
		nodeVar22 = textureLoad( nodeUniform5, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( object.nodeUniform9 * vec3<f32>( nodeVar21, 1.0 ) ).xy ) * vec2<f32>( nodeVar14 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar14 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		nodeVar19.x = ( nodeVar19.x + ( object.nodeUniform4.x * ( vec4<f32>( 255.0 ) * nodeVar22 ).x ) );
		nodeVar17.x = nodeVar19.x;
		nodeVar17.y = nodeVarying2.y;
		nodeVar4.y = nodeVar17.x;
		nodeVar23 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform10 * vec3<f32>( ( nodeVar17 + ( vec2<f32>( 1.0, 0.0 ) * object.nodeUniform4 ) ), 1.0 ) ).xy );
		nodeVar24 = nodeVar23.x;
		nodeVar0.x = nodeVar24;
		nodeVar26 = ( ( vec2<f32>( 0.00625, 0.0017857142857142857 ) * ( ( vec2<f32>( 16.0 ) * round( ( vec2<f32>( 4.0 ) * vec2<f32>( nodeVar16, nodeVar24 ) ) ) ) + sqrt( abs( ( ( nodeVar4 / vec2<f32>( object.nodeUniform4.x ) ) - vec2<f32>( nodeVarying3.x ) ) ) ) ) ) + ( vec2<f32>( 0.5 ) * vec2<f32>( 0.00625, 0.0017857142857142857 ) ) );
		nodeVar26.y = ( nodeVar26.y + ( 0.14285714285714285 * nodeVar1.y ) );
		nodeVar27 = textureSample( nodeUniform11, nodeUniform11_sampler, ( object.nodeUniform12 * vec3<f32>( nodeVar26, 1.0 ) ).xy );
		nodeVar28 = nodeVar27.xy;
		nodeVar0.x = nodeVar28[ 0 ];
		nodeVar0.y = nodeVar28[ 1 ];
		

	}


	if ( ( nodeVar3.x > 0.0 ) ) {

		nodeVar29 = vec2<f32>( 0.0, 0.0 );
		nodeVar30 = vec2<f32>( 0.0, 0.0 );
		nodeVar31 = vec2<f32>( 1.0, 0.0 );
		nodeVar32 = nodeVarying2.xy;

		for ( var i : i32 = 0; i < 8; i ++ ) {

			nodeVar33 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform13 * vec3<f32>( nodeVar32, 1.0 ) ).xy );
			nodeVar31 = nodeVar33.xy;
			nodeVar32 = ( nodeVar32 + ( vec2<f32>( 0.0, -2.0 ) * object.nodeUniform4 ) );

			if ( ( ( nodeVar32.y <= nodeVarying1.z ) || ( ( nodeVar31.x <= 0.8281 ) || ( nodeVar31.y != 0.0 ) ) ) ) {

				break;
				

			}


		}

		nodeVar32.y = ( nodeVar32.y + ( 0.25 * object.nodeUniform4.y ) );
		nodeVar32.y = ( nodeVar32.y + object.nodeUniform4.y );
		nodeVar32.y = ( nodeVar32.y + ( 2.0 * object.nodeUniform4.y ) );
		nodeVar34 = nodeVar31.yx;
		nodeVar34.x = ( 0.0 + ( nodeVar34.x * 0.5 ) );
		nodeVar36 = textureDimensions( nodeUniform5, u32( 0 ) );
		nodeVar35 = textureLoad( nodeUniform5, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( object.nodeUniform14 * vec3<f32>( nodeVar34, 1.0 ) ).xy ) * vec2<f32>( nodeVar36 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar36 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		nodeVar32.y = ( nodeVar32.y - ( object.nodeUniform4.y * ( vec4<f32>( 255.0 ) * nodeVar35 ).x ) );
		nodeVar30.y = nodeVar32.y;
		nodeVar30.x = nodeVarying0.x;
		nodeVar29.x = nodeVar30.y;
		nodeVar37 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform15 * vec3<f32>( nodeVar30, 1.0 ) ).xy );
		nodeVar38 = nodeVar37.y;
		nodeVar39 = vec2<f32>( 0.0, 0.0 );
		nodeVar40 = vec2<f32>( 1.0, 0.0 );
		nodeVar41 = nodeVarying2.zw;

		for ( var i : i32 = 0; i < 8; i ++ ) {

			nodeVar42 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform16 * vec3<f32>( nodeVar41, 1.0 ) ).xy );
			nodeVar40 = nodeVar42.xy;
			nodeVar41 = ( nodeVar41 - ( vec2<f32>( 0.0, -2.0 ) * object.nodeUniform4 ) );

			if ( ( ( nodeVar41.y >= nodeVarying1.w ) || ( ( nodeVar40.x <= 0.8281 ) || ( nodeVar40.y != 0.0 ) ) ) ) {

				break;
				

			}


		}

		nodeVar41.y = ( nodeVar41.y - ( 0.25 * object.nodeUniform4.y ) );
		nodeVar41.y = ( nodeVar41.y - object.nodeUniform4.y );
		nodeVar41.y = ( nodeVar41.y - ( 2.0 * object.nodeUniform4.y ) );
		nodeVar43 = nodeVar40.yx;
		nodeVar43.x = ( 0.5 + ( nodeVar43.x * 0.5 ) );
		nodeVar44 = textureLoad( nodeUniform5, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( object.nodeUniform17 * vec3<f32>( nodeVar43, 1.0 ) ).xy ) * vec2<f32>( nodeVar36 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar36 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
		nodeVar41.y = ( nodeVar41.y + ( object.nodeUniform4.y * ( vec4<f32>( 255.0 ) * nodeVar44 ).x ) );
		nodeVar39.y = nodeVar41.y;
		nodeVar39.x = nodeVarying0.x;
		nodeVar29.y = nodeVar39.y;
		nodeVar45 = textureSample( nodeUniform0, nodeUniform0_sampler, ( object.nodeUniform18 * vec3<f32>( ( nodeVar39 + ( vec2<f32>( 0.0, 1.0 ) * object.nodeUniform4 ) ), 1.0 ) ).xy );
		nodeVar46 = nodeVar45.y;
		nodeVar47 = ( ( vec2<f32>( 0.00625, 0.0017857142857142857 ) * ( ( vec2<f32>( 16.0 ) * round( ( vec2<f32>( 4.0 ) * vec2<f32>( nodeVar38, nodeVar46 ) ) ) ) + sqrt( abs( ( ( nodeVar29 / vec2<f32>( object.nodeUniform4.y ) ) - vec2<f32>( nodeVarying3.y ) ) ) ) ) ) + ( vec2<f32>( 0.5 ) * vec2<f32>( 0.00625, 0.0017857142857142857 ) ) );
		nodeVar47.y = ( nodeVar47.y + ( 0.14285714285714285 * nodeVar1.x ) );
		nodeVar48 = textureSample( nodeUniform11, nodeUniform11_sampler, ( object.nodeUniform19 * vec3<f32>( nodeVar47, 1.0 ) ).xy );
		nodeVar49 = nodeVar48.xy;
		nodeVar0.z = nodeVar49[ 0 ];
		nodeVar0.w = nodeVar49[ 1 ];
		

	}


	// result

	output.color = nodeVar0;

	return output;

}
