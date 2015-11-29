use rpassword::read_password;
use std::path::Path;
use std::fs::PathExt;
use db::{Database, DatabaseInFile, Entry};
use std::io;
use std::io::Write;

static USAGE: &'static str = "Invalid arguments.
Usage: rpass edit <entry>";

fn usage(){
    println!("{}", USAGE);
}

pub fn call(file_db: &mut Box<DatabaseInFile>, params: &[&str]){
    if params.len() == 0 {
        usage();
        return;
    }

    let entry_title = params[0];
    match file_db.db.get(entry_title) {
        Some(entry) => {
            println!("Error: NOT IMPLEMENTED YET");
        },
        None => println!("No entry named '{}' was found.", entry_title)
    }
}
