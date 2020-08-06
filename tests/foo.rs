#[test]
fn the_test()
{
    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }
}
