fn main() {
    let t = std::env::var("TARGET").unwrap();
    println!("cargo:rustc-link-search=lib/{}", t);

    println!("cargo:rustc-link-lib=static=enet");
}
