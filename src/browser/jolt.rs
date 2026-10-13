//! JoltPhysics.js ( jolt-physics 1.0.0, the same wasm-compat build the page
//! imports from jsdelivr ). Like ammo.js ( `super::ammo` ), its emscripten
//! module runs beside this crate's and is called through JavaScript.
//! `JoltWorld` is JoltPhysics.js's system, collision layers, bodies and step,
//! with the instances' poses written into one Float32Array per mesh as the
//! step writes instanceMatrix.array ( three's Matrix4.compose ).
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = r#"
const JOLT_PATH = 'https://cdn.jsdelivr.net/npm/jolt-physics@1.0.0/dist/jolt-physics.wasm-compat.js';
let Jolt = null;
export async function joltLoad() {
	if ( Jolt === null ) {
		const { default: initJolt } = await import( JOLT_PATH );
		Jolt = await initJolt();
	}
}
const LAYER_NON_MOVING = 0;
const LAYER_MOVING = 1;
const NUM_OBJECT_LAYERS = 2;
function setupCollisionFiltering( settings ) {
	const objectFilter = new Jolt.ObjectLayerPairFilterTable( NUM_OBJECT_LAYERS );
	objectFilter.EnableCollision( LAYER_NON_MOVING, LAYER_MOVING );
	objectFilter.EnableCollision( LAYER_MOVING, LAYER_MOVING );
	const BP_LAYER_NON_MOVING = new Jolt.BroadPhaseLayer( 0 );
	const BP_LAYER_MOVING = new Jolt.BroadPhaseLayer( 1 );
	const NUM_BROAD_PHASE_LAYERS = 2;
	const bpInterface = new Jolt.BroadPhaseLayerInterfaceTable( NUM_OBJECT_LAYERS, NUM_BROAD_PHASE_LAYERS );
	bpInterface.MapObjectToBroadPhaseLayer( LAYER_NON_MOVING, BP_LAYER_NON_MOVING );
	bpInterface.MapObjectToBroadPhaseLayer( LAYER_MOVING, BP_LAYER_MOVING );
	settings.mObjectLayerPairFilter = objectFilter;
	settings.mBroadPhaseLayerInterface = bpInterface;
	settings.mObjectVsBroadPhaseLayerFilter = new Jolt.ObjectVsBroadPhaseLayerFilterTable( settings.mBroadPhaseLayerInterface, NUM_BROAD_PHASE_LAYERS, settings.mObjectLayerPairFilter, NUM_OBJECT_LAYERS );
}
// Quaternion.setFromRotationMatrix over Matrix4.fromArray( array, offset ).
function rotationFromArray( te, o ) {
	const m11 = te[ o ], m12 = te[ o + 4 ], m13 = te[ o + 8 ], m21 = te[ o + 1 ], m22 = te[ o + 5 ], m23 = te[ o + 9 ], m31 = te[ o + 2 ], m32 = te[ o + 6 ], m33 = te[ o + 10 ];
	const trace = m11 + m22 + m33;
	if ( trace > 0 ) {
		const s = 0.5 / Math.sqrt( trace + 1.0 );
		return { w: 0.25 / s, x: ( m32 - m23 ) * s, y: ( m13 - m31 ) * s, z: ( m21 - m12 ) * s };
	} else if ( m11 > m22 && m11 > m33 ) {
		const s = 2.0 * Math.sqrt( 1.0 + m11 - m22 - m33 );
		return { w: ( m32 - m23 ) / s, x: 0.25 * s, y: ( m12 + m21 ) / s, z: ( m13 + m31 ) / s };
	} else if ( m22 > m33 ) {
		const s = 2.0 * Math.sqrt( 1.0 + m22 - m11 - m33 );
		return { w: ( m13 - m31 ) / s, x: ( m12 + m21 ) / s, y: 0.25 * s, z: ( m23 + m32 ) / s };
	}
	const s = 2.0 * Math.sqrt( 1.0 + m33 - m11 - m22 );
	return { w: ( m21 - m12 ) / s, x: ( m13 + m31 ) / s, y: ( m23 + m32 ) / s, z: 0.25 * s };
}
// Matrix4.compose( position, quaternion, ( 1, 1, 1 ) ).toArray( array, index ).
function compose( px, py, pz, x, y, z, w, array, index ) {
	const x2 = x + x, y2 = y + y, z2 = z + z;
	const xx = x * x2, xy = x * y2, xz = x * z2;
	const yy = y * y2, yz = y * z2, zz = z * z2;
	const wx = w * x2, wy = w * y2, wz = w * z2;
	array[ index + 0 ] = 1 - ( yy + zz );
	array[ index + 1 ] = xy + wz;
	array[ index + 2 ] = xz - wy;
	array[ index + 3 ] = 0;
	array[ index + 4 ] = xy - wz;
	array[ index + 5 ] = 1 - ( xx + zz );
	array[ index + 6 ] = yz + wx;
	array[ index + 7 ] = 0;
	array[ index + 8 ] = xz + wy;
	array[ index + 9 ] = yz - wx;
	array[ index + 10 ] = 1 - ( xx + yy );
	array[ index + 11 ] = 0;
	array[ index + 12 ] = px;
	array[ index + 13 ] = py;
	array[ index + 14 ] = pz;
	array[ index + 15 ] = 1;
}
export class JoltWorld {
	constructor() {
		const settings = new Jolt.JoltSettings();
		setupCollisionFiltering( settings );
		this.jolt = new Jolt.JoltInterface( settings );
		Jolt.destroy( settings );
		this.bodyInterface = this.jolt.GetPhysicsSystem().GetBodyInterface();
		this.shapes = [];
		this.meshes = [];
	}
	boxShape( sx, sy, sz ) {
		this.shapes.push( new Jolt.BoxShape( new Jolt.Vec3( sx, sy, sz ), 0.05 * Math.min( sx, sy, sz ), null ) );
		return this.shapes.length - 1;
	}
	sphereShape( radius ) {
		this.shapes.push( new Jolt.SphereShape( radius, null ) );
		return this.shapes.length - 1;
	}
	createBody( position, rotation, mass, restitution, shape ) {
		const pos = new Jolt.Vec3( position.x, position.y, position.z );
		const rot = new Jolt.Quat( rotation.x, rotation.y, rotation.z, rotation.w );
		const motion = mass > 0 ? Jolt.EMotionType_Dynamic : Jolt.EMotionType_Static;
		const layer = mass > 0 ? LAYER_MOVING : LAYER_NON_MOVING;
		const creationSettings = new Jolt.BodyCreationSettings( shape, pos, rot, motion, layer );
		creationSettings.mRestitution = restitution;
		const body = this.bodyInterface.CreateBody( creationSettings );
		this.bodyInterface.AddBody( body.GetID(), Jolt.EActivation_Activate );
		Jolt.destroy( creationSettings );
		return body;
	}
	// A plain mesh: returns the mesh index, or -1 for a static body.
	addMesh( shapeIndex, mass, restitution, px, py, pz, qx, qy, qz, qw ) {
		const body = this.createBody( { x: px, y: py, z: pz }, { x: qx, y: qy, z: qz, w: qw }, mass, restitution, this.shapes[ shapeIndex ] );
		if ( mass <= 0 ) return - 1;
		this.meshes.push( { bodies: [ body ], array: new Float32Array( 16 ), mass, restitution } );
		return this.meshes.length - 1;
	}
	// createInstancedBody over instanceMatrix.array.
	addInstancedMesh( shapeIndex, mass, restitution, array ) {
		const bodies = [];
		for ( let i = 0; i < array.length / 16; i ++ ) {
			const position = { x: array[ i * 16 + 12 ], y: array[ i * 16 + 13 ], z: array[ i * 16 + 14 ] };
			bodies.push( this.createBody( position, rotationFromArray( array, i * 16 ), mass, restitution, this.shapes[ shapeIndex ] ) );
		}
		if ( mass <= 0 ) return - 1;
		this.meshes.push( { bodies, array: new Float32Array( array ), mass, restitution } );
		return this.meshes.length - 1;
	}
	// setMeshPosition: the body is replaced by a new one at rest.
	setMeshPosition( meshIndex, index, x, y, z ) {
		const mesh = this.meshes[ meshIndex ];
		const body = mesh.bodies[ index ];
		this.bodyInterface.RemoveBody( body.GetID() );
		this.bodyInterface.DestroyBody( body.GetID() );
		const shape = body.GetShape();
		mesh.bodies[ index ] = this.createBody( { x, y, z }, { x: 0, y: 0, z: 0, w: 1 }, mesh.mass, mesh.restitution, shape );
	}
	// step() with the Timer's delta in seconds.
	step( delta ) {
		let deltaTime = Math.min( delta, 1.0 / 30.0 );
		const numSteps = deltaTime > 1.0 / 55.0 ? 2 : 1;
		this.jolt.Step( deltaTime, numSteps );
		for ( const mesh of this.meshes ) {
			for ( let j = 0; j < mesh.bodies.length; j ++ ) {
				const body = mesh.bodies[ j ];
				const p = body.GetPosition();
				const q = body.GetRotation();
				compose( p.GetX(), p.GetY(), p.GetZ(), q.GetX(), q.GetY(), q.GetZ(), q.GetW(), mesh.array, j * 16 );
			}
		}
	}
	poses( mesh ) {
		return this.meshes[ mesh ].array;
	}
}
"#)]
extern "C" {
    #[wasm_bindgen(catch, js_name = joltLoad)]
    async fn jolt_load() -> Result<JsValue, JsValue>;

    pub type JoltWorld;
    #[wasm_bindgen(constructor)]
    fn new() -> JoltWorld;
    #[wasm_bindgen(method, js_name = boxShape)]
    pub fn box_shape(this: &JoltWorld, sx: f64, sy: f64, sz: f64) -> u32;
    #[wasm_bindgen(method, js_name = sphereShape)]
    pub fn sphere_shape(this: &JoltWorld, radius: f64) -> u32;
    #[wasm_bindgen(method, js_name = addMesh)]
    #[allow(clippy::too_many_arguments)]
    pub fn add_mesh(
        this: &JoltWorld,
        shape: u32,
        mass: f64,
        restitution: f64,
        px: f64,
        py: f64,
        pz: f64,
        qx: f64,
        qy: f64,
        qz: f64,
        qw: f64,
    ) -> i32;
    #[wasm_bindgen(method, js_name = addInstancedMesh)]
    pub fn add_instanced_mesh(
        this: &JoltWorld,
        shape: u32,
        mass: f64,
        restitution: f64,
        array: &[f32],
    ) -> i32;
    #[wasm_bindgen(method, js_name = setMeshPosition)]
    pub fn set_mesh_position(this: &JoltWorld, mesh: i32, index: u32, x: f64, y: f64, z: f64);
    #[wasm_bindgen(method)]
    pub fn step(this: &JoltWorld, delta: f64);
    #[wasm_bindgen(method)]
    pub fn poses(this: &JoltWorld, mesh: i32) -> js_sys::Float32Array;
}

/// JoltPhysics(): the library ( loaded once per page ) and a new system.
pub(super) async fn world() -> crate::Result<JoltWorld> {
    jolt_load()
        .await
        .map_err(|e| crate::Error::Asset(format!("jolt-physics: {e:?}")))?;
    Ok(JoltWorld::new())
}
