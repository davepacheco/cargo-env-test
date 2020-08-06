#[test]
fn the_test()
{
    let value = env!("CARGO_BIN_EXE_cargo-env");
    println!("dap: {}", value);
    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }
}
