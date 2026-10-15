//! webgpu_video_frame: sintel.mp4 demuxed as MP4Demuxer does with MP4Box
//! ( the first video track's avcC description and codec string, its samples
//! from the stts, ctts, stss, stsz, stsc and stco / co64 tables, as
//! EncodedVideoChunks with microsecond timestamps ) into a WebCodecs
//! VideoDecoder, each decoded VideoFrame closing the previous one and copied
//! into the plane's sRGB texture as VideoFrameTexture uploads it. Once every
//! chunk is decoded the decoder flushes and closes, the last frame is closed
//! and the plane leaves the scene. The page streams the file into MP4Box as
//! it downloads; the port demuxes it once fetched.
use super::gltf_viewer::fetch;
use crate::tsl::surface::SurfaceNodes;
use crate::tsl::uv;
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use js_sys::{Array, Function, Object, Reflect, Uint8Array};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

const ASSET: &str = "/web/gallery/assets/exporters-video/sintel.mp4";

fn be32(d: &[u8], o: usize) -> Result<u32> {
    d.get(o..o + 4)
        .map(|b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
        .ok_or(Error::Asset("mp4 truncated".into()))
}
fn be64(d: &[u8], o: usize) -> Result<u64> {
    Ok((u64::from(be32(d, o)?) << 32) | u64::from(be32(d, o + 4)?))
}
/// The boxes between `start` and `end`: ( type, content start, end ).
fn boxes(d: &[u8], start: usize, end: usize) -> Result<Vec<([u8; 4], usize, usize)>> {
    let mut out = vec![];
    let mut o = start;
    while o + 8 <= end {
        let mut size = be32(d, o)? as usize;
        let mut header = 8;
        if size == 1 {
            size = be64(d, o + 8)? as usize;
            header = 16;
        } else if size == 0 {
            size = end - o;
        }
        if size < header || o + size > end {
            return Err(Error::Asset("mp4 box size".into()));
        }
        let kind = [d[o + 4], d[o + 5], d[o + 6], d[o + 7]];
        out.push((kind, o + header, o + size));
        o += size;
    }
    Ok(out)
}
fn child(d: &[u8], start: usize, end: usize, kind: &[u8; 4]) -> Result<Option<(usize, usize)>> {
    Ok(boxes(d, start, end)?
        .into_iter()
        .find(|(k, ..)| k == kind)
        .map(|(_, s, e)| (s, e)))
}
fn require(found: Option<(usize, usize)>, what: &str) -> Result<(usize, usize)> {
    found.ok_or_else(|| Error::Asset(format!("mp4 {what}")))
}

struct Sample {
    offset: usize,
    size: usize,
    cts: i64,
    duration: u32,
    sync: bool,
}
struct Track {
    codec: String,
    width: u32,
    height: u32,
    timescale: u32,
    description: Vec<u8>,
    samples: Vec<Sample>,
}

/// MP4Box's view of the first video track.
fn demux(d: &[u8]) -> Result<Track> {
    let (ms, me) = require(child(d, 0, d.len(), b"moov")?, "moov")?;
    for (kind, ts, te) in boxes(d, ms, me)? {
        if &kind != b"trak" {
            continue;
        }
        let (ds, de) = require(child(d, ts, te, b"mdia")?, "mdia")?;
        let (hs, _) = require(child(d, ds, de, b"hdlr")?, "hdlr")?;
        if d.get(hs + 8..hs + 12) != Some(b"vide") {
            continue;
        }
        let (mhs, _) = require(child(d, ds, de, b"mdhd")?, "mdhd")?;
        let timescale = if d[mhs] == 1 {
            be32(d, mhs + 20)?
        } else {
            be32(d, mhs + 12)?
        };
        let (is, ie) = require(child(d, ds, de, b"minf")?, "minf")?;
        let (ss, se) = require(child(d, is, ie, b"stbl")?, "stbl")?;
        // stsd: the first sample entry, a visual entry with its avcC.
        let (sds, sde) = require(child(d, ss, se, b"stsd")?, "stsd")?;
        let entry = boxes(d, sds + 8, sde)?
            .into_iter()
            .next()
            .ok_or(Error::Asset("mp4 sample entry".into()))?;
        let (entry_kind, es, ee) = entry;
        let width = u32::from(u16::from_be_bytes([d[es + 24], d[es + 25]]));
        let height = u32::from(u16::from_be_bytes([d[es + 26], d[es + 27]]));
        let mut description = vec![];
        let mut codec = String::from_utf8_lossy(&entry_kind).to_string();
        for (k, cs, ce) in boxes(d, es + 78, ee)? {
            if matches!(&k, b"avcC" | b"hvcC" | b"vpcC" | b"av1C") {
                description = d[cs..ce].to_vec();
                if &k == b"avcC" && ce - cs >= 4 {
                    codec = format!(
                        "{codec}.{:02x}{:02x}{:02x}",
                        d[cs + 1],
                        d[cs + 2],
                        d[cs + 3]
                    );
                }
            }
        }
        if codec.starts_with("vp08") {
            codec = "vp8".into();
        }
        // stts: decode deltas.
        let (s, _) = require(child(d, ss, se, b"stts")?, "stts")?;
        let mut deltas = vec![];
        for i in 0..be32(d, s + 4)? as usize {
            let (count, delta) = (be32(d, s + 8 + i * 8)?, be32(d, s + 12 + i * 8)?);
            deltas.extend(std::iter::repeat_n(delta, count as usize));
        }
        // ctts: composition offsets ( signed in version 1 ).
        let mut offsets = vec![0i64; deltas.len()];
        if let Some((s, _)) = child(d, ss, se, b"ctts")? {
            let signed = d[s] == 1;
            let mut k = 0;
            for i in 0..be32(d, s + 4)? as usize {
                let count = be32(d, s + 8 + i * 8)?;
                let raw = be32(d, s + 12 + i * 8)?;
                let offset = if signed {
                    i64::from(raw as i32)
                } else {
                    i64::from(raw)
                };
                for _ in 0..count {
                    if let Some(o) = offsets.get_mut(k) {
                        *o = offset;
                    }
                    k += 1;
                }
            }
        }
        // stss: sync samples ( every sample when absent ).
        let sync: Option<Vec<u32>> = match child(d, ss, se, b"stss")? {
            Some((s, _)) => Some(
                (0..be32(d, s + 4)? as usize)
                    .map(|i| be32(d, s + 8 + i * 4))
                    .collect::<Result<_>>()?,
            ),
            None => None,
        };
        let (s, _) = require(child(d, ss, se, b"stsz")?, "stsz")?;
        let (uniform, count) = (be32(d, s + 4)?, be32(d, s + 8)? as usize);
        let sizes: Vec<u32> = if uniform != 0 {
            vec![uniform; count]
        } else {
            (0..count)
                .map(|i| be32(d, s + 12 + i * 4))
                .collect::<Result<_>>()?
        };
        let (s, _) = require(child(d, ss, se, b"stsc")?, "stsc")?;
        let stsc: Vec<(u32, u32)> = (0..be32(d, s + 4)? as usize)
            .map(|i| Ok((be32(d, s + 8 + i * 12)?, be32(d, s + 12 + i * 12)?)))
            .collect::<Result<_>>()?;
        let chunks: Vec<u64> = if let Some((s, _)) = child(d, ss, se, b"stco")? {
            (0..be32(d, s + 4)? as usize)
                .map(|i| be32(d, s + 8 + i * 4).map(u64::from))
                .collect::<Result<_>>()?
        } else {
            let (s, _) = require(child(d, ss, se, b"co64")?, "co64")?;
            (0..be32(d, s + 4)? as usize)
                .map(|i| be64(d, s + 8 + i * 8))
                .collect::<Result<_>>()?
        };
        let mut samples = Vec::with_capacity(count);
        let mut dts = 0i64;
        let mut n = 0;
        for (c, &chunk_offset) in chunks.iter().enumerate() {
            let chunk = c as u32 + 1;
            let per_chunk = stsc
                .iter()
                .rev()
                .find(|(first, _)| *first <= chunk)
                .map_or(0, |e| e.1);
            let mut offset = chunk_offset as usize;
            for _ in 0..per_chunk {
                if n >= count {
                    break;
                }
                let delta = deltas.get(n).copied().unwrap_or(0);
                samples.push(Sample {
                    offset,
                    size: sizes[n] as usize,
                    cts: dts + offsets.get(n).copied().unwrap_or(0),
                    duration: delta,
                    sync: sync
                        .as_ref()
                        .is_none_or(|s| s.binary_search(&(n as u32 + 1)).is_ok()),
                });
                offset += sizes[n] as usize;
                dts += i64::from(delta);
                n += 1;
            }
        }
        return Ok(Track {
            codec,
            width,
            height,
            timescale,
            description,
            samples,
        });
    }
    Err(Error::Asset("mp4 video track".into()))
}

#[derive(Default)]
struct Shared {
    frame: Option<JsValue>,
    fresh: bool,
    done: bool,
}
fn js(e: JsValue) -> Error {
    Error::Asset(format!("{e:?}"))
}
fn call(target: &JsValue, method: &str, args: &[&JsValue]) -> Result<JsValue> {
    let f: Function = Reflect::get(target, &method.into())
        .map_err(js)?
        .dyn_into()
        .map_err(js)?;
    let array = Array::new();
    for a in args {
        array.push(a);
    }
    f.apply(target, &array).map_err(js)
}
fn object(entries: &[(&str, JsValue)]) -> Result<Object> {
    let o = Object::new();
    for (k, v) in entries {
        Reflect::set(&o, &(*k).into(), v).map_err(js)?;
    }
    Ok(o)
}
fn construct(name: &str, init: &Object) -> Result<JsValue> {
    let window = web_sys::window().ok_or(Error::Invalid("window"))?;
    let ctor: Function = Reflect::get(&window, &name.into())
        .map_err(js)?
        .dyn_into()
        .map_err(js)?;
    Reflect::construct(&ctor, &Array::of1(init)).map_err(js)
}

pub(super) struct Demo {
    shared: Rc<RefCell<Shared>>,
    texture: wgpu::Texture,
    size: wgpu::Extent3d,
    mesh: Option<Object3D>,
}
impl Demo {
    pub async fn create(s: &mut Scene, c: Object3D, _id: u32, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 75.,
            near: 0.25,
            far: 10.,
            aspect,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 0., 1.);
        s.background = Color::BLACK;
        let bytes = fetch(ASSET).await?;
        let track = demux(&bytes)?;
        let size = wgpu::Extent3d {
            width: track.width,
            height: track.height,
            depth_or_array_layers: 1,
        };
        let texture = r.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("video frame texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());
        // VideoTexture: linear filtering without mipmaps, clamped.
        let sampler = r.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let program = SurfaceNodes {
            color: Some(crate::tsl::Texture::External(0).sample(uv()).rgb()),
            ..Default::default()
        }
        .build(r, &[], &[(&view, &sampler)])
        .await?;
        let mut material = MeshBasicMaterial::default();
        material.properties.vertex_program = Some(Arc::new(program));
        let mesh = s.insert(NodeKind::Mesh(Mesh {
            geometry: Arc::new(PlaneGeometry::build(1., 1., 1, 1)?),
            materials: vec![Arc::new(Material::Basic(material))],
        }));
        // VideoDecoder: each output closes the previous frame.
        let shared = Rc::new(RefCell::new(Shared::default()));
        let output = {
            let shared = shared.clone();
            Closure::<dyn FnMut(JsValue)>::new(move |frame: JsValue| {
                let mut state = shared.borrow_mut();
                if let Some(previous) = state.frame.take() {
                    let _ = call(&previous, "close", &[]);
                }
                state.frame = Some(frame);
                state.fresh = true;
            })
        };
        let error = Closure::<dyn FnMut(JsValue)>::new(|e: JsValue| {
            web_sys::console::error_2(&"VideoDecoder:".into(), &e);
        });
        let decoder = construct(
            "VideoDecoder",
            &object(&[
                ("output", output.as_ref().clone()),
                ("error", error.as_ref().clone()),
            ])?,
        )?;
        output.forget();
        error.forget();
        let config = object(&[
            ("codec", track.codec.clone().into()),
            ("codedHeight", track.height.into()),
            ("codedWidth", track.width.into()),
            (
                "description",
                Uint8Array::from(track.description.as_slice()).into(),
            ),
        ])?;
        call(&decoder, "configure", &[&config.into()])?;
        for sample in &track.samples {
            let data = bytes
                .get(sample.offset..sample.offset + sample.size)
                .ok_or(Error::Asset("mp4 sample".into()))?;
            let chunk = construct(
                "EncodedVideoChunk",
                &object(&[
                    ("type", if sample.sync { "key" } else { "delta" }.into()),
                    (
                        "timestamp",
                        (1e6 * sample.cts as f64 / f64::from(track.timescale)).into(),
                    ),
                    (
                        "duration",
                        (1e6 * f64::from(sample.duration) / f64::from(track.timescale)).into(),
                    ),
                    ("data", Uint8Array::from(data).into()),
                ])?,
            )?;
            call(&decoder, "decode", &[&chunk])?;
        }
        // The fetch is done: flush, then close the decoder and drop the plane.
        let flushed: js_sys::Promise = call(&decoder, "flush", &[])?.dyn_into().map_err(js)?;
        {
            let shared = shared.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if wasm_bindgen_futures::JsFuture::from(flushed).await.is_ok() {
                    let _ = call(&decoder, "close", &[]);
                }
                shared.borrow_mut().done = true;
            });
        }
        Ok(Self {
            shared,
            texture,
            size,
            mesh: Some(mesh),
        })
    }
    pub fn update(&mut self, _s: &mut Scene, _c: Object3D, _dt: f64, _animate: bool) -> Result<()> {
        Ok(())
    }
    pub fn prepare(&mut self, s: &mut Scene, _c: Object3D, r: &Renderer) -> Result<()> {
        let mut state = self.shared.borrow_mut();
        if state.done {
            if let Some(frame) = state.frame.take() {
                let _ = call(&frame, "close", &[]);
            }
            if let Some(mesh) = self.mesh.take() {
                s.remove_from_parent(mesh)?;
                s.get_mut(mesh)?.visible = false;
                // The page's end state, observable as the gallery's status.
                if let Some(body) = web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.body())
                {
                    let _ = body.set_attribute("data-video-done", "true");
                }
            }
            return Ok(());
        }
        if std::mem::take(&mut state.fresh)
            && let Some(frame) = &state.frame
        {
            // A VideoFrame is a GPUCopyExternalImageSource; web-sys exposes the
            // type only under its unstable APIs, so it passes as an ImageBitmap.
            r.queue.copy_external_image_to_texture(
                &wgpu::CopyExternalImageSourceInfo {
                    source: wgpu::ExternalImageSource::ImageBitmap(frame.clone().unchecked_into()),
                    origin: wgpu::Origin2d::ZERO,
                    flip_y: true,
                },
                wgpu::CopyExternalImageDestInfo {
                    texture: &self.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                    color_space: wgpu::PredefinedColorSpace::Srgb,
                    premultiplied_alpha: false,
                },
                self.size,
            );
        }
        Ok(())
    }
    pub fn draw(&mut self, _kind: u32, _x: f64, _y: f64) {}
    pub fn key(&mut self, _code: u32, _down: bool) {}
    #[allow(clippy::too_many_arguments)]
    pub fn input(
        &mut self,
        _s: &mut Scene,
        _c: Object3D,
        _dx: f64,
        _dy: f64,
        _wheel: f64,
        _pan: bool,
        _height: f64,
    ) -> Result<()> {
        Ok(())
    }
    pub fn parameter(&mut self, _index: usize, _value: f32) -> Result<()> {
        Ok(())
    }
    pub fn seek(&mut self, _t: f64) {}
}
