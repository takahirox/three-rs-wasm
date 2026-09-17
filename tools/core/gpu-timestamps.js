// Optional, sampled diagnostics. Timestamp readback is outside the render loop.
export function timestamps() {
 const request = GPUAdapter.prototype.requestDevice;
 GPUAdapter.prototype.requestDevice=async function(descriptor={}) {
  const supported=this.features.has('timestamp-query');
  const device=await request.call(this,supported?{...descriptor,requiredFeatures:[...new Set([...(descriptor.requiredFeatures||[]),'timestamp-query'])]}:descriptor);
  window.gpuTimings={supported,passes:[],samples:0};
  if(!supported)return device;
  const create=device.createCommandEncoder.bind(device),submit=device.queue.submit.bind(device.queue),pending=new WeakMap();
  device.createCommandEncoder=(...args)=>{
   const encoder=create(...args);
   if(!window.sampleGPU||window.gpuTimings.samples>=120)return encoder;
   window.gpuTimings.samples++;
   const queries=device.createQuerySet({type:'timestamp',count:32});
   const labels=[];const begin=encoder.beginRenderPass.bind(encoder),finish=encoder.finish.bind(encoder);
   encoder.beginRenderPass=descriptor=>{
    if(labels.length===16||descriptor.timestampWrites)return begin(descriptor);
    const i=labels.length;labels.push(descriptor.label||'render');
    return begin({...descriptor,timestampWrites:{querySet:queries,beginningOfPassWriteIndex:i*2,endOfPassWriteIndex:i*2+1}});
   };
   encoder.finish=(...args)=>{
    if(!labels.length){queries.destroy();return finish(...args);}
    const size=labels.length*16;
    const resolve=device.createBuffer({size,usage:GPUBufferUsage.QUERY_RESOLVE|GPUBufferUsage.COPY_SRC});
    const read=device.createBuffer({size,usage:GPUBufferUsage.MAP_READ|GPUBufferUsage.COPY_DST});
    encoder.resolveQuerySet(queries,0,labels.length*2,resolve,0);encoder.copyBufferToBuffer(resolve,0,read,0,size);
    const commands=finish(...args);pending.set(commands,{queries,resolve,read,labels,frame:window.gpuProfileFrame});return commands;
   };
   return encoder;
  };
  device.queue.submit=commands=>{
   const list=[...commands];submit(list);
   for(const command of list){const p=pending.get(command);if(!p)continue;
    p.read.mapAsync(GPUMapMode.READ).then(()=>{
     const values=new BigUint64Array(p.read.getMappedRange());
     p.labels.forEach((label,i)=>window.gpuTimings.passes.push({label,frame:p.frame,ms:Number(values[i*2+1]-values[i*2])/1e6}));
     p.read.unmap();
    }).finally(()=>{p.read.destroy();p.resolve.destroy();p.queries.destroy();});
   }
  };
  return device;
 };
}
