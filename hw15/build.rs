extern crate cbindgen;

//use std::{env, path::Path};
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config: cbindgen::Config = Default::default();
    config.language = cbindgen::Language::C;
    cbindgen::generate_with_config(crate_dir, config)
        .unwrap()
        .write_to_file("c_src/smartsocket.h");

    // let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    // let build_type = env::var("PROFILE").unwrap();
    // let input_path = Path::new(&manifest_dir_string)
    //     .join("target")
    //     .join(build_type)
    //     .join("libsmartsocket.so");
    // let output_path =
    //     Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("c_src/libsmartsocket.so");
    // std::fs::copy(input_path, output_path).unwrap();
}
