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
@binding( 1 ) @group( 1 ) var nodeUniform17_sampler : sampler_comparison;
@binding( 2 ) @group( 1 ) var nodeUniform17 : texture_depth_2d;

struct objectStruct {
	nodeUniform0 : vec3<f32>,
	nodeUniform1 : f32,
	nodeUniform2 : vec3<f32>,
	nodeUniform3 : f32,
	nodeUniform7 : mat3x3<f32>,
	nodeUniform11 : mat4x4<f32>
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

struct renderStruct {
	nodeUniform23 : f32,
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform5 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform21 : f32,
	nodeUniform22 : f32,
	nodeUniform14 : vec3<f32>,
	nodeUniform10 : mat4x4<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform15 : f32,
	nodeUniform16 : f32,
	nodeUniform18 : f32,
	nodeUniform19 : vec2<f32>,
	nodeUniform20 : f32
};
@binding( 0 ) @group( 0 )
var<uniform> render : renderStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> EmissiveColor : vec3<f32>;
var<private> Output : vec4<f32>;
var<private> irradiance : vec3<f32>;
var<private> normalViewGeometry : vec3<f32>;
var<private> NORMAL_normalView : vec3<f32>;
var<private> normalView : vec3<f32>;
var<private> normalWorld : vec3<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar1 : vec3<f32>;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : f32;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : f32;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> indirectDiffuse : vec3<f32>;
var<private> ambientOcclusion : f32;
var<private> totalDiffuse : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> outgoingLight : vec3<f32>;

// codes
fn fn129 ( p : vec2<f32>, b : vec2<f32> ) -> f32 {

	

	let nodeConst0 = ( abs( p ) - b );

	return ( length( max( nodeConst0, vec2<f32>( 0.0 ) ) ) + min( max( nodeConst0.x, nodeConst0.y ), 0.0 ) );

}


fn fn24 ( position : vec2<f32> ) -> f32 {

	


	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}


fn fn25 ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	

	let nodeConst0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeConst0 ), sin( nodeConst0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

}


fn fn44 ( lightDistance : f32, cutoffDistance : f32, decayExponent : f32 ) -> f32 {

	var nodeVar0 : f32;


	if ( ( cutoffDistance > 0.0 ) ) {

		let nodeConst0 = ( 1.0 / max( pow( lightDistance, decayExponent ), 0.01 ) );
		let nodeConst1 = ( lightDistance / cutoffDistance );
		let nodeConst2 = clamp( ( 1.0 - ( ( ( nodeConst1 * nodeConst1 ) * nodeConst1 ) * nodeConst1 ) ), 0.0, 1.0 );
		nodeVar0 = ( nodeConst0 * ( nodeConst2 * nodeConst2 ) );

	} else {

		let nodeConst0 = ( 1.0 / max( pow( lightDistance, decayExponent ), 0.01 ) );
		nodeVar0 = nodeConst0;

	}


	return nodeVar0;

}


fn fn48 ( x : f32 ) -> i32 {

	var nodeVar0 : f32;

	nodeVar0 = x;

	return i32( floor( nodeVar0 ) );

}


fn fn61 ( x : u32, k : i32 ) -> u32 {

	var nodeVar0 : i32;
	var nodeVar1 : u32;

	nodeVar0 = k;
	nodeVar1 = x;

	return ( ( nodeVar1 << u32( nodeVar0 ) ) | ( nodeVar1 >> u32( ( 32 - nodeVar0 ) ) ) );

}


fn fn62 ( a : u32, b : u32, c : u32 ) -> vec3<u32> {

	var nodeVar0 : u32;
	var nodeVar1 : u32;
	var nodeVar2 : u32;

	nodeVar0 = a;
	nodeVar1 = b;
	nodeVar2 = c;
	nodeVar0 = ( nodeVar0 - nodeVar2 );
	nodeVar0 = ( nodeVar0 ^ fn61( nodeVar2, 4 ) );
	nodeVar2 = ( nodeVar2 + nodeVar1 );
	nodeVar1 = ( nodeVar1 - nodeVar0 );
	nodeVar1 = ( nodeVar1 ^ fn61( nodeVar0, 6 ) );
	nodeVar0 = ( nodeVar0 + nodeVar2 );
	nodeVar2 = ( nodeVar2 - nodeVar1 );
	nodeVar2 = ( nodeVar2 ^ fn61( nodeVar1, 8 ) );
	nodeVar1 = ( nodeVar1 + nodeVar0 );
	nodeVar0 = ( nodeVar0 - nodeVar2 );
	nodeVar0 = ( nodeVar0 ^ fn61( nodeVar2, 16 ) );
	nodeVar2 = ( nodeVar2 + nodeVar1 );
	nodeVar1 = ( nodeVar1 - nodeVar0 );
	nodeVar1 = ( nodeVar1 ^ fn61( nodeVar0, 19 ) );
	nodeVar0 = ( nodeVar0 + nodeVar2 );
	nodeVar2 = ( nodeVar2 - nodeVar1 );
	nodeVar2 = ( nodeVar2 ^ fn61( nodeVar1, 4 ) );
	nodeVar1 = ( nodeVar1 + nodeVar0 );

	return vec3<u32>( nodeVar0, nodeVar1, nodeVar2 );

}


