use std::num::ParseIntError;

use iced::{
    widget::{button, column, image, row, text, text_input},
    Length,
};

use crate::{
    item::Item,
    item_db::ItemDB,
    utils::{get_handle, notify},
};
use crate::{Element, Message};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ItemCreationView {
    pub editing_item: Option<Item>,
    input_image_path: String,
    input_name: String,
    input_price: String,
    input_barcode: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemCreationMessage {
    ImagePathChanged(String),
    NameChanged(String),
    PriceChanged(String),
    BarcodeChanged(String),
    SaveItem,
}

impl ItemCreationView {
    pub fn set_item(&mut self, item: Option<Item>) {
        self.editing_item = item.clone();
        if let Some(item) = item {
            self.input_barcode = item.barcode.to_string();
            self.input_name = item.name.clone();
            self.input_image_path = item.image_path.unwrap_or("".to_owned());
            self.input_price = item.price.to_string();
        } else {
            self.input_barcode = "".to_owned();
            self.input_name = "".to_owned();
            self.input_image_path = "".to_owned();
            self.input_price = "".to_owned();
        }
    }
    pub fn view(&self) -> Element {
        row![
            // left side
            column![
                //image
                image(get_handle(&Some(self.input_image_path.clone()))),
                // image path
                text_input("Item Image Path", &self.input_image_path, |input| {
                    Message::ItemCreation(ItemCreationMessage::ImagePathChanged(input))
                }),
            ]
            .width(Length::FillPortion(1)),
            // right side
            column![
                text_input("Item Name", &self.input_name, |input| {
                    Message::ItemCreation(ItemCreationMessage::NameChanged(input))
                }),
                text_input("Item Price", &self.input_price, |input| {
                    Message::ItemCreation(ItemCreationMessage::PriceChanged(input))
                }),
                text_input("Item Barcode", &self.input_barcode, |input| {
                    Message::ItemCreation(ItemCreationMessage::BarcodeChanged(input))
                }),
                button(text("Save Item"))
                    .on_press(Message::ItemCreation(ItemCreationMessage::SaveItem)),
            ]
            .width(Length::FillPortion(2))
        ]
        .padding(20)
        .spacing(10)
        .into()
    }
    pub fn update(&mut self, message: ItemCreationMessage, item_db: &mut ItemDB) {
        match message {
            ItemCreationMessage::ImagePathChanged(value) => self.input_image_path = value,
            ItemCreationMessage::NameChanged(value) => self.input_name = value,
            ItemCreationMessage::PriceChanged(value) => self.input_price = value,
            ItemCreationMessage::BarcodeChanged(value) => self.input_barcode = value,
            ItemCreationMessage::SaveItem => {
                match parse_item(
                    self.input_name.clone(),
                    self.input_barcode.clone(),
                    self.input_price.clone(),
                    self.input_image_path.clone(),
                ) {
                    Ok(item) => {
                        if self.editing_item.is_some() {
                            let e = self.editing_item.clone().unwrap();
                            let index = item_db.items.iter().position(|i| &e == i).unwrap();
                            item_db.items[index] = item;
                        } else {
                            item_db.items.push(item);
                        }
                        notify("Saved Item", &self.input_name);
                    }
                    Err(err) => notify("Failed saving item", &err.to_string()),
                };
            }
        }
    }
}
fn parse_item(
    name: String,
    barcode: String,
    price: String,
    image_path: String,
) -> Result<Item, ParseIntError> {
    let barcode = barcode.parse()?;
    let price = price.parse()?;

    let image_path = if image_path.is_empty() {
        None
    } else {
        Some(image_path)
    };

    Ok(Item {
        barcode,
        name,
        price,
        image_path,
        amount_in_stock: 5,
    })
}
