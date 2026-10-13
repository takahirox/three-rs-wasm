//! @takram/three-clouds' CPU side as the example configures CloudsEffect:
//! CloudLayers.DEFAULT packed into the layer uniforms, CascadedShadowMaps
//! ( two practical cascades, λ 0.71, farScale 0.25, maxFar 1e5, 512² ),
//! the Bayer temporal jitter and the geodetic helpers of
//! @takram/three-geospatial ( Ellipsoid.WGS84 ) the passes read.
use super::program::Block;
use super::volume::{M4, V3, apply4, cross, dot, invert, length, multiply, normalize};

/// @takram/three-geospatial's Ellipsoid.WGS84 radii.
pub(crate) const RADII: V3 = [6378137., 6378137., 6_356_752.314_245_179];
pub(crate) const BOTTOM_RADIUS: f64 = 6360000.;
pub(crate) const SHADOW_MAP_SIZE: u32 = 512;
pub(crate) const CASCADE_COUNT: usize = 2;
const SPLIT_LAMBDA: f64 = 0.71;
const FAR_SCALE: f64 = 0.25;
const MAX_FAR: f64 = 1e5;

/// CloudLayer: ( altitude, height, densityScale, shapeAmount,
/// shapeDetailAmount, weatherExponent, shapeAlteringBias,
/// coverageFilterWidth, shadow ), densityProfile ( 0, 0, 0.75, 0.25 ).
struct Layer {
    altitude: f64,
    height: f64,
    density_scale: f64,
    shape_amount: f64,
    shape_detail_amount: f64,
    weather_exponent: f64,
    shape_altering_bias: f64,
    coverage_filter_width: f64,
    shadow: bool,
}

const LAYERS: [Layer; 4] = [
    Layer {
        altitude: 750.,
        height: 650.,
        density_scale: 0.2,
        shape_amount: 1.,
        shape_detail_amount: 1.,
        weather_exponent: 1.,
        shape_altering_bias: 0.35,
        coverage_filter_width: 0.6,
        shadow: true,
    },
    Layer {
        altitude: 1000.,
        height: 1200.,
        density_scale: 0.2,
        shape_amount: 1.,
        shape_detail_amount: 1.,
        weather_exponent: 1.,
        shape_altering_bias: 0.35,
        coverage_filter_width: 0.6,
        shadow: true,
    },
    Layer {
        altitude: 7500.,
        height: 500.,
        density_scale: 0.003,
        shape_amount: 0.4,
        shape_detail_amount: 0.,
        weather_exponent: 1.,
        shape_altering_bias: 0.35,
        coverage_filter_width: 0.5,
        shadow: false,
    },
    Layer {
        altitude: 0.,
        height: 0.,
        density_scale: 0.2,
        shape_amount: 1.,
        shape_detail_amount: 1.,
        weather_exponent: 1.,
        shape_altering_bias: 0.35,
        coverage_filter_width: 0.6,
        shadow: false,
    },
];

