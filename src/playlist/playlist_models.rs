use serde::{Serialize, Deserialize};
use mongodb::{bson::{doc, oid::ObjectId}};

#[derive(Deserialize, Serialize, Debug)]
pub struct Playlist {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub links: StreamingLinks,
    pub spotify_playlist_id: String

}

#[derive(Deserialize, Serialize, Debug, Clone)]

pub struct AddPlaylist {
    pub title: String,
    pub links: StreamingLinks,
    pub spotify_playlist_id: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]

pub struct StreamingLinks {
    spotify: String,
    apple_music: String,
    youtube_music: String
}

