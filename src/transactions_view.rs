use iced::{
    alignment::{Horizontal, Vertical},
    keyboard::KeyCode,
    widget::{button, column, container, image, row, scrollable, text, Column},
    Alignment, Element, Event, Length, Renderer,
};

use crate::{
    item::Item,
    item_db::ItemDB,
    transaction::{Transaction, TransactionItem},
    utils::{format_price, get_handle},
};

use crate::theme::MyTheme;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TransactionsView {
    pub current_transaction: Transaction,
    pub input_code: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionsMessage {
    EventOccured(Event),
    FinishTransaction,
    ModifyCountInTransaction(Item, i32),
}

impl TransactionsView {
    pub fn view(&self) -> Element<TransactionsMessage, Renderer<MyTheme>> {
        let left_half: Column<TransactionsMessage, Renderer<MyTheme>> = Column::new()
            .push(text("poopie"))
            // add fruit shit here
            .width(Length::Fill);

        let right_half: Column<TransactionsMessage, Renderer<MyTheme>> = column![
            // TOTAL PRICE LABEL
            text(format!(
                "Total Price: {}",
                format_price(self.current_transaction.total_price())
            ))
            .size(50),
            // BARCODE INPUT
            text(&self.input_code),
            // TRANSACTION ITEMS LIST
            render_transaction(&self.current_transaction),
            // FINISH TRANSACTION BUTTON
            button(
                text("FINISH TRANSACTION")
                    .size(20)
                    .horizontal_alignment(Horizontal::Center),
            )
            .on_press(TransactionsMessage::FinishTransaction)
            .height(Length::Shrink),
        ]
        .padding(20)
        .spacing(10)
        .width(Length::Fill)
        .align_items(Alignment::Fill);

        row![left_half, right_half].into()
    }
    pub fn update(&mut self, message: TransactionsMessage, item_db: &mut ItemDB) {
        match message {
            // finish transaction
            TransactionsMessage::FinishTransaction => {
                println!("{}", self.current_transaction.generate_receipt());
                item_db.update_quantities_from_transaction(&self.current_transaction);
                self.current_transaction = Transaction::default();
            }
            // modify amount
            TransactionsMessage::ModifyCountInTransaction(item, amount) => {
                self.current_transaction.modify_quantity(&item, amount)
            }
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
                                self.current_transaction.add_item(item)
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
        }
    }
}
/// Renders the element for the `Transaction`
pub fn render_transaction(
    transaction: &Transaction,
) -> Element<TransactionsMessage, Renderer<MyTheme>> {
    scrollable(
        column(
            transaction
                .items
                .iter()
                .map(|item| render_item(item))
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
fn render_item(item: &TransactionItem) -> Element<TransactionsMessage, Renderer<MyTheme>> {
    // |---------------------------------|
    // | | IMG | ITEM_NAME |-----------| |
    // | | IMG |   PRICE   | + | 0 | - | |
    // | | IMG |           |-----------| |
    // |---------------------------------|

    container(row(vec![
        // IMG
        image(get_handle(&item.item.image_path))
            .height(Length::Fill)
            .into(),
        // ITEM_NAME
        text(format!(
            "{} | {}",
            &item.item.name,
            format_price(item.item.price)
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
            .on_press(TransactionsMessage::ModifyCountInTransaction(
                item.item.clone(),
                -1,
            ))
            .height(Length::Fill)
            .padding(20)
            .into(),
            // count
            text(&item.quantity.to_string())
                .height(Length::Fill)
                .vertical_alignment(Vertical::Center)
                .into(),
            // +
            button(
                text("+")
                    .height(Length::Fill)
                    .vertical_alignment(Vertical::Center),
            )
            .on_press(TransactionsMessage::ModifyCountInTransaction(
                item.item.clone(),
                1,
            ))
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
    .height(Length::Units(80))
    .center_y()
    .into()
}
