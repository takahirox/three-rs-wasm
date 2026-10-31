use super::*;
impl Demo {
    fn view(
        &mut self,
        s: &mut Scene,
        c: Object3D,
        position: Vector3,
        target: Vector3,
    ) -> Result<()> {
        let p = position - target;
        self.viewer = OrbitViewer::from_camera(target, p.length());
        self.viewer
            .fixture(p.x.atan2(p.z), (p.y / p.length()).asin(), 1.8);
        self.viewer.update(s, c)
    }
    pub(super) async fn solids(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        camera(s, c, 40., 0.1, 100.)?;
        self.view(s, c, Vector3::new(-6., 7., 14.), Vector3::new(0., 0.5, 0.))?;
        s.background = Color::from_hex(0x222222);
        let h = s.insert(NodeKind::Light(Light::Hemisphere {
            sky: Color::WHITE,
            ground: Color::from_hex(0x0e696c),
            intensity: 1.,
        }));
        s.get_mut(h)?.position.y = 1.;
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::WHITE,
            intensity: 6.,
            target: Vector3::ZERO,
        }));
        {
            let n = s.get_mut(light)?;
            n.position = Vector3::new(5., 10., 6.);
            n.cast_shadow = true;
            n.shadow.extent = 10.;
            n.shadow.far = 20.;
            n.shadow.map_size = Some(2048);
            n.shadow.radius = 10.;
        }
        let materials: Vec<_> = [0xe4002b, 0xff7f11, 0xffd100, 0x00a651, 0x0072ce, 0x8a2be2]
            .into_iter()
            .map(|color| {
                let mut m = MeshStandardMaterial {
                    roughness: 0.8,
                    ..Default::default()
                };
                m.properties.color = Color::from_hex(color);
                m.properties.side = Side::Double;
                m.energy_conservation = true;
                Arc::new(Material::Standard(m))
            })
            .collect();
        // makeHoleyGeometry() of the five solids, with their bounding boxes' bottoms.
        let mut geometries = Vec::new();
        let mut minimums = Vec::new();
        for (g, faces) in [
            (TetrahedronGeometry::build(1.2, 0)?, 3),
            (BoxGeometry::build(1.5, 1.5, 1.5)?, 6),
            (OctahedronGeometry::build(1.2, 0)?, 3),
            (DodecahedronGeometry::build(1.1, 0)?, 9),
            (IcosahedronGeometry::build(1.1, 0)?, 3),
        ] {
            let mut h = holey(&g, faces, 0.6, materials.len())?;
            minimums.push(h.compute_bounding_box()?.min.y);
            geometries.push(Arc::new(h));
        }
        let mut m = MeshStandardMaterial {
            roughness: 0.5,
            ..Default::default()
        };
        m.properties.color = Color::from_hex(0x0e696c);
        m.energy_conservation = true;
        let table = mesh(
            s,
            Arc::new(BoxGeometry::build(17., 0.5, 17.)?),
            Material::Standard(m),
        );
        s.get_mut(table)?.position.y = -0.25;
        s.get_mut(table)?.receive_shadow = true;
        for (i, j) in [0, 1, 2, 3, 3, 4, 0, 1, 1, 2, 3, 4, 4, 0, 1, 2]
            .into_iter()
            .enumerate()
        {
            let row = i / 4;
            let col = i % 4;
            let scale = 0.5 + row as f64 * 0.25;
            let mut m = Mesh::new(geometries[j].clone(), materials[0].clone());
            m.materials = materials.clone();
            let h = s.insert(NodeKind::Mesh(m));
            let n = s.get_mut(h)?;
            n.cast_shadow = true;
            n.scale = Vector3::splat(scale);
            n.position = Vector3::new(
                (col as f64 - 1.5) * 3.5,
                -minimums[j] * scale - 0.01,
                (1.5 - row as f64) * 3.5,
            );
        }
        Ok(())
    }
    pub(super) fn clipping(&mut self, s: &mut Scene, c: Object3D) -> Result<()> {
        camera(s, c, 36., 0.25, 16.)?;
        self.view(s, c, Vector3::new(0., 1.3, 3.), Vector3::new(0., 1., 0.))?;
        s.clipping_shadows = false;
        s.insert(NodeKind::Light(Light::Ambient {
            color: Color::from_hex(0xcccccc),
            intensity: 1.,
        }));
        let spot = s.insert(NodeKind::Light(Light::Spot {
            color: Color::WHITE,
            intensity: 60.,
            target: Vector3::ZERO,
            distance: 0.,
            decay: 2.,
            angle: std::f64::consts::PI / 5.,
            penumbra: 0.2,
            ies: false,
        }));
        {
            let n = s.get_mut(spot)?;
            n.position = Vector3::new(2., 3., 3.);
            n.cast_shadow = true;
            n.shadow.near = 3.;
            n.shadow.far = 10.;
            n.shadow.map_size = Some(2048);
            n.shadow.radius = 4.;
        }
        // DirectionalLight: a 1024² map, its shadow camera ±1 from 1 to 10.
        let light = s.insert(NodeKind::Light(Light::Directional {
            color: Color::from_hex(0x55505a),
            intensity: 3.,
            target: Vector3::ZERO,
        }));
        {
            let n = s.get_mut(light)?;
            n.position = Vector3::new(0., 3., 0.);
            n.cast_shadow = true;
            n.shadow.near = 1.;
            n.shadow.far = 10.;
            n.shadow.extent = 1.;
            n.shadow.map_size = Some(1024);
        }
        let mut m = MeshPhongMaterial {
            shininess: 0.,
            ..Default::default()
        };
        m.properties.color = Color::from_hex(0x80ee10);
        m.properties.side = Side::Double;
        m.properties.alpha_to_coverage = true;
        let knot = mesh(
            s,
            Arc::new(TorusKnotGeometry::build(0.4, 0.08, 95, 20, 2, 3)?),
            Material::Phong(m),
        );
        s.get_mut(knot)?.cast_shadow = true;
        let mut m = MeshPhongMaterial {
            shininess: 150.,
            ..Default::default()
        };
        m.properties.color = Color::from_hex(0xa0adaf);
        m.properties.alpha_to_coverage = true;
        let ground = mesh(
            s,
            Arc::new(PlaneGeometry::build(9., 9., 1, 1)?),
            Material::Phong(m),
        );
        let n = s.get_mut(ground)?;
        n.receive_shadow = true;
        n.quaternion = Quaternion::from_rotation_x(-std::f64::consts::FRAC_PI_2);
        self.objects = vec![knot, ground];
        self.update_clipping(s)
    }
    pub(super) fn update_clipping(&self, s: &mut Scene) -> Result<()> {
        s.clipping_planes = if self.params[5] > 0.5 {
            vec![Plane {
                normal: Vector3::NEG_X,
                constant: self.params[6] as f64,
            }]
        } else {
            vec![]
        };
        for (i, &h) in self.objects.iter().enumerate() {
            let n = s.get_mut(h)?;
            if i == 0 {
                n.position.y = 0.8;
                n.quaternion = Quaternion::from_euler(
                    glam::EulerRot::XYZ,
                    self.time * 0.5,
                    self.time * 0.2,
                    0.,
                );
                n.scale = Vector3::splat(self.time.cos() * 0.125 + 0.875);
            }
            if let NodeKind::Mesh(m) = &mut n.kind {
                let p = Arc::make_mut(&mut m.materials[0]).properties_mut();
                p.alpha_to_coverage = self.params[0] > 0.5;
                if i == 0 {
                    p.clip_shadows = self.params[2] > 0.5;
                    p.clip_intersection = self.params[3] > 0.5;
                    p.clipping_planes = if self.params[1] > 0.5 {
                        vec![
                            Plane {
                                normal: Vector3::NEG_Y,
                                constant: self.params[4] as f64,
                            },
                            Plane {
                                normal: Vector3::NEG_Z,
                                constant: 0.1,
                            },
                        ]
                    } else {
                        vec![]
                    };
                }
            }
        }
        Ok(())
    }
}

