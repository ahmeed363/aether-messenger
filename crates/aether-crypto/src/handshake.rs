//! aether-crypto::handshake
//! بروتوكول المصافحة اللامركزية وتبادل المفاتيح لمرة واحدة (Zero-Knowledge Contact Handshake).
//! يتيح للمستخدمين إضافة بعضهم بمسح رمز QR أو رابط لمرة واحدة بدون كشف هوية أو خادم وسيط.

use x25519_dalek::{EphemeralSecret, PublicKey};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HandshakePayload {
    /// المفتاح العام المؤقت للطرف المضيف (X25519)
    pub ephemeral_pubkey: [u8; 32],
    /// رمز موعد الالتقاء العشوائي لمرة واحدة على شبكة Nym (Rendezvous Token)
    pub rendezvous_token: [u8; 16],
    /// توقيت انتهاء صلاحية الرمز بالثواني (Unix Epoch)
    pub expires_at: u64,
}

pub struct EphemeralHandshake {
    secret: EphemeralSecret,
    pub payload: HandshakePayload,
}

impl EphemeralHandshake {
    /// إنشاء حزمة مصافحة جديدة لإنشاء رمز QR لمرة واحدة
    pub fn generate_new(validity_seconds: u64, current_timestamp: u64) -> Self {
        let mut rng = OsRng;
        let secret = EphemeralSecret::random_from_rng(&mut rng);
        let pubkey = PublicKey::from(&secret);

        let mut rendezvous = [0u8; 16];
        rand::RngCore::fill_bytes(&mut rng, &mut rendezvous);

        let payload = HandshakePayload {
            ephemeral_pubkey: *pubkey.as_bytes(),
            rendezvous_token: rendezvous,
            expires_at: current_timestamp + validity_seconds,
        };

        Self { secret, payload }
    }

    /// إكمال المصافحة عند مسح كود الصديق واشتقاق مفتاح الجلسة التأسيسي
    pub fn complete_handshake(
        self,
        peer_pubkey_bytes: &[u8; 32],
    ) -> [u8; 32] {
        let peer_pk = PublicKey::from(*peer_pubkey_bytes);
        let shared = self.secret.diffie_hellman(&peer_pk);
        *shared.as_bytes()
    }
}
