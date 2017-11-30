extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    env::set_var("TARGET_CC", "arm-none-eabi-gcc");

    let bindings = bindgen::Builder::default()
        .header("gen/tiles.h")
        .ctypes_prefix("::ctypes")
        .use_core()
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings");
}
