use crate::{component::topics::TopicInfo, event::AppEvent};
use google_cloud_pubsub::client::{Client, ClientConfig};

#[derive(Default)]
pub struct PubsubState {
    client: Option<Client>,
    pub project_id: Option<String>,
    pub topics: Vec<TopicInfo>,
}

#[derive(Debug, Clone)]
pub enum PubsubEvent {
    Topics(Vec<TopicInfo>),
}

pub enum ProjectIdEvent {
    StartSetting,
    Input(String),
    FinishSetting(Option<String>),
}

impl PubsubState {
    pub async fn new(project_id: String) -> anyhow::Result<Self> {
        let mut config = ClientConfig::default();
        config.project_id = Some(project_id.clone());
        let client = Client::new(config).await?;
        Ok(Self {
            client: Some(client),
            project_id: Some(project_id.to_string()),
            topics: Vec::new(),
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

pub async fn on_event(state: &mut PubsubState, e: PubsubEvent) -> Option<AppEvent> {
    match e {
        PubsubEvent::Topics(topics) => {
            state.topics = topics;
            None
        }
    }
}
