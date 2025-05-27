use crate::app::{App, Focus, Route};
use crate::component::reusable::text_field;
use crate::component::{pubsub, reusable::text_field::TextFieldEvent};
use ratatui::crossterm::event::{KeyCode::Char, KeyEvent, KeyModifiers};
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
    Quit,
}

pub fn quit() -> AppEvent {
    AppEvent::Quit
}

// ==================
// ==== HANDLERS ====
// ==================

pub async fn on_event(app: &mut App, e: AppEvent) -> Option<AppEvent> {
    match e {
        AppEvent::Tick => on_tick(app),
        AppEvent::Input(key) => on_key(app, key).await,
        AppEvent::Pubsub(pubsub_event) => pubsub::on_event(&mut app.pubsub, pubsub_event).await,
        AppEvent::TextField(event) => {
            let field = app.text_fields.get_mut(&event.id);
            text_field::on_event(field, event)
        }
        AppEvent::SelectRoute(route) => {
            app.route = route;
            None
        }
        AppEvent::Quit => on_quit(app),
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

pub async fn on_key(app: &App, key: KeyEvent) -> Option<AppEvent> {
    // Should always work, no matter where focus is
    if key.code == Char('c') {
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            return Some(quit());
        }
    }

    let focused_result = match app.focus {
        Focus::Global => global_on_key(key).await,
        Focus::TextField(ref id) => {
            let field = app
                .text_fields
                .get(id)
                .expect("TextFieldEvent should have a valid id");
            text_field::on_key(field, key)
        }
    };

    match focused_result.is_handled() {
        true => focused_result.into_event(),
        false => match app.focus {
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
        Char(c @ '0'..='9') => on_navigation_key(c),
        Char('q') => handled(quit()),
        _ => not_handled(),
    }
}

fn on_navigation_key(c: char) -> InputHandled {
    if let Some(digit) = c.to_digit(10) {
        if let Some(route) = Route::iter().nth((digit - 1) as usize) {
            return handled(AppEvent::SelectRoute(route));
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
