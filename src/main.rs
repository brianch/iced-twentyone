use iced::widget::{image, container, button, text, column as col, Rule, Row, row};
use iced::{executor, Length};
use iced::alignment::Vertical;
use iced::{Application, Element, Settings, Command};
use iced::Subscription;
use std::time::{Duration, Instant};
use iced::time;

mod theme;
mod card;
use card::{Deck, Hand};

#[derive(PartialEq)]
enum GameStage {
    Init, Dealing, Standing, PlayerWon, HouseWon, Tie
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
    DealCard,
    Stand,
    DealerDraw(Instant),
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
                    if self.player_hand.value() == 21 {
                        if self.dealer_hand.value() == 21 {
                            self.game_stage = GameStage::Tie;
                        } else {
                            self.game_stage = GameStage::PlayerWon;
                        }
                    } else if self.player_hand.value() > 21 {
                        self.game_stage = GameStage::HouseWon;
                    }
                }
            } Message::Start => {
                self.game_stage = GameStage::Dealing;
            } Message::Stand => {
                self.game_stage = GameStage::Standing;
                if self.dealer_hand.value() == self.player_hand.value() {
                    self.game_stage = GameStage::Tie;
                } if self.dealer_hand.value() > self.player_hand.value()  {
                    self.game_stage = GameStage::HouseWon;
                } else if self.dealer_hand.value() < 17 {
                    self.game_stage = GameStage::Standing;
                } else {
                    self.game_stage = GameStage::PlayerWon;
                }
            } Message::DealerDraw(_) => {
                self.dealer_hand.push(self.deck.deal_card().unwrap());
                if self.dealer_hand.value() > 21 {
                    self.game_stage = GameStage::PlayerWon;
                } else if self.dealer_hand.value() > self.player_hand.value()  {
                    self.game_stage = GameStage::HouseWon;
                } else if self.dealer_hand.value() == self.player_hand.value()  {
                    self.game_stage = GameStage::Tie;
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message, iced::Renderer<theme::TwentyOneTheme>> {
        let mut dealer_hand_val = String::from("?");
        let mut dealer_row = Row::new().spacing(10);
        let mut game_result = text("");
        if self.game_stage == GameStage::Init || self.game_stage == GameStage::Dealing {
            dealer_row = dealer_row
                .push(image(String::from("img/") + &self.dealer_hand.cards[0].get_id() + ".png").height(Length::Fixed(200.)))
                .push(image(String::from("img/back.png")).height(Length::Fixed(200.)));
        } else {
            if self.game_stage == GameStage::PlayerWon {
                game_result = text("You won!!! Congrats :)").size(50);
            } else if self.game_stage == GameStage::HouseWon {
                game_result = text("You Lost!").size(50);
            } else if self.game_stage == GameStage::Tie {
                game_result = text("It's a tie!!").size(50);
            }
            for card in &self.dealer_hand.cards {
                dealer_row = dealer_row.push(image(String::from("img/") + &card.get_id() + ".png").height(Length::Fixed(200.)));
            }
            dealer_hand_val = self.dealer_hand.value().to_string();
        };

        let dealer_info = container(
            col![
                dealer_row,
                text(dealer_hand_val).size(35),
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
            let btn_row = if self.game_stage == GameStage::Dealing {
                row![
                    button(text("Deal another card")).on_press(Message::DealCard),
                    button(text("Stand")).on_press(Message::Stand),
                ].spacing(30)
            } else {
                row![
                    button(text("Deal another card")),
                    button(text("Stand")),
                ].spacing(30)
            };
            let mut player_row = Row::new().spacing(10);
            for card in &self.player_hand.cards {
                player_row = player_row.push(image(String::from("img/") + &card.get_id() + ".png").height(Length::Fixed(200.)));
            }
            col![
                Rule::horizontal(4.),
                text(self.player_hand.value().to_string()).size(35),
                player_row,
                btn_row,
            ].width(Length::Fill).align_items(iced::Alignment::Center).spacing(20)
        };
        let player_info = container(player_info_col).height(Length::Fill).align_y(Vertical::Bottom);

        let table_col = col![
            dealer_info,
            game_result,
            player_info,
        ].align_items(iced::Alignment::Center).spacing(10);

        container(table_col)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .padding(40)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.game_stage {
            GameStage::Standing => {
                time::every(Duration::from_millis(1000)).map(Message::DealerDraw)
            } _ => Subscription::none(),
        }
    }

}

pub fn main() -> iced::Result {
    IcedTwentyOne::run(Settings::default())
}
