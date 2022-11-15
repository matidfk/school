use inventory_view::{InventoryMessage, InventoryView};
use theme::MyTheme;

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
    widget::{button, column, text},
    window, Application, Command, Event, Renderer, Settings, Subscription,
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
    active_view: usize,
    show_password_modal: bool,
    should_exit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            item_db: ItemDB::load_yaml("./item_db.yaml"),
            should_exit: false,
            transactions_view: TransactionsView::default(),
            inventory_view: InventoryView::default(),
            show_password_modal: false,
            active_view: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    EventOccured(Event),

    Transactions(TransactionsMessage),
    Inventory(InventoryMessage),

    ClosePasswordModal,
    SetActiveView(usize),
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
                if self.active_view == 0 {
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
            Message::SetActiveView(i) => {
                if i == 1 {
                    self.show_password_modal = true;
                }
                self.active_view = i
            }
            Message::ClosePasswordModal => self.show_password_modal = false,
        }
        Command::none()
    }

    fn view(&self) -> Element {
        let content: Element = Tabs::new(self.active_view, Message::SetActiveView)
            .push(
                TabLabel::Text("Transactions".to_string()),
                self.transactions_view.view(&self.item_db),
            )
            .push(
                TabLabel::Text("Inventory".to_string()),
                self.inventory_view.view(&self.item_db),
            )
            .text_size(20)
            .tab_label_spacing(20)
            .tab_bar_height(iced::Length::Shrink)
            .into();

        Modal::new(self.show_password_modal, content, || {
            column![
                text("poopie"),
                button("exit").on_press(Message::ClosePasswordModal)
            ]
            .into()
            //     Card::new(
            //         Text::new("My modal"),
            //         Text::new("This is a modal!"), //Text::new("Zombie ipsum reversus ab viral inferno, nam rick grimes malum cerebro. De carne lumbering animata corpora quaeritis. Summus brains sit​​, morbo vel maleficia? De apocalypsi gorger omero undead survivor dictum mauris. Hi mindless mortuis soulless creaturas, imo evil stalking monstra adventus resi dentevil vultus comedat cerebella viventium. Qui animated corpse, cricket bat max brucks terribilem incessu zomby. The voodoo sacerdos flesh eater, suscitat mortuos comedere carnem virus. Zonbi tattered for solum oculi eorum defunctis go lum cerebro. Nescio brains an Undead zombies. Sicut malus putrid voodoo horror. Nigh tofth eliv ingdead.")
            //     )
            //     .foot(
            //         Row::new()
            //             .spacing(10)
            //             .padding(5)
            //             .width(Length::Fill)
            //             .push(
            //                 Button::new(Text::new("Cancel").horizontal_alignment(Horizontal::Center))
            //                     .width(Length::Fill)
            //                     .on_press(Message::CancelButtonPressed),
            //             )
            //             .push(
            //                 Button::new(Text::new("Ok").horizontal_alignment(Horizontal::Center))
            //                     .width(Length::Fill)
            //                     .on_press(Message::OkButtonPressed),
            //             ),
            //     )
            //     .max_width(300)
            //     //.width(Length::Shrink)
            //     .on_close(Message::CloseModal)
            //     .into()
            // })
            // .backdrop(Message::CloseModal)
            // .on_esc(Message::CloseModal)
            // .into()
        })
        .into()
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }
}
