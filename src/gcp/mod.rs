pub mod client;
pub mod models;
use crate::gcp::models::{PubsubMessage, SubscriptionInfo, TopicInfo};

#[derive(Debug, Clone)]
pub enum GcpMsg {
    Topics(Vec<TopicInfo>),
    Subscriptions(Vec<SubscriptionInfo>),
    Messages(Vec<PubsubMessage>),
}

pub struct Pubsub {}
impl Pubsub {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self) {}
}
