extern crate libc;
use self::libc::*;

// Inspired by https://github.com/erik/knuckle/blob/master/src/secretbox.rs
// MIT license, by Erik Price
// Only binding functions we need are included

//#[link(name = "tweetnacl", kind="static")]
extern {
	/*
	#include "crypto_secretbox.h"

    const unsigned char k[crypto_secretbox_KEYBYTES];
    const unsigned char n[crypto_secretbox_NONCEBYTES];
    const unsigned char m[...]; unsigned long long mlen;
    unsigned char c[...]; unsigned long long clen;

    crypto_secretbox(c,m,mlen,n,k);
	*/
	
	// --- Secret box ---

    /// Symmetrically encrypt a message using a shared secret key.
    pub fn crypto_secretbox(cipher: *mut c_uchar,
                            msg: *const c_uchar,
                            len: c_ulonglong,
                            nonce: *const c_uchar,
                            k: *const c_uchar) -> c_int;    

    /// Decrypt a message encryped with `crypto_secretbox`.
    pub fn crypto_secretbox_open(msg: *mut c_uchar,
                                 cipher: *const c_uchar,
                                 len: c_ulonglong,
                                 nonce: *const c_uchar,
                                 k: *const c_uchar) -> c_int;
}