use super::*;
impl Demo {
    pub(super) async fn backdrop(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        r: &Renderer,
    ) -> Result<()> {
        let area = self.example == 114;
        s.tone_mapping = ToneMapping::Neutral;
        s.exposure = if area { 0.9 } else { 0.3 };
        let bg = mix(rgb(0x66bbff), rgb(0x4466ff), vp::screen_uv().y());
        let bg = if area {
            hue(bg, time() * float(0.1))
        } else {
            bg
        };
        let bg = background_material(r, vec4(bg, float(1.0))).await?;
        let h = mesh(
            s,
            Arc::new(PlaneGeometry::build(2.0, 2.0, 1, 1)?),
            Material::Shader(bg),
        );
        s.get_mut(h)?.frustum_culled = false;
        s.get_mut(h)?.render_order = -100;
        self.objects.push(h);
        if area {
            s.insert(NodeKind::Light(Light::Ambient {
                color: Color::WHITE,
                intensity: 2.5,
            }));
        } else {
            let h = s.insert(NodeKind::Light(Light::Spot {
                color: Color::WHITE,
                intensity: 2000.0 / std::f64::consts::PI,
                target: Vector3::ZERO,
                distance: 0.0,
                decay: 2.0,
                angle: std::f64::consts::PI / 3.0,
                penumbra: 0.0,
            }));
            s.get_mut(h)?.position.y = 1.0;
            s.add(c, h)?;
        }
        let (a, b, i) = load_asset(&format!("{ASSETS}/models/gltf/Michelle.glb")).await?;
        let imported = crate::gltf::import_animated_decoded(&a, &b, &i)?;
        let instance = imported.instantiate(s)?;
        let mut mixer = crate::animation::AnimationMixer::default();
        mixer.play(instance.clips[0].clone())?;
        self.mixer = Some(mixer);
        let program = if !area {
            Some(Arc::new(
                SurfaceNodes {
                    output: Some(mix(
                        output(),
                        ((output() + float(0.1)) * float(4.0)).floor() / float(4.0) * float(2.0),
                        osc(time() * float(0.1)),
                    )),
                    ..Default::default()
                }
                .build(r, &[], &[])
                .await?,
            ))
        } else {
            None
        };
        for h in instance.meshes {
            if let NodeKind::Mesh(m) = &mut s.get_mut(h)?.kind {
                for m in &mut m.materials {
                    match Arc::make_mut(m) {
                        Material::Standard(m)
                        | Material::Physical(MeshPhysicalMaterial { base: m, .. }) => {
                            m.energy_conservation = true;
                            m.properties.vertex_program = program.clone();
                        }
                        _ => {}
                    }
                }
            }
            self.objects.push(h);
        }
        if area {
            self.params[..3].copy_from_slice(&[1.0, 1.0, 1.0]);
            let p = vp::screen_uv();
            let depth = vp::depth(p.clone());
            let scene_z = vp::perspective_depth_to_view_z(depth, float(0.25), float(25.0));
            let delta = ((-scene_z - view_z()) / float(24.75)).abs();
            let alpha =
                (float(1.0) - delta.clone()).smoothstep(float(0.9), float(2.0)) * float(10.0);
            let alpha = alpha.clamp(float(0.0), float(1.0));
            let blur = delta.smoothstep(float(0.0), float(0.6)) * float(40.0);
            let blur = blur.clamp(float(0.0), float(1.0)) * float(0.1);
            let checker = checker(uv() * float(3.0) * uniform(1, Type::Vec2));
            for (backdrop, opacity) in [
                (
                    vp::hash_blur(p.clone(), blur).rgb()
                        + (float(1.0) - alpha.clone()) * rgb(0x003399) * float(0.3),
                    None,
                ),
                (vp::hash_blur(p.clone(), float(0.05)).rgb(), Some(checker)),
                (splat(alpha, Type::Vec3), None),
                (
                    vp::color((p * float(100.0)).floor() / float(100.0)).rgb(),
                    None,
                ),
            ] {
                let color = opacity
                    .clone()
                    .map(|a| vec4(splat(float(1.0), Type::Vec3), a));
                let program = SurfaceNodes {
                    color,
                    backdrop: Some(vec4(backdrop, opacity.unwrap_or(float(1.0)))),
                    ..Default::default()
                }
                .build(r, &[], &[])
                .await?;
                let mut m = MeshBasicMaterial::default();
                m.properties.transparent = true;
                m.properties.side = Side::Double;
                m.properties.vertex_program = Some(Arc::new(program));
                self.materials.push(Arc::new(Material::Basic(m)));
            }
            // The pixel material is front-sided, as in the original.
            Arc::make_mut(&mut self.materials[3]).properties_mut().side = Side::Front;
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                Arc::new(BoxGeometry::build(2.0, 2.0, 2.0)?),
                self.materials[1].clone(),
            )));
            s.get_mut(h)?.position.y = 1.0;
            s.get_mut(h)?.render_order = 1;
            self.group = Some(h);
            self.objects.push(h);
            let alpha = (float(1.0) - position_world().swizzle("xz").length())
                .clamp(float(0.0), float(1.0));
            let program = SurfaceNodes {
                color: Some(vec4(rgb(0xff6600), alpha)),
                ..Default::default()
            }
            .build(r, &[], &[])
            .await?;
            let mut m = MeshBasicMaterial::default();
            m.properties.transparent = true;
            m.properties.depth_write = false;
            m.properties.vertex_program = Some(Arc::new(program));
            mesh(
                s,
                Arc::new(BoxGeometry::build(5.0, 0.01, 5.0)?),
                Material::Basic(m),
            );
        } else {
            let group = s.insert(NodeKind::Group);
            self.group = Some(group);
            let g = Arc::new(SphereGeometry::build(0.3, 32, 16)?);
            let p = vp::screen_uv();
            let read = vp::color(p.clone()).rgb();
            let grayscale = read
                .clone()
                .dot(vec3(float(0.2126), float(0.7152), float(0.0722)));
            let expressions = [
                (
                    hue(
                        read.swizzle("zyx"),
                        osc(time()) * float(std::f32::consts::PI),
                    ),
                    float(1.0),
                ),
                (float(1.0) - read.clone(), float(1.0)),
                (splat(grayscale, Type::Vec3), float(1.0)),
                (saturation(read.clone(), float(10.0)), osc(time())),
                (
                    blend_overlay(read.clone(), splat(checker(uv() * float(10.0)), Type::Vec3)),
                    float(1.0),
                ),
                (
                    vp::color(vp::safe_uv((p.clone() * float(40.0)).floor() / float(40.0))).rgb(),
                    float(1.0),
                ),
                (
                    vp::color(vp::safe_uv((p * float(80.0)).floor() / float(80.0))).rgb()
                        + rgb(0x0033ff),
                    float(1.0),
                ),
                (vec3(float(0.0), float(0.0), read.swizzle("z")), float(1.0)),
            ];
            for (i, (backdrop, alpha)) in expressions.into_iter().enumerate() {
                let program = SurfaceNodes {
                    backdrop: Some(vec4(backdrop, alpha)),
                    ..Default::default()
                }
                .build(r, &[], &[])
                .await?;
                let mut m = MeshStandardMaterial {
                    roughness: 0.2,
                    metalness: 0.0,
                    energy_conservation: true,
                    ..Default::default()
                };
                m.properties.color = Color::from_hex(0x0066ff);
                m.properties.transparent = true;
                m.properties.vertex_program = Some(Arc::new(program));
                let h = mesh(s, g.clone(), Material::Standard(m));
                let angle = i as f64 * std::f64::consts::FRAC_PI_4;
                s.get_mut(h)?.position = Vector3::new(angle.cos(), 1.0, angle.sin());
                s.add(group, h)?;
                self.objects.push(h);
            }
        }
        Ok(())
    }
}
