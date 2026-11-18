//! aether-mixnet::topology
//! إدارة طوبولوجيا شبكة Nym: اكتشاف العُقد، واختيار المسار العشوائي (3-Hop Random Path Selection).

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Debug)]
pub struct MixNode {
    pub id: [u8; 16],
    pub public_key: [u8; 32],
    pub layer: u8, // 1, 2, or 3
    pub host: String,
}

#[derive(Clone, Debug)]
pub struct GatewayNode {
    pub id: [u8; 16],
    pub public_key: [u8; 32],
    pub host: String,
}

#[derive(Clone, Debug)]
pub struct NetworkTopology {
    pub layer1_nodes: Vec<MixNode>,
    pub layer2_nodes: Vec<MixNode>,
    pub layer3_nodes: Vec<MixNode>,
    pub gateways: Vec<GatewayNode>,
}

#[derive(Clone, Debug)]
pub struct SelectedRoute {
    pub hop1: MixNode,
    pub hop2: MixNode,
    pub hop3: MixNode,
    pub destination_gateway: GatewayNode,
}

impl NetworkTopology {
    pub fn new_mock() -> Self {
        // عُقد افتراضية للتطوير والاختبار المحلي
        let make_node = |id_byte: u8, layer: u8| MixNode {
            id: [id_byte; 16],
            public_key: [id_byte * 2 + 1; 32],
            layer,
            host: format!("127.0.0.1:900{}", id_byte),
        };

        Self {
            layer1_nodes: vec![make_node(1, 1), make_node(2, 1)],
            layer2_nodes: vec![make_node(3, 2), make_node(4, 2)],
            layer3_nodes: vec![make_node(5, 3), make_node(6, 3)],
            gateways: vec![GatewayNode {
                id: [9; 16],
                public_key: [19; 32],
                host: "127.0.0.1:9090".to_string(),
            }],
        }
    }

    /// اختيار مسار عشوائي غير قابل للتتبع عبر الطبقات الثلاث
    pub fn select_random_route(&self, dest_gateway: Option<GatewayNode>) -> Option<SelectedRoute> {
        let mut rng = thread_rng();
        let hop1 = self.layer1_nodes.choose(&mut rng)?.clone();
        let hop2 = self.layer2_nodes.choose(&mut rng)?.clone();
        let hop3 = self.layer3_nodes.choose(&mut rng)?.clone();
        let gateway = dest_gateway.or_else(|| self.gateways.choose(&mut rng).cloned())?;

        Some(SelectedRoute {
            hop1,
            hop2,
            hop3,
            destination_gateway: gateway,
        })
    }
}
