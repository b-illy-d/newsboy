use crate::event::Event;
use crate::gcp::models::{GcpMsg, PubSubMessage};
use google_cloud_pubsub::client::{Client, ClientConfig};
use google_cloud_pubsub::subscription::Subscription;
use tokio::sync::mpsc::Sender;

pub struct PubSubClient {
    sub_id: String,
    client: Client,
}

impl PubSubClient {
    pub async fn new() -> anyhow::Result<Self> {
        let mut config = ClientConfig::default();

        let emulator_host = env::var("PUBSUB_EMULATOR_HOST")
            .context("PUBSUB_EMULATOR_HOST environment variable not set")?;
        debug!("Using PubSub emulator at {}", emulator_host);

        let client = client::new(config).await.context(
            "Failed to create pubsub client - is the emulator running at the specified address?",
        )?;

        Ok(Self {
            sub_id: format!("projects/{}/subscriptions/{}", project_id, sub_id),
            client,
        })
    }
}
