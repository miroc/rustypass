#![allow(dead_code)] // TODO remove this later

mod secstr;
mod add;

extern crate getopts;

use getopts::Options;
use std::env;
use std::path::Path; 
use std::fs::File;
use std::io::Write;
use std::error::Error;
use secstr::SecStr;



// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
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
			password: SecStr::from(password.clone()) // TODO avoid cloning??
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

fn load_passwords(){

}

fn print_passwords(passwords: &Vec<PassEntry>){
    for pass in passwords {
        println!("pass entry {:?}", pass);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //let program = args[0].clone();
    
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
    
    // load passwords
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
        
        /*

    let pass1 = PassEntry::new("Pokec.sk", "skaaj", "secretsauce");
    // os independent path
    let path = Path::new("db.txt");
    let display = path.display();
    
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(pass1.title.as_bytes()) {
        Err(why) => {panic!("couldn't write to {}: {}", display, Error::description(&why))},
        Ok(_) => {}//println!("successfully wrote to {}", display),
    }
        */
    //println!("pass entry title {:?}", pass1);       
}
