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
// The soft body pages' btSoftRigidDynamicsWorld, with their createRigidBody
// and the bodies' handles in creation order.
export class AmmoSoftWorld {
	// soft: btSoftRigidDynamicsWorld; otherwise the rigid-only btDiscreteDynamicsWorld.
	constructor( AmmoLib, gravity, soft ) {
		this.lib = AmmoLib;
		const collisionConfiguration = soft ? new AmmoLib.btSoftBodyRigidBodyCollisionConfiguration() : new AmmoLib.btDefaultCollisionConfiguration();
		const dispatcher = new AmmoLib.btCollisionDispatcher( collisionConfiguration );
		this.dispatcher = dispatcher;
		const broadphase = new AmmoLib.btDbvtBroadphase();
		const solver = new AmmoLib.btSequentialImpulseConstraintSolver();
		this.tempBtVec3_1 = new AmmoLib.btVector3( 0, 0, 0 );
		if ( soft ) {
			const softBodySolver = new AmmoLib.btDefaultSoftBodySolver();
			this.world = new AmmoLib.btSoftRigidDynamicsWorld( dispatcher, broadphase, solver, collisionConfiguration, softBodySolver );
			this.world.setGravity( new AmmoLib.btVector3( 0, gravity, 0 ) );
			this.world.getWorldInfo().set_m_gravity( new AmmoLib.btVector3( 0, gravity, 0 ) );
		} else {
			this.world = new AmmoLib.btDiscreteDynamicsWorld( dispatcher, broadphase, solver, collisionConfiguration );
			this.world.setGravity( new AmmoLib.btVector3( 0, gravity, 0 ) );
		}
		this.transformAux1 = new AmmoLib.btTransform();
		this.bodies = [];
		this.dynamic = [];
		this.softBodies = [];
		this.constraints = [];
		this.helpers = new AmmoLib.btSoftBodyHelpers();
	}
	// createRigidBody( object, shape, mass, pos, quat ) with a box ( half extents ) or a sphere.
	rigid( kind, a, b, c, margin, mass, px, py, pz, qx, qy, qz, qw ) {
		return this.body( kind, a, b, c, margin, mass, px, py, pz, qx, qy, qz, qw, true );
	}
	// A body with a box ( 0, half extents ), sphere ( 1, radius ), cylinder ( 2, half extents )
	// or cone ( 3, radius and height ) shape; a negative margin keeps the shape's default.
	// keepActive disables deactivation of a dynamic body.
	body( kind, a, b, c, margin, mass, px, py, pz, qx, qy, qz, qw, keepActive ) {
		const Ammo = this.lib;
		const shape = kind === 0 ? new Ammo.btBoxShape( new Ammo.btVector3( a, b, c ) )
			: kind === 1 ? new Ammo.btSphereShape( a )
				: kind === 2 ? new Ammo.btCylinderShape( new Ammo.btVector3( a, b, c ) )
					: new Ammo.btConeShape( a, b );
		if ( margin >= 0 ) shape.setMargin( margin );
		return this.addBody( shape, mass, px, py, pz, qx, qy, qz, qw, keepActive );
	}
	// physics_ammo_break's createRigidBody(): a box ( 0, half extents ), sphere ( 1, radius ) or
	// convex hull ( 2, createConvexHullPhysicsShape over coords ) body with restitution and
	// friction 0.5 and the given velocities. A tag ≥ 0 is set as the body's user pointer, as
	// the debris' threeObject.
	breakBody( kind, a, b, c, coords, margin, mass, px, py, pz, qx, qy, qz, qw, vx, vy, vz, wx, wy, wz, tag ) {
		const Ammo = this.lib;
		let shape;
		if ( kind === 0 ) {
			shape = new Ammo.btBoxShape( new Ammo.btVector3( a, b, c ) );
		} else if ( kind === 1 ) {
			shape = new Ammo.btSphereShape( a );
		} else {
			shape = new Ammo.btConvexHullShape();
			for ( let i = 0, il = coords.length; i < il; i += 3 ) {
				this.tempBtVec3_1.setValue( coords[ i ], coords[ i + 1 ], coords[ i + 2 ] );
				const lastOne = ( i >= ( il - 3 ) );
				shape.addPoint( this.tempBtVec3_1, lastOne );
			}
		}
		shape.setMargin( margin );
		const transform = new Ammo.btTransform();
		transform.setIdentity();
		transform.setOrigin( new Ammo.btVector3( px, py, pz ) );
		transform.setRotation( new Ammo.btQuaternion( qx, qy, qz, qw ) );
		const motionState = new Ammo.btDefaultMotionState( transform );
		const localInertia = new Ammo.btVector3( 0, 0, 0 );
		shape.calculateLocalInertia( mass, localInertia );
		const rbInfo = new Ammo.btRigidBodyConstructionInfo( mass, motionState, shape, localInertia );
		rbInfo.set_m_restitution( 0.5 );
		rbInfo.set_m_friction( 0.5 );
		const body = new Ammo.btRigidBody( rbInfo );
		if ( tag >= 0 ) {
			body.setLinearVelocity( new Ammo.btVector3( vx, vy, vz ) );
			body.setAngularVelocity( new Ammo.btVector3( wx, wy, wz ) );
			const btVecUserData = new Ammo.btVector3( 0, 0, 0 );
			btVecUserData.tag = tag;
			body.setUserPointer( btVecUserData );
		}
		if ( mass > 0 ) {
			this.dynamic.push( body );
			body.setActivationState( 4 );
		}
		this.world.addRigidBody( body );
		this.bodies.push( body );
		return this.bodies.length - 1;
	}
	removeBody( body ) {
		this.world.removeRigidBody( this.bodies[ body ] );
	}
	// updatePhysics' manifold scan: for each manifold touching a tagged body with a point
	// in contact, [ tag0, tag1, impulse, impact point, impact normal, body 0's linear and
	// angular velocity, body 1's linear and angular velocity ] ( -1 for an untagged body ).
	contacts() {
		const Ammo = this.lib;
		const out = [];
		const tagOf = rb => {
			const tag = Ammo.castObject( rb.getUserPointer(), Ammo.btVector3 ).tag;
			return tag === undefined ? - 1 : tag;
		};
		for ( let i = 0, il = this.dispatcher.getNumManifolds(); i < il; i ++ ) {
			const contactManifold = this.dispatcher.getManifoldByIndexInternal( i );
			const rb0 = Ammo.castObject( contactManifold.getBody0(), Ammo.btRigidBody );
			const rb1 = Ammo.castObject( contactManifold.getBody1(), Ammo.btRigidBody );
			const tag0 = tagOf( rb0 ), tag1 = tagOf( rb1 );
			if ( tag0 < 0 && tag1 < 0 ) continue;
			let contact = false;
			let maxImpulse = 0;
			const impact = [ 0, 0, 0, 0, 0, 0 ];
			for ( let j = 0, jl = contactManifold.getNumContacts(); j < jl; j ++ ) {
				const contactPoint = contactManifold.getContactPoint( j );
				if ( contactPoint.getDistance() < 0 ) {
					contact = true;
					const impulse = contactPoint.getAppliedImpulse();
					if ( impulse > maxImpulse ) {
						maxImpulse = impulse;
						const pos = contactPoint.get_m_positionWorldOnB();
						const normal = contactPoint.get_m_normalWorldOnB();
						impact.splice( 0, 6, pos.x(), pos.y(), pos.z(), normal.x(), normal.y(), normal.z() );
					}
					break;
				}
			}
			if ( ! contact ) continue;
			const v = [];
			for ( const rb of [ rb0, rb1 ] ) {
				const vel = rb.getLinearVelocity();
				const angVel = rb.getAngularVelocity();
				v.push( vel.x(), vel.y(), vel.z(), angVel.x(), angVel.y(), angVel.z() );
			}
			out.push( tag0, tag1, maxImpulse, ...impact, ...v );
		}
		return new Float64Array( out );
	}
	// physics_ammo_terrain's createTerrainShape(): a PHY_FLOAT heightfield in the Ammo heap,
	// scaled to the extents, as a static body at ( 0, originY, 0 ).
	heightfield( width, depth, heights, minHeight, maxHeight, scaleX, scaleZ, margin, originY ) {
		const Ammo = this.lib;
		const data = Ammo._malloc( 4 * width * depth );
		Ammo.HEAPF32.set( heights, data >> 2 );
		const shape = new Ammo.btHeightfieldTerrainShape( width, depth, data, 1, minHeight, maxHeight, 1, 'PHY_FLOAT', false );
		shape.setLocalScaling( new Ammo.btVector3( scaleX, 1, scaleZ ) );
		shape.setMargin( margin );
		return this.addBody( shape, 0, 0, originY, 0, 0, 0, 0, 1, false );
	}
	addBody( shape, mass, px, py, pz, qx, qy, qz, qw, keepActive ) {
		const Ammo = this.lib;
		const transform = new Ammo.btTransform();
		transform.setIdentity();
		transform.setOrigin( new Ammo.btVector3( px, py, pz ) );
		transform.setRotation( new Ammo.btQuaternion( qx, qy, qz, qw ) );
		const motionState = new Ammo.btDefaultMotionState( transform );
		const localInertia = new Ammo.btVector3( 0, 0, 0 );
		shape.calculateLocalInertia( mass, localInertia );
		const rbInfo = new Ammo.btRigidBodyConstructionInfo( mass, motionState, shape, localInertia );
		const body = new Ammo.btRigidBody( rbInfo );
		if ( mass > 0 ) {
			this.dynamic.push( body );
			if ( keepActive ) body.setActivationState( 4 );
		}
		this.world.addRigidBody( body );
		this.bodies.push( body );
		return this.bodies.length - 1;
	}
	setFriction( body, friction ) {
		this.bodies[ body ].setFriction( friction );
	}
	// softBodyHelpers.CreateRope with the page's config, total mass and margin.
	rope( sx, sy, sz, ex, ey, ez, segments, fixeds, viterations, piterations, mass, margin ) {
		const Ammo = this.lib;
		const body = this.helpers.CreateRope( this.world.getWorldInfo(), new Ammo.btVector3( sx, sy, sz ), new Ammo.btVector3( ex, ey, ez ), segments, fixeds );
		const sbConfig = body.get_m_cfg();
		sbConfig.set_viterations( viterations );
		sbConfig.set_piterations( piterations );
		body.setTotalMass( mass, false );
		Ammo.castObject( body, Ammo.btCollisionObject ).getCollisionShape().setMargin( margin );
		this.world.addSoftBody( body, 1, - 1 );
		body.setActivationState( 4 );
		this.softBodies.push( body );
		return this.softBodies.length - 1;
	}
	// softBodyHelpers.CreatePatch with the page's config, total mass and margin.
	patch( c00x, c00y, c00z, c01x, c01y, c01z, c10x, c10y, c10z, c11x, c11y, c11z, resx, resy, fixeds, gendiags, viterations, piterations, mass, margin ) {
		const Ammo = this.lib;
		const V = ( x, y, z ) => new Ammo.btVector3( x, y, z );
		const body = this.helpers.CreatePatch( this.world.getWorldInfo(), V( c00x, c00y, c00z ), V( c01x, c01y, c01z ), V( c10x, c10y, c10z ), V( c11x, c11y, c11z ), resx, resy, fixeds, gendiags );
		const sbConfig = body.get_m_cfg();
		sbConfig.set_viterations( viterations );
		sbConfig.set_piterations( piterations );
		body.setTotalMass( mass, false );
		Ammo.castObject( body, Ammo.btCollisionObject ).getCollisionShape().setMargin( margin );
		this.world.addSoftBody( body, 1, - 1 );
		body.setActivationState( 4 );
		this.softBodies.push( body );
		return this.softBodies.length - 1;
	}
	// softBodyHelpers.CreateFromTriMesh with the volume page's config.
	triMesh( vertices, indices, randomize, viterations, piterations, collisions, kDF, kDP, kPR, kLST, kAST, mass, margin ) {
		const Ammo = this.lib;
		const body = this.helpers.CreateFromTriMesh( this.world.getWorldInfo(), Array.from( vertices ), Array.from( indices ), indices.length / 3, randomize );
		const sbConfig = body.get_m_cfg();
		sbConfig.set_viterations( viterations );
		sbConfig.set_piterations( piterations );
		sbConfig.set_collisions( collisions );
		sbConfig.set_kDF( kDF );
		sbConfig.set_kDP( kDP );
		sbConfig.set_kPR( kPR );
		body.get_m_materials().at( 0 ).set_m_kLST( kLST );
		body.get_m_materials().at( 0 ).set_m_kAST( kAST );
		body.setTotalMass( mass, false );
		Ammo.castObject( body, Ammo.btCollisionObject ).getCollisionShape().setMargin( margin );
		this.world.addSoftBody( body, 1, - 1 );
		body.setActivationState( 4 );
		this.softBodies.push( body );
		return this.softBodies.length - 1;
	}
	setLinearVelocity( body, x, y, z ) {
		this.bodies[ body ].setLinearVelocity( new this.lib.btVector3( x, y, z ) );
	}
	// The soft body's first `count` nodes: positions ( m_x ) then normals ( m_n ).
	nodesWithNormals( soft, count ) {
		const nodes = this.softBodies[ soft ].get_m_nodes();
		const out = new Float32Array( count * 6 );
		for ( let i = 0; i < count; i ++ ) {
			const node = nodes.at( i );
			const p = node.get_m_x();
			const n = node.get_m_n();
			out.set( [ p.x(), p.y(), p.z() ], i * 3 );
			out.set( [ n.x(), n.y(), n.z() ], ( count + i ) * 3 );
		}
		return out;
	}
	appendAnchor( soft, node, body, disableCollision, influence ) {
		this.softBodies[ soft ].appendAnchor( node, this.bodies[ body ], disableCollision, influence );
	}
	hinge( a, b, pax, pay, paz, pbx, pby, pbz, ax, ay, az ) {
		const Ammo = this.lib;
		const axis = new Ammo.btVector3( ax, ay, az );
		const hinge = new Ammo.btHingeConstraint( this.bodies[ a ], this.bodies[ b ], new Ammo.btVector3( pax, pay, paz ), new Ammo.btVector3( pbx, pby, pbz ), axis, axis, true );
		this.world.addConstraint( hinge, true );
		this.constraints.push( hinge );
		return this.constraints.length - 1;
	}
	enableAngularMotor( hinge, enable, velocity, impulse ) {
		this.constraints[ hinge ].enableAngularMotor( enable, velocity, impulse );
	}
	stepSimulation( delta, maxSubSteps ) {
		this.world.stepSimulation( delta, maxSubSteps );
	}
	// The dynamic bodies' motion state transforms: position and quaternion, 7 floats each.
	poses() {
		const out = new Float64Array( this.dynamic.length * 7 );
		for ( let i = 0; i < this.dynamic.length; i ++ ) {
			const ms = this.dynamic[ i ].getMotionState();
			if ( ! ms ) continue;
			ms.getWorldTransform( this.transformAux1 );
			const p = this.transformAux1.getOrigin();
			const q = this.transformAux1.getRotation();
			out.set( [ p.x(), p.y(), p.z(), q.x(), q.y(), q.z(), q.w() ], i * 7 );
		}
		return out;
	}
	// The soft body's first `count` nodes' positions ( m_x ).
	nodes( soft, count ) {
		const nodes = this.softBodies[ soft ].get_m_nodes();
		const out = new Float32Array( count * 3 );
		for ( let i = 0; i < count; i ++ ) {
			const p = nodes.at( i ).get_m_x();
			out[ i * 3 ] = p.x();
			out[ i * 3 + 1 ] = p.y();
			out[ i * 3 + 2 ] = p.z();
		}
		return out;
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

    pub type AmmoSoftWorld;
    #[wasm_bindgen(constructor)]
    fn new(lib: &JsValue, gravity: f64, soft: bool) -> AmmoSoftWorld;
    #[wasm_bindgen(method)]
    #[allow(clippy::too_many_arguments)]
    pub fn body(
        this: &AmmoSoftWorld,
        kind: u32,
        a: f64,
        b: f64,
        c: f64,
        margin: f64,
        mass: f64,
        px: f64,
        py: f64,
        pz: f64,
        qx: f64,
        qy: f64,
        qz: f64,
        qw: f64,
        keep_active: bool,
    ) -> u32;
    #[wasm_bindgen(method, js_name = breakBody)]
    #[allow(clippy::too_many_arguments)]
    pub fn break_body(
        this: &AmmoSoftWorld,
        kind: u32,
        a: f64,
        b: f64,
        c: f64,
        coords: &[f32],
        margin: f64,
        mass: f64,
        px: f64,
        py: f64,
        pz: f64,
        qx: f64,
        qy: f64,
        qz: f64,
        qw: f64,
        vx: f64,
        vy: f64,
        vz: f64,
        wx: f64,
        wy: f64,
        wz: f64,
        tag: i32,
    ) -> u32;
    #[wasm_bindgen(method, js_name = removeBody)]
    pub fn remove_body(this: &AmmoSoftWorld, body: u32);
    #[wasm_bindgen(method)]
    pub fn contacts(this: &AmmoSoftWorld) -> js_sys::Float64Array;
    #[wasm_bindgen(method)]
    #[allow(clippy::too_many_arguments)]
    pub fn heightfield(
        this: &AmmoSoftWorld,
        width: u32,
        depth: u32,
        heights: &[f32],
        min_height: f64,
        max_height: f64,
        scale_x: f64,
        scale_z: f64,
        margin: f64,
        origin_y: f64,
    ) -> u32;
    /// kind 0: a box with half extents ( a, b, c ); 1: a sphere of radius a.
    #[wasm_bindgen(method)]
    #[allow(clippy::too_many_arguments)]
    pub fn rigid(
        this: &AmmoSoftWorld,
        kind: u32,
        a: f64,
        b: f64,
        c: f64,
        margin: f64,
        mass: f64,
        px: f64,
        py: f64,
        pz: f64,
        qx: f64,
        qy: f64,
        qz: f64,
        qw: f64,
    ) -> u32;
    #[wasm_bindgen(method, js_name = setFriction)]
    pub fn set_friction(this: &AmmoSoftWorld, body: u32, friction: f64);
    #[wasm_bindgen(method)]
    #[allow(clippy::too_many_arguments)]
    pub fn rope(
        this: &AmmoSoftWorld,
        sx: f64,
        sy: f64,
        sz: f64,
        ex: f64,
        ey: f64,
        ez: f64,
        segments: u32,
        fixeds: u32,
        viterations: u32,
        piterations: u32,
        mass: f64,
        margin: f64,
    ) -> u32;
    #[wasm_bindgen(method)]
    #[allow(clippy::too_many_arguments)]
    pub fn patch(
        this: &AmmoSoftWorld,
        c00x: f64,
        c00y: f64,
        c00z: f64,
        c01x: f64,
        c01y: f64,
        c01z: f64,
        c10x: f64,
        c10y: f64,
        c10z: f64,
        c11x: f64,
        c11y: f64,
        c11z: f64,
        resx: u32,
        resy: u32,
        fixeds: u32,
        gendiags: bool,
        viterations: u32,
        piterations: u32,
        mass: f64,
        margin: f64,
    ) -> u32;
    #[wasm_bindgen(method, js_name = triMesh)]
    #[allow(clippy::too_many_arguments)]
    pub fn tri_mesh(
        this: &AmmoSoftWorld,
        vertices: &[f32],
        indices: &[u32],
        randomize: bool,
        viterations: u32,
        piterations: u32,
        collisions: u32,
        k_df: f64,
        k_dp: f64,
        k_pr: f64,
        k_lst: f64,
        k_ast: f64,
        mass: f64,
        margin: f64,
    ) -> u32;
    #[wasm_bindgen(method, js_name = setLinearVelocity)]
    pub fn set_linear_velocity(this: &AmmoSoftWorld, body: u32, x: f64, y: f64, z: f64);
    #[wasm_bindgen(method, js_name = nodesWithNormals)]
    pub fn nodes_with_normals(this: &AmmoSoftWorld, soft: u32, count: u32) -> js_sys::Float32Array;
    #[wasm_bindgen(method, js_name = appendAnchor)]
    pub fn append_anchor(
        this: &AmmoSoftWorld,
        soft: u32,
        node: u32,
        body: u32,
        disable_collision: bool,
        influence: f64,
    );
    #[wasm_bindgen(method)]
    #[allow(clippy::too_many_arguments)]
    pub fn hinge(
        this: &AmmoSoftWorld,
        a: u32,
        b: u32,
        pax: f64,
        pay: f64,
        paz: f64,
        pbx: f64,
        pby: f64,
        pbz: f64,
        ax: f64,
        ay: f64,
        az: f64,
    ) -> u32;
    #[wasm_bindgen(method, js_name = enableAngularMotor)]
    pub fn enable_angular_motor(
        this: &AmmoSoftWorld,
        hinge: u32,
        enable: bool,
        velocity: f64,
        impulse: f64,
    );
    #[wasm_bindgen(method, js_name = stepSimulation)]
    pub fn step_simulation(this: &AmmoSoftWorld, delta: f64, max_sub_steps: u32);
    #[wasm_bindgen(method)]
    pub fn poses(this: &AmmoSoftWorld) -> js_sys::Float64Array;
    #[wasm_bindgen(method)]
    pub fn nodes(this: &AmmoSoftWorld, soft: u32, count: u32) -> js_sys::Float32Array;
}

/// The soft body pages' Ammo().then( init ): the library and their world.
pub(super) async fn soft_world(gravity: f64) -> crate::Result<AmmoSoftWorld> {
    let lib = ammo_load()
        .await
        .map_err(|e| crate::Error::Asset(format!("ammo.js: {e:?}")))?;
    Ok(AmmoSoftWorld::new(&lib, gravity, true))
}

/// A rigid-only btDiscreteDynamicsWorld with the same bridge.
pub(super) async fn rigid_world(gravity: f64) -> crate::Result<AmmoSoftWorld> {
    let lib = ammo_load()
        .await
        .map_err(|e| crate::Error::Asset(format!("ammo.js: {e:?}")))?;
    Ok(AmmoSoftWorld::new(&lib, gravity, false))
}

/// AmmoPhysics(): the library ( loaded once per page ) and a new world.
pub(super) async fn world() -> crate::Result<AmmoWorld> {
    let lib = ammo_load()
        .await
        .map_err(|e| crate::Error::Asset(format!("ammo.js: {e:?}")))?;
    Ok(AmmoWorld::new(&lib))
}
