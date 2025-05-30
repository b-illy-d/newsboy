use crate::input::{handled, not_handled, InputHandled};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Position, Rect},
    style::{Style, Stylize},
    widgets::{Block, Paragraph},
    Frame,
};

pub struct TextField {
    pub name: String,
    pub label: String,
    pub value: String,
    pub input: String,
    pub character_index: usize,
    pub is_editing: bool,
}

impl TextField {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.to_string(),
            label: label.to_string(),
            value: String::new(),
            input: String::new(),
            character_index: 0,
            is_editing: false,
        }
    }

    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }
}

// ================
// ==== EVENTS ====
// ================

#[derive(Debug, Clone)]
pub enum TextFieldEventType {
    StartEditing,
    DoneEditing(bool),
    InputChar(String),
    DeleteChar,
    MoveCursorLeft,
    MoveCursorRight,
}

#[derive(Debug, Clone)]
pub struct TextFieldEvent {
    pub name: String,
    pub event_type: TextFieldEventType,
}

impl TextFieldEvent {
    pub fn new(name: String, event_type: TextFieldEventType) -> Self {
        TextFieldEvent { name, event_type }
    }
}

fn start_editing(name: &str) -> TextFieldEvent {
    TextFieldEvent::new(name.to_string(), TextFieldEventType::StartEditing)
}

fn done_editing(name: &str, submit: bool) -> TextFieldEvent {
    TextFieldEvent::new(name.to_string(), TextFieldEventType::DoneEditing(submit))
}

fn enter_char(name: &str, c: char) -> TextFieldEvent {
    TextFieldEvent::new(
        name.to_string(),
        TextFieldEventType::InputChar(c.to_string()),
    )
}

fn delete_char(name: &str) -> TextFieldEvent {
    TextFieldEvent::new(name.to_string(), TextFieldEventType::DeleteChar)
}

fn move_cursor_left(name: &str) -> TextFieldEvent {
    TextFieldEvent::new(name.to_string(), TextFieldEventType::MoveCursorLeft)
}

fn move_cursor_right(name: &str) -> TextFieldEvent {
    TextFieldEvent::new(name.to_string(), TextFieldEventType::MoveCursorRight)
}

// ==================
// ==== HANDLERS ====
// ==================

pub fn on_event(state: &mut TextField, e: TextFieldEvent) -> Option<TextFieldEvent> {
    match e.event_type {
        TextFieldEventType::StartEditing => on_start_editing(state),
        TextFieldEventType::DoneEditing(submit) => on_done_editing(state, submit),
        TextFieldEventType::InputChar(c) => on_enter_char(state, c.chars().next().unwrap()),
        TextFieldEventType::DeleteChar => on_delete_char(state),
        TextFieldEventType::MoveCursorLeft => on_move_cursor_left(state),
        TextFieldEventType::MoveCursorRight => on_move_cursor_right(state),
    }
}

fn on_start_editing<T>(state: &mut TextField) -> Option<T> {
    state.is_editing = true;
    state.input = state.value.clone();
    None
}

fn on_done_editing<T>(state: &mut TextField, submit: bool) -> Option<T> {
    if submit {
        state.value = state.input.clone();
    }
    state.input.clear();
    state.reset_cursor();
    state.is_editing = false;
    None
}

fn on_move_cursor_left<T>(state: &mut TextField) -> Option<T> {
    let cursor_moved_left = state.character_index.saturating_sub(1);
    state.character_index = clamp_cursor(state, cursor_moved_left);
    None
}

fn on_move_cursor_right<T>(state: &mut TextField) -> Option<T> {
    let cursor_moved_right = state.character_index.saturating_add(1);
    state.character_index = clamp_cursor(state, cursor_moved_right);
    None
}

fn on_enter_char(state: &mut TextField, new_char: char) -> Option<TextFieldEvent> {
    let index = byte_index(state);
    state.input.insert(index, new_char);
    Some(move_cursor_right(&state.name))
}

fn on_delete_char(state: &mut TextField) -> Option<TextFieldEvent> {
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
        Some(move_cursor_left(&state.name))
    } else {
        None
    }
}

// ===============
// ==== INPUT ====
// ===============

pub fn on_key(state: &TextField, key: KeyEvent) -> InputHandled<TextFieldEvent> {
    match state.is_editing {
        true => match key.code {
            KeyCode::Enter => handled(done_editing(&state.name, true).into()),
            KeyCode::Char(k) => handled(enter_char(&state.name, k).into()),
            KeyCode::Backspace => handled(delete_char(&state.name).into()),
            KeyCode::Left => handled(move_cursor_left(&state.name).into()),
            KeyCode::Right => handled(move_cursor_right(&state.name).into()),
            KeyCode::Esc => handled(done_editing(&state.name, false).into()),
            _ => not_handled(),
        },
        false => match key.code {
            KeyCode::Char(' ') => handled(start_editing(&state.name).into()),
            _ => not_handled(),
        },
    }
}

// ===============
// ==== VIEWS ====
// ===============

pub fn draw_simple_text_field(state: &TextField, is_focused: bool, frame: &mut Frame, rect: Rect) {
    let adjusted_rect = Rect {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: 3,
    };

    let input = (match state.is_editing {
        false => Paragraph::new(state.value.as_str()).style(match is_focused {
            true => Style::default().bold().green(),
            false => Style::default(),
        }),
        true => Paragraph::new(state.input.as_str()).style(Style::default().bold().yellow()),
    })
    .block(
        Block::default()
            .title(state.label.clone())
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(match is_focused {
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
