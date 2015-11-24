use rpassword::read_password;
use std::path::Path;
use std::fs::PathExt;
use db::{Database, DatabaseInFile, Entry};

pub fn call(file_db: &Box<DatabaseInFile>){
	// Print table header
	Entry::print_short_info_desc();
	// Print table rows
	for entry in file_db.db.entries.iter(){
		entry.print_short_info();
	}
}
