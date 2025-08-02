use serde::{Serialize, Deserialize};
use mongodb::{bson::{doc, oid::ObjectId}};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Debug, Serialize)]
pub struct NowPlaying {
    pub title: String,
    pub video_id: String,
    pub video_offset: i64,
    pub server_time: i64,
}