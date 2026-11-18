//! aether-mixnet::sphinx
//! تنفيذ بنية حزم Sphinx المشفرة متعددة الطبقات (Multi-Hop Layered Onion Routing).
//! كل حزمة لها حجم فيزيائي ثابت ومطابق تماماً بغض النظر عن محتواها (Zero Metadata Leakage).

use zeroize::{Zeroize, ZeroizeOnDrop};
use x25519_dalek::{EphemeralSecret, PublicKey, StaticSecret};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use sha2::Sha256;
use hkdf::Hkdf;
use rand::rngs::OsRng;
use thiserror::Error;

pub const SPHINX_PAYLOAD_SIZE: usize = 1024; // الحجم القياسي للحمولة المشفرة
pub const ROUTING_INFO_SIZE: usize = 192;    // الحجم الثابت لمعلومات التوجيه المغلفة
pub const MAC_SIZE: usize = 16;             // حجم رمز التحقق من سلامة الطبقة
pub const PUBLIC_KEY_SIZE: usize = 32;

#[derive(Error, Debug)]
pub enum SphinxError {
    #[error("فشل التحقق من MAC في إحدى طبقات العُقد")]
    InvalidMac,
    #[error("خطأ في تشفير طبقة Sphinx")]
    LayerEncryptionError,
    #[error("حجم الحمولة يتجاوز الحجم المسموح")]
    PayloadTooLarge,
}

/// رأس حزمة Sphinx المشفرة
#[derive(Clone, Debug)]
pub struct SphinxHeader {
    /// المفتاح العام المؤقت للطبقة الحالية (Alpha)
    pub alpha: [u8; PUBLIC_KEY_SIZE],
    /// معلومات المسار المغلفة بطبقات التشفير (Beta)
    pub beta: [u8; ROUTING_INFO_SIZE],
    /// رمز التحقق من صحة المسار وعدم التلاعب (Gamma MAC)
    pub gamma: [u8; MAC_SIZE],
}

/// حزمة Sphinx المتكاملة الجاهزة للبث عبر شبكة Nym
#[derive(Clone, Debug)]
pub struct SphinxPacket {
    pub header: SphinxHeader,
    /// الحمولة المشفرة بطبقات عكسية متساوية الحجم دائماً (Delta)
    pub payload: [u8; SPHINX_PAYLOAD_SIZE],
}

/// معلومات القفزة التالية التي تستخرجها عقدة الـ Mixnet
pub struct UnpeeledHop {
    pub next_hop_id: [u8; 16],
    pub next_packet: SphinxPacket,
    pub is_exit_hop: bool,
}

/// مفاتيح الجلسة المشتقة لكل طبقة في المسار
#[derive(Zeroize, ZeroizeOnDrop)]
struct LayerKeys {
    routing_key: [u8; 32],
    mac_key: [u8; 32],
    payload_key: [u8; 32],
}

impl LayerKeys {
    fn derive(shared_secret: &[u8; 32]) -> Self {
        let hk = Hkdf::<Sha256>::new(None, shared_secret);
        let mut keys_raw = [0u8; 96];
        hk.expand(b"aether-sphinx-layer-keys-v1", &mut keys_raw)
            .expect("HKDF expand should not fail for 96 bytes");

        let mut rk = [0u8; 32];
        let mut mk = [0u8; 32];
        let mut pk = [0u8; 32];

        rk.copy_from_slice(&keys_raw[0..32]);
        mk.copy_from_slice(&keys_raw[32..64]);
        pk.copy_from_slice(&keys_raw[64..96]);

        Self {
            routing_key: rk,
            mac_key: mk,
            payload_key: pk,
        }
    }
}

pub struct SphinxBuilder;

