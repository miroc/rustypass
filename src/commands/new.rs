use rpassword::read_password;
use std::path::Path;
use std::fs::PathExt;
use db::{Database, DatabaseInFile, Entry};
use std::io;
use std::io::Write;

pub fn call(file_db: &mut Box<DatabaseInFile>, params: &[&str]){
	let mut input_title = String::new();
	let mut input_username = String::new();
	let mut input_url = String::new();
	// let mut input_password = String::new();

	print!("Title: ");
	io::stdout().flush();
	io::stdin().read_line(&mut input_title);
	if input_title.is_empty(){
		println!("Title cannot be empty.");
		return;
	}

	print!("Username: ");
	io::stdout().flush();
	io::stdin().read_line(&mut input_username);
	if input_username.is_empty(){
		println!("Username cannot be empty.");
		return;
	}

	print!("Password: ");
	io::stdout().flush();
	let input_password = read_password().unwrap();
	if input_password.is_empty(){
		println!("Password cannot be empty.");
		return;
	}

	print!("URL (optional): ");
	io::stdout().flush();
	let mut input_url = String::new();
	io::stdin().read_line(&mut input_url);

	file_db.db.add(
		Entry::new(
			input_title.trim(),
			input_username.trim(),
			input_password.trim()
		)
	);

	let res = file_db.save();
	match res {
		Err(why) => println!("Error while adding new entry, reason: {}.", why),
		_ => println!("New entry '{}' added.", input_title.trim())
	}
}
