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
@binding( 0 ) @group( 1 ) var nodeUniform0_sampler : sampler;
@binding( 1 ) @group( 1 ) var nodeUniform0 : texture_2d<f32>;
@binding( 2 ) @group( 1 ) var nodeUniform1_sampler : sampler;
@binding( 3 ) @group( 1 ) var nodeUniform1 : texture_2d<f32>;
@binding( 5 ) @group( 1 ) var nodeUniform6 : texture_depth_2d;

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
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : vec2<u32>;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : f32;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : f32;
var<private> nodeVar16 : f32;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : f32;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : f32;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : f32;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;

// codes
fn fn39 ( color : vec3<f32> ) -> vec3<f32> {

	


	return ( ( ( color * ( color + vec3<f32>( 0.0245786 ) ) ) - vec3<f32>( 0.000090537 ) ) / ( ( color * ( ( color * vec3<f32>( 0.983729 ) ) + vec3<f32>( 0.432951 ) ) ) + vec3<f32>( 0.238081 ) ) );

}


fn tsl_clampWrapping_float( coord: f32 ) -> f32 { return clamp( coord, 0.0, 1.0 ); }
fn tsl_coord_clampS_clampT_2d( coord : vec2f ) -> vec2f {

	return vec2f(
		tsl_clampWrapping_float( coord.x ),
		tsl_clampWrapping_float( coord.y )
	);

}

fn fn4 ( color : vec4<f32> ) -> vec4<f32> {

	var nodeVar0 : vec4<f32>;


	if ( ( color.w == 0.0 ) ) {

		nodeVar0 = vec4<f32>( 0.0, 0.0, 0.0, 0.0 );

	} else {

		nodeVar0 = vec4<f32>( ( color.xyz / vec3<f32>( color.w ) ), color.w );

	}


	return nodeVar0;

}


fn fn40 ( color : vec3<f32>, exposure : f32 ) -> vec3<f32> {

	


	return clamp( ( mat3x3<f32>( 1.60475, -0.10208, -0.00327, -0.53108, 1.10813, -0.07276, -0.07367, -0.00605, 1.07602 ) * fn39( ( mat3x3<f32>( 0.59719, 0.076, 0.0284, 0.35458, 0.90834, 0.13383, 0.04823, 0.01566, 0.83777 ) * ( ( color * vec3<f32>( exposure ) ) / vec3<f32>( 0.6 ) ) ) ) ), vec3<f32>( 0.0 ), vec3<f32>( 1.0 ) );

}


fn fn2 ( color : vec3<f32> ) -> vec3<f32> {

	


	return mix( ( ( pow( color, vec3<f32>( 0.41666 ) ) * vec3<f32>( 1.055 ) ) - vec3<f32>( 0.055 ) ), ( color * vec3<f32>( 12.92 ) ), vec3<f32>( ( color <= vec3<f32>( 0.0031308 ) ) ) );

}


fn fn3 ( color : vec4<f32> ) -> vec4<f32> {

	


	return vec4<f32>( ( color.xyz * vec3<f32>( color.w ) ), color.w );

}




