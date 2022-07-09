fn main() {
    // link X11 C bindings to rust project
    println!("cargo:rustc-link-lib=X11");
}
