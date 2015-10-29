use std::path::Path;
use std::fs::File;
use std::io::{Write, Read, self, Error, ErrorKind};
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
// TODO 10 iterations instead of 5
const BCRYPT_COST: u32 = 5;

pub struct Database {
    bcrypt_salt: [u8; SALT_SIZE],
    bcrypt_pass: [u8; PASS_SIZE],
    entries: Vec<Entry>
}

impl Database {
    pub fn new(password: &str) -> Database {
        let mut salt = [0u8; SALT_SIZE]; // 16bytes of salt bcrypt
        let mut output = [0u8; PASS_SIZE]; // output 24 bytes
        OsRng::new().unwrap().fill_bytes(&mut salt);

        // TODO take only first 72 characters of input
        bcrypt(BCRYPT_COST, &salt, password.as_bytes(), &mut output);

        Database {
            bcrypt_salt: salt,
            bcrypt_pass: output,
            entries: Vec::new()
		}
    }

    pub fn open(password: &str) -> io::Result<Database> {
        let mut salt = [0u8; SALT_SIZE]; // 16bytes of salt bcrypt
        let mut output = [0u8; PASS_SIZE]; // output 24 bytes

        let mut f = try!(File::open(Path::new(DEFAULT_DB_LOCATION)));

        // Read salt
        match f.read(&mut salt){
            Ok(SALT_SIZE) => (),
            Ok(count) => return Err(
                Error::new(
                    ErrorKind::InvalidData,
                    format!("Bad number of bytes {} read for salt.", count)
                )
            ),
            Err(why) => return Err(why)
        }

        // Read the rest
        let mut buffer = Vec::new();
        try!(f.read_to_end(&mut buffer));

        // Run Bcrypt
        bcrypt(BCRYPT_COST, &salt, password.as_bytes(), &mut output);

        // Decrypt
        let secret =  match SecretMsg::from_bytes(&buffer) {
            Some(msg) => msg,
            None => return Err(
                Error::new(
                    ErrorKind::InvalidData, "Too few bytes (less than NONCE + ZERO bytes of SecretMsg)."
                    )
                )
        };

        let key = SecretKey::from_slice(&output);
        let dec = key.decrypt(&secret).unwrap();

        // Deserialize
        let deserialized: Vec<Entry> = serde_json::from_slice(&dec).unwrap();
        println!("deserialized: {:?}", deserialized);

        Ok(Database::new(password))
	}

    pub fn save(&self) -> io::Result<()>{
        // os independent path
        let path = Path::new(DEFAULT_DB_LOCATION);
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = try!(File::create(path));

        let serialized = serde_json::to_string(&self.entries).unwrap();
        // encrypt
        let key = SecretKey::from_slice(&self.bcrypt_pass);
        // let key = SecretKey::from_str(&self.get_pass());
        let enc: SecretMsg = key.encrypt(serialized.as_bytes());

        // write salt first
        try!(file.write(&self.bcrypt_salt));
        try!(file.flush());
        // write nonce + cipher directly (do not clone)
        try!(file.write(&enc.nonce));
        try!(file.flush());
        try!(file.write(&enc.cipher));
        try!(file.flush());

        Ok(())
    }

    fn get_pass(&self) -> String{
        self.bcrypt_pass.to_base64(base64::STANDARD)
    }

    pub fn add(&mut self, entry: Entry){
        self.entries.push(entry);
    }
}
