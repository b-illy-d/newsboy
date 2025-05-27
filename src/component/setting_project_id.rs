use crate::event::{finish_setting_project_id, input_project_id, InputHandled};
use crossterm::event::{
    KeyCode::{Char, Enter, Esc},
    KeyEvent,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

#[derive(Default)]
pub struct SettingProjectId {
    pub active: bool,
    pub input: String,
}

pub fn draw(state: &SettingProjectId, f: &mut Frame) {
    if !state.active {
        return;
    }

    let full_area = f.area();

    let modal_width = 100;
    let modal_height = 10;

    let x = full_area.x + (full_area.width.saturating_sub(modal_width)) / 2;
    let y = full_area.y + (full_area.height.saturating_sub(modal_height)) / 2;

    let area = Rect::new(x, y, modal_width, modal_height);

    let block = Block::default()
        .title("Set Project ID")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::White).bg(Color::Black));

    f.render_widget(block, area);

    // Inner layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1), // Input
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // Buttons
        ])
        .split(area);

    let input = Paragraph::new(state.input.as_str())
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::ALL).title("Project ID"));

    let buttons = Line::from(vec![
        Span::styled("[Save]", Style::default().fg(Color::White)),
        Span::raw("  "),
        Span::styled("[Cancel]", Style::default().fg(Color::White)),
    ])
    .alignment(Alignment::Center);

    f.render_widget(input, chunks[0]);
    f.render_widget(buttons, chunks[2]);
}

pub async fn on_key(state: &SettingProjectId, key: KeyEvent) -> InputHandled {
    match key.code {
        Enter => {
            let project = match state.input.trim() {
                "" => None,
                input => Some(input.to_string()),
            };
            InputHandled::Handled(Some(finish_setting_project_id(project)))
        }
        Esc => InputHandled::Handled(Some(finish_setting_project_id(None))),
        Char(k) => InputHandled::Handled(Some(input_project_id(k.to_string()))),
        _ => InputHandled::NotHandled,
    }
}
