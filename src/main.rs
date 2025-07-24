use iced::{
    Length,
    widget::{Svg, column, slider},
};

fn main() -> iced::Result {
    iced::run("Svg experiment", MyApp::update, MyApp::view)
}

#[derive(Debug, Clone)]
enum Message {
    UpdateSize(f32),
}

struct MyApp {
    ferris_size: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { ferris_size: 300.0 }
    }
}

impl MyApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::UpdateSize(new_size) => self.ferris_size = new_size,
        }
    }

    fn view(&self) -> iced::Element<Message> {
        column![
            slider(100.0..=600.0, self.ferris_size, Message::UpdateSize),
            Svg::from_path(format!("{}/assets/ferris.svg", env!("CARGO_MANIFEST_DIR")))
                .width(Length::Fixed(self.ferris_size))
                .height(Length::Fixed(self.ferris_size))
        ]
        .into()
    }
}
