use iced::widget::{container, text};
use iced::{Sandbox, Element, Settings};

#[derive(Default)]
struct IcedTwentyOne;

#[derive(Debug, Clone, Copy)]
enum Message {
}

impl Sandbox for IcedTwentyOne {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Iced Twenty-One")
    }

    fn update(&mut self, message: Message) {
    }

    fn view(&self) -> Element<Message> {
        let hello = text("Hello Iced");
        container(hello).into()
    }
}

pub fn main() -> iced::Result {
    IcedTwentyOne::run(Settings::default())
}
