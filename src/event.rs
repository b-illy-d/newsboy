use crate::app::App;
use crate::component::debug::debug_log;
use crate::component::{
    debug::{self, DebugLogsEvent},
    pubsub::{self, PubsubEvent},
    reusable::text_field::{self, TextFieldEvent},
    setup::{self, SetupEvent},
};
use crate::input::{on_focus, on_key, Focus, InputHandled};
use crate::route;
use crate::route::RouteEvent;
use ratatui::crossterm::event::KeyEvent;

// ================
// ==== EVENTS ====
// ================

#[derive(Debug)]
pub enum AppEvent {
    Tick,
    Input(KeyEvent),
    Pubsub(PubsubEvent),
    Route(RouteEvent),
    TextField(TextFieldEvent),
    Debug(DebugLogsEvent),
    Setup(SetupEvent),
    Focus(Focus),
    Quit,
}

pub fn quit() -> AppEvent {
    AppEvent::Quit
}

// ==================
// ==== HANDLERS ====
// ==================

pub async fn on_event(state: &mut App, e: AppEvent) -> Option<AppEvent> {
    if !matches!(e, AppEvent::Tick) {
        debug_log(format!("{:?}", e));
        debug_log(format!("Focus: {:?}", state.focus));
    }
    match e {
        AppEvent::Tick => on_tick(state),
        AppEvent::Input(key) => on_key(state, key).await,
        AppEvent::Focus(focus) => on_focus(state, focus),
        AppEvent::Route(event) => route::on_event(state, event),
        AppEvent::Pubsub(pubsub_event) => pubsub::on_event(&mut state.pubsub, pubsub_event).await,
        AppEvent::TextField(event) => {
            let field = state.text_fields.get_mut(&event.name);
            text_field::on_event(field, event)
        }
        AppEvent::Debug(event) => {
            debug::on_event(&mut state.debug_logs, event);
            None
        }
        AppEvent::Setup(event) => {
            setup::on_event(&mut state.setup, event);
            None
        }
        AppEvent::Quit => on_quit(state),
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

impl From<InputHandled> for Option<AppEvent> {
    fn from(handled: InputHandled) -> Self {
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

impl From<TextFieldEvent> for AppEvent {
    fn from(event: TextFieldEvent) -> Self {
        AppEvent::TextField(event)
    }
}
