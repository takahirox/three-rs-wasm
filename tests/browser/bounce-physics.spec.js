import { test, expect } from '@playwright/test';

// webgpu_postprocessing_ssgi_ballpool steps @perplexdotgg/bounce. The Rust
// port runs the same scripted session ( the example's walls, ball count and
// spawn ranges for a viewport aspect, then respawns, pointer-style impulses
// and the timer's uneven frame deltas ) beside the original package in the
// same page, and every body's position, orientation, sleep flag and
// velocities must match bit for bit after every frame.

const PAGE = `<!doctype html><script type="importmap">{"imports":{
"monomorph":"/node_modules/monomorph/build/monomorph.js",
"@perplexdotgg/bounce":"/node_modules/@perplexdotgg/bounce/build/bounce.js"}}</script><body></body>`;

// The quiet session leaves the pool alone until bodies settle and sleep.
for (const [aspect, frames, quiet] of [[1, 600, false], [16 / 9, 900, false], [1, 1500, true]]) {
	test(`bounce port matches the original bit for bit (aspect ${aspect.toFixed(3)}${quiet ? ', settling' : ''})`, async ({ page }) => {
		test.setTimeout(300000);
		await page.route('**/bounce-oracle.html', (route) => route.fulfill({ contentType: 'text/html', body: PAGE }));
		await page.goto('/bounce-oracle.html');
		const result = await page.evaluate(async ({ aspect, frames, quiet }) => {
			const { World } = await import('@perplexdotgg/bounce');
			const wasm = await import('/web/pkg/three_rs_wasm.js');
			await wasm.default();
			let seed = 186;
			const rnd = () => { seed = (seed * 1103515245 + 12345) & 0x7fffffff; return seed / 0x7fffffff; };
			// The example's constants and sizing ( getBoxWidth, getBallCount ).
			const BALL = 0.4, H = 6, D = 8, T = 0.5;
			const vFov = 22.5 * Math.PI / 180;
			const dist = (H / 2) / Math.tan(vFov);
			const W = Math.tan(vFov) * aspect * dist * 2;
			const count = Math.floor(W * H * D * 0.4 * 0.6 / ((4 / 3) * Math.PI * Math.pow(BALL, 3)));
			const world = new World({ gravity: [0, -9.81, 0], solveVelocityIterations: 6, solvePositionIterations: 2, linearDamping: 0.1, angularDamping: 0.1, restitution: 0.4, friction: 0.5 });
			const port = new wasm.BounceWorld();
			const walls = [[[W, T, D], [0, -T / 2, 0]], [[W, T, D], [0, H + T / 2, 0]], [[W, H, T], [0, H / 2, -D / 2 - T / 2]], [[W, H, T], [0, H / 2, D / 2 + T / 2]], [[T, H, D], [-W / 2 - T / 2, H / 2, 0]], [[T, H, D], [W / 2 + T / 2, H / 2, 0]]];
			for (const [s, p] of walls) {
				world.createStaticBody({ shape: world.createBox({ width: s[0], height: s[1], depth: s[2] }), position: p });
				port.createStaticBox(new Float64Array(s), new Float64Array(p));
			}
			const sphere = world.createSphere({ radius: BALL });
			const portSphere = port.createSphere(BALL);
			const hw = W / 2 - BALL - 0.1, hd = D / 2 - BALL - 0.1;
			const bodies = [], portBodies = [];
			for (let i = 0; i < count; i++) {
				const p = [(rnd() - 0.5) * 2 * hw, BALL + rnd() * (H - BALL * 2), (rnd() - 0.5) * 2 * hd];
				bodies.push(world.createDynamicBody({ shape: sphere, position: p, mass: 1, restitution: 0.5, friction: 0.4 }));
				portBodies.push(port.createDynamicBody(portSphere, new Float64Array(p), 1, 0.5, 0.4));
			}
			const f64 = new Float64Array(1), u64 = new BigUint64Array(f64.buffer);
			const bits = (x) => { f64[0] = x; return u64[0]; };
			let moving = 0, sleeping = 0;
			for (let f = 0; f < frames; f++) {
				if (!quiet && f % 40 === 20) for (let k = 0; k < 5; k++) {
					// respawnBalls()
					const i = Math.floor(rnd() * bodies.length);
					const p = [(rnd() - 0.5) * 2 * hw, H - BALL - rnd() * 1, (rnd() - 0.5) * 2 * hd];
					const b = bodies[i];
					b.position.set(p); b.linearVelocity.set([0, 0, 0]); b.angularVelocity.set([0, 0, 0]); b.commitChanges();
					port.respawn(portBodies[i], new Float64Array(p));
				}
				if (!quiet && f % 25 === 10) for (let k = 0; k < 30; k++) {
					const i = Math.floor(rnd() * bodies.length);
					const s = [(rnd() - 0.5) * 30, rnd() * 15, (rnd() - 0.5) * 30];
					bodies[i].applyLinearImpulse({ x: s[0], y: s[1], z: s[2] });
					port.applyLinearImpulse(portBodies[i], new Float64Array(s));
				}
				const dt = quiet ? 1 / 60 : [1 / 60, 1 / 60, 1 / 30, 1 / 60, 1 / 45, 1 / 60, 1 / 120][f % 7];
				world.advanceTime(1 / 60, dt);
				port.advanceTime(1 / 60, dt, performance.now() / 1e3);
				for (let i = 0; i < count; i++) {
					const b = bodies[i];
					const want = [b.position.x, b.position.y, b.position.z, b.orientation.x, b.orientation.y, b.orientation.z, b.orientation.w, b.isSleeping ? 1 : 0, b.linearVelocity.x, b.linearVelocity.y, b.linearVelocity.z, b.angularVelocity.x, b.angularVelocity.y, b.angularVelocity.z];
					const got = port.state(portBodies[i]);
					for (let j = 0; j < 14; j++) if (bits(got[j]) !== bits(want[j])) return { frame: f, body: i, field: j, got: Array.from(got), want };
					if (b.isSleeping) sleeping++; else moving++;
				}
			}
			return { count, moving, sleeping, steps: world.step };
		}, { aspect, frames, quiet });
		expect(result.frame, JSON.stringify(result)).toBeUndefined();
		expect(result.count).toBe(aspect === 1 ? 257 : 458);
		expect(result.moving).toBeGreaterThan(0);
		if (quiet) expect(result.sleeping).toBeGreaterThan(0);
	});
}
