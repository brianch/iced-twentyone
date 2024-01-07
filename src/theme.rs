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
            background_color: color!(0x477c47),
            text_color: Color::BLACK,
        }
    }
}

impl button::StyleSheet for TwentyOneTheme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(color!(0xFFFFFF))),
            text_color: Color::BLACK,
            ..Default::default()
        }
    }
    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(color!(0x324731))),
            text_color: Color::WHITE,
            ..Default::default()
        }
    }
    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        self.hovered(_style)
    }
}

impl container::StyleSheet for TwentyOneTheme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
                text_color: Some(Color::WHITE),
                ..Default::default()
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