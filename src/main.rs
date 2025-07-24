use iced::{Length, widget::Svg};

fn main() -> iced::Result {
    iced::run("Svg experiment", MyApp::update, MyApp::view)
}

#[derive(Debug, Clone)]
enum Message {}

struct MyApp {
    ferris_size: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { ferris_size: 300.0 }
    }
}

impl MyApp {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<Message> {
        Svg::from_path(format!("{}/assets/ferris.svg", env!("CARGO_MANIFEST_DIR")))
            .width(Length::Fixed(self.ferris_size))
            .height(Length::Fixed(self.ferris_size))
            .into()
    }
}
