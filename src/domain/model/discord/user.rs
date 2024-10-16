use chrono::{DateTime, Utc};
use sea_orm::ActiveValue;

use crate::adapter::mysql::model::{user, user::Model};

#[derive(Debug, Clone, Copy)]
pub struct DiscordUser {
	pub discord_user_id: u64,
	pub last_request_time: Option<DateTime<Utc>>
}

impl Into<user::ActiveModel> for DiscordUser {
	fn into(self) -> user::ActiveModel {
		user::ActiveModel {
			discord_id: ActiveValue::Set(self.discord_user_id),
			timestamp: ActiveValue::Set(if let Some(last_request_time) = self.last_request_time {
				Some(last_request_time)
			} else {
				None
			})
		}
	}
}

impl From<Model> for DiscordUser {
	fn from(value: Model) -> Self {
		Self {
			discord_user_id: value.discord_id,
			last_request_time: value.timestamp
		}
	}
}
