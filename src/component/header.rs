use crate::app::App;
use crate::route::Route;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{block::Title, Block, Borders, Paragraph, Tabs},
    Frame,
};

// ==============
// ==== VIEW ====
// ==============

const LOGO: &str = r#"
                       ▗▖   
 ▄▄▄▄  ▗▞▀▚▖▄   ▄  ▄▄▄ ▐▌    ▄▄▄  ▄   ▄
 █   █ ▐▛▀▀▘█ ▄ █ ▀▄▄  ▐▛▀▚▖█   █ █   █
 █   █ ▝▚▄▄▖█▄█▄█ ▄▄▄▀ ▐▙▄▞▘▀▄▄▄▀  ▀▀▀█
 ================================ ▄   █
                                   ▀▀▀ "#;

fn trimmed_logo() -> String {
    let lines: Vec<&str> = LOGO.lines().collect();
    lines[1..lines.len()].join("\n")
}

pub fn draw(state: &App, f: &mut Frame, area: Rect) {
    use Constraint::Percentage;

    let [logo_area, tabs_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Percentage(30), // Left panel
            Percentage(70), // Right panel
        ])
        .areas(area);
    draw_logo(f, logo_area);
    draw_tabs(state, f, tabs_area);
}

fn draw_logo(f: &mut Frame, area: Rect) {
    let logo_paragraph = Paragraph::new(trimmed_logo())
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(logo_paragraph, area);
}

fn draw_tabs(state: &App, f: &mut Frame, area: Rect) {
    use Constraint::{Length, Min};
    let [_, tabs_area] = Layout::vertical([Min(0), Length(3)]).areas(area);

    let titles = Route::titles();
    let highlight_style = (Color::default(), Color::LightBlue);
    let selected_tab_index = state.route as usize;
    let tabs = Tabs::new(titles)
        .highlight_style(highlight_style)
        .select(selected_tab_index)
        .divider("|")
        .block(
            Block::default()
                .light_blue()
                .borders(Borders::ALL)
                .title(Title::from("Press TAB or number keys to navigate"))
                .title_alignment(Alignment::Center),
        );
    f.render_widget(tabs, tabs_area);
}
