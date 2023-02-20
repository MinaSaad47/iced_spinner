use std::time::Duration;
use iced::{Alignment, Element, Length, Sandbox, Settings, widget::{button, column, text, row, slider, container}};
use iced_spinner::spinner;

struct Example {
    spread: f32,
}

#[derive(Clone, Debug)]
enum Message {
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        Self {
            spread: 0.0,
        }
    }

    fn title(&self) -> String {
        String::from("Spinner")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let s = spinner()
            .spread(0.2)
            .count(1)
            .rate(Duration::from_secs_f32(1.0))
            .width(Length::Fixed(64.0)).height(Length::Fixed(64.0));

        column![
        container(s)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(16)
            .center_x()
            .center_y(),
        ]
        .into()
    }
}

fn main() -> iced::Result {
    Example::run(Settings::default())
}