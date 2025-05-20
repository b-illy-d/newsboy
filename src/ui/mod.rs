mod header;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

use header::draw_header;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    if app.show_console {
        draw_with_console(f, app);
    } else {
        draw_without_console(f, app);
    }
}

fn draw_with_console<B: Backend>(f: &mut Frame<B>, app: &App) {
    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Main content
            Constraint::Length(3),  // Status bar
            Constraint::Length(11), // Console
        ])
        .split(f.size());

    draw_header(f, app, chunks[0]);
    draw_main_content(f, app, chunks[1]);
    draw_status_bar(f, app, chunks[2]);
    draw_console(f, app, chunks[3]);
}

fn draw_without_console<B: Backend>(f: &mut Frame<B>, app: &App) {
    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Status bar
        ])
        .split(f.size());

    draw_header(f, app, chunks[0]);
    draw_main_content(f, app, chunks[1]);
    draw_status_bar(f, app, chunks[2]);

    // Draw help modal on top if active
    if app.show_help {
        draw_help_modal(f, app, f.size());
    }
}

fn draw_main_content<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // Topic list
            Constraint::Percentage(70), // Topic details
        ])
        .split(area);

    draw_topic_list(f, app, chunks[0]);
    draw_topic_details(f, app, chunks[1]);
}

fn draw_topic_list<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let topics: Vec<ListItem> = app
        .visible_topics
        .iter()
        .map(|topic| ListItem::new(topic.name.clone()))
        .collect();

    let filter_title = if app.filter_active {
        format!(" Topics [ Search: {} ] ", app.filter_text)
    } else {
        " Topics ".to_string()
    };

    let topics_list = List::new(topics)
        .block(Block::default().title(filter_title).borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    let mut list_state = tui::widgets::ListState::default();
    list_state.select(app.selected_topic_index);
    f.render_stateful_widget(topics_list, area, &mut list_state);
}

fn draw_topic_details<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let detail_block = Block::default()
        .title(" Topic Details ")
        .borders(Borders::ALL);

    if let Some(index) = app.selected_topic_index {
        if let Some(topic) = app.topics.get(index) {
            let mut detail_text = vec![
                Spans::from(vec![
                    Span::styled("Name: ", Style::default().fg(Color::Yellow)),
                    Span::raw(&topic.name),
                ]),
                Spans::from(vec![
                    Span::styled("Full Path: ", Style::default().fg(Color::Yellow)),
                    Span::raw(&topic.full_name),
                ]),
                Spans::from(Span::styled("Labels:", Style::default().fg(Color::Yellow))),
            ];

            // Add labels
            if topic.labels.is_empty() {
                detail_text.push(Spans::from("  No labels"));
            } else {
                for (key, value) in &topic.labels {
                    detail_text.push(Spans::from(vec![
                        Span::raw("  "),
                        Span::styled(key, Style::default().fg(Color::Green)),
                        Span::raw(": "),
                        Span::raw(value),
                    ]));
                }
            }

            let details = Paragraph::new(detail_text).block(detail_block);

            f.render_widget(details, area);
        } else {
            let empty = Paragraph::new("Select a topic to view details").block(detail_block);

            f.render_widget(empty, area);
        }
    } else {
        let empty = Paragraph::new("No topic selected").block(detail_block);

        f.render_widget(empty, area);
    }
}

fn draw_status_bar<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let status_span = Span::styled(app.status_text.clone(), Style::default().fg(Color::Yellow));
    let cheat_sheet_text = Spans::from(vec![
        Span::styled(" q ", Style::default().bg(Color::DarkGray)),
        Span::raw(" Quit | "),
        Span::styled(" / ", Style::default().bg(Color::DarkGray)),
        Span::raw(" Filter | "),
        Span::styled(" r ", Style::default().bg(Color::DarkGray)),
        Span::raw(" Refresh | "),
        Span::styled(" ? ", Style::default().bg(Color::DarkGray)),
        Span::raw(" Help"),
    ]);

    let spaces = " ".repeat(area.width as usize - status_span.content.len() - 1);
    let status_text = vec![
        Spans::from(status_span),
        Spans::from(spaces),
        Spans::from(cheat_sheet_text),
    ];

    let status_bar = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default());

    f.render_widget(status_bar, area);
}

fn draw_console<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let console_block = Block::default()
        .title(" Console ")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black).fg(Color::Gray));

    let console_text = app
        .debug_logs
        .iter()
        .map(|log| Spans::from(vec![Span::raw(log)]))
        .collect::<Vec<_>>();

    let console_paragraph = Paragraph::new(console_text)
        .block(console_block)
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(tui::widgets::Wrap { trim: true });

    f.render_widget(console_paragraph, area);
}
