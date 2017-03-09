extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .no_unstable_rust()
        .header("API.h")
        .generate()
        .expect("Unable to generate bindings!");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("pros.rs"))
        .expect("Couldn't write bindings!");
}
