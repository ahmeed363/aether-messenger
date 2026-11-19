//! aether-core::duress
//! نظام بصمة الطوارئ والمخزن المموّه (Duress Fingerprint & Decoy Vault).
//! يتيح للمستخدم الإنكار المعقول الكامل (Plausible Deniability) في حال أُجبر قسراً على فتح التطبيق.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VaultMode {
    /// الوضع الحقيقي: فك تشفير الجلسات والمفاتيح الفعلية في الذاكرة
    RealVault,
    /// الوضع المموه (الطوارئ): فتح محادثات وهمية بريئة تماماً دون المساس بالمفاتيح الحقيقية
    DecoyVault,
}

pub struct DecoyMessage {
    pub sender: String,
    pub text: String,
    pub time: String,
}

pub struct DuressManager {
    _primary_finger_id: [u8; 16],
    duress_finger_id: [u8; 16],
}

impl DuressManager {
    pub fn new(primary_id: [u8; 16], duress_id: [u8; 16]) -> Self {
        Self {
            _primary_finger_id: primary_id,
            duress_finger_id: duress_id,
        }
    }

    /// فحص بصمة الدخول وتحديد هل يتم فتح المخزن الحقيقي أم المموه
    pub fn evaluate_biometric_input(&self, scanned_finger_id: &[u8; 16]) -> VaultMode {
        if scanned_finger_id == &self.duress_finger_id {
            // بصمة الطوارئ: يتم تفعيل الوضع المموه
            VaultMode::DecoyVault
        } else {
            VaultMode::RealVault
        }
    }

    /// إنشاء قائمة محادثات وهمية بريئة لعرضها في وضع الإنكار المقبول
    pub fn generate_decoy_chats() -> Vec<DecoyMessage> {
        vec![
            DecoyMessage {
                sender: "مجموعة العائلة".to_string(),
                text: "موعد العشاء غداً الساعة 8 مساءً إن شاء الله".to_string(),
                time: "19:40".to_string(),
            },
            DecoyMessage {
                sender: "خدمات التوصيل".to_string(),
                text: "طلبك في الطريق إليك مع المندوب".to_string(),
                time: "14:15".to_string(),
            },
            DecoyMessage {
                sender: "نادي اللياقة".to_string(),
                text: "تذكير: موعد جلسة التدريب القادمة صباح الغد".to_string(),
                time: "11:00".to_string(),
            },
        ]
    }
}
