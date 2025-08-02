use crate::show::show_models::Show;
use serde::{Serialize, Deserialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct NowPlaying {
    pub show_title: String,
    pub video_id: String,
    pub time_elapsed: u32,
}

/* pub fn to_now_playing(show: &Show, start_time: u64) -> NowPlaying {
    let server_time = Utc::now().timestamp() as u64;
    let duration_secs = show.total_duration.as_secs();

    NowPlaying {
        title: show.title.clone(),
        start_time,
        end_time: start_time + duration_secs,
        server_time,
    }
}
 */