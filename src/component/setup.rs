use ratatui::{
    crossterm::event::{
        KeyCode::{Char, Down, Up},
        KeyEvent,
    },
    layout::Rect,
    style::Stylize,
    text::Text,
    widgets::Paragraph,
    Frame,
};
use std::borrow::Cow;

use crate::{
    app::App,
    component::reusable::text_field::{draw_simple_text_field, TextFieldEvent, TextFieldEventType},
    event::{handled, not_handled, AppEvent, InputHandled},
};

// ===============
// ==== STATE ====
// ===============

const FIELDS: &[(&str, &str)] = &[
    ("project_id", "Project ID"),
    ("host", "Host"),
    ("port", "Port"),
];

#[derive(Default)]
pub struct Setup {
    project_id: String,
    host: String,
    port: u16,
    pub focused: String,
}

impl Setup {
    pub fn default() -> Self {
        Setup {
            project_id: String::new(),
            host: "http://localhost".to_string(),
            port: 8085,
            focused: "project_id".to_string(),
        }
    }

    pub fn get_fields_info() -> &'static [(&'static str, &'static str)] {
        FIELDS
    }

    pub fn get(&self, name: &str) -> Cow<'_, str> {
        match name {
            "project_id" => Cow::Borrowed(&self.project_id),
            "host" => Cow::Borrowed(&self.host),
            "port" => Cow::Owned(self.port.to_string()),
            _ => panic!("Unknown setting: {}", name),
        }
    }

    pub fn set(&mut self, name: &str, value: String) {
        match name {
            "project_id" => self.project_id = value,
            "host" => self.host = value,
            "port" => {
                if let Ok(port) = value.parse::<u16>() {
                    self.port = port;
                } else {
                    panic!("Invalid port value: {}", value);
                }
            }
            _ => panic!("Unknown setting: {}", name),
        }
    }
}

// ================
// ==== EVENTS ====
// ================

pub enum SetupEvent {
    ChangeSetupValue(String, String),
    EditSetupValue(String),
    FocusSetup(String),
}

fn edit_value(name: &str) -> SetupEvent {
    SetupEvent::EditSetupValue(name.to_string())
}

fn set_value(name: &str, value: &str) -> SetupEvent {
    SetupEvent::ChangeSetupValue(name.to_string(), value.to_string())
}

fn focus_setup(name: &str) -> SetupEvent {
    SetupEvent::FocusSetup(name.to_string())
}

// ==================
// ==== HANDLERS ====
// ==================

pub fn on_event(state: &mut Setup, e: SetupEvent) -> Option<AppEvent> {
    match e {
        SetupEvent::ChangeSetupValue(name, value) => on_change_value(state, name, value),
        SetupEvent::FocusSetup(name) => {
            state.focused = name;
            None
        }
        _ => None,
    }
}

fn on_change_value(state: &mut Setup, name: String, value: String) -> Option<AppEvent> {
    state.set(&name, value);
    None
}

// ===============
// ==== INPUT ====
// ===============

pub fn on_key(state: &Setup, key: KeyEvent) -> InputHandled {
    let fields = Setup::get_fields_info();
    let current_index = fields
        .iter()
        .position(|(n, _)| n == &state.focused)
        .unwrap_or(0);
    match key.code {
        Up => {
            let next_index = if current_index == 0 {
                fields.len() - 1
            } else {
                current_index - 1
            };
            handled(focus_setup(fields[next_index].0).into())
        }
        Down => {
            let next_index = (current_index + 1) % fields.len();
            handled(focus_setup(fields[next_index].0).into())
        }
        Char(' ') => {
            let current_field = fields[current_index].0;
            let event = TextFieldEvent {
                id: current_field.to_string(),
                event_type: TextFieldEventType::StartEditing,
            };
            handled(event.into())
        }
        _ => not_handled(),
    }
}

// ==============
// ==== VIEW ====
// ==============

pub fn draw(state: &App, f: &mut Frame, area: Rect) {
    let block = ratatui::widgets::Block::default()
        .title("Setup")
        .light_red()
        .borders(ratatui::widgets::Borders::ALL);
    f.render_widget(block, area);

    let help_text = Paragraph::new(Text::from("↑/↓ to navigate, Space to edit")).light_red();

    let [_padding, content_area] = ratatui::layout::Layout::default()
        .margin(2)
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Length(10),
            ratatui::layout::Constraint::Max(100),
        ])
        .areas(area);

    let [help_area, fields_area] = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Length(2),
            ratatui::layout::Constraint::Min(0),
        ])
        .areas(content_area);
    f.render_widget(help_text, help_area);
    draw_fields(state, f, fields_area);
}

fn draw_fields(state: &App, f: &mut Frame, area: Rect) {
    let fields = Setup::get_fields_info();
    fn width(name: &str) -> u16 {
        match name {
            "port" => 10,
            _ => 80,
        }
    }
    for (i, (name, _label)) in fields.iter().enumerate() {
        let field = state.text_fields.get(name);
        let field_area = ratatui::layout::Rect::new(area.x, area.y + i as u16 * 3, width(name), 1);
        draw_simple_text_field(field, f, field_area);
    }
}
