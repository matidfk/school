use iced::{
    alignment::{Horizontal, Vertical},
    keyboard::KeyCode,
    widget::{button, column, image, row, scrollable, text, Column, Row},
    Alignment, Event, Length, Renderer,
};

use crate::{
    item::Item,
    item_db::ItemDB,
    theme::ButtonStyle,
    transaction::{Transaction, TransactionItem},
    utils::{format_price, get_handle},
    Message,
};

use crate::theme::MyTheme;
use crate::Element;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TransactionsView {
    pub current_transaction: Transaction,
    pub selected_index: usize,
    pub input_code: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionsMessage {
    EventOccured(Event),
    AddItem(Item),
    FinishTransaction,
    ModifySelectedItemQuantity(i32),
    SelectItem(Item),
}

fn map(message: TransactionsMessage) -> Message {
    Message::Transactions(message)
}

impl TransactionsView {
    pub fn view(&self, item_db: &ItemDB) -> Element {
        // ====================================== LEFT HALF =============================================

        let left_half = column![item_db.items.iter().fold(Row::new(), |row, item| row
            .push(render_quick_item_button(item.clone())))]
        .padding(20)
        .spacing(10);

        // ====================================== RIGHT HALF =============================================

        let quantity_bar: Element =
            // if there is an item selected
            if self.current_transaction.items.len() > 0 {
                row![
                    button(
                        text("-")
                            .size(50)
                            .width(Length::Fill)
                            .horizontal_alignment(Horizontal::Center)
                    )
                    .on_press(map(TransactionsMessage::ModifySelectedItemQuantity(-1)))
                    .width(Length::Fill),
                    // quantity text
                    text(self.current_transaction.items[self.selected_index].quantity)
                        .size(50)
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Center),
                    button(
                        text("+")
                            .size(50)
                            .width(Length::Fill)
                            .horizontal_alignment(Horizontal::Center)
                    )
                    .on_press(map(TransactionsMessage::ModifySelectedItemQuantity(1)))
                    .width(Length::Fill)
                ]
                .width(Length::Fill)
                .align_items(Alignment::Fill)
                .into()
            } else {
                // if no item is selected
                text("No items")
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center)
                    .into()
            };

        let finish_transaction_button = button(row![
            // Text
            text("FINISH TRANSACTION")
                .size(40)
                .horizontal_alignment(Horizontal::Center)
                .width(Length::Fill),
            // Price
            text(format_price(self.current_transaction.total_price()))
                .size(40)
                .horizontal_alignment(Horizontal::Center),
        ])
        .padding(20)
        .style(ButtonStyle::Important)
        .on_press(map(TransactionsMessage::FinishTransaction))
        .height(Length::Shrink);

        let right_half: Column<Message, Renderer<MyTheme>> = column![
            text(&self.input_code),
            render_transaction(&self.current_transaction, self.selected_index),
            quantity_bar,
            finish_transaction_button,
        ]
        .padding(20)
        .spacing(10)
        .align_items(Alignment::Fill);

        row![
            left_half.width(Length::Fill),
            right_half.width(Length::Fill)
        ]
        .into()
    }

    pub fn update(&mut self, message: TransactionsMessage, item_db: &mut ItemDB) {
        match message {
            // ========================== KEY PRESSED ===========================
            TransactionsMessage::EventOccured(event) => {
                if let Event::Keyboard(event) = event {
                    match event {
                        // if number pressed, append to input
                        iced::keyboard::Event::CharacterReceived(char) if char.is_numeric() => {
                            self.input_code.push(char)
                        }
                        // if enter pressed, add item
                        iced::keyboard::Event::KeyPressed {
                            key_code,
                            modifiers: _,
                        } if key_code == KeyCode::Enter && !self.input_code.is_empty() => {
                            // get number in input
                            let code = self.input_code.parse().expect("Couldn't parse number");
                            // get corresponding item
                            let item = item_db.get_item(code);

                            // if item is found
                            if let Some(item) = item {
                                // add to transaction
                                self.current_transaction.add_item(item);
                                // set as selected item
                                self.selected_index = self.current_transaction.items.len() - 1;
                            } else {
                                // print error message
                                println!("invalid item {}", self.input_code)
                            }

                            // clear input
                            self.input_code.clear();
                        }
                        _ => {}
                    }
                }
            }

            // ====================== FINISH TRANSACTION ========================
            TransactionsMessage::FinishTransaction => {
                println!("{}", self.current_transaction.generate_receipt());
                item_db.update_quantities_from_transaction(&self.current_transaction);
                self.current_transaction = Transaction::default();
            }
            // modify amount
            TransactionsMessage::ModifySelectedItemQuantity(amount) => {
                let item = &mut self.current_transaction.items[self.selected_index];
                let (new_qty, overflow) = item.quantity.overflowing_add_signed(amount);
                if overflow || new_qty == 0 {
                    // remove item
                    self.current_transaction.items.remove(self.selected_index);
                } else {
                    // modify quantity
                    item.quantity = new_qty;
                }
            }

            // select item
            TransactionsMessage::SelectItem(item) => {
                self.selected_index = self
                    .current_transaction
                    .items
                    .iter()
                    .position(|i| i.item == item)
                    .unwrap();
            }
            TransactionsMessage::AddItem(item) => {
                // add item to transaction
                if !self.current_transaction.add_item(&item) {
                    // if the item was not in it already, select the new item
                    self.selected_index = self.current_transaction.items.len() - 1;
                }
            }
        }
    }
}

fn render_quick_item_button<'a>(item: Item) -> Element<'a> {
    button(image(get_handle(&item.image_path)))
        .on_press(map(TransactionsMessage::AddItem(item.clone())))
        .height(Length::Units(80))
        .width(Length::Units(80))
        .into()
}
/// Renders the element for the `Transaction`
fn render_transaction(transaction: &Transaction, selected_index: usize) -> Element {
    scrollable(
        column(
            transaction
                .items
                .iter()
                .enumerate()
                .map(|(index, item)| render_item(item, index == selected_index))
                .collect(),
        )
        .width(Length::Fill)
        .align_items(Alignment::Fill)
        .spacing(20),
    )
    .height(Length::Fill)
    .into()
}
/// Render an element for a `TransactionItem`
fn render_item(item: &TransactionItem, selected: bool) -> Element {
    let image = image(get_handle(&item.item.image_path)).height(Length::Fill);

    let name = text(&item.item.name)
        .size(30)
        .width(Length::Fill)
        .height(Length::Fill)
        .horizontal_alignment(Horizontal::Center)
        .vertical_alignment(Vertical::Center);

    let price = text(format!(
        "{} ({})",
        format_price(item.item.price * item.quantity),
        item.quantity,
    ))
    .size(30)
    .height(Length::Fill)
    .width(Length::Fill)
    .horizontal_alignment(Horizontal::Center)
    .vertical_alignment(Vertical::Center);

    button(row![image, name, price,])
        .style(if selected {
            ButtonStyle::ItemSelected
        } else {
            ButtonStyle::Item
        })
        .on_press(map(TransactionsMessage::SelectItem(item.item.clone())))
        .height(Length::Units(80))
        .into()
}
