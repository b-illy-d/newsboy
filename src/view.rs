use ratatui::{
    layout::{Alignment::Right, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    app::App,
    component::{debug, header, setup, topics},
    route::Route,
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
            setup::draw(&state.setup, f, area);
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
    let border = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().light_green());
    f.render_widget(border, area);

    let [app_area, static_area] = Layout::horizontal([Constraint::Min(0), Constraint::Length(50)])
        .margin(1)
        .areas(area);

    let app_help_paragraph =
        Paragraph::new(state.help_text.clone()).style(Style::default().fg(Color::Yellow));
    f.render_widget(app_help_paragraph, app_area);

    let static_paragraph = Paragraph::new("Press ? for help ".to_string())
        .style(Style::default().fg(Color::Yellow))
        .alignment(Right);

    f.render_widget(static_paragraph, static_area);
}