fn fn63 ( a : u32, b : u32, c : u32 ) -> u32 {

	var nodeVar0 : u32;
	var nodeVar1 : u32;
	var nodeVar2 : u32;

	nodeVar0 = c;
	nodeVar1 = b;
	nodeVar2 = a;
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - fn61( nodeVar1, 14 ) );
	nodeVar2 = ( nodeVar2 ^ nodeVar0 );
	nodeVar2 = ( nodeVar2 - fn61( nodeVar0, 11 ) );
	nodeVar1 = ( nodeVar1 ^ nodeVar2 );
	nodeVar1 = ( nodeVar1 - fn61( nodeVar2, 25 ) );
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - fn61( nodeVar1, 16 ) );
	nodeVar2 = ( nodeVar2 ^ nodeVar0 );
	nodeVar2 = ( nodeVar2 - fn61( nodeVar0, 4 ) );
	nodeVar1 = ( nodeVar1 ^ nodeVar2 );
	nodeVar1 = ( nodeVar1 - fn61( nodeVar2, 14 ) );
	nodeVar0 = ( nodeVar0 ^ nodeVar1 );
	nodeVar0 = ( nodeVar0 - fn61( nodeVar1, 24 ) );

	return nodeVar0;

}


fn fn64 ( bits : u32 ) -> f32 {

	var nodeVar0 : u32;

	nodeVar0 = bits;

	return ( f32( nodeVar0 ) / 4294967295.0 );

}


fn fn85 ( position : vec3<f32> ) -> vec3<f32> {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : i32;
	var nodeVar2 : i32;
	var nodeVar3 : i32;
	var nodeVar4 : u32;
	var nodeVar5 : u32;
	var nodeVar6 : u32;
	var nodeVar7 : u32;
	var nodeVar8 : vec3<u32>;

	nodeVar0 = position;
	nodeVar1 = i32( floor( nodeVar0.x ) );
	nodeVar2 = i32( floor( nodeVar0.y ) );
	nodeVar3 = i32( floor( nodeVar0.z ) );
	nodeVar4 = 3735928588u;
	nodeVar5 = 0u;
	nodeVar6 = 0u;
	nodeVar7 = 0u;
	nodeVar7 = nodeVar4;
	nodeVar6 = nodeVar7;
	nodeVar5 = nodeVar6;
	nodeVar5 = ( nodeVar5 + u32( nodeVar1 ) );
	nodeVar6 = ( nodeVar6 + u32( nodeVar2 ) );
	nodeVar7 = ( nodeVar7 + u32( nodeVar3 ) );
	nodeVar8 = fn62( nodeVar5, nodeVar6, nodeVar7 );

	return vec3<f32>( fn64( fn63( nodeVar8.x, nodeVar8.y, nodeVar8.z ) ), fn64( fn63( ( nodeVar8.x + 1u ), nodeVar8.y, nodeVar8.z ) ), fn64( fn63( ( nodeVar8.x + 2u ), nodeVar8.y, nodeVar8.z ) ) );

}


