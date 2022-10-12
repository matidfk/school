use serde::{Deserialize, Serialize};
use std::fs;

use crate::item::Item;

// TODO: maybe load into a hashmap for performance

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ItemDB {
    // pub items: HashMap<u64, Item>,
    pub items: Vec<Item>,
}

impl ItemDB {
    pub fn load(path: &str) -> Self {
        let string = fs::read_to_string(path).expect("Couldn't read file");
        serde_json::from_str::<Self>(&string).expect("Couldn't deserialize file")
    }

    pub fn get_item(&self, code: u64) -> Option<&Item> {
        self.items.iter().find(|item| item.code == code)
        // self.items.get(&barcode)
    }

    pub fn save(&self, path: &str) {
        // serde_json::from_str::<Self>(&string).expect("Couldn't deserialize file")
        fs::write(path, serde_json::to_string_pretty::<Self>(self).expect("Couldn't serialize Item Database"));
    }
}
