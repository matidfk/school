use iced::{Element, widget::{button, text}};

use crate::{item::Item, Message};

#[derive(Debug)]
pub struct PurchasedItem {
    pub item: Item,
    pub quantity: u8,
}

impl PurchasedItem {
    pub fn new(item: Item) -> Self {
        Self { item, quantity: 1 }
    }

    pub fn render(&self) -> Element<Message> {
        button(text(format!(
            "{}, £{} x {}",
            self.item.name, self.item.price, self.quantity
        )))
        .padding(20)
        .into()
    }
}
