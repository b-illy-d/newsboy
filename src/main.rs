use crossterm::event::{poll, read, Event as CEvent};
use std::{error::Error, time::Duration};
use tokio::{sync::mpsc, time};

mod app;
mod component;
mod event;
mod pubsub;
mod view;

use app::App;
use event::{on_key, on_pubsub, on_quit, on_tick, AppEvent};
use view::draw;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();

    let (tx, mut rx) = mpsc::channel::<AppEvent>(128);

    {
        let tx = tx.clone();
        tokio::spawn(async move {
            let mut ticker = time::interval(Duration::from_millis(50));
            loop {
                ticker.tick().await;
                if tx.send(AppEvent::Tick).await.is_err() {
                    break;
                }
            }
        });
    }

    {
        let tx = tx.clone();
        tokio::spawn(async move {
            loop {
                if poll(Duration::from_millis(50)).unwrap() {
                    if let CEvent::Key(k) = read().unwrap() {
                        if tx.send(AppEvent::Input(k)).await.is_err() {
                            break;
                        }
                    }
                }
            }
        });
    }

    let mut app = App::new(tx.clone());

    loop {
        if app.should_quit {
            break;
        }

        terminal.draw(|f| draw(f, &mut app))?;

        match rx.recv().await {
            Some(AppEvent::Tick) => on_tick(&mut app),
            Some(AppEvent::Input(key)) => on_key(key, &mut app).await,
            Some(AppEvent::Pubsub(msg)) => on_pubsub(msg, &mut app).await,
            Some(AppEvent::Quit) => on_quit(&mut app),
            None => {}
        }
    }

    ratatui::restore();
    Ok(())
}
