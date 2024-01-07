use iced::widget::{button, container, text, rule};
use iced::{application, Color, color};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TwentyOneTheme {
    #[default]
    Green,
}

impl application::StyleSheet for TwentyOneTheme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: Color::TRANSPARENT,
            text_color: Color::BLACK,
        }
    }
}

#[derive(Default)]
pub enum ButtonStyle {
    /// No style.
    #[default]
    Table,
    Menu,
}

impl button::StyleSheet for TwentyOneTheme {
    type Style = ButtonStyle;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            ButtonStyle::Table => {
                button::Appearance {
                    background: Some(iced::Background::Color(color!(0xFFFFFF))),
                    text_color: Color::BLACK,
                    ..Default::default()
                }
            } ButtonStyle::Menu => {
                button::Appearance {
                    background: Some(iced::Background::Color(color!(0x477c47))),
                    text_color: Color::WHITE,
                    ..Default::default()
                }
            }
        }
    }
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        match style {
            ButtonStyle::Table => {
                button::Appearance {
                    background: Some(iced::Background::Color(color!(0x324731))),
                    text_color: Color::WHITE,
                    ..Default::default()
                }
            } ButtonStyle::Menu => {
                button::Appearance {
                    background: Some(iced::Background::Color(color!(0xFFFFFF))),
                    text_color: Color::BLACK,
                    ..Default::default()
                }
            }
        }
    }
    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        self.hovered(_style)
    }
}

#[derive(Default)]
pub enum ContainerStyle {
    /// No style.
    #[default]
    Table,
    Menu,
}

impl container::StyleSheet for TwentyOneTheme {
    type Style = ContainerStyle;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            ContainerStyle::Table => {
                container::Appearance {
                    text_color: Some(Color::WHITE),
                    background: Some(iced::Background::Color(color!(0x477c47))),
                    ..Default::default()
                }
            } ContainerStyle::Menu => {
                container::Appearance {
                    text_color: Some(Color::WHITE),
                    background: Some(iced::Background::Color(color!(0x192f19))),
                    ..Default::default()
                }
            }
        }
    }
}

impl text::StyleSheet for TwentyOneTheme {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        text::Appearance::default()
    }
}

impl rule::StyleSheet for TwentyOneTheme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> rule::Appearance {
        rule::Appearance {
            color: Color::WHITE,
            width: 1,
            radius: 0.0.into(),
            fill_mode: rule::FillMode::Full,
        }
    }
}