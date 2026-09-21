use super::*;
use crate::{postprocessing::Effect, tsl::smaa::SmaaPass};
pub(super) struct Smaa {
    scene: RenderTarget,
    pass: SmaaPass,
    copy: Effect,
}
impl Demo {
    pub(super) async fn smaa(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let geometry = Arc::new(BoxGeometry::build(120.0, 120.0, 120.0)?);
        let mut material = MeshBasicMaterial::default();
        material.properties.wireframe = true;
        let h = s.insert(NodeKind::Mesh(Mesh::new(
            geometry.clone(),
            Arc::new(Material::Basic(material)),
        )));
        s.get_mut(h)?.position.x = -100.0;
        self.objects.push(h);
        let mut image =
            decode_image(&fetch(&format!("{ASSETS}/textures/brick_diffuse.jpg")).await?).await?;
        image.srgb = true;
        image.mipmap_filter = Some(Filter::Linear);
        let mut material = MeshBasicMaterial::default();
        material.properties.map = Some(Arc::new(image));
        let h = s.insert(NodeKind::Mesh(Mesh::new(
            geometry,
            Arc::new(Material::Basic(material)),
        )));
        s.get_mut(h)?.position.x = 100.0;
        self.objects.push(h);
        let mut textures = vec![];
        for name in ["smaa-area.png", "smaa-search.png"] {
            let mut image = decode_image(&fetch(&format!("{ASSETS}/{name}")).await?).await?;
            image.srgb = false;
            if name.contains("search") {
                image.min_filter = Some(Filter::Nearest);
                image.filter = Filter::Nearest;
            }
            textures.push(r.upload_texture(&Arc::new(image))?);
        }
        let pass = SmaaPass::new(
            r,
            (&textures[0].view, &textures[0].sampler),
            (&textures[1].view, &textures[1].sampler),
        )
        .await?;
        let scene = RenderTarget::with_options(
            &r.device,
            1,
            1,
            RenderTargetOptions {
                format: wgpu::TextureFormat::Rgba16Float,
                ..Default::default()
            },
        )?;
        let copy = tsl::effect(
            r,
            wgpu::TextureFormat::Rgba16Float,
            &tsl::Texture::Input.sample(uv()),
        )
        .await?;
        self.smaa = Some(Smaa { scene, pass, copy });
        Ok(())
    }
}
impl Smaa {
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
        enabled: bool,
    ) -> Result<()> {
        self.scene.set_size(&r.device, out.width, out.height)?;
        r.render(s, c, &self.scene)?;
        if enabled {
            self.pass.render(r, &self.scene, out)
        } else {
            self.copy.apply(r, &self.scene, None, out)
        }
    }
}
