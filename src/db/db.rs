use std::path::Path;
use std::fs::File;
use std::io::{Write, Read, self, Error, ErrorKind};
use db::Entry;
use nacl::secretbox::{SecretKey, SecretMsg};
use rand::{ Rng, OsRng };
use crypto::bcrypt::bcrypt;
use serde_json;

const DB_VERSION: u8 = 1u8;

const SALT_SIZE: usize = 16;
const PASS_SIZE: usize = 24;
const BCRYPT_COST: u32 = 10;

pub struct DatabaseInFile {
    pub db: Database,
    pub filepath: String
}

impl DatabaseInFile {
    pub fn save(&self) -> io::Result<()>{
        self.db.save_to_file(Path::new(&self.filepath))
    }
}

pub struct Database {
    bcrypt_salt: [u8; SALT_SIZE],
    bcrypt_pass: [u8; PASS_SIZE],
    pub entries: Vec<Entry>
}

impl Database {
    pub fn empty(password: &str) -> Database {
        let mut salt = [0u8; SALT_SIZE]; // 16bytes of salt bcrypt
        let mut bcrypt_output = [0u8; PASS_SIZE]; // output 24 bytes
        OsRng::new().unwrap().fill_bytes(&mut salt);

        // TODO take only first 72 characters of input
        bcrypt(BCRYPT_COST, &salt, password.as_bytes(), &mut bcrypt_output);

        Database {
            bcrypt_salt: salt,
            bcrypt_pass: bcrypt_output,
            entries: Vec::new()
		}
    }

    pub fn open_from_file(path: &Path, password: &str) -> io::Result<Database> {
        // let mut file = try!(File::open(Path::new(file_path)));
        let mut file = try!(File::open(path));
        Database::open(password, &mut file)
    }

    pub fn save_to_file(&self, path: &Path) -> io::Result<()> {
        // let path = Path::new(file_path);
        // let display = path.display();
        // Open the file in write-only mode
        let mut file = try!(File::create(path));
        self.save(&mut file)
    }

    pub fn open<T: Read>(password: &str, src: &mut T) -> io::Result<Database> {
        let mut salt = [0u8; SALT_SIZE]; // 16bytes of salt bcrypt
        let mut bcrypt_output = [0u8; PASS_SIZE]; // output 24 bytes
        let mut version_buffer = [0u8; 1];

        match src.read(&mut version_buffer){
            Ok(_) => (),
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
        bcrypt(BCRYPT_COST, &salt, password.as_bytes(), &mut bcrypt_output);

        // Decrypt
        let secret =  match SecretMsg::from_bytes(&buffer) {
            Some(msg) => msg,
            None => return Database::invalid_data_error("Too few bytes (less than NONCE + ZERO bytes of SecretMsg).".to_string())
        };

        let key = SecretKey::from_slice(&bcrypt_output);
        let dec = key.decrypt(&secret).unwrap();

        let deserialized_entries: Vec<Entry> = serde_json::from_slice(&dec).unwrap();

        Ok(Database{
            bcrypt_salt: salt,
            bcrypt_pass: bcrypt_output,
            entries: deserialized_entries
        })
	}

    pub fn save<T: Write>(&self, dest: &mut T) -> io::Result<()>{
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

    pub fn get(&self, entry_title: &str) -> Option<&Entry> {
        self.entries.iter().find(|entry| entry.title.eq(entry_title))
    }

    pub fn remove(&mut self, entry_title: &str) -> bool{
        let pos = self.entries
            .iter()
            .position(|entry| entry.title.eq(entry_title));
        return match pos {
            Some(index) => {
                self.entries.remove(index);
                true
            }
            None =>  false
        }
    }

    fn invalid_data_error(text: String) -> io::Result<Database>{
        Err(Error::new(ErrorKind::InvalidData, text))
    }

}

#[cfg(test)]
mod tests {
    use db::Entry;
    use db::Database;
    use std::io::Cursor;
    use std::io::Read;

    #[test]
    fn test_save_and_load() {
        let mut buff: Cursor<Vec<u8>> = Cursor::new(vec![]);
        {
            let mut db = Database::empty("test");
            db.add(Entry::new("service_a", "name_a", "pass_a"));
            db.add(Entry::new("service_b", "name_b", "pass_b"));
            db.add(Entry::new("service_c", "name_c", "pass_c"));
            db.save(&mut buff);
        }

        // Cursor position has to be reset before reading
        buff.set_position(0);
        let db = Database::open("test", &mut buff).unwrap();
        assert_eq!(db.entries.len(), 3);
    }
}
