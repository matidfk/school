use item::Item;
use theme::{ButtonStyle, MyTheme};

mod item;
mod item_db;
mod theme;
mod transaction;
mod utils;
mod win_add_item;
mod win_inventory;
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
use win_transactions::WinTransactions;

use crate::item_db::ItemDB;

pub fn main() -> iced::Result {
    App::run(Settings {
        window: iced::window::Settings {
            size: (900, 500),
            position: iced::window::Position::Centered,
            icon: Some(Icon::from_file_data(include_bytes!("../icon.png"), None).unwrap()),
            ..Default::default()
        },
        ..Default::default()
    })
}
// pub fn main() {
//     let mut item_db = ItemDB::load_yaml("./item_db.json");
//     // item_db.items.push(Item {
//     //     code: 6,
//     //     name: "pica2".to_owned(),
//     //     price: 20,
//     //     image_path: Some("apple.png".to_owned()),
//     // });
//     // item_db.items.push(Item {
//     //     code: 7,
//     //     name: "apel ifone2".to_owned(),
//     //     price: 90,
//     //     image_path: Some("apple.png".to_owned()),
//     // });
//     item_db.save_yaml("./item_db.yaml");
// }

/// The state model of the application
pub struct App {
    item_db: ItemDB,
    current_view: View,
    should_exit: bool,

    // Transactions
    transactions: WinTransactions,
    current_transaction: Transaction,
    transactions_input_code: String,
    // Inventory Management
    inventory_input_search: String,
    // Add Item
    add_item_name: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            item_db: ItemDB::load_yaml("./item_db.yaml"),
            should_exit: false,
            current_view: Default::default(),
            current_transaction: Default::default(),
            transactions_input_code: Default::default(),
            inventory_input_search: Default::default(),
            add_item_name: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    EventOccured(Event),
    InputChanged(String),
    // TODO: use index instead i guess
    ModifyCountInTransaction(Item, i32),
    FinishTransaction,
    ModifyAmountInStock(Item, i32),
    SwitchView(View),
    Close,
    InventorySearchChanged(String),
    AddItemNameChanged(String),
}

/// Different views (tabs) of the application
#[derive(Debug, Default, Clone, PartialEq)]
pub enum View {
    /// The view for processing transactions
    #[default]
    Transactions,
    /// The view for managing the inventory
    Inventory,
    /// The view for adding new items
    AddItem,
}

/// Height in pixels of an item element in the transaction view
pub const ITEM_HEIGHT: u16 = 80;

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
            Message::InputChanged(value) => self.transactions_input_code = value,

            Message::EventOccured(event) => match event {
                Event::Keyboard(event) if self.current_view == View::Transactions => match event {
                    iced::keyboard::Event::CharacterReceived(char) => {
                        // add only if number
                        if char.is_numeric() {
                            self.transactions_input_code.push(char);
                        }
                    }
                    iced::keyboard::Event::KeyPressed {
                        key_code,
                        modifiers: _,
                    } => {
                        // flush
                        if key_code == KeyCode::Enter {
                            if !self.transactions_input_code.is_empty() {
                                // get number in input
                                let code = self
                                    .transactions_input_code
                                    .parse()
                                    .expect("Couldn't parse number");
                                // get corresponding item
                                let item = self.item_db.get_item(code);

                                // if item is found
                                if let Some(item) = item {
                                    // add to transaction
                                    self.current_transaction.add_item(item)
                                } else {
                                    // print error message
                                    println!("invalid item {}", self.transactions_input_code)
                                }

                                // clear input
                                self.transactions_input_code.clear();
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            Message::ModifyCountInTransaction(item, count) => {
                self.current_transaction.modify_quantity(&item, count);
            }
            Message::FinishTransaction => {
                println!("{}", &self.current_transaction.generate_receipt());
                self.item_db
                    .update_quantities_from_transaction(&self.current_transaction);
                self.current_transaction = Default::default();
            }
            Message::SwitchView(view) => {
                self.current_view = view;
            }
            Message::Close => {
                self.item_db.save_yaml("./item_db.yaml");
                self.should_exit = true
            }
            Message::ModifyAmountInStock(item, count) => {
                self.item_db.modify_quantity(&item, count);
            }
            Message::InventorySearchChanged(value) => self.inventory_input_search = value,
            Message::AddItemNameChanged(value) => self.add_item_name = value,
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message, Renderer<Self::Theme>> {
        match self.current_view {
            View::Transactions => Column::new()
                .push(tab_buttons(&self))
                .push(win_transactions::render(&self))
                .into(),
            View::Inventory => Column::new()
                .push(tab_buttons(&self))
                .push(win_inventory::render(&self))
                .into(),
            View::AddItem => Column::new()
                .push(tab_buttons(&self))
                .push(win_add_item::render(&self))
                .into(),
        }
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }
}

/// Render the tab buttons at the top of the app
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
        button(text("Inventory"))
            .style(get_style(app.current_view == View::Inventory))
            .on_press(Message::SwitchView(View::Inventory))
            .into(),
        button(text("Add Item"))
            .style(get_style(app.current_view == View::AddItem))
            .on_press(Message::SwitchView(View::AddItem))
            .into(),
        // spacer
        button(text(" ")).width(Length::Fill).into(),
        // close button
        button(text("  x  "))
            .style(ButtonStyle::TabInactive)
            .on_press(Message::Close)
            .into(),
    ])
    .width(Length::Fill)
    .into()
}
