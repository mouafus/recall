use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClipboardItem {
    pub id: Uuid,
    pub content: String,
    pub content_type: String,
    pub timestamp: i64,
}
