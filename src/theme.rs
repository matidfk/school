use iced::{
    application,
    widget::{
        button, container,
        scrollable::{self, Scroller},
        text, text_input,
    },
    Background, Color, Vector,
};
use iced_aw::{native::modal, tabs};

const BACKGROUND: Color = Color::from_rgb(1.0, 1.0, 1.0);
const BACKGROUND_DARKER: Color = Color::from_rgb(0.9, 0.9, 0.9);
const TEXT_COLOR: Color = Color::from_rgb(0.0, 0.0, 0.0);
const ACCENT: Color = Color::from_rgb(0.8, 0.2, 0.2);
const BORDER_RADIUS: f32 = 5.0;

/// The theme for the application
#[derive(Default, Clone, Copy)]
pub struct MyTheme;

impl modal::StyleSheet for MyTheme {
    type Style = ();

    fn active(&self, style: Self::Style) -> iced_aw::style::modal::Appearance {
        iced_aw::style::modal::Appearance {
            background: Background::Color(BACKGROUND),
        }
    }
}

impl tabs::StyleSheet for MyTheme {
    type Style = ();

    fn active(&self, style: Self::Style, is_active: bool) -> iced_aw::style::tab_bar::Appearance {
        iced_aw::style::tab_bar::Appearance {
            background: Some(Background::Color(BACKGROUND)),
            border_color: Some(TEXT_COLOR),
            border_width: 0.0,
            tab_label_background: if is_active {
                Background::Color(ACCENT)
            } else {
                Background::Color(BACKGROUND)
            },
            tab_label_border_color: TEXT_COLOR,
            tab_label_border_width: 1.0,
            // icon_color: todo!(),
            // text_color: todo!(),
            ..Default::default()
        }
    }

    fn hovered(&self, style: Self::Style, is_active: bool) -> iced_aw::style::tab_bar::Appearance {
        self.active(style, is_active)
    }
}

impl application::StyleSheet for MyTheme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: BACKGROUND,
            text_color: TEXT_COLOR,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum ButtonStyle {
    Item,
    ItemSelected,
    #[default]
    Important,
}

impl button::StyleSheet for MyTheme {
    type Style = ButtonStyle;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            ButtonStyle::Item => button::Appearance {
                background: Some(Background::Color(BACKGROUND_DARKER)),
                // shadow_offset: Vector::default(),
                border_radius: BORDER_RADIUS,
                border_color: TEXT_COLOR,
                border_width: 1.0,
                ..Default::default()
            },
            ButtonStyle::ItemSelected => button::Appearance {
                background: Some(Background::Color(ACCENT)),
                ..self.active(&ButtonStyle::Item)
            },
            ButtonStyle::Important => button::Appearance {
                background: Some(Background::Color(ACCENT)),
                border_radius: BORDER_RADIUS,
                border_color: TEXT_COLOR,
                border_width: 1.0,
                ..Default::default()
            },
        }
    }

    fn hovered(&self, style: &ButtonStyle) -> button::Appearance {
        let active = self.active(&style);

        // no shadow on items and tabs
        if style == &ButtonStyle::Item || style == &ButtonStyle::ItemSelected {
            return active;
        }

        button::Appearance {
            shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
            ..active
        }
    }
}

impl container::StyleSheet for MyTheme {
    type Style = ();
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            // text_color: (),
            background: Some(Background::Color(BACKGROUND_DARKER)),
            // border_radius: (),
            border_width: 1.0,
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

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(BACKGROUND_DARKER),
            border_radius: BORDER_RADIUS,
            border_width: 1.0,
            border_color: Color::BLACK,
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style)
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb(0.5, 0.5, 0.5)
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        Color::BLACK
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        ACCENT
    }
}

impl scrollable::StyleSheet for MyTheme {
    type Style = ();

    fn active(&self, style: &Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Some(Background::Color(BACKGROUND_DARKER)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::BLACK,
            scroller: Scroller {
                color: ACCENT,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::BLACK,
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> scrollable::Scrollbar {
        self.active(style)
    }

    fn dragging(&self, style: &Self::Style) -> scrollable::Scrollbar {
        self.active(style)
    }
}
