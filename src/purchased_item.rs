use std::vec;

use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, row, text},
    Alignment, Element, Length,
};

use crate::{item::Item, Message};

#[derive(Debug, Clone, PartialEq)]
pub struct PurchasedItem {
    pub item: Item,
    pub quantity: u8,
}

impl PurchasedItem {
    pub fn new(item: Item) -> Self {
        Self { item, quantity: 1 }
    }

    pub fn render(&self) -> Element<Message> {
        // row![
        // text(format!(
        //     "{} £{} x{}",
        //     self.item.name, self.item.price, self.quantity
        // ))
        //     .height(Length::Fill)
        //     .horizontal_alignment(Horizontal::Center)
        //     .vertical_alignment(Vertical::Bottom),
        //     column![
        //         button(text("+")).on_press(Message::IncrementCount(self.item.clone())),
        //         button(text("-")).on_press(Message::DecrementCount(self.item.clone())),
        //     ]
        // ]
        // .into()

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
        // .width(Length::Fill)
        .align_items(Alignment::Fill)
        .into()

        // button(text(format!(
        //     "{}, £{} x {}",
        //     self.item.name, self.item.price, self.quantity
        // )).vertical_alignment(Alignment::Center).width(Length::Shrink))
        // .on_press(msg)
        // .padding(20)
        // .into()
    }
}
