use rpassword::read_password;
use std::path::Path;
use std::fs::PathExt;
use db::{Database, DatabaseInFile, Entry};
use std::io;
use std::io::Write;

pub fn call(file_db: &Box<DatabaseInFile>, params: &[&str]){
    // TODO
}
