use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, Borders, List, ListState},
    Frame,
};

#[derive(Debug, Clone)]
pub struct TopicInfo {
    pub name: String,
}

#[derive(Default)]
pub struct Topics {
    pub all: Vec<TopicInfo>,
    pub visibile: Vec<TopicInfo>,
    list_state: ListState,
}

impl Topics {
    pub fn new() -> Self {
        Self {
            all: Vec::new(),
            visibile: Vec::new(),
            list_state: ListState::default(),
        }
    }

    pub fn set_topics(&mut self, topics: Vec<TopicInfo>) {
        self.all = topics;
        self.visibile = self.all.clone();
        self.list_state.select(None);
    }
}

pub enum TopicsEvent {
    Select(usize),
    Deselect,
    Filter(String),
    ClearFilter,
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

const TITLE: &str = "Topics";
pub fn draw(state: &App, f: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(TITLE.to_string())
        .fg(Color::LightYellow)
        .bg(Color::Black)
        .borders(Borders::ALL);
    f.render_widget(block, area);

    let [list_area, _details_area] = Layout::default()
        .margin(1)
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .areas(area);

    let list = List::new(state.topics.visibile.iter().map(|t| t.name.clone()))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(list, list_area);
}
