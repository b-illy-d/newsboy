use crate::app::{App, Focus};
use crate::component::{
    pubsub::{self, ProjectIdEvent, PubsubEvent},
    setting_project_id,
};
use crossterm::event::{KeyCode::Char, KeyEvent};

pub enum AppEvent {
    Tick,
    Input(KeyEvent),
    Pubsub(pubsub::PubsubEvent),
    Quit,
}

// Event Factories
pub fn start_setting_project_id() -> AppEvent {
    AppEvent::Pubsub(PubsubEvent::ProjectId(ProjectIdEvent::StartSetting))
}
pub fn input_project_id(input: String) -> AppEvent {
    AppEvent::Pubsub(PubsubEvent::ProjectId(ProjectIdEvent::Input(input)))
}
pub fn finish_setting_project_id(project_id: Option<String>) -> AppEvent {
    AppEvent::Pubsub(PubsubEvent::ProjectId(ProjectIdEvent::FinishSetting(
        project_id,
    )))
}
pub fn quit() -> AppEvent {
    AppEvent::Quit
}

// Event handlers
pub async fn on_event(app: &mut App, e: AppEvent) -> Option<AppEvent> {
    match e {
        AppEvent::Tick => on_tick(app),
        AppEvent::Input(key) => on_key(app, key).await,
        AppEvent::Pubsub(pubsub_event) => pubsub::on_event(&mut app.pubsub, pubsub_event).await,
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

// Input handling
pub enum InputHandled {
    Handled(Option<AppEvent>),
    NotHandled,
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

pub async fn on_key(app: &App, key: KeyEvent) -> Option<AppEvent> {
    let focused_result = match app.focus {
        Focus::Global => global_on_key(key).await,
        Focus::SettingProjectId => {
            setting_project_id::on_key(&app.pubsub.setting_project_id, key).await
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
    let event = match key.code {
        Char('p') => Some(start_setting_project_id()),
        Char('q') => Some(quit()),
        _ => None,
    };
    if let Some(e) = event {
        InputHandled::Handled(Some(e))
    } else {
        InputHandled::NotHandled
    }
}
