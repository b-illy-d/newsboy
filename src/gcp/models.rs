#[derive(Clone, Debug)]
pub struct TopicInfo {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct SubscriptionInfo {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct PubsubMessage {
    pub data: String,
    pub ack_id: String,
}
