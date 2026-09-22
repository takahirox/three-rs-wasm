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
