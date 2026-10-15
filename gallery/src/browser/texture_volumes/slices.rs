//! webgl_loader_nrrd: Volume.extractSlice's three VolumeSlices. Each repaint
//! windows and thresholds its slice on the CPU into a 2D canvas, scales it
//! into the plane-sized canvas and copies that into the slice's texture, as
//! the original repaints; nothing is uploaded between repaints.
use super::super::controls_attributes::viewport_css;
use super::super::trackball_sprites::{Mode, Trackball};
use super::Nrrd;
use crate::texture_gpu::GpuTexture;
use crate::{Error, Result, camera::*, geometry::*, material::*, math::*, renderer::*, scene::*};
use std::f64::consts::PI;
use std::sync::Arc;
use wasm_bindgen::{Clamped, JsCast};

/// Volume: the samples, their IJK-to-RAS matrix and the display settings.
struct Volume {
    lengths: [usize; 3],
    data: Vec<f32>,
    spacing: [f64; 3],
    /// axisOrder.indexOf( 'x' | 'y' | 'z' ).
    order: [usize; 3],
    inverse: Matrix4,
    ras: [usize; 3],
    /// lowerThreshold, upperThreshold, windowLow, windowHigh.
    settings: [f64; 4],
}
/// extractPerpendicularPlane's result: the slice size, access and placement.
struct Plane {
    i_length: usize,
    j_length: usize,
    width: f64,
    height: f64,
    matrix: Matrix4,
    /// Per volume axis: the plane direction (0 i, 1 j, 2 the slice axis) and
    /// whether the index runs backwards.
    access: [(usize, bool); 3],
    ijk_index: usize,
}
impl Volume {
    fn new(n: Nrrd) -> Self {
        let v = n
            .vectors
            .unwrap_or([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        let find = |axis: usize| v.iter().position(|d| d[axis] != 0.);
        let (x, y, z) = (find(0), find(1), find(2));
        let order = match (x, y, z) {
            (Some(x), Some(y), Some(z)) if x != y && x != z && y != z => {
                let mut o = [0; 3];
                o[0] = x;
                o[1] = y;
                o[2] = z;
                o
            }
            _ => [0, 1, 2],
        };
        let spacing = v.map(|d| Vector3::from_array(d).length());
        let transition = match n.space.as_str() {
            "left-posterior-superior" => Matrix4::from_diagonal(Vector4::new(-1., -1., 1., 1.)),
            "left-anterior-superior" => Matrix4::from_diagonal(Vector4::new(1., 1., -1., 1.)),
            _ => Matrix4::IDENTITY,
        };
        let matrix = if n.vectors.is_some() {
            Matrix4::from_cols(
                Vector3::from_array(v[0]).extend(0.),
                Vector3::from_array(v[1]).extend(0.),
                Vector3::from_array(v[2]).extend(0.),
                Vector4::W,
            ) * transition
        } else {
            Matrix4::IDENTITY
        };
        let (mut min, mut max) = (f64::INFINITY, f64::NEG_INFINITY);
        for &value in &n.data {
            if !value.is_nan() {
                min = min.min(value as f64);
                max = max.max(value as f64);
            }
        }
        let lengths = n.sizes;
        Self {
            ras: [0, 1, 2].map(|a| (lengths[a] as f64 * spacing[a]).floor() as usize),
            lengths,
            data: n.data,
            spacing,
            order,
            inverse: matrix.inverse(),
            settings: [min, max, min, max],
        }
    }
    /// The IJK-to-RAS matrix, recomputed for the box helper.
    fn matrix(&self) -> Matrix4 {
        self.inverse.inverse()
    }
    fn extract(&self, axis: usize, ras_index: usize) -> Plane {
        let r = ras_index as f64;
        let offset = (self.ras[axis] as f64 - 1.) / 2.;
        let (axis_ijk, first, second, spacing, ijk, matrix) = match axis {
            0 => (
                Vector3::X,
                Vector3::NEG_Z,
                Vector3::NEG_Y,
                (self.order[2], self.order[1]),
                Vector3::new(r, 0., 0.),
                Matrix4::from_translation(Vector3::new(r - offset, 0., 0.))
                    * Matrix4::from_rotation_y(PI / 2.),
            ),
            1 => (
                Vector3::Y,
                Vector3::X,
                Vector3::Z,
                (self.order[0], self.order[2]),
                Vector3::new(0., r, 0.),
                Matrix4::from_translation(Vector3::new(0., r - offset, 0.))
                    * Matrix4::from_rotation_x(-PI / 2.),
            ),
            _ => (
                Vector3::Z,
                Vector3::X,
                Vector3::NEG_Y,
                (self.order[0], self.order[1]),
                Vector3::new(0., 0., r),
                Matrix4::from_translation(Vector3::new(0., 0., r - offset)),
            ),
        };
        let to_ijk = |d: Vector3| self.inverse.transform_point3(d).normalize();
        let dirs = [to_ijk(first), to_ijk(second), to_ijk(axis_ijk)];
        let lengths = Vector3::new(
            self.lengths[0] as f64,
            self.lengths[1] as f64,
            self.lengths[2] as f64,
        );
        let i_length = dirs[0].dot(lengths).abs().floor() as usize;
        let j_length = dirs[1].dot(lengths).abs().floor() as usize;
        let ijk_index = (self.inverse.transform_point3(ijk).dot(dirs[2]) + 0.5)
            .floor()
            .abs() as usize;
        let base = [Vector3::X, Vector3::Y, Vector3::Z];
        let access = base.map(|b| {
            let d = dirs.iter().position(|d| d.dot(b).abs() > 0.9).unwrap_or(2);
            (d, dirs[d].dot(b) <= 0.)
        });
        Plane {
            i_length,
            j_length,
            width: (i_length as f64 * self.spacing[spacing.0]).abs(),
            height: (j_length as f64 * self.spacing[spacing.1]).abs(),
            matrix,
            access,
            ijk_index,
        }
    }
}
fn canvas_2d(
    width: u32,
    height: u32,
) -> Result<(
    web_sys::HtmlCanvasElement,
    web_sys::CanvasRenderingContext2d,
)> {
    let canvas: web_sys::HtmlCanvasElement = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.create_element("canvas").ok())
        .and_then(|c| c.dyn_into().ok())
        .ok_or(Error::Invalid("canvas"))?;
    canvas.set_width(width);
    canvas.set_height(height);
    let context = canvas
        .get_context("2d")
        .ok()
        .flatten()
        .and_then(|c| c.dyn_into().ok())
        .ok_or(Error::Invalid("2d context"))?;
    Ok((canvas, context))
}
/// VolumeSlice: the plane mesh, its canvases and texture.
struct Slice {
    axis: usize,
    index: usize,
    plane: Plane,
    mesh: Object3D,
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
    buffer: web_sys::HtmlCanvasElement,
    buffer_context: web_sys::CanvasRenderingContext2d,
    texture: GpuTexture,
    geometry_needs_update: bool,
    /// repaint() requested, run in prepare().
    stale: bool,
}
impl Slice {
    /// updateGeometry(): the plane for the current index. Resizing the canvases
    /// clears them; the plane size depends only on the axis, so the resident
    /// PlaneGeometry is kept.
    fn update_geometry(&mut self, s: &mut Scene, volume: &Volume) -> Result<()> {
        self.plane = volume.extract(self.axis, self.index);
        let p = &self.plane;
        self.canvas.set_width(p.width as u32);
        self.canvas.set_height(p.height as u32);
        self.buffer.set_width(p.i_length as u32);
        self.buffer.set_height(p.j_length as u32);
        let n = s.get_mut(self.mesh)?;
        n.matrix = p.matrix;
        n.matrix_world_needs_update = true;
        self.geometry_needs_update = false;
        Ok(())
    }
    /// repaint(): threshold and window the slice, scale it into the canvas and
    /// mark the texture for upload.
    fn repaint(&mut self, s: &mut Scene, r: &Renderer, volume: &Volume) -> Result<()> {
        if self.geometry_needs_update {
            self.update_geometry(s, volume)?;
        }
        let p = &self.plane;
        let [lower, upper, low, high] = volume.settings;
        let (xl, yl, zl) = (volume.lengths[0], volume.lengths[1], volume.lengths[2]);
        let mut pixels = vec![0u8; p.i_length * p.j_length * 4];
        for j in 0..p.j_length {
            for i in 0..p.i_length {
                // access(): JavaScript index arithmetic, where an index past an
                // edge lands in the neighbouring row and outside the data reads
                // undefined, which fails both threshold tests and windows to 0.
                let index = |axis: usize, length: usize| {
                    let (d, backwards) = p.access[axis];
                    let v = [i, j, p.ijk_index][d] as i64;
                    if backwards { length as i64 - 1 - v } else { v }
                };
                let at = index(2, zl) * (xl * yl) as i64 + index(1, yl) * xl as i64 + index(0, xl);
                let value = usize::try_from(at)
                    .ok()
                    .and_then(|at| volume.data.get(at))
                    .map_or(f64::NAN, |&v| v as f64);
                let alpha = if upper >= value && lower <= value {
                    255
                } else {
                    0
                };
                let w = (255. * (value - low) / (high - low)).floor();
                let w = if w > 255. {
                    255
                } else if w < 0. || w.is_nan() {
                    0
                } else {
                    w as u8
                };
                let o = 4 * (j * p.i_length + i);
                pixels[o..o + 4].copy_from_slice(&[w, w, w, alpha]);
            }
        }
        let fail = |_| Error::Invalid("slice canvas");
        let image = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&pixels),
            p.i_length as u32,
            p.j_length as u32,
        )
        .map_err(fail)?;
        self.buffer_context
            .put_image_data(&image, 0., 0.)
            .map_err(fail)?;
        self.context
            .draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &self.buffer,
                0.,
                0.,
                p.i_length as f64,
                p.j_length as f64,
                0.,
                0.,
                self.canvas.width() as f64,
                self.canvas.height() as f64,
            )
            .map_err(fail)?;
        // map.needsUpdate: the canvas replaces the texture. Material maps keep
        // image rows top-down and apply flipY in their UV transform.
        r.queue.copy_external_image_to_texture(
            &wgpu::CopyExternalImageSourceInfo {
                source: wgpu::ExternalImageSource::HTMLCanvasElement(self.canvas.clone()),
                origin: wgpu::Origin2d::ZERO,
                flip_y: false,
            },
            wgpu::CopyExternalImageDestInfo {
                texture: &self.texture.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
                color_space: wgpu::PredefinedColorSpace::Srgb,
                premultiplied_alpha: false,
            },
            wgpu::Extent3d {
                width: self.canvas.width(),
                height: self.canvas.height(),
                depth_or_array_layers: 1,
            },
        );
        self.stale = false;
        Ok(())
    }
}
pub(super) struct Slices {
    volume: Volume,
    slices: Vec<Slice>,
    trackball: Trackball,
    time: f64,
    last: f64,
}
impl Slices {
    pub(super) async fn new(s: &mut Scene, c: Object3D, r: &Renderer, nrrd: Nrrd) -> Result<Self> {
        let (w, h, _) = viewport_css();
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 60.,
            near: 0.01,
            far: 1e10,
            aspect: w / h,
            ..Default::default()
        }));
        let n = s.get_mut(c)?;
        n.position = Vector3::new(0., 0., 300.);
        n.quaternion = Quaternion::IDENTITY;
        s.background = Color::BLACK;
        s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::WHITE,
            ground: Color::BLACK,
            intensity: 3.,
        }));
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 1.5,
            target: Vector3::ZERO,
        }));
        s.get_mut(light)?.position = Vector3::new(200., 200., 200.);
        let volume = Volume::new(nrrd);
        // The hidden extent cube and its BoxHelper, moved by the volume matrix.
        let [xl, yl, zl] = volume.lengths.map(|l| l as f64);
        let mut green = MeshBasicMaterial::default();
        green.properties.color = Color::from_hex(0x00ff00);
        let cube = s.insert(NodeKind::Mesh(Mesh::new(
            Arc::new(BoxGeometry::build(xl, yl, zl)?),
            Arc::new(Material::Basic(green)),
        )));
        s.get_mut(cube)?.visible = false;
        let (x, y, z) = (xl / 2., yl / 2., zl / 2.);
        let corners = [
            [x, y, z],
            [-x, y, z],
            [-x, -y, z],
            [x, -y, z],
            [x, y, -z],
            [-x, y, -z],
            [-x, -y, -z],
            [x, -y, -z],
        ];
        let mut g = BufferGeometry::default();
        g.set_attribute(
            "position",
            Attribute::F32(crate::attribute::BufferAttribute::new(
                corners.iter().flatten().map(|&v| v as f32).collect(),
                3,
                false,
            )?),
        );
        g.set_index(Some(vec![
            0, 1, 1, 2, 2, 3, 3, 0, 4, 5, 5, 6, 6, 7, 7, 4, 0, 4, 1, 5, 2, 6, 3, 7,
        ]));
        let mut yellow = LineBasicMaterial::default();
        yellow.properties.color = Color::from_hex(0xffff00);
        yellow.properties.tone_mapped = false;
        let helper = s.insert(NodeKind::Line(Line {
            geometry: Arc::new(g),
            material: Arc::new(Material::Line(yellow)),
            segments: true,
        }));
        s.get_mut(helper)?.apply_matrix4(volume.matrix());
        // extractSlice( 'z', RAS / 4 ), ( 'y', RAS / 2 ), ( 'x', RAS / 2 ).
        let mut slices = vec![];
        for (axis, index) in [
            (2, volume.ras[2] / 4),
            (1, volume.ras[1] / 2),
            (0, volume.ras[0] / 2),
        ] {
            let plane = volume.extract(axis, index);
            let (canvas, context) = canvas_2d(plane.width as u32, plane.height as u32)?;
            let (buffer, buffer_context) = canvas_2d(plane.i_length as u32, plane.j_length as u32)?;
            let (tw, th) = (canvas.width(), canvas.height());
            let mut image = Texture::from_rgba(tw, th, vec![0; (tw * th * 4) as usize], true)?;
            image.min_filter = Some(Filter::Linear);
            let image = Arc::new(image);
            let texture = r.upload_texture(&image)?;
            let mut m = MeshBasicMaterial::default();
            m.properties.map = Some(image);
            m.properties.side = Side::Double;
            m.properties.transparent = true;
            let mesh = s.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(PlaneGeometry::build(plane.width, plane.height, 1, 1)?),
                Arc::new(Material::Basic(m)),
            )));
            s.get_mut(mesh)?.matrix_auto_update = false;
            slices.push(Slice {
                axis,
                index,
                plane,
                mesh,
                canvas,
                context,
                buffer,
                buffer_context,
                texture,
                geometry_needs_update: true,
                stale: true,
            });
        }
        let mut trackball = Trackball::new(s, c, Vector2::new(w, h))?.with_pan_speed(2.);
        trackball.rotate_speed = 5.;
        trackball.zoom_speed = 5.;
        trackball.distance = (100., 500.);
        Ok(Self {
            volume,
            slices,
            trackball,
            time: 0.,
            last: 0.,
        })
    }
    pub(super) fn update(&mut self, dt: f64, animate: bool) {
        if animate {
            self.time += dt;
        }
    }
    pub(super) fn seek(&mut self, t: f64) {
        self.time = t;
    }
    /// The pending repaints, then controls.update() per 60 fps step.
    pub(super) fn prepare(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        for slice in &mut self.slices {
            if slice.stale {
                slice.repaint(s, r, &self.volume)?;
            }
        }
        let steps = ((self.time - self.last) * 60.).round().max(0.) as usize;
        self.last = self.time;
        let (w, h, _) = viewport_css();
        self.trackball.screen = Vector2::new(w, h);
        for _ in 0..steps {
            self.trackball.update(s)?;
        }
        Ok(())
    }
    pub(super) fn draw(&mut self, kind: u32, x: f64, y: f64) {
        let t = &mut self.trackball;
        match kind {
            10..=19 => t.down(kind - 10, x, y),
            20..=29 => t.state = Mode::None,
            _ => t.moved(x, y),
        }
    }
    pub(super) fn wheel(&mut self, wheel: f64) {
        self.trackball.zoom_start.y -= wheel * 0.00025;
    }
    pub(super) fn key(&mut self, code: u32, down: bool) {
        let t = &mut self.trackball;
        if !down {
            t.key_state = Mode::None;
        } else if t.key_state == Mode::None {
            t.key_state = match code {
                65 => Mode::Rotate,
                83 => Mode::Zoom,
                68 => Mode::Pan,
                _ => Mode::None,
            };
        }
    }
    /// indexX, indexY, indexZ repaint their slice; the thresholds and window
    /// repaint all slices (repaintAllSlices).
    pub(super) fn parameter(&mut self, index: usize, value: f64) -> Result<()> {
        match index {
            0..=2 => {
                // The GUI rows are X, Y, Z; the slices were extracted Z, Y, X.
                let slice = &mut self.slices[2 - index];
                slice.index = value as usize;
                slice.geometry_needs_update = true;
                slice.stale = true;
            }
            3..=6 => {
                self.volume.settings[index - 3] = value;
                for slice in &mut self.slices {
                    // The threshold setters mark every slice's geometry, so
                    // their repaint clears the canvas; the window does not.
                    slice.geometry_needs_update |= index < 5;
                    slice.stale = true;
                }
            }
            _ => return Err(Error::Invalid("nrrd parameter")),
        }
        Ok(())
    }
}
