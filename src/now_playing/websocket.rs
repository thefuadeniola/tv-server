use chrono::{DateTime, Utc};
use mongodb::Database;
use mongodb::bson::doc;
use futures::stream::TryStreamExt;


use super::super::show::show_models::Show;
use super::super::schedule::schedule_models::ScheduleEntry;
use super::models::NowPlaying;

pub async fn get_now_playing(now: DateTime<Utc>, db: &Database) -> Option<NowPlaying> {

    let schedule_coll = db.collection::<ScheduleEntry>("schedule");
    let show_coll = db.collection::<Show>("shows");

    // Find the schedule entry that is currently playing
    let filter = doc! {
        "start_time": { "$lte": now }
    };

    let mut cursor = match schedule_coll.find(filter).sort(doc! { "start_time": -1 }).limit(1).await {
        Ok(c) => c,
        Err(_) => return None,
    };

    let Some(schedule) = cursor.try_next().await.ok().flatten() else {
        return None;
    };

    let show = match show_coll.find_one(doc! { "_id": &schedule.show_id }).await {
        Ok(Some(show)) => show,
        _ => return None,
    };

    // Compute how many seconds have passed since the show started
    let elapsed = (now - schedule.start_time).num_seconds();

    Some(NowPlaying {
        show_title: show.title.clone(),
        video_id: show.videos[0].clone(),
        time_elapsed: elapsed as u32,
    })
}
