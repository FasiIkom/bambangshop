use std::thread;

use bambangshop::{Result, compose_error_response}
use rocket::http::Status;
use create:model::notification::Notification;
use create::model::product::Product;
use create::model::subscriber::Subscriber;
use create::repository:subscriber::SubscriberRepository;

pub struct NotificationService;

impl NotificationService {
    
}