extern crate bindgen;
extern crate reqwest;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    // Fetch newest API.h from PROS repo
    let mut content = String::new();
    let mut resp = reqwest::get("https://raw.githubusercontent.com/purduesigbots/pros/master/include/API.h").expect("Failed to get newest API.h");
    resp.read_to_string(&mut content).expect("Failed to read newest API.h");
    let path = Path::new("API.h");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };
    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("Couldn't write {}: {}", display, why.description()),
        Ok(_) => println!("Successfully wrote to {}", display),
    };
    // Generate Rust bindings from API.h file
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
