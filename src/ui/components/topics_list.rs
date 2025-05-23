use crate::app::App;

pub struct TopicsList {
    pub app: App,
    pub is_active: bool,
    pub topics: Vec<TopicInfo>,
    pub visible_topics: Vec<TopicInfo>,
    pub selected_topic_index: Option<usize>,
    pub filter_active: bool,
    pub filter_text: String,
}

impl TopicsList {
    pub fn new(app: &App) -> Result<Self> {
        Ok(Self {
            app: app,
            is_active: false,
            topics: Vec::new(),
            visible_topics: Vec::new(),
            selected_topic_index: None,
            filter_active: false,
            filter_text: String::new(),
        })
    }

    async fn refresh_topics(&mut self) -> Result<()> {
        self.set_status_text("Refreshing...");
        let topics = self.pubsub_client.list_topics().await?;
        self.app
            .set_status_text(&format!("Found {} Topics", topics.len()));
        self.topics = topics;
        self.filter_and_sort_topics();
        Ok(())
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
        if let Some(index) = self.selected_topic_index {
            if index >= self.visible_topics.len() {
                self.selected_topic_index = if self.visible_topics.is_empty() {
                    None
                } else {
                    Some(self.visible_topics.len() - 1)
                };
            }
        }
    }

    fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let topics: Vec<ListItem> = app
            .visible_topics
            .iter()
            .map(|topic| ListItem::new(topic.name.clone()))
            .collect();

        let filter_title = if app.filter_active {
            format!(" Topics [ Search: {} ] ", app.filter_text)
        } else {
            " Topics ".to_string()
        };

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

        let mut list_state = ratatui::widgets::ListState::default();
        list_state.select(app.selected_topic_index);
        f.render_stateful_widget(topics_list, area, &mut list_state);
    }
}
