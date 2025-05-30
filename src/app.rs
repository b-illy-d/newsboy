use crate::component::{
    debug::DebugLogs,
    pubsub::PubsubState,
    setup::{self, Setup},
    topics::TopicsState,
};
use crate::route::Route;
use std::time::Instant;

pub struct App {
    pub route: Route,
    pub last_tick: Instant,
    pub pubsub: PubsubState,
    pub should_quit: bool,
    pub ticks: u64,
    pub topics: TopicsState,
    pub setup: Setup,
    pub debug_logs: DebugLogs,
    pub help_text: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            route: Route::Setup,
            last_tick: Instant::now(),
            pubsub: PubsubState::default(),
            should_quit: false,
            ticks: 0,
            topics: TopicsState::new(),
            setup: Setup::default(),
            debug_logs: DebugLogs::default(),
            help_text: String::new(),
        }
    }
}

pub fn init(state: &mut App) {
    setup::init(&mut state.setup);
}
