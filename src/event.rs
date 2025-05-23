use crossterm::event::KeyEvent;

#[derive(Debug)]
pub enum Event {
    Tick,
    Input(KeyEvent),
}
