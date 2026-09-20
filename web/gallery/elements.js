// DOM layout and input only; all forty scenes share the Rust renderer/device.
export function installElements(canvas,app,multipleCanvas=false){
 const style=document.createElement('style');style.textContent=`
 *{box-sizing:border-box;-webkit-font-smoothing:antialiased;-moz-osx-font-smoothing:auto}html,body{color-scheme:light;overflow:auto;background:#fff;color:#444}body{margin:0;font:13px/24px monospace}
 #elements-content{position:absolute;top:0;width:100%;z-index:1;padding:3em 0 0}
 canvas{position:fixed;left:0;top:0;width:100%;height:100%;pointer-events:none}
 .list-item{display:inline-block;margin:1em;padding:1em;box-shadow:1px 2px 4px 0 rgba(0,0,0,.25)}
 .list-item>div:first-child{width:200px;height:200px;touch-action:none}
 .list-item>div:nth-child(2){color:#888;font-family:sans-serif;font-size:large;font-weight:400;line-height:24px;width:200px;margin-top:.5em}
 #notice{z-index:2}`;document.head.append(style);
 const content=document.createElement('div');content.lang='en';content.id='elements-content';document.body.append(content);
 const views=[];const canvases=[];if(multipleCanvas){canvas.style.display='none';style.textContent+=' .list-item>canvas{position:static;display:block;width:200px;height:200px;pointer-events:auto}';}
 for(let i=0;i<40;i++){
  const item=document.createElement('div');item.className='list-item';
  const view=document.createElement(multipleCanvas?'canvas':'div');if(multipleCanvas){view.width=view.height=Math.round(200*devicePixelRatio);canvases.push(view);}item.append(view);views.push(view);
  const label=document.createElement('div');label.textContent=`Scene ${i+1}`;item.append(label);content.append(item);
  let previous=null;
  view.addEventListener('pointerdown',e=>{previous=[e.clientX,e.clientY];view.setPointerCapture(e.pointerId);app.tsl_parameter(0,i);});
  view.addEventListener('pointermove',e=>{if(previous){app.tsl_parameter(0,i);app.gallery_input(e.clientX-previous[0],e.clientY-previous[1],0,true);previous=[e.clientX,e.clientY];}});
  for(const event of ['pointerup','pointercancel','lostpointercapture'])view.addEventListener(event,()=>previous=null);
 }
 if(multipleCanvas){app.gallery_canvases(canvases);return;}
 const update=()=>{const ratio=canvas.width/innerWidth;views.forEach((view,i)=>{const r=view.getBoundingClientRect();app.gallery_viewport(i,r.left*ratio,r.top*ratio,r.width*ratio,r.height*ratio);});};
 addEventListener('resize',update);addEventListener('scroll',update,{passive:true});update();
}
