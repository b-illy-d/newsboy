use crate::app::App;
use crate::component::{
    debug::{debug_log, toggle_debug_logs},
    reusable::text_field,
    setup, topics,
};
use crate::event::{quit, AppEvent};
use crate::route::{next_route, previous_route, select_route, Route};
use ratatui::crossterm::event::{
    KeyCode::{BackTab, Char, Tab},
    KeyEvent, KeyModifiers,
};
use strum::IntoEnumIterator;

#[derive(Debug)]
pub enum Focus {
    Global,
    TextField(String),
}

pub fn on_focus(state: &mut App, focus: Focus) -> Option<AppEvent> {
    debug_log(format!(
        "Changing focus from {:?} to {:?}",
        state.focus, focus
    ));
    state.focus = focus;
    match state.focus {
        Focus::Global => {}
        Focus::TextField(ref name) => {
            state.text_fields.set_focus(name);
        }
    }
    None
}

pub fn focus_to(focus: Focus) -> AppEvent {
    AppEvent::Focus(focus)
}

// ======================
// ==== HANDLE INPUT ====
// ======================

pub async fn on_key(state: &App, key: KeyEvent) -> Option<AppEvent> {
    if matches!(key.code, Char('c' | 'd')) && key.modifiers.contains(KeyModifiers::CONTROL) {
        return Some(quit());
    }

    let focused_result = match state.focus {
        Focus::Global => global_on_key(key).await,
        Focus::TextField(ref id) => {
            let field = state.text_fields.get(id);
            text_field::on_key(field, key)
        }
    };

    if focused_result.is_handled() {
        debug_log(format!(
            "Key {:?} handled focused: {:?}",
            key.code, state.focus
        ));
        return focused_result.into();
    }

    let route_result = match state.route {
        Route::Setup => setup::on_key(&state.setup, key),
        _ => not_handled(),
    };

    if route_result.is_handled() {
        debug_log(format!(
            "Key {:?} handled route: {:?}",
            key.code, state.route
        ));
        return route_result.into();
    }

    if !matches!(state.focus, Focus::Global) {
        return global_on_key(key).await.into();
    }

    None
}

async fn global_on_key(key: KeyEvent) -> InputHandled {
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
// ==== INPUT UTILS ====
// =====================

pub enum InputHandled {
    Handled(Option<AppEvent>),
    NotHandled,
}

impl InputHandled {
    fn is_handled(&self) -> bool {
        matches!(self, InputHandled::Handled(_))
    }
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
