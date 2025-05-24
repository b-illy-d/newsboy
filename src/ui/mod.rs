pub mod components;
use crate::ui::header::draw_header;
pub use components::*;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::app::App;

pub trait Component {
    fn init(&mut self, app: &App);
    fn on_key(&mut self, key: &crossterm::event::KeyEvent);
    fn view(&self, f: &mut Frame, area: Rect, app: &App);
}

pub fn draw(f: &mut Frame, area: Rect, app: &App) {
    // Build the UI layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(11), // Header
            Constraint::Min(0),     // Main content
        ])
        .split(area);
    draw_header(f, chunks[0]);
    draw_main_content(f, app, chunks[1]);
}

fn draw_main_content(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // Left panel
            Constraint::Percentage(70), // Right panel
        ])
        .split(area);

    app.topics.view(f, chunks[0], app);
}
