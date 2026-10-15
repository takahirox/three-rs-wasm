// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );

// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms
@binding( 1 ) @group( 1 ) var nodeUniform17_sampler : sampler_comparison;
@binding( 2 ) @group( 1 ) var nodeUniform17 : texture_depth_2d;
@binding( 3 ) @group( 1 ) var nodeUniform23_sampler : sampler;
@binding( 4 ) @group( 1 ) var nodeUniform23 : texture_2d<f32>;

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
	cameraProjectionMatrix : mat4x4<f32>,
	cameraViewMatrix : mat4x4<f32>,
	nodeUniform5 : vec3<f32>,
	nodeUniform9 : vec3<f32>,
	nodeUniform4 : vec3<f32>,
	nodeUniform12 : f32,
	nodeUniform21 : f32,
	nodeUniform22 : f32,
	nodeUniform14 : vec3<f32>,
	nodeUniform13 : vec3<f32>,
	nodeUniform10 : mat4x4<f32>,
	nodeUniform15 : f32,
	nodeUniform16 : f32,
	nodeUniform20 : f32,
	nodeUniform18 : f32,
	nodeUniform19 : vec2<f32>
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
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : vec3<f32>;
var<private> nodeVar4 : vec3<f32>;
var<private> nodeVar5 : f32;
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : bool;
var<private> nodeVar8 : vec2<f32>;
var<private> directDiffuse : vec3<f32>;
var<private> nodeVar9 : vec3<f32>;
var<private> nodeVar10 : vec3<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : vec4<f32>;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : vec3<f32>;
var<private> nodeVar15 : vec3<f32>;
var<private> nodeVar16 : vec3<f32>;
var<private> nodeVar17 : vec3<f32>;
var<private> nodeVar18 : vec3<bool>;
var<private> nodeVar19 : bool;
var<private> shadowPositionWorld : vec3<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : vec4<f32>;
var<private> nodeVar22 : vec3<f32>;
var<private> nodeVar23 : vec3<f32>;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : f32;
var<private> nodeVar26 : vec2<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec2<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec2<f32>;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : vec2<f32>;
var<private> nodeVar33 : f32;
var<private> nodeVar34 : vec2<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : vec4<f32>;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : f32;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : vec4<f32>;
var<private> nodeVar47 : vec3<f32>;
var<private> nodeVar48 : vec4<f32>;
var<private> nodeVar49 : vec4<f32>;
var<private> indirectDiffuse : vec3<f32>;
var<private> nodeVar50 : vec4<f32>;
var<private> nodeVar51 : vec4<f32>;
var<private> nodeVar52 : vec4<f32>;
var<private> ambientOcclusion : f32;
var<private> nodeVar53 : vec3<f32>;
var<private> totalDiffuse : vec3<f32>;
var<private> nodeVar54 : vec3<f32>;
var<private> totalSpecular : vec3<f32>;
var<private> directSpecular : vec3<f32>;
var<private> indirectSpecular : vec3<f32>;
var<private> nodeVar55 : vec3<f32>;
var<private> outgoingLight : vec3<f32>;
var<private> nodeVar56 : vec3<f32>;
var<private> nodeVar57 : vec4<f32>;

// codes
fn interleavedGradientNoise ( position : vec2<f32> ) -> f32 {

	

	return fract( ( 52.9829189 * fract( dot( position, vec2<f32>( 0.06711056, 0.00583715 ) ) ) ) );

}

