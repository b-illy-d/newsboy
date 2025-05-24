pub mod components;
use crate::ui::header::draw_header;
pub use components::*;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders},
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
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(30), // Left panel
            Constraint::Percentage(70), // Right panel
        ])
        .split(area);
    draw_left_content(f, app, chunks[0]);
    draw_right_content(f, app, chunks[1]);
}

fn draw_left_content(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7), // Header
            Constraint::Min(0),    // Main content
        ])
        .split(area);

    draw_header(f, chunks[0]);
    app.topics.view(f, chunks[1], app);
}

fn draw_right_content(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer
        ])
        .split(area);

    app.subscriptions.view(f, chunks[0], app);
    draw_footer(f, app, chunks[1]);
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let footer = format!("Ticks {}", app.ticks);
    let footer_paragraph = ratatui::widgets::Paragraph::new(footer)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Yellow));

    f.render_widget(footer_paragraph, area);
}