fn fn92 ( p : vec3<f32>, x : i32, y : i32, z : i32, xoff : i32, yoff : i32, zoff : i32, jitter : f32, metric : i32 ) -> f32 {

	var nodeVar0 : i32;
	var nodeVar1 : f32;
	var nodeVar2 : i32;
	var nodeVar3 : i32;
	var nodeVar4 : i32;
	var nodeVar5 : i32;
	var nodeVar6 : i32;
	var nodeVar7 : i32;
	var nodeVar8 : vec3<f32>;
	var nodeVar9 : vec3<f32>;
	var nodeVar10 : vec3<f32>;
	var nodeVar11 : vec3<f32>;

	nodeVar0 = metric;
	nodeVar1 = jitter;
	nodeVar2 = zoff;
	nodeVar3 = yoff;
	nodeVar4 = xoff;
	nodeVar5 = z;
	nodeVar6 = y;
	nodeVar7 = x;
	nodeVar8 = p;
	nodeVar9 = fn85( vec3<f32>( f32( ( nodeVar7 + nodeVar4 ) ), f32( ( nodeVar6 + nodeVar3 ) ), f32( ( nodeVar5 + nodeVar2 ) ) ) );
	nodeVar9 = ( nodeVar9 - vec3<f32>( 0.5 ) );
	nodeVar9 = ( nodeVar9 * vec3<f32>( nodeVar1 ) );
	nodeVar9 = ( nodeVar9 + vec3<f32>( 0.5 ) );
	nodeVar10 = ( vec3<f32>( f32( nodeVar7 ), f32( nodeVar6 ), f32( nodeVar5 ) ) + nodeVar9 );
	nodeVar11 = ( nodeVar10 - nodeVar8 );

	if ( ( nodeVar0 == 2 ) ) {

		return ( ( abs( nodeVar11.x ) + abs( nodeVar11.y ) ) + abs( nodeVar11.z ) );

	}


	if ( ( nodeVar0 == 3 ) ) {

		return max( max( abs( nodeVar11.x ), abs( nodeVar11.y ) ), abs( nodeVar11.z ) );

	}


	return dot( nodeVar11, nodeVar11 );

}


fn fn68 ( x : i32, y : i32, z : i32 ) -> u32 {

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

	return fn63( nodeVar4, nodeVar5, nodeVar6 );

}


fn fn79 ( p : vec3<f32> ) -> f32 {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : i32;
	var nodeVar2 : i32;
	var nodeVar3 : i32;

	nodeVar0 = p;
	nodeVar1 = fn48( nodeVar0.x );
	nodeVar2 = fn48( nodeVar0.y );
	nodeVar3 = fn48( nodeVar0.z );

	return fn64( fn68( nodeVar1, nodeVar2, nodeVar3 ) );

}


fn fn93 ( position : vec3<f32>, jitter : f32, style : i32 ) -> f32 {

	var nodeVar0 : vec3<f32>;
	var nodeVar1 : f32;
	var nodeVar2 : i32;
	var nodeVar3 : i32;
	var nodeVar4 : i32;
	var nodeVar5 : i32;
	var nodeVar6 : f32;
	var nodeVar7 : f32;
	var nodeVar8 : f32;
	var nodeVar9 : vec3<f32>;
	var nodeVar10 : f32;
	var nodeVar11 : vec3<f32>;
	var nodeVar12 : f32;
	var nodeVar13 : vec3<f32>;
	var nodeVar14 : vec3<f32>;

	nodeVar0 = position;
	nodeVar1 = jitter;
	nodeVar2 = style;
	nodeVar3 = 0;
	nodeVar4 = 0;
	nodeVar5 = 0;
	nodeVar6 = nodeVar0.x;
	nodeVar3 = fn48( nodeVar6 );
	nodeVar7 = nodeVar0.y;
	nodeVar4 = fn48( nodeVar7 );
	nodeVar8 = nodeVar0.z;
	nodeVar5 = fn48( nodeVar8 );
	nodeVar9 = vec3<f32>( ( nodeVar6 - f32( nodeVar3 ) ), ( nodeVar7 - f32( nodeVar4 ) ), ( nodeVar8 - f32( nodeVar5 ) ) );
	nodeVar10 = 1000000.0;
	nodeVar11 = vec3<f32>( 0.0, 0.0, 0.0 );

	for ( var x : i32 = -1; x <= 1; x ++ ) {


		for ( var y : i32 = -1; y <= 1; y ++ ) {


			for ( var z : i32 = -1; z <= 1; z ++ ) {

				nodeVar12 = fn92( nodeVar9, x, y, z, nodeVar3, nodeVar4, nodeVar5, nodeVar1, 0 );
				nodeVar13 = fn85( vec3<f32>( f32( ( nodeVar3 + x ) ), f32( ( nodeVar4 + y ) ), f32( ( nodeVar5 + z ) ) ) );
				nodeVar13 = ( nodeVar13 - vec3<f32>( 0.5 ) );
				nodeVar13 = ( nodeVar13 * vec3<f32>( nodeVar1 ) );
				nodeVar13 = ( nodeVar13 + vec3<f32>( 0.5 ) );
				nodeVar14 = ( ( vec3<f32>( f32( x ), f32( y ), f32( z ) ) + nodeVar13 ) - nodeVar9 );

				if ( ( nodeVar12 < nodeVar10 ) ) {

					nodeVar10 = nodeVar12;
					nodeVar11 = nodeVar14;
					

				}


			}


		}


	}


	if ( ( nodeVar2 == 1 ) ) {

		nodeVar10 = fn79( ( nodeVar11 + nodeVar0 ) );
		

	} else {

		nodeVar10 = sqrt( nodeVar10 );
		

	}


	return nodeVar10;

}




