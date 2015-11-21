#![feature(custom_derive, plugin)]
#![feature(path_ext_deprecated)] // for Path.exists()
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;

extern crate libc;
extern crate getopts;
extern crate rand;
extern crate crypto;
extern crate rpassword;

use getopts::Options;
use std::env;
use std::error::Error;
use secstr::SecStr;
use db::{Database, Entry};

mod secstr;
mod texts;
mod commands;
mod db;
mod nacl; // bindings to tweetnacl crypto library

fn usage(){
    println!("{}", texts::USAGE);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") || matches.free.is_empty(){
        usage();
        return;
    }


    let command = matches.free.get(0);
    // //
    match command {
         Some(value) => match value.as_ref() {
            "new" => {
                commands::new::call(&matches.free[1..]);
            },
            "open" => {
            },
            _ => println!("unknown command '{}'", value),
        },
        None => panic!("no command!"),
    }
}
