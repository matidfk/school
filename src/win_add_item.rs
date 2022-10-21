use iced::{widget::text_input, Element, Renderer};

use crate::{theme::MyTheme, App, Message};

pub fn render(app: &App) -> Element<Message, Renderer<MyTheme>> {
    text_input("Item name", &app.add_item_name, Message::AddItemNameChanged).into()
}
