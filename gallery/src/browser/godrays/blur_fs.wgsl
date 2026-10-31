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
var<private> nodeVar6 : vec4<f32>;
var<private> nodeVar7 : f32;
var<private> nodeVar8 : vec4<f32>;
var<private> nodeVar9 : f32;
var<private> nodeVar10 : vec2<f32>;
var<private> nodeVar11 : vec4<f32>;
var<private> nodeVar12 : f32;
var<private> nodeVar13 : vec4<f32>;
var<private> nodeVar14 : f32;
var<private> nodeVar15 : vec2<f32>;
var<private> nodeVar16 : vec4<f32>;
var<private> nodeVar17 : f32;
var<private> nodeVar18 : vec4<f32>;
var<private> nodeVar19 : f32;
var<private> nodeVar20 : vec2<f32>;
var<private> nodeVar21 : vec4<f32>;
var<private> nodeVar22 : f32;
var<private> nodeVar23 : vec4<f32>;
var<private> nodeVar24 : f32;
var<private> nodeVar25 : vec2<f32>;
var<private> nodeVar26 : vec4<f32>;
var<private> nodeVar27 : f32;
var<private> nodeVar28 : vec4<f32>;
var<private> nodeVar29 : f32;
var<private> nodeVar30 : vec2<f32>;
var<private> nodeVar31 : vec4<f32>;
var<private> nodeVar32 : f32;
var<private> nodeVar33 : vec4<f32>;
var<private> nodeVar34 : f32;
var<private> nodeVar35 : vec2<f32>;
var<private> nodeVar36 : vec4<f32>;
var<private> nodeVar37 : f32;
var<private> nodeVar38 : vec4<f32>;
var<private> nodeVar39 : f32;
var<private> nodeVar40 : vec2<f32>;
var<private> nodeVar41 : vec4<f32>;
var<private> nodeVar42 : f32;
var<private> nodeVar43 : vec4<f32>;
var<private> nodeVar44 : f32;
var<private> nodeVar45 : vec2<f32>;
var<private> nodeVar46 : vec4<f32>;
var<private> nodeVar47 : f32;
var<private> nodeVar48 : vec4<f32>;
var<private> nodeVar49 : f32;
var<private> nodeVar50 : vec2<f32>;
var<private> nodeVar51 : vec4<f32>;
var<private> nodeVar52 : f32;
var<private> nodeVar53 : vec4<f32>;
var<private> nodeVar54 : f32;

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
	const nodeConst0 = ( -0.5 / 0.010000000000000002 );
	let nodeConst1 = ( vec2<f32>( 1.0, 1.0 ) * object.nodeUniform1 );
	nodeVar5 = ( nodeConst1 * ( object.nodeUniform2 * vec2<f32>( 1.0 ) ) );
	nodeVar6 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar5 ) );
	let nodeConst2 = nodeVar6;
	let nodeConst3 = abs( ( dot( nodeConst2.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar7 = exp( ( ( nodeConst3 * nodeConst3 ) * nodeConst0 ) );
	nodeVar8 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar5 ) );
	let nodeConst4 = nodeVar8;
	let nodeConst5 = abs( ( dot( nodeConst4.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar9 = exp( ( ( nodeConst5 * nodeConst5 ) * nodeConst0 ) );
	let nodeConst6 = ( 0.10482978744722705 * nodeVar7 );
	nodeVar4 = ( nodeVar4 + ( nodeConst2 * vec4<f32>( nodeConst6 ) ) );
	let nodeConst7 = ( 0.10482978744722705 * nodeVar9 );
	nodeVar4 = ( nodeVar4 + ( nodeConst4 * vec4<f32>( nodeConst7 ) ) );
	nodeVar3 = ( nodeVar3 + nodeConst6 );
	nodeVar3 = ( nodeVar3 + nodeConst7 );
	nodeVar10 = ( nodeConst1 * ( object.nodeUniform2 * vec2<f32>( 2.0 ) ) );
	nodeVar11 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar10 ) );
	let nodeConst8 = nodeVar11;
	let nodeConst9 = abs( ( dot( nodeConst8.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar12 = exp( ( ( nodeConst9 * nodeConst9 ) * nodeConst0 ) );
	nodeVar13 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar10 ) );
	let nodeConst10 = nodeVar13;
	let nodeConst11 = abs( ( dot( nodeConst10.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar14 = exp( ( ( nodeConst11 * nodeConst11 ) * nodeConst0 ) );
	let nodeConst12 = ( 0.09376275556297102 * nodeVar12 );
	nodeVar4 = ( nodeVar4 + ( nodeConst8 * vec4<f32>( nodeConst12 ) ) );
	let nodeConst13 = ( 0.09376275556297102 * nodeVar14 );
	nodeVar4 = ( nodeVar4 + ( nodeConst10 * vec4<f32>( nodeConst13 ) ) );
	nodeVar3 = ( nodeVar3 + nodeConst12 );
	nodeVar3 = ( nodeVar3 + nodeConst13 );
	nodeVar15 = ( nodeConst1 * ( object.nodeUniform2 * vec2<f32>( 3.0 ) ) );
	nodeVar16 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar15 ) );
	let nodeConst14 = nodeVar16;
	let nodeConst15 = abs( ( dot( nodeConst14.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar17 = exp( ( ( nodeConst15 * nodeConst15 ) * nodeConst0 ) );
	nodeVar18 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar15 ) );
	let nodeConst16 = nodeVar18;
	let nodeConst17 = abs( ( dot( nodeConst16.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar19 = exp( ( ( nodeConst17 * nodeConst17 ) * nodeConst0 ) );
	let nodeConst18 = ( 0.07785260050049682 * nodeVar17 );
	nodeVar4 = ( nodeVar4 + ( nodeConst14 * vec4<f32>( nodeConst18 ) ) );
	let nodeConst19 = ( 0.07785260050049682 * nodeVar19 );
	nodeVar4 = ( nodeVar4 + ( nodeConst16 * vec4<f32>( nodeConst19 ) ) );
	nodeVar3 = ( nodeVar3 + nodeConst18 );
	nodeVar3 = ( nodeVar3 + nodeConst19 );
	nodeVar20 = ( nodeConst1 * ( object.nodeUniform2 * vec2<f32>( 4.0 ) ) );
	nodeVar21 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar20 ) );
	let nodeConst20 = nodeVar21;
	let nodeConst21 = abs( ( dot( nodeConst20.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar22 = exp( ( ( nodeConst21 * nodeConst21 ) * nodeConst0 ) );
	nodeVar23 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar20 ) );
	let nodeConst22 = nodeVar23;
	let nodeConst23 = abs( ( dot( nodeConst22.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar24 = exp( ( ( nodeConst23 * nodeConst23 ) * nodeConst0 ) );
	let nodeConst24 = ( 0.060008530264881475 * nodeVar22 );
	nodeVar4 = ( nodeVar4 + ( nodeConst20 * vec4<f32>( nodeConst24 ) ) );
	let nodeConst25 = ( 0.060008530264881475 * nodeVar24 );
	nodeVar4 = ( nodeVar4 + ( nodeConst22 * vec4<f32>( nodeConst25 ) ) );
	nodeVar3 = ( nodeVar3 + nodeConst24 );
	nodeVar3 = ( nodeVar3 + nodeConst25 );
	nodeVar25 = ( nodeConst1 * ( object.nodeUniform2 * vec2<f32>( 5.0 ) ) );
	nodeVar26 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar25 ) );
	let nodeConst26 = nodeVar26;
	let nodeConst27 = abs( ( dot( nodeConst26.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar27 = exp( ( ( nodeConst27 * nodeConst27 ) * nodeConst0 ) );
	nodeVar28 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar25 ) );
	let nodeConst28 = nodeVar28;
	let nodeConst29 = abs( ( dot( nodeConst28.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar29 = exp( ( ( nodeConst29 * nodeConst29 ) * nodeConst0 ) );
	let nodeConst30 = ( 0.042938805724061835 * nodeVar27 );
	nodeVar4 = ( nodeVar4 + ( nodeConst26 * vec4<f32>( nodeConst30 ) ) );
	let nodeConst31 = ( 0.042938805724061835 * nodeVar29 );
	nodeVar4 = ( nodeVar4 + ( nodeConst28 * vec4<f32>( nodeConst31 ) ) );
	nodeVar3 = ( nodeVar3 + nodeConst30 );
	nodeVar3 = ( nodeVar3 + nodeConst31 );
	nodeVar30 = ( nodeConst1 * ( object.nodeUniform2 * vec2<f32>( 6.0 ) ) );
	nodeVar31 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar30 ) );
	let nodeConst32 = nodeVar31;
	let nodeConst33 = abs( ( dot( nodeConst32.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar32 = exp( ( ( nodeConst33 * nodeConst33 ) * nodeConst0 ) );
	nodeVar33 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar30 ) );
	let nodeConst34 = nodeVar33;
	let nodeConst35 = abs( ( dot( nodeConst34.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar34 = exp( ( ( nodeConst35 * nodeConst35 ) * nodeConst0 ) );
	let nodeConst36 = ( 0.02852226671021147 * nodeVar32 );
	nodeVar4 = ( nodeVar4 + ( nodeConst32 * vec4<f32>( nodeConst36 ) ) );
	let nodeConst37 = ( 0.02852226671021147 * nodeVar34 );
	nodeVar4 = ( nodeVar4 + ( nodeConst34 * vec4<f32>( nodeConst37 ) ) );
	nodeVar3 = ( nodeVar3 + nodeConst36 );
	nodeVar3 = ( nodeVar3 + nodeConst37 );
	nodeVar35 = ( nodeConst1 * ( object.nodeUniform2 * vec2<f32>( 7.0 ) ) );
	nodeVar36 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar35 ) );
	let nodeConst38 = nodeVar36;
	let nodeConst39 = abs( ( dot( nodeConst38.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar37 = exp( ( ( nodeConst39 * nodeConst39 ) * nodeConst0 ) );
	nodeVar38 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar35 ) );
	let nodeConst40 = nodeVar38;
	let nodeConst41 = abs( ( dot( nodeConst40.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar39 = exp( ( ( nodeConst41 * nodeConst41 ) * nodeConst0 ) );
	let nodeConst42 = ( 0.017587949779416395 * nodeVar37 );
	nodeVar4 = ( nodeVar4 + ( nodeConst38 * vec4<f32>( nodeConst42 ) ) );
	let nodeConst43 = ( 0.017587949779416395 * nodeVar39 );
	nodeVar4 = ( nodeVar4 + ( nodeConst40 * vec4<f32>( nodeConst43 ) ) );
	nodeVar3 = ( nodeVar3 + nodeConst42 );
	nodeVar3 = ( nodeVar3 + nodeConst43 );
	nodeVar40 = ( nodeConst1 * ( object.nodeUniform2 * vec2<f32>( 8.0 ) ) );
	nodeVar41 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar40 ) );
	let nodeConst44 = nodeVar41;
	let nodeConst45 = abs( ( dot( nodeConst44.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar42 = exp( ( ( nodeConst45 * nodeConst45 ) * nodeConst0 ) );
	nodeVar43 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar40 ) );
	let nodeConst46 = nodeVar43;
	let nodeConst47 = abs( ( dot( nodeConst46.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar44 = exp( ( ( nodeConst47 * nodeConst47 ) * nodeConst0 ) );
	let nodeConst48 = ( 0.010068006836002057 * nodeVar42 );
	nodeVar4 = ( nodeVar4 + ( nodeConst44 * vec4<f32>( nodeConst48 ) ) );
	let nodeConst49 = ( 0.010068006836002057 * nodeVar44 );
	nodeVar4 = ( nodeVar4 + ( nodeConst46 * vec4<f32>( nodeConst49 ) ) );
	nodeVar3 = ( nodeVar3 + nodeConst48 );
	nodeVar3 = ( nodeVar3 + nodeConst49 );
	nodeVar45 = ( nodeConst1 * ( object.nodeUniform2 * vec2<f32>( 9.0 ) ) );
	nodeVar46 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar45 ) );
	let nodeConst50 = nodeVar46;
	let nodeConst51 = abs( ( dot( nodeConst50.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar47 = exp( ( ( nodeConst51 * nodeConst51 ) * nodeConst0 ) );
	nodeVar48 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar45 ) );
	let nodeConst52 = nodeVar48;
	let nodeConst53 = abs( ( dot( nodeConst52.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar49 = exp( ( ( nodeConst53 * nodeConst53 ) * nodeConst0 ) );
	let nodeConst54 = ( 0.005350186131820889 * nodeVar47 );
	nodeVar4 = ( nodeVar4 + ( nodeConst50 * vec4<f32>( nodeConst54 ) ) );
	let nodeConst55 = ( 0.005350186131820889 * nodeVar49 );
	nodeVar4 = ( nodeVar4 + ( nodeConst52 * vec4<f32>( nodeConst55 ) ) );
	nodeVar3 = ( nodeVar3 + nodeConst54 );
	nodeVar3 = ( nodeVar3 + nodeConst55 );
	nodeVar50 = ( nodeConst1 * ( object.nodeUniform2 * vec2<f32>( 10.0 ) ) );
	nodeVar51 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 + nodeVar50 ) );
	let nodeConst56 = nodeVar51;
	let nodeConst57 = abs( ( dot( nodeConst56.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar52 = exp( ( ( nodeConst57 * nodeConst57 ) * nodeConst0 ) );
	nodeVar53 = textureSample( nodeUniform0, nodeUniform0_sampler, ( nodeVarying0 - nodeVar50 ) );
	let nodeConst58 = nodeVar53;
	let nodeConst59 = abs( ( dot( nodeConst58.xyz, vec3<f32>( 0.2126, 0.7152, 0.0722 ) ) - nodeVar2 ) );
	nodeVar54 = exp( ( ( nodeConst59 * nodeConst59 ) * nodeConst0 ) );
	let nodeConst60 = ( 0.002639315969304918 * nodeVar52 );
	nodeVar4 = ( nodeVar4 + ( nodeConst56 * vec4<f32>( nodeConst60 ) ) );
	let nodeConst61 = ( 0.002639315969304918 * nodeVar54 );
	nodeVar4 = ( nodeVar4 + ( nodeConst58 * vec4<f32>( nodeConst61 ) ) );
	nodeVar3 = ( nodeVar3 + nodeConst60 );
	nodeVar3 = ( nodeVar3 + nodeConst61 );

	// result

	output.color = ( nodeVar4 / vec4<f32>( max( nodeVar3, 0.0001 ) ) );

	return output;

}
