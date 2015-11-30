use rpassword::read_password;
use db::{DatabaseInFile, Entry};
use std::io;
use std::io::Write;
use std::io::{Error, ErrorKind};

pub fn call(file_db: &mut Box<DatabaseInFile>){

	match read_inputs_to_entry() {
		Ok(entry) => {
			let title = entry.title.clone();
			file_db.db.add(entry);
			let res = file_db.save();
			match res {
				Err(why) => println!("Error while adding new entry, reason: {}.", why),
				_ => println!("New entry '{}' added.", title)
			}
		},
		Err(_) => {
			// TODO log problem
		}
	};
}

fn read_inputs_to_entry() -> Result<Entry, io::Error>{
	print!("Title: ");
	try!(io::stdout().flush());
	let mut input_title = String::new();
	try!(io::stdin().read_line(&mut input_title));
	if input_title.is_empty(){
		println!("Title cannot be empty.");
		return Err(Error::new(ErrorKind::Other, "Empty title"));
	}

	print!("Username: ");
	try!(io::stdout().flush());
	let mut input_username = String::new();
	try!(io::stdin().read_line(&mut input_username));
	if input_username.is_empty(){
		println!("Username cannot be empty.");
		return Err(Error::new(ErrorKind::Other, "Empty username"));
	}

	print!("Password: ");
	try!(io::stdout().flush());
	let input_password = try!(read_password());
	if input_password.is_empty(){
		println!("Password cannot be empty.");
		return Err(Error::new(ErrorKind::Other, "Empty password."));
	}

	print!("URL (optional): ");
	try!(io::stdout().flush());
	let mut input_url = String::new();
	try!(io::stdin().read_line(&mut input_url));

	return Ok(Entry::new(
			input_title.trim(),
			input_username.trim(),
			input_password.trim()
		));
}
