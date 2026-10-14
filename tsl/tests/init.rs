#[test]
fn init_all_modules() {
    if let Err(e) = three_tsl::three::order::init_all() {
        let stack = three_tsl::rt::get(&e.0, "stack").map(|s| three_tsl::rt::display(&s)).unwrap_or_default();
        panic!("init failed: {}\n{stack}", three_tsl::rt::error_message(&e.0));
    }
}
