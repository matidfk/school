use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub code: u64,
    pub name: String,
    pub price: u32,
    pub image_path: Option<String>
}
