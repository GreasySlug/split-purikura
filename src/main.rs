use iced::{
    event::{self, Status},
    font::{Family, Weight},
    widget::{container, image as iced_image, text},
    window::{self, Position},
    Element, Event, Font, Length, Size, Subscription, Task, Theme,
};
use std::path::PathBuf;

#[derive(Debug, Clone)]
enum Message {
    InputPath(PathBuf),
}

#[derive(Debug, Clone)]
struct State {
    input_path: Option<PathBuf>,
}

impl Default for State {
    fn default() -> Self {
        Self { input_path: None }
    }
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::InputPath(path) => {
            state.input_path = Some(path);
        }
    }
    Task::none()
}

fn view(state: &State) -> Element<Message> {
    let area = if let Some(input_path) = state.input_path.as_ref() {
        container(iced_image(input_path))
            .width(Length::Fill)
            .height(Length::Fill)
    } else {
        container("画像をドラック & ドロップしてください")
            .style(container::rounded_box)
            .width(Length::Fill)
            .height(Length::Fill)
    };

    area.into()
}

fn mouse_event_handling(_: &State) -> Subscription<Message> {
    event::listen_with(|event, status, window| match (event, status, window) {
        (
            Event::Window(window::Event::FileHovered(path))
            | Event::Window(window::Event::FileDropped(path)),
            Status::Ignored,
            _,
        ) => Some(Message::InputPath(path)),
        _ => None,
    })
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
        .subscription(mouse_event_handling)
        .run()
}
