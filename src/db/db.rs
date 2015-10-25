use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::error::Error;
use secstr::SecStr;
use db::Entry;
use nacl::secretbox::{SecretKey, SecretMsg};
use rand::{ Rng, OsRng };
use crypto::bcrypt::bcrypt;

static DEFAULT_DB_LOCATION: &'static str = "./rustypass.db";

pub struct Database {
    bcrypt_salt: [u8; 16],
    bcrypt_pass: [u8; 24],
    passwords: Vec<Entry>
}

impl Database {
    pub fn new(password: &str) -> Database {
        let mut salt = [0u8; 16]; // 16bytes of salt bcrypt
        let mut output = [0u8; 24]; // output 24 bytes
        OsRng::new().unwrap().fill_bytes(&mut salt);

        // TODO take only first 72 characters of input
        bcrypt(10, &salt, password.as_bytes(), &mut output);
        // let salt_string = salt.to_base64(base64::STANDARD);
        // let pass_hash = output.to_base64(base64::STANDARD);

        Database {
            bcrypt_salt: salt,
            bcrypt_pass: output,
            passwords: Vec::new()
		}
    }

    pub fn open(password: &str) -> Database {
        // TODO do not generate salt, take one from
		return Database::new(password);
	}

    pub fn save(&self){
        // os independent path
        let path = Path::new(DEFAULT_DB_LOCATION);
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, Error::description(&why)),
            Ok(file) => file,
        };
        println!("db successfully saved");

        // let serialized = serde_json::to_string(passwords).unwrap();
        // match writeln!(file, "{}", serialized) {
        //     Err(why) => {panic!("couldn't write to {}: {}", display, Error::description(&why))},
        //     Ok(_) => {}//println!("successfully wrote to {}", display),
        // }
    }

    pub fn add(&mut self, entry: Entry){
        self.passwords.push(entry);
    }
}
