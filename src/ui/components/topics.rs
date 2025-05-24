use std::cmp::max;

use crate::app::App;
use crate::gcp::models::*;
use crate::ui::Component;
use crossterm::event::KeyCode::*;
use crossterm::event::KeyEvent;
use ratatui::{layout::*, Frame};
use ratatui::{
    prelude::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

impl Component for TopicsComponent {
    fn init(&mut self, _app: &App) {}

    fn on_key(&mut self, key: &KeyEvent) {
        match key.code {
            Char('j') => self.down(),
            Char('k') => self.up(),
            _ => {}
        }
    }

    fn view(&self, f: &mut Frame, area: Rect, _app: &App) {
        // build list items
        let topics: Vec<ListItem> = self
            .visible_topics
            .iter()
            .map(|topic| ListItem::new(topic.name.clone()))
            .collect();

        // title text
        let filter_title = if self.filter_active {
            format!(" Topics [ Search: {} ] ", self.filter_text)
        } else {
            " Topics ".to_string()
        };

        // full list widget
        let topics_list = List::new(topics)
            .block(Block::default().title(filter_title).borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::LightCyan)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        // selection state
        let mut list_state = ListState::default();
        let selected = if self.visible_topics.len() > 0 {
            Some(self.selected)
        } else {
            None
        };
        list_state.select(selected);

        // render
        f.render_stateful_widget(topics_list, area, &mut list_state);
    }
}

pub struct TopicsComponent {
    pub is_active: bool,
    pub topics: Vec<TopicInfo>,
    pub visible_topics: Vec<TopicInfo>,
    pub selected: usize,
    pub filter_active: bool,
    pub filter_text: String,
}

impl TopicsComponent {
    pub fn new() -> Self {
        Self {
            is_active: false,
            topics: Vec::new(),
            visible_topics: Vec::new(),
            selected: 0,
            filter_active: false,
            filter_text: String::new(),
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn on_topics(&mut self, topics: &Vec<TopicInfo>) {
        self.topics = topics.clone();
        self.filter_and_sort_topics();
    }

    fn filter_and_sort_topics(&mut self) {
        if !self.filter_active || self.filter_text.is_empty() {
            self.visible_topics = self.topics.clone();
            self.visible_topics.sort_by(|a, b| a.name.cmp(&b.name));
            return;
        }
        let filter_text = self.filter_text.to_lowercase();
        self.visible_topics = self
            .topics
            .iter()
            .filter(|topic| topic.name.to_lowercase().contains(&filter_text))
            .cloned()
            .collect();
        self.visible_topics.sort_by(|a, b| a.name.cmp(&b.name));
        self.update_selected_topic_index();
    }

    fn update_selected_topic_index(&mut self) {
        if self.selected >= self.visible_topics.len() {
            self.selected = max(0, self.visible_topics.len() - 1);
        }
    }

    fn down(&mut self) {
        if self.selected == self.visible_topics.len() - 1 {
            self.selected = 0;
        } else {
            self.selected += 1;
        }
    }

    fn up(&mut self) {
        if self.selected == 0 {
            self.selected = self.visible_topics.len() - 1;
        } else {
            self.selected -= 1;
        }
    }
}
