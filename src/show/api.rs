use axum::{extract::{State, Path}, http::StatusCode, Json, response::IntoResponse};
use mongodb::{bson::{doc, oid::ObjectId, to_document, Document}, Collection, Database};
use std::time::Duration;
use super::*;
use futures::stream::TryStreamExt;

pub async fn create_a_show(State(db): State<Database>, Json(body): Json<CreateShow>) -> impl IntoResponse { 

    let shows_coll: Collection<Document> = db.collection("shows");

    let insert_struct = Show {
        id: None,
        title: body.title.clone(),
        total_duration: Duration::from_secs((&body.minutes * 60) as u64),
        videos: body.video_urls.clone()
    };

    let insert_doc = to_document(&insert_struct).unwrap();
    let res = shows_coll.insert_one(insert_doc).await;

    match res {
        Ok(resp) => {
            let id = resp.inserted_id.as_object_id().unwrap();
            (StatusCode::CREATED, Json(doc! { "inserted_id": id })).into_response()
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}

pub async fn fetch_all_shows(State(db): State<Database>) -> impl IntoResponse {
    let shows_coll: Collection<Document> = db.collection("shows");

    let cursor_result = shows_coll.find(doc! {}).await;

    match cursor_result {
        Ok(cursor) => {
            // Collect documents into a Vec<Document>
            match cursor.try_collect::<Vec<Document>>().await {
                Ok(shows) => (StatusCode::OK, Json(shows)).into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            }
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn fetch_single_show(State(db): State<Database>, Path(id): Path<String>) -> impl IntoResponse {
    let object_id = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid Object Id format").into_response()
    };
    let shows_coll: Collection<Document> = db.collection("shows");
    let res = shows_coll.find_one(doc! {"_id": object_id}).await;

    match res {
        Ok(Some(show)) => (StatusCode::OK, Json(show)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Show not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }

}

pub async fn update_a_show(State(db): State<Database>, Path(id): Path<String>, Json(body): Json<UpdateShow>) -> impl IntoResponse {
    let shows_coll: Collection<Document> = db.collection("shows");

    let object_id = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid Object Id format").into_response(),
    };

    let filter = doc! { "_id": object_id };
    // Set the video_urls field with the new vector from body
    let update = doc! {
        "$set": { "videos": &body.video_urls }
    };

    match shows_coll.update_one(filter, update).await {
        Ok(update_result) => {
            if update_result.matched_count == 1 {
                (StatusCode::OK, "Show updated successfully").into_response()
            } else {
                (StatusCode::NOT_FOUND, "Show not found").into_response()
            }
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn delete_a_show(State(db): State<Database>, Path(id): Path<String>) -> impl IntoResponse {
     let shows_coll: Collection<Document> = db.collection("shows");

    let object_id = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid Object Id format").into_response(),
    };

    let filter = doc! { "_id": object_id };

    let res = shows_coll.delete_one(filter).await;

    match(res) {
        Ok(_) => (StatusCode::OK, "Show deleted").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response()
    }
   
}