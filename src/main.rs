#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;

extern crate libc;
extern crate getopts;
extern crate rand;
extern crate crypto;

use getopts::Options;
use std::env;
use std::error::Error;
use secstr::SecStr;
use db::{Database, Entry};

mod secstr;
mod add;
mod db;
mod nacl; // bindings to tweetnacl crypto library


static USAGE: &'static str = "Usage: rusty_pass [OPTIONS] COMMAND [arg...]

MultiPass is keeping your passwords safe, while:
    * being memory safe by default (unlike C/C++)
    * having no runtime (unlike java, C#, go)
    * no interpreted code (unlike python, ruby)
    * (TODO) compatible with .kdbx format used by keepass2

Options:
    -h, --help\t Show this help

Commands:
    add \t Add new password
    get \t Get password
    nsa \t Send password to NSA
    ";

fn usage(){
    println!("{}", USAGE);
}

fn main() {
    // let mut s = SecStr::new("wakalaka".to_string());
    // s.unlock();
    // s.delete();
    // println!("{}", &s.string);


    // let args: Vec<String> = env::args().collect();
    // //let program = args[0].clone();
    //
    // let pass = "sweet_";

    //
    // println!("password {:?}, saltstring {:?}", pass_hash, salt_string);
    //
    // let key = SecretKey::from_str(&salt_string);
    // let enc: SecretMsg = key.encrypt("abc".as_bytes());
    // let decr_opt = key.decrypt(&enc);
    // println!("decrypted: {:?}", decr_opt.unwrap());
    //
    //
    // let mut opts = Options::new();
    // opts.optflag("h", "help", "print this help menu");
    // let matches = match opts.parse(&args[1..]) {
    //     Ok(m) => { m }
    //     Err(f) => { panic!(f.to_string()) }
    // };
    //
    // if matches.opt_present("h") || matches.free.is_empty(){
    //     usage();
    //     return;
    // }
    //
    // let mut passwords: Vec<PassEntry> = Vec::new();
    //
    // let command = matches.free.get(0);
    //
    // match command {
    //     Some(value) => match value.as_ref() {
    //         "add" => {
    //             add::call(&matches.free[1..], &mut passwords);
    //         },
    //         "get" => {
    //
    //         },
    //         _ => println!("unknown command '{}'", value),
    //     },
    //     None => panic!("no command!"),
    // }
    //
    // print_passwords(&passwords);
    // db::save_passwords(&passwords);
}
