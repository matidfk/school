use item::Item;
use theme::{ButtonStyle, MyTheme};

mod item;
mod item_db;
mod theme;
mod transaction;
mod win_inventorymanagement;
mod win_transactions;

use iced::{
    executor,
    keyboard::KeyCode,
    subscription::events,
    widget::{button, row, text, Column},
    window::Icon,
    Application, Command, Element, Event, Length, Renderer, Settings, Subscription,
};
use transaction::Transaction;

use crate::item_db::ItemDB;

pub fn main() -> iced::Result {
    App::run(Settings {
        window: iced::window::Settings {
            size: (900, 500),
            position: iced::window::Position::Centered,
            // min_size: (),
            // max_size: (),
            // visible: (),
            // resizable: (),
            // decorations: (),
            // transparent: (),
            // always_on_top: (),
            icon: Some(Icon::from_file_data(include_bytes!("../icon.png"), None).unwrap()),
            ..Default::default()
        },
        // default_font: todo!(),
        // default_text_size: todo!(),
        // text_multithreading: todo!(),
        // antialiasing: todo!(),
        ..Default::default()
    })
}
// pub fn main() {
//     let mut item_db = ItemDB::load("./item_db.json");
//     item_db.items.push(Item {
//         code: 6,
//         name: "pica2".to_owned(),
//         price: 20,
//         image_path: Some("apple.png".to_owned()),
//     });
//     item_db.items.push(Item {
//         code: 7,
//         name: "apel ifone2".to_owned(),
//         price: 90,
//         image_path: Some("apple.png".to_owned()),
//     });
//     item_db.save("./item_db.json");
// }

// setup model
pub struct App {
    item_db: ItemDB,
    current_view: View,
    should_exit: bool,

    // Transactions
    current_transaction: Transaction,
    input_value: String,
    // Inventory Management
}

impl Default for App {
    fn default() -> Self {
        Self {
            item_db: ItemDB::load("./item_db.json"),
            should_exit: false,
            current_view: Default::default(),
            current_transaction: Default::default(),
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
    FinishTransaction,
    SwitchView(View),
    Close,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum View {
    #[default]
    Transactions,
    InventoryManagement,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = MyTheme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn subscription(&self) -> Subscription<Message> {
        events().map(Message::EventOccured)
    }

    fn theme(&self) -> Self::Theme {
        Self::Theme::default()
    }

    fn title(&self) -> String {
        String::from("SchoolApp")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            // update input value
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
            Message::FinishTransaction => {
                println!("{}", &self.current_transaction.generate_receipt());
                self.current_transaction = Default::default();
            }
            Message::SwitchView(view) => {
                println!("open window {:?}", view);
                self.current_view = view;
            }
            Message::Close => self.should_exit = true,
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message, Renderer<Self::Theme>> {
        match self.current_view {
            View::Transactions => Column::new()
                .push(tab_buttons(&self))
                .push(win_transactions::render(&self))
                .into(),
            View::InventoryManagement => Column::new()
                .push(tab_buttons(&self))
                .push(win_inventorymanagement::render(&self))
                .into(),
        }
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }
}

fn tab_buttons(app: &App) -> Element<Message, Renderer<MyTheme>> {
    fn get_style(active: bool) -> ButtonStyle {
        if active == true {
            ButtonStyle::TabActive
        } else {
            ButtonStyle::TabInactive
        }
    }
    row(vec![
        button(text("Transactions"))
            .style(get_style(app.current_view == View::Transactions))
            .on_press(Message::SwitchView(View::Transactions))
            .into(),
        button(text("Inventory Management"))
            .style(get_style(app.current_view == View::InventoryManagement))
            .on_press(Message::SwitchView(View::InventoryManagement))
            .into(),
        // spacer
        button(text(" ")).width(Length::Fill).into(),
        // close button
        button(text("  X  "))
            .style(ButtonStyle::TabInactive)
            .on_press(Message::Close)
            .into(),
    ])
    .width(Length::Fill)
    .into()
}
