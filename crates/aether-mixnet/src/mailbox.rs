//! aether-mixnet::mailbox
//! بروتوكول صندوق البريد الأعمى المؤقت (Blind Ephemeral Mixnet Mailbox).
//! يحل مشكلة استلام الرسائل في حال كان المستلم غير متصل (Offline Messaging) دون الحاجة لأي خادم مركزي أو كشف الهوية.

use crate::sphinx::SphinxPacket;
use sha2::{Sha256, Digest};
use std::collections::HashMap;

/// تذكرة استرجاع عمياء لمرة واحدة
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RetrievalTag([u8; 32]);

impl RetrievalTag {
    /// توليد وسم استرجاع أعمى لا يمكن ربطه بعنوان المستخدم الحقيقي
    pub fn derive_from_ticket(user_private_seed: &[u8; 32], epoch_day: u64) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(b"aether-blind-mailbox-tag-v1");
        hasher.update(user_private_seed);
        hasher.update(&epoch_day.to_le_bytes());
        let result = hasher.finalize();
        let mut tag = [0u8; 32];
        tag.copy_from_slice(&result);
        Self(tag)
    }
}

/// صندوق بريد مؤقت مشفر ومجهول على عُقد البوابة (Nym Gateways)
pub struct BlindMailboxStore {
    // تخزين الحزم المشفرة تحت وسوم عمياء فقط دون معرفة هوية المالك
    storage: HashMap<RetrievalTag, Vec<SphinxPacket>>,
}

impl BlindMailboxStore {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    /// إيداع حزمة مشفرة للمستلم الغائب (تخزين أعمى)
    pub fn deposit_packet(&mut self, tag: RetrievalTag, packet: SphinxPacket) {
        self.storage.entry(tag).or_default().push(packet);
    }

    /// استرجاع الحزم فور عودة المستلم للاتصال مع الحذف الفوري التلقائي (Self-Destruct on Pull)
    pub fn pull_and_destroy(&mut self, tag: &RetrievalTag) -> Vec<SphinxPacket> {
        self.storage.remove(tag).unwrap_or_default()
    }
}
