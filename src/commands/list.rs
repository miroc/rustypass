use rpassword::read_password;
use std::path::Path;
use std::fs::PathExt;
use db::Database;
use db::DatabaseInFile;


pub fn call(file_db: &Box<DatabaseInFile>){
	println!("{:?}", file_db.db.entries);
}
