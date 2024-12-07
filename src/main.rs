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
    Paperrotate(bool),
    ConfigTheme(Theme),
    Submit,
}

#[derive(Debug, Clone)]
struct State {
    input_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
    image_config: image_processor::ImageConfig,
    paper_size: combo_box::State<image_processor::PaperSize>,
    selected_paper_size: image_processor::PaperSize,
    is_config: bool,
    themes: combo_box::State<Theme>,
    selecte_theme: Theme,
}

impl Default for State {
    fn default() -> Self {
        Self {
            input_path: None,
            output_path: None,
            image_config: image_processor::ImageConfig::default(),
            paper_size: combo_box::State::new(image_processor::PaperSize::vec_new()),
            selected_paper_size: image_processor::PaperSize::A4,
            is_config: false,
            themes: combo_box::State::new(vec![
                Theme::KanagawaDragon,
                Theme::KanagawaLotus,
                Theme::KanagawaWave,
                Theme::Dark,
                Theme::Light,
                Theme::CatppuccinFrappe,
                Theme::CatppuccinLatte,
                Theme::CatppuccinMocha,
                Theme::GruvboxDark,
                Theme::GruvboxLight,
            ]),
            selecte_theme: Theme::CatppuccinFrappe,
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
            if let Some(path) =
                image_processor::process_image(image_config, selected_paper_size, input_path)
            {
                state.output_path = Some(path);
            }
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
        Message::Paperrotate(is_rotate) => {
            state.image_config.is_rotate = is_rotate;
        }
        Message::ConfigTheme(theme) => {
            state.selecte_theme = theme;
        }
    }
    Task::none()
}

fn view(state: &State) -> Element<Message> {
    let area = if let Some(input_path) = state.input_path.as_ref() {
        container(image(input_path))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(container::rounded_box)
            .width(Length::Fill)
            .height(Length::Fill)
    } else {
        container("画像をドラック & ドロップしてください")
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(container::rounded_box)
            .width(Length::Fill)
            .height(Length::Fill)
    };
    let submit_button = button("プリを作成").on_press(Message::Submit);
    let is_config = toggler(state.is_config)
        .label("設定を変更")
        .on_toggle(Message::Config);
    let column = column![
        row![submit_button, is_config]
            .spacing(10)
            .align_y(Alignment::Center),
        area
    ]
    .align_x(Alignment::Center)
    .padding(10)
    .spacing(10);
    let column = if state.is_config {
        column.push(config_view(state))
    } else {
        column
    };
    column.into()
}

macro_rules! slider_view {
    ($label:expr, $min_value:expr, $max_value:expr, $step:expr, $value:expr, $message:expr) => {{
        let l = text($label).width(Length::Fill);
        let s = slider($min_value..=$max_value, $value, $message).step($step);
        let v = text($value.to_string()).width(Length::Fill);
        row![l, s, v].spacing(10)
    }};
}

fn config_view(state: &State) -> Element<Message> {
    let dpi = slider_view!(
        "DPI",
        0.0,
        1500.0,
        10.0,
        state.image_config.dpi,
        Message::Dpi
    );
    let width_mm = slider_view!(
        "幅(mm)",
        0.0,
        100.0,
        1.0,
        state.image_config.width_mm,
        Message::Width
    );
    let height_mm = slider_view!(
        "高さ(mm)",
        0.0,
        100.0,
        1.0,
        state.image_config.height_mm,
        Message::Height
    );
    let diff_mm = slider_view!(
        "余白(mm)",
        0.0,
        10.0,
        0.1,
        state.image_config.diff_mm,
        Message::Diff
    );
    let rows = slider_view!("行数", 1, 20, 1u32, state.image_config.rows, Message::Row);
    let cols = slider_view!("列数", 1, 20, 1u32, state.image_config.cols, Message::Col);
    let paper_size_desc = text("紙のサイズを指定してください");
    let paper_size = combo_box(
        &state.paper_size,
        "",
        Some(&state.selected_paper_size),
        Message::PaperSize,
    )
    .width(Length::Fill);
    let paper_config = row![paper_size_desc, paper_size]
        .align_y(Alignment::Center)
        .spacing(10);
    let rotate_desc = text("紙を回転");
    let rotate = toggler(state.image_config.is_rotate).on_toggle(Message::Paperrotate);
    let rotate_config = row![rotate_desc, rotate]
        .align_y(Alignment::Center)
        .spacing(10);
    let aspect_desc = text("アスペクト比を固定");
    let aspect_ratio = toggler(state.image_config.is_aspect_ratio).on_toggle(Message::AspectRatio);
    let aspect_config = row![aspect_desc, aspect_ratio]
        .align_y(Alignment::Center)
        .spacing(10);
    let theme_desc = text("テーマを変更");
    let theme = combo_box(
        &state.themes,
        "",
        Some(&state.selecte_theme),
        Message::ConfigTheme,
    );
    let theme_config = row![theme_desc, theme]
        .align_y(Alignment::Center)
        .spacing(10);
    column![
        dpi,
        width_mm,
        height_mm,
        diff_mm,
        rows,
        cols,
        paper_config,
        rotate_config,
        aspect_config,
        theme_config
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

fn set_theme(state: &State) -> Theme {
    state.selecte_theme.clone()
}

fn main() -> iced::Result {
    let window_settings = window::Settings {
        size: Size::new(400.0, 500.0),
        max_size: Some(Size::new(500.0, 800.0)),
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
