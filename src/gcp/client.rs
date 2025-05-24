use std::collections::HashMap;

use crate::app::App;
use crate::event::Event;
use google_cloud_pubsub::client::{Client, ClientConfig};
use google_cloud_pubsub::subscription::{Subscription, SubscriptionConfig};
use tokio::sync::mpsc::Sender;

struct GcpFakeSubscriptionPool {
    project_id: String,
    client: Client,
    subscriptions: HashMap<String, Subscription>,
}

impl GcpFakeSubscriptionPool {
    pub async fn new(project_id: &str) -> anyhow::Result<Self> {
        let mut config = ClientConfig::default();
        config.project_id = Some(project_id.to_string());
        let client = Client::new(config).await?;
        Ok(Self {
            project_id: project_id.to_string(),
            client,
            subscriptions: HashMap::new(),
        })
    }

    pub async fn get(&mut self, topic_id: &str) -> anyhow::Result<Subscription> {
        if let Some(sub) = self.subscriptions.get(topic_id) {
            return Ok(sub.clone());
        }
        let sub_id = format!("{}-fake-sub", topic_id);
        let topic = self.client.topic(topic_id);
        let sub_config = SubscriptionConfig::default();
        let subscription = self
            .client
            .create_subscription(&sub_id, topic.fully_qualified_name(), sub_config, None)
            .await?;
        self.subscriptions
            .insert(topic_id.to_string(), subscription);
        match self.subscriptions.get(topic_id) {
            Some(sub) => Ok(sub.clone()),
            None => {
                return Err(anyhow::anyhow!("Failed to create subscription"));
            }
        }
    }
}

pub async fn run(_tx: Sender<Event>, _app: &App) {
    loop {
        // match sub.pull(None, Some(10)).await {
        //     Ok(messages) => {
        //         let parsed = messages
        //             .into_iter()
        //             .map(|mut m| {
        //                 let data = String::from_utf8_lossy(&m.message.data).to_string();
        //                 let ack_id = m.ack_id.clone();
        //                 m.ack().await.ok();
        //                 PubSubMessage { data, ack_id }
        //             })
        //             .collect::<Vec<_>>();

        //         if tx.send(Event::Gcp(GcpMsg::Messages(parsed))).await.is_err() {
        //             break;
        //         }
        //     }
        //     Err(e) => {
        //         eprintln!("pull error: {}", e);
        //         tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        //     }
        // }
    }
}
