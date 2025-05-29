use crate::app::{App, Focus};
use crate::component::debug::debug_log;
use crate::component::{
    debug::{self, toggle_debug_logs, DebugLogsEvent},
    pubsub,
    reusable::text_field::{self, TextFieldEvent},
    setup::{self, SetupEvent},
};
use crate::route;
use crate::route::{next_route, previous_route, select_route, Route, RouteEvent};
use ratatui::crossterm::event::{
    KeyCode::{BackTab, Char, Tab},
    KeyEvent, KeyModifiers,
};
use strum::IntoEnumIterator;

// ================
// ==== EVENTS ====
// ================

#[derive(Debug)]
pub enum AppEvent {
    Tick,
    Input(KeyEvent),
    Pubsub(pubsub::PubsubEvent),
    Route(RouteEvent),
    TextField(TextFieldEvent),
    Debug(DebugLogsEvent),
    Setup(SetupEvent),
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
        debug_log(format!("Handling event: {:?}", e));
    }
    match e {
        AppEvent::Tick => on_tick(state),
        AppEvent::Input(key) => on_key(state, key).await,
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

// ======================
// ==== HANDLE INPUT ====
// ======================

pub async fn on_key(state: &App, key: KeyEvent) -> Option<AppEvent> {
    // Should always work, no matter where focus is
    if key.code == Char('c') || key.code == Char('d') {
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            return Some(quit());
        }
    }

    debug_log(format!("App focus: {:?}", state.focus));
    let focused_result = match state.focus {
        Focus::Global => global_on_key(key).await,
        Focus::TextField(ref id) => {
            let field = state
                .text_fields
                .get(id)
                .expect("TextFieldEvent should have a valid id");
            text_field::on_key(field, key)
        }
    };

    match focused_result.is_handled() {
        true => focused_result.into_event(),
        false => match state.focus {
            Focus::Global => None,
            _ => {
                // If not handled by focused component, we can still handle it globally
                global_on_key(key).await.into_event()
            }
        },
    }
}

async fn global_on_key(key: KeyEvent) -> InputHandled {
    debug_log(format!(
        "Global key event: code {:?}, modifiers {:?}",
        key.code, key.modifiers
    ));
    match key.code {
        Tab => handled(next_route()),
        BackTab => handled(previous_route()),
        Char(c @ '0'..='9') => on_numeral_key(c),
        Char(';') => toggle_debug_logs(),
        Char('q') => handled(quit()),
        _ => not_handled(),
    }
}

fn on_numeral_key(c: char) -> InputHandled {
    if let Some(digit) = c.to_digit(10) {
        if let Some(route) = Route::iter().nth((digit - 1) as usize) {
            return handled(select_route(route));
        }
    }
    not_handled()
}

// =====================
// ==== EVENT UTILS ====
// =====================

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

// =====================
// ==== INPUT UTILS ====
// =====================

pub enum InputHandled {
    Handled(Option<AppEvent>),
    NotHandled,
}

pub fn handled(event: AppEvent) -> InputHandled {
    InputHandled::Handled(Some(event))
}

pub fn handled_empty() -> InputHandled {
    InputHandled::Handled(None)
}

pub fn not_handled() -> InputHandled {
    InputHandled::NotHandled
}

impl InputHandled {
    pub fn is_handled(&self) -> bool {
        matches!(self, InputHandled::Handled(_))
    }

    pub fn into_event(self) -> Option<AppEvent> {
        match self {
            InputHandled::Handled(e) => e,
            InputHandled::NotHandled => None,
        }
    }
}
