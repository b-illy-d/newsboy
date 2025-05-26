use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders},
    Frame,
};

use crate::{
    app::{App, Route},
    component::header,
    component::topics,
};

pub fn draw(f: &mut Frame, app: &mut App) {
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
    header::draw(f, app, chunks[0]);
    draw_main(f, app, chunks[1]);
    draw_footer(f, app, chunks[2]);
}

fn draw_main(f: &mut Frame, app: &mut App, area: Rect) {
    match app.route {
        Route::Topics => {
            topics::draw(f, area, app);
        }
    }
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let footer = format!("Ticks {}", app.ticks);
    let footer_paragraph = ratatui::widgets::Paragraph::new(footer)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Yellow));

    f.render_widget(footer_paragraph, area);
}
