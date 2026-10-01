// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 0 ) @group( 1 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform0 : texture_2d<f32>;
@binding( 2 ) @group( 1 ) var nodeUniform1_sampler : sampler;
@binding( 3 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform6 : texture_depth_multisampled_2d;

struct renderStruct {
	nodeUniform2 : vec2<f32>,
	nodeUniform8 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

struct objectStruct {
	nodeUniform3 : f32,
	nodeUniform4 : f32,
	nodeUniform5 : mat4x4<f32>,
	nodeUniform7 : f32
};
@binding( 4 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : vec2<f32>;
var<private> nodeVar4 : vec2<f32>;
var<private> nodeVar5 : vec2<f32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : vec2<u32>;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec2<f32>;
var<private> nodeVar16 : vec2<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : vec2<f32>;
var<private> nodeVar22 : vec2<f32>;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : vec2<f32>;
var<private> nodeVar28 : vec2<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec2<f32>;
var<private> nodeVar34 : vec2<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : vec2<f32>;
var<private> nodeVar40 : vec2<f32>;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : vec2<f32>;
var<private> nodeVar46 : vec2<f32>;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec2<f32>;
var<private> nodeVar52 : vec2<f32>;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : f32;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : vec2<f32>;
var<private> nodeVar58 : vec2<f32>;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : f32;
var<private> nodeVar61 : f32;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : vec2<f32>;
var<private> nodeVar64 : vec2<f32>;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : vec2<f32>;
var<private> nodeVar70 : vec2<f32>;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : f32;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : vec2<f32>;
var<private> nodeVar76 : vec2<f32>;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : f32;
var<private> nodeVar79 : f32;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : vec2<f32>;
var<private> nodeVar82 : vec2<f32>;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : vec2<f32>;
var<private> nodeVar88 : vec2<f32>;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : f32;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : vec2<f32>;
var<private> nodeVar94 : vec2<f32>;
var<private> nodeVar95 : f32;
var<private> nodeVar96 : f32;
var<private> nodeVar97 : f32;
var<private> nodeVar98 : f32;
var<private> nodeVar99 : vec2<f32>;
var<private> nodeVar100 : vec2<f32>;
var<private> nodeVar101 : f32;
var<private> nodeVar102 : f32;
var<private> nodeVar103 : f32;
var<private> nodeVar104 : f32;
var<private> nodeVar105 : vec2<f32>;
var<private> nodeVar106 : vec2<f32>;
var<private> nodeVar107 : f32;
var<private> nodeVar108 : f32;
var<private> nodeVar109 : f32;
var<private> nodeVar110 : f32;
var<private> nodeVar111 : vec2<f32>;
var<private> nodeVar112 : vec2<f32>;
var<private> nodeVar113 : f32;
var<private> nodeVar114 : f32;
var<private> nodeVar115 : f32;
var<private> nodeVar116 : f32;
var<private> nodeVar117 : vec2<f32>;
var<private> nodeVar118 : vec2<f32>;
var<private> nodeVar119 : f32;
var<private> nodeVar120 : f32;
var<private> nodeVar121 : f32;
var<private> nodeVar122 : f32;
var<private> nodeVar123 : vec2<f32>;
var<private> nodeVar124 : vec2<f32>;
var<private> nodeVar125 : f32;
var<private> nodeVar126 : f32;
var<private> nodeVar127 : f32;
var<private> nodeVar128 : f32;
var<private> nodeVar129 : vec2<f32>;
var<private> nodeVar130 : vec2<f32>;
var<private> nodeVar131 : f32;
var<private> nodeVar132 : f32;
var<private> nodeVar133 : f32;
var<private> nodeVar134 : f32;
var<private> nodeVar135 : vec2<f32>;
var<private> nodeVar136 : vec2<f32>;
var<private> nodeVar137 : f32;
var<private> nodeVar138 : f32;
var<private> nodeVar139 : f32;
var<private> nodeVar140 : f32;
var<private> nodeVar141 : vec2<f32>;
var<private> nodeVar142 : vec2<f32>;
var<private> nodeVar143 : f32;
var<private> nodeVar144 : f32;
var<private> nodeVar145 : f32;
var<private> nodeVar146 : f32;
var<private> nodeVar147 : vec2<f32>;
var<private> nodeVar148 : vec2<f32>;
var<private> nodeVar149 : f32;
var<private> nodeVar150 : f32;
var<private> nodeVar151 : f32;
var<private> nodeVar152 : f32;
var<private> nodeVar153 : vec2<f32>;
var<private> nodeVar154 : vec2<f32>;
var<private> nodeVar155 : f32;
var<private> nodeVar156 : f32;
var<private> nodeVar157 : f32;
var<private> nodeVar158 : f32;
var<private> nodeVar159 : vec3<f32>;
var<private> nodeVar160 : vec4<f32>;
var<private> nodeVar161 : vec4<f32>;

// codes
fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}

fn fn1 ( color : vec4<f32> ) -> vec4<f32> {

	var nodeVar0 : vec4<f32>;


	if ( ( color.w == 0.0 ) ) {

		nodeVar0 = vec4<f32>( 0.0, 0.0, 0.0, 0.0 );

	} else {

		nodeVar0 = vec4<f32>( ( color.xyz / vec3<f32>( color.w ) ), color.w );

	}


	return nodeVar0;

}


fn acesFilmicToneMapping ( color : vec3<f32>, exposure : f32 ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;

	nodeVar0 = ( mat3x3<f32>( 0.59719, 0.076, 0.0284, 0.35458, 0.90834, 0.13383, 0.04823, 0.01566, 0.83777 ) * ( ( color * vec3<f32>( exposure ) ) / vec3<f32>( 0.6 ) ) );

	return clamp( ( mat3x3<f32>( 1.60475, -0.10208, -0.00327, -0.53108, 1.10813, -0.07276, -0.07367, -0.00605, 1.07602 ) * ( ( ( nodeVar0 * ( nodeVar0 + vec3<f32>( 0.0245786 ) ) ) - vec3<f32>( 0.000090537 ) ) / ( ( nodeVar0 * ( ( nodeVar0 + vec3<f32>( 0.432951 ) ) * vec3<f32>( 0.983729 ) ) ) + vec3<f32>( 0.238081 ) ) ) ), vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) );

}


fn sRGBTransferOETF ( color : vec3<f32> ) -> vec3<f32> {

	


	return mix( ( ( pow( color, vec3<f32>( 0.41666 ) ) * vec3<f32>( 1.055 ) ) - vec3<f32>( 0.055 ) ), ( color * vec3<f32>( 12.92 ) ), vec3<f32>( ( color <= vec3<f32>( 0.0031308 ) ) ) );

}


fn fn0 ( color : vec4<f32> ) -> vec4<f32> {

	


	return vec4<f32>( ( color.xyz * vec3<f32>( color.w ) ), color.w );

}




@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform0, nodeUniform0_sampler, nodeVarying0 );
	nodeVar1 = 0.0;
	nodeVar2 = 0.0;
	nodeVar3 = vec2<f32>( -2.0, -2.0 );
	nodeVar4 = ( fwidth( ( fragCoord.xy / render.nodeUniform2 ) ) / vec2<f32>( object.nodeUniform3 ) );
	nodeVar5 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar3 * nodeVar4 ) );
	nodeVar6 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar5 ).x;
	nodeVar7 = ( -0.5 / max( ( object.nodeUniform4 * object.nodeUniform4 ), 0.01 ) );
	nodeVar9 = textureDimensions( nodeUniform6 );
	nodeVar8 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar5 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar10 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( fragCoord.xy / render.nodeUniform2 ) ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar11 = max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( ( fragCoord.xy / render.nodeUniform2 ).x, ( 1.0 - ( fragCoord.xy / render.nodeUniform2 ).y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar10 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( ( fragCoord.xy / render.nodeUniform2 ).x, ( 1.0 - ( fragCoord.xy / render.nodeUniform2 ).y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar10 ), 1.0 ) ).w ) ).z ), 0.001 );
	nodeVar12 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar5.x, ( 1.0 - nodeVar5.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar8 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar5.x, ( 1.0 - nodeVar5.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar8 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar13 = ( ( object.nodeUniform7 * object.nodeUniform7 ) * -0.5 );
	nodeVar14 = ( exp( ( ( ( nodeVar3.x * nodeVar3.x ) + ( nodeVar3.y * nodeVar3.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar12 * nodeVar12 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar6 * nodeVar14 ) );
	nodeVar2 = ( nodeVar2 + nodeVar14 );
	nodeVar15 = vec2<f32>( -1.0, -2.0 );
	nodeVar16 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar15 * nodeVar4 ) );
	nodeVar17 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar16 ).x;
	nodeVar18 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar16 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar19 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar16.x, ( 1.0 - nodeVar16.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar18 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar16.x, ( 1.0 - nodeVar16.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar18 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar20 = ( exp( ( ( ( nodeVar15.x * nodeVar15.x ) + ( nodeVar15.y * nodeVar15.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar19 * nodeVar19 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar17 * nodeVar20 ) );
	nodeVar2 = ( nodeVar2 + nodeVar20 );
	nodeVar21 = vec2<f32>( 0.0, -2.0 );
	nodeVar22 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar21 * nodeVar4 ) );
	nodeVar23 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar22 ).x;
	nodeVar24 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar22 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar25 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar22.x, ( 1.0 - nodeVar22.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar24 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar22.x, ( 1.0 - nodeVar22.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar24 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar26 = ( exp( ( ( ( nodeVar21.x * nodeVar21.x ) + ( nodeVar21.y * nodeVar21.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar25 * nodeVar25 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar23 * nodeVar26 ) );
	nodeVar2 = ( nodeVar2 + nodeVar26 );
	nodeVar27 = vec2<f32>( 1.0, -2.0 );
	nodeVar28 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar27 * nodeVar4 ) );
	nodeVar29 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar28 ).x;
	nodeVar30 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar28 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar31 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar28.x, ( 1.0 - nodeVar28.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar30 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar28.x, ( 1.0 - nodeVar28.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar30 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar32 = ( exp( ( ( ( nodeVar27.x * nodeVar27.x ) + ( nodeVar27.y * nodeVar27.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar31 * nodeVar31 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar29 * nodeVar32 ) );
	nodeVar2 = ( nodeVar2 + nodeVar32 );
	nodeVar33 = vec2<f32>( 2.0, -2.0 );
	nodeVar34 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar33 * nodeVar4 ) );
	nodeVar35 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar34 ).x;
	nodeVar36 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar34 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar37 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar34.x, ( 1.0 - nodeVar34.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar36 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar34.x, ( 1.0 - nodeVar34.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar36 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar38 = ( exp( ( ( ( nodeVar33.x * nodeVar33.x ) + ( nodeVar33.y * nodeVar33.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar37 * nodeVar37 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar35 * nodeVar38 ) );
	nodeVar2 = ( nodeVar2 + nodeVar38 );
	nodeVar39 = vec2<f32>( -2.0, -1.0 );
	nodeVar40 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar39 * nodeVar4 ) );
	nodeVar41 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar40 ).x;
	nodeVar42 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar40 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar43 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar40.x, ( 1.0 - nodeVar40.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar42 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar40.x, ( 1.0 - nodeVar40.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar42 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar44 = ( exp( ( ( ( nodeVar39.x * nodeVar39.x ) + ( nodeVar39.y * nodeVar39.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar43 * nodeVar43 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar41 * nodeVar44 ) );
	nodeVar2 = ( nodeVar2 + nodeVar44 );
	nodeVar45 = vec2<f32>( -1.0, -1.0 );
	nodeVar46 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar45 * nodeVar4 ) );
	nodeVar47 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar46 ).x;
	nodeVar48 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar46 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar49 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar46.x, ( 1.0 - nodeVar46.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar48 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar46.x, ( 1.0 - nodeVar46.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar48 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar50 = ( exp( ( ( ( nodeVar45.x * nodeVar45.x ) + ( nodeVar45.y * nodeVar45.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar49 * nodeVar49 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar47 * nodeVar50 ) );
	nodeVar2 = ( nodeVar2 + nodeVar50 );
	nodeVar51 = vec2<f32>( 0.0, -1.0 );
	nodeVar52 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar51 * nodeVar4 ) );
	nodeVar53 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar52 ).x;
	nodeVar54 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar52 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar55 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar52.x, ( 1.0 - nodeVar52.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar54 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar52.x, ( 1.0 - nodeVar52.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar54 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar56 = ( exp( ( ( ( nodeVar51.x * nodeVar51.x ) + ( nodeVar51.y * nodeVar51.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar55 * nodeVar55 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar53 * nodeVar56 ) );
	nodeVar2 = ( nodeVar2 + nodeVar56 );
	nodeVar57 = vec2<f32>( 1.0, -1.0 );
	nodeVar58 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar57 * nodeVar4 ) );
	nodeVar59 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar58 ).x;
	nodeVar60 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar58 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar61 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar58.x, ( 1.0 - nodeVar58.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar60 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar58.x, ( 1.0 - nodeVar58.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar60 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar62 = ( exp( ( ( ( nodeVar57.x * nodeVar57.x ) + ( nodeVar57.y * nodeVar57.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar61 * nodeVar61 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar59 * nodeVar62 ) );
	nodeVar2 = ( nodeVar2 + nodeVar62 );
	nodeVar63 = vec2<f32>( 2.0, -1.0 );
	nodeVar64 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar63 * nodeVar4 ) );
	nodeVar65 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar64 ).x;
	nodeVar66 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar64 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar67 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar64.x, ( 1.0 - nodeVar64.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar66 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar64.x, ( 1.0 - nodeVar64.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar66 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar68 = ( exp( ( ( ( nodeVar63.x * nodeVar63.x ) + ( nodeVar63.y * nodeVar63.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar67 * nodeVar67 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar65 * nodeVar68 ) );
	nodeVar2 = ( nodeVar2 + nodeVar68 );
	nodeVar69 = vec2<f32>( -2.0, 0.0 );
	nodeVar70 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar69 * nodeVar4 ) );
	nodeVar71 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar70 ).x;
	nodeVar72 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar70 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar73 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar70.x, ( 1.0 - nodeVar70.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar72 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar70.x, ( 1.0 - nodeVar70.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar72 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar74 = ( exp( ( ( ( nodeVar69.x * nodeVar69.x ) + ( nodeVar69.y * nodeVar69.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar73 * nodeVar73 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar71 * nodeVar74 ) );
	nodeVar2 = ( nodeVar2 + nodeVar74 );
	nodeVar75 = vec2<f32>( -1.0, 0.0 );
	nodeVar76 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar75 * nodeVar4 ) );
	nodeVar77 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar76 ).x;
	nodeVar78 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar76 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar79 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar76.x, ( 1.0 - nodeVar76.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar78 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar76.x, ( 1.0 - nodeVar76.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar78 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar80 = ( exp( ( ( ( nodeVar75.x * nodeVar75.x ) + ( nodeVar75.y * nodeVar75.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar79 * nodeVar79 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar77 * nodeVar80 ) );
	nodeVar2 = ( nodeVar2 + nodeVar80 );
	nodeVar81 = vec2<f32>( 0.0, 0.0 );
	nodeVar82 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar81 * nodeVar4 ) );
	nodeVar83 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar82 ).x;
	nodeVar84 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar82 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar85 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar82.x, ( 1.0 - nodeVar82.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar84 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar82.x, ( 1.0 - nodeVar82.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar84 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar86 = ( exp( ( ( ( nodeVar81.x * nodeVar81.x ) + ( nodeVar81.y * nodeVar81.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar85 * nodeVar85 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar83 * nodeVar86 ) );
	nodeVar2 = ( nodeVar2 + nodeVar86 );
	nodeVar87 = vec2<f32>( 1.0, 0.0 );
	nodeVar88 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar87 * nodeVar4 ) );
	nodeVar89 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar88 ).x;
	nodeVar90 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar88 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar91 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar88.x, ( 1.0 - nodeVar88.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar90 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar88.x, ( 1.0 - nodeVar88.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar90 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar92 = ( exp( ( ( ( nodeVar87.x * nodeVar87.x ) + ( nodeVar87.y * nodeVar87.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar91 * nodeVar91 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar89 * nodeVar92 ) );
	nodeVar2 = ( nodeVar2 + nodeVar92 );
	nodeVar93 = vec2<f32>( 2.0, 0.0 );
	nodeVar94 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar93 * nodeVar4 ) );
	nodeVar95 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar94 ).x;
	nodeVar96 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar94 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar97 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar94.x, ( 1.0 - nodeVar94.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar96 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar94.x, ( 1.0 - nodeVar94.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar96 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar98 = ( exp( ( ( ( nodeVar93.x * nodeVar93.x ) + ( nodeVar93.y * nodeVar93.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar97 * nodeVar97 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar95 * nodeVar98 ) );
	nodeVar2 = ( nodeVar2 + nodeVar98 );
	nodeVar99 = vec2<f32>( -2.0, 1.0 );
	nodeVar100 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar99 * nodeVar4 ) );
	nodeVar101 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar100 ).x;
	nodeVar102 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar100 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar103 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar100.x, ( 1.0 - nodeVar100.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar102 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar100.x, ( 1.0 - nodeVar100.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar102 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar104 = ( exp( ( ( ( nodeVar99.x * nodeVar99.x ) + ( nodeVar99.y * nodeVar99.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar103 * nodeVar103 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar101 * nodeVar104 ) );
	nodeVar2 = ( nodeVar2 + nodeVar104 );
	nodeVar105 = vec2<f32>( -1.0, 1.0 );
	nodeVar106 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar105 * nodeVar4 ) );
	nodeVar107 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar106 ).x;
	nodeVar108 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar106 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar109 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar106.x, ( 1.0 - nodeVar106.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar108 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar106.x, ( 1.0 - nodeVar106.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar108 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar110 = ( exp( ( ( ( nodeVar105.x * nodeVar105.x ) + ( nodeVar105.y * nodeVar105.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar109 * nodeVar109 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar107 * nodeVar110 ) );
	nodeVar2 = ( nodeVar2 + nodeVar110 );
	nodeVar111 = vec2<f32>( 0.0, 1.0 );
	nodeVar112 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar111 * nodeVar4 ) );
	nodeVar113 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar112 ).x;
	nodeVar114 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar112 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar115 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar112.x, ( 1.0 - nodeVar112.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar114 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar112.x, ( 1.0 - nodeVar112.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar114 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar116 = ( exp( ( ( ( nodeVar111.x * nodeVar111.x ) + ( nodeVar111.y * nodeVar111.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar115 * nodeVar115 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar113 * nodeVar116 ) );
	nodeVar2 = ( nodeVar2 + nodeVar116 );
	nodeVar117 = vec2<f32>( 1.0, 1.0 );
	nodeVar118 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar117 * nodeVar4 ) );
	nodeVar119 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar118 ).x;
	nodeVar120 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar118 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar121 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar118.x, ( 1.0 - nodeVar118.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar120 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar118.x, ( 1.0 - nodeVar118.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar120 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar122 = ( exp( ( ( ( nodeVar117.x * nodeVar117.x ) + ( nodeVar117.y * nodeVar117.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar121 * nodeVar121 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar119 * nodeVar122 ) );
	nodeVar2 = ( nodeVar2 + nodeVar122 );
	nodeVar123 = vec2<f32>( 2.0, 1.0 );
	nodeVar124 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar123 * nodeVar4 ) );
	nodeVar125 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar124 ).x;
	nodeVar126 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar124 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar127 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar124.x, ( 1.0 - nodeVar124.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar126 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar124.x, ( 1.0 - nodeVar124.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar126 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar128 = ( exp( ( ( ( nodeVar123.x * nodeVar123.x ) + ( nodeVar123.y * nodeVar123.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar127 * nodeVar127 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar125 * nodeVar128 ) );
	nodeVar2 = ( nodeVar2 + nodeVar128 );
	nodeVar129 = vec2<f32>( -2.0, 2.0 );
	nodeVar130 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar129 * nodeVar4 ) );
	nodeVar131 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar130 ).x;
	nodeVar132 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar130 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar133 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar130.x, ( 1.0 - nodeVar130.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar132 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar130.x, ( 1.0 - nodeVar130.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar132 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar134 = ( exp( ( ( ( nodeVar129.x * nodeVar129.x ) + ( nodeVar129.y * nodeVar129.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar133 * nodeVar133 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar131 * nodeVar134 ) );
	nodeVar2 = ( nodeVar2 + nodeVar134 );
	nodeVar135 = vec2<f32>( -1.0, 2.0 );
	nodeVar136 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar135 * nodeVar4 ) );
	nodeVar137 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar136 ).x;
	nodeVar138 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar136 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar139 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar136.x, ( 1.0 - nodeVar136.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar138 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar136.x, ( 1.0 - nodeVar136.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar138 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar140 = ( exp( ( ( ( nodeVar135.x * nodeVar135.x ) + ( nodeVar135.y * nodeVar135.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar139 * nodeVar139 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar137 * nodeVar140 ) );
	nodeVar2 = ( nodeVar2 + nodeVar140 );
	nodeVar141 = vec2<f32>( 0.0, 2.0 );
	nodeVar142 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar141 * nodeVar4 ) );
	nodeVar143 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar142 ).x;
	nodeVar144 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar142 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar145 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar142.x, ( 1.0 - nodeVar142.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar144 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar142.x, ( 1.0 - nodeVar142.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar144 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar146 = ( exp( ( ( ( nodeVar141.x * nodeVar141.x ) + ( nodeVar141.y * nodeVar141.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar145 * nodeVar145 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar143 * nodeVar146 ) );
	nodeVar2 = ( nodeVar2 + nodeVar146 );
	nodeVar147 = vec2<f32>( 1.0, 2.0 );
	nodeVar148 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar147 * nodeVar4 ) );
	nodeVar149 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar148 ).x;
	nodeVar150 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar148 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar151 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar148.x, ( 1.0 - nodeVar148.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar150 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar148.x, ( 1.0 - nodeVar148.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar150 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar152 = ( exp( ( ( ( nodeVar147.x * nodeVar147.x ) + ( nodeVar147.y * nodeVar147.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar151 * nodeVar151 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar149 * nodeVar152 ) );
	nodeVar2 = ( nodeVar2 + nodeVar152 );
	nodeVar153 = vec2<f32>( 2.0, 2.0 );
	nodeVar154 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeVar153 * nodeVar4 ) );
	nodeVar155 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeVar154 ).x;
	nodeVar156 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeVar154 ) * vec2<f32>( nodeVar9 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar9 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	nodeVar157 = ( ( max( ( - ( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar154.x, ( 1.0 - nodeVar154.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar156 ), 1.0 ) ).xyz / vec3<f32>( ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeVar154.x, ( 1.0 - nodeVar154.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar156 ), 1.0 ) ).w ) ).z ), 0.001 ) - nodeVar11 ) / max( nodeVar11, 0.1 ) );
	nodeVar158 = ( exp( ( ( ( nodeVar153.x * nodeVar153.x ) + ( nodeVar153.y * nodeVar153.y ) ) * nodeVar7 ) ) * exp( ( ( nodeVar157 * nodeVar157 ) * nodeVar13 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar155 * nodeVar158 ) );
	nodeVar2 = ( nodeVar2 + nodeVar158 );
	nodeVar159 = mix( nodeVar0.xyz, vec3<f32>( 1.0, 1.0, 1.0 ), ( nodeVar1 / max( nodeVar2, 0.0001 ) ) );
	nodeVar160 = fn1( vec4<f32>( nodeVar159, clamp( vec4<f32>( nodeVar159, 1.0 ).w, 0.0, 1.0 ) ) );
	nodeVar161 = vec4<f32>( acesFilmicToneMapping( nodeVar160.xyz, render.nodeUniform8 ), nodeVar160.w );

	// result

	output.color = fn0( vec4<f32>( sRGBTransferOETF( nodeVar161.xyz ), nodeVar161.w ) );

	return output;

}
