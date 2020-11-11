use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscriptionRequest {
    pub msgid: String,
    pub pk: String,
}