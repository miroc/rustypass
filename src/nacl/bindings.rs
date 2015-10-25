extern crate libc;
use self::libc::*;

// Inspired by https://github.com/erik/knuckle/blob/master/src/secretbox.rs
// MIT license, by Erik Price

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
    /// Symmetrically (+authentication) encrypt a message using a shared secret key.
	//extern int crypto_secretbox(unsigned char *,const unsigned char *,unsigned long long,const unsigned char *,const unsigned char *);
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

	 // --- Secret box ---
	 /// Encryption without authentication, for both encryption/decryption
	 // extern int crypto_stream_xor(unsigned char *,const unsigned char *,unsigned long long,const unsigned char *,const unsigned char *);
	 pub fn crypto_stream_xor(cipher: *mut c_uchar,
                             msg: *const c_uchar,
                             len: c_ulonglong,
                             nonce: *const c_uchar,
                             k: *const c_uchar) -> c_int;

}
