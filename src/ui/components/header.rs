use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

const LOGO: &str = r#"
 ▄▄▄▄  ▗▞▀▚▖▄   ▄  ▄▄▄ ▗▖    ▄▄▄  ▄   ▄
 █   █ ▐▛▀▀▘█ ▄ █ ▀▄▄  ▐▌   █   █ █   █
 █   █ ▝▚▄▄▖█▄█▄█ ▄▄▄▀ ▐▛▀▚▖▀▄▄▄▀  ▀▀▀█
                       ▐▙▄▞▘      ▄   █
                                   ▀▀▀ 
"#;

pub fn draw_header(f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(40)])
        .split(area);

    draw_logo(f, chunks[0]);
}

fn draw_logo(f: &mut Frame, area: Rect) {
    let logo_paragraph = Paragraph::new(LOGO)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::LightBlue));

    f.render_widget(logo_paragraph, area);
}
