use iced::{
    widget::{button, column, container, image, row, scrollable, text, text_input, Column},
    Element, Length, Renderer,
};

use crate::{item::Item, item_db::ItemDB, theme::MyTheme, utils::get_handle};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct InventoryView {
    pub input_search: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InventoryMessage {
    SearchChanged(String),
    ModifyAmountInStock(Item, i32),
}

impl InventoryView {
    pub fn view<'a>(&'a self, item_db: &'a ItemDB) -> Element<InventoryMessage, Renderer<MyTheme>> {
        let items = column(
            item_db
                .items
                .iter()
                .filter(|item| item.name.contains(&self.input_search))
                .map(|item| render_item(item))
                .collect(),
        );

        Column::new()
            .push(text_input(
                "Search...",
                &self.input_search,
                InventoryMessage::SearchChanged,
            ))
            .push(scrollable(items))
            .into()
    }

    pub fn update(&mut self, message: InventoryMessage, item_db: &mut ItemDB) {
        match message {
            InventoryMessage::SearchChanged(value) => self.input_search = value,
            InventoryMessage::ModifyAmountInStock(item, amount) => {
                item_db.modify_quantity(&item, amount)
            }
        }
    }
}

fn render_item(item: &Item) -> Element<InventoryMessage, Renderer<MyTheme>> {
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
            .on_press(InventoryMessage::ModifyAmountInStock(item.clone(), -1))
            .into(),
        button(text("+"))
            .on_press(InventoryMessage::ModifyAmountInStock(item.clone(), 1))
            .into(),
    ]))
    .height(Length::Units(160))
    .width(Length::Fill)
    .padding(20)
    .into()
}
