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

struct objectStruct {
	nodeUniform1 : vec2<f32>,
	nodeUniform2 : vec2<f32>
};
@binding( 2 ) @group( 0 )
var<uniform> object : objectStruct;

// vars
var<private> nodeVar0 : vec4<f32>;
var<private> nodeVar1 : vec4<f32>;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : vec4<f32>;
var<private> nodeVar5 : vec2<f32>;
var<private> nodeVar6 : vec2<f32>;
var<private> nodeVar7 : vec4<f32>;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : vec4<f32>;
var<private> nodeVar11 : f32;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : f32;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec2<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : f32;
var<private> nodeVar19 : vec4<f32>;
var<private> nodeVar20 : f32;
var<private> nodeVar21 : f32;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : f32;
var<private> nodeVar24 : vec2<f32>;
var<private> nodeVar25 : vec4<f32>;
var<private> nodeVar26 : f32;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec4<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : f32;
var<private> nodeVar31 : f32;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec2<f32>;
var<private> nodeVar34 : vec4<f32>;
var<private> nodeVar35 : f32;
var<private> nodeVar36 : f32;
var<private> nodeVar37 : vec4<f32>;
var<private> nodeVar38 : f32;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : f32;
var<private> nodeVar41 : f32;
var<private> nodeVar42 : vec2<f32>;
var<private> nodeVar43 : vec4<f32>;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : f32;
var<private> nodeVar46 : vec4<f32>;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : f32;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : f32;
var<private> nodeVar51 : vec2<f32>;
var<private> nodeVar52 : vec4<f32>;
var<private> nodeVar53 : f32;
var<private> nodeVar54 : f32;
var<private> nodeVar55 : vec4<f32>;
var<private> nodeVar56 : f32;
var<private> nodeVar57 : f32;
var<private> nodeVar58 : f32;
var<private> nodeVar59 : f32;
var<private> nodeVar60 : vec2<f32>;
var<private> nodeVar61 : vec4<f32>;
var<private> nodeVar62 : f32;
var<private> nodeVar63 : f32;
var<private> nodeVar64 : vec4<f32>;
var<private> nodeVar65 : f32;
var<private> nodeVar66 : f32;
var<private> nodeVar67 : f32;
var<private> nodeVar68 : f32;
var<private> nodeVar69 : vec2<f32>;
var<private> nodeVar70 : vec4<f32>;
var<private> nodeVar71 : f32;
var<private> nodeVar72 : f32;
var<private> nodeVar73 : vec4<f32>;
var<private> nodeVar74 : f32;
var<private> nodeVar75 : f32;
var<private> nodeVar76 : f32;
var<private> nodeVar77 : f32;
var<private> nodeVar78 : vec2<f32>;
var<private> nodeVar79 : vec4<f32>;
var<private> nodeVar80 : f32;
var<private> nodeVar81 : f32;
var<private> nodeVar82 : vec4<f32>;
var<private> nodeVar83 : f32;
var<private> nodeVar84 : f32;
var<private> nodeVar85 : f32;
var<private> nodeVar86 : f32;
var<private> nodeVar87 : vec2<f32>;
var<private> nodeVar88 : vec4<f32>;
var<private> nodeVar89 : f32;
var<private> nodeVar90 : f32;
var<private> nodeVar91 : vec4<f32>;
var<private> nodeVar92 : f32;
var<private> nodeVar93 : f32;
var<private> nodeVar94 : f32;
var<private> nodeVar95 : f32;

// codes

