use super::*;
use crate::attribute::BufferAttribute;
pub(super) async fn castle(r: &Renderer) -> Result<GpuTexture> {
    let mut faces = Vec::new();
    for name in ["px", "nx", "py", "ny", "pz", "nz"] {
        let mut image =
            decode_image(&fetch(&format!("/web/gallery/assets/castle-{name}.jpg")).await?).await?;
        image.srgb = true;
        faces.push(image);
    }
    GpuTexture::from_cube_rgba(
        r,
        &faces.try_into().map_err(|_| Error::Invalid("cube faces"))?,
    )
}
pub(super) fn reflection() -> Result<tsl::Node> {
    Ok(WgslFn::new("cube_reflection", "fn cube_reflection(p:vec3<f32>,n:vec3<f32>)->vec3<f32>{let d=reflect(normalize(p-u.camera.xyz),normalize(n));return vec3(-d.x,d.yz);}",&[Type::Vec3,Type::Vec3],Type::Vec3)?.call(&[position_world(),normal_world()]))
}
impl Demo {
    pub(super) async fn instance_uniform(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        r: &Renderer,
    ) -> Result<()> {
        let position = Vector3::new(0.0, 200.0, 1200.0);
        self.viewer = OrbitViewer::from_camera(Vector3::ZERO, position.length());
        self.viewer
            .fixture(0.0, (position.y / position.length()).asin(), 1.8);
        self.viewer.update(s, c)?;
        let mut positions = Vec::new();
        for i in 0..=40 {
            let k = -500.0 + i as f32 * 25.0;
            positions.extend_from_slice(&[
                -500.0, -75.0, k, 500.0, -75.0, k, k, -75.0, -500.0, k, -75.0, 500.0,
            ]);
        }
        let mut grid = BufferGeometry::default();
        grid.set_attribute(
            "position",
            Attribute::F32(BufferAttribute::new(positions, 3, false)?),
        );
        let mut line = LineBasicMaterial::default();
        line.properties.color = Color::from_hex(0x303030);
        s.insert(NodeKind::Line(Line {
            geometry: Arc::new(grid),
            material: Arc::new(Material::Line(line)),
            segments: true,
        }));
        #[derive(serde::Deserialize)]
        struct AttributeData {
            size: usize,
            array: Vec<f32>,
        }
        #[derive(serde::Deserialize)]
        struct GeometryData {
            index: Vec<u32>,
            attributes: std::collections::HashMap<String, AttributeData>,
        }
        let data: GeometryData =
            serde_json::from_slice(&fetch("/web/gallery/assets/teapot-50-18.json").await?)
                .map_err(|e| Error::Asset(e.to_string()))?;
        let mut geometry = BufferGeometry::default();
        for (name, a) in data.attributes {
            geometry.set_attribute(
                &name,
                Attribute::F32(BufferAttribute::new(a.array, a.size, false)?),
            );
        }
        geometry.set_index(Some(data.index));
        let geometry = Arc::new(geometry);
        let cube = castle(r).await?;
        let reflected = tsl::sampling::cube(tsl::Texture::External(0), reflection()?).rgb();
        let color = uniform(0, Type::Vec3);
        let material = NodeMaterial::new(color.clone() + reflected.clone() + color * reflected)
            .build_with_texture_types(r, &[(&cube.view, &cube.sampler, Type::TextureCube)])
            .await?;
        let mut seed = 186;
        for i in 0..12 {
            let mut m = material.clone();
            m.uniforms[0] = Color::from_hex((random(&mut seed) * 0xffffff as f64) as u32)
                .0
                .extend(1.0)
                .as_vec4()
                .to_array();
            let rotation = [
                random(&mut seed) * 200.0 - 100.0,
                random(&mut seed) * 200.0 - 100.0,
                random(&mut seed) * 200.0 - 100.0,
            ];
            self.initial_rotations.push(rotation);
            let h = mesh(s, geometry.clone(), Material::Shader(m));
            s.get_mut(h)?.position = Vector3::new(
                (i % 4) as f64 * 200.0 - 300.0,
                0.0,
                (i / 4) as f64 * 200.0 - 200.0,
            );
            self.objects.push(h);
        }
        Ok(())
    }
    pub(super) fn update_instance_uniform(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        self.viewer.update(s, c)?;
        for (h, rotation) in self.objects.iter().zip(&self.initial_rotations) {
            s.get_mut(*h)?.quaternion = Quaternion::from_euler(
                glam::EulerRot::XYZ,
                rotation[0] + self.time * 0.6,
                rotation[1] + self.time * 0.3,
                rotation[2],
            );
        }
        Ok(())
    }
}

impl Demo {
    pub(super) async fn cube_mipmaps(&mut self, s: &mut Scene, r: &Renderer) -> Result<()> {
        let mut levels = Vec::new();
        for level in 0..9 {
            let mut faces = Vec::new();
            for face in 0..6 {
                let mut image = decode_image(
                    &fetch(&format!("/web/gallery/assets/cube_m0{level}_c0{face}.jpg")).await?,
                )
                .await?;
                image.srgb = true;
                faces.push(image);
            }
            levels.push(faces.try_into().map_err(|_| Error::Invalid("cube faces"))?);
        }
        let auto = GpuTexture::from_cube_rgba(r, &levels[0])?;
        let manual = GpuTexture::from_cube_mipmaps(r, &levels)?;
        let geometry = Arc::new(SphereGeometry::build(100.0, 128, 128)?);
        for (x, cube) in [(-100.0, auto), (100.0, manual)] {
            let m = NodeMaterial::new(
                tsl::sampling::cube(tsl::Texture::External(0), reflection()?).rgb(),
            )
            .build_with_texture_types(r, &[(&cube.view, &cube.sampler, Type::TextureCube)])
            .await?;
            let h = mesh(s, geometry.clone(), Material::Shader(m));
            s.get_mut(h)?.position.x = x;
            self.objects.push(h);
        }
        Ok(())
    }
}
