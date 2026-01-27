use aes_gcm::{Aes256Gcm, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::RngCore;
use anyhow::{Result, anyhow};

pub struct AesEncrypted {
    pub nonce: [u8; 12],
    pub ciphertext: Vec<u8>,
}

pub fn aes_encrypt(key: &[u8; 32], data: &[u8]) -> Result<AesEncrypted> {
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|_| anyhow!("Invalid AES-256 key"))?;

    let mut nonce = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);

    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), data)
        .map_err(|_| anyhow!("AES-GCM encryption failed"))?;

    Ok(AesEncrypted { nonce, ciphertext })
}

pub fn aes_decrypt(
    key: &[u8; 32],
    nonce: &[u8; 12],
    ciphertext: &[u8],
) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|_| anyhow!("Invalid AES-256 key"))?;

    cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|_| anyhow!("AES-GCM authentication failed"))
}
