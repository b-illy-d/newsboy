use crate::component::{
    pubsub::PubsubState, reusable::text_field::TextFields, setup::Setup, topics::TopicsState,
};
use ratatui::{style::Stylize, text::Line};
use std::time::Instant;
use strum_macros::{Display, EnumIter, FromRepr};

pub enum Focus {
    Global,
    TextField(String),
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum Route {
    #[default]
    #[strum(serialize = "Setup")]
    Setup,
    #[strum(serialize = "Topics")]
    Topics,
}

impl Route {
    pub fn title(self) -> Line<'static> {
        Line::from(format!("  {self}  ")).light_blue()
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
        }
    }

    pub fn init(&mut self) {
        let fields = Setup::get_fields_info();
        for (name, label) in fields {
            self.text_fields.add(name, label);
        }
    }
}
