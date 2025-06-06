use crate::app::App;
use crate::component::{
    debug::{self, debug_log, DebugLogsEvent},
    pubsub::{self, ConfigEvent, PubsubEvent},
};
use crate::input::{on_key, InputHandled};
use crate::route;
use crate::route::RouteEvent;
use once_cell::sync::OnceCell;
use ratatui::crossterm::event::KeyEvent;
use tokio::sync::mpsc;

pub static TX: OnceCell<mpsc::Sender<AppEvent>> = OnceCell::new();

pub fn send_event(event: AppEvent) -> impl std::future::Future<Output = ()> {
    let tx = TX.get().expect("TX channel not initialized");
    async move {
        if tx.send(event).await.is_err() {
            debug_log("Failed to send event".to_string());
        }
    }
}

// ================
// ==== EVENTS ====
// ================

#[derive(Debug, Clone)]
pub enum AppEvent {
    Tick,
    Input(KeyEvent),
    Pubsub(PubsubEvent),
    Route(RouteEvent),
    Debug(DebugLogsEvent),
    Quit,
}

pub fn quit() -> AppEvent {
    AppEvent::Quit
}

// ==================
// ==== HANDLERS ====
// ==================

pub async fn on_event(state: &mut App, e: AppEvent) {
    if !matches!(e, AppEvent::Tick) {
        debug_log(format!("IN {:?}", e));
    }
    let ret = match e {
        AppEvent::Tick => on_tick(state),
        AppEvent::Input(key) => on_key(state, key).await,
        AppEvent::Route(event) => route::on_event(state, event),
        AppEvent::Pubsub(pubsub_event) => pubsub::on_event(&mut state.pubsub, pubsub_event).await,
        AppEvent::Debug(event) => {
            debug::on_event(&mut state.debug_logs, event);
            None
        }
        AppEvent::Quit => on_quit(state),
    };
    if let Some(ref chain) = ret {
        debug_log(format!("OUT {:?}", chain));
    }
    match ret {
        Some(event) => send_event(event).await,
        None => {}
    }
}

pub fn on_tick(state: &mut App) -> Option<AppEvent> {
    state.ticks += 1;
    state.last_tick = std::time::Instant::now();
    debug::on_tick(state);
    None
}

pub fn on_quit(app: &mut App) -> Option<AppEvent> {
    app.should_quit = true;
    None
}

// =====================
// ==== EVENT UTILS ====
// =====================

impl From<InputHandled<AppEvent>> for Option<AppEvent> {
    fn from(handled: InputHandled<AppEvent>) -> Self {
        match handled {
            InputHandled::Handled(event) => event,
            InputHandled::NotHandled => None,
        }
    }
}

impl From<RouteEvent> for AppEvent {
    fn from(event: RouteEvent) -> Self {
        AppEvent::Route(event)
    }
}

impl From<ConfigEvent> for AppEvent {
    fn from(event: ConfigEvent) -> Self {
        AppEvent::Pubsub(PubsubEvent::Config(event))
    }
}

impl From<DebugLogsEvent> for AppEvent {
    fn from(event: DebugLogsEvent) -> Self {
        AppEvent::Debug(event)
    }
}

impl From<PubsubEvent> for AppEvent {
    fn from(event: PubsubEvent) -> Self {
        AppEvent::Pubsub(event)
    }
}
