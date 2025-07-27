use axum::{extract::{State, Path}, http::StatusCode, Json, response::IntoResponse};
use mongodb::{bson::{doc, oid::ObjectId, to_document, Document}, Collection, Database};
use super::*;
use futures::stream::TryStreamExt;

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

pub async fn fetch_all_playlists(State(db): State<Database>) -> impl IntoResponse {
    let playlists_coll: Collection<Document> = db.collection("playlists");

    let cursor_result = playlists_coll.find(doc! {}).await;

    match cursor_result {
        Ok(cursor) => {
            match cursor.try_collect::<Vec<Document>>().await {
                Ok(playlists) => (StatusCode::OK, Json(playlists)).into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            }
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn fetch_single_playlist(
    State(db): State<Database>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let object_id = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid Object Id format").into_response(),
    };

    let playlists_coll: Collection<Document> = db.collection("playlists");
    let res = playlists_coll.find_one(doc! { "_id": object_id }).await;

    match res {
        Ok(Some(playlist)) => (StatusCode::OK, Json(playlist)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Playlist not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}