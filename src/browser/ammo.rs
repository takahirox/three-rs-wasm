//! ammo.js ( Bullet compiled with emscripten ), the same build the physics_ammo
//! pages load from jsdelivr. Its wasm module has emscripten's own memory and
//! glue, so it runs beside this crate's module and is called through
//! JavaScript. `AmmoWorld` is AmmoPhysics.js's world, bodies and step, with
//! the instances' poses written into one Float32Array per mesh, as the step
//! writes instanceMatrix.array.
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = r#"
const AMMO_PATH = 'https://cdn.jsdelivr.net/gh/kripken/ammo.js@79190a1f03845794b1bba1777f30037349967658/builds/ammo.wasm.js';
export async function ammoLoad() {
	if ( typeof Ammo === 'undefined' ) {
		await new Promise( ( resolve, reject ) => {
			const script = document.createElement( 'script' );
			script.src = AMMO_PATH;
			script.onload = resolve;
			script.onerror = reject;
			document.head.appendChild( script );
		} );
	}
	if ( globalThis.galleryAmmoLib === undefined ) globalThis.galleryAmmoLib = await Ammo();
	return globalThis.galleryAmmoLib;
}
function compose( position, quaternion, array, index ) {
	const x = quaternion.x(), y = quaternion.y(), z = quaternion.z(), w = quaternion.w();
	const x2 = x + x, y2 = y + y, z2 = z + z;
	const xx = x * x2, xy = x * y2, xz = x * z2;
	const yy = y * y2, yz = y * z2, zz = z * z2;
	const wx = w * x2, wy = w * y2, wz = w * z2;
	array[ index + 0 ] = ( 1 - ( yy + zz ) );
	array[ index + 1 ] = ( xy + wz );
	array[ index + 2 ] = ( xz - wy );
	array[ index + 3 ] = 0;
	array[ index + 4 ] = ( xy - wz );
	array[ index + 5 ] = ( 1 - ( xx + zz ) );
	array[ index + 6 ] = ( yz + wx );
	array[ index + 7 ] = 0;
	array[ index + 8 ] = ( xz + wy );
	array[ index + 9 ] = ( yz - wx );
	array[ index + 10 ] = ( 1 - ( xx + yy ) );
	array[ index + 11 ] = 0;
	array[ index + 12 ] = position.x();
	array[ index + 13 ] = position.y();
	array[ index + 14 ] = position.z();
	array[ index + 15 ] = 1;
}
export class AmmoWorld {
	constructor( AmmoLib ) {
		this.lib = AmmoLib;
		const collisionConfiguration = new AmmoLib.btDefaultCollisionConfiguration();
		const dispatcher = new AmmoLib.btCollisionDispatcher( collisionConfiguration );
		const broadphase = new AmmoLib.btDbvtBroadphase();
		const solver = new AmmoLib.btSequentialImpulseConstraintSolver();
		this.world = new AmmoLib.btDiscreteDynamicsWorld( dispatcher, broadphase, solver, collisionConfiguration );
		this.world.setGravity( new AmmoLib.btVector3( 0, - 9.8, 0 ) );
		this.worldTransform = new AmmoLib.btTransform();
		this.shapes = [];
		this.meshes = [];
		this.lastTime = 0;
	}
	boxShape( sx, sy, sz ) {
		const shape = new this.lib.btBoxShape( new this.lib.btVector3( sx, sy, sz ) );
		shape.setMargin( 0.05 );
		this.shapes.push( shape );
		return this.shapes.length - 1;
	}
	sphereShape( radius ) {
		const shape = new this.lib.btSphereShape( radius );
		shape.setMargin( 0.05 );
		this.shapes.push( shape );
		return this.shapes.length - 1;
	}
	body( shape, mass, restitution, transform ) {
		const AmmoLib = this.lib;
		const motionState = new AmmoLib.btDefaultMotionState( transform );
		const localInertia = new AmmoLib.btVector3( 0, 0, 0 );
		shape.calculateLocalInertia( mass, localInertia );
		const rbInfo = new AmmoLib.btRigidBodyConstructionInfo( mass, motionState, shape, localInertia );
		rbInfo.set_m_restitution( restitution );
		const body = new AmmoLib.btRigidBody( rbInfo );
		this.world.addRigidBody( body );
		return body;
	}
	// handleMesh: returns the mesh index, or -1 for a static body.
	addMesh( shapeIndex, mass, restitution, px, py, pz, qx, qy, qz, qw ) {
		const AmmoLib = this.lib;
		const transform = new AmmoLib.btTransform();
		transform.setIdentity();
		transform.setOrigin( new AmmoLib.btVector3( px, py, pz ) );
		transform.setRotation( new AmmoLib.btQuaternion( qx, qy, qz, qw ) );
		const body = this.body( this.shapes[ shapeIndex ], mass, restitution, transform );
		if ( mass <= 0 ) return - 1;
		this.meshes.push( { bodies: [ body ], array: new Float32Array( 16 ), instanced: false } );
		return this.meshes.length - 1;
	}
	// handleInstancedMesh over instanceMatrix.array.
	addInstancedMesh( shapeIndex, mass, restitution, array ) {
		const bodies = [];
		for ( let i = 0; i < array.length / 16; i ++ ) {
			const index = i * 16;
			const transform = new this.lib.btTransform();
			transform.setFromOpenGLMatrix( array.slice( index, index + 16 ) );
			bodies.push( this.body( this.shapes[ shapeIndex ], mass, restitution, transform ) );
		}
		if ( mass <= 0 ) return - 1;
		this.meshes.push( { bodies, array: new Float32Array( array ), instanced: true } );
		return this.meshes.length - 1;
	}
	setMeshPosition( mesh, index, x, y, z ) {
		const AmmoLib = this.lib;
		const body = this.meshes[ mesh ].bodies[ index ];
		body.setAngularVelocity( new AmmoLib.btVector3( 0, 0, 0 ) );
		body.setLinearVelocity( new AmmoLib.btVector3( 0, 0, 0 ) );
		this.worldTransform.setIdentity();
		this.worldTransform.setOrigin( new AmmoLib.btVector3( x, y, z ) );
		body.setWorldTransform( this.worldTransform );
	}
	// step(): returns whether the world stepped ( the first call only records the time ).
	step( time ) {
		let stepped = false;
		if ( this.lastTime > 0 ) {
			const delta = ( time - this.lastTime ) / 1000;
			this.world.stepSimulation( delta, 10 );
			for ( const mesh of this.meshes ) {
				for ( let j = 0; j < mesh.bodies.length; j ++ ) {
					mesh.bodies[ j ].getMotionState().getWorldTransform( this.worldTransform );
					compose( this.worldTransform.getOrigin(), this.worldTransform.getRotation(), mesh.array, j * 16 );
				}
			}
			stepped = true;
		}
		this.lastTime = time;
		return stepped;
	}
	// The mesh's poses: column-major matrices ( a plain mesh's position and quaternion are its compose ).
	poses( mesh ) {
		return this.meshes[ mesh ].array;
	}
}
"#)]
extern "C" {
    #[wasm_bindgen(catch, js_name = ammoLoad)]
    async fn ammo_load() -> Result<JsValue, JsValue>;

    pub type AmmoWorld;
    #[wasm_bindgen(constructor)]
    fn new(lib: &JsValue) -> AmmoWorld;
    #[wasm_bindgen(method, js_name = boxShape)]
    pub fn box_shape(this: &AmmoWorld, sx: f64, sy: f64, sz: f64) -> u32;
    #[wasm_bindgen(method, js_name = sphereShape)]
    pub fn sphere_shape(this: &AmmoWorld, radius: f64) -> u32;
    #[wasm_bindgen(method, js_name = addMesh)]
    #[allow(clippy::too_many_arguments)]
    pub fn add_mesh(
        this: &AmmoWorld,
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
        this: &AmmoWorld,
        shape: u32,
        mass: f64,
        restitution: f64,
        array: &[f32],
    ) -> i32;
    #[wasm_bindgen(method, js_name = setMeshPosition)]
    pub fn set_mesh_position(this: &AmmoWorld, mesh: i32, index: u32, x: f64, y: f64, z: f64);
    #[wasm_bindgen(method)]
    pub fn step(this: &AmmoWorld, time: f64) -> bool;
    #[wasm_bindgen(method)]
    pub fn poses(this: &AmmoWorld, mesh: i32) -> js_sys::Float32Array;
}

/// AmmoPhysics(): the library ( loaded once per page ) and a new world.
pub(super) async fn world() -> crate::Result<AmmoWorld> {
    let lib = ammo_load()
        .await
        .map_err(|e| crate::Error::Asset(format!("ammo.js: {e:?}")))?;
    Ok(AmmoWorld::new(&lib))
}
