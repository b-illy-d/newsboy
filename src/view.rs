use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders},
    Frame,
};

use crate::{
    app::{App, Route},
    component::{header, setting_project_id, topics},
};

pub fn draw(f: &mut Frame, app: &App) {
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
    header::draw(f, chunks[0], app);
    draw_main(f, chunks[1], app);
    draw_footer(f, chunks[2], app);
    setting_project_id::draw(f, app);
}

fn draw_main(f: &mut Frame, area: Rect, app: &App) {
    match app.route {
        Route::Topics => {
            topics::draw(f, area, app);
        }
    }
}
fn draw_footer(f: &mut Frame, area: Rect, app: &App) {
    let footer = format!("Ticks {}", app.ticks);
    let footer_paragraph = ratatui::widgets::Paragraph::new(footer)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Yellow));

    f.render_widget(footer_paragraph, area);
}
