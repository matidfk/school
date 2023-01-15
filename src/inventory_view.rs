use iced::{
    widget::{button, column, container, image, row, scrollable, text, text_input, Column, Space},
    Length,
};

use crate::{item::Item, item_db::ItemDB, theme::ButtonStyle, utils::get_handle, ViewIndex};
use crate::{Element, Message};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct InventoryView {
    pub input_search: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InventoryMessage {
    SearchChanged(String),
    DeleteItem(Item),
    PrintItemsLowInStock,
}

impl InventoryView {
    pub fn view(&self, item_db: &ItemDB) -> Element {
        const COL_COUNT: usize = 4;
        const COL_HEIGHT: u16 = 140;

        fn get_grid_item<'a>(item: &Item) -> Element<'a> {
            button(
                row![
                    image(get_handle(&item.image_path))
                        .width(Length::Units(COL_HEIGHT))
                        .height(Length::Units(COL_HEIGHT)),
                    column![
                        text(&item.name),
                        button("Delete").on_press(Message::Inventory(
                            InventoryMessage::DeleteItem(item.clone())
                        ))
                    ]
                ]
                .width(Length::Fill),
            )
            .style(ButtonStyle::Item)
            .on_press(Message::EditItem(item.clone()))
            .width(Length::Fill)
            .into()
        }

        fn get_grid_row<'a>(chunk: &[Item]) -> Element<'a> {
            row(chunk.iter().map(|item| get_grid_item(item)).collect())
                .width(Length::Fill)
                .spacing(10)
                .into()
        }

        fn get_remainder_row<'a>(rem: &[Item]) -> Element<'a> {
            row(rem
                .iter()
                .map(|item| Some(item))
                // pad with None
                .chain((0..(COL_COUNT - rem.len())).map(|_| None))
                .map(|item| {
                    if let Some(item) = item {
                        get_grid_item(item)
                    } else {
                        Space::new(Length::Fill, Length::Shrink).into()
                    }
                })
                .collect())
            .spacing(10)
            .into()
        }

        // setup grid

        let i = item_db
            .items
            .iter()
            .filter(|item| {
                item.name
                    .to_lowercase()
                    .contains(&self.input_search.to_lowercase())
            })
            .cloned()
            .collect::<Vec<_>>();

        let chunks = i.chunks_exact(COL_COUNT);

        let rem = chunks.remainder();

        let items: Element = column(chunks.map(|chunk| get_grid_row(chunk)).collect::<Vec<_>>())
            .push(get_remainder_row(rem))
            .spacing(10)
            .width(Length::Fill)
            // .padding(20)
            .into();

        Column::new()
            .push(
                row![
                    text_input("Search...", &self.input_search, |v| {
                        Message::Inventory(InventoryMessage::SearchChanged(v))
                    }),
                    button(text("Add New Item"))
                        .style(ButtonStyle::Important)
                        .on_press(Message::SetActiveView(ViewIndex::ItemCreation)),
                    button(text("Print low in stock list"))
                        .style(ButtonStyle::Important)
                        .on_press(Message::Inventory(InventoryMessage::PrintItemsLowInStock))
                ]
                .spacing(20),
            )
            .push(
                scrollable(
                    row![items, Space::new(Length::Units(15), Length::Units(1))]
                        .width(Length::Fill),
                )
                .scrollbar_width(5),
            )
            .spacing(20)
            .padding(20)
            .into()
    }
    pub fn update(&mut self, message: InventoryMessage, item_db: &mut ItemDB) {
        match message {
            InventoryMessage::SearchChanged(value) => self.input_search = value,
            InventoryMessage::DeleteItem(item) => item_db.delete_item(item),
            InventoryMessage::PrintItemsLowInStock => {
                println!("Items low in stock:");
                for item in item_db
                    .items
                    .iter()
                    .filter(|item| item.amount_in_stock < 20)
                {
                    println!("Item: {}, in stock: {}", &item.name, item.amount_in_stock);
                }
            }
        }
    }
}
