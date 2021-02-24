use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct WkResponse {
    url: String,
    data_updated_at: String,
    data: WkResponseData,
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
