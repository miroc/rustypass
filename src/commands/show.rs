use rpassword::read_password;
use std::path::Path;
use std::fs::PathExt;
use db::{Database, DatabaseInFile, Entry};
use std::io;
use std::io::Write;

static USAGE: &'static str = "Invalid arguments.
Usage: rpass show <entry>";

pub fn call(file_db: &Box<DatabaseInFile>, params: &[&str]){
    if params.len() == 0 {
        println!("{}", USAGE);
        return;
    }

    let entry_title = params[0];
    match file_db.db.get(entry_title) {
        Some(entry) => entry.print_full_info(),
        None => println!("No entry named '{}' was found.", entry_title)
    }
}
