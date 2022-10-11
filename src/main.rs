use item::Item;
// TODO: use git submodules for iced
use purchased_item::PurchasedItem;

mod item;
mod item_db;
mod purchased_item;

use iced::{
    executor,
    keyboard::KeyCode,
    subscription::events,
    widget::{column, text, text_input, Column},
    Alignment, Application, Command, Element, Event, Length, Settings, Subscription, Theme,
};

use crate::item_db::ItemDB;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

// setup model
struct App {
    item_db: ItemDB,
    purchased_items: Vec<PurchasedItem>,
    input_value: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            item_db: ItemDB::load("./item_db.json"),
            purchased_items: Default::default(),
            input_value: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    EventOccured(Event),
    InputChanged(String),
    // TODO: use index instead i guess
    IncrementCount(Item),
    DecrementCount(Item),
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("App")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(value) => self.input_value = value,
            Message::EventOccured(event) => match event {
                Event::Keyboard(event) => match event {
                    iced::keyboard::Event::CharacterReceived(char) => {
                        // add if number
                        if char.is_numeric() {
                            self.input_value.push(char);
                        }
                    }
                    iced::keyboard::Event::KeyPressed {
                        key_code,
                        modifiers: _,
                    } => {
                        // flush
                        if key_code == KeyCode::Enter {
                            if !self.input_value.is_empty() {
                                // get number in input
                                let barcode =
                                    self.input_value.parse().expect("Couldn't parse number");
                                // get corresponding item
                                let item = self.item_db.get_item(barcode);

                                // add to transaction
                                match item {
                                    Some(item) => {
                                        // if list contains item, add to quantity
                                        if let Some(mut found_item) = self
                                            .purchased_items
                                            .iter_mut()
                                            .find(|i| item == &i.item)
                                        {
                                            found_item.quantity += 1;
                                        // else add as new
                                        } else {
                                            self.purchased_items
                                                .push(PurchasedItem::new(item.clone()));
                                        }
                                    }
                                    None => println!("invalid item {}", self.input_value),
                                }
                                self.input_value.clear();
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            Message::IncrementCount(item) => {
                self.purchased_items
                    .iter_mut()
                    .find(|i| item == i.item)
                    .unwrap()
                    .quantity += 1
            }
            Message::DecrementCount(item) => {
                let item = self
                    .purchased_items
                    .iter_mut()
                    .find(|i| item == i.item)
                    .unwrap();
                item.quantity -= 1;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let col = Column::new()
            .push(
                text(format!(
                    "Total Price: £{}",
                    get_total_price(&self.purchased_items)
                ))
                .size(50),
            )
            .push(text_input(
                "barcode number go here",
                &self.input_value,
                Message::InputChanged,
            ))
            .padding(20)
            .align_items(Alignment::Center);

        let items_list: Element<Message> = column(
            self.purchased_items
                .iter()
                .map(|item| item.render())
                .collect(),
        )
        .width(Length::Fill)
        .align_items(Alignment::Fill)
        .spacing(20)
        .into();

        column![col, items_list]
            .padding(20)
            .width(Length::Units(400))
            .align_items(Alignment::Center)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        events().map(Message::EventOccured)
    }
}

fn get_total_price(items: &Vec<PurchasedItem>) -> u32 {
    items.iter().fold(0, |sum, item| {
        sum + (item.item.price * item.quantity as u32)
    })
}
