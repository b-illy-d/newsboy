use crate::app::App;
use crate::gcp::models::SubscriptionInfo;
use crate::ui::Component;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{layout::*, Frame};

impl Component for SubscriptionsComponent {
    fn init(&mut self, _app: &App) {}

    fn on_key(&mut self, key: &KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.is_active = false;
            }
            _ => {}
        }
    }

    fn view(&self, _f: &mut Frame, _area: Rect, _app: &App) {
        // Render subscriptions UI here
    }
}
pub struct SubscriptionsComponent {
    is_active: bool,
}

impl SubscriptionsComponent {
    pub fn new() -> Self {
        Self { is_active: false }
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn on_subscriptions(&mut self, _subscriptions: &[SubscriptionInfo]) {
        // Handle subscriptions data
    }
}
