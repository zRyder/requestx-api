use chrono::{DateTime, Duration, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: u64,
    iat: DateTime<Utc>,
    exp: DateTime<Utc>,
}

impl Claims {
    pub fn new(aud: u64) -> Self {
        let now = Utc::now();
        Self {
            aud,
            iat: now,
            exp: now + Duration::days(7),
        }
    }
}