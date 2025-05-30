use crate::app::App;
use ratatui::{
    crossterm::event::{
        KeyCode::{Down, Esc, Up},
        KeyEvent,
    },
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Borders},
    Frame,
};
use std::collections::HashMap;

use crate::{
    component::debug::debug_log,
    component::reusable::text_field::{draw_simple_text_field, TextField},
    event::AppEvent,
    input::{handled, not_handled, InputHandled},
};

use super::reusable::text_field::{self, TextFieldEvent};

// ===============
// ==== STATE ====
// ===============

struct SetupField {
    name: &'static str,
    label: &'static str,
    value: &'static str,
}

const FIELDS: &[SetupField] = &[
    SetupField {
        name: "project_id",
        label: "Project ID",
        value: "",
    },
    SetupField {
        name: "host",
        label: "Host",
        value: "localhost",
    },
    SetupField {
        name: "port",
        label: "Port",
        value: "8065",
    },
];

const FIELD_NAMES: &[&str] = &["project_id", "host", "port"];

#[derive(Default)]
pub struct Setup {
    fields: HashMap<String, TextField>,
    pub focused: Option<String>,
}

impl Setup {
    pub fn default() -> Self {
        Setup {
            fields: HashMap::new(),
            focused: None,
        }
    }

    pub fn get_field_names() -> &'static [&'static str] {
        FIELD_NAMES
    }

    pub fn get(&self, name: &str) -> String {
        if !self.fields.contains_key(name) {
            panic!("Unknown setting: {}", name);
        }
        self.fields.get(name).unwrap().value.clone()
    }

    pub fn set(&mut self, name: &str, value: String) {
        if !self.fields.contains_key(name) {
            panic!("Unknown setting: {}", name);
        }
        let field = self.fields.get_mut(name).unwrap();
        field.value = value;
    }
}

pub fn init(setup: &mut Setup) {
    FIELDS.iter().for_each(|field| {
        let mut tf = TextField::new(field.name, field.label);
        if !field.value.is_empty() {
            tf.set_value(field.value);
        }
        setup.fields.insert(field.name.to_string(), tf);
    });
}

pub fn on_arrive(setup: &mut Setup) {
    setup.focused = Some("project_id".to_string());
}

pub fn on_leave(setup: &mut Setup) {
    setup.focused = None;
}

// ================
// ==== EVENTS ====
// ================

#[derive(Debug, Clone)]
pub enum SetupEvent {
    ChangeSetupValue(String, String),
    SetupFieldEvent(TextFieldEvent),
    Focus(Option<String>),
}

impl From<TextFieldEvent> for SetupEvent {
    fn from(event: TextFieldEvent) -> Self {
        SetupEvent::SetupFieldEvent(event)
    }
}

impl From<InputHandled<TextFieldEvent>> for InputHandled<SetupEvent> {
    fn from(handled: InputHandled<TextFieldEvent>) -> Self {
        match handled {
            InputHandled::Handled(event) => match event {
                Some(e) => InputHandled::Handled(Some(e.into())),
                None => InputHandled::Handled(None),
            },
            InputHandled::NotHandled => InputHandled::NotHandled,
        }
    }
}

impl From<InputHandled<SetupEvent>> for InputHandled<AppEvent> {
    fn from(handled: InputHandled<SetupEvent>) -> Self {
        match handled {
            InputHandled::Handled(event) => InputHandled::Handled(event.map(AppEvent::from)),
            InputHandled::NotHandled => InputHandled::NotHandled,
        }
    }
}

fn set_value(name: &str, value: &str) -> SetupEvent {
    SetupEvent::ChangeSetupValue(name.to_string(), value.to_string())
}

fn focus(name: &str) -> SetupEvent {
    SetupEvent::Focus(Some(name.to_string()))
}

fn unfocus() -> SetupEvent {
    SetupEvent::Focus(None)
}

// ==================
// ==== HANDLERS ====
// ==================

pub fn on_event(state: &mut Setup, e: SetupEvent) -> Option<AppEvent> {
    match e {
        SetupEvent::SetupFieldEvent(e) => on_field_event(state, e),
        SetupEvent::ChangeSetupValue(name, value) => on_change_value(state, name, value),
        SetupEvent::Focus(name) => {
            state.focused = name.clone();
            None
        }
    }
    .map(AppEvent::from)
}

