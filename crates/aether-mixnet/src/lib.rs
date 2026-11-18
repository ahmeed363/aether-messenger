//! aether-mixnet: محرك شبكة Nym وحزم Sphinx المشفرة لتطبيق أثير
//! يوفر توجيهاً بصلّياً متعدد الطبقات (Onion Routing) مع حجم حزم ثابت لمقاومة المراقبة وتحليل التوقيت.

pub mod sphinx;
pub mod topology;
pub mod surb;
pub mod client;
pub mod mailbox;

pub use sphinx::{SphinxBuilder, SphinxPacket, SphinxHeader, SPHINX_PAYLOAD_SIZE};
pub use topology::{NetworkTopology, MixNode, GatewayNode, SelectedRoute};
pub use surb::SURB;
pub use client::AetherMixnetClient;

#[cfg(test)]
mod tests {
    use super::*;
    use x25519_dalek::{StaticSecret, PublicKey};
    use rand::rngs::OsRng;

    #[test]
    fn test_sphinx_packet_build_and_unpeel_flow() {
        let mut rng = OsRng;

        // إعداد 3 عُقد Mixnet بمفاتيحها الخاصة والعامة
        let node1_sec = StaticSecret::random_from_rng(&mut rng);
        let node1_pub = PublicKey::from(&node1_sec);

        let node2_sec = StaticSecret::random_from_rng(&mut rng);
        let node2_pub = PublicKey::from(&node2_sec);

        let node3_sec = StaticSecret::random_from_rng(&mut rng);
        let node3_pub = PublicKey::from(&node3_sec);

        let hops_keys = [
            *node1_pub.as_bytes(),
            *node2_pub.as_bytes(),
            *node3_pub.as_bytes(),
        ];

        let hops_ids = [
            [2u8; 16], // بعد العقدة 1 اذهب للعقدة 2
            [3u8; 16], // بعد العقدة 2 اذهب للعقدة 3
            [0u8; 16], // العقدة 3 هي عقدة الخروج (Exit Node)
        ];

        let secret_message = b"Aether Post-Quantum Mixnet Secret Message!";

        // 1. إنشاء حزمة Sphinx المشفرة
        let packet = SphinxBuilder::build_packet(&hops_keys, &hops_ids, secret_message)
            .expect("فشل بناء الحزمة");

        // التأكد من أن الحجم مطابق دائماً للحجم المعياري
        assert_eq!(packet.payload.len(), SPHINX_PAYLOAD_SIZE);

        // 2. محاكاة وصول الحزمة للعقدة الأولى وتقشيرها
        let hop1_result = SphinxBuilder::unpeel_layer(&packet, &node1_sec)
            .expect("فشل تقشير الطبقة الأولى");
        assert_eq!(hop1_result.next_hop_id, [2u8; 16]);
        assert!(!hop1_result.is_exit_hop);

        // 3. محاكاة وصول الحزمة للعقدة الثانية وتقشيرها
        let hop2_result = SphinxBuilder::unpeel_layer(&hop1_result.next_packet, &node2_sec)
            .expect("فشل تقشير الطبقة الثانية");
        assert_eq!(hop2_result.next_hop_id, [3u8; 16]);
        assert!(!hop2_result.is_exit_hop);

        // 4. محاكاة وصول الحزمة للعقدة الثالثة (عقدة التسليم)
        let hop3_result = SphinxBuilder::unpeel_layer(&hop2_result.next_packet, &node3_sec)
            .expect("فشل تقشير الطبقة الثالثة");
        assert!(hop3_result.is_exit_hop);
        assert_eq!(hop3_result.next_hop_id, [0u8; 16]);
    }
}
