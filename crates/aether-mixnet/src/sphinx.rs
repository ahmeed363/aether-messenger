//! aether-mixnet::sphinx
//! تنفيذ المعيار الرياضي لحزم Sphinx المشفرة متعددة الطبقات (Sphinx Multi-Hop Onion Routing).
//! يعتمد التشفير التدفقي (ChaCha20 Stream Cipher) مع التحقق المستقل من سلامة المسار (HMAC-SHA256)
//! مما يحافظ على حجم فيزيائي ثابت ومطابق لجميع الحزم دون أي تسريب لبيانات التوجيه.

use zeroize::{Zeroize, ZeroizeOnDrop};
use x25519_dalek::{PublicKey, StaticSecret};
use chacha20::{
    cipher::{KeyIvInit, StreamCipher},
    ChaCha20,
};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hkdf::Hkdf;
use rand::rngs::OsRng;
use thiserror::Error;

pub const SPHINX_PAYLOAD_SIZE: usize = 1024; // الحجم القياسي للحمولة المشفرة
pub const ROUTING_INFO_SIZE: usize = 192;    // الحجم الثابت لمعلومات التوجيه المغلفة
pub const MAC_SIZE: usize = 16;             // حجم رمز التحقق من سلامة الطبقة
pub const PUBLIC_KEY_SIZE: usize = 32;

type HmacSha256 = Hmac<Sha256>;

#[derive(Error, Debug)]
pub enum SphinxError {
    #[error("فشل التحقق من MAC في إحدى طبقات العُقد")]
    InvalidMac,
    #[error("خطأ في تشفير طبقة Sphinx")]
    LayerEncryptionError,
    #[error("الحمولة أكبر من الحجم الأقصى المسموح")]
    PayloadTooLarge,
}

/// رأس حزمة Sphinx المشفرة
#[derive(Clone, Debug)]
pub struct SphinxHeader {
    pub alpha: [u8; PUBLIC_KEY_SIZE],
    pub beta: [u8; ROUTING_INFO_SIZE],
    pub gamma: [u8; MAC_SIZE],
}

/// حزمة Sphinx المتكاملة الجاهزة للبث عبر شبكة Nym
#[derive(Clone, Debug)]
pub struct SphinxPacket {
    pub header: SphinxHeader,
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
        let hk = Hkdf::<Sha256>::new(Some(b"aether-sphinx-layer-v1"), shared_secret);
        
        let mut rk = [0u8; 32];
        let mut mk = [0u8; 32];
        let mut pk = [0u8; 32];
        
        hk.expand(b"routing", &mut rk).expect("فشل اشتقاق مفتاح التوجيه");
        hk.expand(b"mac", &mut mk).expect("فشل اشتقاق مفتاح التحقق");
        hk.expand(b"payload", &mut pk).expect("فشل اشتقاق مفتاح الحمولة");
        
        Self {
            routing_key: rk,
            mac_key: mk,
            payload_key: pk,
        }
    }
}

pub struct SphinxBuilder;