@fragment
fn main( @location( 0 ) v_positionView : vec3<f32>,
	@location( 1 ) v_positionWorld : vec3<f32>,
	@location( 2 ) v_normalViewGeometry : vec3<f32>,
	@builtin( position ) fragCoord : vec4<f32> ) -> OutputStruct {

	// flow
	// code

	DiffuseColor = vec4<f32>( object.nodeUniform0, 1.0 );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform1 );
	DiffuseColor.w = 1.0;
	EmissiveColor = ( object.nodeUniform2 * vec3<f32>( object.nodeUniform3 ) );
	irradiance = vec3<f32>( 0.0, 0.0, 0.0 );
	normalViewGeometry = normalize( v_normalViewGeometry );
	NORMAL_normalView = normalViewGeometry;
	normalView = NORMAL_normalView;
	normalWorld = normalize( ( vec4<f32>( normalView, 0.0 ) * render.cameraViewMatrix ).xyz );
	irradiance = ( irradiance + mix( render.nodeUniform4, render.nodeUniform5, ( ( dot( normalWorld, normalize( render.nodeUniform9 ) ) * 0.5 ) + 0.5 ) ) );
	var nodeVar0 : f32 = 0.0;
	let nodeConst0 = ( render.nodeUniform10 * vec4<f32>( v_positionWorld, 1.0 ) );

	if ( ( nodeConst0.w > 0.0 ) ) {

		nodeVar0 = clamp( ( ( fn129( ( ( nodeConst0.xyz / vec3<f32>( nodeConst0.w ) ).xy - vec2<f32>( 0.5, 0.5 ) ), vec2<f32>( 0.5, 0.5 ) ) * -2.0 ) * ( -1.0 / ( ( 1.0 - acos( render.nodeUniform12 ) ) - 1.0 ) ) ), 0.0, 1.0 );
		

	}

	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	let nodeConst1 = ( render.nodeUniform13 - v_positionView );
	let nodeConst2 = ( render.nodeUniform10 * vec4<f32>( v_positionWorld, 1.0 ) );
	let nodeConst3 = ( nodeConst2.xyz / vec3<f32>( nodeConst2.w ) );

	if ( all( ( abs( ( ( nodeConst3 * vec3<f32>( 2.0 ) ) - vec3<f32>( 1.0 ) ) ) < vec3<f32>( 1.0 ) ) ) ) {

		shadowPositionWorld = v_positionWorld;
		let nodeConst4 = ( render.nodeUniform10 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform15 ) ) ), 1.0 ) );
		let nodeConst5 = ( nodeConst4.xyz / vec3<f32>( nodeConst4.w ) );
		let nodeConst6 = vec3<f32>( nodeConst5.x, ( 1.0 - nodeConst5.y ), ( nodeConst5.z + render.nodeUniform16 ) );

		if ( ( ( ( ( ( nodeConst6.x >= 0.0 ) && ( nodeConst6.x <= 1.0 ) ) && ( nodeConst6.y >= 0.0 ) ) && ( nodeConst6.y <= 1.0 ) ) && ( nodeConst6.z <= 1.0 ) ) ) {

			let nodeConst7 = ( fn24( fragCoord.xy ) * 6.28318530718 );
			let nodeConst8 = ( vec2<f32>( render.nodeUniform18 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform19 ) );
			let nodeConst9 = ( nodeConst6.xy + ( fn25( 0, 5, nodeConst7 ) * nodeConst8 ) );
			nodeVar3 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeConst9, nodeConst6.z );
			let nodeConst10 = ( nodeConst6.xy + ( fn25( 1, 5, nodeConst7 ) * nodeConst8 ) );
			nodeVar4 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeConst10, nodeConst6.z );
			let nodeConst11 = ( nodeConst6.xy + ( fn25( 2, 5, nodeConst7 ) * nodeConst8 ) );
			nodeVar5 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeConst11, nodeConst6.z );
			let nodeConst12 = ( nodeConst6.xy + ( fn25( 3, 5, nodeConst7 ) * nodeConst8 ) );
			nodeVar6 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeConst12, nodeConst6.z );
			let nodeConst13 = ( nodeConst6.xy + ( fn25( 4, 5, nodeConst7 ) * nodeConst8 ) );
			nodeVar7 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeConst13, nodeConst6.z );
			nodeVar2 = ( ( ( ( ( nodeVar3 + nodeVar4 ) + nodeVar5 ) + nodeVar6 ) + nodeVar7 ) * 0.2 );

		} else {

			nodeVar2 = 1.0;

		}

		nodeVar8 = mix( 1.0, nodeVar2, render.nodeUniform20 );
		let nodeConst14 = ( ( ( render.nodeUniform14 * vec3<f32>( nodeVar8 ) ) * vec3<f32>( nodeVar0 ) ) * vec3<f32>( fn44( length( nodeConst1 ), render.nodeUniform21, render.nodeUniform22 ) ) );
		nodeVar1 = ( nodeConst14 * ( ( vec3<f32>( pow( fn93( ( ( nodeConst3 * vec3<f32>( 10.0 ) ) + vec3<f32>( render.nodeUniform23 ) ), 1.0, 0 ), 2.0 ) ) * vec3<f32>( 0.10224173307914941, 0.5028864580233624, 0.6866853124288864 ) ) * vec3<f32>( 2.0 ) ) );

	} else {

		shadowPositionWorld = v_positionWorld;
		let nodeConst4 = ( render.nodeUniform10 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform15 ) ) ), 1.0 ) );
		let nodeConst5 = ( nodeConst4.xyz / vec3<f32>( nodeConst4.w ) );
		let nodeConst6 = vec3<f32>( nodeConst5.x, ( 1.0 - nodeConst5.y ), ( nodeConst5.z + render.nodeUniform16 ) );

		if ( ( ( ( ( ( nodeConst6.x >= 0.0 ) && ( nodeConst6.x <= 1.0 ) ) && ( nodeConst6.y >= 0.0 ) ) && ( nodeConst6.y <= 1.0 ) ) && ( nodeConst6.z <= 1.0 ) ) ) {

			let nodeConst7 = ( fn24( fragCoord.xy ) * 6.28318530718 );
			let nodeConst8 = ( vec2<f32>( render.nodeUniform18 ) * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform19 ) );
			let nodeConst9 = ( nodeConst6.xy + ( fn25( 0, 5, nodeConst7 ) * nodeConst8 ) );
			nodeVar3 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeConst9, nodeConst6.z );
			let nodeConst10 = ( nodeConst6.xy + ( fn25( 1, 5, nodeConst7 ) * nodeConst8 ) );
			nodeVar4 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeConst10, nodeConst6.z );
			let nodeConst11 = ( nodeConst6.xy + ( fn25( 2, 5, nodeConst7 ) * nodeConst8 ) );
			nodeVar5 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeConst11, nodeConst6.z );
			let nodeConst12 = ( nodeConst6.xy + ( fn25( 3, 5, nodeConst7 ) * nodeConst8 ) );
			nodeVar6 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeConst12, nodeConst6.z );
			let nodeConst13 = ( nodeConst6.xy + ( fn25( 4, 5, nodeConst7 ) * nodeConst8 ) );
			nodeVar7 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeConst13, nodeConst6.z );
			nodeVar9 = ( ( ( ( ( nodeVar3 + nodeVar4 ) + nodeVar5 ) + nodeVar6 ) + nodeVar7 ) * 0.2 );

		} else {

			nodeVar9 = 1.0;

		}

		nodeVar8 = mix( 1.0, nodeVar9, render.nodeUniform20 );
		let nodeConst14 = ( ( ( render.nodeUniform14 * vec3<f32>( nodeVar8 ) ) * vec3<f32>( nodeVar0 ) ) * vec3<f32>( fn44( length( nodeConst1 ), render.nodeUniform21, render.nodeUniform22 ) ) );
		nodeVar1 = nodeConst14;

	}

	directDiffuse = ( directDiffuse + ( ( vec3<f32>( clamp( dot( normalView, normalize( nodeConst1 ) ), 0.0, 1.0 ) ) * nodeVar1 ) * ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) ) ) );
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectDiffuse = ( vec4<f32>( indirectDiffuse, 1.0 ) + ( vec4<f32>( irradiance, 1.0 ) * ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) ) ) ).xyz;
	ambientOcclusion = 1.0;
	indirectDiffuse = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	totalDiffuse = ( directDiffuse + indirectDiffuse );
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	totalSpecular = ( directSpecular + indirectSpecular );
	outgoingLight = ( totalDiffuse + totalSpecular );
	let nodeConst15 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeConst15;

	// result

	output.color = nodeConst15;

	return output;

}
