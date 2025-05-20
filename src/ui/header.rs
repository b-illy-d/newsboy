use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

const LOGO: &str = r#"
| |\ | | |_  \ \    /( (` | |_) / / \ \ \_/ 
|_| \| |_|__  \_\/\/ _)_) |_|_) \_\_/  |_|  
 "#;

pub fn draw_header<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(area);

    draw_logo(f, chunks[0]);
    draw_help(f, app, chunks[1]);
}

fn draw_logo<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let logo_paragraph = Paragraph::new(LOGO)
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Blue));

    f.render_widget(logo_paragraph, area);
}

fn draw_help<B: Backend>(f: &mut Frame<B>, _app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(area);

    // Define the help items (triplets)
    let items = split_vec_into_three(vec![
        ("q", "Quit application"),
        ("c", "Open console to view logs"),
        ("r", "Refresh topics list"),
        ("/", "Toggle filter mode"),
        ("j/↓", "Navigate down"),
        ("k/↑", "Navigate up"),
        ("?", "Show/hide this help"),
        ("ESC", "Exit filter mode or close this help"),
    ]);

    // Render each column
    for (i, col) in chunks.into_iter().enumerate() {
        let block = Block::default().borders(Borders::ALL).title(Span::styled(
            " Keyboard Shortcuts ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ));
        let paragraph = Paragraph::new(col)
            .block(block)
            .wrap(tui::widgets::Wrap { trim: true });
        f.render_widget(paragraph, col);
    }
}

fn split_vec_into_three<T>(vec: Vec<T>) -> (Vec<T>, Vec<T>, Vec<T>) {
    let len = vec.len();
    let chunk_size = (len + 2) / 3;

    let col1 = vec[0..chunk_size].to_vec();
    let col2 = vec[chunk_size..(2 * chunk_size)].to_vec();
    let col3 = vec[(2 * chunk_size)..].to_vec();

    (col1, col2, col3)
}
