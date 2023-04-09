use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let pwd = env::var("CARGO_MANIFEST_DIR").unwrap();
    let c_lib_path = Path::new(&pwd).join("C").to_str().unwrap().to_owned();

    Command::new("make")
        .arg("-C")
        .arg(&c_lib_path)
        .spawn()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", &c_lib_path);
    println!("cargo:rustc-link-lib=static=blurhash");
}

