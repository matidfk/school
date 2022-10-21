use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, image, row, scrollable, text},
    Alignment, Element, Length, Renderer,
};

use crate::{
    item::Item,
    theme::MyTheme,
    utils::{format_price, get_handle},
    Message, ITEM_HEIGHT,
};

/// Stores the state of a transaction
#[derive(Debug, Default)]
pub struct Transaction {
    pub items: Vec<TransactionItem>,
}

/// An item in a transaction
#[derive(Debug, PartialEq)]
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

    /// Render an element for a `TransactionItem`
    fn render(&self) -> Element<Message, Renderer<MyTheme>> {
        // |---------------------------------|
        // | | IMG | ITEM_NAME |-----------| |
        // | | IMG |   PRICE   | + | 0 | - | |
        // | | IMG |           |-----------| |
        // |---------------------------------|

        container(row(vec![
            // IMG
            image(get_handle(&self.item.image_path))
                .height(Length::Fill)
                .into(),
            // ITEM_NAME
            text(format!(
                "{} | {}",
                &self.item.name,
                format_price(self.item.price)
            ))
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
                .on_press(Message::ModifyCountInTransaction(self.item.clone(), -1))
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
                .on_press(Message::ModifyCountInTransaction(self.item.clone(), 1))
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
    /// Calculates total price of all items
    pub fn total_price(&self) -> u32 {
        self.items.iter().fold(0, |sum, item| {
            sum + (item.item.price * item.quantity as u32)
        })
    }

    /// Renders the element for the `Transaction`
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

    /// Adds an item to the transaction
    pub fn add_item(&mut self, item: &Item) {
        let found = self.items.iter_mut().find(|t_item| &t_item.item == item);

        match found {
            // If item exists already, add 1 to the quantity
            Some(found) => found.quantity += 1,
            // If not, create it with quantity 1
            None => self.items.push(TransactionItem::new(item)),
        }
    }

    /// Modifies the quantity of an `Item` in the transaction
    pub fn modify_quantity(&mut self, item: &Item, quantity: i32) {
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

    /// Create a string with the receipt in plain text
    pub fn generate_receipt(&self) -> String {
        if self.items.len() == 0 {
            "".to_string();
        }
        let mut string = String::from("\n\n\n\n");

        //  3     30      7
        // qty | name | total

        for item in &self.items {
            string.push_str(&format!(
                "{0: <3}{1: <30}{2: <7}\n",
                &item.quantity,
                &item.item.name,
                format_price(&item.item.price * item.quantity),
            ));
        }

        string.push_str("========================================\n");

        string.push_str(&format!(
            "{0: <33}{1: <7}\n",
            "TOTAL PRICE PAID:",
            format_price(self.total_price())
        ));

        string
    }
}
