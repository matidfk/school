use chrono;

use crate::{item::Item, utils::format_price};

/// Stores the state of a transaction
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Transaction {
    pub items: Vec<TransactionItem>,
}

/// An item in a transaction
#[derive(Debug, Clone, PartialEq)]
pub struct TransactionItem {
    pub item: Item,
    pub quantity: u32,
}

impl TransactionItem {
    /// Creates a new `TransactionItem` with quantity 1 given an `Item`
    fn new(item: &Item) -> Self {
        Self {
            item: item.clone(),
            quantity: 1,
        }
    }
}

impl Transaction {
    /// Calculates total price of all items
    pub fn total_price(&self) -> u32 {
        self.items.iter().fold(0, |sum, item| {
            sum + (item.item.price * item.quantity as u32)
        })
    }

    /// Adds an item to the transaction
    /// Returns true if the item has existed already
    pub fn add_item(&mut self, item: &Item) -> bool {
        let found = self.items.iter_mut().find(|t_item| &t_item.item == item);

        match found {
            // If item exists already, add 1 to the quantity
            Some(found) => {
                found.quantity += 1;
                return true;
            }
            // If not, create it with quantity 1
            None => {
                self.items.push(TransactionItem::new(item));
                return false;
            }
        }
    }

    /// Modifies the quantity of an `Item` in the transaction
    pub fn modify_quantity(&mut self, item: &Item, quantity: i32) {
        let (index, found) = self
            .items
            .iter_mut()
            .enumerate()
            .find(|(_index, t_item)| &t_item.item == item)
            .unwrap();

        let (new_qty, overflow) = found.quantity.overflowing_add_signed(quantity);

        if overflow || new_qty == 0 {
            // remove item
            self.items.remove(index);
        } else {
            // modify quantity
            found.quantity = new_qty;
        }
    }

    /// Create a string with the receipt in plain text
    pub fn generate_receipt(&self) -> String {
        if self.items.len() == 0 {
            "".to_string();
        }
        let mut string = String::from("\n\n\n\n");
        string.push_str(&format!(
            "gypsy shop              {}\n",
            chrono::offset::Local::now().format("%d. %m. %H:%M:%S")
        ));
        string.push_str("========================================\n");

        //  3     30      7
        // qty | name | total

        for item in &self.items {
            string.push_str(&format!(
                "{0: <3}{1: <30}{2: >7}\n",
                &item.quantity,
                &item.item.name,
                format_price(&item.item.price * item.quantity),
            ));
        }

        string.push_str("========================================\n");

        string.push_str(&format!(
            "{0: <33}{1: >7}\n",
            "TOTAL PRICE PAID:",
            format_price(self.total_price())
        ));

        string
    }
}
