use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, List, ListState},
    Frame,
};

#[derive(Debug, Clone)]
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
    let list = List::new(state.topics.visibile.iter().map(|topic| topic.name.clone())).block(
        Block::bordered()
            .title("Topics")
            .style(Style::default().fg(Color::LightYellow)),
    );

    f.render_widget(list, area);
}
