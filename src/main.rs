#![feature(custom_derive, plugin)]
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
use std::io;
use std::io::Write;
use db::{DatabaseInFile};

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
    match command {
         Some(value) => match value.as_ref() {
            "create" => {
                let opt_db = commands::create::call(&matches.free[1..]);
                if opt_db.is_some(){
                    println!("Database successfully created.");
                    command_loop(opt_db.unwrap());
                }
            },
            "open" => {
                let opt_db = commands::open::call(&matches.free[1..]);
                if opt_db.is_some(){
                    println!("Database successfully opened.");
                    command_loop(opt_db.unwrap());
                }
            },
            _ => {
                println!("Unknown command '{}'", value);
                usage()
            }
        },
        None => panic!("No command!"),
    }
}

fn command_loop(mut file_db: Box<DatabaseInFile>){
    print_db_commands();

    loop {
        let mut input = String::new();
        print!("rpass> ");
        io::stdout().flush().unwrap();
        let res = io::stdin().read_line(&mut input);
        if res.is_err(){
            println!("Error reading input, terminating");
            return;
        }

        let words: Vec<&str> = input.split_whitespace().collect();

        match words[0] {
            "list" => commands::list::call(&file_db),
            "new" => commands::new::call(&mut file_db),
            "show" => commands::show::call(&file_db, &words[1..]),
            "copy" => commands::copy::call(&file_db, &words[1..]),
            "edit" => commands::edit::call(&mut file_db, &words[1..]),
            "remove" => commands::remove::call(&mut file_db, &words[1..]),
            _ => print_db_commands()
        }
    }
}

fn print_db_commands(){
    println!("{}", texts::DB_COMMANDS);
}
