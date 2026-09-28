// Web Audio and DOM integration only. Pitch/delay processing executes in Rust TSL on the GPU.
export async function prepareAudio(loading) {
 const button=document.createElement('button');button.textContent='Play';loading.replaceChildren(button);
 await new Promise(resolve=>button.addEventListener('click',resolve,{once:true}));button.disabled=true;
 const context=new AudioContext();
 const encoded=await fetch(new URL('assets/tsl-procedural/webgpu-audio-processing.mp3',import.meta.url)).then(r=>{if(!r.ok)throw Error(`Audio: ${r.status}`);return r.arrayBuffer();});
 const decoded=await context.decodeAudioData(encoded);
 const wave=new Float32Array(decoded.length+200000);wave.set(decoded.getChannelData(0));
 window.galleryAudioSource=wave;
 // Pinned r186 divides by the channel count. Preserve its playback rate.
 window.galleryAudioRate=decoded.sampleRate/decoded.numberOfChannels;
 await context.close();
}
export function installAudio(app,onError) {
 delete window.galleryAudioSource;
 let context,source,analyser,frame,closed=false,queued=false;
 const bytes=new Uint8Array(1024);
 const play=()=>{try{source?.stop();if(!app.audio_play())queued=true;}catch(e){onError(e);}};
 const ready=async()=>{
  if(closed)return;
  try {
   const wave=app.audio_result();if(!wave.length||closed)return;
   if(queued){queued=false;play();return;}
   source?.stop();await context?.close();if(closed)return;
   context=new AudioContext({sampleRate:window.galleryAudioRate});
   const buffer=context.createBuffer(1,wave.length,window.galleryAudioRate);buffer.copyToChannel(wave,0);
   source=context.createBufferSource();source.buffer=buffer;source.connect(context.destination);
   analyser=context.createAnalyser();analyser.fftSize=2048;source.connect(analyser);source.start();
   window.galleryAudioPlays=(window.galleryAudioPlays||0)+1;
  }catch(e){onError(e);}
 };
 const tick=()=>{if(closed)return;if(analyser){analyser.getByteFrequencyData(bytes);try{app.audio_spectrum(bytes);}catch(e){onError(e);return;}}frame=requestAnimationFrame(tick);};
 document.addEventListener('click',play);addEventListener('gallery-audio-ready',ready);
 addEventListener('pagehide',()=>{closed=true;cancelAnimationFrame(frame);document.removeEventListener('click',play);removeEventListener('gallery-audio-ready',ready);source?.stop();context?.close();},{once:true});
 play();frame=requestAnimationFrame(tick);
}
// webaudio_timing: the Play overlay, then three's AudioListener and one HRTF
// PositionalAudio per ball. Rust detects the bounces and places the listener and
// sources; each frame's audio_frame() drives the graph as three's
// updateMatrixWorld and Audio.play() do.
export function installPositionalAudio(app,assets,onError) {
 const overlay=document.createElement('div');overlay.id='overlay';
 overlay.style.cssText='position:absolute;font-size:16px;z-index:2;top:0;left:0;width:100%;height:100%;display:flex;align-items:center;justify-content:center;flex-direction:column;background:rgba(0,0,0,0.7)';
 const button=document.createElement('button');button.id='startButton';button.textContent='Play';
 button.style.cssText='background:transparent;border:1px solid rgb(255,255,255);border-radius:4px;color:#ffffff;padding:12px 18px;text-transform:uppercase;cursor:pointer';
 overlay.append(button);document.body.append(overlay);
 button.addEventListener('click',async()=>{
  overlay.remove();
  try {
   const context=new AudioContext();
   // AudioListener: its gain feeds the destination.
   const input=context.createGain();input.connect(context.destination);
   const encoded=await fetch(new URL(`${assets}/ping_pong.mp3`,import.meta.url)).then(r=>{if(!r.ok)throw Error(`Audio: ${r.status}`);return r.arrayBuffer();});
   const buffer=await context.decodeAudioData(encoded);
   const sources=[];
   for(let i=0;i<5;i++){const gain=context.createGain();gain.connect(input);const panner=context.createPanner();panner.panningModel='HRTF';panner.connect(gain);sources.push({panner,playing:false});}
   let last=performance.now();
   const ramp=(params,values,end)=>params.forEach((p,i)=>p.linearRampToValueAtTime(values[i],end));
   const tick=()=>{
    try {
     const data=app.audio_frame();const now=performance.now();const delta=(now-last)/1000;last=now;
     const end=context.currentTime+delta;const l=context.listener;
     sources.forEach((s,i)=>{const o=9+i*7;
      // Audio.play(): ignored while the previous sound still plays.
      if(data[o+6]>0&&!s.playing){const node=context.createBufferSource();node.buffer=buffer;node.onended=()=>{s.playing=false;};node.connect(s.panner);node.start(context.currentTime,0);s.playing=true;}
      if(s.playing){ramp([s.panner.positionX,s.panner.positionY,s.panner.positionZ],data.slice(o,o+3),end);ramp([s.panner.orientationX,s.panner.orientationY,s.panner.orientationZ],data.slice(o+3,o+6),end);}
     });
     ramp([l.positionX,l.positionY,l.positionZ,l.forwardX,l.forwardY,l.forwardZ,l.upX,l.upY,l.upZ],data.slice(0,9),end);
    }catch(e){onError(e);return;}
    requestAnimationFrame(tick);
   };
   // The balls appear once the buffer is decoded, as the loader callback adds them.
   app.tsl_parameter(0,1);window.galleryAudioStarted=true;requestAnimationFrame(tick);
  }catch(e){onError(e);}
 },{once:true});
}
