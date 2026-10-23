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
// webaudio_visualizer and webaudio_orientation: the Play overlay, then the
// looping song as a media element source. The visualizer feeds an AudioAnalyser
// (fftSize 128) and sends its bins each frame; the orientation scene routes the
// song through the BoomBox's directional HRTF panner, placed from Rust each frame.
export function installMediaAudio(app,assets,positional,onError) {
 const overlay=document.createElement('div');overlay.id='overlay';
 overlay.style.cssText='position:absolute;font-size:16px;z-index:2;top:0;left:0;width:100%;height:100%;display:flex;align-items:center;justify-content:center;flex-direction:column;background:rgba(0,0,0,0.7)';
 const button=document.createElement('button');button.id='startButton';button.textContent='Play';
 button.style.cssText='background:transparent;border:1px solid rgb(255,255,255);border-radius:4px;color:#ffffff;padding:12px 18px;text-transform:uppercase;cursor:pointer';
 overlay.append(button);document.body.append(overlay);
 button.addEventListener('click',()=>{
  overlay.remove();
  try {
   const context=new AudioContext();
   const input=context.createGain();input.connect(context.destination);
   const media=new Audio(new URL(`${assets}/376737_Skullbeatz___Bad_Cat_Maste.mp3`,import.meta.url).href);media.loop=positional;
   // A page without user activation cannot start playback; the scene still runs.
   media.play().catch(()=>{});
   const source=context.createMediaElementSource(media);const gain=context.createGain();gain.connect(input);
   let panner=null,analyser=null;const bytes=new Uint8Array(64);
   if(positional){panner=context.createPanner();panner.panningModel='HRTF';panner.refDistance=1;panner.coneInnerAngle=180;panner.coneOuterAngle=230;panner.coneOuterGain=0.1;panner.connect(gain);source.connect(panner);}
   else{source.connect(gain);analyser=context.createAnalyser();analyser.fftSize=128;gain.connect(analyser);}
   let last=performance.now();
   const ramp=(params,values,end)=>params.forEach((p,i)=>p.linearRampToValueAtTime(values[i],end));
   const tick=()=>{
    try {
     if(analyser){analyser.getByteFrequencyData(bytes);app.audio_data(bytes);}
     else{const data=app.audio_frame();const now=performance.now();const end=context.currentTime+(now-last)/1000;last=now;const l=context.listener;
      ramp([l.positionX,l.positionY,l.positionZ,l.forwardX,l.forwardY,l.forwardZ,l.upX,l.upY,l.upZ],data.slice(0,9),end);
      ramp([panner.positionX,panner.positionY,panner.positionZ,panner.orientationX,panner.orientationY,panner.orientationZ],data.slice(9,15),end);}
    }catch(e){onError(e);return;}
    requestAnimationFrame(tick);
   };
   app.tsl_parameter(0,1);window.galleryAudioStarted=true;requestAnimationFrame(tick);
  }catch(e){onError(e);}
 },{once:true});
}
// webaudio_sandbox: two songs and a 144 Hz sine oscillator on the spheres'
// HRTF panners (refDistance 20), the ambient loop, and three 32-point analysers
// whose bins return to Rust for the spheres' emissive blue. Rust places the
// listener and sources and holds the GUI's volumes and generator settings.
export function installSandboxAudio(app,assets,onError) {
 const overlay=document.createElement('div');overlay.id='overlay';
 overlay.style.cssText='position:absolute;font-size:16px;z-index:2;top:0;left:0;width:100%;height:100%;display:flex;align-items:center;justify-content:center;flex-direction:column;background:rgba(0,0,0,0.7)';
 const button=document.createElement('button');button.id='startButton';button.textContent='Play';
 button.style.cssText='background:transparent;border:1px solid rgb(255,255,255);border-radius:4px;color:#ffffff;padding:12px 18px;text-transform:uppercase;cursor:pointer';
 overlay.append(button);document.body.append(overlay);
 button.addEventListener('click',()=>{
  overlay.remove();
  try {
   const context=new AudioContext();
   const master=context.createGain();master.connect(context.destination);
   const media=(name,loop)=>{const m=new Audio(new URL(`${assets}/${name}`,import.meta.url).href);m.loop=loop;m.play().catch(()=>{});return context.createMediaElementSource(m);};
   const oscillator=context.createOscillator();oscillator.type='sine';oscillator.frequency.setValueAtTime(144,context.currentTime);oscillator.start(0);
   const sources=[media('358232_j_s_song.mp3',false),media('376737_Skullbeatz___Bad_Cat_Maste.mp3',false),oscillator];
   const spheres=sources.map(source=>{const gain=context.createGain();gain.connect(master);const panner=context.createPanner();panner.panningModel='HRTF';panner.refDistance=20;panner.connect(gain);source.connect(panner);const analyser=context.createAnalyser();analyser.fftSize=32;panner.connect(analyser);return {gain,panner,analyser};});
   const ambient=context.createGain();ambient.connect(master);media('Project_Utopia.mp3',true).connect(ambient);
   const bytes=new Uint8Array(48);const bins=new Uint8Array(16);let last=performance.now();const wave=['sine','square','sawtooth','triangle'];
   const ramp=(params,values,end)=>params.forEach((p,i)=>p.linearRampToValueAtTime(values[i],end));
   const volume=(param,value)=>{if(param.value!==value)param.setTargetAtTime(value,context.currentTime,0.01);};
   let frequency=144;
   const tick=()=>{
    try {
     spheres.forEach((s,i)=>{s.analyser.getByteFrequencyData(bins);bytes.set(bins,i*16);});app.audio_data(bytes);
     const data=app.audio_frame();const now=performance.now();const end=context.currentTime+(now-last)/1000;last=now;const l=context.listener;
     ramp([l.positionX,l.positionY,l.positionZ,l.forwardX,l.forwardY,l.forwardZ,l.upX,l.upY,l.upZ],data.slice(0,9),end);
     spheres.forEach((s,i)=>{const o=9+i*7;ramp([s.panner.positionX,s.panner.positionY,s.panner.positionZ,s.panner.orientationX,s.panner.orientationY,s.panner.orientationZ],data.slice(o,o+6),end);});
     const g=data.slice(30,37);volume(master.gain,g[0]);spheres.forEach((s,i)=>volume(s.gain.gain,g[1+i]));volume(ambient.gain,g[4]);
     if(g[5]!==frequency){frequency=g[5];oscillator.frequency.setValueAtTime(frequency,context.currentTime);}
     oscillator.type=wave[Math.round(g[6])]??'sine';
    }catch(e){onError(e);return;}
    requestAnimationFrame(tick);
   };
   app.tsl_parameter(7,1);window.galleryAudioStarted=true;requestAnimationFrame(tick);
  }catch(e){onError(e);}
 },{once:true});
}
