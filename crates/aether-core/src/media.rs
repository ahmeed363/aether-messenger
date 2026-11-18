//! aether-core::media
//! معالج المرفقات والوسائط الضخمة (صور، صوت، فيديو، مستندات)
//! يقطع الملف إلى شرائح (Chunks) متساوية لتمريرها ضمن حزم Sphinx وتيار الحزم الوهمية بدون خادم مركزي.

use aether_mixnet::SPHINX_PAYLOAD_SIZE;
use thiserror::Error;

pub const CHUNK_METADATA_HEADER_SIZE: usize = 32;
pub const MAX_MEDIA_CHUNK_PAYLOAD: usize = SPHINX_PAYLOAD_SIZE - CHUNK_METADATA_HEADER_SIZE;

#[derive(Error, Debug)]
pub enum MediaError {
    #[error("فشل تجميع شرائح المرفق")]
    AssemblyFailed,
    #[error("حجم الملف يتجاوز الحد المسموح للجلسة الفورية")]
    FileTooLarge,
}

#[derive(Clone, Debug)]
pub struct MediaChunk {
    pub transfer_id: [u8; 16], // معرّف عشوائي مؤقت للملف
    pub chunk_index: u32,      // رقم الشريحة الحالية
    pub total_chunks: u32,     // إجمالي الشرائح
    pub data: Vec<u8>,         // بيانات الشريحة المشفرة
}

pub struct MediaStreamer;

impl MediaStreamer {
    /// تقطيع ملف وسائط (صورة / تسجيل صوتي / فيديو) إلى شرائح متوافقة مع Nym Mixnet
    pub fn chunk_file(transfer_id: [u8; 16], raw_bytes: &[u8]) -> Vec<MediaChunk> {
        let chunk_size = MAX_MEDIA_CHUNK_PAYLOAD;
        let total_chunks = ((raw_bytes.len() + chunk_size - 1) / chunk_size) as u32;
        let mut chunks = Vec::with_capacity(total_chunks as usize);

        for (i, chunk_slice) in raw_bytes.chunks(chunk_size).enumerate() {
            chunks.push(MediaChunk {
                transfer_id,
                chunk_index: i as u32,
                total_chunks,
                data: chunk_slice.to_vec(),
            });
        }

        chunks
    }

    /// إعادة تجميع الشرائح في الذاكرة العشوائية فور استلامها وعرضها كملف كامل للمستخدم
    pub fn assemble_chunks(chunks: &mut [MediaChunk]) -> Result<Vec<u8>, MediaError> {
        chunks.sort_by_key(|c| c.chunk_index);
        let mut full_file = Vec::new();
        for chunk in chunks {
            full_file.extend_from_slice(&chunk.data);
        }
        Ok(full_file)
    }
}
