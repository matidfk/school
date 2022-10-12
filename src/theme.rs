use iced::{
    application,
    theme::{Application, Button, Container, Text},
    widget::{button, container, text, text_input},
    Background, Color,
};

#[derive(Default, Clone, Copy)]
pub struct MyTheme;

impl application::StyleSheet for MyTheme {
    type Style = Application;

    fn appearance(&self, style: Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: Color::from_rgb(0.8, 0.9, 0.3),
            text_color: Color::BLACK,
        }
    }
}

impl button::StyleSheet for MyTheme {
    type Style = Button;
    fn active(&self, style: Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(1.0, 0.4, 0.6))),
            ..Default::default()
        }
    }
}

impl container::StyleSheet for MyTheme {
    type Style = Container;
    fn appearance(&self, style: Self::Style) -> container::Appearance {
        container::Appearance {
            // text_color: (),
            background: Some(Background::Color(Color::from_rgb(0.5, 0.5, 0.5))),
            // border_radius: (),
            border_width: 2.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }
}

impl text::StyleSheet for MyTheme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        text::Appearance {
            color: Some(Color::BLACK),
        }
    }
}

impl text_input::StyleSheet for MyTheme {
    type Style = ();

    fn active(&self, style: Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(Color::from_rgb(0.6, 0.8, 0.9)),
            border_radius: 3.0,
            border_width: 2.0,
            border_color: Color::BLACK,
        }
    }

    fn focused(&self, style: Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(Color::from_rgb(0.6, 0.8, 0.9)),
            border_radius: 3.0,
            border_width: 2.0,
            border_color: Color::BLACK,
        }
    }

    fn placeholder_color(&self, style: Self::Style) -> Color {
        Color::from_rgb(0.5, 0.5, 0.5)
    }

    fn value_color(&self, style: Self::Style) -> Color {
        Color::BLACK
    }

    fn selection_color(&self, style: Self::Style) -> Color {
        Color::from_rgb(0.0, 0.3, 0.9)
    }
}