fn vogelDiskSample ( sampleIndex : i32, samplesCount : i32, phi : f32 ) -> vec2<f32> {

	var nodeVar0 : f32;

	nodeVar0 = ( ( f32( sampleIndex ) * 2.399963229728653 ) + phi );

	return ( vec2<f32>( cos( nodeVar0 ), sin( nodeVar0 ) ) * vec2<f32>( sqrt( ( ( f32( sampleIndex ) + 0.5 ) / f32( samplesCount ) ) ) ) );

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
	nodeVar0 = dot( normalWorld, normalize( render.nodeUniform9 ) );
	nodeVar1 = ( nodeVar0 * 0.5 );
	nodeVar2 = ( nodeVar1 + 0.5 );
	nodeVar3 = mix( render.nodeUniform4, render.nodeUniform5, nodeVar2 );
	nodeVar4 = ( irradiance + nodeVar3 );
	irradiance = nodeVar4;
	nodeVar5 = 0.0;
	nodeVar6 = ( render.nodeUniform10 * vec4<f32>( v_positionWorld, 1.0 ) );
	nodeVar7 = ( nodeVar6.w > 0.0 );

	if ( nodeVar7 ) {

		nodeVar8 = ( abs( ( ( nodeVar6.xyz / vec3<f32>( nodeVar6.w ) ).xy - vec2<f32>( 0.5, 0.5 ) ) ) - vec2<f32>( 0.5, 0.5 ) );
		nodeVar5 = clamp( ( ( ( length( max( nodeVar8, vec2<f32>( 0.0 ) ) ) + min( max( nodeVar8.x, nodeVar8.y ), 0.0 ) ) * -2.0 ) * ( -1.0 / ( ( 1.0 - acos( render.nodeUniform12 ) ) - 1.0 ) ) ), 0.0, 1.0 );
		

	}

	directDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar9 = ( render.nodeUniform13 - v_positionView );
	nodeVar10 = normalize( nodeVar9 );
	nodeVar11 = dot( normalView, nodeVar10 );
	nodeVar13 = ( render.nodeUniform10 * vec4<f32>( v_positionWorld, 1.0 ) );
	nodeVar14 = ( nodeVar13.xyz / vec3<f32>( nodeVar13.w ) );
	nodeVar15 = ( nodeVar14 * vec3<f32>( 2.0 ) );
	nodeVar16 = ( nodeVar15 - vec3<f32>( 1.0 ) );
	nodeVar17 = abs( nodeVar16 );
	nodeVar18 = ( nodeVar17 < vec3<f32>( 1.0 ) );
	nodeVar19 = all( nodeVar18 );

	if ( nodeVar19 ) {

		shadowPositionWorld = v_positionWorld;
		nodeVar21 = ( render.nodeUniform10 * vec4<f32>( ( shadowPositionWorld + ( normalWorld * vec3<f32>( render.nodeUniform15 ) ) ), 1.0 ) );
		nodeVar22 = ( nodeVar21.xyz / vec3<f32>( nodeVar21.w ) );
		nodeVar23 = vec3<f32>( nodeVar22.x, ( 1.0 - nodeVar22.y ), ( nodeVar22.z + render.nodeUniform16 ) );

		if ( ( ( ( ( ( nodeVar23.x >= 0.0 ) && ( nodeVar23.x <= 1.0 ) ) && ( nodeVar23.y >= 0.0 ) ) && ( nodeVar23.y <= 1.0 ) ) && ( nodeVar23.z <= 1.0 ) ) ) {

			nodeVar24 = ( interleavedGradientNoise( fragCoord.xy ) * 6.28318530718 );
			nodeVar25 = ( render.nodeUniform18 * ( vec2<f32>( 1.0, 1.0 ) / render.nodeUniform19 ).x );
			nodeVar26 = ( nodeVar23.xy + ( vogelDiskSample( 0, 5, nodeVar24 ) * vec2<f32>( nodeVar25 ) ) );
			nodeVar27 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar26, nodeVar23.z );
			nodeVar28 = ( nodeVar23.xy + ( vogelDiskSample( 1, 5, nodeVar24 ) * vec2<f32>( nodeVar25 ) ) );
			nodeVar29 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar28, nodeVar23.z );
			nodeVar30 = ( nodeVar23.xy + ( vogelDiskSample( 2, 5, nodeVar24 ) * vec2<f32>( nodeVar25 ) ) );
			nodeVar31 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar30, nodeVar23.z );
			nodeVar32 = ( nodeVar23.xy + ( vogelDiskSample( 3, 5, nodeVar24 ) * vec2<f32>( nodeVar25 ) ) );
			nodeVar33 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar32, nodeVar23.z );
			nodeVar34 = ( nodeVar23.xy + ( vogelDiskSample( 4, 5, nodeVar24 ) * vec2<f32>( nodeVar25 ) ) );
			nodeVar35 = textureSampleCompare( nodeUniform17, nodeUniform17_sampler, nodeVar34, nodeVar23.z );
			nodeVar20 = ( ( ( ( ( nodeVar27 + nodeVar29 ) + nodeVar31 ) + nodeVar33 ) + nodeVar35 ) * 0.2 );

		} else {

			nodeVar20 = 1.0;

		}

		nodeVar36 = mix( 1.0, nodeVar20, render.nodeUniform20 );

		if ( ( render.nodeUniform21 > 0.0 ) ) {

			nodeVar38 = length( nodeVar9 );
			nodeVar39 = ( nodeVar38 / render.nodeUniform21 );
			nodeVar40 = clamp( ( 1.0 - ( ( ( nodeVar39 * nodeVar39 ) * nodeVar39 ) * nodeVar39 ) ), 0.0, 1.0 );
			nodeVar37 = ( ( 1.0 / max( pow( nodeVar38, render.nodeUniform22 ), 0.01 ) ) * ( nodeVar40 * nodeVar40 ) );

		} else {

			nodeVar37 = ( 1.0 / max( pow( length( nodeVar9 ), render.nodeUniform22 ), 0.01 ) );

		}

		nodeVar41 = textureSample( nodeUniform23, nodeUniform23_sampler, nodeVar14.xy );
		nodeVar12 = ( vec4<f32>( ( ( ( render.nodeUniform14 * vec3<f32>( nodeVar36 ) ) * vec3<f32>( nodeVar5 ) ) * vec3<f32>( nodeVar37 ) ), 1.0 ) * nodeVar41 );

	} else {

		shadowPositionWorld = v_positionWorld;
		nodeVar36 = mix( 1.0, nodeVar20, render.nodeUniform20 );
		nodeVar36 = mix( 1.0, nodeVar20, render.nodeUniform20 );

		if ( ( render.nodeUniform21 > 0.0 ) ) {

			nodeVar43 = length( nodeVar9 );
			nodeVar44 = ( nodeVar43 / render.nodeUniform21 );
			nodeVar45 = clamp( ( 1.0 - ( ( ( nodeVar44 * nodeVar44 ) * nodeVar44 ) * nodeVar44 ) ), 0.0, 1.0 );
			nodeVar42 = ( ( 1.0 / max( pow( nodeVar43, render.nodeUniform22 ), 0.01 ) ) * ( nodeVar45 * nodeVar45 ) );

		} else {

			nodeVar42 = ( 1.0 / max( pow( length( nodeVar9 ), render.nodeUniform22 ), 0.01 ) );

		}

		nodeVar12 = vec4<f32>( ( ( ( render.nodeUniform14 * vec3<f32>( nodeVar36 ) ) * vec3<f32>( nodeVar5 ) ) * vec3<f32>( nodeVar42 ) ), 1.0 );

	}

	nodeVar46 = ( vec4<f32>( clamp( nodeVar11, 0.0, 1.0 ) ) * nodeVar12 );
	nodeVar47 = ( DiffuseColor.xyz * vec3<f32>( 0.3183098861837907 ) );
	nodeVar48 = ( nodeVar46 * vec4<f32>( nodeVar47, 1.0 ) );
	nodeVar49 = ( vec4<f32>( directDiffuse, 1.0 ) + nodeVar48 );
	directDiffuse = nodeVar49.xyz;
	indirectDiffuse = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar50 = ( DiffuseColor * vec4<f32>( 0.3183098861837907 ) );
	nodeVar51 = ( vec4<f32>( irradiance, 1.0 ) * nodeVar50 );
	nodeVar52 = ( vec4<f32>( indirectDiffuse, 1.0 ) + nodeVar51 );
	indirectDiffuse = nodeVar52.xyz;
	ambientOcclusion = 1.0;
	nodeVar53 = ( indirectDiffuse * vec3<f32>( ambientOcclusion ) );
	indirectDiffuse = nodeVar53;
	nodeVar54 = ( directDiffuse + indirectDiffuse );
	totalDiffuse = nodeVar54;
	directSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	indirectSpecular = vec3<f32>( 0.0, 0.0, 0.0 );
	nodeVar55 = ( directSpecular + indirectSpecular );
	totalSpecular = nodeVar55;
	nodeVar56 = ( totalDiffuse + totalSpecular );
	outgoingLight = nodeVar56;
	nodeVar57 = max( vec4<f32>( ( outgoingLight + EmissiveColor ), DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar57;

	// result

	output.color = nodeVar57;

	return output;

}
