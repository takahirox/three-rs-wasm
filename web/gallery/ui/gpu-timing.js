// GPU time of the port's frames, for the Inspector's Performance tab: when
// the device has 'timestamp-query', each render and compute pass the page
// begins gets timestampWrites into a resident query set, and each frame's
// pass durations are resolved and read back through a ring of resident
// buffers. three's WebGPU backend times its passes the same way.
const CAPACITY = 512; // queries per frame ( 256 passes )
const RING = 4;

export function installGpuTiming( device ) {

	if ( ! device || ! device.features.has( 'timestamp-query' ) ) return null;
	const querySet = device.createQuerySet( { type: 'timestamp', count: CAPACITY } );
	const resolve = device.createBuffer( { size: CAPACITY * 8, usage: GPUBufferUsage.QUERY_RESOLVE | GPUBufferUsage.COPY_SRC } );
	const ring = Array.from( { length: RING }, () => ( { buffer: device.createBuffer( { size: CAPACITY * 8, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ } ), busy: false } ) );
	const results = new Map();
	let next = 0;
	let active = false;
	const inject = ( descriptor ) => {

		if ( ! active || descriptor.timestampWrites || next + 2 > CAPACITY ) return descriptor;
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

	return {
		results,
		begin() {

			next = 0;
			active = true;

		},
		end( frameId ) {

			active = false;
			const count = next;
			const slot = ring.find( s => ! s.busy );
			if ( count === 0 || ! slot ) return;
			slot.busy = true;
			const encoder = device.createCommandEncoder();
			encoder.resolveQuerySet( querySet, 0, count, resolve, 0 );
			encoder.copyBufferToBuffer( resolve, 0, slot.buffer, 0, count * 8 );
			device.queue.submit( [ encoder.finish() ] );
			slot.buffer.mapAsync( GPUMapMode.READ, 0, count * 8 ).then( () => {

				const times = new BigUint64Array( slot.buffer.getMappedRange( 0, count * 8 ) );
				let ns = 0n;
				for ( let i = 0; i < count; i += 2 ) if ( times[ i + 1 ] > times[ i ] ) ns += times[ i + 1 ] - times[ i ];
				slot.buffer.unmap();
				slot.busy = false;
				results.set( frameId, Number( ns ) / 1e6 );
				if ( results.size > 600 ) results.delete( results.keys().next().value );

			} ).catch( () => {

				slot.busy = false;

			} );

		}
	};

}
