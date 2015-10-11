use std::env;   
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("gcc")
        .args(&["lib/tweetnacl/tweetnacl.c", "-c", "-fPIC", "-Wall", "-std=c99", "-o"])        
        .arg(&format!("{}/tweetnacl.o", out_dir))
        .status().unwrap();

    Command::new("ar")
        .args(&["rcs", "libtweetnacl.a", "tweetnacl.o"])        
        .current_dir(&Path::new(&out_dir))
        .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=tweetnacl");
}
