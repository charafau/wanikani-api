use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct WkResponse<T> {
    url: String,
    data_updated_at: String,
    data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WkElement {
    available_at: String,
    subject_ids: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WkResponseData {
    lessons: Vec<WkElement>,
    reviews: Vec<WkElement>,
    next_reviews_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WkUserData {
    id: String,
    username: String,
    level: u32,
    profile_url: String,
    started_at: String,
    subscription: WkUserSubscription,
    current_vacation_started_at: Option<String>,
    preferences: WkUserPreferences,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WkUserPreferences {
    lessons_batch_size: u32,
    default_voice_actor_id: u32,
    lessons_autoplay_audio: bool,
    reviews_autoplay_audio: bool,
    lessons_presentation_order: String,
    reviews_display_srs_indicator: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WkUserSubscription {
    active: bool,
    #[serde(rename = "type")]
    sub_type: String,
    max_level_granted: u32,
    period_ends_at: Option<String>,
}