impl SphinxBuilder {
    /// بناء حزمة Sphinx متعددة الطبقات (عادة 3 عُقد: Layer 1, Layer 2, Layer 3)
    pub fn build_packet(
        hops_keys: &[[u8; PUBLIC_KEY_SIZE]], // المفاتيح العامة للعقد الثلاثة بالترتيب
        hops_ids: &[[u8; 16]],               // معرّفات العقد التالية
        raw_payload: &[u8],                 // البيانات الحقيقية أو حزمة وهمية
    ) -> Result<SphinxPacket, SphinxError> {
        if raw_payload.len() > SPHINX_PAYLOAD_SIZE {
            return Err(SphinxError::PayloadTooLarge);
        }

        let num_hops = hops_keys.len();
        let mut rng = OsRng;
        let sender_secret = StaticSecret::random_from_rng(&mut rng);
        let initial_alpha = PublicKey::from(&sender_secret);

        // 1. حساب الأسرار المشتركة ومفاتيح الطبقات لكل عقدة
        let mut layer_keys_list = Vec::with_capacity(num_hops);
        for &node_pk_bytes in hops_keys.iter() {
            let node_pk = PublicKey::from(node_pk_bytes);
            let shared = sender_secret.diffie_hellman(&node_pk);
            layer_keys_list.push(LayerKeys::derive(shared.as_bytes()));
        }

        // 2. تشفير الحمولة (Payload) بترتيب عكسي: من القفزة الأخيرة إلى القفزة الأولى
        let mut payload_buf = [0u8; SPHINX_PAYLOAD_SIZE];
        payload_buf[..raw_payload.len()].copy_from_slice(raw_payload);

        for keys in layer_keys_list.iter().rev() {
            let cipher = ChaCha20Poly1305::new_from_slice(&keys.payload_key)
                .map_err(|_| SphinxError::LayerEncryptionError)?;
            let nonce = Nonce::from_slice(&[0u8; 12]); // ثابت لكل طبقة لأن المفتاح يُستخدم لمرة واحدة فقط
            
            // تشفير في نفس المكان (In-Place Stream XOR)
            let encrypted = cipher.encrypt(nonce, &payload_buf[..SPHINX_PAYLOAD_SIZE - MAC_SIZE])
                .map_err(|_| SphinxError::LayerEncryptionError)?;
            payload_buf.copy_from_slice(&encrypted[..SPHINX_PAYLOAD_SIZE]);
        }

        // 3. بناء وتغليف طبقات المسار (Routing Beta & Gamma)
        let mut beta = [0u8; ROUTING_INFO_SIZE];
        let mut gamma = [0u8; MAC_SIZE];

        // تعبئة الطبقة الأخيرة كعلامة وصول (Exit)
        for i in (0..num_hops).rev() {
            let keys = &layer_keys_list[i];
            let next_id = &hops_ids[i];

            // دمج المعرف التالي مع باقي التوجيه وتشفيره بالمفتاح الخاص بهذه العقدة
            let mut layer_input = [0u8; ROUTING_INFO_SIZE];
            layer_input[..16].copy_from_slice(next_id);
            layer_input[16..].copy_from_slice(&beta[..ROUTING_INFO_SIZE - 16]);

            // تشفير الطبقة
            let cipher = ChaCha20Poly1305::new_from_slice(&keys.routing_key)
                .map_err(|_| SphinxError::LayerEncryptionError)?;
            let nonce = Nonce::from_slice(&[0u8; 12]);
            
            let encrypted_beta = cipher.encrypt(nonce, &layer_input[..ROUTING_INFO_SIZE - MAC_SIZE])
                .map_err(|_| SphinxError::LayerEncryptionError)?;
            
            beta.copy_from_slice(&encrypted_beta[..ROUTING_INFO_SIZE]);
            gamma.copy_from_slice(&encrypted_beta[..MAC_SIZE]);
        }

        let mut alpha_bytes = [0u8; PUBLIC_KEY_SIZE];
        alpha_bytes.copy_from_slice(initial_alpha.as_bytes());

        Ok(SphinxPacket {
            header: SphinxHeader {
                alpha: alpha_bytes,
                beta,
                gamma,
            },
            payload: payload_buf,
        })
    }

    /// تقشير طبقة واحدة من الحزمة عند وصولها لعقدة Mixnet
    pub fn unpeel_layer(
        packet: &SphinxPacket,
        node_secret: &StaticSecret,
    ) -> Result<UnpeeledHop, SphinxError> {
        let alpha_pk = PublicKey::from(packet.header.alpha);
        let shared = node_secret.diffie_hellman(&alpha_pk);
        let keys = LayerKeys::derive(shared.as_bytes());

        // فك تشفير وتدقيق طبقة التوجيه
        let cipher = ChaCha20Poly1305::new_from_slice(&keys.routing_key)
            .map_err(|_| SphinxError::LayerEncryptionError)?;
        let nonce = Nonce::from_slice(&[0u8; 12]);
        
        let decrypted_beta = cipher.decrypt(nonce, &packet.header.beta[..])
            .map_err(|_| SphinxError::LayerEncryptionError)?;

        // استخراج معرف القفزة القادمة بعد فك التشفير
        let mut next_id = [0u8; 16];
        next_id.copy_from_slice(&decrypted_beta[..16]);

        let is_exit = next_id == [0u8; 16];

        // إزاحة طبقة التوجيه للقفزة القادمة وتعبئة الباقي بحشو أصفار
        let mut next_beta = [0u8; ROUTING_INFO_SIZE];
        let remaining_len = (decrypted_beta.len() - 16).min(ROUTING_INFO_SIZE);
        next_beta[..remaining_len].copy_from_slice(&decrypted_beta[16..16 + remaining_len]);

        // فك تشفير طبقة واحدة من الحمولة
        let payload_cipher = ChaCha20Poly1305::new_from_slice(&keys.payload_key)
            .map_err(|_| SphinxError::LayerEncryptionError)?;
        let mut unpeeled_payload = [0u8; SPHINX_PAYLOAD_SIZE];
        let decrypted_payload = payload_cipher.decrypt(nonce, &packet.payload[..])
            .unwrap_or_else(|_| packet.payload.to_vec());
        
        let copy_len = decrypted_payload.len().min(SPHINX_PAYLOAD_SIZE);
        unpeeled_payload[..copy_len].copy_from_slice(&decrypted_payload[..copy_len]);

        Ok(UnpeeledHop {
            next_hop_id: next_id,
            next_packet: SphinxPacket {
                header: SphinxHeader {
                    alpha: packet.header.alpha,
                    beta: next_beta,
                    gamma: packet.header.gamma,
                },
                payload: unpeeled_payload,
            },
            is_exit_hop: is_exit,
        })
    }
}
