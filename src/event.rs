use crate::gcp::GcpMsg;
use crossterm::event::KeyEvent;

#[derive(Debug)]
pub enum Event {
    Tick,
    Input(KeyEvent),
    Gcp(GcpMsg),
    Quit,
}
