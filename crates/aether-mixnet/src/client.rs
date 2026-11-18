//! aether-mixnet::client
//! عميل شبكة Nym لتطبيق أثير:
//! يتولى إرسال الحزم الحقيقية والوهمية (Sphinx Packets)، الاتصال بالبوابة (Gateway)، واستلام الرسائل دون كشف أي أثر.

use crate::sphinx::{SphinxBuilder, SphinxPacket, SPHINX_PAYLOAD_SIZE};
use crate::topology::{NetworkTopology, SelectedRoute};
use crate::surb::SURB;
use tokio::sync::mpsc;
use tracing::{info, warn};

pub struct AetherMixnetClient {
    topology: NetworkTopology,
    gateway_tx: mpsc::Sender<SphinxPacket>,
    incoming_rx: mpsc::Receiver<Vec<u8>>,
}

impl AetherMixnetClient {
    pub fn new(topology: NetworkTopology) -> (Self, mpsc::Receiver<SphinxPacket>, mpsc::Sender<Vec<u8>>) {
        let (gateway_tx, gateway_rx) = mpsc::channel(100);
        let (incoming_tx, incoming_rx) = mpsc::channel(100);

        let client = Self {
            topology,
            gateway_tx,
            incoming_rx,
        };

        (client, gateway_rx, incoming_tx)
    }

    /// إرسال رسالة حقيقية أو جزء من وسائط (صوت / صورة / مستند) عبر مسار Sphinx عشوائي
    pub async fn send_real_message(
        &self,
        recipient_public_key: [u8; 32],
        payload: &[u8],
    ) -> Result<(), &'static str> {
        let route = self.topology.select_random_route(None)
            .ok_or("تعذر العثور على مسار عُقد متاح في شبكة Nym")?;

        let hop_keys = [
            route.hop1.public_key,
            route.hop2.public_key,
            route.hop3.public_key,
            recipient_public_key,
        ];

        let hop_ids = [
            route.hop2.id,
            route.hop3.id,
            route.destination_gateway.id,
            [0u8; 16], // علامة الوصول النهائي للمستلم
        ];

        let packet = SphinxBuilder::build_packet(&hop_keys, &hop_ids, payload)
            .map_err(|_| "فشل بناء حزمة Sphinx المشفرة")?;

        self.gateway_tx.send(packet).await
            .map_err(|_| "فشل إرسال الحزمة إلى بوابة Nym Gateway")?;

        info!("تم إرسال حزمة Sphinx مشفرة بنجاح عبر 3 عُقد خلط");
        Ok(())
    }

    /// إرسال حزمة وهمية (Cover / Chaff Traffic) متطابقة الحجم تماماً
    pub async fn send_dummy_packet(&self) -> Result<(), &'static str> {
        let route = self.topology.select_random_route(None)
            .ok_or("تعذر اختيار مسار للحزمة الوهمية")?;

        let mut dummy_payload = [0u8; SPHINX_PAYLOAD_SIZE];
        rand::RngCore::fill_bytes(&mut rand::thread_rng(), &mut dummy_payload);

        let hop_keys = [
            route.hop1.public_key,
            route.hop2.public_key,
            route.hop3.public_key,
        ];

        let hop_ids = [
            route.hop2.id,
            route.hop3.id,
            [0u8; 16], // تموت الحزمة داخل الشبكة تلقائياً ولا تُسلّم لأحد
        ];

        let packet = SphinxBuilder::build_packet(&hop_keys, &hop_ids, &dummy_payload)
            .map_err(|_| "فشل بناء الحزمة الوهمية")?;

        self.gateway_tx.send(packet).await
            .map_err(|_| "فشل حقن الحزمة الوهمية في الشبكة")?;

        Ok(())
    }

    /// استلام الرسائل الواردة المفكوكة
    pub async fn recv_next_message(&mut self) -> Option<Vec<u8>> {
        self.incoming_rx.recv().await
    }
}
