//! aether-mixnet::surb
//! كتل الرد لمرة واحدة (Single-Use Reply Blocks - SURBs).
//! تتيح للمستلم الرد على الرسالة دون معرفة عنوان أو هوية أو موقع المرسل الأصلي.

use crate::sphinx::SphinxHeader;

#[derive(Clone, Debug)]
pub struct SURB {
    /// رأس Sphinx مشفر وموجّه مسبقاً نحو عنوان المرسل الأصلي
    pub precomputed_header: SphinxHeader,
    /// مفتاح تشفير الحمولة الذي يحتفظ به المرسل محلياً فقط لفك الرد
    pub reply_encryption_key: [u8; 32],
}

impl SURB {
    pub fn new(header: SphinxHeader, key: [u8; 32]) -> Self {
        Self {
            precomputed_header: header,
            reply_encryption_key: key,
        }
    }
}
