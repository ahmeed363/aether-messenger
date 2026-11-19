//! aether-crypto: النواة التشفيرية لتطبيق أثير
//! تدعم التشفير الهجين، مسح الذاكرة الفوري (Zeroize)، وتبديل المفاتيح المؤقتة لكل رسالة (Double Ratchet).

pub mod handshake;

use zeroize::{Zeroize, ZeroizeOnDrop};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use sha2::Sha256;
use hkdf::Hkdf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("فشل التشفير")]
    EncryptionFailed,
    #[error("فشل فك التشفير - تم التلاعب بالحزمة أو مفتاح خاطئ")]
    DecryptionFailed,
    #[error("خطأ في اشتقاق المفاتيح")]
    DerivationFailed,
}

/// مفتاح رسالة سريع الزوال يُمحى من الذاكرة فوراً بمجرد انتهاء عمره
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct EphemeralMessageKey {
    key: [u8; 32],
}

impl EphemeralMessageKey {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    /// تشفير رسالة أو جزء وسائط (Chunk) بمفتاح لمرة واحدة
    pub fn encrypt(&self, plaintext: &[u8], nonce_bytes: &[u8; 12]) -> Result<Vec<u8>, CryptoError> {
        let cipher = ChaCha20Poly1305::new_from_slice(&self.key)
            .map_err(|_| CryptoError::EncryptionFailed)?;
        let nonce = Nonce::from_slice(nonce_bytes);
        cipher.encrypt(nonce, plaintext)
            .map_err(|_| CryptoError::EncryptionFailed)
    }

    /// فك تشفير الرسالة
    pub fn decrypt(&self, ciphertext: &[u8], nonce_bytes: &[u8; 12]) -> Result<Vec<u8>, CryptoError> {
        let cipher = ChaCha20Poly1305::new_from_slice(&self.key)
            .map_err(|_| CryptoError::DecryptionFailed)?;
        let nonce = Nonce::from_slice(nonce_bytes);
        cipher.decrypt(nonce, ciphertext)
            .map_err(|_| CryptoError::DecryptionFailed)
    }
}

/// اشتقاق مفاتيح جديدة لكل رسالة لضمان السرية التامة للأمام (PFS)
pub fn derive_ephemeral_ratchet_key(
    shared_secret: &[u8; 32],
    salt: &[u8; 32],
    info: &[u8],
) -> Result<EphemeralMessageKey, CryptoError> {
    let hk = Hkdf::<Sha256>::new(Some(salt), shared_secret);
    let mut okm = [0u8; 32];
    hk.expand(info, &mut okm)
        .map_err(|_| CryptoError::DerivationFailed)?;
    Ok(EphemeralMessageKey::new(okm))
}
