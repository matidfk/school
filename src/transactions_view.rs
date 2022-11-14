use iced::{
    alignment::{Horizontal, Vertical},
    keyboard::KeyCode,
    widget::{button, column, image, row, scrollable, text, Column},
    Alignment, Element, Event, Length, Renderer,
};

use crate::{
    item::Item,
    item_db::ItemDB,
    theme::ButtonStyle,
    transaction::{Transaction, TransactionItem},
    utils::{format_price, get_handle}, Message, View, App,
};

use crate::theme::MyTheme;

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



impl View for TransactionsView {
    type Message = TransactionsMessage;

    fn title(&self) -> String {
        "Transactions".to_string()
    }

    fn tab_label(&self) -> iced_aw::TabLabel {
        iced_aw::TabLabel::Text("transactions".to_string())
    }

    fn view(&self) -> Element<'_, Message, Renderer<MyTheme>> {
        // ====================================== LEFT HALF =============================================

        let left_half = column![row![
            // quick access fruit and things
            // render_quick_item_button(&app.item_db.items[0]),
            // render_quick_item_button(&app.item_db.items[1]),
        ]
        .spacing(10)]
        .padding(20)
        .spacing(10);

        // ====================================== RIGHT HALF =============================================

        let quantity_bar: Element<Message, Renderer<MyTheme>> =
            // if there is an item selected
            if self.current_transaction.items.len() > 0 {
                row![
                    button(
                        text("-")
                            .size(50)
                            .width(Length::Fill)
                            .horizontal_alignment(Horizontal::Center)
                    )
                    // .on_press(TransactionsMessage::ModifySelectedItemQuantity(-1))
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
                    // .on_press(TransactionsMessage::ModifySelectedItemQuantity(1))
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
        .on_press(Message::Transactions(TransactionsMessage::FinishTransaction))
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

    fn update(&mut self, message: Self::Message) {
        match message {
            // finish transaction
            TransactionsMessage::FinishTransaction => {
                println!("{}", self.current_transaction.generate_receipt());
                // item_db.update_quantities_from_transaction(&self.current_transaction);
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
            // keys pressed
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
                            // let code = self.input_code.parse().expect("Couldn't parse number");
                            // get corresponding item
                            // let item = item_db.get_item(code);

                            // if item is found
                            // if let Some(item) = item {
                                // add to transaction
                                // self.current_transaction.add_item(item);
                                // set as selected item
                                // self.selected_index = self.current_transaction.items.len() - 1;
                            // } else {
                                // print error message
                                // println!("invalid item {}", self.input_code)
                            // }

                            // clear input
                            self.input_code.clear();
                        }
                        _ => {}
                    }
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
                self.current_transaction.add_item(&item);
                self.selected_index = self.current_transaction.items.len() - 1;
            }
        }
    }
}


fn render_quick_item_button(item: &Item) -> Element<Message, Renderer<MyTheme>> {
    button(image(get_handle(&item.image_path)))
        // .on_press(TransactionsMessage::AddItem(item.clone()))
        .height(Length::Units(80))
        .width(Length::Units(80))
        .into()
}
/// Renders the element for the `Transaction`
fn render_transaction(
    transaction: &Transaction,
    selected_index: usize,
) -> Element<Message, Renderer<MyTheme>> {
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
fn render_item(item: &TransactionItem, selected: bool) -> Element<Message, Renderer<MyTheme>> {
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
        // .on_press(TransactionsMessage::SelectItem(item.item.clone()))
        .height(Length::Units(80))
        .into()
}
