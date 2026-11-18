//! aether-chaff: مولد حركة المرور الدائمة وحزم التضليل (Cover Traffic Generator)
//! يحافظ على معدل إرسال حزم ثابت بنمط Poisson Distribution لمنع تحليل التوقيت (Timing Analysis).

use std::time::Duration;
use rand::thread_rng;
use rand_distr::{Exp, Distribution};
use tokio::sync::mpsc;

pub const SPHINX_PACKET_SIZE: usize = 1024; // الحجم القياسي الثابت لكل حزمة

pub enum PacketType {
    RealMessage(Vec<u8>),
    DummyPacket([u8; SPHINX_PACKET_SIZE]),
}

pub struct ChaffEngine {
    lambda_rate: f64, // متوسط عدد الحزم بالثانية (مثلاً 1.5)
}

impl ChaffEngine {
    pub fn new(lambda_rate: f64) -> Self {
        Self { lambda_rate }
    }

    /// تشغيل حلقة الإرسال المستمر غير المتوقفة
    pub async fn run_loop(
        &self,
        mut outbound_msg_rx: mpsc::Receiver<Vec<u8>>,
        network_sender: mpsc::Sender<PacketType>,
    ) {
        let exp = Exp::new(self.lambda_rate).expect("Invalid lambda rate for Poisson distribution");

        loop {
            // حساب التأخير العشوائي التالي لكسر انتظام التوقيت ضد الذكاء الاصطناعي والمحللين
            let delay_secs = exp.sample(&mut thread_rng());
            tokio::time::sleep(Duration::from_secs_f64(delay_secs)).await;

            // هل توجد رسالة أو جزء وسائط حقيقي في الطابور؟
            let packet = match outbound_msg_rx.try_recv() {
                Ok(real_bytes) => PacketType::RealMessage(real_bytes),
                Err(_) => {
                    // إذا لم يكتب المستخدم شيئاً، نرسل حزمة عشوائية مطابقة في الشكل والحجم
                    let mut dummy = [0u8; SPHINX_PACKET_SIZE];
                    rand::RngCore::fill_bytes(&mut thread_rng(), &mut dummy);
                    PacketType::DummyPacket(dummy)
                }
            };

            let _ = network_sender.send(packet).await;
        }
    }
}
