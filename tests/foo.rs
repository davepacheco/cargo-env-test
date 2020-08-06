#[test]
fn the_test()
{
    let value = env!("CARGO_BIN_EXE_cargo-env");
    println!("compile-time: CARGO_BIN_EXE_cargo-env: {}", value);
    for (key, value) in std::env::vars() {
        if key.starts_with("CARGO_") {
            println!("runtime: {}: {}", key, value);
        }
    }
}
