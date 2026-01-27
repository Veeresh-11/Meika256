use meika256_lib::*;

#[test]
fn wrong_password_fails() {
    let data = b"top secret message";
    let enc = encrypt_file(data, "correct-password").unwrap();
    let dec = decrypt_file(&enc, "wrong-password");
    assert!(dec.is_err());
}

#[test]
fn corrupted_data_fails() {
    let data = b"hello world";
    let mut enc = encrypt_file(data, "pw").unwrap();

    // Flip a byte
    enc[40] ^= 0xFF;

    let dec = decrypt_file(&enc, "pw");
    assert!(dec.is_err());
}
