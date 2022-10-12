use std::vec;

use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, container, row, text},
    Alignment, Element, Length, Renderer,
};

use crate::{item::Item, theme::MyTheme, Message};

#[derive(Debug, Clone, PartialEq)]
pub struct PurchasedItem {
    pub item: Item,
    pub quantity: u8,
}

impl PurchasedItem {
    pub fn new(item: Item) -> Self {
        Self { item, quantity: 1 }
    }

    pub fn render(&self) -> Element<Message, Renderer<MyTheme>> {
        container(
            row(vec![
                text(format!(
                    "{} £{} x{}",
                    self.item.name, self.item.price, self.quantity
                ))
                .vertical_alignment(Vertical::Center)
                .horizontal_alignment(Horizontal::Center)
                .width(Length::FillPortion(8))
                .into(),
                button(text("+").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::IncrementCount(self.item.clone()))
                    .width(Length::FillPortion(1))
                    .into(),
                button(text("-").horizontal_alignment(Horizontal::Center))
                    .on_press(Message::DecrementCount(self.item.clone()))
                    .width(Length::FillPortion(1))
                    .into(),
            ])
            .width(Length::Fill)
            .align_items(Alignment::Fill),
        )
        .width(Length::Fill)
        .into()
    }
}
