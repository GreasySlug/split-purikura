use iced::{
    font::{Family, Weight},
    widget::text,
    window::{self, Position},
    Element, Font, Size, Task, Theme,
};

#[derive(Debug, Clone)]
enum Message {}

#[derive(Debug, Clone)]
struct State {}

impl Default for State {
    fn default() -> Self {
        Self {}
    }
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {}
}

fn view(state: &State) -> Element<Message> {
    text("Hello, world!").into()
}

fn set_theme(_: &State) -> Theme {
    Theme::Light
}

fn main() -> iced::Result {
    let window_settings = window::Settings {
        size: Size::new(400.0, 500.0),
        max_size: Some(Size::new(500.0, 600.0)),
        min_size: Some(Size::new(100.0, 100.0)),
        position: Position::Centered,
        ..Default::default()
    };
    let default_font = Font {
        family: Family::Monospace,
        weight: Weight::Normal,
        ..Default::default()
    };

    iced::application("プリプリメーカー", update, view)
        .default_font(default_font)
        .window(window_settings)
        .theme(set_theme)
        .run()
}
