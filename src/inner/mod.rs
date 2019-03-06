pub fn f<F: FnOnce(u64) -> u64>(f1: F) -> u64 {
        f1(1)
}

#[test]
fn test_weird_rustc_thing() {
    let y = f({
        |x| x+1
    ); // shouldn't this need a closing curly brace?
    assert_eq!(y, 2);
    println!("WEIRD RUSTC BUG");
}
