use chrono::{Duration, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: u64,
    iat: i64,
    exp: i64,
}

impl Claims {
    pub fn new(aud: u64) -> Self {
        let now = Utc::now();
        Self {
            aud,
            iat: now.timestamp(),
            exp: (now + Duration::days(7)).timestamp(),
        }
    }
}