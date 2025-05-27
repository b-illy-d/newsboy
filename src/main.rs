use crossterm::event::{poll, read, Event as CEvent};
use std::{error::Error, time::Duration};
use tokio::{sync::mpsc, time};

mod app;
mod component;
mod event;
mod pubsub;
mod view;

use app::App;
use event::{on_event, Event};
use view::draw;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();

    let (tx, mut rx) = mpsc::channel::<Event>(128);

    // Tick Loop
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

    // Input handling
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

    let mut app = App::new();

    // Event handling
    loop {
        if app.should_quit {
            break;
        }

        terminal.draw(|f| draw(&app, f))?;

        match rx.recv().await {
            Some(e) => match on_event(&mut app, e).await {
                Some(e) => {
                    if tx.send(e).await.is_err() {
                        break;
                    }
                }
                None => {}
            },
            None => {}
        }
    }

    ratatui::restore();
    Ok(())
}
