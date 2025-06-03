# 🚀 Car DTC Translator Upgrade Guide

A guide to upgrade your Rust project to support multi-language translations for OBD-II DTCs, including 8 variations (4 Arabic dialects, 2 English types, 2 Swedish types).

---

## 📦 `Cargo.toml`

```toml
[package]
name = "car_dtc_translator"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rayon = "1.8" # Optional: for parallel processing
```

---

## 🧱 1. Data Structures (`main.rs`)

```rust
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
    khaleeji: DialectTranslation,
    masri: DialectTranslation,
    shami: DialectTranslation,
    maghribi: DialectTranslation,
    english_formal: SimpleTranslation,
    english_simple: SimpleTranslation,
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
```

---

## 🔄 2. Translation Generator

```rust
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
```

---

## 🧠 3. Main Execution Function

```rust
fn main() -> std::io::Result<()> {
    let path = "dtc_codes.json";
    let data = std::fs::read_to_string(path)?;
    let mut dtc_list: Vec<DtcEntry> = serde_json::from_str(&data)?;

    #[cfg(feature = "parallel")]
    dtc_list.par_iter_mut().for_each(|dtc| {
        dtc.translations = Some(generate_translations(&dtc.description, &dtc.cause, &dtc.severity));
    });

    #[cfg(not(feature = "parallel"))]
    for dtc in dtc_list.iter_mut() {
        dtc.translations = Some(generate_translations(&dtc.description, &dtc.cause, &dtc.severity));
    }

    let updated_json = serde_json::to_string_pretty(&dtc_list)?;
    std::fs::write(path, updated_json)?;

    println!("✅ File updated successfully with translations.");
    Ok(())
}
```

---

## 📂 Input Example

**File:** `dtc_codes.json`

```json
[
  {
    "code": "U0500",
    "description": "Invalid Data Received From Multiple Modules",
    "category": "Network",
    "cause": "Data Integrity",
    "severity": "Critical"
  }
]
```

---

## 📤 Output Example

```json
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
      // ... other translations
    }
  }
]
```

---

## 💡 Usage Tips

### For large files:

```bash
cargo run --release --features "parallel"
```

### Save to a new file:

```rust
fs::write("dtc_codes_translated.json", updated_json)?;
```

### Add new languages:

- Add fields to `TranslationSet`
- Extend `generate_translations` logic

### Performance boost:

- Use `BufReader` for very large files
- Consider SQLite for >10k entries