fn on_field_event(state: &mut Setup, e: TextFieldEvent) -> Option<SetupEvent> {
    debug_log(format!("    Field event: {:?}", e));
    let target = e.name.clone();
    if !state.fields.contains_key(&target) {
        panic!("Unknown field: {}", target);
    }
    let field = state.fields.get_mut(&target).unwrap();
    text_field::on_event(field, e).map(SetupEvent::from)
}

fn on_change_value(state: &mut Setup, name: String, value: String) -> Option<SetupEvent> {
    state.set(&name, value);
    None
}

// ===============
// ==== INPUT ====
// ===============

pub fn on_key(state: &Setup, key: KeyEvent) -> InputHandled<AppEvent> {
    let text_handled = on_text_field_key(state, key)
        .map(SetupEvent::from)
        .map(AppEvent::from);
    if text_handled.is_handled() {
        return text_handled;
    }

    match key.code {
        Up | Down => on_arrow_key(state, key).into(),
        Esc => {
            if state.focused.is_some() {
                handled(unfocus().into())
            } else {
                not_handled()
            }
        }
        _ => not_handled(),
    }
}

fn on_text_field_key(state: &Setup, key: KeyEvent) -> InputHandled<TextFieldEvent> {
    if let Some(ref focused) = state.focused {
        let field = state.fields.get(focused).unwrap();
        text_field::on_key(field, key)
    } else {
        not_handled()
    }
}

fn on_arrow_key(state: &Setup, key: KeyEvent) -> InputHandled<SetupEvent> {
    let fields = Setup::get_field_names();

    if matches!(state.focused, None) {
        return handled(focus(fields[0]).into());
    }

    let current_index = fields
        .iter()
        .position(|n| n == &state.focused.clone().unwrap())
        .unwrap_or(0);

    match key.code {
        Up => {
            let next_index = if current_index == 0 {
                fields.len() - 1
            } else {
                current_index - 1
            };
            debug_log(format!(
                "Next idx {} next field {}",
                next_index, fields[next_index]
            ));
            handled(focus(fields[next_index]))
        }
        Down => {
            let next_index = (current_index + 1) % fields.len();
            debug_log(format!(
                "Next idx {} next field {}",
                next_index, fields[next_index]
            ));
            handled(focus(fields[next_index]))
        }
        _ => not_handled(),
    }
}

// ==============
// ==== VIEW ====
// ==============

const TITLE: &str = "Setup";
const VIEWING_HELP: &str = "↑/↓ to navigate, Spacebar to edit";
const EDITING_HELP: &str = "Editing: Press Enter to save, Esc to cancel";
pub fn draw(state: &Setup, f: &mut Frame, area: Rect) {
    let is_editing = match state.focused {
        None => false,
        Some(ref name) => state.fields.get(name).unwrap().is_editing,
    };
    let block = Block::default()
        .title(TITLE.to_string())
        .title_bottom(
            Line::from(match is_editing {
                true => EDITING_HELP.to_string(),
                false => VIEWING_HELP.to_string(),
            })
            .right_aligned(),
        )
        .light_blue()
        .fg(Color::LightCyan)
        .bg(Color::Black)
        .borders(Borders::ALL);
    f.render_widget(block, area);

    let [content_area] = Layout::default()
        .vertical_margin(2)
        .horizontal_margin(10)
        .constraints([Constraint::Length(100)])
        .direction(Direction::Horizontal)
        .areas(area);
    draw_fields(state, f, content_area);
}

fn draw_fields(state: &Setup, f: &mut Frame, area: Rect) {
    let fields = Setup::get_field_names();
    fn width(name: &str) -> u16 {
        match name {
            "port" => 10,
            _ => 80,
        }
    }
    for (i, name) in fields.iter().enumerate() {
        let field = state.fields.get(*name).unwrap();
        let field_area = Rect::new(area.x, area.y + i as u16 * 3, width(name), 1);
        let is_focused = match &state.focused {
            Some(n) => n == name,
            None => false,
        };
        draw_simple_text_field(field, is_focused, f, field_area);
    }
}
