use add_item_view::AddItemMessage;
use inventory_view::{InventoryMessage, InventoryView};
use theme::MyTheme;

mod add_item_view;
mod inventory_view;
mod item;
mod item_db;
mod theme;
mod transaction;
mod transactions_view;
mod utils;

use iced::{
    executor, subscription::events, Application, Command, Element, Event, Renderer, Settings,
    Subscription, window,
};
use iced_aw::{TabLabel, Tabs};

use transactions_view::{TransactionsMessage, TransactionsView};

use crate::item_db::ItemDB;

pub fn main() -> iced::Result {
    App::run(Settings {
        window: iced::window::Settings {
            size: (1280, 720),
            position: iced::window::Position::Centered,
            // icon: Some(Icon::from_file_data(include_bytes!("../icon.png"), None).unwrap()),
            ..Default::default()
        },
        antialiasing: true,
        exit_on_close_request: false,
        default_font: Some(include_bytes!("../fonts/FiraSans-Medium.ttf")),
        ..Default::default()
    })
}

/// The state model of the application
pub struct App {
    item_db: ItemDB,
    transactions_view: TransactionsView,
    inventory_view: InventoryView,
    active_view: usize,
    should_exit: bool,
}

pub trait View {
    type Message;

    fn title(&self) -> String;

    fn tab_label(&self) -> TabLabel;

    fn view(&self) -> Element<'_, Message, Renderer<MyTheme>>;

    fn update(&mut self, message: Self::Message);
}

impl Default for App {
    fn default() -> Self {
        Self {
            item_db: ItemDB::load_yaml("./item_db.yaml"),
            should_exit: false,
            transactions_view: TransactionsView::default(),
            inventory_view: InventoryView::default(),
            active_view: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    EventOccured(Event),

    Transactions(TransactionsMessage),
    Inventory(InventoryMessage),
    AddItem(AddItemMessage),

    SetActiveView(usize),

    Close,
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
            Message::EventOccured(event) => {
                // if let View::Transactions(v) = &mut self.current_view {
                // v.update(TransactionsMessage::EventOccured(event), &mut self.item_db)
                // }
                if let Event::Window(window::Event::CloseRequested) = event {
                    self.should_exit = true;
                    println!("Closing");
                }
            }

            // Close window and save database
            Message::Close => {
                self.item_db.save_yaml("./item_db.yaml");
                self.should_exit = true;
                println!("Closing gracefully, Saving Database.");
            }
            // Map to TransactionsMessage
            Message::Transactions(message) => self.transactions_view.update(message),
            // Map to InventoryMessage
            Message::Inventory(message) => {
                // if let View::Inventory(v) = &mut self.current_view {
                // v.update(message, &mut self.item_db);
                // }
            }
            Message::AddItem(message) => {
                // if let View::AddItem(v) = &mut self.current_view {
                // v.update(message);
                // }
            }
            Message::SetActiveView(i) => self.active_view = i,
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message, Renderer<Self::Theme>> {
        Tabs::new(self.active_view, Message::SetActiveView)
            .push(
                self.transactions_view.tab_label(),
                self.transactions_view.view(),
            )
            .push(self.inventory_view.tab_label(), self.inventory_view.view())
            .into()
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }
}
