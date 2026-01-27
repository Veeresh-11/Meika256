use argon2::{Argon2, Params};
use hkdf::Hkdf;
use sha3::Sha3_256;
use anyhow::Result;
use zeroize::Zeroize;
use rand::RngCore;

pub struct DerivedKeys {
    pub meika_key: [u8; 32],
    pub aes_key: [u8; 32],
}

pub fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    rand::rngs::OsRng.fill_bytes(&mut salt);
    salt
}

pub fn derive_keys(password: &str, salt: &[u8]) -> Result<DerivedKeys> {
    let mut master = [0u8; 32];

    let params = Params::new(65_536, 3, 1, Some(32))
        .map_err(|e| anyhow::anyhow!("Invalid Argon2 params: {e}"))?;

    let argon = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        params,
    );

    argon
        .hash_password_into(password.as_bytes(), salt, &mut master)
        .map_err(|e| anyhow::anyhow!("Argon2 failed: {e}"))?;

    let hk = Hkdf::<Sha3_256>::new(None, &master);

    let mut meika_key = [0u8; 32];
    let mut aes_key = [0u8; 32];

    hk.expand(b"MEIKA_KEY", &mut meika_key)
        .map_err(|_| anyhow::anyhow!("HKDF MEIKA_KEY failed"))?;

    hk.expand(b"AES_KEY", &mut aes_key)
        .map_err(|_| anyhow::anyhow!("HKDF AES_KEY failed"))?;

    master.zeroize();

    Ok(DerivedKeys { meika_key, aes_key })
}
