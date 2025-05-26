use crate::component::topics::TopicInfo;
use google_cloud_pubsub::client::{Client, ClientConfig};

pub struct PubsubClient {
    pub project_id: String,
    client: Client,
}

impl PubsubClient {
    pub async fn new(project_id: String) -> anyhow::Result<Self> {
        let mut config = ClientConfig::default();
        config.project_id = Some(project_id.clone());
        let client = Client::new(config).await?;
        Ok(Self {
            project_id: project_id.to_string(),
            client,
        })
    }

    pub async fn list_topics(&self) -> anyhow::Result<Vec<TopicInfo>> {
        let topics = self.client.get_topics(None).await?;
        let topic_ids: Vec<TopicInfo> = topics
            .iter()
            .map(|t| TopicInfo {
                name: t.to_string(),
            })
            .collect();
        Ok(topic_ids)
    }
}
