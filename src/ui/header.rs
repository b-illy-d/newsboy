use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
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

pub fn draw_header<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(40), Constraint::Min(0)])
        .split(area);

    draw_logo(f, chunks[0]);
    draw_help(f, chunks[1]);
}

fn draw_logo<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let logo_paragraph = Paragraph::new(LOGO)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::LightBlue));

    f.render_widget(logo_paragraph, area);
}

fn draw_help<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let items = split_vec_into_n(
        vec![
            ("q", "Quit application"),
            ("c", "Open console to view logs"),
            ("r", "Refresh topics list"),
            ("/", "Toggle filter mode"),
            ("j/↓", "Navigate down"),
            ("k/↑", "Navigate up"),
            ("ESC", "Exit filter mode or close this help"),
        ],
        3,
    );

    let outer_block = Block::default().borders(Borders::ALL).title(Span::styled(
        " Keyboard Shortcuts ",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ));

    let inner_area = outer_block.inner(area); // area is the full Rect
    f.render_widget(outer_block, area);

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33); 3])
        .split(inner_area);

    // Then render each column without borders
    for (i, col) in columns.into_iter().enumerate() {
        let text = items[i]
            .iter()
            .map(|&(key, desc)| {
                Spans::from(vec![
                    Span::styled(format!("{}: ", key), Style::default().fg(Color::Yellow)),
                    Span::raw(desc),
                ])
            })
            .collect::<Vec<_>>();
        let paragraph = Paragraph::new(text).wrap(tui::widgets::Wrap { trim: true });

        f.render_widget(paragraph, col);
    }
}

fn split_vec_into_n<T: Clone>(vec: Vec<T>, n: usize) -> Vec<Vec<T>> {
    let len = vec.len();
    let chunk_size = (len + n - 1) / n;
    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        let start = i * chunk_size;
        let end = std::cmp::min(start + chunk_size, len);
        if start < len {
            result.push(vec[start..end].to_vec());
        } else {
            result.push(vec![]);
        }
    }
    result
}
