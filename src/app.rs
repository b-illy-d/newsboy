use crate::component::topics::TopicsState;
use crate::event::AppEvent;
use crate::pubsub::PubsubClient;
use std::time::Instant;
use tokio::sync::mpsc::Sender;

pub enum Route {
    Topics,
}

pub struct App {
    pub route: Route,
    pub ticks: u64,
    pub last_tick: Instant,
    pub should_quit: bool,
    pub topics: TopicsState,
    pub pubsub: Option<PubsubClient>,
    pub sender: Sender<AppEvent>,
}

impl App {
    pub fn new(tx: Sender<AppEvent>) -> Self {
        Self {
            route: Route::Topics,
            ticks: 0,
            last_tick: Instant::now(),
            should_quit: false,
            topics: TopicsState::new(),
            pubsub: None,
            sender: tx,
        }
    }
}
