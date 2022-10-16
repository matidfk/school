use iced::{
    alignment::{self, Horizontal},
    widget::{button, column, row, text, text_input, Column},
    Alignment, Element, Length, Renderer,
};

use crate::{theme::MyTheme, App, Message};

type Theme = MyTheme;

pub fn render_main_window(app: &App) -> Element<Message, Renderer<Theme>> {
    let right_half: Column<Message, Renderer<Theme>> = Column::new()
        // TOTAL PRICE LABEL
        .push(
            text(format!(
                "Total Price: £{}",
                app.current_transaction.total_price()
            ))
            .size(50),
        )
        // BARCODE INPUT
        .push(text_input(
            "barcode number go here",
            &app.input_value,
            Message::InputChanged,
        ))
        // TRANSACTION ITEMS LIST
        .push(app.current_transaction.render())
        // FINISH TRANSACTION BUTTON
        .push(
            button(
                text("FINISH TRANSACTION")
                    .size(20)
                    .horizontal_alignment(Horizontal::Center),
            )
            .on_press(Message::FinishTransaction)
            .height(Length::Shrink),
        )
        .padding(20)
        .spacing(10)
        .width(Length::Fill)
        .align_items(Alignment::Fill);

    row![text("poopie").width(Length::Fill), right_half,].into()
}
