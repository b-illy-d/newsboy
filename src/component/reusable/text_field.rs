use crate::event::{handled, not_handled, AppEvent, InputHandled};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Position, Rect},
    style::{Color, Style, Styled, Stylize},
    widgets::{Block, Paragraph},
    Frame,
};
use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

pub struct TextFields {
    pub fields: HashMap<String, TextField>,
}

impl TextFields {
    pub fn new() -> Self {
        TextFields {
            fields: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, label: &str, init_value: Option<String>) {
        let field = TextField::new(name, label, init_value);
        self.fields.insert(field.name.clone(), field);
    }

    pub fn get(&self, name: &str) -> &TextField {
        match self.fields.get(name) {
            Some(field) => field,
            None => panic!("TextField with name '{}' not found", name),
        }
    }

    pub fn get_mut(&mut self, name: &str) -> &mut TextField {
        match self.fields.get_mut(name) {
            Some(field) => field,
            None => panic!("TextField with name '{}' not found", name),
        }
    }
}

pub struct TextField {
    pub id: String,
    pub name: String,
    pub label: String,
    pub value: String,
    pub input: String,
    pub character_index: usize,
    pub is_editing: bool,
    pub is_focused: bool,
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

impl TextField {
    pub fn new(name: &str, label: &str, init_value: Option<String>) -> Self {
        let uniq = COUNTER.fetch_add(1, Ordering::Relaxed);
        let id = format!("{}-{}", name, uniq);
        Self {
            id,
            name: name.to_string(),
            label: label.to_string(),
            value: init_value.unwrap_or(String::new()),
            input: String::new(),
            character_index: 0,
            is_editing: false,
            is_focused: false,
        }
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    pub fn expect(&self, id: &str) -> &TextField {
        if self.id == id {
            self
        } else {
            panic!(
                "Expected TextField with id '{}', but found '{}'",
                id, self.id
            );
        }
    }
}

// ================
// ==== EVENTS ====
// ================

#[derive(Clone)]
pub enum TextFieldEventType {
    StartEditing,
    DoneEditing(bool),
    InputChar(String),
    DeleteChar,
    MoveCursorLeft,
    MoveCursorRight,
}

#[derive(Clone)]
pub struct TextFieldEvent {
    pub id: String,
    pub event_type: TextFieldEventType,
}

impl TextFieldEvent {
    pub fn new(id: String, event_type: TextFieldEventType) -> Self {
        TextFieldEvent { id, event_type }
    }
}

fn start_editing(id: &str) -> TextFieldEvent {
    TextFieldEvent::new(id.to_string(), TextFieldEventType::StartEditing)
}

fn done_editing(id: &str, submit: bool) -> TextFieldEvent {
    TextFieldEvent::new(id.to_string(), TextFieldEventType::DoneEditing(submit))
}

fn enter_char(id: &str, c: char) -> TextFieldEvent {
    TextFieldEvent::new(id.to_string(), TextFieldEventType::InputChar(c.to_string()))
}

fn delete_char(id: &str) -> TextFieldEvent {
    TextFieldEvent::new(id.to_string(), TextFieldEventType::DeleteChar)
}

fn move_cursor_left(id: &str) -> TextFieldEvent {
    TextFieldEvent::new(id.to_string(), TextFieldEventType::MoveCursorLeft)
}

fn move_cursor_right(id: &str) -> TextFieldEvent {
    TextFieldEvent::new(id.to_string(), TextFieldEventType::MoveCursorRight)
}

// ==================
// ==== HANDLERS ====
// ==================

pub fn on_event(state: &mut TextField, e: TextFieldEvent) -> Option<AppEvent> {
    match e.event_type {
        TextFieldEventType::StartEditing => on_start_editing(state),
        TextFieldEventType::DoneEditing(submit) => on_done_editing(state, submit),
        TextFieldEventType::InputChar(c) => on_enter_char(state, c.chars().next().unwrap()),
        TextFieldEventType::DeleteChar => on_delete_char(state),
        TextFieldEventType::MoveCursorLeft => on_move_cursor_left(state),
        TextFieldEventType::MoveCursorRight => on_move_cursor_right(state),
    }
}

fn on_start_editing(state: &mut TextField) -> Option<AppEvent> {
    state.is_editing = true;
    None
}

fn on_done_editing(state: &mut TextField, submit: bool) -> Option<AppEvent> {
    if submit {
        state.value = state.input.clone();
    }
    state.input.clear();
    state.reset_cursor();
    state.is_editing = false;
    None
}

fn on_move_cursor_left(state: &mut TextField) -> Option<AppEvent> {
    let cursor_moved_left = state.character_index.saturating_sub(1);
    state.character_index = clamp_cursor(state, cursor_moved_left);
    None
}

fn on_move_cursor_right(state: &mut TextField) -> Option<AppEvent> {
    let cursor_moved_right = state.character_index.saturating_add(1);
    state.character_index = clamp_cursor(state, cursor_moved_right);
    None
}

fn on_enter_char(state: &mut TextField, new_char: char) -> Option<AppEvent> {
    let index = byte_index(state);
    state.input.insert(index, new_char);
    Some(AppEvent::TextField(move_cursor_right(&state.id)))
}

fn on_delete_char(state: &mut TextField) -> Option<AppEvent> {
    let is_not_cursor_leftmost = state.character_index != 0;
    if is_not_cursor_leftmost {
        // Method "remove" is not used on the saved text for deleting the selected char.
        // Reason: Using remove on String works on bytes instead of the chars.
        // Using remove would require special care because of char boundaries.

        let current_index = state.character_index;
        let from_left_to_current_index = current_index - 1;

        // Getting all characters before the selected character.
        let before_char_to_delete = state.input.chars().take(from_left_to_current_index);
        // Getting all characters after selected character.
        let after_char_to_delete = state.input.chars().skip(current_index);

        // Put all characters together except the selected one.
        // By leaving the selected one out, it is forgotten and therefore deleted.
        state.input = before_char_to_delete.chain(after_char_to_delete).collect();
        Some(AppEvent::TextField(move_cursor_left(&state.id)))
    } else {
        None
    }
}

// ===============
// ==== INPUT ====
// ===============

pub fn on_key(state: &TextField, key: KeyEvent) -> InputHandled {
    match state.is_editing {
        true => match key.code {
            KeyCode::Enter => handled(done_editing(&state.id, true).into()),
            KeyCode::Char(k) => handled(enter_char(&state.id, k).into()),
            KeyCode::Backspace => handled(delete_char(&state.id).into()),
            KeyCode::Left => handled(move_cursor_left(&state.id).into()),
            KeyCode::Right => handled(move_cursor_right(&state.id).into()),
            KeyCode::Esc => handled(done_editing(&state.id, false).into()),
            _ => not_handled(),
        },
        false => match key.code {
            KeyCode::Enter => handled(start_editing(&state.id).into()),
            _ => not_handled(),
        },
    }
}

// ===============
// ==== VIEWS ====
// ===============

pub fn draw_simple_text_field(state: &TextField, frame: &mut Frame, rect: Rect) {
    let adjusted_rect = Rect {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: 3,
    };

    let input = (match state.is_editing {
        false => Paragraph::new(state.value.as_str()).style(match state.is_focused {
            true => Style::default().bold(),
            false => Style::default(),
        }),
        true => Paragraph::new(state.input.as_str()).style(Style::default().bold()),
    })
    .block(
        Block::default()
            .title(state.label.clone())
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(match state.is_focused {
                true => Style::default().bold(),
                false => Style::default(),
            }),
    );
    frame.render_widget(input, adjusted_rect);

    if state.is_editing {
        #[allow(clippy::cast_possible_truncation)]
        frame.set_cursor_position(Position::new(
            adjusted_rect.x + state.character_index as u16 + 1,
            adjusted_rect.y + 1,
        ));
    }
}

// ===============
// ==== UTILS ====
// ===============

fn clamp_cursor(state: &TextField, new_cursor_pos: usize) -> usize {
    new_cursor_pos.clamp(0, state.input.chars().count())
}

fn byte_index(state: &TextField) -> usize {
    state
        .input
        .char_indices()
        .map(|(i, _)| i)
        .nth(state.character_index)
        .unwrap_or(state.input.len())
}
