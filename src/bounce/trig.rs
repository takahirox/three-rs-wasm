//! `Math.sin` / `Math.cos` as the reference runtime computes them.
//!
//! The physics state feeds orientations through these calls, and browsers
//! differ in the last bit: Chrome's V8 uses LLVM libc's routines, which are
//! not correctly rounded for every input and whose results depend on the
//! host CPU's FMA support. In the browser the port therefore calls the host's
//! own `Math.sin` / `Math.cos`, so it reproduces the original bit for bit on
//! every engine. The native build ( unit tests ) uses correctly rounded
//! sin / cos, which agree with Chrome on all but a few inputs per million.

#[cfg(target_arch = "wasm32")]
pub fn sin(x: f64) -> f64 {
    js_sys::Math::sin(x)
}

#[cfg(target_arch = "wasm32")]
pub fn cos(x: f64) -> f64 {
    js_sys::Math::cos(x)
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::{cos, sin};

#[cfg(not(target_arch = "wasm32"))]
mod native {
    //! Argument reduction by π/2 and the Taylor series in double-double
    //! arithmetic ( about 100 bits ), rounded once.

    #[derive(Clone, Copy)]
    struct Dd(f64, f64);

    fn two_sum(a: f64, b: f64) -> Dd {
        let s = a + b;
        let bb = s - a;
        let e = (a - (s - bb)) + (b - bb);
        Dd(s, e)
    }

    fn quick_two_sum(a: f64, b: f64) -> Dd {
        let s = a + b;
        Dd(s, b - (s - a))
    }

    /// Dekker's exact product through Veltkamp splitting.
    fn two_prod(a: f64, b: f64) -> Dd {
        let p = a * b;
        let split = |v: f64| {
            let t = 134217729. * v;
            let hi = t - (t - v);
            (hi, v - hi)
        };
        let (ah, al) = split(a);
        let (bh, bl) = split(b);
        let e = ((ah * bh - p) + ah * bl + al * bh) + al * bl;
        Dd(p, e)
    }

    impl Dd {
        fn add(self, o: Dd) -> Dd {
            let s = two_sum(self.0, o.0);
            let t = two_sum(self.1, o.1);
            let s = quick_two_sum(s.0, s.1 + t.0);
            quick_two_sum(s.0, s.1 + t.1)
        }
        fn neg(self) -> Dd {
            Dd(-self.0, -self.1)
        }
        fn mul(self, o: Dd) -> Dd {
            let p = two_prod(self.0, o.0);
            quick_two_sum(p.0, p.1 + (self.0 * o.1 + self.1 * o.0))
        }
        fn div_f(self, d: f64) -> Dd {
            let q1 = self.0 / d;
            let p = two_prod(q1, d);
            let r = two_sum(self.0, -p.0);
            let r1 = r.1 + self.1 - p.1;
            let q2 = (r.0 + r1) / d;
            quick_two_sum(q1, q2)
        }
    }

    /// π/2 in four 33-bit parts ( fdlibm's pio2_1, pio2_2, pio2_3, pio2_3t ).
    const PIO2: [f64; 4] = [
        f64::from_bits(0x3ff921fb54400000),
        f64::from_bits(0x3dd0b4611a600000),
        f64::from_bits(0x3ba3198a2e000000),
        f64::from_bits(0x397b839a252049c1),
    ];

    /// ( quadrant, x − k π/2 ) for |x| below 2^20 π/2.
    fn reduce(x: f64) -> (i64, Dd) {
        let k = (x * std::f64::consts::FRAC_2_PI).round();
        let mut r = Dd(x, 0.);
        for part in PIO2 {
            r = r.add(two_prod(k, part).neg());
        }
        (k as i64, r)
    }

    /// sin and cos of a reduced argument ( |r| ≤ π/4 ) by their Taylor series.
    fn sin_cos(r: Dd) -> (Dd, Dd) {
        let r2 = r.mul(r);
        // sin r = r ( 1 − r²/3! ( 1 − r²/(4·5) ( 1 − ... ) ) )
        let mut s = Dd(1., 0.);
        let mut n = 31.;
        while n > 1. {
            let term = r2.div_f((n - 1.) * n);
            s = Dd(1., 0.).add(term.mul(s).neg());
            n -= 2.;
        }
        let sin = r.mul(s);
        let mut c = Dd(1., 0.);
        let mut n = 32.;
        while n > 0. {
            let term = r2.div_f((n - 1.) * n);
            c = Dd(1., 0.).add(term.mul(c).neg());
            n -= 2.;
        }
        (sin, c)
    }

    fn round(d: Dd) -> f64 {
        d.0 + d.1
    }

    pub fn sin(x: f64) -> f64 {
        if x == 0. || !x.is_finite() {
            return if x.is_finite() { x } else { f64::NAN };
        }
        if x.abs() > 1.6e6 {
            return libm::sin(x);
        }
        let (k, r) = reduce(x);
        let (s, c) = sin_cos(r);
        round(match k.rem_euclid(4) {
            0 => s,
            1 => c,
            2 => s.neg(),
            _ => c.neg(),
        })
    }

    pub fn cos(x: f64) -> f64 {
        if !x.is_finite() {
            return f64::NAN;
        }
        if x == 0. {
            return 1.;
        }
        if x.abs() > 1.6e6 {
            return libm::cos(x);
        }
        let (k, r) = reduce(x);
        let (s, c) = sin_cos(r);
        round(match k.rem_euclid(4) {
            0 => c,
            1 => s.neg(),
            2 => c.neg(),
            _ => s,
        })
    }
}
