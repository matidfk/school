use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, image, image::Handle, row, text, text_input},
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
        // TODO: add price
        //
        // |---------------------------------|
        // | | IMG |           |-----------| |
        // | | IMG | ITEM_NAME | + | 0 | - | |
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
            text(&self.item.name)
                .width(Length::Fill)
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
        column(self.items.iter().map(|item| item.render()).collect())
            .width(Length::Fill)
            .align_items(Alignment::Fill)
            .spacing(20)
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
}
