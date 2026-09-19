//! r186 color-only SSAAPass: jittered GPU scene draws with weighted accumulation.
use super::Effect;
use crate::{
    Error, Result,
    camera::{Camera, ViewOffset},
    renderer::*,
    scene::{NodeKind, Object3D, Scene},
    tsl::*,
};
pub struct SsaaPass {
    sample: RenderTarget,
    accumulation: Effect,
    pub sample_level: u32,
    pub unbiased: bool,
}
impl SsaaPass {
    pub async fn new(renderer: &Renderer) -> Result<Self> {
        let format = wgpu::TextureFormat::Rgba16Float;
        let sample = RenderTarget::with_options(
            &renderer.device,
            1,
            1,
            RenderTargetOptions {
                format,
                ..Default::default()
            },
        )?;
        let component = wgpu::BlendComponent {
            src_factor: wgpu::BlendFactor::One,
            dst_factor: wgpu::BlendFactor::One,
            operation: wgpu::BlendOperation::Add,
        };
        let source = effect_wgsl(&(Texture::Input.sample(uv()) * uniform(0, Type::Float)))?;
        let accumulation = Effect::with_blend(
            renderer,
            format,
            &source,
            wgpu::BlendState {
                color: component,
                alpha: component,
            },
        )
        .await?;
        Ok(Self {
            sample,
            accumulation,
            sample_level: 4,
            unbiased: true,
        })
    }
    pub fn render(
        &mut self,
        renderer: &Renderer,
        scene: &mut Scene,
        camera: Object3D,
        output: &RenderTarget,
    ) -> Result<()> {
        if self.sample_level > 5 {
            return Err(Error::Invalid("SSAA sample level"));
        }
        self.sample
            .set_size(&renderer.device, output.width, output.height)?;
        let original = match &scene.get(camera)?.kind {
            NodeKind::Camera(Camera::Perspective(c)) => c.view,
            _ => return Err(Error::Invalid("SSAA requires perspective camera")),
        };
        let view = original.unwrap_or(ViewOffset {
            full_width: output.width as f64,
            full_height: output.height as f64,
            offset_x: 0.0,
            offset_y: 0.0,
            width: output.width as f64,
            height: output.height as f64,
        });
        let offsets = JITTER[self.sample_level as usize];
        let result = (|| {
            for (i, [x, y]) in offsets.iter().enumerate() {
                if let NodeKind::Camera(Camera::Perspective(c)) = &mut scene.get_mut(camera)?.kind {
                    c.view = Some(ViewOffset {
                        offset_x: view.offset_x + *x as f64 / 16.0,
                        offset_y: view.offset_y + *y as f64 / 16.0,
                        ..view
                    });
                }
                renderer.render(scene, camera, &self.sample)?;
                self.accumulation.parameters[0][0] = (1.0 / offsets.len() as f64
                    + if self.unbiased {
                        (-0.5 + (i as f64 + 0.5) / offsets.len() as f64) / 32.0
                    } else {
                        0.0
                    }) as f32;
                self.accumulation
                    .apply_with_load(renderer, &self.sample, None, output, i > 0)?;
            }
            Ok(())
        })();
        if let NodeKind::Camera(Camera::Perspective(c)) = &mut scene.get_mut(camera)?.kind {
            c.view = original;
        }
        result
    }
}
// Pinned SSAAPassNode's 1/16-pixel sample patterns, MIT.
const JITTER: &[&[[i32; 2]]] = &[
    &[[0, 0]],
    &[[4, 4], [-4, -4]],
    &[[-2, -6], [6, -2], [-6, 2], [2, 6]],
    &[
        [1, -3],
        [-1, 3],
        [5, 1],
        [-3, -5],
        [-5, 5],
        [-7, -1],
        [3, 7],
        [7, -7],
    ],
    &[
        [1, 1],
        [-1, -3],
        [-3, 2],
        [4, -1],
        [-5, -2],
        [2, 5],
        [5, 3],
        [3, -5],
        [-2, 6],
        [0, -7],
        [-4, -6],
        [-6, 4],
        [-8, 0],
        [7, -4],
        [6, 7],
        [-7, -8],
    ],
    &[
        [-4, -7],
        [-7, -5],
        [-3, -5],
        [-5, -4],
        [-1, -4],
        [-2, -2],
        [-6, -1],
        [-4, 0],
        [-7, 1],
        [-1, 2],
        [-6, 3],
        [-3, 3],
        [-7, 6],
        [-3, 6],
        [-5, 7],
        [-1, 7],
        [5, -7],
        [1, -6],
        [6, -5],
        [4, -4],
        [2, -3],
        [7, -2],
        [1, -1],
        [4, -1],
        [2, 1],
        [6, 2],
        [0, 4],
        [4, 4],
        [2, 5],
        [7, 5],
        [5, 6],
        [3, 7],
    ],
];
