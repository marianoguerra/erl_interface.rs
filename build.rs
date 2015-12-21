use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:outdir={}", out_dir);
    println!("cargo:rustc-link-lib=erl_interface");
    println!("cargo:rustc-link-lib=ei");
}
