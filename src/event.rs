use crate::app::{App, Focus};
use crate::component::setting_project_id;
use crate::pubsub::{self, ProjectIdEvent, PubsubEvent};
use crossterm::event::{KeyCode::Char, KeyEvent};

pub enum Event {
    Tick,
    Input(KeyEvent),
    Pubsub(pubsub::PubsubEvent),
    Quit,
}

struct AppEvents {
    receiver: tokio::sync::mpsc::Receiver<Event>,
    sender: tokio::sync::mpsc::Sender<Event>,
}

// Event Factories
pub fn start_setting_project_id() -> Event {
    Event::Pubsub(PubsubEvent::ProjectId(ProjectIdEvent::StartSetting))
}
pub fn input_project_id(input: String) -> Event {
    Event::Pubsub(PubsubEvent::ProjectId(ProjectIdEvent::Input(input)))
}
pub fn finish_setting_project_id(project_id: Option<String>) -> Event {
    Event::Pubsub(PubsubEvent::ProjectId(ProjectIdEvent::FinishSetting(
        project_id,
    )))
}
pub fn quit() -> Event {
    Event::Quit
}

// Event handlers
pub async fn on_event(app: &mut App, event: Event) -> Option<Event> {
    match event {
        Event::Tick => on_tick(app),
        Event::Input(key) => on_key(app, key).await,
        Event::Pubsub(pubsub_event) => pubsub::on_event(&mut app.pubsub, pubsub_event).await,
        Event::Quit => on_quit(app),
    }
}

pub fn on_tick(app: &mut App) -> Option<Event> {
    app.ticks += 1;
    app.last_tick = std::time::Instant::now();
    None
}

pub fn on_quit(app: &mut App) -> Option<Event> {
    app.should_quit = true;
    None
}

// Input handling
pub enum InputHandled {
    Handled(Option<Event>),
    NotHandled,
}

impl InputHandled {
    pub fn is_handled(&self) -> bool {
        matches!(self, InputHandled::Handled(_))
    }

    pub fn into_msg(self) -> Option<Event> {
        match self {
            InputHandled::Handled(msg) => msg,
            InputHandled::NotHandled => None,
        }
    }
}

pub async fn on_key(app: &App, key: KeyEvent) -> Option<Event> {
    let focused_result = match app.focus {
        Focus::Global => global_on_key(key).await,
        Focus::SettingProjectId => {
            setting_project_id::on_key(&app.pubsub.setting_project_id, key).await
        }
    };

    match focused_result.is_handled() {
        true => focused_result.into_msg(),
        false => match app.focus {
            Focus::Global => None,
            _ => {
                // If not handled by focused component, we can still handle it globally
                global_on_key(key).await.into_msg()
            }
        },
    }
}

async fn global_on_key(key: KeyEvent) -> InputHandled {
    let message = match key.code {
        Char('p') => Some(start_setting_project_id()),
        Char('q') => Some(quit()),
        _ => None,
    };
    if let Some(msg) = message {
        InputHandled::Handled(Some(msg))
    } else {
        InputHandled::NotHandled
    }
}
