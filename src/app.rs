use crate::component::{
    debug::{debug_log, DebugLogs},
    pubsub::PubsubState,
    reusable::text_field::TextFields,
    setup::Setup,
    topics::TopicsState,
};
use crate::event::AppEvent;
use ratatui::{style::Stylize, text::Line};
use std::time::Instant;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, FromRepr};
use tokio::sync::mpsc::Sender;

pub enum Focus {
    Global,
    TextField(String),
}

#[derive(Debug, Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum Route {
    #[default]
    #[strum(serialize = "Setup")]
    Setup,
    #[strum(serialize = "Topics")]
    Topics,
}

impl Route {
    pub fn titles() -> Vec<Line<'static>> {
        Route::iter()
            .enumerate()
            .map(|(i, r)| Line::from(format!("[{}] {}", i + 1, r)).light_blue())
            .collect()
    }

    pub fn next(self) -> Self {
        let index = (self as usize + 1) % Self::iter().len();
        Self::from_repr(index).unwrap()
    }

    pub fn previous(self) -> Self {
        let count = Self::iter().len();
        let index = (self as usize + count - 1) % count;
        let prev = Self::from_repr(index).unwrap();
        debug_log(format!("Previous route: {:?}", prev));
        prev
    }
}

pub struct App {
    pub route: Route,
    pub focus: Focus,
    pub last_tick: Instant,
    pub pubsub: PubsubState,
    pub should_quit: bool,
    pub ticks: u64,
    pub topics: TopicsState,
    pub text_fields: TextFields,
    pub setup: Setup,
    pub debug_logs: DebugLogs,
}

impl App {
    pub fn new() -> Self {
        Self {
            route: Route::Setup,
            focus: Focus::Global,
            last_tick: Instant::now(),
            pubsub: PubsubState::default(),
            should_quit: false,
            ticks: 0,
            topics: TopicsState::new(),
            text_fields: TextFields::new(),
            setup: Setup::default(),
            debug_logs: DebugLogs::default(),
        }
    }

    pub fn init(&mut self) {
        // Initialize the setup fields
        let fields = Setup::get_fields_info();
        for (name, label) in fields {
            self.text_fields
                .add(name, label, Some(self.setup.get(name).to_string()));
        }
    }
}
