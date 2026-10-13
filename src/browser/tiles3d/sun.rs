//! @takram/three-atmosphere's getSunDirectionECEF( date ) with the
//! astronomy-engine functions it calls ( GeoVector of the Sun without
//! aberration, Rotation_EQJ_EQD, SiderealTime, Pivot ), in the host's
//! Math.sin / Math.cos as the reference evaluates them.
use super::vsop_earth::EARTH;

const DEG2RAD: f64 = 0.017453292519943296;
const ASEC2RAD: f64 = 4.848_136_811_095_36e-6;
const ASEC360: f64 = 1296000.;
const PI2: f64 = 2. * std::f64::consts::PI;
const DAYS_PER_TROPICAL_YEAR: f64 = 365.24217;
const DAYS_PER_MILLENNIUM: f64 = 365250.;

fn sin(x: f64) -> f64 {
    crate::bounce::trig::sin(x)
}
fn cos(x: f64) -> f64 {
    crate::bounce::trig::cos(x)
}

/// AstroTime: UT and TT in days since J2000 ( 2000-01-01T12:00:00Z ).
struct Time {
    ut: f64,
    tt: f64,
}

/// DeltaT_EspenakMeeus for the years this example reaches ( 2005..2050 ).
fn delta_t(ut: f64) -> f64 {
    let y = 2000. + ((ut - 14.) / DAYS_PER_TROPICAL_YEAR);
    if (2005. ..2050.).contains(&y) {
        let u = y - 2000.;
        return 62.92 + 0.32217 * u + 0.005589 * u * u;
    }
    if (1986. ..2005.).contains(&y) {
        let u = y - 2000.;
        let (u2, u3) = (u * u, u * u * u);
        let (u4, u5) = (u2 * u2, u2 * u3);
        return 63.86 + 0.3345 * u - 0.060374 * u2
            + 0.0017275 * u3
            + 0.000651814 * u4
            + 0.00002373599 * u5;
    }
    if (2050. ..2150.).contains(&y) {
        let u = (y - 1820.) / 100.;
        return -20. + 32. * u * u - 0.5628 * (2150. - y);
    }
    let u = (y - 1820.) / 100.;
    -20. + (32. * u * u)
}

impl Time {
    /// new AstroTime( date ) from a JS timestamp in milliseconds.
    fn from_millis(ms: f64) -> Self {
        // J2000.getTime().
        let ut = (ms - 946728000000.) / (1000. * 3600. * 24.);
        Time {
            ut,
            tt: ut + delta_t(ut) / 86400.,
        }
    }
}

type Rotation = [[f64; 3]; 3];

fn iau2000b(time: &Time) -> (f64, f64) {
    let m = |x: f64| (x % ASEC360) * ASEC2RAD;
    let t = time.tt / 36525.;
    let elp = m(1287104.79305 + t * 129596581.0481);
    let f = m(335779.526232 + t * 1739527262.8478);
    let d = m(1072260.70369 + t * 1602961601.2090);
    let om = m(450160.398036 - t * 6962890.5431);
    let (mut sarg, mut carg) = (sin(om), cos(om));
    let mut dp = (-172064161.0 - 174666.0 * t) * sarg + 33386.0 * carg;
    let mut de = (92052331.0 + 9086.0 * t) * carg + 15377.0 * sarg;
    let mut arg = 2.0 * (f - d + om);
    (sarg, carg) = (sin(arg), cos(arg));
    dp += (-13170906.0 - 1675.0 * t) * sarg - 13696.0 * carg;
    de += (5730336.0 - 3015.0 * t) * carg - 4587.0 * sarg;
    arg = 2.0 * (f + om);
    (sarg, carg) = (sin(arg), cos(arg));
    dp += (-2276413.0 - 234.0 * t) * sarg + 2796.0 * carg;
    de += (978459.0 - 485.0 * t) * carg + 1374.0 * sarg;
    arg = 2.0 * om;
    (sarg, carg) = (sin(arg), cos(arg));
    dp += (2074554.0 + 207.0 * t) * sarg - 698.0 * carg;
    de += (-897492.0 + 470.0 * t) * carg - 291.0 * sarg;
    (sarg, carg) = (sin(elp), cos(elp));
    dp += (1475877.0 - 3633.0 * t) * sarg + 11817.0 * carg;
    de += (73871.0 - 184.0 * t) * carg - 1924.0 * sarg;
    (-0.000135 + (dp * 1.0e-7), 0.000388 + (de * 1.0e-7))
}

