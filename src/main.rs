use std::{
    error::Error,
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    event::{DisableMouseCapture, Event as CEvent, KeyCode, poll, read},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::{sync::mpsc, time};

mod app;
mod event;
mod tui;
mod gcp;

use app::App;
use event::Event;
use tui::{init, restore};
use gcp::PubSubClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = tui::init()?;

    // ── event bus ──────────────────────────────────────────────────────
    let (tx, mut rx) = mpsc::channel::<Event>(128);

    // tick producer
    {
        let tx = tx.clone();
        tokio::spawn(async move {
            let mut ticker = time::interval(Duration::from_millis(1000));
            loop {
                ticker.tick().await;
                if tx.send(Event::Tick).await.is_err() {
                    break;
                }
            }
        });
    }

    // keyboard producer (blocking, in a std thread)
    {
        let tx = tx.clone();
        std::thread::spawn(move || {
            while let Ok(true) = poll(Duration::from_millis(50)) {
                if let CEvent::Key(k) = read().unwrap() {
                    futures::executor::block_on(tx.send(Event::Input(k))).ok();
                }
            }
        });
    }

    // pub/sub task
    {
        let tx = tx.clone();
        tokio::spawn(async move {
            let mut client = PubSubClient::new(None).await.unwrap();
            client.run(tx).await;
        });
    }

    // ── app state ──────────────────────────────────────────────────────
    let mut app = App::new(tx);

    // ── main loop ──────────────────────────────────────────────────────
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        match rx.recv().await {
            Some(Event::Tick) => app.on_tick().await,
            Some(Event::Input(key)) => {
                if matches!(key.code, KeyCode::Char('q')) {
                    break;
                }
                app.on_key(key).await;
            }
            Some(Event::Gcp(msg)) => app.on_pubsub(msg).await,
            None => break,
        }
    }

    tui::restore(terminal)
    Ok(())
}
