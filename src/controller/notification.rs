use rocket::serde::json::Json;

use bambangshop_receiver::Result;
use crate::Model::notification::Notification;
use crate::model::subscribe::SubscriberRequest;
use crate::service::notification::NotificationService;
