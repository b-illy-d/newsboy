use crate::{
    component::{setting_project_id::SettingProjectId, topics::TopicInfo},
    event::AppEvent,
};
use google_cloud_pubsub::client::{Client, ClientConfig};

#[derive(Default)]
pub struct Pubsub {
    client: Option<Client>,
    pub project_id: Option<String>,
    pub topics: Vec<TopicInfo>,
    pub setting_project_id: SettingProjectId,
}

pub enum PubsubEvent {
    ProjectId(ProjectIdEvent),
    Topics(Vec<TopicInfo>),
}

pub enum ProjectIdEvent {
    StartSetting,
    Input(String),
    FinishSetting(Option<String>),
}

impl Pubsub {
    pub async fn new(project_id: String) -> anyhow::Result<Self> {
        let mut config = ClientConfig::default();
        config.project_id = Some(project_id.clone());
        let client = Client::new(config).await?;
        Ok(Self {
            client: Some(client),
            project_id: Some(project_id.to_string()),
            topics: Vec::new(),
            setting_project_id: SettingProjectId::default(),
        })
    }

    pub async fn list_topics(&mut self) -> anyhow::Result<Vec<TopicInfo>> {
        let topics = self
            .client
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Client not initialized"))?
            .get_topics(None)
            .await?;
        let topic_ids: Vec<TopicInfo> = topics
            .iter()
            .map(|t| TopicInfo {
                name: t.to_string(),
            })
            .collect();
        Ok(topic_ids)
    }
}

pub async fn on_event(state: &mut Pubsub, e: PubsubEvent) -> Option<AppEvent> {
    match e {
        PubsubEvent::ProjectId(e) => on_project_id_event(state, e).await,
        PubsubEvent::Topics(topics) => {
            state.topics = topics;
            None
        }
    }
}

async fn on_project_id_event(state: &mut Pubsub, e: ProjectIdEvent) -> Option<AppEvent> {
    match e {
        ProjectIdEvent::StartSetting => {
            state.setting_project_id.active = true;
            state.setting_project_id.input.clear();
            None
        }
        ProjectIdEvent::Input(input) => {
            state.setting_project_id.input = input;
            None
        }
        ProjectIdEvent::FinishSetting(project_id) => {
            state.setting_project_id.input.clear();
            state.setting_project_id.active = false;
            match project_id {
                Some(ref id) if id.is_empty() => {
                    state.project_id = None;
                    None
                }
                Some(ref id) => {
                    state.project_id = Some(id.clone());
                    None
                }
                None => {
                    state.project_id = None;
                    None
                }
            }
        }
    }
}
