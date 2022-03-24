
fn main() {
    let _build = cxx_build::bridge("src/lib.rs").compile("cxx_demo");
    println!("cargo:rerun-if-changed=src/lib.rs");

}
