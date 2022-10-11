use std::{collections::HashMap, fs};
use serde::{Serialize, Deserialize};

use crate::item::Item;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ItemDB {
    pub items: HashMap<u64, Item>,
}

impl ItemDB {
    pub fn load(path: &str) -> Self {
        let string = fs::read_to_string(path).expect("Couldn't read file");
        serde_json::from_str::<Self>(&string).expect("Couldn't deserialize file")
    }

    pub fn get_item(&self, barcode: u64) -> Option<&Item> {
        self.items.get(&barcode)
    }
}