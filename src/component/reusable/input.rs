use color_eyre::Result;
use ratatui::{
    crossterm::event::{KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Position},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph},
    DefaultTerminal, Frame,
};

use crate::app::App;
use crate::event::Event;

use crate::tokio::mpsc::{Receiver, Sender};

struct TextField {
    name: String,
    input: String,
    character_index: usize,
    input_mode: InputMode,
    messages: Vec<String>,
    tx: Sender<Event>,
    rx: &mut Receiver<Event>,
    is_done: bool,
}

enum InputMode {
    Normal,
    Editing,
}

impl TextField {
    const fn new(field_name: String) -> Self {
        Self {
            name: field_name,
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            character_index: 0,
        }
    }

    pub async fn run(&mut self, app: &mut App, frame: &mut Frame, rect: Rect) -> Result<String> {
        loop {
            if self.is_done {
                break;
            }
            while let Some(event) = app.receiver.recv().await {
                match event {
                    Event::Input(key) if key.kind == KeyEventKind::Press => {
                        self.on_key(key, app);
                    }
                    _ => {}
                }
            }
        }

        app.sender
            .send(Event::TextInput(self.name.clone(), self.input.clone()))
            .await?;
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn submit_message(&mut self, app: &mut App) {
        self.is_done = true;
    }

    pub fn on_key(&mut self, key: KeyEvent, app: &mut App) {
        match key.code {
            KeyCode::Enter => self.submit_message(),
            KeyCode::Char(to_insert) => self.enter_char(to_insert),
            KeyCode::Backspace => self.delete_char(),
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Esc => self.input_mode = InputMode::Normal,
            _ => {}
        };
    }

    fn draw(&self, frame: &mut Frame, rect: Rect) {
        let vertical = Layout::vertical([Constraint::Length(1), Constraint::Length(3)]);
        let [help_area, input_area] = vertical.areas(rect);

        let msg = vec![
            "Press ".into(),
            "Esc".bold(),
            " to cancel, ".into(),
            "Enter".bold(),
            " to submit".into(),
        ];
        let text = Text::from(Line::from(msg)).patch_style(Style::default().fg(Color::Cyan));
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, help_area);

        let input = Paragraph::new(self.input.as_str())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::bordered().title("Input"));
        frame.render_widget(input, input_area);

        // Make the cursor visible and ask ratatui to put it at the specified coordinates after
        // rendering
        #[allow(clippy::cast_possible_truncation)]
        frame.set_cursor_position(Position::new(
            // Draw the cursor at the current position in the input field.
            // This position is can be controlled via the left and right arrow key
            input_area.x + self.character_index as u16 + 1,
            // Move one line down, from the border to the input line
            input_area.y + 1,
        ));
    }
}
