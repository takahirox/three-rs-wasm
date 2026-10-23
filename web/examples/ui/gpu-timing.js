// GPU time of the port's frames, for the Inspector's Performance tab: when
// the device has 'timestamp-query', each render and compute pass the page
// begins gets timestampWrites into a resident query set, and each frame's
// pass durations are read back through a resident buffer. As three's
// WebGPUTimestampQueryPool does, one readback is in flight at a time and the
// frames rendered meanwhile accumulate in the query set until it returns.
const CAPACITY = 2048; // queries ( 1024 passes ) between readbacks
const HISTORY = 16; // frames listed

export function installGpuTiming( device ) {

	if ( ! device || ! device.features.has( 'timestamp-query' ) ) return null;
	const querySet = device.createQuerySet( { type: 'timestamp', count: CAPACITY } );
	const resolve = device.createBuffer( { size: CAPACITY * 8, usage: GPUBufferUsage.QUERY_RESOLVE | GPUBufferUsage.COPY_SRC } );
	const result = device.createBuffer( { size: CAPACITY * 8, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ } );
	const results = new Map();
	let frames = []; // { id, start, end, complete } since the last readback
	let next = 0;
	let start = 0;
	let complete = true;
	let active = false;
	let pending = null; // the readback in flight
	const inject = ( descriptor ) => {

		if ( ! active || descriptor.timestampWrites ) return descriptor;
		if ( next + 2 > CAPACITY ) {

			complete = false;
			return descriptor;

		}

		const index = next;
		next += 2;
		return { ...descriptor, timestampWrites: { querySet, beginningOfPassWriteIndex: index, endOfPassWriteIndex: index + 1 } };

	};

	const renderPass = GPUCommandEncoder.prototype.beginRenderPass;
	GPUCommandEncoder.prototype.beginRenderPass = function ( descriptor ) {

		return renderPass.call( this, inject( descriptor ) );

	};

	const computePass = GPUCommandEncoder.prototype.beginComputePass;
	GPUCommandEncoder.prototype.beginComputePass = function ( descriptor = {} ) {

		return computePass.call( this, inject( descriptor ) );

	};

	const read = () => {

		const count = next;
		const read = frames;
		frames = [];
		next = 0;
		const encoder = device.createCommandEncoder();
		encoder.resolveQuerySet( querySet, 0, count, resolve, 0 );
		encoder.copyBufferToBuffer( resolve, 0, result, 0, count * 8 );
		device.queue.submit( [ encoder.finish() ] );
		pending = result.mapAsync( GPUMapMode.READ, 0, count * 8 ).then( () => {

			const times = new BigUint64Array( result.getMappedRange( 0, count * 8 ) );
			for ( const frame of read ) {

				if ( ! frame.complete ) continue;
				let ns = 0n;
				for ( let i = frame.start; i < frame.end; i += 2 ) if ( times[ i + 1 ] > times[ i ] ) ns += times[ i + 1 ] - times[ i ];
				results.set( frame.id, Number( ns ) / 1e6 );

			}

			result.unmap();
			// The Inspector looks up every listed frame on each of its frames:
			// list the last few, which it adds a frame after they render.
			while ( results.size > HISTORY ) results.delete( results.keys().next().value );

		} ).catch( () => {} ).finally( () => {

			pending = null;

		} );

	};

	return {
		results,
		// renderer.resolveTimestampsAsync(): the readback in flight.
		settled: () => pending ?? Promise.resolve(),
		begin() {

			start = next;
			complete = true;
			active = true;

		},
		end( frameId ) {

			active = false;
			if ( next > start ) frames.push( { id: frameId, start, end: next, complete } );
			if ( ! pending && next > 0 ) read();

		}
	};

}
