use crate::component::{pubsub::Pubsub, topics::TopicsState};
use std::time::Instant;

pub enum Focus {
    Global,
    SettingProjectId,
}

pub struct App {
    pub focus: Focus,
    pub last_tick: Instant,
    pub project_id: Option<String>,
    pub pubsub: Pubsub,
    pub should_quit: bool,
    pub ticks: u64,
    pub topics: TopicsState,
}

impl App {
    pub fn new() -> Self {
        Self {
            focus: Focus::Global,
            last_tick: Instant::now(),
            project_id: None,
            pubsub: Pubsub::default(),
            should_quit: false,
            ticks: 0,
            topics: TopicsState::new(),
        }
    }
}
