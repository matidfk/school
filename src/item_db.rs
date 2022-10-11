use std::fs;
use serde::{Serialize, Deserialize};

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
}