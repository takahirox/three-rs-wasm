// DOM input only. Camera state and all scene work remain in Rust/Wasm.
export function installOrbit(canvas,app){
 const pointers=new Map();let button=0;
 canvas.style.touchAction='none';
 canvas.addEventListener('contextmenu',event=>event.preventDefault());
 canvas.addEventListener('pointerdown',event=>{
  button=event.button;pointers.set(event.pointerId,[event.clientX,event.clientY]);canvas.setPointerCapture(event.pointerId);
 });
 canvas.addEventListener('pointermove',event=>{
  const previous=pointers.get(event.pointerId);if(!previous)return;
  const before=[...pointers.values()];pointers.set(event.pointerId,[event.clientX,event.clientY]);
  if(pointers.size===1){
   const dx=event.clientX-previous[0],dy=event.clientY-previous[1];
   if(button===2||event.ctrlKey||event.metaKey||event.shiftKey)app.gallery_pan(dx,dy);
   else if(button===1)app.gallery_input(0,0,dy,false);
   else app.gallery_input(dx,dy,0,true);
  }else if(pointers.size===2){
   const after=[...pointers.values()],distance=p=>Math.hypot(p[1][0]-p[0][0],p[1][1]-p[0][1]);
   const a=distance(before),b=distance(after);
   if(a>0&&b>0)app.gallery_input(0,0,Math.log(b/a)/(.01*Math.log(.95)),false);
   app.gallery_pan((after[0][0]+after[1][0]-before[0][0]-before[1][0])/2,(after[0][1]+after[1][1]-before[0][1]-before[1][1])/2);
  }
 });
 for(const type of ['pointerup','pointercancel','lostpointercapture'])canvas.addEventListener(type,event=>pointers.delete(event.pointerId));
 canvas.addEventListener('wheel',event=>{event.preventDefault();const delta=event.deltaY*(event.deltaMode===1?16:event.deltaMode===2?100:1);app.gallery_input(0,0,delta,false);},{passive:false});
}
