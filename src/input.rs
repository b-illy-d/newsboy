use crate::app::App;
use crate::component::{
    debug::{debug_log, debug_logs_clear, toggle_debug_logs},
    setup,
};
use crate::event::{quit, AppEvent};
use crate::route::{next_route, previous_route, select_route, Route};
use ratatui::crossterm::event::{
    KeyCode::{BackTab, Char, Tab},
    KeyEvent, KeyModifiers,
};
use strum::IntoEnumIterator;

// ======================
// ==== HANDLE INPUT ====
// ======================

pub async fn on_key(state: &App, key: KeyEvent) -> Option<AppEvent> {
    if matches!(key.code, Char('c' | 'd')) && key.modifiers.contains(KeyModifiers::CONTROL) {
        return Some(quit());
    }

    let route_result = match state.route {
        Route::Setup => setup::on_key(&state.setup, key),
        _ => not_handled(),
    };

    if route_result.is_handled() {
        debug_log(format!(
            "  Key {:?} handled route: {:?}",
            key.code, state.route
        ));
        return route_result.into();
    }

    global_on_key(key).await.into()
}

async fn global_on_key(key: KeyEvent) -> InputHandled<AppEvent> {
    match key.code {
        Tab => handled(next_route()),
        BackTab => handled(previous_route()),
        Char(c @ '0'..='9') => on_numeral_key(c),
        Char(';') => toggle_debug_logs(),
        Char('q') => handled(quit()),
        _ => not_handled(),
    }
}

fn on_numeral_key(c: char) -> InputHandled<AppEvent> {
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

pub enum InputHandled<T> {
    Handled(Option<T>),
    NotHandled,
}

impl<T: Clone> InputHandled<T> {
    pub fn is_handled(&self) -> bool {
        matches!(self, InputHandled::Handled(_))
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> InputHandled<U> {
        match self {
            InputHandled::Handled(Some(t)) => InputHandled::Handled(Some(f(t))),
            InputHandled::Handled(None) => InputHandled::Handled(None),
            InputHandled::NotHandled => InputHandled::NotHandled,
        }
    }

    pub fn unwrap(self) -> Option<T> {
        match self {
            InputHandled::Handled(event) => event,
            InputHandled::NotHandled => None,
        }
    }

    pub fn clone(&self) -> InputHandled<T> {
        match self {
            InputHandled::Handled(Some(event)) => InputHandled::Handled(Some(event.clone())),
            InputHandled::Handled(None) => InputHandled::Handled(None),
            InputHandled::NotHandled => InputHandled::NotHandled,
        }
    }
}

pub fn handled<T>(event: T) -> InputHandled<T> {
    InputHandled::Handled(Some(event))
}

pub fn handled_empty<T>() -> InputHandled<T> {
    InputHandled::Handled(None)
}

pub fn not_handled<T>() -> InputHandled<T> {
    InputHandled::NotHandled
}
