//extern crate getopts;
use PassEntry;

static USAGE: &'static str = "Usage: rusty_pass add <title> <username> <password>"; 

fn usage(){
	println!("{}", USAGE);
}

pub fn call(params: &[String], passwords: &mut Vec<PassEntry>) {
	if params.len() != 3 {
		usage();	
	} else {
		passwords.push(
			PassEntry::new(
				params[0].as_ref(), 
				params[1].as_ref(),
				&params[2]
				//&params[2].into_bytes()
				)
		);		
	}	
	//println!("size of matches {}", 1);
}