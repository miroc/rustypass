use std::env;   
use std::path::Path;
use std::process::Command;

// Bring in a dependency on an externally maintained `gcc` package which manages
// invoking the C compiler.
extern crate gcc;

fn main() {
    gcc::compile_library("libtweetnacl.a", &["lib/tweetnacl/tweetnacl.c"]);

/*
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("gcc")
        .args(&["lib/tweetnacl/tweetnacl.c", "-c", "-fPIC", "-Wall", "-std=c99", "-Ilib/tweetnacl/", "-o"])        
        .arg(&format!("{}/tweetnacl.o", out_dir))
        .status().unwrap();

    Command::new("ar")
        .args(&["rcs", "libtweetnacl.a", "tweetnacl.o"])        
        .current_dir(&Path::new(&out_dir))
        .status().unwrap();
    
    
    println!("cargo:rustc-link-search=native={}", out_dir);    
    //println!("cargo:rustc-link-lib=static=tweetnacl");
    */
    
    //println!("cargo:rustc-link-lib=tweetnacl");
    //println!("cargo:rustc-link-search=lib/tweetnacl/");
    //println!("cargo:include=lib/tweetnacl/tweetnacl.h");
    //println!("cargo:include=lib/tweetnacl/");
}

