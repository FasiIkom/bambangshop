use rocket::response::status::Created;
use rocket::serde::json::Json;

use bambangshop::Result;
use create::model::subscriber::Subscriber;
use create::service::notification::NotificationService;