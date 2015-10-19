//#![allow(dead_code)] // TODO remove this later
#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate libc;
extern crate serde;
extern crate serde_json;
extern crate getopts;
extern crate rand;
extern crate crypto;
extern crate rustc_serialize;

use getopts::Options;
use std::env;
use secstr::SecStr;
use nacl::secretbox::{SecretKey, SecretMsg};
use rand::{ Rng, OsRng };
use crypto::bcrypt::bcrypt;
use crypto::pbkdf2::pbkdf2_simple;

use rustc_serialize::base64::{self, FromBase64, ToBase64};

mod secstr;
mod add;
mod db;
mod nacl; // bindings to tweetnacl crypto library

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Serialize, Deserialize, Debug)]
pub struct PassEntry {
    title: String,
    username: String,
    password: SecStr
}

impl PassEntry {
    //&str. This is a reference to another string
	// no self -- associated function
	//fn new(title: &str, username: &str, password: &str) -> PassEntry {
    fn new<S: Into<String>>(title: S, username: S, password: &String) -> PassEntry {
        //let () = password;
		PassEntry {
			title: title.into(),
			username: username.into(),
			password: SecStr::from(password.clone()) // todo avoid cloning??
		}
	}
}

static USAGE: &'static str = "Usage: rusty_pass [OPTIONS] COMMAND [arg...]

RustyPass is keeping your passwords safe, while:
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

fn print_passwords(passwords: &Vec<PassEntry>){
    for pass in passwords {
        println!("pass entry {:?}", pass);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    //let program = args[0].clone();

    let pass = "sweet_";
    let mut salt = [0u8; 16]; // 16bytes of salt bcrypt
    let mut output = [0u8; 24]; // output 24 bytes

    OsRng::new().unwrap().fill_bytes(&mut salt);

    // TODO take only first 72 characters of input
    bcrypt(10, &salt, pass.as_bytes(), &mut output);
    let salt_string = salt.to_base64(base64::STANDARD);
    let pass_hash = output.to_base64(base64::STANDARD);

    println!("password {:?}, saltstring {:?}", pass_hash, salt_string);

    let key = SecretKey::from_str(&salt_string);
    let enc: SecretMsg = key.encrypt("abc".as_bytes());

    let decr_opt = key.decrypt(&enc);
    println!("decrypted: {:?}", decr_opt.unwrap());


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

    let mut passwords: Vec<PassEntry> = Vec::new();

    let command = matches.free.get(0);

    match command {
        Some(value) => match value.as_ref() {
            "add" => {
                add::call(&matches.free[1..], &mut passwords);
            },
            "get" => {

            },
            _ => println!("unknown command '{}'", value),
        },
        None => panic!("no command!"),
    }

    print_passwords(&passwords);
    db::save_passwords(&passwords);
}