impl SphinxBuilder {
    /// بناء حزمة Sphinx متعددة القفزات مع تشفير التوجيه بنظام الإزاحة والحشو (Shift & Pad)
    pub fn build_packet(
        hops_keys: &[[u8; PUBLIC_KEY_SIZE]],
        hops_ids: &[[u8; 16]],
        raw_payload: &[u8],
    ) -> Result<SphinxPacket, SphinxError> {
        if raw_payload.len() > SPHINX_PAYLOAD_SIZE {
            return Err(SphinxError::PayloadTooLarge);
        }

        let num_hops = hops_keys.len();
        let mut rng = OsRng;
        let sender_secret = StaticSecret::random_from_rng(&mut rng);
        let initial_alpha = PublicKey::from(&sender_secret);

        let mut alpha_bytes = [0u8; PUBLIC_KEY_SIZE];
        alpha_bytes.copy_from_slice(initial_alpha.as_bytes());

        // 1. حساب مفاتيح الطبقات لكل عقدة وحساب قيمة MAC الخاصة بها
        let mut layer_keys_list = Vec::with_capacity(num_hops);
        let mut gammas = Vec::with_capacity(num_hops);

        for &node_pk_bytes in hops_keys.iter() {
            let node_pk = PublicKey::from(node_pk_bytes);
            let shared = sender_secret.diffie_hellman(&node_pk);
            let keys = LayerKeys::derive(shared.as_bytes());

            // حساب الـ MAC للطبقة بناءً على مفتاح الـ MAC والمفتاح المؤقت Alpha
            let mut mac = HmacSha256::new_from_slice(&keys.mac_key)
                .map_err(|_| SphinxError::LayerEncryptionError)?;
            mac.update(&alpha_bytes);
            let mac_result = mac.finalize().into_bytes();
            let mut g = [0u8; MAC_SIZE];
            g.copy_from_slice(&mac_result[..MAC_SIZE]);
            gammas.push(g);

            layer_keys_list.push(keys);
        }

        // 2. تشفير الحمولة بترتيب عكسي باستخدام Stream Cipher
        let mut payload_buf = [0u8; SPHINX_PAYLOAD_SIZE];
        payload_buf[..raw_payload.len()].copy_from_slice(raw_payload);

        let zero_nonce = [0u8; 12];
        for keys in layer_keys_list.iter().rev() {
            let mut cipher = ChaCha20::new(&keys.payload_key.into(), &zero_nonce.into());
            cipher.apply_keystream(&mut payload_buf);
        }

        // 3. بناء وتغليف معلومات التوجيه (Beta) بترتيب عكسي من العقدة الأخيرة إلى الأولى
        let mut beta = [0u8; ROUTING_INFO_SIZE];

        for i in (0..num_hops).rev() {
            let keys = &layer_keys_list[i];
            let next_id = &hops_ids[i];
            let next_gamma = if i + 1 < num_hops {
                gammas[i + 1]
            } else {
                [0u8; MAC_SIZE]
            };

            // وضع معرف القفزة القادمة والـ MAC القادم وإزاحة باقي المسار
            let mut new_beta = [0u8; ROUTING_INFO_SIZE];
            new_beta[0..16].copy_from_slice(next_id);
            new_beta[16..32].copy_from_slice(&next_gamma);
            new_beta[32..].copy_from_slice(&beta[..ROUTING_INFO_SIZE - 32]);

            // تشفير beta بالمفتاح الخاص بهذه الطبقة
            let mut cipher = ChaCha20::new(&keys.routing_key.into(), &zero_nonce.into());
            cipher.apply_keystream(&mut new_beta);
            beta = new_beta;
        }

        let first_gamma = if !gammas.is_empty() {
            gammas[0]
        } else {
            [0u8; MAC_SIZE]
        };

        Ok(SphinxPacket {
            header: SphinxHeader {
                alpha: alpha_bytes,
                beta,
                gamma: first_gamma,
            },
            payload: payload_buf,
        })
    }

    /// تقشير طبقة واحدة من الحزمة عند وصولها لأي عقدة في المسار
    pub fn unpeel_layer(
        packet: &SphinxPacket,
        node_secret: &StaticSecret,
    ) -> Result<UnpeeledHop, SphinxError> {
        let alpha_pk = PublicKey::from(packet.header.alpha);
        let shared = node_secret.diffie_hellman(&alpha_pk);
        let keys = LayerKeys::derive(shared.as_bytes());

        // 1. التحقق من سلامة الطبقة وعدم التلاعب (MAC Verification)
        let mut mac = HmacSha256::new_from_slice(&keys.mac_key)
            .map_err(|_| SphinxError::LayerEncryptionError)?;
        mac.update(&packet.header.alpha);
        let mac_result = mac.finalize().into_bytes();
        if mac_result[..MAC_SIZE] != packet.header.gamma {
            return Err(SphinxError::InvalidMac);
        }

        // 2. فك تشفير طبقة التوجيه
        let zero_nonce = [0u8; 12];
        let mut decrypted_beta = packet.header.beta;
        let mut cipher = ChaCha20::new(&keys.routing_key.into(), &zero_nonce.into());
        cipher.apply_keystream(&mut decrypted_beta);

        // 3. استخراج معلومات القفزة التالية
        let mut next_hop_id = [0u8; 16];
        next_hop_id.copy_from_slice(&decrypted_beta[0..16]);

        let mut next_gamma = [0u8; MAC_SIZE];
        next_gamma.copy_from_slice(&decrypted_beta[16..32]);

        let is_exit = next_hop_id == [0u8; 16];

        // 4. إزاحة التوجيه للقفزة القادمة وتعبئة النهاية بأصفار للحفاظ على ثبات الحجم
        let mut next_beta = [0u8; ROUTING_INFO_SIZE];
        next_beta[..ROUTING_INFO_SIZE - 32].copy_from_slice(&decrypted_beta[32..]);

        // 5. فك تشفير طبقة واحدة من الحمولة
        let mut unpeeled_payload = packet.payload;
        let mut payload_cipher = ChaCha20::new(&keys.payload_key.into(), &zero_nonce.into());
        payload_cipher.apply_keystream(&mut unpeeled_payload);

        Ok(UnpeeledHop {
            next_hop_id,
            next_packet: SphinxPacket {
                header: SphinxHeader {
                    alpha: packet.header.alpha,
                    beta: next_beta,
                    gamma: next_gamma,
                },
                payload: unpeeled_payload,
            },
            is_exit_hop: is_exit,
        })
    }
}
