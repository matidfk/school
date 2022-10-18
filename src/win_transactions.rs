use iced::{
    alignment::Horizontal,
    widget::{button, row, text, Column},
    Alignment, Element, Length, Renderer,
};

use crate::{theme::MyTheme, App, Message};

pub fn render(app: &App) -> Element<Message, Renderer<MyTheme>> {
    let left_half: Column<Message, Renderer<MyTheme>> = Column::new()
        .push(text("poopie"))
        // add fruit shit here
        .width(Length::Fill);

    let right_half: Column<Message, Renderer<MyTheme>> = Column::new()
        // TOTAL PRICE LABEL
        .push(
            text(format!(
                "Total Price: £{}",
                app.current_transaction.total_price()
            ))
            .size(50),
        )
        // BARCODE INPUT
        .push(text(&app.input_value))
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

    row![left_half, right_half].into()
}
