use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders},
    Frame,
};

use crate::{
    app::App,
    component::{header, setting_project_id, topics},
};

pub fn draw(state: &App, f: &mut Frame) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(8),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);
    header::draw(state, f, chunks[0]);
    draw_main(state, f, chunks[1]);
    draw_footer(state, f, chunks[2]);
    setting_project_id::draw(&state.pubsub.setting_project_id, f);
}

fn draw_main(state: &App, f: &mut Frame, area: Rect) {
    topics::draw(state, f, area);
}
fn draw_footer(state: &App, f: &mut Frame, area: Rect) {
    let footer = format!("Ticks {}", state.ticks);
    let footer_paragraph = ratatui::widgets::Paragraph::new(footer)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Yellow));

    f.render_widget(footer_paragraph, area);
}
