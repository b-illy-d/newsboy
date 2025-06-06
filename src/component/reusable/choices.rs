use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, List, ListItem, Paragraph},
    Frame,
};

use crate::input::{handled, not_handled, InputHandled};

pub struct Choice {
    pub label: String,
    pub value: String,
}
impl<'a> From<&Choice> for ListItem<'a> {
    fn from(choice: &Choice) -> Self {
        ListItem::new(choice.label.clone())
    }
}

pub struct Choices {
    pub name: String,
    pub label: String,
    pub value: String,
    pub is_editing: bool,
    pub chosen_idx: Option<usize>,
    pub choices: Vec<Choice>,
}

impl Choices {
    pub fn new(name: &str, label: &str, choices: Vec<Choice>) -> Self {
        Self {
            name: name.to_string(),
            label: label.to_string(),
            value: String::new(),
            is_editing: false,
            chosen_idx: None,
            choices,
        }
    }

    pub fn choose_index(&mut self, index: Option<usize>) {
        if let Some(idx) = index {
            if idx < self.choices.len() {
                self.value = self.choices[idx].value.clone();
                self.chosen_idx = Some(idx);
            } else {
                panic!("Index out of bounds for choices");
            }
        } else {
            self.value.clear();
            self.chosen_idx = None;
        }
    }
}

// ================
// ==== EVENTS ====
// ================

#[derive(Debug, Clone)]
pub enum ChoicesEventType {
    StartEditing,
    DoneEditing(bool),
    ValueChanged,
}

#[derive(Debug, Clone)]
pub struct ChoicesEvent {
    pub name: String,
    pub event_type: ChoicesEventType,
}

impl ChoicesEvent {
    pub fn new(name: String, event_type: ChoicesEventType) -> Self {
        ChoicesEvent { name, event_type }
    }
}

fn start_choosing(name: &str) -> ChoicesEvent {
    ChoicesEvent::new(name.to_string(), ChoicesEventType::StartEditing)
}

fn done_choosing(name: String, confirmed: bool) -> ChoicesEvent {
    ChoicesEvent::new(name, ChoicesEventType::DoneEditing(confirmed))
}

fn value_changed(name: String, chosen: Option<usize>) -> ChoicesEvent {
    ChoicesEvent::new(name, ChoicesEventType::ValueChanged)
}

// ==================
// ==== HANDLERS ====
// ==================

pub fn on_event(state: &mut Choices, event: ChoicesEventType) -> Option<ChoicesEvent> {
    match event {
        ChoicesEventType::StartEditing => {
            state.is_editing = true;
            None
        }
        ChoicesEventType::DoneEditing(confirmed) => {
            state.is_editing = false;
            if confirmed {
                state.choose_index(state.chosen_idx);
                Some(value_changed(state.name.clone(), state.chosen_idx.clone()))
            } else {
                None
            }
        }
        ChoicesEventType::ValueChanged => None,
    }
}

// ===============
// ==== INPUT ====
// ===============

pub fn on_key(state: &Choices, key: KeyEvent) -> InputHandled<ChoicesEvent> {
    match state.is_editing {
        true => match key.code {
            KeyCode::Esc => handled(done_choosing(state.name.clone(), false).into()),
            _ => not_handled(),
        },
        false => match key.code {
            KeyCode::Char(' ') => handled(start_choosing(&state.name).into()),
            _ => not_handled(),
        },
    }
}

// ===============
// ==== VIEWS ====
// ===============

pub fn draw(state: &Choices, is_focused: bool, frame: &mut Frame, rect: Rect) {
    let adjusted_rect = Rect {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: 3,
    };

    let open_choices_rect = Rect {
        x: adjusted_rect.x + adjusted_rect.width + 2,
        y: adjusted_rect.y,
        width: adjusted_rect.width,
        height: state.choices.len() as u16,
    };

    let input = Paragraph::new(state.value.as_str())
        .style(match is_focused {
            true => Style::default().bold().green(),
            false => Style::default(),
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
        draw_choices_open(state, frame, open_choices_rect);
    };
}

fn draw_choices_open(state: &Choices, frame: &mut Frame, rect: Rect) {
    let choices: Vec<_> = state.choices.iter().collect();

    let list = List::new(choices)
        .highlight_style(Style::default().bold().green())
        .highlight_symbol(">> ");
    frame.render_widget(list, rect);
}
