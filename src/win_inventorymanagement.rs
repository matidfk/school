use iced::{
    widget::{column, container, scrollable, text, Column},
    Element, Length, Renderer,
};

use crate::{item::Item, theme::MyTheme, App, Message};

pub fn render(app: &App) -> Element<Message, Renderer<MyTheme>> {
    let items = column(
        app.item_db
            .items
            .iter()
            .map(|item| render_item(item))
            .collect(),
    );

    Column::new()
        .push(text("poopie"))
        .push(scrollable(items))
        .into()
}

fn render_item(item: &Item) -> Element<Message, Renderer<MyTheme>> {
    container(column(vec![
        text(&item.name).into(),
        text(&item.image_path.as_ref().unwrap_or(&"No image".to_string())).into(),
        text(&item.code).into(),
    ]))
    .width(Length::Fill)
    .padding(20)
    .into()
}
