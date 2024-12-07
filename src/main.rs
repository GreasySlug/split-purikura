mod image_processor;

use iced::{
    event::{self, Status},
    font::{Family, Weight},
    widget::{button, column, combo_box, container, image, row, slider, text, toggler},
    window::{self, Position},
    Alignment, Element, Event, Font, Length, Size, Subscription, Task, Theme,
};
use std::path::PathBuf;

#[derive(Debug, Clone)]
enum Message {
    InputPath(PathBuf),
    Config(bool),
    Width(f64),
    Height(f64),
    Dpi(f64),
    Diff(f64),
    Col(u32),
    Row(u32),
    AspectRatio(bool),
    PaperSize(image_processor::PaperSize),
    Submit,
}

#[derive(Debug, Clone)]
struct State {
    input_path: Option<PathBuf>,
    image_config: image_processor::ImageConfig,
    paper_size: combo_box::State<image_processor::PaperSize>,
    selected_paper_size: image_processor::PaperSize,
    is_config: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            input_path: None,
            image_config: image_processor::ImageConfig::default(),
            paper_size: combo_box::State::new(image_processor::PaperSize::vec_new()),
            selected_paper_size: image_processor::PaperSize::A4,
            is_config: false,
        }
    }
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Config(is_config) => {
            state.is_config = is_config;
        }
        Message::InputPath(path) => {
            state.input_path = Some(path);
        }
        Message::Submit => {
            let Some(input_path) = state.input_path.as_ref() else {
                return Task::none();
            };
            let image_config = &state.image_config;
            let selected_paper_size = &state.selected_paper_size;
            let Ok(img) =
                image_processor::process_image(image_config, selected_paper_size, input_path)
            else {
                return Task::none();
            };
            let output_path = input_path.parent().unwrap().join("output.png");
            img.save(output_path).unwrap();
            state.input_path = None;
            return Task::none();
        }
        Message::Width(width) => {
            state.image_config.width_mm = width;
        }
        Message::Height(height) => {
            state.image_config.height_mm = height;
        }
        Message::Dpi(dpi) => {
            state.image_config.dpi = dpi;
        }
        Message::Diff(diff) => {
            state.image_config.diff_mm = diff;
        }
        Message::Col(col) => {
            state.image_config.cols = col;
        }
        Message::Row(row) => {
            state.image_config.rows = row;
        }
        Message::AspectRatio(is_aspect_ratio) => {
            state.image_config.is_aspect_ratio = is_aspect_ratio;
        }
        Message::PaperSize(paper_size) => {
            state.selected_paper_size = paper_size;
        }
    }
    Task::none()
}

fn view(state: &State) -> Element<Message> {
    let area = if let Some(input_path) = state.input_path.as_ref() {
        container(image(input_path))
            .width(Length::Fill)
            .height(Length::Fill)
    } else {
        container("画像をドラック & ドロップしてください")
            .style(container::rounded_box)
            .width(Length::Fill)
            .height(Length::Fill)
    };
    let submit_button = button("プリを作成").on_press(Message::Submit);
    let is_config = toggler(state.is_config)
        .label("設定を変更")
        .on_toggle(Message::Config);
    let column = column![row![submit_button, is_config], area]
        .align_x(Alignment::Center)
        .spacing(10);
    let column = if state.is_config {
        column.push(config_view(state))
    } else {
        column
    };
    column.into()
}

macro_rules! slider_view {
    ($label:expr, $min_value:expr, $max_value:expr, $value:expr, $message:expr) => {{
        let label = text($label).width(Length::Fill);
        let slider = slider($min_value..=$max_value, $value, $message);
        let value = text($value.to_string()).width(Length::Fill);
        row![label, slider, value].spacing(10)
    }};
}

fn config_view(state: &State) -> Element<Message> {
    let dpi = slider_view!(
        "解像度(DPI)",
        0.0,
        1500.0,
        state.image_config.dpi,
        Message::Dpi
    );
    let width_mm = slider_view!(
        "幅(mm)",
        0.0,
        100.0,
        state.image_config.width_mm,
        Message::Width
    );
    let height_mm = slider_view!(
        "高さ(mm)",
        0.0,
        100.0,
        state.image_config.height_mm,
        Message::Height
    );
    let diff_mm = slider_view!(
        "余白(mm)",
        0.0,
        10.0,
        state.image_config.diff_mm,
        Message::Diff
    );
    let rows = slider_view!("行数", 0, 50, state.image_config.rows, Message::Row);
    let cols = slider_view!("列数", 0, 50, state.image_config.cols, Message::Col);
    let paper_size = combo_box(
        &state.paper_size,
        "紙のサイズを指定してください",
        Some(&state.selected_paper_size),
        Message::PaperSize,
    )
    .width(Length::Fill);
    let aspect_ratio = toggler(state.image_config.is_aspect_ratio)
        .label("アスペクト比を固定")
        .on_toggle(Message::AspectRatio);
    column![
        dpi,
        width_mm,
        height_mm,
        diff_mm,
        rows,
        cols,
        row![aspect_ratio, paper_size]
            .align_y(Alignment::Center)
            .spacing(10)
    ]
    .spacing(10)
    .padding(10)
    .into()
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
    Theme::CatppuccinFrappe
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
