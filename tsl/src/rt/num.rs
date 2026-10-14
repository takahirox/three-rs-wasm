//! Number::toString ( ECMAScript 6.1.6.1.20, radix 10 ): the shortest digits
//! that round-trip, in decimal for exponents −7 < n ≤ 21 and exponential
//! otherwise, as `String( value )` and template literals print numbers.

/// The shortest round-trip digits and the decimal exponent: value = 0.d1d2… × 10^n.
fn digits(v: f64) -> (String, i32) {
    // `{:e}` prints the shortest round-trip mantissa: "d.ddde±x".
    let e = format!("{v:e}");
    let (mantissa, exp) = e.split_once('e').expect("exponent");
    let exp: i32 = exp.parse().expect("exponent digits");
    let digits: String = mantissa.chars().filter(char::is_ascii_digit).collect();
    let digits = digits.trim_end_matches('0');
    let digits = if digits.is_empty() { "0" } else { digits };
    (digits.to_string(), exp + 1)
}

pub fn to_string(v: f64) -> String {
    if v.is_nan() {
        return "NaN".into();
    }
    if v == 0.0 {
        return "0".into();
    }
    if v.is_infinite() {
        return if v > 0.0 { "Infinity" } else { "-Infinity" }.into();
    }
    if v < 0.0 {
        return format!("-{}", to_string(-v));
    }
    let (d, n) = digits(v);
    let k = d.len() as i32;
    if k <= n && n <= 21 {
        format!("{d}{}", "0".repeat((n - k) as usize))
    } else if 0 < n && n <= 21 {
        format!("{}.{}", &d[..n as usize], &d[n as usize..])
    } else if -6 < n && n <= 0 {
        format!("0.{}{d}", "0".repeat((-n) as usize))
    } else {
        let e = n - 1;
        let sign = if e < 0 { '-' } else { '+' };
        if k == 1 {
            format!("{d}e{sign}{}", e.abs())
        } else {
            format!("{}.{}e{sign}{}", &d[..1], &d[1..], e.abs())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::to_string;
    #[test]
    fn matches_javascript() {
        for (v, s) in [
            (0.7, "0.7"),
            (1.0, "1"),
            (6.283185307179586, "6.283185307179586"),
            (1e-6, "0.000001"),
            (1e-7, "1e-7"),
            (1.5e-7, "1.5e-7"),
            (1e21, "1e+21"),
            (1e20, "100000000000000000000"),
            (123456.789, "123456.789"),
            (-0.5, "-0.5"),
            (1.0 / 3.0, "0.3333333333333333"),
            (0.1 + 0.2, "0.30000000000000004"),
            (2.0f64.powi(53), "9007199254740992"),
            (5e-324, "5e-324"),
        ] {
            assert_eq!(to_string(v), s);
        }
    }
}
