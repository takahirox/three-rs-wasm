use super::*;
use crate::{compute_skinning::ComputeSkinning, tsl::sprites::SpriteNodeMaterial};
pub(in crate::browser) struct SkinningPoints {
    time: f64,
    mixer: crate::animation::AnimationMixer,
    skins: Vec<ComputeSkinning>,
    initialized: bool,
    points: Vec<Object3D>,
}
impl SkinningPoints {
    pub async fn create(s: &mut Scene, c: Object3D, r: &Renderer) -> Result<Self> {
        let aspect = match s.camera(c)?.0 {
            Camera::Perspective(p) => p.aspect,
            _ => 1.,
        };
        s.get_mut(c)?.kind = NodeKind::Camera(Camera::Perspective(PerspectiveCamera {
            fov: 50.,
            aspect,
            near: 1.,
            far: 1000.,
            ..Default::default()
        }));
        s.get_mut(c)?.position = Vector3::new(0., 300., -85.);
        s.look_at(c, Vector3::new(0., 0., -85.))?;
        s.background = Color::from_hex(0x111111);
        let (a, b, i) =
            load_asset("/web/gallery/assets/tsl-viewport/models/gltf/Michelle.glb").await?;
        let instance = crate::gltf::import_animated_decoded(&a, &b, &i)?.instantiate(s)?;
        let group = s.insert(NodeKind::Group);
        for root in instance.roots {
            s.add(group, root)?;
        }
        s.get_mut(group)?.scale = Vector3::splat(100.);
        s.get_mut(group)?.quaternion = Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
        let mut mixer = crate::animation::AnimationMixer::default();
        mixer.play(instance.clips[0].clone())?;
        let mut skins = vec![];
        let mut points = vec![];
        for object in instance.meshes {
            s.get_mut(object)?.visible = false;
            let skin = ComputeSkinning::new(r, s, object).await?;
            let speed = storage_element(1, instance_index()).rgb();
            let color = mix(rgb(0x0066ff), rgb(0xff9000), speed.clone() * float(0.6));
            let mut sprite = SpriteNodeMaterial::new(vec4(color, shape_circle(true)));
            sprite.position = storage_element(0, instance_index()).rgb();
            sprite.scale = speed.length().exp().clamp(float(0.), float(5.)) * float(5.) + float(1.);
            sprite.scale = sprite.scale * uniform(0, Type::Float);
            sprite.size_attenuation = float(0.);
            let mut material = sprite
                .build_points(
                    r,
                    &[
                        (&skin.positions, Type::Vec4),
                        (&skin.displacement, Type::Vec4),
                    ],
                    &[],
                )
                .await?;
            material.properties.transparent = false;
            material.properties.alpha_test = 0.5;
            let mut geometry = PlaneGeometry::build(1., 1., 1, 1)?;
            geometry.instance_count = Some(skin.count());
            let h = mesh(s, Arc::new(geometry), Material::Shader(material));
            s.get_mut(h)?.frustum_culled = false;
            points.push(h);
            skins.push(skin);
        }
        Ok(Self {
            time: 0.,
            mixer,
            skins,
            initialized: false,
            points,
        })
    }
    pub fn seek(&mut self, t: f64) {
        self.time = t;
    }
    pub fn update(&mut self, s: &mut Scene, _c: Object3D, d: f64, a: bool) -> Result<()> {
        if a {
            self.time += d;
        }
        for action in &mut self.mixer.actions {
            action.time = self.time;
        }
        self.mixer.update(s, 0.)?;
        let ratio = web_sys::window().map_or(1., |w| w.device_pixel_ratio()) as f32;
        for &h in &self.points {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind
                && let Material::Shader(m) = Arc::make_mut(&mut m.materials[0])
            {
                m.uniforms[0][0] = ratio;
            }
        }
        Ok(())
    }
    pub fn render(
        &mut self,
        r: &Renderer,
        s: &mut Scene,
        c: Object3D,
        out: &RenderTarget,
    ) -> Result<bool> {
        s.update()?;
        for skin in &self.skins {
            skin.update(r, s)?;
            if !self.initialized {
                skin.update(r, s)?;
            }
        }
        self.initialized = true;
        r.render(s, c, out)?;
        Ok(true)
    }
}
