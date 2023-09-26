extern crate bindgen;
extern crate cmake;

use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let bindings = bindgen::Builder::default()
        .clang_arg("-Ivendor/enet/include/")
        .header("wrapper.h")
        .derive_debug(false)
        .blocklist_type("ENetPacket")
        .blocklist_type("_ENetPacket")
        .blocklist_type("_?P?IMAGE_TLS_DIRECTORY.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let dst = Config::new("vendor/enet").build();

    eprintln!("LUL: {}", dst.display());

    if target.contains("windows") {
        println!("cargo:rustc-link-lib=dylib=winmm");
    }

    println!("cargo:rustc-link-search=native={}/lib/static", dst.display());
    println!("cargo:rustc-link-lib=static=enet");
}
