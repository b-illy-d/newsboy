mod header;

use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

use header::draw_header;

pub trait Component {
    fn init(&mut self, app: &App);
    fn handle(&mut self, ev: &Event, app: &mut App);
    fn view(&self, f: &mut Frame);
}

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(7), // Header
            Constraint::Min(0),
        ])
        .split(f.size());

    draw_header(f, chunks[0]);
    if app.show_console {
        draw_with_console(f, app, chunks[1]);
    } else {
        draw_without_console(f, app, chunks[1]);
    }
}

fn draw_with_console(f: &mut Frame, app: &App, area: Rect) {
    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),     // Main content
            Constraint::Length(3),  // Status bar
            Constraint::Length(11), // Console
        ])
        .split(area);

    draw_main_content(f, app, chunks[0]);
    draw_status_bar(f, app, chunks[1]);
    draw_console(f, app, chunks[2]);
}

fn draw_without_console(f: &mut Frame, app: &App, area: Rect) {
    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Status bar
        ])
        .split(area);

    draw_main_content(f, app, chunks[0]);
    draw_status_bar(f, app, chunks[1]);
}

fn draw_main_content(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // Topic list
            Constraint::Percentage(70), // Topic details
        ])
        .split(area);

    // Placeholder for topic list - needs to be implemented
    let topics_block = Block::default().title(" Topics ").borders(Borders::ALL);
    f.render_widget(topics_block, chunks[0]);
    draw_topic_details(f, app, chunks[1]);
}

fn draw_topic_details(f: &mut Frame, app: &App, area: Rect) {
    let detail_block = Block::default()
        .title(" Topic Details ")
        .borders(Borders::ALL);

    if let Some(index) = app.selected_topic_index {
        if let Some(topic) = app.topics.get(index) {
            let mut detail_text = Text::from(vec![
                Line::from(vec![
                    Span::styled("Name: ", Style::default().fg(Color::Yellow)),
                    Span::raw(&topic.name),
                ]),
                Line::from(vec![
                    Span::styled("Full Path: ", Style::default().fg(Color::Yellow)),
                    Span::raw(&topic.full_name),
                ]),
                Line::from(Span::styled("Labels:", Style::default().fg(Color::Yellow))),
            ]);

            // Add labels
            if topic.labels.is_empty() {
                detail_text.lines.push(Line::from("  No labels"));
            } else {
                for (key, value) in &topic.labels {
                    detail_text.lines.push(Line::from(vec![
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

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let status_span = Span::styled(app.status_text.clone(), Style::default().fg(Color::Yellow));
    let status_text = Text::from(Line::from(status_span));

    let status_bar = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default());

    f.render_widget(status_bar, area);
}

fn draw_console(f: &mut Frame, app: &App, area: Rect) {
    let console_block = Block::default()
        .title(" Console ")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black).fg(Color::Gray));

    let console_text = Text::from(
        app.debug_logs
            .iter()
            .map(|log| Line::from(vec![Span::raw(log)]))
            .collect::<Vec<_>>(),
    );

    let console_paragraph = Paragraph::new(console_text)
        .block(console_block)
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(console_paragraph, area);
}
