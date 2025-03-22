use super::{subscriber_email::SubscriberEmail, subscriber_name::SubscriberName};

#[derive(serde::Deserialize)]
pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
