//! aether-core: النواة الشاملة لتطبيق أثير
//! تجمع محرك التشفير الهجين، عميل Nym Mixnet، تيار الحزم المستمرة (Chaff)، ومعالج الوسائط اللامركزي.

pub mod media;
pub mod duress;

pub use aether_crypto as crypto;
pub use aether_mixnet as mixnet;
pub use aether_chaff as chaff;

use mixnet::{AetherMixnetClient, NetworkTopology};
use chaff::ChaffEngine;
use tokio::sync::mpsc;
use tracing::info;

pub struct AetherEngine {
    pub client: AetherMixnetClient,
    pub chaff: ChaffEngine,
}

impl AetherEngine {
    pub fn initialize() -> Self {
        let topology = NetworkTopology::new_mock();
        let (client, _gateway_rx, _incoming_tx) = AetherMixnetClient::new(topology);
        let chaff = ChaffEngine::new(1.2); // 1.2 حزمة وهمية في الثانية بتوزيع بواسوني

        info!("تمت تهيئة نواة أثير (Aether Core) ومحرك الـ Mixnet بنجاح");
        Self { client, chaff }
    }
}
