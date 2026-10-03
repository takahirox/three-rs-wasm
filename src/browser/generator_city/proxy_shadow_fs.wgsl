// Three.js r186 - Node System

// global
diagnostic( off, derivative_uniformity );


// structs

struct OutputStruct {
	@location( 0 ) color: vec4<f32>
};
var<private> output : OutputStruct;

// uniforms

struct objectStruct {
	nodeUniform1 : mat4x4<f32>,
	nodeUniform2 : u32,
	nodeUniform3 : f32
};
@binding( 0 ) @group( 1 )
var<uniform> object : objectStruct;

// vars
var<private> DiffuseColor : vec4<f32>;
var<private> nodeVar0 : f32;
var<private> nodeVar1 : f32;
var<private> nodeVar2 : f32;
var<private> nodeVar3 : f32;
var<private> nodeVar4 : u32;
var<private> nodeVar5 : u32;
var<private> nodeVar6 : u32;
var<private> nodeVar7 : u32;
var<private> nodeVar8 : f32;
var<private> nodeVar9 : u32;
var<private> nodeVar10 : u32;
var<private> Output : vec4<f32>;
var<private> nodeVar11 : vec4<f32>;

// codes


@fragment
fn main( @location( 0 ) v_positionWorld : vec3<f32> ) -> OutputStruct {

	// flow
	// code

	nodeVar0 = ( v_positionWorld.x + 101.0 );
	nodeVar1 = floor( ( nodeVar0 / 112.0 ) );
	nodeVar2 = ( v_positionWorld.z + 71.0 );
	nodeVar3 = floor( ( nodeVar2 / 82.0 ) );
	nodeVar4 = object.nodeUniform2;
	nodeVar5 = ( ( ( u32( ( ( ( nodeVar1 * 3.0 ) + clamp( floor( ( ( ( nodeVar0 - ( nodeVar1 * 112.0 ) ) - 5.0 ) / 26.666666666666668 ) ), 0.0, 2.0 ) ) + 4096.0 ) ) * 73856093u ) ^ ( u32( ( ( ( nodeVar3 * 2.0 ) + clamp( floor( ( ( ( nodeVar2 - ( nodeVar3 * 82.0 ) ) - 5.0 ) / 25.0 ) ), 0.0, 1.0 ) ) + 4096.0 ) ) * 19349663u ) ) ^ ( nodeVar4 * 2654435761u ) );
	nodeVar6 = ( ( ( nodeVar5 + 230900u ) * 747796405u ) + 2891336453u );
	nodeVar7 = ( ( ( nodeVar6 >> ( ( nodeVar6 >> 28u ) + 4u ) ) ^ nodeVar6 ) * 277803737u );
	nodeVar8 = ( f32( ( ( nodeVar7 >> 22u ) ^ nodeVar7 ) ) * 2.3283064365386963e-10 );
	nodeVar9 = ( ( ( nodeVar5 + 155260u ) * 747796405u ) + 2891336453u );
	nodeVar10 = ( ( ( nodeVar9 >> ( ( nodeVar9 >> 28u ) + 4u ) ) ^ nodeVar9 ) * 277803737u );
	DiffuseColor = vec4<f32>( vec3<f32>( 0.0, 0.0, 0.0 ), ( 1.0 * vec4<f32>( ( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( mix( vec3<f32>( 0.3915724777393922, 0.09084171117479915, 0.04518620437910499 ), vec3<f32>( 0.33245153633549385, 0.06847816983662762, 0.0343398068028541 ), step( 0.058823529411764705, nodeVar8 ) ), vec3<f32>( 0.2541520943200296, 0.14412847084818123, 0.08437621153575764 ), step( 0.11764705882352941, nodeVar8 ) ), vec3<f32>( 0.20507873637973145, 0.12743768042608497, 0.08021982030622662 ), step( 0.17647058823529413, nodeVar8 ) ), vec3<f32>( 0.55201140150344, 0.36625259558833256, 0.16202937562896222 ), step( 0.23529411764705882, nodeVar8 ) ), vec3<f32>( 0.4793201830913402, 0.3231432091022285, 0.158960835050774 ), step( 0.29411764705882354, nodeVar8 ) ), vec3<f32>( 0.5394794890033748, 0.4396571738310091, 0.22696587349938613 ), step( 0.35294117647058826, nodeVar8 ) ), vec3<f32>( 0.5647115056965487, 0.5271151256969157, 0.4452011945063733 ), step( 0.4117647058823529, nodeVar8 ) ), vec3<f32>( 0.5647115056965487, 0.5271151256969157, 0.4452011945063733 ), step( 0.47058823529411764, nodeVar8 ) ), vec3<f32>( 0.508881320845802, 0.4735314961384573, 0.3915724777393922 ), step( 0.5294117647058824, nodeVar8 ) ), vec3<f32>( 0.6375968739867731, 0.6038273388475408, 0.514917665367466 ), step( 0.5882352941176471, nodeVar8 ) ), vec3<f32>( 0.45641102317066595, 0.4286904966038916, 0.35640014413537763 ), step( 0.6470588235294118, nodeVar8 ) ), vec3<f32>( 0.3231432091022285, 0.3139887133649649, 0.2746773120495699 ), step( 0.7058823529411765, nodeVar8 ) ), vec3<f32>( 0.25818285291079235, 0.2501582847191642, 0.22696587349938613 ), step( 0.7647058823529411, nodeVar8 ) ), vec3<f32>( 0.37626212298046485, 0.36625259558833256, 0.3231432091022285 ), step( 0.8235294117647058, nodeVar8 ) ), vec3<f32>( 0.7083757798856457, 0.6724431569510133, 0.5972017883558645 ), step( 0.8823529411764706, nodeVar8 ) ), vec3<f32>( 0.20155625378383743, 0.23839757380151394, 0.2663556047920505 ), step( 0.9411764705882353, nodeVar8 ) ) * vec3<f32>( ( ( ( f32( ( ( nodeVar10 >> 22u ) ^ nodeVar10 ) ) * 2.3283064365386963e-10 ) * 0.12 ) + 0.94 ) ) ), 1.0 ).w ) );
	DiffuseColor.w = ( DiffuseColor.w * object.nodeUniform3 );
	nodeVar11 = max( vec4<f32>( DiffuseColor.xyz, DiffuseColor.w ), vec4<f32>( 0.0 ) );
	Output = nodeVar11;

	// result

	output.color = nodeVar11;

	return output;

}
