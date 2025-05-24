use crossterm::event::{poll, read, Event as CEvent};
use std::{error::Error, time::Duration};
use tokio::{sync::mpsc, time};

mod app;
mod event;
mod gcp;
mod ui;

use app::App;
use event::Event;
use gcp::Pubsub;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();

    let (tx, mut rx) = mpsc::channel::<Event>(128);

    // tick producer
    {
        let tx = tx.clone();
        tokio::spawn(async move {
            let mut ticker = time::interval(Duration::from_millis(50));
            loop {
                ticker.tick().await;
                if tx.send(Event::Tick).await.is_err() {
                    break;
                }
            }
        });
    }

    // keyboard producer (async, no blocking)
    {
        let tx = tx.clone();
        tokio::spawn(async move {
            loop {
                if poll(Duration::from_millis(50)).unwrap() {
                    if let CEvent::Key(k) = read().unwrap() {
                        if tx.send(Event::Input(k)).await.is_err() {
                            break;
                        }
                    }
                }
            }
        });
    }

    // pub/sub task
    {
        // let tx = tx.clone();
        tokio::spawn(async move {
            let mut pubsub = Pubsub::new();
            pubsub.run();
        });
    }

    // ── app state ──────────────────────────────────────────────────────
    let mut app = App::new(tx);

    // ── main loop ──────────────────────────────────────────────────────
    loop {
        terminal.draw(|f| {
            let area = f.size();
            ui::draw(f, area, &mut app)
        })?;

        match rx.recv().await {
            Some(Event::Tick) => app.on_tick(),
            Some(Event::Input(key)) => app.on_key(&key).await,
            Some(Event::Gcp(msg)) => app.on_pubsub(&msg),
            Some(Event::Quit) => break,
            None => break,
        }
    }

    ratatui::restore();
    Ok(())
}
