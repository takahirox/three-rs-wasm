//! Shorthands for writing the JavaScript sources' expressions over `rt`.

/// `v.key`.
#[macro_export]
macro_rules! g {
    ($v:expr, $k:expr) => {
        $crate::rt::get(&$v, $k)?
    };
}

/// `v.key = value`.
#[macro_export]
macro_rules! st {
    ($v:expr, $k:expr, $val:expr) => {
        $crate::rt::set(&$v, $k, $crate::rt::Val::from($val))?
    };
}

/// `v.method( …args )`.
#[macro_export]
macro_rules! c {
    ($v:expr, $k:expr $(, $a:expr)* $(,)?) => {
        $crate::rt::m_site(&$v, $k, &[$($crate::rt::Val::from($a)),*], stringify!($v))?
    };
}

/// `new C( …args )`.
#[macro_export]
macro_rules! new {
    ($cls:expr $(, $a:expr)* $(,)?) => {
        $crate::rt::construct(&$cls, &[$($crate::rt::Val::from($a)),*])?
    };
}

/// `f( …args )` for a function value.
#[macro_export]
macro_rules! f {
    ($fv:expr $(, $a:expr)* $(,)?) => {
        $crate::rt::call_site(&$fv, &[$($crate::rt::Val::from($a)),*], stringify!($fv))?
    };
}

/// `[ …items ]`.
#[macro_export]
macro_rules! arr {
    ($($a:expr),* $(,)?) => {
        $crate::rt::array(vec![$($crate::rt::Val::from($a)),*])
    };
}

/// `{ key: value, … }`.
#[macro_export]
macro_rules! obj {
    ($($k:expr => $v:expr),* $(,)?) => {
        $crate::rt::object(&[$(($k, $crate::rt::Val::from($v))),*])
    };
}

/// `for ( const x of v )`.
#[macro_export]
macro_rules! iter {
    ($v:expr) => {
        $crate::rt::iterate_site(&$v, stringify!($v))?
    };
}
