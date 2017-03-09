# rusty-pros
expose [PROS](https://github.com/purduesigbots/pros) API to rust using [servo/rust-bindgen](https://github.com/servo/rust-bindgen)

# get bindings
currently, you have to run `cargo build` to generate the bindings which are written to `$OUT_DIR/pros.rs` (`$OUT_DIR` is a cargo environment variable).

# lib.rs
```rust
// stop the compiler from complaining about
// generated code and/or code that uses different naming conventions
!#[allow(non_upper_case_globals)]
!#[allow(non_camel_case_types)]
!#[allow(non_snake_case)]

// actually include the generated bindings (also runs some tests)
include!(concat!(env!("OUT_DIR", "/pros.rs"));
```
eventually, this will wrap the unsafe calls to the API and expose them in a nice way. also eventually, this will be distributed nicely (pre-generated bindings and all that).
