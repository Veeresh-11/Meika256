use std::io::Cursor;

use meika256_lib::{
    encrypt_file,
    decrypt_file,
    encrypt_stream,
    decrypt_stream,
};

#[test]
fn buffer_roundtrip() {
    let data = b"hello meika buffer test";
    let password = "correct horse battery staple";

    let encrypted = encrypt_file(data, password).unwrap();
    let decrypted = decrypt_file(&encrypted, password).unwrap();

    assert_eq!(decrypted, data);
}

#[test]
fn wrong_password_fails() {
    let data = b"secret data";

    let encrypted = encrypt_file(data, "password1").unwrap();
    let decrypted = decrypt_file(&encrypted, "password2");

    assert!(decrypted.is_err());
}

#[test]
fn streaming_roundtrip() {
    let data = vec![0xAB; 512 * 1024]; // 512 KB
    let password = "stream-pass";

    let mut encrypted = Vec::new();
    encrypt_stream(
        Cursor::new(&data),
        &mut encrypted,
        password,
        64 * 1024,
    )
    .unwrap();

    let mut decrypted = Vec::new();
    decrypt_stream(
        Cursor::new(&encrypted),
        &mut decrypted,
        password,
    )
    .unwrap();

    assert_eq!(decrypted, data);
}

#[test]
fn buffer_vs_stream_consistency() {
    let data = b"compare buffer and stream";
    let password = "consistency";

    let buffer_enc = encrypt_file(data, password).unwrap();

    let mut stream_enc = Vec::new();
    encrypt_stream(
        Cursor::new(data),
        &mut stream_enc,
        password,
        16 * 1024,
    )
    .unwrap();

    let buffer_dec = decrypt_file(&buffer_enc, password).unwrap();

    let mut stream_dec = Vec::new();
    decrypt_stream(
        Cursor::new(&stream_enc),
        &mut stream_dec,
        password,
    )
    .unwrap();

    assert_eq!(buffer_dec, stream_dec);
}

#[test]
fn invalid_header_rejected() {
    let mut data = encrypt_file(b"test", "pw").unwrap();
    data[0] = 0x00; // corrupt MAGIC

    let result = decrypt_file(&data, "pw");
    assert!(result.is_err());
}

#[test]
fn tampered_ciphertext_detected() {
    let mut encrypted = encrypt_file(b"tamper test", "pw").unwrap();

    // Flip a byte in ciphertext
    let last = encrypted.len() - 1;
    encrypted[last] ^= 0xFF;

    let result = decrypt_file(&encrypted, "pw");
    assert!(result.is_err());
}