@fragment
fn main( @location( 0 ) nodeVarying0 : vec2<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = textureSample( nodeUniform0, nodeUniform0_sampler, nodeVarying0 );
	nodeVar1 = nodeVar0;
	nodeVar2 = dot( nodeVar1.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) );
	nodeVar3 = 0.1088018181818182;
	nodeVar4 = ( nodeVar1 * vec4<f32>( 0.1088018181818182 ) );
	let nodeConst0 = ( -0.5 / 0.010000000000000002 );
	nodeVar5 = ( vec2<f32>( 1.0, 1.0 ) * object.nodeUniform1 );
	nodeVar6 = ( nodeVar5 * ( object.nodeUniform2 * vec2<f32>( 1.0 ) ) );
	nodeVar7 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar6 ) );
	nodeVar8 = abs( ( dot( nodeVar7.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar9 = exp( ( ( nodeVar8 * nodeVar8 ) * nodeConst0 ) );
	nodeVar10 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar6 ) );
	nodeVar11 = abs( ( dot( nodeVar10.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar12 = exp( ( ( nodeVar11 * nodeVar11 ) * nodeConst0 ) );
	nodeVar13 = ( 0.10482978744722705 * nodeVar9 );
	nodeVar4 = ( nodeVar4 + ( nodeVar7 * vec4<f32>( nodeVar13 ) ) );
	nodeVar14 = ( 0.10482978744722705 * nodeVar12 );
	nodeVar4 = ( nodeVar4 + ( nodeVar10 * vec4<f32>( nodeVar14 ) ) );
	nodeVar3 = ( nodeVar3 + nodeVar13 );
	nodeVar3 = ( nodeVar3 + nodeVar14 );
	nodeVar15 = ( nodeVar5 * ( object.nodeUniform2 * vec2<f32>( 2.0 ) ) );
	nodeVar16 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar15 ) );
	nodeVar17 = abs( ( dot( nodeVar16.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar18 = exp( ( ( nodeVar17 * nodeVar17 ) * nodeConst0 ) );
	nodeVar19 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar15 ) );
	nodeVar20 = abs( ( dot( nodeVar19.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar21 = exp( ( ( nodeVar20 * nodeVar20 ) * nodeConst0 ) );
	nodeVar22 = ( 0.09376275556297102 * nodeVar18 );
	nodeVar4 = ( nodeVar4 + ( nodeVar16 * vec4<f32>( nodeVar22 ) ) );
	nodeVar23 = ( 0.09376275556297102 * nodeVar21 );
	nodeVar4 = ( nodeVar4 + ( nodeVar19 * vec4<f32>( nodeVar23 ) ) );
	nodeVar3 = ( nodeVar3 + nodeVar22 );
	nodeVar3 = ( nodeVar3 + nodeVar23 );
	nodeVar24 = ( nodeVar5 * ( object.nodeUniform2 * vec2<f32>( 3.0 ) ) );
	nodeVar25 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar24 ) );
	nodeVar26 = abs( ( dot( nodeVar25.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar27 = exp( ( ( nodeVar26 * nodeVar26 ) * nodeConst0 ) );
	nodeVar28 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar24 ) );
	nodeVar29 = abs( ( dot( nodeVar28.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar30 = exp( ( ( nodeVar29 * nodeVar29 ) * nodeConst0 ) );
	nodeVar31 = ( 0.07785260050049682 * nodeVar27 );
	nodeVar4 = ( nodeVar4 + ( nodeVar25 * vec4<f32>( nodeVar31 ) ) );
	nodeVar32 = ( 0.07785260050049682 * nodeVar30 );
	nodeVar4 = ( nodeVar4 + ( nodeVar28 * vec4<f32>( nodeVar32 ) ) );
	nodeVar3 = ( nodeVar3 + nodeVar31 );
	nodeVar3 = ( nodeVar3 + nodeVar32 );
	nodeVar33 = ( nodeVar5 * ( object.nodeUniform2 * vec2<f32>( 4.0 ) ) );
	nodeVar34 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar33 ) );
	nodeVar35 = abs( ( dot( nodeVar34.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar36 = exp( ( ( nodeVar35 * nodeVar35 ) * nodeConst0 ) );
	nodeVar37 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar33 ) );
	nodeVar38 = abs( ( dot( nodeVar37.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar39 = exp( ( ( nodeVar38 * nodeVar38 ) * nodeConst0 ) );
	nodeVar40 = ( 0.060008530264881475 * nodeVar36 );
	nodeVar4 = ( nodeVar4 + ( nodeVar34 * vec4<f32>( nodeVar40 ) ) );
	nodeVar41 = ( 0.060008530264881475 * nodeVar39 );
	nodeVar4 = ( nodeVar4 + ( nodeVar37 * vec4<f32>( nodeVar41 ) ) );
	nodeVar3 = ( nodeVar3 + nodeVar40 );
	nodeVar3 = ( nodeVar3 + nodeVar41 );
	nodeVar42 = ( nodeVar5 * ( object.nodeUniform2 * vec2<f32>( 5.0 ) ) );
	nodeVar43 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar42 ) );
	nodeVar44 = abs( ( dot( nodeVar43.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar45 = exp( ( ( nodeVar44 * nodeVar44 ) * nodeConst0 ) );
	nodeVar46 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar42 ) );
	nodeVar47 = abs( ( dot( nodeVar46.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar48 = exp( ( ( nodeVar47 * nodeVar47 ) * nodeConst0 ) );
	nodeVar49 = ( 0.042938805724061835 * nodeVar45 );
	nodeVar4 = ( nodeVar4 + ( nodeVar43 * vec4<f32>( nodeVar49 ) ) );
	nodeVar50 = ( 0.042938805724061835 * nodeVar48 );
	nodeVar4 = ( nodeVar4 + ( nodeVar46 * vec4<f32>( nodeVar50 ) ) );
	nodeVar3 = ( nodeVar3 + nodeVar49 );
	nodeVar3 = ( nodeVar3 + nodeVar50 );
	nodeVar51 = ( nodeVar5 * ( object.nodeUniform2 * vec2<f32>( 6.0 ) ) );
	nodeVar52 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar51 ) );
	nodeVar53 = abs( ( dot( nodeVar52.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar54 = exp( ( ( nodeVar53 * nodeVar53 ) * nodeConst0 ) );
	nodeVar55 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar51 ) );
	nodeVar56 = abs( ( dot( nodeVar55.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar57 = exp( ( ( nodeVar56 * nodeVar56 ) * nodeConst0 ) );
	nodeVar58 = ( 0.02852226671021147 * nodeVar54 );
	nodeVar4 = ( nodeVar4 + ( nodeVar52 * vec4<f32>( nodeVar58 ) ) );
	nodeVar59 = ( 0.02852226671021147 * nodeVar57 );
	nodeVar4 = ( nodeVar4 + ( nodeVar55 * vec4<f32>( nodeVar59 ) ) );
	nodeVar3 = ( nodeVar3 + nodeVar58 );
	nodeVar3 = ( nodeVar3 + nodeVar59 );
	nodeVar60 = ( nodeVar5 * ( object.nodeUniform2 * vec2<f32>( 7.0 ) ) );
	nodeVar61 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar60 ) );
	nodeVar62 = abs( ( dot( nodeVar61.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar63 = exp( ( ( nodeVar62 * nodeVar62 ) * nodeConst0 ) );
	nodeVar64 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar60 ) );
	nodeVar65 = abs( ( dot( nodeVar64.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar66 = exp( ( ( nodeVar65 * nodeVar65 ) * nodeConst0 ) );
	nodeVar67 = ( 0.017587949779416395 * nodeVar63 );
	nodeVar4 = ( nodeVar4 + ( nodeVar61 * vec4<f32>( nodeVar67 ) ) );
	nodeVar68 = ( 0.017587949779416395 * nodeVar66 );
	nodeVar4 = ( nodeVar4 + ( nodeVar64 * vec4<f32>( nodeVar68 ) ) );
	nodeVar3 = ( nodeVar3 + nodeVar67 );
	nodeVar3 = ( nodeVar3 + nodeVar68 );
	nodeVar69 = ( nodeVar5 * ( object.nodeUniform2 * vec2<f32>( 8.0 ) ) );
	nodeVar70 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar69 ) );
	nodeVar71 = abs( ( dot( nodeVar70.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar72 = exp( ( ( nodeVar71 * nodeVar71 ) * nodeConst0 ) );
	nodeVar73 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar69 ) );
	nodeVar74 = abs( ( dot( nodeVar73.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar75 = exp( ( ( nodeVar74 * nodeVar74 ) * nodeConst0 ) );
	nodeVar76 = ( 0.010068006836002057 * nodeVar72 );
	nodeVar4 = ( nodeVar4 + ( nodeVar70 * vec4<f32>( nodeVar76 ) ) );
	nodeVar77 = ( 0.010068006836002057 * nodeVar75 );
	nodeVar4 = ( nodeVar4 + ( nodeVar73 * vec4<f32>( nodeVar77 ) ) );
	nodeVar3 = ( nodeVar3 + nodeVar76 );
	nodeVar3 = ( nodeVar3 + nodeVar77 );
	nodeVar78 = ( nodeVar5 * ( object.nodeUniform2 * vec2<f32>( 9.0 ) ) );
	nodeVar79 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar78 ) );
	nodeVar80 = abs( ( dot( nodeVar79.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar81 = exp( ( ( nodeVar80 * nodeVar80 ) * nodeConst0 ) );
	nodeVar82 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar78 ) );
	nodeVar83 = abs( ( dot( nodeVar82.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar84 = exp( ( ( nodeVar83 * nodeVar83 ) * nodeConst0 ) );
	nodeVar85 = ( 0.005350186131820889 * nodeVar81 );
	nodeVar4 = ( nodeVar4 + ( nodeVar79 * vec4<f32>( nodeVar85 ) ) );
	nodeVar86 = ( 0.005350186131820889 * nodeVar84 );
	nodeVar4 = ( nodeVar4 + ( nodeVar82 * vec4<f32>( nodeVar86 ) ) );
	nodeVar3 = ( nodeVar3 + nodeVar85 );
	nodeVar3 = ( nodeVar3 + nodeVar86 );
	nodeVar87 = ( nodeVar5 * ( object.nodeUniform2 * vec2<f32>( 10.0 ) ) );
	nodeVar88 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar87 ) );
	nodeVar89 = abs( ( dot( nodeVar88.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar90 = exp( ( ( nodeVar89 * nodeVar89 ) * nodeConst0 ) );
	nodeVar91 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar87 ) );
	nodeVar92 = abs( ( dot( nodeVar91.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar93 = exp( ( ( nodeVar92 * nodeVar92 ) * nodeConst0 ) );
	nodeVar94 = ( 0.002639315969304918 * nodeVar90 );
	nodeVar4 = ( nodeVar4 + ( nodeVar88 * vec4<f32>( nodeVar94 ) ) );
	nodeVar95 = ( 0.002639315969304918 * nodeVar93 );
	nodeVar4 = ( nodeVar4 + ( nodeVar91 * vec4<f32>( nodeVar95 ) ) );
	nodeVar3 = ( nodeVar3 + nodeVar94 );
	nodeVar3 = ( nodeVar3 + nodeVar95 );

	// result

	output.color = ( nodeVar4 / vec4<f32>( max( nodeVar3, 0.0001 ) ) );

	return output;

}
