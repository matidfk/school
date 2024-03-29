use iced::{
    widget::{button, column, row, slider, text, text_input},
    Length,
};

use crate::{
    utils::{notify, set_password},
    Element, Message,
};

#[derive(Debug, Clone, PartialEq)]
pub struct SettingsView {
    input_password: String,
    pub ui_scale: f64,
}

impl Default for SettingsView {
    fn default() -> Self {
        Self {
            input_password: "".to_owned(),
            ui_scale: 1.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SettingsMessage {
    PasswordChanged(String),
    ScaleChanged(f64),
    SavePassword,
}

impl SettingsView {
    pub fn view(&self) -> Element {
        column![
            // password
            row![
                text_input("Admin Password", &self.input_password, |input| {
                    Message::Settings(SettingsMessage::PasswordChanged(input))
                }),
                button(text("Save Password"))
                    .on_press(Message::Settings(SettingsMessage::SavePassword))
            ]
            .spacing(10)
            .width(Length::Fill),
            // ui scale
            row![
                "UI Scale",
                slider(0.5..=2.0, self.ui_scale, |value| {
                    Message::Settings(SettingsMessage::ScaleChanged(value))
                })
                .step(0.01),
                text(self.ui_scale).width(Length::Units(60)),
            ]
            .spacing(10),
        ]
        .padding(20)
        .spacing(10)
        .into()
    }

    pub fn update(&mut self, message: SettingsMessage) {
        match message {
            SettingsMessage::PasswordChanged(value) => self.input_password = value,
            SettingsMessage::SavePassword => {
                set_password(&self.input_password);
                notify(
                    "Password set!",
                    &format!("Set password to {}", self.input_password),
                )
            }
            SettingsMessage::ScaleChanged(value) => self.ui_scale = value,
        }
    }
}
