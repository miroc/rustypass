use nacl::bindings::crypto_stream_xor;
use std::iter::repeat;

pub const KEY_BYTES: usize = 32;
pub const NONCE_BYTES: usize = 24;

pub fn stream_encrypt_xor(input: &[u8], nonce: &[u8], key: &[u8]) -> Vec<u8> {
    // todo change this runtime length check to compile check
    if nonce.len() != NONCE_BYTES || key.len() != KEY_BYTES{
        panic!("key or nonce have bad length");
    }

    let mut output: Vec<u8> = repeat(0u8).take(input.len()).collect();

    unsafe {
        // todo check the c call returns null
        crypto_stream_xor(output.as_mut_ptr(),
            input.as_ptr(),
            input.len() as u64,
            nonce.as_ptr(),
            key.as_ptr());
    }

    return output;
}
