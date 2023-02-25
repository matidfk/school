use iced::{
    widget::{button, column, image, row, text, text_input},
    Length,
};

use crate::{
    item::Item,
    item_db::ItemDB,
    utils::{get_handle, notify, parse_price},
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
    BrowseImagePath,
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
            self.input_price = (item.price as f32 / 100.0).to_string();
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
                row![
                    text_input("Item Image Path", &self.input_image_path, |input| {
                        Message::ItemCreation(ItemCreationMessage::ImagePathChanged(input))
                    }),
                    button("Browse")
                        .on_press(Message::ItemCreation(ItemCreationMessage::BrowseImagePath))
                ]
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

    pub fn update(
        &mut self,
        message: ItemCreationMessage,
        item_db: &mut ItemDB,
    ) -> Option<Message> {
        match message {
            ItemCreationMessage::ImagePathChanged(value) => self.input_image_path = value,
            ItemCreationMessage::NameChanged(value) => self.input_name = value,
            ItemCreationMessage::PriceChanged(value) => self.input_price = value,
            ItemCreationMessage::BarcodeChanged(value) => self.input_barcode = value,

            ItemCreationMessage::SaveItem => {
                // try parse the item data
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
                        return Some(Message::SetActiveView(crate::ViewIndex::Inventory));
                    }
                    Err(_) => notify("Failed saving item", ""),
                };
            }

            ItemCreationMessage::BrowseImagePath => {
                // open a filepicker
                let file = rfd::FileDialog::new().set_directory("images").pick_file();

                if let Some(file) = file {
                    self.input_image_path = file
                        .to_string_lossy()
                        .to_string()
                        .split_once("images")
                        .unwrap()
                        .1
                        .to_owned();
                }
            }
        }
        None
    }
}

/// Try parse an item from strings
fn parse_item(
    name: String,
    barcode: String,
    price: String,
    image_path: String,
) -> Result<Item, ()> {
    let barcode = barcode.parse().map_err(|_| ())?;
    let price = parse_price(&price)?;

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
