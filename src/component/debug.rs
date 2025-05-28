use ratatui::{
    layout::{Margin, Rect},
    text::{Line, Text},
    widgets::{Block, Paragraph},
    Frame,
};
use std::collections::VecDeque;

use crate::event::{handled, AppEvent, InputHandled};

const MAX_LOG_LINES: usize = 100;
#[derive(Default)]
pub struct DebugLogs {
    pub logs: VecDeque<String>,
    pub visible: bool,
}

impl DebugLogs {
    pub fn default() -> Self {
        DebugLogs {
            logs: VecDeque::with_capacity(MAX_LOG_LINES),
            visible: false,
        }
    }
    pub fn log<S: Into<String>>(&mut self, msg: S) {
        if self.logs.len() >= MAX_LOG_LINES {
            self.logs.pop_front();
        }
        self.logs.push_back(msg.into());
    }
}

pub enum DebugLogsEvent {
    ToggleVisibility,
    LogMessage(String),
}

impl DebugLogsEvent {
    fn to_app_event(self) -> AppEvent {
        match self {
            DebugLogsEvent::ToggleVisibility => AppEvent::Debug(DebugLogsEvent::ToggleVisibility),
            DebugLogsEvent::LogMessage(msg) => AppEvent::Debug(DebugLogsEvent::LogMessage(msg)),
        }
    }
}

pub fn toggle_debug_logs() -> InputHandled {
    handled(DebugLogsEvent::ToggleVisibility.to_app_event())
}

pub fn debug_log(msg: String) -> AppEvent {
    DebugLogsEvent::LogMessage(msg).to_app_event()
}

pub fn on_event(state: &mut DebugLogs, event: DebugLogsEvent) {
    match event {
        DebugLogsEvent::ToggleVisibility => on_logs_visibility_toggle(state),
        DebugLogsEvent::LogMessage(msg) => on_log_message(state, msg),
    }
}

fn on_logs_visibility_toggle(state: &mut DebugLogs) {
    state.visible = !state.visible;
}

fn on_log_message(state: &mut DebugLogs, msg: String) {
    state.log(msg);
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