fn mean_obliq(time: &Time) -> f64 {
    let t = time.tt / 36525.;
    let asec = ((((-0.0000000434 * t - 0.000000576) * t + 0.00200340) * t - 0.0001831) * t
        - 46.836769)
        * t
        + 84381.406;
    asec / 3600.0
}

/// e_tilt: ( dpsi, ee, mean obliquity, true obliquity ).
fn e_tilt(time: &Time) -> (f64, f64, f64, f64) {
    let (dpsi, deps) = iau2000b(time);
    let mobl = mean_obliq(time);
    (
        dpsi,
        dpsi * cos(mobl * DEG2RAD) / 15.,
        mobl,
        mobl + (deps / 3600.),
    )
}

/// precession_rot( time, From2000 ).
fn precession_rot(time: &Time) -> Rotation {
    let t = time.tt / 36525.;
    let mut eps0 = 84381.406;
    let mut psia = ((((-0.0000000951 * t + 0.000132851) * t - 0.00114045) * t - 1.0790069) * t
        + 5038.481507)
        * t;
    let mut omegaa =
        ((((0.0000003337 * t - 0.000000467) * t - 0.00772503) * t + 0.0512623) * t - 0.025754) * t
            + eps0;
    let mut chia = ((((-0.0000000560 * t + 0.000170663) * t - 0.00121197) * t - 2.3814292) * t
        + 10.556403)
        * t;
    eps0 *= ASEC2RAD;
    psia *= ASEC2RAD;
    omegaa *= ASEC2RAD;
    chia *= ASEC2RAD;
    let (sa, ca) = (sin(eps0), cos(eps0));
    let (sb, cb) = (sin(-psia), cos(-psia));
    let (sc, cc) = (sin(-omegaa), cos(-omegaa));
    let (sd, cd) = (sin(chia), cos(chia));
    let xx = cd * cb - sb * sd * cc;
    let yx = cd * sb * ca + sd * cc * cb * ca - sa * sd * sc;
    let zx = cd * sb * sa + sd * cc * cb * sa + ca * sd * sc;
    let xy = -sd * cb - sb * cd * cc;
    let yy = -sd * sb * ca + cd * cc * cb * ca - sa * cd * sc;
    let zy = -sd * sb * sa + cd * cc * cb * sa + ca * cd * sc;
    let xz = sb * sc;
    let yz = -sc * cb * ca - sa * cc;
    let zz = -sc * cb * sa + cc * ca;
    [[xx, xy, xz], [yx, yy, yz], [zx, zy, zz]]
}

/// nutation_rot( time, From2000 ).
fn nutation_rot(time: &Time) -> Rotation {
    let (dpsi, _, mobl, tobl) = e_tilt(time);
    let (oblm, oblt, psi) = (mobl * DEG2RAD, tobl * DEG2RAD, dpsi * ASEC2RAD);
    let (cobm, sobm) = (cos(oblm), sin(oblm));
    let (cobt, sobt) = (cos(oblt), sin(oblt));
    let (cpsi, spsi) = (cos(psi), sin(psi));
    let xx = cpsi;
    let yx = -spsi * cobm;
    let zx = -spsi * sobm;
    let xy = spsi * cobt;
    let yy = cpsi * cobm * cobt + sobm * sobt;
    let zy = cpsi * sobm * cobt - cobm * sobt;
    let xz = spsi * sobt;
    let yz = cpsi * cobm * sobt - sobm * cobt;
    let zz = cpsi * sobm * sobt + cobm * cobt;
    [[xx, xy, xz], [yx, yy, yz], [zx, zy, zz]]
}

/// CombineRotation( a, b ): b × a.
fn combine(a: &Rotation, b: &Rotation) -> Rotation {
    let mut c = [[0.; 3]; 3];
    for (i, row) in c.iter_mut().enumerate() {
        for (j, v) in row.iter_mut().enumerate() {
            *v = b[0][j] * a[i][0] + b[1][j] * a[i][1] + b[2][j] * a[i][2];
        }
    }
    c
}

/// Earth rotation angle.
fn era(time: &Time) -> f64 {
    let thet1 = 0.7790572732640 + 0.00273781191135448 * time.ut;
    let thet3 = time.ut % 1.;
    let mut theta = 360. * ((thet1 + thet3) % 1.);
    if theta < 0. {
        theta += 360.;
    }
    theta
}

