use iced::{widget::text_input, Element, Renderer};

use crate::theme::MyTheme;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AddItemView {
    item_name: String,
}

#[derive(Clone, PartialEq, Debug)]
pub enum AddItemMessage {
    NameChanged(String),
}

impl AddItemView {
    pub fn view(&self) -> Element<AddItemMessage, Renderer<MyTheme>> {
        text_input("Item name", &self.item_name, AddItemMessage::NameChanged).into()
    }

    pub fn update(&mut self, message: AddItemMessage) {
        match message {
            AddItemMessage::NameChanged(value) => self.item_name = value,
        }
    }
}
