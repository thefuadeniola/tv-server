
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use mongodb::bson::{oid::ObjectId};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScheduleEntry {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub show_id: ObjectId,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct CreateScheduleEntry {
    pub show_id: ObjectId,
    pub start_time: DateTime<Utc>,
    
/*     #[serde(default)]
    pub force: bool
 */
}