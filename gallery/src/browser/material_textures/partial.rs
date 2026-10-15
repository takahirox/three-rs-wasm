use super::*;
pub(super) struct Partial {
    destination: GpuTexture,
    source: GpuTexture,
    last: f64,
    seed: u32,
}
impl Partial {
    pub async fn new(s: &mut Scene, r: &Renderer) -> Result<Self> {
        let t = Arc::new(image("Carbon.png").await?);
        let destination = r.upload_texture(&t)?;
        let source =
            GpuTexture::from_rgba_mipmaps(r, &[Texture::from_rgba(32, 32, vec![0; 4096], true)?])?;
        let mut m = MeshBasicMaterial::default();
        m.properties.vertex_program = Some(Arc::new(
            SurfaceNodes {
                color: Some(
                    tsl::Texture::External(0)
                        .sample(vec2(uv().x(), float(1.) - uv().y()))
                        .rgb(),
                ),
                ..Default::default()
            }
            .build(r, &[], &[(&destination.view, &destination.sampler)])
            .await?,
        ));
        mesh(
            s,
            Arc::new(PlaneGeometry::build(2., 2., 1, 1)?),
            Material::Basic(m),
        );
        Ok(Self {
            destination,
            source,
            last: 0.,
            seed: 186,
        })
    }
    fn random(&mut self) -> f64 {
        self.seed = self.seed.wrapping_mul(1664525).wrapping_add(1013904223);
        self.seed as f64 / 4294967296.
    }
    pub fn update(&mut self, r: &Renderer, time: f64) -> Result<()> {
        if time - self.last <= 0.1 {
            return Ok(());
        }
        self.last = time;
        let x = (self.random() * 16.).floor() as u32 * 32;
        let y = (self.random() * 16.).floor() as u32 * 32;
        let color = Color::from_hex((self.random() * 0xffffff as f64) as u32);
        let pixel = [
            (color.0.x * 255.).floor() as u8,
            (color.0.y * 255.).floor() as u8,
            (color.0.z * 255.).floor() as u8,
            1,
        ];
        r.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.source.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &pixel.repeat(1024),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(128),
                rows_per_image: Some(32),
            },
            wgpu::Extent3d {
                width: 32,
                height: 32,
                depth_or_array_layers: 1,
            },
        );
        self.destination.copy_region_from(
            r,
            &self.source,
            [0, 0],
            [x, self.destination.texture.height() - 32 - y],
            [32, 32],
        )
    }
}
