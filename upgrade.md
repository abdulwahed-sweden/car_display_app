# نظام ترجمة أكواد أعطال السيارات بلغة Rust

## الهيكل العام للمشروع

### ملف `Cargo.toml`
```toml
[package]
name = "car_dtc_translator"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rayon = "1.8" # للمعالجة المتوازية (اختياري)
### الكود الرئيسي (main.rs)

#### 1. تعريف هياكل البيانات
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct DtcEntry {
    code: String,
    description: String,
    category: String,
    cause: String,
    severity: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    translations: Option<TranslationSet>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TranslationSet {
    // العربية بلهجاتها
    khaleeji: DialectTranslation,
    masri: DialectTranslation,
    shami: DialectTranslation,
    maghribi: DialectTranslation,
    
    // الإنجليزية
    english_formal: SimpleTranslation,
    english_simple: SimpleTranslation,
    
    // السويدية
    swedish_formal: SimpleTranslation,
    swedish_simple: SimpleTranslation,
}

#[derive(Debug, Serialize, Deserialize)]
struct DialectTranslation {
    region: String,
    fault: String,
    cause: String,
    severity: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SimpleTranslation {
    region: String,
    fault: String,
    cause: String,
    severity: String,
}
#### 2. دالة توليد الترجمات
fn generate_translations(description: &str, cause: &str, severity: &str) -> TranslationSet {
    TranslationSet {
        khaleeji: DialectTranslation {
            region: "🇸🇦 اللهجة الخليجية".into(),
            fault: format!("السيارة تستلم بيانات غلط من أكثر من كمبيوتر.\n\nالعطل الأصلي: {}", description),
            cause: format!("خلل بالتواصل بين كمبيوترات السيارة.\n\nالسبب الأصلي: {}", cause),
            severity: "مهم وخطير.".into(),
        },
        masri: DialectTranslation {
            region: "🇪🇬 اللهجة المصرية".into(),
            fault: format!("العربية بتستقبل بيانات غلط من أكتر من وحدة تحكم.\n\nالعطل الأصلي: {}", description),
            cause: format!("فيه مشكلة في الاتصال بين وحدات التحكم بتاعة العربية.\n\nالسبب الأصلي: {}", cause),
            severity: "مهم وخطير.".into(),
        },
        shami: DialectTranslation {
            region: "🇸🇾 اللهجة الشامية".into(),
            fault: format!("السيارة عم تستقبل معلومات خاطئة من أكتر من وحدة تحكم.\n\nالعطل الأصلي: {}", description),
            cause: format!("مشكلة باتصال وحدات التحكم ببعضها.\n\nالسبب الأصلي: {}", cause),
            severity: "مهم وحرج.".into(),
        },
        maghribi: DialectTranslation {
            region: "🇲🇦 اللهجة المغاربية".into(),
            fault: format!("الطوموبيل كتوصلها معلومات خاطئة من بزاف ديال الوحدات.\n\nالعطل الأصلي: {}", description),
            cause: format!("مشكل فالاتصال بين الوحدات الإلكترونية.\n\nالسبب الأصلي: {}", cause),
            severity: "مهمة بزاف وخطيرة.".into(),
        },
        english_formal: SimpleTranslation {
            region: "🇺🇸 English (Formal)".into(),
            fault: format!("Invalid data received from multiple control modules.\n\nOriginal: {}", description),
            cause: format!("Communication error between ECUs or CAN bus failure.\n\nCause: {}", cause),
            severity: severity.into(),
        },
        english_simple: SimpleTranslation {
            region: "🇺🇸 English (Simple)".into(),
            fault: "Car computers receiving wrong data from multiple systems".into(),
            cause: "Communication problem between car's control units".into(),
            severity: "Critical - Requires immediate attention".into(),
        },
        swedish_formal: SimpleTranslation {
            region: "🇸🇪 Svenska (Formell)".into(),
            fault: format!("Ogiltiga data mottagna från flera styrmoduler.\n\nOriginal: {}", description),
            cause: format!("Kommunikationsfel mellan ECU:er eller CAN-bussfel.\n\nOrsak: {}", cause),
            severity: severity.into(),
        },
        swedish_simple: SimpleTranslation {
            region: "🇸🇪 Svenska (Enkel)".into(),
            fault: "Bilen får felaktig data från flera system".into(),
            cause: "Problem i kommunikationen mellan bilens datorer".into(),
            severity: "Allvarligt - Kräver omedelbar åtgärd".into(),
        },
    }
}
3. الدالة الرئيسية
#### 3. الدالة الرئيسية
    // 1. قراءة ملف JSON الأصلي
    let path = "dtc_codes.json";
    let data = fs::read_to_string(path)?;
    let mut dtc_list: Vec<DtcEntry> = serde_json::from_str(&data)?;

    // 2. معالجة البيانات (باستخدام rayon للملفات الكبيرة)
    #[cfg(feature = "parallel")]
    dtc_list.par_iter_mut().for_each(|dtc| {
        dtc.translations = Some(generate_translations(
            &dtc.description,
            &dtc.cause,
            &dtc.severity
        ));
    });

    #[cfg(not(feature = "parallel"))]
    for dtc in dtc_list.iter_mut() {
        dtc.translations = Some(generate_translations(
            &dtc.description,
            &dtc.cause,
            &dtc.severity
        ));
    }

    // 3. حفظ الملف المحدث
    let updated_json = serde_json::to_string_pretty(&dtc_list)?;
    fs::write(path, updated_json)?;
    
    println!("✅ تم تحديث الملف بنجاح بإضافة جميع الترجمات!");
    Ok(())
}
مثال لملف الإدخال والإخراج
ملف الإدخال (dtc_codes.json)
json
[
  {
    "code": "U0500",
    "description": "Invalid Data Received From Multiple Modules",
    "category": "Network",
    "cause": "Data Integrity",
    "severity": "Critical"
  }
]
ملف الإخراج (بعد المعالجة)
json
[
  {
    "code": "U0500",
    "description": "Invalid Data Received From Multiple Modules",
    "category": "Network",
    "cause": "Data Integrity",
    "severity": "Critical",
    "translations": {
      "khaleeji": {
        "region": "🇸🇦 اللهجة الخليجية",
        "fault": "السيارة تستلم بيانات غلط من أكثر من كمبيوتر...",
        "cause": "خلل بالتواصل بين كمبيوترات السيارة...",
        "severity": "مهم وخطير."
      },
      "english_simple": {
        "region": "🇺🇸 English (Simple)",
        "fault": "Car computers receiving wrong data...",
        "cause": "Communication problem between...",
        "severity": "Critical - Requires immediate attention"
      }
      // ... باقي الترجمات
    }
  }
]
نصائح للاستخدام
للملفات الكبيرة (4000+ عطل):

bash
cargo run --release --features "parallel"
لإنشاء ملف جديد بدل تعديل الملف الأصلي:

rust
fs::write("dtc_codes_translated.json", updated_json)?;
لإضافة لغات جديدة:

أضف حقول جديدة في TranslationSet

أضف منطق الترجمة في generate_translations

لتحسين الأداء:

استخدم BufReader للملفات الكبيرة جداً

فكر في استخدام قاعدة بيانات مثل SQLite إذا تجاوزت البيانات 10,000 سجل