fn main() {
    println!("cargo:rustc-link-search=native=./static");
    println!("cargo:rustc-link-lib=dylib=grasshopper");
    println!("cargo:rerun-if-changed=build.rs");
}