@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform0, nodeUniform0_sampler, nodeVarying0 );
	let nodeConst0 = nodeVar0;
	nodeVar1 = 0.0;
	nodeVar2 = 0.0;
	let nodeConst1 = vec2<f32>( -2.0, -2.0 );
	let nodeConst2 = ( fwidth( ( fragCoord.xy / render.nodeUniform2 ) ) / vec2<f32>( object.nodeUniform3 ) );
	let nodeConst3 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst1 * nodeConst2 ) );
	nodeVar3 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst3 ).x;
	let nodeConst4 = ( -0.5 / max( ( object.nodeUniform4 * object.nodeUniform4 ), 0.01 ) );
	nodeVar5 = textureDimensions( nodeUniform6, u32( 0 ) );
	nodeVar4 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst3 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst5 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst3.x, ( 1.0 - nodeConst3.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar4 ), 1.0 ) );
	nodeVar6 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( ( fragCoord.xy / render.nodeUniform2 ) ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst6 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( ( fragCoord.xy / render.nodeUniform2 ).x, ( 1.0 - ( fragCoord.xy / render.nodeUniform2 ).y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar6 ), 1.0 ) );
	let nodeConst7 = max( ( - ( nodeConst6.xyz / vec3<f32>( nodeConst6.w ) ).z ), 0.001 );
	let nodeConst8 = ( ( max( ( - ( nodeConst5.xyz / vec3<f32>( nodeConst5.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst9 = ( ( object.nodeUniform7 * object.nodeUniform7 ) * -0.5 );
	let nodeConst10 = ( exp( ( ( ( nodeConst1.x * nodeConst1.x ) + ( nodeConst1.y * nodeConst1.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst8 * nodeConst8 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar3 * nodeConst10 ) );
	nodeVar2 = ( nodeVar2 + nodeConst10 );
	let nodeConst11 = vec2<f32>( -1.0, -2.0 );
	let nodeConst12 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst11 * nodeConst2 ) );
	nodeVar7 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst12 ).x;
	nodeVar8 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst12 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst13 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst12.x, ( 1.0 - nodeConst12.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar8 ), 1.0 ) );
	let nodeConst14 = ( ( max( ( - ( nodeConst13.xyz / vec3<f32>( nodeConst13.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst15 = ( exp( ( ( ( nodeConst11.x * nodeConst11.x ) + ( nodeConst11.y * nodeConst11.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst14 * nodeConst14 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar7 * nodeConst15 ) );
	nodeVar2 = ( nodeVar2 + nodeConst15 );
	let nodeConst16 = vec2<f32>( 0.0, -2.0 );
	let nodeConst17 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst16 * nodeConst2 ) );
	nodeVar9 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst17 ).x;
	nodeVar10 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst17 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst18 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst17.x, ( 1.0 - nodeConst17.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar10 ), 1.0 ) );
	let nodeConst19 = ( ( max( ( - ( nodeConst18.xyz / vec3<f32>( nodeConst18.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst20 = ( exp( ( ( ( nodeConst16.x * nodeConst16.x ) + ( nodeConst16.y * nodeConst16.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst19 * nodeConst19 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar9 * nodeConst20 ) );
	nodeVar2 = ( nodeVar2 + nodeConst20 );
	let nodeConst21 = vec2<f32>( 1.0, -2.0 );
	let nodeConst22 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst21 * nodeConst2 ) );
	nodeVar11 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst22 ).x;
	nodeVar12 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst22 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst23 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst22.x, ( 1.0 - nodeConst22.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar12 ), 1.0 ) );
	let nodeConst24 = ( ( max( ( - ( nodeConst23.xyz / vec3<f32>( nodeConst23.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst25 = ( exp( ( ( ( nodeConst21.x * nodeConst21.x ) + ( nodeConst21.y * nodeConst21.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst24 * nodeConst24 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar11 * nodeConst25 ) );
	nodeVar2 = ( nodeVar2 + nodeConst25 );
	let nodeConst26 = vec2<f32>( 2.0, -2.0 );
	let nodeConst27 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst26 * nodeConst2 ) );
	nodeVar13 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst27 ).x;
	nodeVar14 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst27 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst28 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst27.x, ( 1.0 - nodeConst27.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar14 ), 1.0 ) );
	let nodeConst29 = ( ( max( ( - ( nodeConst28.xyz / vec3<f32>( nodeConst28.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst30 = ( exp( ( ( ( nodeConst26.x * nodeConst26.x ) + ( nodeConst26.y * nodeConst26.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst29 * nodeConst29 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar13 * nodeConst30 ) );
	nodeVar2 = ( nodeVar2 + nodeConst30 );
	let nodeConst31 = vec2<f32>( -2.0, -1.0 );
	let nodeConst32 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst31 * nodeConst2 ) );
	nodeVar15 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst32 ).x;
	nodeVar16 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst32 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst33 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst32.x, ( 1.0 - nodeConst32.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar16 ), 1.0 ) );
	let nodeConst34 = ( ( max( ( - ( nodeConst33.xyz / vec3<f32>( nodeConst33.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst35 = ( exp( ( ( ( nodeConst31.x * nodeConst31.x ) + ( nodeConst31.y * nodeConst31.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst34 * nodeConst34 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar15 * nodeConst35 ) );
	nodeVar2 = ( nodeVar2 + nodeConst35 );
	let nodeConst36 = vec2<f32>( -1.0, -1.0 );
	let nodeConst37 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst36 * nodeConst2 ) );
	nodeVar17 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst37 ).x;
	nodeVar18 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst37 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst38 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst37.x, ( 1.0 - nodeConst37.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar18 ), 1.0 ) );
	let nodeConst39 = ( ( max( ( - ( nodeConst38.xyz / vec3<f32>( nodeConst38.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst40 = ( exp( ( ( ( nodeConst36.x * nodeConst36.x ) + ( nodeConst36.y * nodeConst36.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst39 * nodeConst39 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar17 * nodeConst40 ) );
	nodeVar2 = ( nodeVar2 + nodeConst40 );
	let nodeConst41 = vec2<f32>( 0.0, -1.0 );
	let nodeConst42 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst41 * nodeConst2 ) );
	nodeVar19 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst42 ).x;
	nodeVar20 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst42 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst43 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst42.x, ( 1.0 - nodeConst42.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar20 ), 1.0 ) );
	let nodeConst44 = ( ( max( ( - ( nodeConst43.xyz / vec3<f32>( nodeConst43.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst45 = ( exp( ( ( ( nodeConst41.x * nodeConst41.x ) + ( nodeConst41.y * nodeConst41.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst44 * nodeConst44 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar19 * nodeConst45 ) );
	nodeVar2 = ( nodeVar2 + nodeConst45 );
	let nodeConst46 = vec2<f32>( 1.0, -1.0 );
	let nodeConst47 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst46 * nodeConst2 ) );
	nodeVar21 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst47 ).x;
	nodeVar22 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst47 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst48 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst47.x, ( 1.0 - nodeConst47.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar22 ), 1.0 ) );
	let nodeConst49 = ( ( max( ( - ( nodeConst48.xyz / vec3<f32>( nodeConst48.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst50 = ( exp( ( ( ( nodeConst46.x * nodeConst46.x ) + ( nodeConst46.y * nodeConst46.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst49 * nodeConst49 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar21 * nodeConst50 ) );
	nodeVar2 = ( nodeVar2 + nodeConst50 );
	let nodeConst51 = vec2<f32>( 2.0, -1.0 );
	let nodeConst52 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst51 * nodeConst2 ) );
	nodeVar23 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst52 ).x;
	nodeVar24 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst52 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst53 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst52.x, ( 1.0 - nodeConst52.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar24 ), 1.0 ) );
	let nodeConst54 = ( ( max( ( - ( nodeConst53.xyz / vec3<f32>( nodeConst53.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst55 = ( exp( ( ( ( nodeConst51.x * nodeConst51.x ) + ( nodeConst51.y * nodeConst51.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst54 * nodeConst54 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar23 * nodeConst55 ) );
	nodeVar2 = ( nodeVar2 + nodeConst55 );
	let nodeConst56 = vec2<f32>( -2.0, 0.0 );
	let nodeConst57 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst56 * nodeConst2 ) );
	nodeVar25 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst57 ).x;
	nodeVar26 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst57 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst58 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst57.x, ( 1.0 - nodeConst57.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar26 ), 1.0 ) );
	let nodeConst59 = ( ( max( ( - ( nodeConst58.xyz / vec3<f32>( nodeConst58.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst60 = ( exp( ( ( ( nodeConst56.x * nodeConst56.x ) + ( nodeConst56.y * nodeConst56.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst59 * nodeConst59 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar25 * nodeConst60 ) );
	nodeVar2 = ( nodeVar2 + nodeConst60 );
	let nodeConst61 = vec2<f32>( -1.0, 0.0 );
	let nodeConst62 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst61 * nodeConst2 ) );
	nodeVar27 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst62 ).x;
	nodeVar28 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst62 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst63 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst62.x, ( 1.0 - nodeConst62.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar28 ), 1.0 ) );
	let nodeConst64 = ( ( max( ( - ( nodeConst63.xyz / vec3<f32>( nodeConst63.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst65 = ( exp( ( ( ( nodeConst61.x * nodeConst61.x ) + ( nodeConst61.y * nodeConst61.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst64 * nodeConst64 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar27 * nodeConst65 ) );
	nodeVar2 = ( nodeVar2 + nodeConst65 );
	let nodeConst66 = vec2<f32>( 0.0, 0.0 );
	let nodeConst67 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst66 * nodeConst2 ) );
	nodeVar29 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst67 ).x;
	nodeVar30 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst67 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst68 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst67.x, ( 1.0 - nodeConst67.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar30 ), 1.0 ) );
	let nodeConst69 = ( ( max( ( - ( nodeConst68.xyz / vec3<f32>( nodeConst68.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst70 = ( exp( ( ( ( nodeConst66.x * nodeConst66.x ) + ( nodeConst66.y * nodeConst66.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst69 * nodeConst69 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar29 * nodeConst70 ) );
	nodeVar2 = ( nodeVar2 + nodeConst70 );
	let nodeConst71 = vec2<f32>( 1.0, 0.0 );
	let nodeConst72 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst71 * nodeConst2 ) );
	nodeVar31 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst72 ).x;
	nodeVar32 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst72 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst73 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst72.x, ( 1.0 - nodeConst72.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar32 ), 1.0 ) );
	let nodeConst74 = ( ( max( ( - ( nodeConst73.xyz / vec3<f32>( nodeConst73.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst75 = ( exp( ( ( ( nodeConst71.x * nodeConst71.x ) + ( nodeConst71.y * nodeConst71.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst74 * nodeConst74 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar31 * nodeConst75 ) );
	nodeVar2 = ( nodeVar2 + nodeConst75 );
	let nodeConst76 = vec2<f32>( 2.0, 0.0 );
	let nodeConst77 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst76 * nodeConst2 ) );
	nodeVar33 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst77 ).x;
	nodeVar34 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst77 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst78 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst77.x, ( 1.0 - nodeConst77.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar34 ), 1.0 ) );
	let nodeConst79 = ( ( max( ( - ( nodeConst78.xyz / vec3<f32>( nodeConst78.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst80 = ( exp( ( ( ( nodeConst76.x * nodeConst76.x ) + ( nodeConst76.y * nodeConst76.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst79 * nodeConst79 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar33 * nodeConst80 ) );
	nodeVar2 = ( nodeVar2 + nodeConst80 );
	let nodeConst81 = vec2<f32>( -2.0, 1.0 );
	let nodeConst82 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst81 * nodeConst2 ) );
	nodeVar35 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst82 ).x;
	nodeVar36 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst82 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst83 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst82.x, ( 1.0 - nodeConst82.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar36 ), 1.0 ) );
	let nodeConst84 = ( ( max( ( - ( nodeConst83.xyz / vec3<f32>( nodeConst83.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst85 = ( exp( ( ( ( nodeConst81.x * nodeConst81.x ) + ( nodeConst81.y * nodeConst81.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst84 * nodeConst84 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar35 * nodeConst85 ) );
	nodeVar2 = ( nodeVar2 + nodeConst85 );
	let nodeConst86 = vec2<f32>( -1.0, 1.0 );
	let nodeConst87 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst86 * nodeConst2 ) );
	nodeVar37 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst87 ).x;
	nodeVar38 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst87 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst88 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst87.x, ( 1.0 - nodeConst87.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar38 ), 1.0 ) );
	let nodeConst89 = ( ( max( ( - ( nodeConst88.xyz / vec3<f32>( nodeConst88.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst90 = ( exp( ( ( ( nodeConst86.x * nodeConst86.x ) + ( nodeConst86.y * nodeConst86.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst89 * nodeConst89 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar37 * nodeConst90 ) );
	nodeVar2 = ( nodeVar2 + nodeConst90 );
	let nodeConst91 = vec2<f32>( 0.0, 1.0 );
	let nodeConst92 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst91 * nodeConst2 ) );
	nodeVar39 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst92 ).x;
	nodeVar40 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst92 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst93 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst92.x, ( 1.0 - nodeConst92.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar40 ), 1.0 ) );
	let nodeConst94 = ( ( max( ( - ( nodeConst93.xyz / vec3<f32>( nodeConst93.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst95 = ( exp( ( ( ( nodeConst91.x * nodeConst91.x ) + ( nodeConst91.y * nodeConst91.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst94 * nodeConst94 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar39 * nodeConst95 ) );
	nodeVar2 = ( nodeVar2 + nodeConst95 );
	let nodeConst96 = vec2<f32>( 1.0, 1.0 );
	let nodeConst97 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst96 * nodeConst2 ) );
	nodeVar41 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst97 ).x;
	nodeVar42 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst97 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst98 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst97.x, ( 1.0 - nodeConst97.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar42 ), 1.0 ) );
	let nodeConst99 = ( ( max( ( - ( nodeConst98.xyz / vec3<f32>( nodeConst98.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst100 = ( exp( ( ( ( nodeConst96.x * nodeConst96.x ) + ( nodeConst96.y * nodeConst96.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst99 * nodeConst99 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar41 * nodeConst100 ) );
	nodeVar2 = ( nodeVar2 + nodeConst100 );
	let nodeConst101 = vec2<f32>( 2.0, 1.0 );
	let nodeConst102 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst101 * nodeConst2 ) );
	nodeVar43 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst102 ).x;
	nodeVar44 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst102 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst103 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst102.x, ( 1.0 - nodeConst102.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar44 ), 1.0 ) );
	let nodeConst104 = ( ( max( ( - ( nodeConst103.xyz / vec3<f32>( nodeConst103.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst105 = ( exp( ( ( ( nodeConst101.x * nodeConst101.x ) + ( nodeConst101.y * nodeConst101.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst104 * nodeConst104 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar43 * nodeConst105 ) );
	nodeVar2 = ( nodeVar2 + nodeConst105 );
	let nodeConst106 = vec2<f32>( -2.0, 2.0 );
	let nodeConst107 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst106 * nodeConst2 ) );
	nodeVar45 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst107 ).x;
	nodeVar46 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst107 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst108 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst107.x, ( 1.0 - nodeConst107.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar46 ), 1.0 ) );
	let nodeConst109 = ( ( max( ( - ( nodeConst108.xyz / vec3<f32>( nodeConst108.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst110 = ( exp( ( ( ( nodeConst106.x * nodeConst106.x ) + ( nodeConst106.y * nodeConst106.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst109 * nodeConst109 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar45 * nodeConst110 ) );
	nodeVar2 = ( nodeVar2 + nodeConst110 );
	let nodeConst111 = vec2<f32>( -1.0, 2.0 );
	let nodeConst112 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst111 * nodeConst2 ) );
	nodeVar47 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst112 ).x;
	nodeVar48 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst112 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst113 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst112.x, ( 1.0 - nodeConst112.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar48 ), 1.0 ) );
	let nodeConst114 = ( ( max( ( - ( nodeConst113.xyz / vec3<f32>( nodeConst113.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst115 = ( exp( ( ( ( nodeConst111.x * nodeConst111.x ) + ( nodeConst111.y * nodeConst111.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst114 * nodeConst114 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar47 * nodeConst115 ) );
	nodeVar2 = ( nodeVar2 + nodeConst115 );
	let nodeConst116 = vec2<f32>( 0.0, 2.0 );
	let nodeConst117 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst116 * nodeConst2 ) );
	nodeVar49 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst117 ).x;
	nodeVar50 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst117 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst118 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst117.x, ( 1.0 - nodeConst117.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar50 ), 1.0 ) );
	let nodeConst119 = ( ( max( ( - ( nodeConst118.xyz / vec3<f32>( nodeConst118.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst120 = ( exp( ( ( ( nodeConst116.x * nodeConst116.x ) + ( nodeConst116.y * nodeConst116.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst119 * nodeConst119 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar49 * nodeConst120 ) );
	nodeVar2 = ( nodeVar2 + nodeConst120 );
	let nodeConst121 = vec2<f32>( 1.0, 2.0 );
	let nodeConst122 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst121 * nodeConst2 ) );
	nodeVar51 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst122 ).x;
	nodeVar52 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst122 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst123 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst122.x, ( 1.0 - nodeConst122.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar52 ), 1.0 ) );
	let nodeConst124 = ( ( max( ( - ( nodeConst123.xyz / vec3<f32>( nodeConst123.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst125 = ( exp( ( ( ( nodeConst121.x * nodeConst121.x ) + ( nodeConst121.y * nodeConst121.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst124 * nodeConst124 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar51 * nodeConst125 ) );
	nodeVar2 = ( nodeVar2 + nodeConst125 );
	let nodeConst126 = vec2<f32>( 2.0, 2.0 );
	let nodeConst127 = ( ( fragCoord.xy / render.nodeUniform2 ) + ( nodeConst126 * nodeConst2 ) );
	nodeVar53 = textureSample( nodeUniform1, nodeUniform1_sampler, nodeConst127 ).x;
	nodeVar54 = textureLoad( nodeUniform6, vec2<u32>( clamp( floor( tsl_coord_clampS_clampT_2d( nodeConst127 ) * vec2<f32>( nodeVar5 ) ), vec2<f32>( 0 ), vec2<f32>( nodeVar5 - vec2<u32>( 1, 1 ) ) ) ), u32( 0 ) );
	let nodeConst128 = ( object.nodeUniform5 * vec4<f32>( vec3<f32>( ( ( vec2<f32>( nodeConst127.x, ( 1.0 - nodeConst127.y ) ) * vec2<f32>( 2.0 ) ) - vec2<f32>( 1.0 ) ), nodeVar54 ), 1.0 ) );
	let nodeConst129 = ( ( max( ( - ( nodeConst128.xyz / vec3<f32>( nodeConst128.w ) ).z ), 0.001 ) - nodeConst7 ) / max( nodeConst7, 0.1 ) );
	let nodeConst130 = ( exp( ( ( ( nodeConst126.x * nodeConst126.x ) + ( nodeConst126.y * nodeConst126.y ) ) * nodeConst4 ) ) * exp( ( ( nodeConst129 * nodeConst129 ) * nodeConst9 ) ) );
	nodeVar1 = ( nodeVar1 + ( nodeVar53 * nodeConst130 ) );
	nodeVar2 = ( nodeVar2 + nodeConst130 );
	let nodeConst131 = mix( nodeConst0.xyz, vec3<f32>( 1.0, 1.0, 1.0 ), ( nodeVar1 / max( nodeVar2, 0.0001 ) ) );
	let nodeConst132 = fn4( vec4<f32>( nodeConst131, clamp( vec4<f32>( nodeConst131, 1.0 ).w, 0.0, 1.0 ) ) );
	let nodeConst133 = vec4<f32>( fn40( nodeConst132.xyz, render.nodeUniform8 ), nodeConst132.w );

	// result

	output.color = fn3( vec4<f32>( fn2( nodeConst133.xyz ), nodeConst133.w ) );

	return output;

}
