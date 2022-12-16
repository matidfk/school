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
    pub slider_scale: f64,
}

impl Default for SettingsView {
    fn default() -> Self {
        Self {
            input_password: "".to_owned(),
            slider_scale: 1.0,
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
        // right side
        column![
            row![
                text_input("Admin Password", &self.input_password, |input| {
                    Message::Settings(SettingsMessage::PasswordChanged(input))
                }),
                button(text("Save Password"))
                    .on_press(Message::Settings(SettingsMessage::SavePassword))
            ]
            .width(Length::Fill),
            row![
                slider(0.5..=2.0, self.slider_scale, |value| {
                    Message::Settings(SettingsMessage::ScaleChanged(value))
                })
                .step(0.01),
                text(self.slider_scale).width(Length::Units(60)),
            ],
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
            SettingsMessage::ScaleChanged(value) => self.slider_scale = value,
        }
    }
}
