# rustc_parser_bug
Rustc parser bug

Seems to compile even w/ a missing paren

Repro steps

```
cargo test -- --nocapture
```

Output
```
nipunn-mbp:weird_rustc_bug nipunn$ cargo test -- --nocapture
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running target/debug/deps/weird_rustc_bug-73865e7743d3a87d

running 1 test
WEIRD RUSTC BUG
test inner::test_weird_rustc_thing ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests weird_rustc_bug

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Check out for filing
https://github.com/rust-lang/rust/issues/58962
