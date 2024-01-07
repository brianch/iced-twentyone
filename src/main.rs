use iced::widget::{image, container, button, text, column as col, Rule, Row};
use iced::{executor, Length};
use iced::alignment::Vertical;
use iced::{Application, Element, Settings, Command};

mod theme;
mod card;
use card::{Deck, Hand};

struct IcedTwentyOne {
    deck: Deck,
    player_hand: Hand,
    dealer_hand: Hand,
}

impl Default for IcedTwentyOne {
    fn default() -> IcedTwentyOne {
        let mut deck = Deck::new();
        let mut player = Hand::new();
        let mut dealer = Hand::new();
        deck.shuffle();
        player.push(deck.deal_card().unwrap());
        player.push(deck.deal_card().unwrap());
        dealer.push(deck.deal_card().unwrap());
        dealer.push(deck.deal_card().unwrap());

        IcedTwentyOne {
            deck: deck,
            player_hand: player,
            dealer_hand: dealer
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
    type Theme = theme::TwentyOneTheme;

    fn new(_flags: ()) -> (IcedTwentyOne, Command<Self::Message>) {
        (IcedTwentyOne::default(), iced::window::maximize(true))
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

    fn view(&self) -> Element<Message, iced::Renderer<theme::TwentyOneTheme>> {
        let mut dealer_row = Row::new();
        for card in &self.dealer_hand.cards {
            dealer_row = dealer_row.push(image(String::from("img/") + &card.get_id() + ".png").height(Length::Fixed(250.)));
        }

        let mut player_row = Row::new();
        for card in &self.player_hand.cards {
            player_row = player_row.push(image(String::from("img/") + &card.get_id() + ".png").height(Length::Fixed(250.)));
        }

        let table_col = col![
            text(self.dealer_hand.value().to_string()).size(35),
            Rule::horizontal(4.),
            dealer_row,
            player_row,
            Rule::horizontal(4.),
            text(self.player_hand.value().to_string()).size(35),
            button(text("Deal another card")).on_press(Message::DealCard),
        ].align_items(iced::Alignment::Center).spacing(10);

        container(table_col)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(Vertical::Bottom)
            .center_x()
            .padding(40)
            .into()
    }
}

pub fn main() -> iced::Result {
    IcedTwentyOne::run(Settings::default())
}
