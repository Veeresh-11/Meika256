use zeroize::Zeroize;

/// Meika-256 reversible transformation core
///
/// This is NOT a standalone cipher.
/// It is a diffusion layer applied before AEAD (AES-GCM).
pub struct Meika256 {
    key: [u8; 32],
}

impl Meika256 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    /// Apply forward Meika transformation
    pub fn transform_forward(&self, data: &mut [u8]) {
        for round in 0..32 {
            self.round_forward(data, round);
        }
    }

    /// Apply inverse Meika transformation
    pub fn transform_inverse(&self, data: &mut [u8]) {
        for round in (0..32).rev() {
            self.round_inverse(data, round);
        }
    }

    fn round_forward(&self, data: &mut [u8], round: usize) {
        let k = self.key[round & 31];

        for i in 0..data.len() {
            let idx = (i + round) & 31;
            data[i] = data[i].wrapping_add(k);
            data[i] ^= self.key[idx];
            data[i] = data[i].rotate_left(3);
        }

        data.reverse();
    }

    fn round_inverse(&self, data: &mut [u8], round: usize) {
        let k = self.key[round & 31];

        data.reverse();

        for i in 0..data.len() {
            let idx = (i + round) & 31;
            data[i] = data[i].rotate_right(3);
            data[i] ^= self.key[idx];
            data[i] = data[i].wrapping_sub(k);
        }
    }
}

/// Ensure key material is wiped when dropped
impl Drop for Meika256 {
    fn drop(&mut self) {
        self.key.zeroize();
    }
}
