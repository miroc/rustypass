use rpassword::read_password;
use std::path::Path;
use std::fs::PathExt;
use db::Database;

static USAGE: &'static str = "Invalid arguments.\n
Usage: rpass new <filename>";
static NEW_PASS: &'static str = "Please enter the master password:";
static NEW_PASS_CONFIRM: &'static str = "Confirm the password:";

fn usage(){
	println!("{}", USAGE);
}

pub fn call(params: &[String]) {
	if params.len() != 1 {
		usage();
	} else {
		let db_path = Path::new(&params[0]);

		if db_path.exists(){
			println!("File already exists, cannot create database.");
			return;
		}

		let pass = get_pass();
		println!("The password is: '{}'", pass);
		let db = Database::empty(pass.as_ref());
		db.save_to_file(&db_path);

		// println!("new database created, file name {}", params[0]);
		// db.add(
			// Entry::new(
				// params[0].as_ref(),
				// params[1].as_ref(),
				// &params[2]
				//&params[2].into_bytes()
				// )
		// );
	}
	//println!("size of matches {}", 1);
}

fn get_pass() -> String{
	loop {
		println!("Please enter new master password:");
		let password = read_password().unwrap();
		if password.len() < 10 {
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
