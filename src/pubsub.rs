use anyhow::{anyhow, Context, Result};
use google_cloud_pubsub::client::{Client, ClientConfig};
use log::{debug, error};
use std::collections::HashMap;
use std::env;

pub struct PubSubClient {
    client: Client,
    project_id: String,
}

#[derive(Debug, Clone)]
pub struct TopicInfo {
    pub name: String,
    pub full_name: String,
    pub labels: HashMap<String, String>,
}

impl PubSubClient {
    pub async fn new(project_id: &str) -> Result<Self> {
        // Verify emulator is configured
        let emulator_host = env::var("PUBSUB_EMULATOR_HOST")
            .context("PUBSUB_EMULATOR_HOST environment variable not set")?;
        debug!("Using PubSub emulator at {}", emulator_host);

        // For emulator, use config with explicit project ID
        let mut config = ClientConfig::default();
        
        // Set the project ID in the config
        config.project_id = Some(project_id.to_string());

        let client = Client::new(config).await.context(
            "Failed to create PubSub client - is the emulator running at the specified address?",
        )?;

        debug!("PubSub client created successfully with project ID: {}", project_id);

        Ok(Self {
            client,
            project_id: project_id.to_string(),
        })
    }

    pub async fn list_topics(&self) -> Result<Vec<TopicInfo>> {
        debug!("Fetching topics for project {}", self.project_id);

        let mut result = Vec::new();

        // Check if we have a parent property set
        let parent_path = format!("projects/{}", self.project_id);
        debug!("Using parent path: {}", parent_path);

        // Get emulator host for logging
        let emulator_host = env::var("PUBSUB_EMULATOR_HOST").unwrap_or_else(|_| "<not set>".to_string());
        debug!("PubSub emulator host: {}", emulator_host);

        // List topics with the get_topics method
        let list_future = self.client.get_topics(None);

        let topics = match list_future.await {
            Ok(topics) => topics,
            Err(err) => {
                error!("PubSub error: {}", err);
                error!("Error details: {:?}", err);

                // Try to determine the specific issue
                if let Some(emulator_host) = env::var_os("PUBSUB_EMULATOR_HOST") {
                    return Err(anyhow!(
                        "Failed to connect to PubSub emulator at {}. Make sure it's running and accessible on that port", 
                        emulator_host.to_string_lossy()
                    ));
                } else {
                    return Err(anyhow!("Failed to list topics: {}", err));
                }
            }
        };

        debug!("Found {} topics", topics.len());

        for topic in topics {
            // Extract the short name from the full topic name
            let full_name = topic.to_string();
            let name = full_name
                .split('/')
                .last()
                .unwrap_or(&full_name)
                .to_string();

            debug!("Found topic: {}", full_name);

            // Get topic labels if available - default to empty hashmap
            let labels = HashMap::new();

            result.push(TopicInfo {
                name,
                full_name,
                labels,
            });
        }

        if result.is_empty() {
            debug!("No topics found - check project ID and emulator connection");
        }

        Ok(result)
    }
}
