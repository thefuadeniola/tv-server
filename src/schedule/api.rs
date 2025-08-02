use axum::{extract::{State}, http::StatusCode, Json, response::IntoResponse};
use mongodb::{bson::{doc}, Collection, Database};
use super::super::show::show_models::Show;
use super::{ScheduleEntry, CreateScheduleEntry};
use crate::error::AppError;
use chrono::{Duration as ChronoDuration};


pub async fn create_schedule_entry(State(db): State<Database>, Json(body): Json<CreateScheduleEntry>) -> impl IntoResponse {
    let shows_coll: Collection<Show> = db.collection("shows");
    let schedule_coll: Collection<ScheduleEntry> = db.collection("schedule");

    // Get show to calculate duration
    // Try to find the show by ID
    let show_option = shows_coll
        .find_one(doc! { "_id": &body.show_id })
        .await;

    let show = match show_option {
        Ok(Some(s)) => s,
        Ok(None) => {
            return AppError::NotFound("Show not found".into()).into_response();
        }
        Err(e) => {
            return AppError::DatabaseError(e.to_string()).into_response();
        }
    };

    // Convert std::time::Duration to chrono::Duration
    let chrono_duration = match ChronoDuration::from_std(show.total_duration) {
        Ok(dur) => dur,
        Err(_) => {
            return AppError::Internal("Failed to convert duration".into()).into_response();
        }
    };


    let end_time = body.start_time + chrono_duration;

    let new_entry = ScheduleEntry {
        id: None,
        show_id: body.show_id,
        start_time: body.start_time,
        end_time,
    };

    let result = schedule_coll.insert_one(new_entry.clone()).await;

    match result {
        Ok(value) => Json(value).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}