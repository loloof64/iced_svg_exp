use iced::widget::Svg;

fn main() -> iced::Result {
    iced::run("Svg experiment", MyApp::update, MyApp::view)
}

#[derive(Debug, Clone)]
enum Message {}

#[derive(Default)]
struct MyApp {}

impl MyApp {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<Message> {
        Svg::from_path(format!("{}/assets/ferris.svg", env!("CARGO_MANIFEST_DIR")))
            .width(100)
            .height(100)
            .into()
    }
}