/// sidereal_time: Greenwich apparent sidereal time in hours.
fn sidereal_time(time: &Time) -> f64 {
    let t = time.tt / 36525.;
    let eqeq = 15. * e_tilt(time).1;
    let theta = era(time);
    let st = eqeq
        + 0.014506
        + ((((-0.0000000368 * t - 0.000029956) * t - 0.00000044) * t + 1.3915817) * t
            + 4612.156534)
            * t;
    let mut gst = ((st / 3600. + theta) % 360.) / 15.;
    if gst < 0. {
        gst += 24.;
    }
    gst
}

/// Pivot( rotation, axis, angle in degrees ).
fn pivot(rotation: &Rotation, axis: usize, angle: f64) -> Rotation {
    let radians = angle * DEG2RAD;
    let (c, s) = (cos(radians), sin(radians));
    let (i, j, k) = ((axis + 1) % 3, (axis + 2) % 3, axis);
    let r = rotation;
    let mut rot = [[0.; 3]; 3];
    rot[i][i] = c * r[i][i] - s * r[i][j];
    rot[i][j] = s * r[i][i] + c * r[i][j];
    rot[i][k] = r[i][k];
    rot[j][i] = c * r[j][i] - s * r[j][j];
    rot[j][j] = s * r[j][i] + c * r[j][j];
    rot[j][k] = r[j][k];
    rot[k][i] = c * r[k][i] - s * r[k][j];
    rot[k][j] = s * r[k][i] + c * r[k][j];
    rot[k][k] = r[k][k];
    rot
}

fn vsop_formula(formula: &[&[[f64; 3]]], t: f64, clamp_angle: bool) -> f64 {
    let mut tpower = 1.;
    let mut coord = 0.;
    for series in formula {
        let mut sum = 0.;
        for [ampl, phas, freq] in series.iter() {
            sum += ampl * cos(phas + (t * freq));
        }
        let mut incr = tpower * sum;
        if clamp_angle {
            incr %= PI2;
        }
        coord += incr;
        tpower *= t;
    }
    coord
}

/// CalcVsop( vsop.Earth, time ): heliocentric J2000 equatorial, in AU.
fn earth(time: &Time) -> [f64; 3] {
    let t = time.tt / DAYS_PER_MILLENNIUM;
    let lon = vsop_formula(EARTH[0], t, true);
    let lat = vsop_formula(EARTH[1], t, false);
    let rad = vsop_formula(EARTH[2], t, false);
    let r_coslat = rad * cos(lat);
    let e = [r_coslat * cos(lon), r_coslat * sin(lon), rad * sin(lat)];
    // VsopRotate.
    [
        e[0] + 0.000000440360 * e[1] - 0.000000190919 * e[2],
        -0.000000479966 * e[0] + 0.917482137087 * e[1] - 0.397776982902 * e[2],
        0.397776982902 * e[1] + 0.917482137087 * e[2],
    ]
}

/// getSunDirectionECEF( new Date( ms ) ).
pub(crate) fn sun_direction_ecef(ms: f64) -> [f64; 3] {
    let time = Time::from_millis(ms);
    // GeoVector( Sun, time, false ): the Sun at the heliocentric origin seen
    // from the Earth ( the light-travel iteration leaves it unchanged ).
    let e = earth(&time);
    let v = [-e[0], -e[1], -e[2]];
    // Vector3.normalize: × ( 1 / length ).
    let inverse = 1. / (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    let v = v.map(|c| c * inverse);
    // getECIToECEFRotationMatrix: Pivot( Rotation_EQJ_EQD, 2, −15 × GAST ).
    let rot = pivot(
        &combine(&precession_rot(&time), &nutation_rot(&time)),
        2,
        -15. * sidereal_time(&time),
    );
    // fromAstroRotationMatrix, then applyMatrix4.
    [0, 1, 2].map(|r| rot[0][r] * v[0] + rot[1][r] * v[1] + rot[2][r] * v[2])
}

#[cfg(test)]
mod tests {
    #[test]
    fn sun_at_2024_03_01() {
        // Date.UTC( 2024, 2, 1 ): the reference's sunDirection uniform.
        let d = super::sun_direction_ecef(1709251200000.);
        let expected = [-0.990057542, -0.0532697231, -0.130186023];
        for k in 0..3 {
            assert!((d[k] - expected[k]).abs() < 1e-7, "{d:?}");
        }
    }
}
