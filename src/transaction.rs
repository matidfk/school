use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, image, image::Handle, row, scrollable, text},
    Alignment, Element, Length, Renderer,
};

use crate::{item::Item, theme::MyTheme, Message};

#[derive(Debug, Default)]
pub struct Transaction {
    pub items: Vec<TransactionItem>,
}

#[derive(Debug, PartialEq)]
pub struct TransactionItem {
    pub item: Item,
    pub quantity: u8,
}

impl TransactionItem {
    fn new(item: &Item) -> Self {
        Self {
            item: item.clone(),
            quantity: 1,
        }
    }

    fn render(&self) -> Element<Message, Renderer<MyTheme>> {
        // |---------------------------------|
        // | | IMG | ITEM_NAME |-----------| |
        // | | IMG |   PRICE   | + | 0 | - | |
        // | | IMG |           |-----------| |
        // |---------------------------------|

        const ITEM_HEIGHT: u16 = 80;
        const NO_IMAGE_PATH: &str = "_none.jpg";
        container(row(vec![
            // IMG
            image(Handle::from_path(format!(
                "./images/{}",
                self.item
                    .image_path
                    .clone()
                    .unwrap_or(NO_IMAGE_PATH.to_string())
            )))
            .height(Length::Fill)
            .into(),
            // ITEM_NAME
            text(format!("{} | £{}", &self.item.name, &self.item.price))
                .size(30)
                .width(Length::Fill)
                .height(Length::Fill)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center)
                .into(),
            // BUTTONS
            row(vec![
                // -
                button(
                    text("-")
                        .height(Length::Fill)
                        .vertical_alignment(Vertical::Center),
                )
                .on_press(Message::DecrementCount(self.item.clone()))
                .height(Length::Fill)
                .padding(20)
                .into(),
                // count
                text(&self.quantity.to_string())
                    .height(Length::Fill)
                    .vertical_alignment(Vertical::Center)
                    .into(),
                // +
                button(
                    text("+")
                        .height(Length::Fill)
                        .vertical_alignment(Vertical::Center),
                )
                .on_press(Message::IncrementCount(self.item.clone()))
                .height(Length::Fill)
                .padding(20)
                .into(),
            ])
            .width(Length::Shrink)
            .align_items(Alignment::Center)
            .height(Length::Fill)
            .spacing(20)
            .into(),
        ]))
        .height(Length::Units(ITEM_HEIGHT))
        .center_y()
        .into()
    }
}

impl Transaction {
    pub fn total_price(&self) -> u32 {
        self.items.iter().fold(0, |sum, item| {
            sum + (item.item.price * item.quantity as u32)
        })
    }

    pub fn render(&self) -> Element<Message, Renderer<MyTheme>> {
        scrollable(
            column(self.items.iter().map(|item| item.render()).collect())
                .width(Length::Fill)
                .align_items(Alignment::Fill)
                .spacing(20),
        )
        .height(Length::Fill)
        .into()
    }

    pub fn add_item(&mut self, item: &Item) {
        let found = self.items.iter_mut().find(|t_item| &t_item.item == item);

        match found {
            Some(found) => found.quantity += 1,
            None => self.items.push(TransactionItem::new(item)),
        }
    }

    pub fn modify_quantity(&mut self, item: &Item, quantity: i8) {
        let (index, found) = self
            .items
            .iter_mut()
            .enumerate()
            .find(|(_index, t_item)| &t_item.item == item)
            .expect("Couldn't find item");

        let (new_qty, overflow) = found.quantity.overflowing_add_signed(quantity);

        if overflow || new_qty == 0 {
            // remove item
            self.items.remove(index);
        } else {
            // modify quantity
            found.quantity = new_qty;
        }
    }

    pub fn generate_receipt(&self) -> String {
        if self.items.len() == 0 {
            "".to_string();
        }
        let mut string = String::new();

        //  3     30      5
        // qty | name | total

        for item in &self.items {
            string.push_str(&format!(
                "{0: <3}{1: <30}{2: <5}\n",
                &item.quantity,
                &item.item.name,
                &item.item.price * item.quantity as u32,
            ));
        }

        string.push_str("======================================\n");

        string.push_str(&format!(
            "{0: <33}{1: <5}",
            "TOTAL PRICE PAID:",
            self.total_price()
        ));

        string
    }
}
