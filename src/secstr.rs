use libc::{c_void, size_t};
use libc::funcs::posix88::mman;
use std::ptr;
use rand::{ Rng, OsRng };
use nacl::stream::{self, stream_encrypt_xor};
use serde::ser::{Serialize, Serializer};

#[doc = "
SecStr implements a secure string. This means in particular:
* The input string moves to the struct, i.e. it's not just borrowed
* The string is encrypted with a random password for obfuscation
* mlock() is called on the string to prevent swapping
* A method to overwrite the string with zeroes is implemented
* The overwrite method is called on drop of the struct automatically
* Implements fmt::Show to prevent logging of the secrets, i.e. you can
  access the plaintext string only via the string value.
"]
pub struct SecStr {
    /// Holds the decrypted string if unlock() is called.
    /// Don't forget to call delete if you don't need the decrypted
    /// string anymore.
    /// Use String as type to move ownership to the struct.
    pub string: String,
    // Use of Vec instead of &[u8] because specific lifetimes aren't needed
    //encrypted: SecretMsg,
    //password: SecretKey

    encrypted_string: Vec<u8>,
    password: Vec<u8>,
    iv: Vec<u8>,
}

impl SecStr {
    /// Create a new SecureString
    /// The input string should already lie on the heap, i.e. the type should
    /// be String and not &str, otherwise a copy of the plain text string would
    /// lie in memory. The string will be automatically encrypted and deleted.
    pub fn new(string: String) -> SecStr {
        // Lock the string against swapping
        unsafe { mman::mlock(string.as_ptr() as *const c_void,
                             string.len() as size_t); }


        // todo why does rng has to be mutable?
        let mut rng = OsRng::new().unwrap();

        // let mut x: Box<[u8; stream::NONCE_BYTES]>;
        // rng.fill_bytes(x);

        let mut sec_str = SecStr {
            string: string,
            encrypted_string: vec![],
            password: (0..stream::KEY_BYTES).map(|_| rng.gen::<u8>()).collect(),
            iv: (0..stream::NONCE_BYTES).map(|_| rng.gen::<u8>()).collect()
        };
        unsafe {
            mman::mlock(sec_str.encrypted_string.as_ptr() as *const c_void,
                             sec_str.encrypted_string.len() as size_t);
        }
        sec_str.lock();
        sec_str.delete();
        sec_str
    }

    /// Overwrite the string with zeroes. Call this everytime after unlock() if you don't
    /// need the string anymore.
    pub fn delete(&self) {
        // Use volatile_set_memory to make sure that the operation is executed.
        unsafe {
            // https://users.rust-lang.org/t/optimization-by-the-compiler-of-non-volatile-and-volatile-io-operations/3181
            ptr::write_bytes(self.string.as_ptr() as *mut c_void, 0u8, self.string.len());
            // intrinsics::volatile_set_memory(self.string.as_ptr() as *mut c_void, 0u8,
                                                //  self.string.len())
        };
    }

    fn lock(&mut self) {
        self.encrypted_string = stream_encrypt_xor(
            self.string.as_bytes(),
            &self.iv,
            &self.password);
    }

    /// Unlock the string, i.e. decrypt it and make it available via the string value.
    /// Don't forget to call delete() if you don't need the plain text anymore.
    pub fn unlock(&mut self) {
        self.string = String::from_utf8(
            stream_encrypt_xor(
                &self.encrypted_string,
                &self.iv,
                &self.password
            )
        ).unwrap();
    }

    // Private export function used for serialization to json
    fn export(&self) -> String{
        String::from_utf8(
            stream_encrypt_xor(
                &self.encrypted_string,
                &self.iv,
                &self.password
            )
        ).unwrap()
    }
}

// Serde json serialization
impl Serialize for SecStr {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        serializer.visit_str(self.export().as_ref();
    }
}

// string value and encrypted_string value will be overwritten with zeroes after drop of struct
impl Drop for SecStr {
    fn drop(&mut self) {
        self.delete();
        unsafe { mman::munlock(self.string.as_ptr() as *const c_void,
                               self.string.len() as size_t);


                ptr::write_bytes(self.encrypted_string.as_ptr() as *mut c_void, 0u8,
                                                self.encrypted_string.len());
                //  intrinsics::volatile_set_memory(self.encrypted_string.as_ptr() as *mut c_void, 0u8,
                                                //  self.encrypted_string.len());


             mman::munlock(self.encrypted_string.as_ptr() as *const c_void,
                               self.encrypted_string.len() as size_t); }
    }
}

#[cfg(test)]
mod tests {
    use super::SecStr;
    use std::str;
    use std::ptr::copy;

    #[test]
    fn test_drop() {
        let mut test_vec:  Vec<u8> = Vec::with_capacity(4);
        let mut test_vec2: Vec<u8> = Vec::with_capacity(4);
        unsafe {
            test_vec.set_len(4);
            test_vec2.set_len(4);
            let str = "drop".to_string();
            let mut sec_str = SecStr::new(str);
            let enc_str_ptr = sec_str.encrypted_string.as_mut_ptr();
            let str_ptr = sec_str.string.as_mut_vec().as_mut_ptr();
            drop(sec_str);
            copy(enc_str_ptr, test_vec.as_mut_ptr(), 4);
            copy(str_ptr, test_vec2.as_mut_ptr(), 4);
        }
        assert_eq!(test_vec,  vec![0u8, 0u8, 0u8, 0u8]);
        assert_eq!(test_vec2, vec![0u8, 0u8, 0u8, 0u8]);
    }
    #[test]
    fn test_new() {
        let str = "Hello, box!".to_string();
        // Ownership of str moves to SecureString <- secure input interface
        let mut sec_str = SecStr::new(str);
        sec_str.unlock();
        assert_eq!(sec_str.string, "Hello, box!");
    }

    #[test]
    fn test_delete() {
        let str = "delete".to_string();
        let sec_str = SecStr::new(str);
        assert_eq!(sec_str.string, "\0\0\0\0\0\0");

        // Test with umlauts
        let str = "ä".to_string();
        let sec_str = SecStr::new(str);
        assert_eq!(sec_str.string, "\0\0");
    }

    #[test]
    fn test_lock() {
        let str = "delete".to_string();
        let mut sec_str = SecStr::new(str);

        assert!(str::from_utf8(&sec_str.encrypted_string) !=  Ok("delete"));

        sec_str.unlock();
        assert_eq!(sec_str.string, "delete");
    }

    #[test]
    fn test_encryption() {
        let str = "delete".to_string();
        let sec_str = SecStr::new(str);

        let str = "delete".to_string();
        let mut sec_str2 = SecStr::new(str);
        assert!(sec_str.encrypted_string != sec_str2.encrypted_string);

        sec_str2.unlock();
        sec_str2.iv = sec_str.iv.clone();
        sec_str2.password = sec_str.password.clone();
        sec_str2.lock();
        assert_eq!(sec_str.encrypted_string, sec_str2.encrypted_string);
    }
}
