use iced::{
    widget::{button, column, container, image, row, scrollable, text, text_input, Column},
    Element, Length, Renderer,
};

use crate::{item::Item, theme::MyTheme, utils::get_handle, App, Message, ITEM_HEIGHT};

pub fn render(app: &App) -> Element<Message, Renderer<MyTheme>> {
    let items = column(
        app.item_db
            .items
            .iter()
            .filter(|i| i.name.contains(&app.inventory_input_search))
            .map(|item| render_item(item))
            .collect(),
    );

    Column::new()
        .push(text_input(
            "Search...",
            &app.inventory_input_search,
            Message::InventorySearchChanged,
        ))
        .push(scrollable(items))
        .into()
}

fn render_item(item: &Item) -> Element<Message, Renderer<MyTheme>> {
    container(row(vec![
        // item image
        image(get_handle(&item.image_path))
            .height(Length::Fill)
            .into(),
        // item info
        column(vec![
            text(&item.name).into(),
            text(&item.image_path.as_ref().unwrap_or(&"No image".to_string())).into(),
            text(&item.code).into(),
            text(&item.price).into(),
            text(&item.amount_in_stock).into(),
        ])
        .into(),
        // amount in stock buttons
        button(text("-"))
            .on_press(Message::ModifyAmountInStock(item.clone(), -1))
            .into(),
        button(text("+"))
            .on_press(Message::ModifyAmountInStock(item.clone(), 1))
            .into(),
    ]))
    .height(Length::Units(ITEM_HEIGHT * 2))
    .width(Length::Fill)
    .padding(20)
    .into()
}
