use three_tsl::reg::imp;
use three_tsl::rt::*;

#[test]
fn vector2() {
    three_tsl::three::order::init_all().unwrap();
    let v2 = imp("math/Vector2.js", "Vector2");
    let v = construct(&v2, &[3.0.into(), 4.0.into()]).unwrap();
    assert_eq!(to_number(&m(&v, "length", &[]).unwrap()), 5.0);
    let w = m(&v, "clone", &[]).unwrap();
    m(&w, "normalize", &[]).unwrap();
    assert_eq!(display(&get(&w, "x").unwrap()), "0.6000000000000001"); // 3 * ( 1 / 5 ), as in JavaScript
    assert_eq!(display(&get(&v, "width").unwrap()), "3");
    set(&v, "height", Val::Num(7.0)).unwrap();
    assert_eq!(display(&get(&v, "y").unwrap()), "7");
    let items: Vec<String> = iterate(&v).unwrap().iter().map(display).collect();
    assert_eq!(items, ["3", "7"]);
    assert!(get(&v, "isVector2").unwrap().truthy());
    let e = m(&v, "setComponent", &[5.0.into(), 1.0.into()]).unwrap_err();
    assert_eq!(error_message(&e.0), "THREE.Vector2: index is out of range: 5");
    let e = m(&Val::Undef, "x", &[]).unwrap_err();
    assert_eq!(error_message(&e.0), "Cannot read properties of undefined (reading 'x')");
}
