use serde::{Deserialize, Serialize};
use sqlx::types::{time::OffsetDateTime, uuid};

use crate::database;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: database::DB,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub age: Option<i32>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub address: Option<String>,
    pub postalcode: Option<i32>,
    pub city: Option<String>,
    pub number_phone: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUser {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateUser {
    pub id: uuid::Uuid,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub age: Option<i32>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub address: Option<String>,
    pub postalcode: Option<i32>,
    pub city: Option<String>,
    pub number_phone: Option<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateEvent {
    pub title: String,
    pub description: Option<String>,
    pub location: String,
    pub image_url: Option<String>,
    pub category: Option<String>,
    pub id_user: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetEvent {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub date: OffsetDateTime,
    pub location: String,
    pub public: bool,
    pub image_url: Option<String>,
    pub category: Option<String>,
    pub id_user: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateEvent {
    pub id: uuid::Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub image_url: Option<String>,
    pub category: Option<String>,
    pub public: Option<bool>,
}

pub struct CreateCategory {
    pub category: String,
}

pub struct GetCategory {
    pub category: String,
}
