use std::path::Path;
use std::fs::File;
use std::io::{Write, Read, self, Error, ErrorKind};
use secstr::SecStr;
use db::Entry;
use nacl::secretbox::{SecretKey, SecretMsg};
use rand::{ Rng, OsRng };
use crypto::bcrypt::bcrypt;
use serde_json;

static DEFAULT_DB_LOCATION: &'static str = "./rustypass.db";

const DB_VERSION: u8 = 1u8; // first version
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

    pub fn open_from_file(password: &str) -> io::Result<Database> {
        let mut file = try!(File::open(Path::new(DEFAULT_DB_LOCATION)));
        Database::open(password, file)
    }

    pub fn save_to_file(&self) -> io::Result<()> {
        let path = Path::new(DEFAULT_DB_LOCATION);
        let display = path.display();
        // Open the file in write-only mode
        let mut file = try!(File::create(path));
        self.save(file)
    }

    fn open<T: Read>(password: &str, mut src: T) -> io::Result<Database> {
        let mut salt = [0u8; SALT_SIZE]; // 16bytes of salt bcrypt
        let mut output = [0u8; PASS_SIZE]; // output 24 bytes
        let mut version_buffer = [0u8; 1];

        match src.read(&mut version_buffer){
            Ok(size) => (),
            Err(why) => return Err(why)
        };
        if version_buffer[0] != DB_VERSION {
            return Database::invalid_data_error(format!("Cannot process DB version {}", version_buffer[0]));
        }

        match src.read(&mut salt){
            Ok(SALT_SIZE) => (),
            Ok(count) => return Database::invalid_data_error(format!("Bad number of bytes {} read for salt.", count)),
            Err(why) => return Err(why)
        }

        // Read the rest
        let mut buffer = Vec::new();
        try!(src.read_to_end(&mut buffer));

        // Run Bcrypt
        bcrypt(BCRYPT_COST, &salt, password.as_bytes(), &mut output);

        // Decrypt
        let secret =  match SecretMsg::from_bytes(&buffer) {
            Some(msg) => msg,
            None => return Database::invalid_data_error("Too few bytes (less than NONCE + ZERO bytes of SecretMsg).".to_string())
        };

        let key = SecretKey::from_slice(&output);
        let dec = key.decrypt(&secret).unwrap();

        let deserialized: Vec<Entry> = serde_json::from_slice(&dec).unwrap();
        // println!("deserialized: {:?}", deserialized);

        Ok(Database::new(password))
	}

    fn invalid_data_error(text: String) -> io::Result<Database>{
        Err(Error::new(ErrorKind::InvalidData, text))
    }
    
    fn save<T: Write>(&self, mut dest: T) -> io::Result<()>{

        let serialized = serde_json::to_string(&self.entries).unwrap();

        let key = SecretKey::from_slice(&self.bcrypt_pass);
        let enc: SecretMsg = key.encrypt(serialized.as_bytes());

        // write version
        try!(dest.write(&[DB_VERSION]));
        // write salt first
        try!(dest.write(&self.bcrypt_salt));
        try!(dest.flush());
        // write nonce + encrypted data
        try!(dest.write(&enc.nonce));
        try!(dest.flush());
        try!(dest.write(&enc.cipher));
        try!(dest.flush());

        Ok(())
    }



    pub fn add(&mut self, entry: Entry){
        self.entries.push(entry);
    }
}
