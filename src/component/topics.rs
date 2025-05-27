use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Stylize,
    style::Style,
    widgets::{Block, List, ListState},
    Frame,
};

pub struct TopicInfo {
    pub name: String,
}

pub struct TopicsState {
    pub all: Vec<TopicInfo>,
    pub visibile: Vec<TopicInfo>,
    list_state: ListState,
}

impl TopicsState {
    pub fn new() -> Self {
        Self {
            all: Vec::new(),
            visibile: Vec::new(),
            list_state: ListState::default(),
        }
    }
}

// pub struct TopicFilterState {
//     pub active: bool,
//     pub text: String,
// }

// impl TopicFilterState {
//     pub fn new() -> Self {
//         Self {
//             active: false,
//             text: String::new(),
//         }
//     }
// }

pub fn draw(state: &App, f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(30), // Left panel
            Constraint::Percentage(70), // Right panel
        ])
        .split(area);
    draw_left_content(f, chunks[0], state);
}

fn draw_left_content(f: &mut ratatui::Frame, area: Rect, app: &App) {
    let list = List::new(app.topics.visibile.iter().map(|topic| topic.name.clone()))
        .block(Block::bordered().title("Topics"));

    f.render_widget(list, area);
}
