# أثيـر | Aether Messenger
> **Ultra-Private, Post-Quantum, Metadata-Resistant Stealth Messenger**  
> تشفير هجين ما بعد الكم • شبكة Nym Mixnet • لا خادم • لا هوية • مقاومة تحليل التوقيت وحركة البيانات.

---

## 🛡️ المبادئ الأمنية الصارمة (Security Invariants)

1. **Zero-Identity & Local Biometrics:** لا أرقام هواتف، لا بريد إلكتروني، لا أسماء مستخدمين مركزية. المفتاح الرئيسي مولّد ومحمي محلياً عبر المعالج الآمن (Secure Enclave / StrongBox) ويُفتح ببصمة الإصبع الحيوية فقط.
2. **Nym Mixnet Routing:** توجيه الرسائل عبر عُقد خلط بيانات بنظام حزم Sphinx المشفرة متعددة الطبقات لمنع مراقبة مزودي الإنترنت وأجهزة الاستخبارات.
3. **Continuous Cover Traffic (Chaffing):** التطبيق يرسل حزماً بمعدل ثابت وتوزيع بواسوني (Poisson Distribution) سواء كتب المستخدم رسالة أم لا، فلا يستطيع أي مراقب معرفة متى يبدأ أو ينتهي الحديث.
4. **Hybrid Post-Quantum Cryptography:** دمج معيار NIST الجديد (ML-KEM-768 / Kyber) مع X25519 لضمان صمود المحادثات للأبد حتى ضد الحواسيب الكمومية.
5. **Instant RAM Zeroize & Plausible Deniability:** كل رسالة تستخدم مفتاحاً مؤقتاً لمرة واحدة يُمسح فوراً من الذاكرة العشوائية الفيزيائية بكتابة أصفار مكانها (`zeroize`). لا توجد توقيعات غير قابلة للنقض.
6. **WhatsApp-Grade UX:** إرسال رسائل صوتية، صور، فيديو، ومستندات بضغطة زر واحدة بنفس السلاسة والسرعة المعتادة عبر تقنية الـ Chunking اللامركزية.

---

## 📂 هيكلية المشروع (Repository Structure)

```text
Massenger/
├── crates/
│   ├── aether-core/          # المحرك الأساسي وإدارة الجلسات
│   ├── aether-crypto/        # التشفير الهجين (ML-KEM + X25519 + Ratchet)
│   ├── aether-mixnet/        # الربط مع Nym Sphinx Packets
│   ├── aether-chaff/         # مولد الحزم الوهمية المستمرة (Cover Traffic)
│   └── aether-uniffi/        # واجهات الربط الأصلية لـ iOS (Swift) و Android (Kotlin)
├── mobile/                   # واجهات وتطبيقات الهواتف
│   ├── ios/                  # تطبيق SwiftUI الأصلي
│   └── android/              # تطبيق Jetpack Compose الأصلي
└── docs/                     # المواصفات والتوثيق الأمني
```

---

## 🎨 الهوية والتصميم المعتمد
- **الاسم:** أثيـر (Aether)
- **الشعار:** المونوغرام الصفري (A + Zero Trace)
- **الألوان:** Deep Obsidian (`#0B0E14`) + Cyber Emerald (`#00E599`)
