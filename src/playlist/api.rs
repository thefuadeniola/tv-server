use axum::{extract::{State, Path}, http::StatusCode, Json, response::IntoResponse};
use mongodb::{bson::{doc, oid::ObjectId, to_document, Document}, Collection, Database};
use super::*;

pub async fn add_a_playlist(State(db): State<Database>, Json(body): Json<AddPlaylist>) -> impl IntoResponse {
    let playlists_coll: Collection<Document> = db.collection("playlists");

    let insert_struct = Playlist {
        id: None,
        title: body.title.clone(),
        links: body.links.clone(),
        spotify_playlist_id: body.spotify_playlist_id.clone()
    };

    let insert_doc = to_document(&insert_struct).unwrap();
    let res = playlists_coll.insert_one(insert_doc).await;

    match res {
        Ok(resp) => {
            let id = resp.inserted_id.as_object_id().unwrap();
            (StatusCode::CREATED, Json(doc! { "inserted_id": id })).into_response()
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }

}