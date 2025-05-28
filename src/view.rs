use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders},
    Frame,
};

use crate::{
    app::{App, Route},
    component::{debug, header, setup, topics},
};

pub fn draw(state: &App, f: &mut Frame) {
    let area = f.area();
    let footer_h: u16 = match state.debug_logs.visible {
        true => 15,
        false => 3,
    };
    let [header_area, main_area, footer_area] = Layout::vertical([
        Constraint::Length(6),
        Constraint::Min(0),
        Constraint::Length(footer_h),
    ])
    .margin(1)
    .areas(area);
    header::draw(state, f, header_area);
    draw_main(state, f, main_area);
    draw_footer(state, f, footer_area);
}

fn draw_main(state: &App, f: &mut Frame, area: Rect) {
    match state.route {
        Route::Setup => {
            setup::draw(state, f, area);
        }
        Route::Topics => {
            topics::draw(state, f, area);
        }
    }
}

fn draw_footer(state: &App, f: &mut Frame, area: Rect) {
    if state.debug_logs.visible {
        return debug::draw(&state.debug_logs, f, area);
    }
    let footer = format!("Ticks {}", state.ticks);
    let footer_paragraph = ratatui::widgets::Paragraph::new(footer)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Yellow));

    f.render_widget(footer_paragraph, area);
}
