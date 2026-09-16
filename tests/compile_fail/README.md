# Type-safety checks

Doctests exercise two meaningful guarantees: scene handles cannot be fabricated,
and a mutable scene borrow cannot coexist with a borrowed node. Both cases are
compiled as `compile_fail` tests by `cargo test --doc`; the valid API is exercised
by the regular scene tests.
