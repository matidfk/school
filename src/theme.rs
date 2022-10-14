use iced::{
    application,
    widget::{button, container, text, text_input},
    Background, Color,
};

const BACKGROUND: Color = Color::from_rgb(1.0, 1.0, 1.0);
const TEXT_COLOR: Color = Color::from_rgb(0.0, 0.0, 0.0);
const ACCENT: Color = Color::from_rgb(0.8, 0.2, 0.2);

#[derive(Default, Clone, Copy)]
pub struct MyTheme;

impl application::StyleSheet for MyTheme {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: BACKGROUND,
            text_color: TEXT_COLOR,
        }
    }
}

impl button::StyleSheet for MyTheme {
    type Style = ();
    fn active(&self, _style: Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(ACCENT)),
            ..Default::default()
        }
    }
}

impl container::StyleSheet for MyTheme {
    type Style = ();
    fn appearance(&self, _style: Self::Style) -> container::Appearance {
        container::Appearance {
            // text_color: (),
            background: Some(Background::Color(Color::from_rgb(0.9, 0.9, 0.9))),
            // border_radius: (),
            border_width: 2.0,
            border_color: Color::BLACK,
            ..Default::default()
        }
    }
}

impl text::StyleSheet for MyTheme {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        text::Appearance {
            color: Some(Color::BLACK),
        }
    }
}

impl text_input::StyleSheet for MyTheme {
    type Style = ();

    fn active(&self, _style: Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(Color::from_rgb(0.6, 0.8, 0.9)),
            border_radius: 3.0,
            border_width: 2.0,
            border_color: Color::BLACK,
        }
    }

    fn focused(&self, _style: Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(Color::from_rgb(0.6, 0.8, 0.9)),
            border_radius: 3.0,
            border_width: 2.0,
            border_color: Color::BLACK,
        }
    }

    fn placeholder_color(&self, _style: Self::Style) -> Color {
        Color::from_rgb(0.5, 0.5, 0.5)
    }

    fn value_color(&self, _style: Self::Style) -> Color {
        Color::BLACK
    }

    fn selection_color(&self, _style: Self::Style) -> Color {
        Color::from_rgb(0.0, 0.3, 0.9)
    }
}
