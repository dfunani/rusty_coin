use std::{fmt::Write, io::Read};

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm,
    Key, // Or `Aes128Gcm`
    Nonce,
};

const NONCE: &[u8] = &[0; 12];

pub fn encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    let key: &Key<Aes256Gcm> = key.into();

    // Create an AES-GCM cipher
    let cipher = Aes256Gcm::new(&key);

    // Encrypt the plaintext
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&NONCE), data)
        .expect("Encryption Error - Error Decipher.");

    return ciphertext;
}

pub fn decrypt(data: &[u8], key: &[u8]) -> String {
    let key: &Key<Aes256Gcm> = key.into();

    let cipher = Aes256Gcm::new(&key);

    // Encrypt the plaintext
    let ciphertext = cipher
        .decrypt(Nonce::from_slice(&NONCE), data)
        .expect("Decryption Error - Error Decipher");

    return String::from_utf8(ciphertext).expect("Decryption Error.");
}
