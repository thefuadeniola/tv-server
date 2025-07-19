use serde::{Serialize, Deserialize};
use mongodb::{bson::{doc, oid::ObjectId}};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct Show {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub total_duration: Duration,
    pub videos: Vec<String>
}


#[derive(Deserialize, Serialize)]
pub struct CreateShow {
    pub title: String,
    pub minutes: i32,
    pub video_urls: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateShow {
    pub video_urls: Vec<String>
}

