extern crate cc;

fn main() {
    let cc = cc::Build::new()
        .try_get_compiler()
        .expect("failed to get ccompiler");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-env=CCOMPILER={}", cc.path().display());
}
