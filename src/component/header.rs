use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
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

pub fn draw(f: &mut Frame, _app: &App, area: Rect) {
    let logo_paragraph = Paragraph::new(LOGO)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::LightBlue));

    f.render_widget(logo_paragraph, area);
}
