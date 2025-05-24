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
            Constraint::Length(9), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer
        ])
        .split(area);
    draw_header(f, chunks[0]);
    draw_main_content(f, app, chunks[1]);
    draw_footer(f, app, chunks[2]);
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

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let footer = format!("Ticks {}", app.ticks);
    let footer_paragraph = ratatui::widgets::Paragraph::new(footer)
        .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::NONE))
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::Gray));

    f.render_widget(footer_paragraph, area);
}
