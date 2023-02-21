use iced::{
    widget::{column, container},
    Element, Length, Sandbox, Settings,
};
use iced::widget::vertical_space;
use iced_spinner::spinner;
use std::time::Duration;
use iced_native::widget::scrollable;

struct ManySpinnersExample;

#[derive(Clone, Debug)]
enum Message {}

impl Sandbox for ManySpinnersExample {
    type Message = Message;

    fn new() -> Self {
        Self {}
    }

    fn title(&self) -> String {
        String::from("Many Spinners")
    }

    fn update(&mut self, message: Self::Message) {
        match message {}
    }

    fn view(&self) -> Element<Self::Message> {
        let content = (0i32..500i32).fold(column![], |column, _|{
            column.push(spinner().width(Length::Fixed(64.0)).height(Length::Fixed(64.0)))
        });

        scrollable(content).into()
    }
}

fn main() -> iced::Result {
    ManySpinnersExample::run(Settings::default())
}
