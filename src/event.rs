use crate::app::{App, Focus, Route};
use crate::component::{
    debug::{self, toggle_debug_logs},
    pubsub,
    reusable::text_field::{self, TextFieldEvent},
};
use ratatui::crossterm::event::{
    KeyCode::{Char, Tab},
    KeyEvent, KeyModifiers,
};
use strum::IntoEnumIterator;

// ================
// ==== EVENTS ====
// ================

pub enum AppEvent {
    Tick,
    Input(KeyEvent),
    Pubsub(pubsub::PubsubEvent),
    TextField(TextFieldEvent),
    SelectRoute(Route),
    NextRoute,
    PrevRoute,
    Debug(DebugLogsEvent),
    Quit,
}

pub fn quit() -> AppEvent {
    AppEvent::Quit
}

// ==================
// ==== HANDLERS ====
// ==================

pub async fn on_event(state: &mut App, e: AppEvent) -> Option<AppEvent> {
    match e {
        AppEvent::Tick => on_tick(state),
        AppEvent::Input(key) => on_key(state, key).await,
        AppEvent::Pubsub(pubsub_event) => pubsub::on_event(&mut state.pubsub, pubsub_event).await,
        AppEvent::TextField(event) => {
            let field = state.text_fields.get_mut(&event.id);
            text_field::on_event(field, event)
        }
        AppEvent::SelectRoute(route) => {
            state.route = route;
            None
        }
        AppEvent::NextRoute => {
            state.route = state.route.next();
            None
        }
        AppEvent::PrevRoute => {
            state.route = state.route.previous();
            None
        }
        AppEvent::Debug(event) => {
            debug::on_event(state.debug_logs, event);
            None
        }
        AppEvent::Quit => on_quit(state),
    }
}

pub fn on_tick(app: &mut App) -> Option<AppEvent> {
    app.ticks += 1;
    app.last_tick = std::time::Instant::now();
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
    match key.code {
        Tab => on_tab_key(key),
        Char(c @ '0'..='9') => on_numeral_key(c),
        Char(':') => toggle_debug_logs(),
        Char('q') => handled(quit()),
        _ => not_handled(),
    }
}

fn on_numeral_key(c: char) -> InputHandled {
    if let Some(digit) = c.to_digit(10) {
        if let Some(route) = Route::iter().nth((digit - 1) as usize) {
            return handled(AppEvent::SelectRoute(route));
        }
    }
    not_handled()
}

fn on_tab_key(key: KeyEvent) -> InputHandled {
    if key.modifiers.contains(KeyModifiers::SHIFT) {
        println!("Prev route triggered");
        handled(AppEvent::PrevRoute)
    } else {
        handled(AppEvent::NextRoute)
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
