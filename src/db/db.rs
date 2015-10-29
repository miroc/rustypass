use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::io;
use std::error::Error;
use secstr::SecStr;
use db::Entry;
use nacl::secretbox::{SecretKey, SecretMsg};
use rustc_serialize::base64::{self, FromBase64, ToBase64};
use rand::{ Rng, OsRng };
use crypto::bcrypt::bcrypt;
use serde_json;

static DEFAULT_DB_LOCATION: &'static str = "./rustypass.db";

const SALT_SIZE: usize = 16;
const PASS_SIZE: usize = 24;

pub struct Database {
    bcrypt_salt: [u8; SALT_SIZE],
    bcrypt_pass: [u8; PASS_SIZE],
    passwords: Vec<Entry>
}

impl Database {
    pub fn new(password: &str) -> Database {
        let mut salt = [0u8; SALT_SIZE]; // 16bytes of salt bcrypt
        let mut output = [0u8; PASS_SIZE]; // output 24 bytes
        OsRng::new().unwrap().fill_bytes(&mut salt);

        // TODO take only first 72 characters of input
        // TODO 10 iterations instead of 5
        bcrypt(5, &salt, password.as_bytes(), &mut output);
        // let salt_string = salt.to_base64(base64::STANDARD);
        // let pass_hash = output.to_base64(base64::STANDARD);

        Database {
            bcrypt_salt: salt,
            bcrypt_pass: output,
            passwords: Vec::new()
		}
    }

    pub fn open(password: &str) -> io::Result<Database> {
        // TODO do not generate salt, take one from
		//

        let path = Path::new(DEFAULT_DB_LOCATION);
        let display = path.display();
        let mut file = try!(File::open(path));

        let mut buffer = String::new();
        try!(file.read_to_string(&mut buffer));

        let deserialized: Vec<Entry> = serde_json::from_str(&buffer).unwrap();
        println!("deserialized: {:?}", deserialized);

        Ok(Database::new(password))
	}

    pub fn save(&self) -> io::Result<()>{
        // os independent path
        let path = Path::new(DEFAULT_DB_LOCATION);
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = try!(File::create(path));

        let serialized = serde_json::to_string(&self.passwords).unwrap();
        // encrypt
        let key = SecretKey::from_str(&self.get_pass());
        let enc: SecretMsg = key.encrypt(serialized.as_bytes());

        // write bytes
        try!(file.write(&self.bcrypt_pass));
        try!(file.flush());
        try!(file.write(&enc.cipher));
        try!(file.flush());

        Ok(())
        // match writeln!(file, "{}", serialized) {
        //     Err(why) => {panic!("couldn't write to {}: {}", display, Error::description(&why))},
        //     Ok(_) => {}//println!("successfully wrote to {}", display),
        // }
    }

    fn get_pass(&self) -> String{
        self.bcrypt_pass.to_base64(base64::STANDARD)
    }

    pub fn add(&mut self, entry: Entry){
        self.passwords.push(entry);
    }
}
