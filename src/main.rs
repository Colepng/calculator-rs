use iced::widget::{button, Column, text};
use iced::{Alignment, Element, Settings, Sandbox};

pub fn main() -> iced::Result {
    Calcultor::run(Settings::default())
}

struct Calcultor {
    input: i64,
    result: i64,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Plus,
    Minus,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
}

impl Sandbox for Calcultor {
    type Message = Message;

    fn new() -> Self {
        Self { input: 0, result: 0}
    }
    fn title(&self) -> String {
        String::from("Calcultor")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Plus => {
                self.result += self.input;
                self.input = 0;
            }
            Message::Minus => {
                self.result -= self.input;
                self.input = 0;

            }
            Message::One => {
                self.input = self.input*10 + 1;
            }
            Message::Two => {
                self.input = self.input*10 + 2;
            }
            Message::Three => {
                self.input = self.input*10 + 3;
            }
            Message::Four => {
                self.input = self.input*10 + 4;
            }
            Message::Five => {
                self.input = self.input*10 + 5;
            }
            Message::Six => {
                self.input = self.input*10 + 6;
            }
            Message::Seven => {
                self.input = self.input*10 + 7;
            }
            Message::Eight => {
                self.input = self.input*10 + 8;
            }
            Message::Nine => {
                self.input = self.input*10 + 9;
            }
            Message::Zero => {
                self.input = self.input*10 + 0;
            }
        }
    }

   fn view(&self) -> Element<Message> {
        // We use a column: a simple vertical layout
        // column![
        //     // The increment button. We tell it to produce an
        //     // `IncrementPressed` message when pressed
        //     button("+").on_press(Operations::Plus),
        //
        //     // We show the value of the counter here
        //     text(self.value).size(50),
        //
        //     // The decrement button. We tell it to produce a
        //     // `DecrementPressed` message when pressed
        //     button("-").on_press(Operations::Minus),
        // ].padding(20)
        // .align_items(Alignment::Center)
        // .into();
        //
        Column::new() 
            .push(button("7").on_press(Message::Seven))
            .push(button("4").on_press(Message::Four))
            .push(button("1").on_press(Message::One))
            .push(button("+").on_press(Message::Plus))
            .push(button("-").on_press(Message::Minus))
            .push(text(self.input))
            .push(text(self.result))
            .padding(200)
            .align_items(Alignment::Center)
            .into()
    }

}
