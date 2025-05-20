use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::pubsub::{PubSubClient, TopicInfo};
use crate::ui;

pub struct App {
    pub pubsub_client: PubSubClient,
    pub topics: Vec<TopicInfo>,
    pub visible_topics: Vec<TopicInfo>,
    pub selected_topic_index: Option<usize>,
    pub should_quit: bool,
    pub filter_active: bool,
    pub filter_text: String,
    pub project_id: String,
    pub show_help: bool,
    pub show_console: bool,
    pub debug_logs: Vec<String>,
    pub status_text: String,
}

impl App {
    pub fn new(project_id: &str, pubsub_client: PubSubClient) -> Result<Self> {
        Ok(Self {
            pubsub_client,
            topics: Vec::new(),
            visible_topics: Vec::new(),
            selected_topic_index: None,
            should_quit: false,
            filter_active: false,
            filter_text: String::new(),
            project_id: project_id.to_owned(),
            show_help: false,
            show_console: false,
            debug_logs: Vec::new(),
            status_text: String::new(),
        })
    }

    async fn refresh_topics(&mut self) -> Result<()> {
        self.set_status_text("Refreshing...");
        let topics = self.pubsub_client.list_topics().await?;
        self.set_status_text(&format!("Found {} Topics", topics.len()));
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
            if index >= self.topics.len() {
                self.selected_topic_index = if self.topics.is_empty() {
                    None
                } else {
                    Some(self.topics.len() - 1)
                };
            }
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        // Set up terminal
        crossterm::terminal::enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen,)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Run the app with proper error handling
        let result = self.run_app(&mut terminal).await;

        // Always clean up terminal, even if app returns an error
        self.restore_terminal(&mut terminal)?;

        // Return any errors from running the app
        result
    }

    async fn run_app<B: tui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<()> {
        // Initial topics refresh with error handling
        if let Err(err) = self.refresh_topics().await {
            // Return early, but with a clean terminal
            return Err(err);
        }

        // Main event loop
        while !self.should_quit {
            // Render UI
            terminal.draw(|f| ui::draw(f, self))?;

            // Handle input with a short timeout to make the UI more responsive
            if crossterm::event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if self.filter_active {
                        if key.code == KeyCode::Enter {
                            self.filter_active = false;
                        } else if key.code == KeyCode::Backspace {
                            if !self.filter_text.is_empty() {
                                self.filter_text.pop();
                                self.filter_and_sort_topics();
                            }
                        } else if key.code == KeyCode::Esc {
                            self.filter_active = false;
                            self.filter_text.clear();
                            self.filter_and_sort_topics();
                        } else if let KeyCode::Char(c) = key.code {
                            self.filter_text.push(c);
                            self.filter_and_sort_topics();
                        }
                    } else if self.show_help {
                        if key.code == KeyCode::Esc {
                            self.show_help = false;
                        }
                    } else {
                        match key.code {
                            KeyCode::Char('c') => {
                                self.show_console = !self.show_console;
                            }
                            KeyCode::Char('j') | KeyCode::Down => {
                                self.next_topic();
                            }
                            KeyCode::Char('k') | KeyCode::Up => {
                                self.previous_topic();
                            }
                            KeyCode::Char('q') => {
                                self.should_quit = true;
                            }
                            KeyCode::Char('r') => {
                                self.refresh_topics().await?;
                            }
                            KeyCode::Char('/') => {
                                self.filter_active = !self.filter_active;
                                if !self.filter_active {
                                    self.filter_text.clear();
                                    self.refresh_topics().await?;
                                }
                            }
                            KeyCode::Char('?') => {
                                self.show_help = !self.show_help;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn restore_terminal<B: tui::backend::Backend + std::io::Write>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<()> {
        // Clean up terminal
        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(
            terminal.backend_mut(),
            crossterm::terminal::LeaveAlternateScreen
        )?;
        terminal.show_cursor()?;

        Ok(())
    }

    fn next_topic(&mut self) {
        let len = self.topics.len();
        if len == 0 {
            return;
        }

        self.selected_topic_index = Some(match self.selected_topic_index {
            Some(i) if i + 1 < len => i + 1,
            _ => 0,
        });
    }

    fn previous_topic(&mut self) {
        let len = self.topics.len();
        if len == 0 {
            return;
        }

        self.selected_topic_index = Some(match self.selected_topic_index {
            Some(i) if i > 0 => i - 1,
            _ => len - 1,
        });
    }

    fn debug_log(&mut self, message: &str) {
        self.debug_logs.push(message.to_string());
    }

    fn set_status_text(&mut self, text: &str) {
        self.status_text = text.to_string();
    }

    fn visible_topics(&self) -> Vec<&TopicInfo> {
        if !self.filter_active {
            return self.topics.iter().collect();
        }
        self.topics
            .iter()
            .filter(|topic| {
                if self.filter_active {
                    topic
                        .name
                        .to_lowercase()
                        .contains(&self.filter_text.to_lowercase())
                } else {
                    true
                }
            })
            .collect()
    }
}
