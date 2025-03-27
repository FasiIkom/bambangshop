use std::thread;

use bambangshop::{Result, compose_error_response};
use rocket::http::Status;
use create::model::notification::Notification;
use create::model::product::Product;
use create::model::subscriber::Subscriber;
use create::repository::subscriber::SubscriberRepository;

pub struct NotificationService;

impl NotificationService {
    pub fn subscribe(product_type: &str, subscriber: Subscriber) -> Result<Subscriber> {
        let product_type_upper: String = product_type.to_uppercase();
        let product_type_str: &str = product_type_upper.as_str();
        let subscriber_result: Subscriber = SubscriberRepository::add(product_type_str, subscriber);
        return Ok(subscriber_result);
    }
}