use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

const LOGO: &str = r#"
                       ▗▖   
 ▄▄▄▄  ▗▞▀▚▖▄   ▄  ▄▄▄ ▐▌    ▄▄▄  ▄   ▄
 █   █ ▐▛▀▀▘█ ▄ █ ▀▄▄  ▐▛▀▚▖█   █ █   █
 █   █ ▝▚▄▄▖█▄█▄█ ▄▄▄▀ ▐▙▄▞▘▀▄▄▄▀  ▀▀▀█
 ================================ ▄   █
                                   ▀▀▀ 
"#;

pub fn draw(state: &App, f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // Left panel
            Constraint::Percentage(70), // Right panel
        ])
        .split(area);
    draw_logo(f, chunks[0]);
    draw_summary(state, f, chunks[1]);
}

fn draw_logo(f: &mut Frame, area: Rect) {
    let logo_paragraph = Paragraph::new(LOGO)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::LightBlue));

    f.render_widget(logo_paragraph, area);
}

fn draw_summary(state: &App, f: &mut Frame, area: Rect) {
    let project_id_str = match &state.project_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => "Press 'P' to set project".to_string(),
    };
    let summary_lines: Vec<Line> = vec![
        format!(" - Project ID: {}", project_id_str),
        format!(" - Total Topics: {}", state.topics.all.len()),
        format!(" - Visible Topics: {}", state.topics.visibile.len()),
    ]
    .into_iter()
    .map(|s| Line::from(Span::raw(s)))
    .collect();

    let summary_paragraph = Paragraph::new(summary_lines)
        .block(Block::default().borders(Borders::ALL).yellow())
        .style(Style::default().yellow());

    f.render_widget(summary_paragraph, area);
}
