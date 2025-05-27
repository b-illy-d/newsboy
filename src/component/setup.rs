use ratatui::{layout::Rect, style::Stylize, text::Text, widgets::Paragraph, Frame};

use crate::{app::App, component::reusable::text_field::draw_simple_text_field, event::AppEvent};

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
    port: u8,
}

impl Setup {
    pub fn get_fields_info() -> &'static [(&'static str, &'static str)] {
        FIELDS
    }

    pub fn init(&self, app: &mut App) {
        app.text_fields.add("project_id", "Project ID");
    }

    pub fn get(&self, name: &str) -> &String {
        match name {
            "project_id" => &self.project_id,
            _ => panic!("Unknown setting: {}", name),
        }
    }

    pub fn set(&mut self, name: &str, value: String) {
        match name {
            "project_id" => self.project_id = value,
            _ => panic!("Unknown setting: {}", name),
        }
    }
}

// ================
// ==== EVENTS ====
// ================

pub enum SetupEvent {
    ProjectId(String),
}

fn set_project_id(id: &str) -> SetupEvent {
    SetupEvent::ProjectId(id.to_string())
}

// ==================
// ==== HANDLERS ====
// ==================

pub fn on_event(state: &mut Setup, e: SetupEvent) -> Option<AppEvent> {
    match e {
        SetupEvent::ProjectId(id) => on_set_project_id(state, id),
    }
}

fn on_set_project_id(state: &mut Setup, id: String) -> Option<AppEvent> {
    state.set("project_id", id.to_string());
    None
}

// ================
// ==== EVENTS ====
// ================

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
