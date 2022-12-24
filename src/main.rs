#![feature(iter_next_chunk)]
#![feature(iter_array_chunks)]
#![feature(array_chunks)]
use inventory_view::{InventoryMessage, InventoryView};
use item::Item;
use item_creation_view::{ItemCreationMessage, ItemCreationView};
use settings_view::{SettingsMessage, SettingsView};
use theme::MyTheme;

mod inventory_view;
mod item;
mod item_creation_view;
mod item_db;
mod settings_view;
mod theme;
mod transaction;
mod transactions_view;
mod utils;

use iced::{
    alignment::Horizontal,
    executor,
    subscription::events,
    widget::text_input::{focus, Id},
    widget::{button, column, text, text_input},
    window, Alignment, Application, Color, Command, Event, Length, Renderer, Settings,
    Subscription,
};

use iced_aw::{Modal, TabLabel, Tabs};

use transactions_view::{TransactionsMessage, TransactionsView};
use utils::{encrypt, get_password, notify};

use crate::item_db::ItemDB;

pub fn main() -> iced::Result {
    App::run(Settings {
        window: iced::window::Settings {
            // size: (1280, 720),
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
    settings_view: SettingsView,

    active_view: ViewIndex,
    desired_view: Option<ViewIndex>,
    password_input: String,
    password_input_id: Id,
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
            settings_view: SettingsView::default(),
            active_view: ViewIndex::Transactions,
            password_input: Default::default(),
            password_input_id: Id::new("password"),
            desired_view: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    EventOccured(Event),

    Transactions(TransactionsMessage),
    Inventory(InventoryMessage),
    ItemCreation(ItemCreationMessage),
    EditItem(Item),
    Settings(SettingsMessage),

    ClosePasswordModal,
    PasswordChanged(String),
    SetActiveView(ViewIndex),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ViewIndex {
    Transactions = 0,
    Inventory = 1,
    ItemCreation = 2,
    Settings = 3,
}

impl ViewIndex {
    pub fn to_usize(&self) -> usize {
        match self {
            ViewIndex::Transactions => 0,
            ViewIndex::Inventory => 1,
            ViewIndex::ItemCreation => 2,
            ViewIndex::Settings => 3,
        }
    }
    pub fn from_usize(usize: usize) -> Self {
        match usize {
            0 => Self::Transactions,
            1 => Self::Inventory,
            2 => Self::ItemCreation,
            3 => Self::Settings,
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
        String::from("Swansea Food Centre POS Software")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        let mut command = Command::none();
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
            Message::ItemCreation(message) => {
                if let Some(message) = self.item_creation_view.update(message, &mut self.item_db) {
                    self.update(message);
                }
            }
            Message::Settings(message) => self.settings_view.update(message),
            Message::SetActiveView(new_index) => {
                // inventory
                const PASSWORD_PROTECTED_VIEWS: &'static [ViewIndex] = &[
                    ViewIndex::Inventory,
                    ViewIndex::ItemCreation,
                    ViewIndex::Settings,
                ];

                if PASSWORD_PROTECTED_VIEWS.contains(&new_index)
                    && !PASSWORD_PROTECTED_VIEWS.contains(&self.active_view)
                {
                    self.desired_view = Some(new_index);
                    command = focus(self.password_input_id.clone());
                } else {
                    self.active_view = new_index;
                }

                if new_index == ViewIndex::ItemCreation {
                    self.item_creation_view.set_item(None);
                }
            }
            Message::ClosePasswordModal => {
                if encrypt(&self.password_input) == get_password() {
                    self.active_view = self.desired_view.unwrap();
                } else {
                    notify("Access denied", "Incorrect Password");
                }
                self.desired_view = None;
                self.password_input.clear();
            }
            Message::PasswordChanged(v) => self.password_input = v,
            Message::EditItem(item) => {
                self.active_view = ViewIndex::ItemCreation;
                self.item_creation_view.set_item(Some(item));
            }
        }
        command
    }

    fn view(&self) -> Element {
        let content: Element = Tabs::new(self.active_view.to_usize(), |index| {
            Message::SetActiveView(ViewIndex::from_usize(index))
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
        .push(
            TabLabel::Text("Settings".to_string()),
            self.settings_view.view(),
        )
        .text_size(20)
        // .tab_label_spacing(20)
        .tab_bar_height(iced::Length::Shrink)
        .into();

        let element: Element = Modal::new(self.desired_view.is_some(), content, || {
            render_password_prompt(&self.password_input, self.password_input_id.clone())
        })
        .into();

        element
        // element.explain(Color::BLACK)
    }
    fn scale_factor(&self) -> f64 {
        self.settings_view.ui_scale
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }
}

fn render_password_prompt<'a>(password_input: &String, id: Id) -> Element<'a> {
    column![
        text("A Password is required"),
        text_input("Enter Password", password_input, Message::PasswordChanged)
            .id(id)
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
