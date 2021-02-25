use crate::models::{WkResponse, WkResponseData, WkUserData};

#[derive(Debug)]
pub struct Api {
    token: String,
}

impl Api {
    pub fn new(token: &str) -> Api {
        Api {
            token: token.to_string(),
        }
    }

    pub async fn summary(&self) -> Result<WkResponse<WkResponseData>, reqwest::Error> {
        let client = reqwest::Client::new();
        return client
            .get("https://api.wanikani.com/v2/summary")
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?
            .json::<WkResponse<WkResponseData>>()
            .await;
    }

    pub async fn user(&self) -> Result<WkResponse<WkUserData>, reqwest::Error> {
        let client = reqwest::Client::new();
        return client
            .get("https://api.wanikani.com/v2/user")
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?
            .json::<WkResponse<WkUserData>>()
            .await;
    }
}
