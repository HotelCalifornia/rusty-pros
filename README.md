# rusty-pros [(very) WIP]
expose [purduesigbots/pros](https://github.com/purduesigbots/pros) API to rust using [servo/rust-bindgen](https://github.com/servo/rust-bindgen)

# get bindings
currently, you have to run `cargo build` to generate the bindings which are written to `$OUT_DIR/pros.rs` (`$OUT_DIR` is a cargo environment variable).
## step-by-step
1. make sure you have Clang 3.9+ installed
2. run `cargo build`
   cargo will start downloading and compiling rust-bindgen and its dependencies for you
3. run `cargo test` to make sure everything works (TODO @myself: write some tests maybe?)

(I did the more complicated stuff already)

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

# note
I haven't tested this at all, and I'm still working on getting safe wrapper functions for people to call. only use this if you really know what you're doing (unlike me), otherwise just wait until I've tested it and made sure it works properly with the PROS ecosystem
