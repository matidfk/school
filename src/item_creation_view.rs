use iced::{
    widget::{button, column, container, image, row, scrollable, text, text_input, Column, Space},
    Length,
};

use crate::{item::Item, item_db::ItemDB, theme::ButtonStyle, utils::get_handle, ViewIndex};
use crate::{Element, Message};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ItemCreationView {}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemCreationMessage {}

impl ItemCreationView {
    pub fn view(&self) -> Element {
        text("poo").into()
    }
    pub fn update(&mut self, message: ItemCreationMessage, item_db: &mut ItemDB) {
        // match message {
        //     InventoryMessage::SearchChanged(value) => self.input_search = value,
        //     InventoryMessage::ModifyAmountInStock(item, amount) => {
        //         item_db.modify_quantity(&item, amount)
        //     }
        // }
    }
}
