fn main() {
    let t = std::env::var("TARGET").unwrap();
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    match &*t {
        "x86_64-pc-windows-gnu" => println!("cargo:rustc-link-lib=static=winmm"),
        "i686-pc-windows-gnu" => println!("cargo:rustc-link-lib=static=winmm"),
        "x86_64-unknown-linux-gnu" => {},
        //"i686-unknown-linux-gnu" => {},
        _ => panic!("Target not supported."),
    }
    println!("cargo:rustc-link-search={}/lib/{}", dir, t);
    println!("cargo:rustc-link-lib=static=enet");
}