/// updateCloudLayerUniforms( uniforms, CloudLayers.DEFAULT ).
/// `mask` is false for CloudsMaterial, whose shadowLayerMask stays ( 1, 1, 1, 1 ).
pub(crate) fn set_layers(b: &mut Block, mask: bool) {
    let pack = |f: &dyn Fn(&Layer) -> f64| LAYERS.iter().map(f).collect::<Vec<f64>>();
    b.set("minLayerHeights", &pack(&|l| l.altitude));
    b.set("maxLayerHeights", &pack(&|l| l.altitude + l.height));
    // packIntervalHeights: the union of the layers' [ altitude, top ] spans.
    let mut entries: Vec<(f64, u8)> = LAYERS
        .iter()
        .flat_map(|l| [(l.altitude, 0u8), (l.altitude + l.height, 1u8)])
        .collect();
    entries.sort_by(|a, b| {
        if a.0 != b.0 {
            a.0.total_cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });
    let mut intervals = vec![];
    let mut balance = 0i32;
    for (k, &(value, flag)) in entries.iter().enumerate() {
        if balance == 0 && k > 0 && intervals.len() < 3 {
            intervals.push((entries[k - 1].0, value));
        }
        balance += if flag == 0 { 1 } else { -1 };
    }
    intervals.resize(3, (0., 0.));
    b.set(
        "minIntervalHeights",
        &intervals.iter().map(|i| i.0).collect::<Vec<_>>(),
    );
    b.set(
        "maxIntervalHeights",
        &intervals.iter().map(|i| i.1).collect::<Vec<_>>(),
    );
    b.set("densityScales", &pack(&|l| l.density_scale));
    b.set("shapeAmounts", &pack(&|l| l.shape_amount));
    b.set("shapeDetailAmounts", &pack(&|l| l.shape_detail_amount));
    b.set("weatherExponents", &pack(&|l| l.weather_exponent));
    b.set("shapeAlteringBiases", &pack(&|l| l.shape_altering_bias));
    b.set("coverageFilterWidths", &pack(&|l| l.coverage_filter_width));
    b.set("densityProfile.expTerms", &[0.; 4]);
    b.set("densityProfile.exponents", &[0.; 4]);
    b.set("densityProfile.linearTerms", &[0.75; 4]);
    b.set("densityProfile.constantTerms", &[0.25; 4]);
    let (mut min, mut max, mut shadow_bottom, mut shadow_top) =
        (f64::INFINITY, 0f64, f64::INFINITY, 0f64);
    for l in &LAYERS {
        let top = l.altitude + l.height;
        if l.height > 0. {
            min = min.min(l.altitude);
            if l.shadow {
                shadow_bottom = shadow_bottom.min(l.altitude);
                shadow_top = shadow_top.max(top);
            }
            max = max.max(top);
        }
    }
    b.set1("minHeight", min);
    b.set1("maxHeight", max);
    b.set1("shadowBottomHeight", shadow_bottom);
    b.set1("shadowTopHeight", shadow_top);
    if mask {
        b.set(
            "shadowLayerMask",
            &pack(&|l| if l.shadow { 1. } else { 0. }),
        );
    } else {
        b.set("shadowLayerMask", &[1.; 4]);
    }
}

/// createCloudParameterUniforms with the example's settings.
pub(crate) fn set_parameters(b: &mut Block, local_weather_offset: [f64; 2]) {
    b.set1("scatteringCoefficient", 1.);
    b.set1("absorptionCoefficient", 0.);
    b.set1("coverage", 0.3);
    b.set("localWeatherRepeat", &[100., 100.]);
    b.set("localWeatherOffset", &local_weather_offset);
    b.set("shapeRepeat", &[0.0003; 3]);
    b.set("shapeOffset", &[0.; 3]);
    b.set("shapeDetailRepeat", &[0.006; 3]);
    b.set("shapeDetailOffset", &[0.; 3]);
    b.set("turbulenceRepeat", &[20., 20.]);
    b.set1("turbulenceDisplacement", 350.);
}

/// projectOnEllipsoidSurface ( centerTolerance 0.1 ).
pub(crate) fn project_on_surface(p: V3) -> Option<V3> {
    let r = [
        1. / RADII[0].powi(2),
        1. / RADII[1].powi(2),
        1. / RADII[2].powi(2),
    ];
    let (x2, y2, z2) = (p[0] * p[0] * r[0], p[1] * p[1] * r[1], p[2] * p[2] * r[2]);
    let norm_squared = x2 + y2 + z2;
    let ratio = (1. / norm_squared).sqrt();
    if !ratio.is_finite() {
        return None;
    }
    let intersection = [p[0] * ratio, p[1] * ratio, p[2] * ratio];
    if norm_squared < 0.1 {
        return Some(intersection);
    }
    let gradient = [
        intersection[0] * r[0] * 2.,
        intersection[1] * r[1] * 2.,
        intersection[2] * r[2] * 2.,
    ];
    let mut lambda = ((1. - ratio) * length(p)) / (length(gradient) / 2.);
    let mut correction = 0.;
    loop {
        lambda -= correction;
        let (sx, sy, sz) = (
            1. / (1. + lambda * r[0]),
            1. / (1. + lambda * r[1]),
            1. / (1. + lambda * r[2]),
        );
        let (sx2, sy2, sz2) = (sx * sx, sy * sy, sz * sz);
        let (sx3, sy3, sz3) = (sx2 * sx, sy2 * sy, sz2 * sz);
        let error = x2 * sx2 + y2 * sy2 + z2 * sz2 - 1.;
        correction = error / ((x2 * sx3 * r[0] + y2 * sy3 * r[1] + z2 * sz3 * r[2]) * -2.);
        if error.abs() <= 1e-12 {
            return Some([p[0] * sx, p[1] * sy, p[2] * sz]);
        }
    }
}

/// Geodetic.setFromECEF( position ).height.
pub(crate) fn geodetic_height(p: V3) -> Option<f64> {
    let s = project_on_surface(p)?;
    let h = [p[0] - s[0], p[1] - s[1], p[2] - s[2]];
    let sign = dot(h, p);
    let sign = if sign > 0. {
        1.
    } else if sign < 0. {
        -1.
    } else {
        sign
    };
    Some(sign * length(h))
}

/// getAltitudeCorrectionOffset( position, bottomRadius, WGS84 ).
pub(crate) fn altitude_correction(p: V3) -> V3 {
    let Some(s) = project_on_surface(p) else {
        return [0.; 3];
    };
    let (a2, b2) = (RADII[0].powi(2), RADII[2].powi(2));
    let n = normalize([s[0] / a2, s[1] / a2, s[2] / b2]);
    // getOsculatingSphereCenter( ... ).negate().
    [
        -(n[0] * -BOTTOM_RADIUS + s[0]),
        -(n[1] * -BOTTOM_RADIUS + s[1]),
        -(n[2] * -BOTTOM_RADIUS + s[2]),
    ]
}

/// Ellipsoid.getSurfaceNormal: the reciprocal radii squared times the position.
pub(crate) fn surface_normal(p: V3) -> V3 {
    let r = [
        1. / RADII[0].powi(2),
        1. / RADII[1].powi(2),
        1. / RADII[2].powi(2),
    ];
    normalize([r[0] * p[0], r[1] * p[1], r[2] * p[2]])
}

/// bayerOffsets[ frame % 16 ].
pub(crate) fn bayer_offset(frame: u64) -> (f64, f64) {
    const INDICES: [usize; 16] = [0, 8, 2, 10, 12, 4, 14, 6, 3, 11, 1, 9, 15, 7, 13, 5];
    let index = (frame % 16) as usize;
    let i = INDICES.iter().position(|&v| v == index).unwrap_or(0);
    (((i % 4) as f64 + 0.5) / 4., ((i / 4) as f64 + 0.5) / 4.)
}

/// Matrix4.lookAt( eye, target, up ) on the rotation part of `te`.
fn look_at(te: &mut M4, eye: V3, target: V3, up: V3) {
    let mut z = [eye[0] - target[0], eye[1] - target[1], eye[2] - target[2]];
    if dot(z, z) == 0. {
        z[2] = 1.;
    }
    z = normalize(z);
    let mut x = cross(up, z);
    if dot(x, x) == 0. {
        if up[2].abs() == 1. {
            z[0] += 0.0001;
        } else {
            z[2] += 0.0001;
        }
        z = normalize(z);
        x = cross(up, z);
    }
    x = normalize(x);
    let y = cross(z, x);
    te[0] = x[0];
    te[4] = y[0];
    te[8] = z[0];
    te[1] = x[1];
    te[5] = y[1];
    te[9] = z[1];
    te[2] = x[2];
    te[6] = y[2];
    te[10] = z[2];
}

/// Matrix4.makeOrthographic ( WebGL ).
fn orthographic(left: f64, right: f64, top: f64, bottom: f64, near: f64, far: f64) -> M4 {
    let (w, h, p) = (1. / (right - left), 1. / (top - bottom), 1. / (far - near));
    let (x, y, z) = ((right + left) * w, (top + bottom) * h, (far + near) * p);
    [
        2. * w,
        0.,
        0.,
        0.,
        0.,
        2. * h,
        0.,
        0.,
        0.,
        0.,
        -2. * p,
        0.,
        -x,
        -y,
        -z,
        1.,
    ]
}

#[derive(Clone, Copy, Default)]
pub(crate) struct Cascade {
    pub interval: [f64; 2],
    pub matrix: M4,
    pub inverse_matrix: M4,
}

type Corners = ([V3; 4], [V3; 4]);

fn lerp(a: V3, b: V3, t: f64) -> V3 {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

/// CascadedShadowMaps.update( camera, sunDirection, distance ): the far
/// distance and each cascade's matrices.
pub(crate) fn cascades(
    camera_world: &M4,
    projection: &M4,
    near: f64,
    camera_far: f64,
    sun: V3,
    distance: f64,
) -> (f64, Vec<Cascade>) {
    let far = MAX_FAR.min(camera_far * FAR_SCALE);
    // splitFrustum( 'practical', ... ).
    let splits: Vec<f64> = (0..CASCADE_COUNT)
        .map(|i| {
            let uniform = (near + ((far - near) * (i + 1) as f64) / CASCADE_COUNT as f64) / far;
            let logarithmic =
                (near * (far / near).powf((i + 1) as f64 / CASCADE_COUNT as f64)) / far;
            uniform + (logarithmic - uniform) * SPLIT_LAMBDA
        })
        .collect();
    // FrustumCorners.setFromCamera( camera, far ).
    let inverse_projection = invert(projection);
    let mut corners: Corners = ([[0.; 3]; 4], [[0.; 3]; 4]);
    let signs = [[1., 1.], [1., -1.], [-1., -1.], [-1., 1.]];
    for (i, sign) in signs.iter().enumerate() {
        corners.0[i] = apply4([sign[0], sign[1], -1.], &inverse_projection);
        let c = apply4([sign[0], sign[1], 1.], &inverse_projection);
        let scale = (far / c[2].abs()).min(1.);
        corners.1[i] = [c[0] * scale, c[1] * scale, c[2] * scale];
    }
    // split( splits ).
    let frusta: Vec<Corners> = (0..CASCADE_COUNT)
        .map(|index| {
            let mut f: Corners = corners;
            for i in 0..4 {
                if index > 0 {
                    f.0[i] = lerp(corners.0[i], corners.1[i], splits[index - 1]);
                }
                if index != CASCADE_COUNT - 1 {
                    f.1[i] = lerp(corners.0[i], corners.1[i], splits[index]);
                }
            }
            f
        })
        .collect();
    // updateMatrices.
    let mut light_orientation = super::volume::IDENTITY;
    look_at(
        &mut light_orientation,
        [0.; 3],
        [-sun[0], -sun[1], -sun[2]],
        [0., 1., 0.],
    );
    let camera_to_light = multiply(&invert(&light_orientation), camera_world);
    let mut out = vec![];
    for (i, frustum) in frusta.iter().enumerate() {
        let (near_c, far_c) = frustum;
        // getFrustumRadius ( fade ).
        let mut diagonal = super::volume::distance(far_c[0], far_c[2])
            .max(super::volume::distance(far_c[0], near_c[2]));
        let d = far_c[0][2] / (far - near);
        diagonal += 0.25 * d.powi(2) * (far - near);
        let radius = diagonal * 0.5;
        let (left, right, top, bottom) = (-radius, radius, radius, -radius);
        let projection_matrix = orthographic(left, right, top, bottom, -0., radius * 2. + 0.);
        let mut min = [f64::INFINITY; 3];
        let mut max = [f64::NEG_INFINITY; 3];
        for j in 0..4 {
            for p in [
                apply4(near_c[j], &camera_to_light),
                apply4(far_c[j], &camera_to_light),
            ] {
                for k in 0..3 {
                    min[k] = min[k].min(p[k]);
                    max[k] = max[k].max(p[k]);
                }
            }
        }
        let mut center = [
            (min[0] + max[0]) * 0.5,
            (min[1] + max[1]) * 0.5,
            (min[2] + max[2]) * 0.5,
        ];
        center[2] = max[2] + 0.;
        let texel_width = (right - left) / SHADOW_MAP_SIZE as f64;
        let texel_height = (top - bottom) / SHADOW_MAP_SIZE as f64;
        center[0] = (center[0] / texel_width).round_js() * texel_width;
        center[1] = (center[1] / texel_height).round_js() * texel_height;
        let center = apply4(center, &light_orientation);
        let position = [
            sun[0] * distance + center[0],
            sun[1] * distance + center[1],
            sun[2] * distance + center[2],
        ];
        let mut inverse_view = super::volume::IDENTITY;
        look_at(&mut inverse_view, center, position, [0., 1., 0.]);
        inverse_view[12] = position[0];
        inverse_view[13] = position[1];
        inverse_view[14] = position[2];
        let inverse_projection = invert(&projection_matrix);
        let view = invert(&inverse_view);
        out.push(Cascade {
            interval: [if i == 0 { 0. } else { splits[i - 1] }, splits[i]],
            matrix: multiply(&projection_matrix, &view),
            inverse_matrix: multiply(&inverse_view, &inverse_projection),
        });
    }
    (far, out)
}

/// Math.round: halves toward +∞.
trait RoundJs {
    fn round_js(self) -> f64;
}
impl RoundJs for f64 {
    fn round_js(self) -> f64 {
        (self + 0.5).floor()
    }
}
