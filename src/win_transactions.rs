use iced::{
    alignment::Horizontal,
    widget::{button, column, row, text, Column},
    Alignment, Element, Length, Renderer,
};

use crate::{transaction::Transaction, utils::format_price};

use crate::{theme::MyTheme, App, Message};

pub struct WinTransactions {
    current_transaction: Transaction,
    input_code: String,
}

pub fn render(app: &App) -> Element<Message, Renderer<MyTheme>> {
    let left_half: Column<Message, Renderer<MyTheme>> = Column::new()
        .push(text("poopie"))
        // add fruit shit here
        .width(Length::Fill);

    let right_half: Column<Message, Renderer<MyTheme>> = column![
        // TOTAL PRICE LABEL
        text(format!(
            "Total Price: {}",
            format_price(app.current_transaction.total_price())
        ))
        .size(50),
        // BARCODE INPUT
        text(&app.transactions_input_code),
        // TRANSACTION ITEMS LIST
        app.current_transaction.render(),
        // FINISH TRANSACTION BUTTON
        button(
            text("FINISH TRANSACTION")
                .size(20)
                .horizontal_alignment(Horizontal::Center),
        )
        .on_press(Message::FinishTransaction)
        .height(Length::Shrink),
    ]
    .padding(20)
    .spacing(10)
    .width(Length::Fill)
    .align_items(Alignment::Fill);

    row![left_half, right_half].into()
}
