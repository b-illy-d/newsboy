use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint::Percentage, Direction::Vertical, Layout, Rect},
    style::{
        Color::{Black, Green},
        Style, Stylize,
    },
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
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
    editing_idx: Option<usize>,
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
            editing_idx: None,
            choices,
        }
    }

    pub fn choose_index(&mut self, index: Option<usize>) {
        if let Some(idx) = index {
            if idx < self.choices.len() {
                self.value = self.choices[idx].value.clone();
                self.chosen_idx = Some(idx);
                self.editing_idx = Some(idx);
            } else {
                panic!("Index out of bounds for choices");
            }
        } else {
            self.value.clear();
            self.chosen_idx = None;
            self.editing_idx = None;
        }
    }

    fn get_selected_label(&self) -> String {
        if let Some(idx) = self.chosen_idx {
            if idx < self.choices.len() {
                self.choices[idx].label.clone()
            } else {
                "None".to_string()
            }
        } else {
            "None".to_string()
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
    SetChosenIndex(Option<usize>),
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

fn pick_choice(name: String, idx: Option<usize>) -> ChoicesEvent {
    ChoicesEvent::new(name, ChoicesEventType::SetChosenIndex(idx))
}

fn value_changed(name: String) -> ChoicesEvent {
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
        ChoicesEventType::SetChosenIndex(idx) => {
            match idx {
                Some(index) => {
                    state.editing_idx = Some(index);
                }
                None => {
                    state.editing_idx = None;
                }
            };
            None
        }
        ChoicesEventType::DoneEditing(confirmed) => {
            state.is_editing = false;
            if confirmed {
                state.choose_index(state.editing_idx);
                Some(value_changed(state.name.clone()))
            } else {
                state.editing_idx = state.chosen_idx;
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
            KeyCode::Up => {
                if let Some(idx) = state.editing_idx {
                    let new_idx = if idx == 0 {
                        state.choices.len() - 1
                    } else {
                        idx - 1
                    };
                    handled(pick_choice(state.name.clone(), Some(new_idx)))
                } else {
                    not_handled()
                }
            }
            KeyCode::Down => {
                if let Some(idx) = state.editing_idx {
                    let new_idx = if idx == state.choices.len() - 1 {
                        0
                    } else {
                        idx + 1
                    };
                    handled(pick_choice(state.name.clone(), Some(new_idx)))
                } else {
                    not_handled()
                }
            }
            KeyCode::Enter => handled(done_choosing(state.name.clone(), true).into()),
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

    let longest_choice: u16 = state
        .choices
        .iter()
        .map(|c| c.label.len() as u16)
        .max()
        .unwrap_or(0);

    let open_choices_rect = Rect {
        x: adjusted_rect.x + adjusted_rect.width,
        y: adjusted_rect.y,
        width: longest_choice + 4,
        height: state.choices.len() as u16 + 2,
    };

    let selected = state.get_selected_label();
    let input = Paragraph::new(selected.as_str())
        .style(match is_focused {
            true => Style::default().bold().green(),
            false => Style::default(),
        })
        .block(
            Block::default()
                .title(state.label.clone())
                .borders(Borders::ALL)
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
    let [content_area] = Layout::default()
        .direction(Vertical)
        .constraints([Percentage(100)])
        .areas(rect);
    let selected = state.editing_idx.map_or(0, |idx| idx);
    let mut state = ListState::default().with_selected(Some(selected));
    let list = List::new(choices)
        .highlight_style(Style::default().bg(Green).fg(Black))
        .highlight_symbol(">>")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().bold()),
        );
    frame.render_stateful_widget(list, content_area, &mut state);
}
