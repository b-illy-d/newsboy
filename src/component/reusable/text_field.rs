use std::usize;

use crate::{
    component::debug::debug_log,
    input::{handled, handled_empty, not_handled, InputHandled},
};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
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

    pub fn set_value(&mut self, value: String) {
        self.value = value;
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
    DeleteChar(usize, CursorDirection),
    MoveCursor(usize, CursorDirection),
    ValueChanged,
}

#[derive(Debug, Clone)]
pub enum CursorDirection {
    Left,
    Right,
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

fn delete_left(name: &str, number: usize) -> TextFieldEvent {
    TextFieldEvent::new(
        name.to_string(),
        TextFieldEventType::DeleteChar(number, CursorDirection::Left),
    )
}

fn delete_right(name: &str, number: usize) -> TextFieldEvent {
    TextFieldEvent::new(
        name.to_string(),
        TextFieldEventType::DeleteChar(number, CursorDirection::Right),
    )
}

fn delete_left_by_word(state: &TextField) -> TextFieldEvent {
    TextFieldEvent::new(
        state.name.to_string(),
        TextFieldEventType::DeleteChar(distance_to_prev_non_alpha(state), CursorDirection::Left),
    )
}

fn delete_right_by_word(state: &TextField) -> TextFieldEvent {
    TextFieldEvent::new(
        state.name.to_string(),
        TextFieldEventType::DeleteChar(distance_to_next_non_alpha(state), CursorDirection::Right),
    )
}

fn move_cursor_left(name: &str, distance: usize) -> TextFieldEvent {
    TextFieldEvent::new(
        name.to_string(),
        TextFieldEventType::MoveCursor(distance, CursorDirection::Left),
    )
}

fn move_cursor_right(name: &str, distance: usize) -> TextFieldEvent {
    TextFieldEvent::new(
        name.to_string(),
        TextFieldEventType::MoveCursor(distance, CursorDirection::Right),
    )
}

fn move_cursor_left_by_word(state: &TextField) -> TextFieldEvent {
    TextFieldEvent::new(
        state.name.to_string(),
        TextFieldEventType::MoveCursor(distance_to_prev_non_alpha(state), CursorDirection::Left),
    )
}

fn move_cursor_right_by_word(state: &TextField) -> TextFieldEvent {
    TextFieldEvent::new(
        state.name.to_string(),
        TextFieldEventType::MoveCursor(distance_to_next_non_alpha(state), CursorDirection::Right),
    )
}

fn value_changed(name: &str) -> TextFieldEvent {
    TextFieldEvent::new(name.to_string(), TextFieldEventType::ValueChanged)
}

fn distance_to_next_non_alpha(state: &TextField) -> usize {
    let input = &state.input;
    let current_index = byte_index(state);
    let next_non_alpha = input
        .char_indices()
        .skip(current_index + 1)
        .find(|(_, c)| !c.is_alphanumeric())
        .map_or(input.len(), |(i, _)| i);
    next_non_alpha - current_index
}

fn distance_to_prev_non_alpha(state: &TextField) -> usize {
    let input = &state.input;
    let current_index = byte_index(state);
    let prev_non_alpha = input
        .char_indices()
        .rev()
        .skip(input.len() - current_index + 1)
        .find(|(_, c)| !c.is_alphanumeric())
        .map_or(0, |(i, _)| i);
    current_index - prev_non_alpha
}

// ==================
// ==== HANDLERS ====
// ==================

pub fn on_event(state: &mut TextField, e: TextFieldEventType) -> Option<TextFieldEvent> {
    match e {
        TextFieldEventType::StartEditing => on_start_editing(state),
        TextFieldEventType::DoneEditing(submit) => on_done_editing(state, submit),
        TextFieldEventType::InputChar(c) => on_enter_char(state, c.chars().next().unwrap()),
        TextFieldEventType::DeleteChar(n, d) => on_delete_char(state, n, d),
        TextFieldEventType::MoveCursor(n, d) => on_move_cursor(state, n, d),
        TextFieldEventType::ValueChanged => None,
    }
}

fn on_start_editing(state: &mut TextField) -> Option<TextFieldEvent> {
    state.is_editing = true;
    state.input = state.value.clone();
    None
}

fn on_done_editing(state: &mut TextField, submit: bool) -> Option<TextFieldEvent> {
    if submit {
        state.set_value(state.input.clone());
    }
    state.input.clear();
    state.reset_cursor();
    state.is_editing = false;
    if submit {
        Some(value_changed(&state.name))
    } else {
        None
    }
}

fn on_move_cursor(
    state: &mut TextField,
    distance: usize,
    direction: CursorDirection,
) -> Option<TextFieldEvent> {
    match direction {
        CursorDirection::Left => on_move_cursor_left(state, distance),
        CursorDirection::Right => on_move_cursor_right(state, distance),
    }
}

fn on_move_cursor_left(state: &mut TextField, distance: usize) -> Option<TextFieldEvent> {
    debug_log(format!("Move left by {distance}"));
    let cursor_moved_left = state.character_index.saturating_sub(distance);
    state.character_index = clamp_cursor(state, cursor_moved_left);
    None
}

fn on_move_cursor_right(state: &mut TextField, distance: usize) -> Option<TextFieldEvent> {
    debug_log(format!("Move right by {distance}"));
    let cursor_moved_right = state.character_index.saturating_add(distance);
    state.character_index = clamp_cursor(state, cursor_moved_right);
    None
}

fn on_enter_char(state: &mut TextField, new_char: char) -> Option<TextFieldEvent> {
    let index = byte_index(state);
    state.input.insert(index, new_char);
    Some(move_cursor_right(&state.name, 1))
}

fn on_delete_char(
    state: &mut TextField,
    number: usize,
    direction: CursorDirection,
) -> Option<TextFieldEvent> {
    match direction {
        CursorDirection::Left => on_delete_left(state, number),
        CursorDirection::Right => on_delete_right(state, number),
    }
}

fn on_delete_left(state: &mut TextField, number: usize) -> Option<TextFieldEvent> {
    let is_not_cursor_leftmost = state.character_index != 0;
    if is_not_cursor_leftmost {
        let current_index = state.character_index;
        let from_left_to_current_index = current_index - number;
        let before_char_to_delete = state.input.chars().take(from_left_to_current_index);
        let after_char_to_delete = state.input.chars().skip(current_index);
        state.input = before_char_to_delete.chain(after_char_to_delete).collect();
        Some(move_cursor_left(&state.name, number))
    } else {
        None
    }
}

fn on_delete_right(state: &mut TextField, number: usize) -> Option<TextFieldEvent> {
    let is_not_cursor_rightmost = state.character_index < state.input.chars().count();
    if is_not_cursor_rightmost {
        let current_index = state.character_index;
        let from_current_index_to_right = current_index + number;
        let before_char_to_delete = state.input.chars().take(current_index);
        let after_char_to_delete = state.input.chars().skip(from_current_index_to_right);
        state.input = before_char_to_delete.chain(after_char_to_delete).collect();
    };
    None
}

// ===============
// ==== INPUT ====
// ===============

pub fn on_key(state: &TextField, key: KeyEvent) -> InputHandled<TextFieldEvent> {
    match state.is_editing {
        true => match key.code {
            KeyCode::Enter => handled(done_editing(&state.name, true).into()),
            KeyCode::Char(k) if key.modifiers.is_empty() => {
                handled(enter_char(&state.name, k).into())
            }
            KeyCode::Backspace => handled(delete_left(&state.name, 1).into()),
            KeyCode::Delete => {
                if state.character_index < state.input.chars().count() {
                    handled(delete_right(&state.name, 1).into())
                } else {
                    handled_empty()
                }
            }
            KeyCode::Left => handled(move_cursor_left(&state.name, 1).into()),
            KeyCode::Right => handled(move_cursor_right(&state.name, 1).into()),
            KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::ALT) => {
                handled(move_cursor_right_by_word(state).into())
            }
            KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::ALT) => {
                handled(move_cursor_left_by_word(state).into())
            }
            KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::ALT) => {
                handled(delete_right_by_word(state).into())
            }
            KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::ALT) => {
                handled(delete_left_by_word(state).into())
            }
            KeyCode::Char('w') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                handled(delete_left_by_word(state).into())
            }
            KeyCode::Home => handled(move_cursor_left(&state.name, byte_index(state)).into()),
            KeyCode::End => {
                handled(move_cursor_right(&state.name, state.input.char_indices().count()).into())
            }
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
