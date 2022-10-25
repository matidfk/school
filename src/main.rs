use add_item_view::{AddItemMessage, AddItemView};
use inventory_view::{InventoryMessage, InventoryView};
use theme::{ButtonStyle, MyTheme};

mod add_item_view;
mod inventory_view;
mod item;
mod item_db;
mod theme;
mod transaction;
mod transactions_view;
mod utils;

use iced::{
    executor,
    subscription::events,
    widget::{button, text, Column, Row},
    window::Icon,
    Application, Command, Element, Event, Length, Renderer, Settings, Subscription,
};

use transactions_view::{TransactionsMessage, TransactionsView};

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

/// The state model of the application
pub struct App {
    item_db: ItemDB,
    current_view: View,
    should_exit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            item_db: ItemDB::load_yaml("./item_db.yaml"),
            should_exit: false,
            current_view: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    EventOccured(Event),

    Transactions(TransactionsMessage),
    Inventory(InventoryMessage),

    SwitchView(View),

    Close,
    AddItem(AddItemMessage),
}

/// Different views (tabs) of the application
#[derive(Debug, Clone, PartialEq)]
pub enum View {
    /// The view for processing transactions
    Transactions(TransactionsView),
    /// The view for managing the inventory
    Inventory(InventoryView),
    /// The view for adding new items
    AddItem(AddItemView),
}

impl ToString for View {
    fn to_string(&self) -> String {
        match self {
            View::Transactions(_) => "Transactions".to_string(),
            View::Inventory(_) => "Inventory".to_string(),
            View::AddItem(_) => "Add Item".to_string(),
        }
    }
}

impl View {
    pub fn view(app: &App) -> Element<Message, Renderer<MyTheme>> {
        match &app.current_view {
            View::Transactions(v) => v.view().map(move |message| Message::Transactions(message)),
            View::Inventory(v) => v
                .view(&app.item_db)
                .map(move |message| Message::Inventory(message)),
            View::AddItem(v) => v.view().map(move |message| Message::AddItem(message)),
        }
    }
}

impl Default for View {
    fn default() -> Self {
        View::Transactions(TransactionsView::default())
    }
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
            Message::EventOccured(event) => match &mut self.current_view {
                View::Transactions(v) => {
                    v.update(TransactionsMessage::EventOccured(event), &mut self.item_db)
                }
                _ => {}
            },
            // Switch view
            Message::SwitchView(view) => {
                self.current_view = view;
            }
            // Close window and save database
            Message::Close => {
                self.item_db.save_yaml("./item_db.yaml");
                self.should_exit = true;
                println!("Closing gracefully, Saving Database.");
            }
            // Map to TransactionsMessage
            Message::Transactions(message) => {
                if let View::Transactions(v) = &mut self.current_view {
                    v.update(message, &mut self.item_db);
                }
            }
            // Map to InventoryMessage
            Message::Inventory(message) => {
                if let View::Inventory(v) = &mut self.current_view {
                    v.update(message, &mut self.item_db);
                }
            }
            Message::AddItem(message) => {
                if let View::AddItem(v) = &mut self.current_view {
                    v.update(message);
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message, Renderer<Self::Theme>> {
        Column::new()
            .push(tab_buttons(&self))
            .push(View::view(&self))
            .into()
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }
}

/// Render the tab buttons at the top of the app
fn tab_buttons(app: &App) -> Element<Message, Renderer<MyTheme>> {
    // Helper function that returns a ButtonStyle
    fn get_style(active: bool) -> ButtonStyle {
        if active {
            ButtonStyle::TabActive
        } else {
            ButtonStyle::TabInactive
        }
    }

    // Render tab buttons
    [
        View::Transactions(Default::default()),
        View::Inventory(Default::default()),
        View::AddItem(Default::default()),
    ]
    .iter()
    .fold(Row::new(), |row, view| {
        row.push(
            button(text(view.to_string()))
                .style(get_style(app.current_view.to_string() == view.to_string()))
                .on_press(Message::SwitchView(view.clone())),
        )
    })
    // spacer
    .push(button(text(" ")).width(Length::Fill))
    .push(
        // close button
        button(text("  x  "))
            .style(ButtonStyle::TabInactive)
            .on_press(Message::Close),
    )
    .width(Length::Fill)
    .into()
}
