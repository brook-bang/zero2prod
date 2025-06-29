use crate::domain::{subscriber_name::SubscriberName, SubscriberEmail};

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
