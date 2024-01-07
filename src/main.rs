use iced::widget::{image, container, button, text, column as col, Rule, Row};
use iced::{executor, Length};
use iced::theme::Button;
use iced::alignment::Vertical;
use iced::{Application, Element, Settings, Theme, Command, Color, color};

mod card;
use card::{Deck, Hand};

#[derive(Default)]
struct TwentyOneButtonStyle;

impl iced::widget::button::StyleSheet for TwentyOneButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(color!(0xBBFF44))),
            text_color: Color::BLACK,
            ..Default::default()
        }
    }
    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(color!(0x559911))),
            text_color: Color::WHITE,
            ..Default::default()
        }
    }
    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        self.hovered(_style)
    }
}

struct IcedTwentyOne {
    deck: Deck,
    player_hand: Hand,
}

impl Default for IcedTwentyOne {
    fn default() -> IcedTwentyOne {
        let mut deck = Deck::new();
        let mut hand = Hand::new();
        deck.shuffle();
        hand.push(deck.deal_card().unwrap());
        IcedTwentyOne {
            deck: deck,
            player_hand: hand,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    DealCard
}

impl Application for IcedTwentyOne {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (IcedTwentyOne, Command<Self::Message>) {
        (IcedTwentyOne::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Iced Twenty-One")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::DealCard => {
                if let Some(new_card) = self.deck.deal_card() {
                    self.player_hand.push(new_card);
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut player_row = Row::new();
        for card in &self.player_hand.cards {
            player_row = player_row.push(image(String::from("img/") + &card.get_id() + ".png").height(Length::Fixed(250.)));
        }

        let table_col = col![
            player_row,
            Rule::horizontal(4.),
            text(self.player_hand.value().to_string()).size(35),
            button(text("Deal another card")).on_press(Message::DealCard).style(Button::Custom( Box::new(TwentyOneButtonStyle)))
        ].align_items(iced::Alignment::Center).spacing(10);

        container(table_col)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(Vertical::Bottom)
            .center_x()
            .padding(40)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Custom(Box::new(
            iced::theme::Custom::new(
                iced::theme::Palette {
                    background: color!(0x114411),
                    text: color!(0xCCFFCC),
                    primary: color!(0xffffff),
                    success: color!(0xffffff),
                    danger: color!(0xCC0000),
                }
            )
        ))
    }
}

pub fn main() -> iced::Result {
    IcedTwentyOne::run(Settings::default())
}
