use crate::app::App;
use crate::component::topics::TopicInfo;
use crate::pubsub::PubsubClient;
use crossterm::event::KeyCode::Char;
use crossterm::event::KeyEvent;

pub enum AppEvent {
    Tick,
    Input(KeyEvent),
    Pubsub(PubsubEvent),
    Quit,
}

pub enum PubsubEvent {
    SetProjectId(String),
    Topics(Vec<TopicInfo>),
}

pub fn on_tick(app: &mut App) {
    app.ticks += 1;
    app.last_tick = std::time::Instant::now();
}

pub async fn on_key(key: KeyEvent, app: &mut App) {
    match key.code {
        Char('q') => {
            app.sender.send(AppEvent::Quit).await.unwrap();
        }
        _ => {}
    }
}

pub async fn on_pubsub(event: PubsubEvent, app: &mut App) {
    match event {
        PubsubEvent::SetProjectId(project_id) => {
            app.pubsub = Some(PubsubClient::new(project_id).await.unwrap());
        }
        PubsubEvent::Topics(topics) => {
            app.topics.all = topics;
        }
    }
}

pub fn on_quit(app: &mut App) {
    app.should_quit = true;
}
