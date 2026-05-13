use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use ring::rand::SecureRandom;

pub fn encrypt(plaintext: &[u8], key: &[u8; 32]) -> Vec<u8> {
    let cipher = Aes256Gcm::new_from_slice(key).expect("Invalid key");
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill(&mut nonce_bytes).unwrap();
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher.encrypt(nonce, plaintext).expect("Encryption failed");
    [nonce_bytes.to_vec(), ciphertext].concat()
}

pub fn decrypt(encrypted: &[u8], key: &[u8; 32]) -> Option<Vec<u8>> {
    if encrypted.len() < 12 { return None; }
    let (nonce_bytes, ciphertext) = encrypted.split_at(12);
    let cipher = Aes256Gcm::new_from_slice(key).ok()?;
    let nonce = Nonce::from_slice(nonce_bytes);
    cipher.decrypt(nonce, ciphertext).ok()
}
