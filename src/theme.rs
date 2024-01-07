use iced::widget::{button, container, text, rule, radio};
use iced::{application, Color};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TwentyOneTheme {
    Green,
    #[default]
    Burgundy
}

impl TwentyOneTheme {
    pub fn palette(&self) -> TwentyOnePalette {
        match self {
            TwentyOneTheme::Green => TwentyOnePalette::GREEN,
            TwentyOneTheme::Burgundy => TwentyOnePalette::BURGUNDY,
        }
    }
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
                    background: Some(iced::Background::Color(self.palette().tertiary)),
                    text_color: self.palette().text_dark,
                    ..Default::default()
                }
            } ButtonStyle::Menu => {
                button::Appearance {
                    background: Some(iced::Background::Color(self.palette().primary)),
                    text_color: self.palette().text_light,
                    ..Default::default()
                }
            }
        }
    }
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        match style {
            ButtonStyle::Table => {
                button::Appearance {
                    background: Some(iced::Background::Color(self.palette().secondary)),
                    text_color: Color::WHITE,
                    ..Default::default()
                }
            } ButtonStyle::Menu => {
                button::Appearance {
                    background: Some(iced::Background::Color(self.palette().tertiary)),
                    text_color: Color::BLACK,
                    ..Default::default()
                }
            }
        }
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
                    text_color: Some(self.palette().text_light),
                    background: Some(iced::Background::Color(self.palette().primary)),
                    ..Default::default()
                }
            } ContainerStyle::Menu => {
                container::Appearance {
                    text_color: Some(self.palette().text_light),
                    background: Some(iced::Background::Color(self.palette().secondary)),
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

impl radio::StyleSheet for TwentyOneTheme {
    type Style = ();

    fn active(&self, _style: &Self::Style, _is_selected: bool) -> radio::Appearance {
        radio::Appearance {
            background: iced::Background::Color(self.palette().primary),
            dot_color: self.palette().secondary,
            border_width: 0.5,
            border_color: Color::TRANSPARENT,
            text_color: Some(self.palette().text_light),
        }
    }

    fn hovered(&self, _style: &Self::Style, _is_selected: bool) -> radio::Appearance {
        radio::Appearance {
            background: iced::Background::Color(self.palette().tertiary),
            dot_color: self.palette().primary,
            border_width: 0.5,
            border_color: Color::TRANSPARENT,
            text_color: Some(self.palette().text_light),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TwentyOnePalette {
    pub primary: Color,
    pub secondary: Color,
    pub tertiary: Color,
    pub text_light: Color,
    pub text_dark: Color,
}

impl TwentyOnePalette {
    pub const GREEN: Self = Self {
        primary: Color::from_rgb(
            0x47 as f32 / 255.0,
            0x7c as f32 / 255.0,
            0x47 as f32 / 255.0,
        ),
        secondary: Color::from_rgb(
            0x32 as f32 / 255.0,
            0x47 as f32 / 255.0,
            0x31 as f32 / 255.0,
        ),
        tertiary: Color::WHITE,
        text_dark: Color::BLACK,
        text_light: Color::WHITE,
    };
    pub const BURGUNDY: Self = Self {
        primary: Color::from_rgb(
            0x99 as f32 / 255.0,
            0x62 as f32 / 255.0,
            0x6b as f32 / 255.0,
        ),
        secondary: Color::from_rgb(
            0x4b as f32 / 255.0,
            0x30 as f32 / 255.0,
            0x34 as f32 / 255.0,
        ),
        tertiary: Color::WHITE,
        text_dark: Color::BLACK,
        text_light: Color::WHITE,
    };
}

