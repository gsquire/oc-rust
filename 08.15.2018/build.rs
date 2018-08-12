extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("include/whereami.c")
        .include("include/")
        .compile("libwhereami.a");

    let bindings = bindgen::Builder::default()
        .header("include/whereami.h")
        .generate()
        .expect("to generate the bindings");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("to write the bindings");
}
