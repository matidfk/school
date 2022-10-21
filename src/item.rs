use serde::{Deserialize, Serialize};

/// Represents an item
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub code: u64,
    pub name: String,
    pub price: u32,
    pub image_path: Option<String>,
    pub amount_in_stock: u32,
}
