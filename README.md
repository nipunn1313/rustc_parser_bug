# rustc_parser_bug
Rustc parser bug

Seems to compile even w/ a missing paren

Repro steps

```
cargo test -- --nocapture
```

Check out for filing
https://github.com/rust-lang/rust/issues/58962
