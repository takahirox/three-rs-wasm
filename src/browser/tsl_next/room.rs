//! Pinned r186 RoomEnvironment: one-time GPU cubemap capture, not a baked image.
use super::*;
pub(super) fn environment(r: &Renderer) -> Result<Arc<crate::environment::EnvironmentMap>> {
    let mut s = Scene::new();
    s.background = Color::BLACK;
    let geometry = Arc::new(BoxGeometry::build(1.0, 1.0, 1.0)?);
    let mut add =
        |position: [f64; 3], scale: [f64; 3], rotation: f64, material: Material| -> Result<()> {
            let h = s.insert(NodeKind::Mesh(Mesh::new(
                geometry.clone(),
                Arc::new(material),
            )));
            let n = s.get_mut(h)?;
            n.position = Vector3::from_array(position) - Vector3::new(0.0, 3.5, 0.0);
            n.scale = Vector3::from_array(scale);
            n.quaternion = Quaternion::from_rotation_y(rotation);
            Ok(())
        };
    let mut room = MeshStandardMaterial {
        energy_conservation: true,
        ..Default::default()
    };
    room.properties.side = Side::Back;
    add(
        [-0.757, 13.219, 0.717],
        [31.713, 28.305, 28.591],
        0.0,
        Material::Standard(room),
    )?;
    for (p, scale, intensity) in [
        ([-16.116, 14.37, 8.208], [0.1, 2.428, 2.739], 50.0),
        ([-16.109, 18.021, -8.207], [0.1, 2.425, 2.751], 50.0),
        ([14.904, 12.198, -1.832], [0.15, 4.265, 6.331], 17.0),
        ([-0.462, 8.89, 14.520], [4.38, 5.441, 0.088], 43.0),
        ([3.235, 11.486, -12.541], [2.5, 2.0, 0.1], 20.0),
        ([0.0, 20.0, 0.0], [1.0, 0.1, 1.0], 100.0),
    ] {
        let mut m = MeshBasicMaterial::default();
        m.properties.color = Color(Vector3::splat(intensity));
        add(p, scale, 0.0, Material::Basic(m))?;
    }
    let h = s.insert(NodeKind::Mesh(Mesh::new(
        geometry,
        Arc::new(Material::Standard(MeshStandardMaterial {
            energy_conservation: true,
            ..Default::default()
        })),
    )));
    s.get_mut(h)?.instances = [
        ([-10.906, 2.009, 1.846], [2.328, 7.905, 4.651], -0.195),
        ([-5.607, -0.754, -0.758], [1.970, 1.534, 3.955], 0.994),
        ([6.167, 0.857, 7.803], [3.927, 6.285, 3.687], 0.561),
        ([-2.017, 0.018, 6.124], [2.002, 4.566, 2.064], 0.333),
        ([2.291, -0.756, -2.621], [1.546, 1.552, 1.496], -0.286),
        ([-2.193, -0.369, -5.547], [3.875, 3.487, 2.986], 0.516),
    ]
    .into_iter()
    .map(|(p, scale, rot)| Instance {
        matrix: Matrix4::from_scale_rotation_translation(
            Vector3::from_array(scale),
            Quaternion::from_rotation_y(rot),
            Vector3::from_array(p) - Vector3::new(0.0, 3.5, 0.0),
        ),
        color: Color::WHITE,
    })
    .collect();
    let h = s.insert(NodeKind::Light(Light::Point {
        color: Color::WHITE,
        intensity: 900.0,
        distance: 28.0,
        decay: 2.0,
    }));
    s.get_mut(h)?.position = Vector3::new(0.418, 16.199 - 3.5, 0.300);
    Ok(Arc::new(crate::environment::EnvironmentMap::from_scene(
        r, &mut s, 256, 0.04,
    )?))
}
