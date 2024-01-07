use iced::widget::{image, container, button, text, column as col};
use iced::{executor, Length};
use iced::alignment::Vertical;
use iced::{Application, Element, Settings, Theme, Command};

mod card;
use card::{Deck, Hand};

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
                    self.player_hand.pop();
                    self.player_hand.push(new_card);
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let img_path = String::from("img/") + &self.player_hand.cards[0].get_id() + ".png";

        let table_col = col![
            image(img_path).height(Length::Fixed(500.)),
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
