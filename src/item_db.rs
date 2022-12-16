use serde::{Deserialize, Serialize};
use std::fs;

use crate::{item::Item, transaction::Transaction};

// TODO: maybe load into a hashmap for performance

/// A database of all `Item`s
#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ItemDB {
    // pub items: HashMap<u64, Item>,
    pub items: Vec<Item>,
}

impl ItemDB {
    /// Loads databse from JSON file given the path
    pub fn load(path: &str) -> Self {
        let string = fs::read_to_string(path).expect("Couldn't read file");
        serde_json::from_str::<Self>(&string).expect("Couldn't deserialize file")
    }

    /// Loads databse from YAML file given the path
    pub fn load_yaml(path: &str) -> Self {
        let string = fs::read_to_string(path).expect("Couldn't read file");
        serde_yaml::from_str::<Self>(&string).expect("Couldn't deserialize file")
    }

    /// Gets an item from the databse given a code
    pub fn get_item(&self, code: u64) -> Option<&Item> {
        self.items.iter().find(|item| item.barcode == code)
        // self.items.get(&barcode)
    }

    /// Updates quantities of items in the database from a transaction
    pub fn update_quantities_from_transaction(&mut self, transaction: &Transaction) {
        for transaction_item in transaction.items.iter() {
            self.modify_quantity(&transaction_item.item, -(transaction_item.quantity as i32));
        }
    }

    /// Modifies quantity of an item
    pub fn modify_quantity(&mut self, item: &Item, count: i32) {
        let found = self
            .items
            .iter_mut()
            .find(|i| i.barcode == item.barcode)
            .unwrap();

        found.amount_in_stock = found.amount_in_stock.saturating_add_signed(count);
    }

    /// Saves itself to a JSON file
    pub fn save(&self, path: &str) {
        // serde_json::from_str::<Self>(&string).expect("Couldn't deserialize file")
        fs::write(
            path,
            serde_json::to_string_pretty::<Self>(self).expect("Couldn't serialize Item Database"),
        )
        .unwrap();
    }

    /// Saves itself to a YAML file
    pub fn save_yaml(&self, path: &str) {
        fs::write(
            path,
            serde_yaml::to_string::<Self>(self).expect("Couldn't serialize Item Database"),
        )
        .unwrap();
    }
}
