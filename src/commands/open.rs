use db::{Database, Entry, DatabaseInFile};
use std::path::Path;
use std::fs::PathExt;
use rpassword::read_password;

static USAGE: &'static str = "Usage: rpass open <filename>";

fn usage(){
	println!("{}", USAGE);
}

pub fn call(params: &[String]) -> Option<Box<DatabaseInFile>>{
	let db_path = Path::new(&params[0]);
	if !db_path.exists(){
		println!("No such file exists.");
		return None;
	}

	println!("Please enter master password:");
	let password = read_password().unwrap();

	let res = Database::open_from_file(&db_path, &password);
	match res {
		Ok(database) => Some(Box::new(
			DatabaseInFile{
				db: database,
				filepath: params[0].clone()
			}
		)),
		Err(why) => {
			println!("Error opening file, reason: {}", why);
			None
		}
	}
}
