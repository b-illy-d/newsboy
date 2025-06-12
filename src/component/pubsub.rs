use crate::{
    component::{
        debug::debug_log,
        reusable::{
            choices::{self, Choice, Choices, ChoicesEvent, ChoicesEventType},
            text_field::{
                self, draw_simple_text_field, TextField, TextFieldEvent, TextFieldEventType,
            },
        },
        topics::{TopicInfo, Topics},
    },
    event::{send_event, AppEvent},
    input::{handled, not_handled, InputHandled, IntoHandled},
};
use google_cloud_pubsub::client::{Client, ClientConfig};
use ratatui::{
    crossterm::event::{
        KeyCode::{Down, Esc, Up},
        KeyEvent,
    },
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::collections::HashMap;

// ======================
// ==== PUBSUB STATE ====
// ======================

#[derive(Default)]
pub struct Pubsub {
    client: Option<Client>,
    pub config: PubsubConfig,
    pub status: PubsubStatus,
    pub project_id: Option<String>,
    pub topics: Topics,
}

pub struct PubsubStatus {
    pub connection: ConnectionStatus,
    pub topics: usize,
    pub info: Option<String>,
}

impl Default for PubsubStatus {
    fn default() -> Self {
        Self {
            connection: ConnectionStatus::Disconnected,
            topics: 0,
            info: None,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum ConnectionStatus {
    Connected,
    #[default]
    Disconnected,
    Connecting,
}

impl Pubsub {
    pub async fn new(
        project_id: String,
        host: String,
        port: u16,
        emulator: bool,
    ) -> anyhow::Result<Self> {
        let mut config: ClientConfig;
        if emulator {
            std::env::set_var("PUBSUB_EMULATOR_HOST", format!("{host}:{port}"));
            config = ClientConfig::default();
        } else {
            std::env::remove_var("PUBSUB_EMULATOR_HOST");
            config = ClientConfig::default();
            config.endpoint = format!("{host}:{port}")
        }
        config.project_id = Some(project_id.clone());
        let client = Client::new(config).await?;
        Ok(Self {
            client: Some(client),
            config: PubsubConfig::default(),
            status: PubsubStatus::default(),
            project_id: Some(project_id.to_string()),
            topics: Topics::new(),
        })
    }
}

// ======================
// ==== CONFIG STATE ====
// ======================

trait ConfigField {
    fn is_editing(&self) -> bool {
        false
    }
    fn name(&self) -> &String;
    fn label(&self) -> &String;
    fn as_text(&self) -> Option<&String> {
        None
    }
    fn as_choices(&self) -> Option<&[Choice]> {
        None
    }
    fn value(&self) -> &String;
    fn set_value(&mut self, value: String);
}

impl ConfigField for TextField {
    fn name(&self) -> &String {
        &self.name
    }
    fn label(&self) -> &String {
        &self.label
    }
    fn value(&self) -> &String {
        &self.value
    }
    fn as_text(&self) -> Option<&String> {
        Some(&self.value)
    }
    fn set_value(&mut self, value: String) {
        self.set_value(value);
    }
}

impl ConfigField for Choices {
    fn name(&self) -> &String {
        &self.name
    }
    fn label(&self) -> &String {
        &self.label
    }
    fn value(&self) -> &String {
        &self.value
    }
    fn as_choices(&self) -> Option<&[Choice]> {
        Some(&self.choices)
    }
    fn set_value(&mut self, value: String) {
        self.choose_index(self.choices.iter().position(|c| c.value == value));
    }
}

enum Field {
    Text(TextField),
    Choices(Choices),
}

impl ConfigField for Field {
    fn name(&self) -> &String {
        match self {
            Field::Text(f) => f.name(),
            Field::Choices(f) => f.name(),
        }
    }
    fn label(&self) -> &String {
        match self {
            Field::Text(f) => f.label(),
            Field::Choices(f) => f.label(),
        }
    }
    fn value(&self) -> &String {
        match self {
            Field::Text(f) => f.value(),
            Field::Choices(f) => f.value(),
        }
    }
    fn as_text(&self) -> Option<&String> {
        match self {
            Field::Text(f) => f.as_text(),
            _ => None,
        }
    }
    fn as_choices(&self) -> Option<&[Choice]> {
        match self {
            Field::Choices(f) => f.as_choices(),
            _ => None,
        }
    }
    fn set_value(&mut self, value: String) {
        match self {
            Field::Text(f) => f.set_value(value),
            Field::Choices(f) => f.set_value(value),
        }
    }
    fn is_editing(&self) -> bool {
        match self {
            Field::Text(f) => f.is_editing,
            Field::Choices(f) => f.is_editing,
        }
    }
}

pub struct PubsubConfig {
    fields: HashMap<String, Field>,
    pub focused: Option<String>,
}

impl Default for PubsubConfig {
    fn default() -> Self {
        let mut fields = HashMap::new();
        fields.insert(
            "project_id".to_string(),
            Field::Text(TextField::new("project_id".into(), "Project ID".into())),
        );
        fields.insert(
            "host".to_string(),
            Field::Text(TextField::new("host".into(), "Host".into())),
        );
        fields.insert(
            "port".to_string(),
            Field::Text(TextField::new("port".into(), "Port".into())),
        );
        fields.insert(
            "emulator".to_string(),
            Field::Choices(Choices::new(
                "emulator".into(),
                "Emulator".into(),
                vec![
                    Choice {
                        label: "Yes".into(),
                        value: "true".into(),
                    },
                    Choice {
                        label: "No".into(),
                        value: "false".into(),
                    },
                ],
            )),
        );
        PubsubConfig {
            fields,
            focused: None,
        }
    }
}

const DEFAULT_FIELD_VALUES: &[(&str, &str)] = &[
    ("project_id", ""),
    ("host", "localhost"),
    ("port", "8065"),
    ("emulator", "false"),
];

const DEFAULT_FIELD_ORDER: &[&str] = &["project_id", "host", "port", "emulator"];

impl PubsubConfig {
    pub fn get(&self, name: &str) -> &String {
        let field = self.fields.get(name).unwrap_or_else(|| {
            panic!("Unknown setting: {}", name);
        });
        field.value()
    }

    pub fn set(&mut self, name: &str, value: String) {
        let field = self.fields.get_mut(name).unwrap_or_else(|| {
            panic!("Unknown setting: {}", name);
        });
        field.set_value(value);
    }

    fn get_text_field(&self, name: &str) -> &TextField {
        match self.fields.get(name) {
            Some(Field::Text(f)) => f,
            _ => panic!("Field {} is not a Text field", name),
        }
    }

    fn get_choices_field(&self, name: &str) -> &Choices {
        match self.fields.get(name) {
            Some(Field::Choices(f)) => f,
            _ => panic!("Field {} is not a Choices field", name),
        }
    }

    fn get_text_field_mut(&mut self, name: &str) -> &mut TextField {
        match self.fields.get_mut(name) {
            Some(Field::Text(f)) => f,
            _ => panic!("Field {} is not a Text field", name),
        }
    }

    fn get_choices_field_mut(&mut self, name: &str) -> &mut Choices {
        match self.fields.get_mut(name) {
            Some(Field::Choices(f)) => f,
            _ => panic!("Field {} is not a Choices field", name),
        }
    }
}

pub fn init_config(state: &mut PubsubConfig) {
    for (name, value) in DEFAULT_FIELD_VALUES {
        state.set(name, value.to_string());
    }
}

// =======================
// ==== PUBSUB EVENTS ====
// =======================

#[derive(Debug, Clone)]
pub enum PubsubEvent {
    Connect,
    GetTopics,
    Config(ConfigEvent),
    ChangeProjectId(String),
    GotTopics(Vec<TopicInfo>),
}

pub fn set_project_id(id: String) -> PubsubEvent {
    PubsubEvent::ChangeProjectId(id)
}

async fn connect_to_pubsub() {
    send_event(PubsubEvent::Connect.into()).await
}

// =======================
// ==== CONFIG EVENTS ====
// =======================

#[derive(Debug, Clone)]
pub enum FieldEvent {
    TextFieldEvent(TextFieldEvent),
    ChoicesEvent(ChoicesEvent),
}

impl From<TextFieldEvent> for FieldEvent {
    fn from(event: TextFieldEvent) -> Self {
        FieldEvent::TextFieldEvent(event)
    }
}

impl From<ChoicesEvent> for FieldEvent {
    fn from(event: ChoicesEvent) -> Self {
        FieldEvent::ChoicesEvent(event)
    }
}

#[derive(Debug, Clone)]
pub enum ConfigEvent {
    ConfigFieldEvent(FieldEvent),
    Focus(Option<String>),
}

impl From<FieldEvent> for ConfigEvent {
    fn from(event: FieldEvent) -> Self {
        ConfigEvent::ConfigFieldEvent(event)
    }
}

fn focus(name: &str) -> ConfigEvent {
    ConfigEvent::Focus(Some(name.to_string()))
}

fn unfocus() -> ConfigEvent {
    ConfigEvent::Focus(None)
}

// ========================
// ==== EVENT HANDLERS ====
// ========================

pub async fn on_event(state: &mut Pubsub, e: PubsubEvent) -> Option<AppEvent> {
    match e {
        PubsubEvent::Connect => {
            on_connect_to_pubsub(state).await;
            None
        }
        PubsubEvent::ChangeProjectId(id) => {
            on_change_project_id(state, id).await;
            None
        }
        PubsubEvent::Config(event) => {
            on_config_event(&mut state.config, event).await;
            None
        }
        PubsubEvent::GetTopics => on_get_topics(state).await,
        PubsubEvent::GotTopics(topics) => {
            state.status.topics = topics.len();
            state.topics.set_topics(topics);
            None
        }
    };
    None
}

async fn on_change_project_id(state: &mut Pubsub, id: String) {
    if let Some(curr_project) = state.project_id.clone() {
        if curr_project == id {
            state.status.info = Some("Already connected to this project".to_string());
            return;
        }
    }
    state.project_id = Some(id);
    state.status.connection = ConnectionStatus::Connecting;
    connect_to_pubsub().await;
}

async fn on_connect_to_pubsub(state: &mut Pubsub) {
    if let Some(id) = state.project_id.as_ref() {
        match Pubsub::new(id.clone(), "localhost".to_string(), 8065, true).await {
            Ok(pubsub) => {
                state.client = pubsub.client;
                state.status.connection = ConnectionStatus::Connected;
                state.status.info = Some("Connected to Pub/Sub".to_string());
            }
            Err(e) => {
                state.client = None;
                state.status.connection = ConnectionStatus::Disconnected;
                state.status.info = Some(format!("Failed to connect: {}", e));
            }
        };
    } else {
        debug_log("Project ID is empty!!".to_string());
        state.status.info = Some("Project ID is empty".to_string());
        return;
    }
}

async fn on_get_topics(state: &mut Pubsub) -> Option<AppEvent> {
    if let Some(client) = &state.client {
        match client.get_topics(None).await {
            Ok(topics) => {
                let topic_infos: Vec<TopicInfo> =
                    topics.into_iter().map(|t| TopicInfo { name: t }).collect();
                Some(PubsubEvent::GotTopics(topic_infos).into())
            }
            Err(e) => {
                state.status.connection = ConnectionStatus::Disconnected;
                state.status.info = Some(format!("Failed to get topics: {}", e));
                None
            }
        }
    } else {
        state.status.info = Some("Not connected to Pub/Sub".to_string());
        None
    }
}

pub async fn on_config_event(state: &mut PubsubConfig, e: ConfigEvent) {
    match e {
        ConfigEvent::ConfigFieldEvent(e) => on_config_field_event(state, e).await,
        ConfigEvent::Focus(name) => {
            state.focused = name.clone();
        }
    };
}

async fn on_config_field_event(state: &mut PubsubConfig, e: FieldEvent) {
    let next_event = match e {
        FieldEvent::TextFieldEvent(e) => {
            if matches!(e.event_type, TextFieldEventType::ValueChanged) {
                on_config_value_changed(state, &e.name).await;
            }
            let field = state.get_text_field_mut(&e.name);
            text_field::on_event(field, e.event_type)
                .map(FieldEvent::from)
                .map(ConfigEvent::from)
                .map(AppEvent::from)
        }
        FieldEvent::ChoicesEvent(e) => {
            if matches!(e.event_type, ChoicesEventType::ValueChanged) {
                on_config_value_changed(state, &e.name).await;
            }
            let field = state.get_choices_field_mut(&e.name);
            choices::on_event(field, e.event_type)
                .map(FieldEvent::from)
                .map(ConfigEvent::from)
                .map(AppEvent::from)
        }
    };
    if let Some(event) = next_event {
        send_event(event).await;
    }
}

async fn on_config_value_changed(state: &mut PubsubConfig, name: &String) {
    if name == "project_id" {
        send_event(set_project_id(state.get(name).clone()).into()).await;
    }
}

// ===============
// ==== INPUT ====
// ===============

pub fn on_key(state: &PubsubConfig, key: KeyEvent) -> InputHandled<AppEvent> {
    let text_handled = on_text_field_key(state, key)
        .map(ConfigEvent::from)
        .map(AppEvent::from);
    if text_handled.is_handled() {
        return text_handled;
    }

    match key.code {
        Up | Down => on_arrow_key(state, key).into_handled(),
        Esc => {
            if state.focused.is_some() {
                handled(unfocus().into())
            } else {
                not_handled()
            }
        }
        _ => not_handled(),
    }
}

fn on_text_field_key(state: &PubsubConfig, key: KeyEvent) -> InputHandled<FieldEvent> {
    if let Some(ref focused) = state.focused {
        match state.fields.get(focused) {
            Some(Field::Text(field)) => text_field::on_key(field, key).map(FieldEvent::from),
            Some(Field::Choices(field)) => choices::on_key(field, key).map(FieldEvent::from),
            None => not_handled(),
        }
    } else {
        not_handled()
    }
}

fn on_arrow_key(state: &PubsubConfig, key: KeyEvent) -> InputHandled<ConfigEvent> {
    let field_names = DEFAULT_FIELD_ORDER;

    if matches!(state.focused, None) {
        return handled(focus(field_names[0]).into());
    }

    let current_index = field_names
        .iter()
        .position(|n| n == &state.focused.clone().unwrap())
        .unwrap_or(0);

    match key.code {
        Up => {
            let next_index = if current_index == 0 {
                field_names.len() - 1
            } else {
                current_index - 1
            };
            debug_log(format!(
                "Next idx {} next field {}",
                next_index, field_names[next_index]
            ));
            handled(focus(field_names[next_index]))
        }
        Down => {
            let next_index = (current_index + 1) % field_names.len();
            debug_log(format!(
                "Next idx {} next field {}",
                next_index, field_names[next_index]
            ));
            handled(focus(field_names[next_index]))
        }
        _ => not_handled(),
    }
}

// ==============
// ==== VIEW ====
// ==============

const TITLE: &str = "Config";
const VIEWING_HELP: &str = "↑/↓ to navigate, Spacebar to edit";
const EDITING_HELP: &str = "Editing: Press Enter to save, Esc to cancel";
pub fn draw_config_page(state: &PubsubConfig, f: &mut Frame, area: Rect) {
    let is_editing = match state.focused {
        None => false,
        Some(ref name) => state.fields.get(name).unwrap().is_editing(),
    };
    let help_text = Text::from(vec![
        Line::default(),
        Line::from(match is_editing {
            true => EDITING_HELP.to_string(),
            false => VIEWING_HELP.to_string(),
        }),
        Line::default(),
    ])
    .style(Style::default().fg(Color::Gray));

    let block = Block::default()
        .title(TITLE.to_string())
        .light_blue()
        .fg(Color::LightCyan)
        .bg(Color::Black)
        .borders(Borders::ALL);
    f.render_widget(block, area);

    let [content_area] = Layout::default()
        .vertical_margin(1)
        .horizontal_margin(2)
        .constraints([Constraint::Length(100)])
        .direction(Direction::Horizontal)
        .areas(area);
    let [help_text_area, fields_area] = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .direction(Direction::Vertical)
        .areas(content_area);
    f.render_widget(Paragraph::new(help_text), help_text_area);
    draw_fields(state, f, fields_area);
}

fn draw_fields(state: &PubsubConfig, f: &mut Frame, area: Rect) {
    let field_names = DEFAULT_FIELD_ORDER;
    fn width(name: &str) -> u16 {
        match name {
            "port" => 10,
            "emulator" => 10,
            _ => 80,
        }
    }
    for (i, name) in field_names.iter().enumerate() {
        let field_area = Rect::new(area.x, area.y + i as u16 * 3, width(name), 1);
        let is_focused = match &state.focused {
            Some(n) => n == name,
            None => false,
        };
        let field = state.fields.get(*name).unwrap();
        match field {
            Field::Text(t) => {
                draw_simple_text_field(t, is_focused, f, field_area);
            }
            Field::Choices(c) => {
                choices::draw(c, is_focused, f, field_area);
            }
        }
    }
}

pub fn draw_pubsub_status(state: &Pubsub, f: &mut Frame, area: Rect) {
    use ratatui::widgets::{Paragraph, Wrap};

    let status_text = match &state.status.connection {
        ConnectionStatus::Connected => "Connected",
        ConnectionStatus::Disconnected => "Disconnected",
        ConnectionStatus::Connecting => "Connecting...",
    };

    let info_text = state
        .status
        .info
        .clone()
        .unwrap_or_else(|| "No info".to_string());
    let topics_count = state.status.topics;

    let paragraph = Paragraph::new(format!(
        "Status: {} Topics: {} Info: {}",
        status_text, topics_count, info_text
    ))
    .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}
