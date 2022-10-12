use item::Item;
use theme::MyTheme;

mod item;
mod item_db;
mod theme;
mod transaction;

use iced::{
    executor,
    keyboard::KeyCode,
    subscription::events,
    widget::{column, text, text_input, Column},
    Alignment, Application, Command, Element, Event, Length, Renderer, Settings, Subscription,
};
use transaction::Transaction;

use crate::item_db::ItemDB;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}
// pub fn main() {

//     let mut item_db = ItemDB::load("./item_db.json");

//     item_db.items.push(Item {
//         code: 69,
//         name: "hahahaha".to_owned(),
//         price: 20,
//         image_path: Some("apple.png".to_owned()),
//     });

//     item_db.save("./item_db.json");
// }

// setup model
struct App {
    item_db: ItemDB,
    current_transaction: Transaction,
    input_value: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            item_db: ItemDB::load("./item_db.json"),
            input_value: Default::default(),
            current_transaction: Default::default(),
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
    type Theme = MyTheme;
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
                        // add only if number
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
                                let code = self.input_value.parse().expect("Couldn't parse number");
                                // get corresponding item
                                let item = self.item_db.get_item(code);

                                // if item is found
                                if let Some(item) = item {
                                    // add to transaction
                                    self.current_transaction.add_item(item)
                                } else {
                                    // print error message
                                    println!("invalid item {}", self.input_value)
                                }

                                // clear input
                                self.input_value.clear();
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            Message::IncrementCount(item) => {
                self.current_transaction.modify_quantity(&item, 1);
            }
            Message::DecrementCount(item) => {
                self.current_transaction.modify_quantity(&item, -1);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message, Renderer<Self::Theme>> {
        let col: Column<Self::Message, Renderer<Self::Theme>> = Column::new()
            .push(
                text(format!(
                    "Total Price: £{}",
                    self.current_transaction.total_price()
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

        column![col, self.current_transaction.render()]
            .padding(20)
            .width(Length::Fill)
            .align_items(Alignment::Fill)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Self::Theme::default()
    }

    fn subscription(&self) -> Subscription<Message> {
        events().map(Message::EventOccured)
    }
}
