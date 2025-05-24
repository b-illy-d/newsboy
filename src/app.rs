use crate::event::Event;
use crate::gcp::GcpMsg;
use crate::ui::{Component, SubscriptionsComponent, TopicsComponent};
use crossterm::event::{KeyCode, KeyEvent};
use std::time::Instant;
use tokio::sync::mpsc::Sender;

pub enum Route {
    Topics,
    Subscriptions,
}

pub struct App {
    pub route: Route,
    pub topics: TopicsComponent,
    pub subscriptions: SubscriptionsComponent,
    pub ticks: u64,
    pub last_tick: Instant,
    pub tx: Sender<Event>,
}

impl App {
    pub fn new(tx: Sender<Event>) -> Self {
        Self {
            route: Route::Topics,
            topics: TopicsComponent::new(),
            subscriptions: SubscriptionsComponent::new(),
            ticks: 0,
            last_tick: Instant::now(),
            tx,
        }
    }

    pub fn on_tick(&mut self) {
        self.ticks += 1;
        self.last_tick = Instant::now();
    }

    pub async fn on_key(&mut self, key: &KeyEvent) {
        match self.route {
            Route::Topics => self.topics.on_key(key),
            Route::Subscriptions => self.subscriptions.on_key(key),
        }
    }

    pub fn on_pubsub(&mut self, event: &GcpMsg) {
        match event {
            GcpMsg::Topics(topics) => {
                self.topics.on_topics(topics);
            }
            GcpMsg::Subscriptions(subscriptions) => {
                self.subscriptions.on_subscriptions(subscriptions);
            }
            _ => {} // GcpMsg::Messages(message) => {
                    //     self.messages.on_pubsub(message);
                    // }
        }
    }

    async fn on_key_global(&mut self, key: &KeyEvent) {
        match key.code {
            KeyCode::Char('c') if key.modifiers == crossterm::event::KeyModifiers::CONTROL => {
                // Ctrl+C to quit
                self.tx.send(Event::Quit).await.unwrap();
            }
            KeyCode::Char('q') => {
                self.tx.send(Event::Quit).await.unwrap();
            }
            KeyCode::Char('t') => {
                self.route = Route::Topics;
            }
            KeyCode::Char('s') => {
                self.route = Route::Subscriptions;
            }
            _ => {}
        }
    }
}
