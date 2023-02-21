use iced::{
    widget::{column, container},
    Element, Length, Sandbox, Settings,
};
use iced_spinner::spinner;
use std::time::Duration;

struct Example;

#[derive(Clone, Debug)]
enum Message {}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        Self {}
    }

    fn title(&self) -> String {
        String::from("Spinner")
    }

    fn update(&mut self, message: Self::Message) {
        match message {}
    }

    fn view(&self) -> Element<Self::Message> {
        let s = spinner()
            .rate(Duration::from_secs_f32(1.0))
            .width(Length::Fixed(64.0))
            .height(Length::Fixed(64.0));

        column![container(s)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y(),]
        .into()
    }
}

fn main() -> iced::Result {
    Example::run(Settings::default())
}
