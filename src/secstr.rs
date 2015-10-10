// Secure string library for Rust
// by https://github.com/myfreeweb/secstr

/*
This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <http://unlicense.org/>
*/

//! A data type suitable for storing sensitive information such as passwords and private keys in memory, featuring constant time equality, mlock and zeroing out.
extern crate libc;
use std::fmt;
use std::ptr;
use std::borrow::Borrow;
use std::borrow::BorrowMut;

/// A data type suitable for storing sensitive information such as passwords and private keys in memory, that implements:  
/// 
/// - Automatic zeroing in `Drop`  
/// - Constant time comparison in `PartialEq` (does not short circuit on the first different character; but terminates instantly if strings have different length)  
/// - Outputting `***SECRET***` to prevent leaking secrets into logs in `fmt::Debug` and `fmt::Display`  
/// - Automatic `mlock` to protect against leaking into swap  
/// 
/// Be careful with `SecStr::from`: if you have a borrowed string, it will be copied.  
/// Use `SecStr::new` if you have a `Vec<u8>`.
pub struct SecStr {
    content: Vec<u8>
}

impl SecStr {
    pub fn new(cont: Vec<u8>) -> SecStr {
        memlock::mlock(&cont);
        SecStr { content: cont }
    }

    /// Borrow the contents of the string.
    pub fn unsecure(&self) -> &[u8] {
        self.borrow()
    }

    /// Mutably borrow the contents of the string.
    pub fn unsecure_mut(&mut self) -> &mut [u8] {
        self.borrow_mut()
    }

    #[inline(never)]
    /// Overwrite the string with zeros. This is automatically called in the destructor.
    pub fn zero_out(&mut self) {
        unsafe {
            // TODO guarantee that this is not removed by compiler 
            // https://users.rust-lang.org/t/optimization-by-the-compiler-of-non-volatile-and-volatile-io-operations/3181
            ptr::write_bytes(self.content.as_ptr() as *mut libc::c_void, 0, self.content.len());
        }
    }
}

// Creation
impl<T> From<T> for SecStr where T: Into<Vec<u8>> {
    fn from(s: T) -> SecStr {
        SecStr::new(s.into())
    }
}

// Borrowing
impl Borrow<[u8]> for SecStr {
    fn borrow(&self) -> &[u8] {
        self.content.borrow()
    }
}

impl BorrowMut<[u8]> for SecStr {
    fn borrow_mut(&mut self) -> &mut [u8] {
        self.content.borrow_mut()
    }
}

// Overwrite memory with zeros when we're done
impl Drop for SecStr {
    fn drop(&mut self) {
        self.zero_out();
        memlock::munlock(&self.content);
    }
}

// Constant time comparison
impl PartialEq for SecStr {
    #[inline(never)]
    fn eq(&self, other: &SecStr) -> bool {
        let ref us = self.content;
        let ref them = other.content;
        if us.len() != them.len() {
            return false;
        }
        let mut result = 0;
        for i in 0..us.len() {
            result |= us[i] ^ them[i];
        }
        result == 0
    }
}

// Make sure sensitive information is not logged accidentally
impl fmt::Debug for SecStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("***SECRET***").map_err(|_| { fmt::Error })
    }
}

impl fmt::Display for SecStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("***SECRET***").map_err(|_| { fmt::Error })
    }
}

#[cfg(unix)]
mod memlock {
    extern crate libc;
    use self::libc::funcs::posix88::mman;

    pub fn mlock(cont: &Vec<u8>) {
        unsafe {
            mman::mlock(cont.as_ptr() as *const libc::c_void, cont.len() as libc::size_t);
        }
    }

    pub fn munlock(cont: &Vec<u8>) {
        unsafe {
            mman::munlock(cont.as_ptr() as *const libc::c_void, cont.len() as libc::size_t);
        }
    }
}

#[cfg(not(unix))]
mod memlock {
    fn mlock(cont: &Vec<u8>) {
    }

    fn munlock(cont: &Vec<u8>) {
    }
}

#[cfg(test)]
mod tests {
    use super::SecStr;

    #[test]
    fn test_basic() {
        let my_sec = SecStr::from("hello");
        assert_eq!(my_sec, SecStr::from("hello".to_string()));
        assert_eq!(my_sec.unsecure(), b"hello");
    }

    #[test]
    fn test_zero_out() {
        let mut my_sec = SecStr::from("hello");
        my_sec.zero_out();
        assert_eq!(my_sec.unsecure(), b"\x00\x00\x00\x00\x00");
    }

    #[test]
    fn test_comparison() {
        assert_eq!(SecStr::from("hello"),  SecStr::from("hello"));
        assert!(  SecStr::from("hello") != SecStr::from("yolo"));
        assert!(  SecStr::from("hello") != SecStr::from("olleh"));
    }

    #[test]
    fn test_show() {
        assert_eq!(format!("{}", SecStr::from("hello")), "***SECRET***".to_string());
    }

}