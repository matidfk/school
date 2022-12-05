#![feature(iter_next_chunk)]
#![feature(iter_array_chunks)]
#![feature(array_chunks)]
use inventory_view::{InventoryMessage, InventoryView};
use item_creation_view::ItemCreationView;
use theme::MyTheme;

mod inventory_view;
mod item;
mod item_creation_view;
mod item_db;
mod theme;
mod transaction;
mod transactions_view;
mod utils;

use iced::{
    alignment::Horizontal,
    executor,
    subscription::events,
    widget::{button, column, text, text_input, Space},
    window, Alignment, Application, Color, Command, Event, Length, Renderer, Settings,
    Subscription,
};

use iced_aw::{Modal, TabLabel, Tabs};

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

// Aliases
pub type Element<'a> = iced::Element<'a, Message, Renderer<MyTheme>>;

/// The state model of the application
pub struct App {
    item_db: ItemDB,
    transactions_view: TransactionsView,
    inventory_view: InventoryView,
    item_creation_view: ItemCreationView,
    active_view: ViewIndex,
    desired_view: Option<ViewIndex>,
    password_input: String,
    should_exit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            item_db: ItemDB::load_yaml("./item_db.yaml"),
            should_exit: false,
            transactions_view: TransactionsView::default(),
            inventory_view: InventoryView::default(),
            item_creation_view: ItemCreationView::default(),
            active_view: ViewIndex::Transactions,
            password_input: Default::default(),
            desired_view: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    EventOccured(Event),

    Transactions(TransactionsMessage),
    Inventory(InventoryMessage),

    ClosePasswordModal,
    PasswordChanged(String),
    SetActiveView(ViewIndex),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ViewIndex {
    Transactions = 0,
    Inventory = 1,
    ItemCreation = 2,
}

impl ViewIndex {
    pub fn to_usize(&self) -> usize {
        match self {
            ViewIndex::Transactions => 0,
            ViewIndex::Inventory => 1,
            ViewIndex::ItemCreation => 2,
        }
    }
    pub fn from_usize(usize: usize) -> Self {
        match usize {
            0 => Self::Transactions,
            1 => Self::Inventory,
            2 => Self::ItemCreation,
            _ => panic!("oh no"),
        }
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
            Message::EventOccured(event) => {
                // if transactions view is open
                if self.active_view == ViewIndex::Transactions {
                    self.transactions_view.update(
                        TransactionsMessage::EventOccured(event.clone()),
                        &mut self.item_db,
                    )
                }
                // upon receiving signal to close
                if let Event::Window(window::Event::CloseRequested) = event {
                    self.item_db.save_yaml("./item_db.yaml");
                    self.should_exit = true;
                    println!("Closing");
                }
            }

            Message::Transactions(message) => {
                self.transactions_view.update(message, &mut self.item_db)
            }
            Message::Inventory(message) => self.inventory_view.update(message, &mut self.item_db),
            Message::SetActiveView(new_index) => {
                // inventory
                const PASSWORD_PROTECTED_VIEWS: &'static [ViewIndex] = &[ViewIndex::Inventory];

                if PASSWORD_PROTECTED_VIEWS.contains(&new_index) {
                    self.desired_view = Some(new_index);
                } else {
                    self.active_view = new_index
                }
            }
            Message::ClosePasswordModal => {
                if self.password_input == "password123" {
                    self.active_view = self.desired_view.unwrap();
                } else {
                    std::process::Command::new("notify-send")
                        .arg("Incorrect Password")
                        .status()
                        .expect("Failed to notify");
                }
                self.desired_view = None;
                self.password_input.clear();
            }
            Message::PasswordChanged(v) => self.password_input = v,
        }
        Command::none()
    }

    fn view(&self) -> Element {
        let content: Element = Tabs::new(self.active_view as usize, |index_usize| {
            Message::SetActiveView(ViewIndex::from_usize(index_usize))
        })
        .push(
            TabLabel::Text("Transactions".to_string()),
            self.transactions_view.view(&self.item_db),
        )
        .push(
            TabLabel::Text("Inventory".to_string()),
            self.inventory_view.view(&self.item_db),
        )
        .push(
            TabLabel::Text("Item Creation".to_string()),
            self.item_creation_view.view(),
        )
        .text_size(20)
        // .tab_label_spacing(20)
        .tab_bar_height(iced::Length::Shrink)
        .into();

        let element: Element = Modal::new(self.desired_view.is_some(), content, || {
            render_password_prompt(&self.password_input)
        })
        .into();

        element
        // element.explain(Color::BLACK)
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }
}

fn render_password_prompt<'a>(password_input: &String) -> Element<'a> {
    column![
        text("A Password is required"),
        text_input("Enter Password", password_input, Message::PasswordChanged)
            .password()
            .on_submit(Message::ClosePasswordModal),
        button(
            text("Confirm")
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center)
        )
        .on_press(Message::ClosePasswordModal)
        .width(Length::Fill)
    ]
    .spacing(10)
    .align_items(Alignment::Center)
    .width(Length::Units(300))
    .into()
}
