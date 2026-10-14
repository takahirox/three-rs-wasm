use three_tsl::reg::*;
use three_tsl::rt::*;

fn show(label: &str, r: R<Val>) -> Val {
    match r {
        Ok(v) => {
            println!("{label}: ok {:?}", v);
            v
        }
        Err(e) => {
            println!("{label}: err {} / {}", error_message(&e.0), display(&get(&e.0, "stack").unwrap()));
            Val::Undef
        }
    }
}

#[test]
fn probe() {
    let _ = three_tsl::three::order::init_all();
    let uvf = imp("nodes/accessors/UV.js", "uv");
    let uv = show("uv()", call(&uvf, &Val::Undef, &[]));
    let pv = imp("nodes/accessors/Position.js", "positionView");
    let q0 = show("q0", m(&pv, "dFdx", &[]));
    let q1 = show("q1", m(&pv, "dFdy", &[]));
    let st0 = show("st0", m(&uv, "dFdx", &[]));
    let n = imp("nodes/accessors/Normal.js", "normalView");
    let q1perp = show("q1perp", m(&q1, "cross", &[n.clone()]));
    let q0perp = show("q0perp", m(&n, "cross", &[q0.clone()]));
    let x = show("st0.x", get(&st0, "x"));
    let st1 = show("st1", m(&uv, "dFdy", &[]));
    let a = show("a", m(&q1perp, "mul", &[get(&st0, "x").unwrap()]));
    let b = show("b", m(&q0perp, "mul", &[get(&st1, "x").unwrap()]));
    let t = show("T", m(&a, "add", &[b]));
    let tt = show("T.dot", m(&t, "dot", &[t.clone()]));
    let det = show("det", m(&tt, "max", &[tt.clone()]));
    let eq = show("equal", m(&det, "equal", &[Val::Num(0.0)]));
    let inv = show("inverseSqrt", m(&det, "inverseSqrt", &[]));
    let scale = show("select", m(&eq, "select", &[Val::Num(0.0), inv]));
    let tm = show("T.mul", m(&t, "mul", &[scale]));
    let _ = show("toVar", m(&tm, "toVar", &[s("tangentViewFrame")]));
}
