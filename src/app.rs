use crate::component::{
    debug::DebugLogs, pubsub::PubsubState, reusable::text_field::TextFields, setup::Setup,
    topics::TopicsState,
};
use crate::route::Route;
use std::time::Instant;

#[derive(Debug)]
pub enum Focus {
    Global,
    TextField(String),
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
}

pub fn init(state: &mut App) {
    // Initialize the setup fields
    let fields = Setup::get_fields_info();
    for (name, label) in fields {
        state
            .text_fields
            .add(name, label, Some(state.setup.get(name).to_string()));
    }
}
