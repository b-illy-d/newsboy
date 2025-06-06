use crate::app::App;
use crate::component::debug::debug_log;
use crate::event::AppEvent;
use ratatui::{style::Stylize, text::Line};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, FromRepr};

#[derive(Debug, Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum Route {
    #[default]
    #[strum(serialize = "Config")]
    Config,
    #[strum(serialize = "Topics")]
    Topics,
}

impl Route {
    pub fn titles() -> Vec<Line<'static>> {
        Route::iter()
            .enumerate()
            .map(|(i, r)| Line::from(format!("[{}] {}", i + 1, r)).light_blue())
            .collect()
    }

    pub fn next(self) -> Self {
        let index = (self as usize + 1) % Self::iter().len();
        Self::from_repr(index).unwrap()
    }

    pub fn previous(self) -> Self {
        let count = Self::iter().len();
        let index = (self as usize + count - 1) % count;
        let prev = Self::from_repr(index).unwrap();
        debug_log(format!("Previous route: {:?}", prev));
        prev
    }
}

#[derive(Debug, Clone)]
pub enum RouteEvent {
    Select(Route),
    Next,
    Prev,
}

pub fn previous_route() -> AppEvent {
    RouteEvent::Prev.into()
}

pub fn next_route() -> AppEvent {
    RouteEvent::Next.into()
}

pub fn select_route(route: Route) -> AppEvent {
    RouteEvent::Select(route).into()
}

pub fn on_event(state: &mut App, event: RouteEvent) -> Option<AppEvent> {
    let new_route = match event {
        RouteEvent::Select(route) => route,
        RouteEvent::Next => state.route.next(),
        RouteEvent::Prev => state.route.previous(),
    };
    state.route = new_route;
    None
}
