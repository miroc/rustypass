//!
//! Exposes the crypto_secretbox functionality of NaCl.
//!
//! The `secretbox` module symmetrically encrypts given plaintext and
//! then uses a one time authenticator to ensure tamper-resistance.
//!
//! In other words, this uses an encrypt-then-MAC scheme.
//!
//! ## Usage
//!
//! ```rust{.example}
//! use nacl::secretbox::{SecretKey, SecretMsg};
//!
//! let key = SecretKey::from_str("some secret key");
//! let enc: SecretMsg = key.encrypt("my secret msg".as_bytes());
//!
//! // ...
//!
//! let decr_opt = key.decrypt(&enc);
//! println!("decrypted: {:?}", decr_opt.unwrap());
//! ```

// Inspired by https://github.com/erik/knuckle/blob/master/src/secretbox.rs
// MIT license, by Erik Price

use std::iter::repeat;
use std::ptr::copy_nonoverlapping;
use rand::{ Rng, OsRng };
use nacl::bindings::{crypto_secretbox, crypto_secretbox_open};

/// Size of shared secret key used for symmetric encryption.
/// Corresponds to nacl crypto_secretbox_KEYBYTES
pub const KEY_BYTES: usize = 32; //256b
/// Size of the nonce value.
/// Corresponds to nacl crypto_secretbox_NONCEBYTES
pub const NONCE_BYTES: usize = 24; // 192b
/// Size of the zero padding applied to each message.
/// Corresponds to crypto_secretbox_ZEROBYTES
pub const ZERO_BYTES: usize = 32; // 256b

#[derive(Debug, Clone, Copy)]
pub enum SecretBoxError {
    VerificationFail
}

/// Encapsulates both the nonce value and cipher text returned by `encrypt`.
pub struct SecretMsg {
    /// Nonce value used for this ciphertext.
    pub nonce: [u8; NONCE_BYTES],
    pub cipher: Vec<u8>
}

impl SecretMsg {
    pub fn from_bytes(bytes: &[u8]) -> Option<SecretMsg> {
        if bytes.len() <= NONCE_BYTES + ZERO_BYTES {
            return None
        }

        let mut nonce = [0u8; NONCE_BYTES];
        let cipher = &bytes[NONCE_BYTES..];

        unsafe { copy_nonoverlapping(bytes.as_ptr(), nonce.as_mut_ptr(), NONCE_BYTES); }

        Some(SecretMsg { nonce: nonce, cipher: cipher.to_vec() })
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buf = self.nonce.to_vec();
        buf.extend(self.cipher.iter().cloned());

        buf
    }
}


/// Shared secret key. Must be `<= KEY_BYTES` bytes long.
///
/// This struct wraps access to encrypting and decrypting messages.
#[derive(Copy, Clone)]
pub struct SecretKey ([u8; KEY_BYTES]);

impl SecretKey {
    /// Generate a secret key from the bytes of a given string.
    pub fn from_str(str: &str) -> SecretKey {
        SecretKey::from_slice(str.as_bytes())
    }

    /// Generate a secret key from a slice (turn a slice into a sized slice).
    pub fn from_slice(slice: &[u8]) -> SecretKey {
        assert!(slice.len() <= KEY_BYTES);

        let mut sized = [0u8; KEY_BYTES];
        unsafe { copy_nonoverlapping(slice.as_ptr(), sized.as_mut_ptr(), slice.len()); }

        SecretKey(sized)
    }

    /// Using this secret key, symmetrically encrypt the given message.
    ///
    /// A random nonce value will be securely generated and returned
    /// as part of the response.
    pub fn encrypt(&self, msg: &[u8]) -> SecretMsg {
        let mut stretched  = [0u8; ZERO_BYTES].to_vec();
        stretched.extend(msg.iter().cloned());

        let mut nonce = [0u8; NONCE_BYTES];
        let &SecretKey(sk) = self;

        unsafe {
            let mut cipher: Vec<u8> = repeat(0u8).take(stretched.len()).collect();

            // generate Nonce from os source or rand (e.g. linux /dev/urandom)
            // TODO verify we have enough entropy
            OsRng::new().unwrap().fill_bytes(&mut nonce);
            //randombytes(nonce.as_mut_ptr(), NONCE_BYTES as u64);

            // TODO: Better error handling
            match crypto_secretbox(cipher.as_mut_ptr(),
                                   stretched.as_ptr(),
                                   stretched.len() as u64,
                                   nonce.as_ptr(),
                                   sk.as_ptr()) {
                0 => SecretMsg {
                    nonce: nonce,
                    cipher: cipher
                },
                _ => panic!("crypto_secretbox failed")
            }
        }
    }

