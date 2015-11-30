use rpassword::read_password;
use std::path::Path;
use db::{Database, DatabaseInFile};

static USAGE: &'static str = "Invalid arguments.\n
Usage: rpass create <filename>";
static NEW_PASS: &'static str = "Please enter the master password:";
static NEW_PASS_CONFIRM: &'static str = "Confirm the password:";

const PASS_MIN_LENGTH: usize = 2;

fn usage(){
	println!("{}", USAGE);
}

pub fn call(params: &[String]) -> Option<Box<DatabaseInFile>>{
	if params.len() != 1 {
		usage();
		return None;
	} else {
		let db_path = Path::new(&params[0]);

		if db_path.exists(){
			println!("File at path '{}' already exists, cannot create new database.", &params[0]);
			return None;
		}

		let pass = get_pass();
		let database = Database::empty(pass.as_ref());

		return match database.save_to_file(&db_path) {
			Ok(_) => Some(Box::new(
				DatabaseInFile{
					db: database,
					filepath: params[0].clone()
				}
			)),
			Err(why) => {
				println!("Error creating file, reason: {}", why);
				None
			}
		}
	}
}

fn get_pass() -> String{
	loop {
		println!("Please enter new master password:");
		let password = read_password().unwrap();
		if password.len() < PASS_MIN_LENGTH {
			println!("Password is too short (has to be at least 10 characters).");
			continue;
		}

		println!("{}", NEW_PASS_CONFIRM);
		let password2 = read_password().unwrap();

		if password != password2 {
			println!("Passwords are not the same.");
			continue;
		}
		return password;
	}
}