/// makeHoleyGeometry( geometry, verticesPerFace, holeScale ): each face's
/// unique corners, sorted around its centroid, bridged to the corners scaled
/// toward the centroid; one group per face, then computeVertexNormals().
fn holey(
    geometry: &BufferGeometry,
    vertices_per_face: usize,
    hole_scale: f64,
    materials: usize,
) -> Result<BufferGeometry> {
    let geometry = geometry.to_non_indexed()?;
    let position = geometry.positions()?;
    let mut positions: Vec<f64> = vec![];
    let mut out = BufferGeometry::default();
    for face in 0..position.len() / vertices_per_face {
        let mut corners: Vec<Vector3> = vec![];
        let mut keys: Vec<[u64; 3]> = vec![];
        for i in 0..vertices_per_face {
            let v = position[face * vertices_per_face + i];
            // v.toArray().join( ',' ): −0 prints as 0.
            let key = v
                .to_array()
                .map(|c| if c == 0. { 0f64.to_bits() } else { c.to_bits() });
            if !keys.contains(&key) {
                keys.push(key);
                corners.push(v);
            }
        }
        let n = corners.len() as f64;
        let centroid = corners.iter().fold(Vector3::ZERO, |c, &v| c + v) * (1. / n);
        let normalize = |v: Vector3| {
            let l = v.length();
            v * (1. / if l == 0. { 1. } else { l })
        };
        let normal = normalize((corners[1] - corners[0]).cross(corners[2] - corners[0]));
        let tangent = normalize(corners[0] - centroid);
        let bitangent = normal.cross(tangent);
        let angle = |v: Vector3| {
            let d = v - centroid;
            d.dot(bitangent).atan2(d.dot(tangent))
        };
        corners.sort_by(|p, q| {
            (angle(*p) - angle(*q))
                .partial_cmp(&0.)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        let inner: Vec<Vector3> = corners
            .iter()
            .map(|&v| centroid + (v - centroid) * hole_scale)
            .collect();
        out.add_group(positions.len() / 3, corners.len() * 6, face % materials);
        for i in 0..corners.len() {
            let j = (i + 1) % corners.len();
            for v in [
                corners[i], corners[j], inner[j], corners[i], inner[j], inner[i],
            ] {
                positions.extend(v.to_array());
            }
        }
    }
    out.set_attribute(
        "position",
        Attribute::F32(BufferAttribute::new(
            positions.into_iter().map(|v| v as f32).collect(),
            3,
            false,
        )?),
    );
    out.compute_vertex_normals()?;
    Ok(out)
}