    /// Using this box's secret key, decrypt the given ciphertext into
    /// plain text.
    pub fn decrypt(&self, msg: &SecretMsg) -> Result<Vec<u8>, SecretBoxError> {
        let &SecretKey(sk) = self;
        let mut plaintext: Vec<u8> = repeat(0u8).take(msg.cipher.len()).collect();

        unsafe {
            match crypto_secretbox_open(plaintext.as_mut_ptr(),
                                        msg.cipher.as_ptr(),
                                        msg.cipher.len() as u64,
                                        msg.nonce.as_ptr(),
                                        sk.as_ptr()) {
                0 => Ok((&plaintext[ZERO_BYTES .. plaintext.len()]).to_vec()),
                -1 => Err(SecretBoxError::VerificationFail),
                res => panic!(format!("crypto_secretbox_open failed, reason {}", res))
            }
        }
    }
}


#[test]
fn test_secretbox_sanity() {
    for i in 0..16 {
        let msg: Vec<u8> = repeat(i as u8).take(i * 4).collect();

        let key = SecretKey::from_str("some secret key");
        let SecretMsg { nonce, cipher } = key.encrypt(&msg);

        println!("enc:\t{:?}\nnonce:\t{:?}", cipher, nonce.to_vec());

        let decr_opt = key.decrypt(&SecretMsg { nonce: nonce, cipher: cipher });

        assert!(decr_opt.is_ok());

        let decr = decr_opt.unwrap();
        println!("dec:\t{:?}", decr);

        assert!(msg == decr);
    }
}

#[test]
fn test_secretbox_uniqueness() {
    let msg: Vec<u8> = repeat(0x53u8).take(128).collect();

    let key1 = SecretKey::from_str("1");
    let key2 = SecretKey::from_str("");

    let SecretMsg { nonce: n1, cipher: c1 } = key1.encrypt(&msg);
    let SecretMsg { nonce: n2, cipher: c2 } = key2.encrypt(&msg);

    assert!(n1 != n2);
    assert!(c1 != c2);
    assert!(c1 != msg);
    assert!(c2 != msg);
}

#[test]
fn test_secretbox_mac_sanity() {

    let msg: Vec<u8> = repeat(0xff).take(0xff).collect();

    let key = SecretKey::from_str("some secret key");

    let SecretMsg { nonce, cipher } = key.encrypt(&msg);

    let mut ciphers = [cipher.clone(), cipher.clone(), cipher.clone()];

    // tamper with the cipher text in various ways
    ciphers[0].push(0u8);
    ciphers[1].pop();

    let last = ciphers[2].pop().unwrap();
    ciphers[2].push(last + 1);

    for c in ciphers.iter() {
        let decr = key.decrypt(&SecretMsg { nonce: nonce, cipher: c.clone() });

        println!("cipher:\t{:?}\ndecr:\t{:?}", c, decr);
        assert!(decr.is_err());
    }

}

#[test]
fn test_secretbox_secretmsg() {
    let msg = b"some message";
    let key = SecretKey::from_str("some secret key");
    let encr = key.encrypt(msg);

    let secret_msg= encr.as_bytes();
    let re_encr = SecretMsg::from_bytes(&secret_msg);

    assert!(re_encr.is_some());

    let decr_opt = key.decrypt(&re_encr.unwrap());

    assert!(decr_opt.is_ok());
    assert!(decr_opt.unwrap() == msg);
}

#[test]
fn test_secretkey_tamper_resistance() {
    let msg = b"some message";
    let key = SecretKey::from_str("some secret key");
    let encr = key.encrypt(msg);
    let mut tampered_msg = encr.cipher.clone();

    // Start past the end of the nonce padding
    for i in 16..tampered_msg.len() {
        tampered_msg[i] = tampered_msg[i] ^ 0xFF;

        let tampered = SecretMsg { nonce: encr.nonce, cipher: tampered_msg.clone() };
        let plaintext = key.decrypt(&tampered);

        println!("msg:\t{:?}\ntampered:\t{:?}\nplaintext:\t{:?}", msg, tampered.cipher, plaintext);
        assert!(plaintext.is_err());
    }
}
