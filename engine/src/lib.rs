//! Meika-256 Engine v0.1
//! Public API is frozen.

mod aeswrap;
mod keyderivation;
mod meika;

use std::io::{Read, Write};
use anyhow::{Result, ensure};
use zeroize::Zeroize;

use aeswrap::{aes_encrypt, aes_decrypt};
use keyderivation::{derive_keys, generate_salt};
use meika::Meika256;


pub const MAGIC: &[u8; 8] = b"MEIKA256";
pub const VERSION: u8 = 1;
const PASSWORD_TAG: &[u8] = b"MEIKA_OK";

/// =======================
/// BUFFER MODE
/// =======================

pub fn encrypt_file(data: &[u8], password: &str) -> Result<Vec<u8>> {
    let salt = generate_salt();
    let keys = derive_keys(password, &salt)?;

    let mut buffer = Vec::new();
    buffer.extend_from_slice(PASSWORD_TAG);
    buffer.extend_from_slice(data);

    let meika = Meika256::new(keys.meika_key);
    meika.transform_forward(&mut buffer);

    let encrypted = aes_encrypt(&keys.aes_key, &buffer)?;

    buffer.zeroize();

    let mut out = Vec::new();
    out.extend_from_slice(MAGIC);
    out.push(VERSION);
    out.extend_from_slice(&salt);
    out.extend_from_slice(&encrypted.nonce);
    out.extend_from_slice(&encrypted.ciphertext);

    Ok(out)
}

pub fn decrypt_file(data: &[u8], password: &str) -> Result<Vec<u8>> {
    ensure!(data.len() >= 37, "Invalid file");
    ensure!(&data[..8] == MAGIC, "Invalid header");
    ensure!(data[8] == VERSION, "Unsupported version");

    let salt = &data[9..25];
    let nonce: &[u8; 12] = (&data[25..37]).try_into().unwrap();
    let ciphertext = &data[37..];

    let keys = derive_keys(password, salt)?;

    let mut decrypted = aes_decrypt(&keys.aes_key, nonce, ciphertext)?;

    let meika = Meika256::new(keys.meika_key);
    meika.transform_inverse(&mut decrypted);

    ensure!(
        decrypted.starts_with(PASSWORD_TAG),
        "Wrong password or corrupted file"
    );

    decrypted.drain(..PASSWORD_TAG.len());
    Ok(decrypted)
}

/// =======================
/// STREAMING MODE
/// =======================

pub fn encrypt_stream<R: Read, W: Write>(
    mut reader: R,
    mut writer: W,
    password: &str,
    chunk_size: usize,
) -> Result<()> {
    let salt = generate_salt();
    let keys = derive_keys(password, &salt)?;
    let meika = Meika256::new(keys.meika_key);

    writer.write_all(MAGIC)?;
    writer.write_all(&[VERSION])?;
    writer.write_all(&salt)?;
    writer.write_all(&(chunk_size as u32).to_le_bytes())?;

    let mut buffer = vec![0u8; chunk_size];

    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }

        let mut chunk = Vec::new();
        chunk.extend_from_slice(PASSWORD_TAG);
        chunk.extend_from_slice(&buffer[..read]);

        meika.transform_forward(&mut chunk);
        let encrypted = aes_encrypt(&keys.aes_key, &chunk)?;

        writer.write_all(&encrypted.nonce)?;
        writer.write_all(&(encrypted.ciphertext.len() as u32).to_le_bytes())?;
        writer.write_all(&encrypted.ciphertext)?;

        chunk.zeroize();
    }

    Ok(())
}

pub fn decrypt_stream<R: Read, W: Write>(
    mut reader: R,
    mut writer: W,
    password: &str,
) -> Result<()> {
    let mut magic = [0u8; 8];
    reader.read_exact(&mut magic)?;
    ensure!(&magic == MAGIC, "Invalid header");

    let mut version = [0u8; 1];
    reader.read_exact(&mut version)?;
    ensure!(version[0] == VERSION, "Unsupported version");

    let mut salt = [0u8; 16];
    reader.read_exact(&mut salt)?;

    let mut _chunk_size = [0u8; 4];
    reader.read_exact(&mut _chunk_size)?;

    let keys = derive_keys(password, &salt)?;
    let meika = Meika256::new(keys.meika_key);

    loop {
        let mut nonce = [0u8; 12];
        if reader.read_exact(&mut nonce).is_err() {
            break;
        }

        let mut len_bytes = [0u8; 4];
        reader.read_exact(&mut len_bytes)?;
        let len = u32::from_le_bytes(len_bytes) as usize;

        let mut ciphertext = vec![0u8; len];
        reader.read_exact(&mut ciphertext)?;

        let mut decrypted = aes_decrypt(&keys.aes_key, &nonce, &ciphertext)?;
        meika.transform_inverse(&mut decrypted);

        ensure!(
            decrypted.starts_with(PASSWORD_TAG),
            "Wrong password"
        );

        writer.write_all(&decrypted[PASSWORD_TAG.len()..])?;
    }

    Ok(())
}
