use crate::component::{
    debug::DebugLogs,
    pubsub::{self, Pubsub},
    topics::Topics,
};
use crate::route::Route;
use std::time::Instant;

pub struct App {
    pub route: Route,
    pub last_tick: Instant,
    pub pubsub: Pubsub,
    pub should_quit: bool,
    pub ticks: u64,
    pub topics: Topics,
    pub debug_logs: DebugLogs,
}

impl App {
    pub fn new() -> Self {
        Self {
            route: Route::Config,
            last_tick: Instant::now(),
            pubsub: Pubsub::default(),
            should_quit: false,
            ticks: 0,
            topics: Topics::new(),
            debug_logs: DebugLogs::default(),
        }
    }
}

pub fn init(state: &mut App) {
    pubsub::init_config(&mut state.pubsub.config);
}
