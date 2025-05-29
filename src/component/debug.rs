use ratatui::{
    layout::{Margin, Rect},
    text::{Line, Text},
    widgets::{Block, Paragraph},
    Frame,
};
use std::collections::VecDeque;
use std::sync::Mutex;

use crate::app::App;
use crate::event::{handled, AppEvent, InputHandled};
use once_cell::sync::Lazy;

pub static DEBUG_LOGS: Lazy<Mutex<VecDeque<String>>> = Lazy::new(|| Mutex::new(VecDeque::new()));

const MAX_LOGS: usize = 10;

pub fn debug_log<S: Into<String>>(msg: S) {
    let mut logs = DEBUG_LOGS.lock().unwrap();
    if logs.len() >= MAX_LOGS {
        logs.pop_front();
    }
    logs.push_back(msg.into());
}

#[derive(Default)]
pub struct DebugLogs {
    pub visible: bool,
    pub logs: Vec<String>,
}

impl DebugLogs {
    pub fn default() -> Self {
        DebugLogs {
            logs: Vec::new(),
            visible: false,
        }
    }

    fn drain_logs(&mut self) {
        let logs = DEBUG_LOGS.lock().unwrap();
        self.logs = logs.iter().cloned().collect();
    }
}

#[derive(Debug)]
pub enum DebugLogsEvent {
    ToggleVisibility,
}

pub fn toggle_debug_logs() -> InputHandled {
    handled(DebugLogsEvent::ToggleVisibility.into())
}

pub fn on_tick(state: &mut App) -> Option<AppEvent> {
    if state.ticks % 5 != 0 {
        return None;
    }
    state.debug_logs.drain_logs();
    None
}

pub fn on_event(state: &mut DebugLogs, event: DebugLogsEvent) {
    match event {
        DebugLogsEvent::ToggleVisibility => on_logs_visibility_toggle(state),
    }
}

fn on_logs_visibility_toggle(state: &mut DebugLogs) {
    state.visible = !state.visible;
}

pub fn draw(state: &DebugLogs, f: &mut Frame, area: Rect) {
    let area = area.inner(Margin {
        vertical: 0,
        horizontal: 3,
    });
    if area.is_empty() || !state.visible {
        return;
    }

    let lines: Vec<Line> = state
        .logs
        .iter()
        .map(|msg| Line::raw(msg.clone()))
        .collect();
    let par = Paragraph::new(Text::from(lines)).block(Block::bordered().title("Logs"));

    f.render_widget(par, area);
}
