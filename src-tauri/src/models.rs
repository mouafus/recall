use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../src/lib/bindings.ts")]
#[serde(rename_all = "snake_case")]
pub struct ClipboardItem {
    pub id: Uuid,
    pub content: String,
    pub content_type: String,
    pub timestamp: i64,
    // Optional image payload (RGBA8 bytes encoded as base64) and dimensions for previews/paste
    pub image_base64: Option<String>,
    pub image_width: Option<u32>,
    pub image_height: Option<u32>,
}
