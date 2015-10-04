extern crate getopts;
use getopts::Matches;
use PassEntry;

static USAGE: &'static str = "Usage: rusty_pass add <title> <username> <password>"; 

fn usage(){
	println!("{}", USAGE);
}

pub fn call(matches: &Matches, passwords: &mut Vec<PassEntry>){
	passwords.push(PassEntry::new("Pokec.sk", "skaaj1", "secretsauce"));
	passwords.push(PassEntry::new("Pokec.sk", "skaaj2", "secretsauce"));
	passwords.push(PassEntry::new("Pokec.sk", "skaaj3", "secretsauce"));
	passwords.push(PassEntry::new("Pokec.sk", "skaaj4", "secretsauce"));
	//usage();
	//println!("size of matches {}", 1);
}