use crate::app::App;
use crate::component::debug::debug_log;
use crate::component::{
    debug::{self, DebugLogsEvent},
    pubsub::{self, PubsubEvent},
    setup::{self, SetupEvent},
};
use crate::input::{on_key, InputHandled};
use crate::route;
use crate::route::RouteEvent;
use ratatui::crossterm::event::KeyEvent;

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
    Setup(SetupEvent),
    HelpText(String),
    Quit,
}

pub fn quit() -> AppEvent {
    AppEvent::Quit
}

pub fn status_text(text: &str) -> AppEvent {
    AppEvent::HelpText(text.to_string())
}

// ==================
// ==== HANDLERS ====
// ==================

pub async fn on_event(state: &mut App, e: AppEvent) -> Option<AppEvent> {
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
        AppEvent::Setup(event) => setup::on_event(&mut state.setup, event),
        AppEvent::HelpText(text) => {
            state.help_text = text;
            None
        }
        AppEvent::Quit => on_quit(state),
    };
    if let Some(ref chain) = ret {
        debug_log(format!("OUT {:?}", chain));
    }
    ret
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

impl From<SetupEvent> for AppEvent {
    fn from(event: SetupEvent) -> Self {
        AppEvent::Setup(event)
    }
}

impl From<DebugLogsEvent> for AppEvent {
    fn from(event: DebugLogsEvent) -> Self {
        AppEvent::Debug(event)
    }
}
