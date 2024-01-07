use iced::widget::{image, container, button, text, column as col, Rule, Row, row};
use iced::{executor, Length};
use iced::alignment::Vertical;
use iced::{Application, Element, Settings, Command};

mod theme;
mod card;
use card::{Deck, Hand};

#[derive(PartialEq)]
enum GameStage {
    Init, Dealing
}

struct IcedTwentyOne {
    deck: Deck,
    player_hand: Hand,
    dealer_hand: Hand,
    game_stage: GameStage,
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
            dealer_hand: dealer,
            game_stage: GameStage::Init,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Start,
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
            } Message::Start => {
                self.game_stage = GameStage::Dealing;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message, iced::Renderer<theme::TwentyOneTheme>> {

        let dealer_row = row![
            image(String::from("img/") + &self.dealer_hand.cards[0].get_id() + ".png").height(Length::Fixed(200.)),
            image(String::from("img/back.png")).height(Length::Fixed(200.))
        ].spacing(10);

        let dealer_info = container(
            col![
                dealer_row,
                text("?").size(35),
                Rule::horizontal(4.),
            ].width(Length::Fill).align_items(iced::Alignment::Center).spacing(20)
        ).height(Length::Fill).align_y(Vertical::Top);

        let player_info_col = if self.game_stage == GameStage::Init {
            col![
                Rule::horizontal(4.),
                text("?").size(35),
                row![
                    image(String::from("img/") + &self.player_hand.cards[0].get_id() + ".png").height(Length::Fixed(200.)),
                    image(String::from("img/back.png")).height(Length::Fixed(200.)),
                ].spacing(10),
                button(text("Start Game")).on_press(Message::Start),
            ].width(Length::Fill).align_items(iced::Alignment::Center).spacing(20)
        } else {
            let mut player_row = Row::new().spacing(10);
            for card in &self.player_hand.cards {
                player_row = player_row.push(image(String::from("img/") + &card.get_id() + ".png").height(Length::Fixed(200.)));
            }
            col![
                Rule::horizontal(4.),
                text(self.player_hand.value().to_string()).size(35),
                player_row,
                button(text("Deal another card")).on_press(Message::DealCard),
            ].width(Length::Fill).align_items(iced::Alignment::Center).spacing(20)
        };
        let player_info = container(player_info_col).height(Length::Fill).align_y(Vertical::Bottom);

        let table_col = col![
            dealer_info,
            player_info,
        ].align_items(iced::Alignment::Center).spacing(10);

        container(table_col)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .padding(40)
            .into()
    }

}

pub fn main() -> iced::Result {
    IcedTwentyOne::run(Settings::default())
}
